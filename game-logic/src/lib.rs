use entity::{EntitySet, TempEntityType};
#[cfg(all(not(feature = "webgl"), feature = "piston"))]
use piston_window::{draw_state::Blend, G2d, *};
#[cfg(feature = "webgl")]
use std::rc::Rc;
use std::vec;
#[cfg(feature = "webgl")]
use wasm_bindgen::{prelude::*, JsCast};
#[cfg(feature = "webgl")]
use web_sys::{HtmlImageElement, WebGlBuffer, WebGlRenderingContext as GL, WebGlTexture};

#[cfg(feature = "webgl")]
#[macro_export]
macro_rules! console_log {
    ($fmt:expr, $($arg1:expr),*) => {
        crate::log(&format!($fmt, $($arg1),+))
    };
    ($fmt:expr) => {
        crate::log($fmt)
    }
}

#[cfg(not(feature = "webgl"))]
macro_rules! console_log {
    ($fmt:expr, $($arg1:expr),*) => {
        println!($fmt, $($arg1),+)
    };
    ($fmt:expr) => {
        println!($fmt)
    }
}

#[cfg(feature = "webgl")]
#[macro_export]
/// format-like macro that returns js_sys::String
macro_rules! js_str {
    ($fmt:expr, $($arg1:expr),*) => {
        JsValue::from_str(&format!($fmt, $($arg1),+))
    };
    ($fmt:expr) => {
        JsValue::from_str($fmt)
    }
}

#[cfg(feature = "webgl")]
#[macro_export]
/// format-like macro that returns Err(js_sys::String)
macro_rules! js_err {
    ($fmt:expr, $($arg1:expr),*) => {
        Err(JsValue::from_str(&format!($fmt, $($arg1),+)))
    };
    ($fmt:expr) => {
        Err(JsValue::from_str($fmt))
    }
}

#[cfg(all(not(feature = "webgl"), feature = "piston"))]
pub mod assets_piston;
#[cfg(feature = "webgl")]
pub mod assets_webgl;
pub mod consts;
pub mod entity;
pub mod xor128;

#[cfg(all(not(feature = "webgl"), feature = "piston"))]
use crate::assets_piston::Assets;
#[cfg(feature = "webgl")]
use crate::assets_webgl::Assets;
use crate::consts::*;
use crate::entity::{
    BulletBase, DeathReason, Enemy, EnemyBase, Entity, Item, Player, Projectile, ShieldedBoss,
    TempEntity, Weapon,
};
use xor128::Xor128;

#[cfg(feature = "webgl")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub(crate) fn log(s: &str);
}

#[cfg(feature = "webgl")]
pub type ShooterError = JsValue;

#[cfg(not(feature = "webgl"))]
pub type ShooterError = std::io::Error;

pub struct ShooterState {
    pub time: usize,
    pub disptime: usize,
    pub paused: bool,
    pub game_over: bool,
    pub player: Player,
    pub enemies: EntitySet<Enemy>,
    pub items: EntitySet<Item>,
    pub bullets: EntitySet<Projectile>,
    pub tent: EntitySet<TempEntity>,
    pub rng: Xor128,
    pub shots_bullet: usize,
    pub shots_missile: usize,
}

impl Default for ShooterState {
    fn default() -> Self {
        let mut player = Player::new(Entity::new([FWIDTH / 2., FHEIGHT * 3. / 4.], [0., 0.]));
        player.reset();

        ShooterState {
            time: 0,
            disptime: 0,
            paused: false,
            game_over: false,
            player,
            enemies: EntitySet::new(),
            items: EntitySet::new(),
            bullets: EntitySet::new(),
            tent: EntitySet::new(),
            rng: Xor128::new(3232132),
            shots_bullet: 0,
            shots_missile: 0,
        }
    }
}

impl ShooterState {
    pub fn restart(&mut self) -> Result<(), ShooterError> {
        self.items.clear();
        self.enemies.clear();
        self.bullets.clear();
        #[cfg(feature = "webgl")]
        self.tent.clear();
        self.time = 0;
        self.player.reset();
        self.shots_bullet = 0;
        self.shots_missile = 0;
        self.paused = false;
        self.game_over = false;
        Ok(())
    }

    #[cfg(not(feature = "piston"))]
    fn add_blend(ent: Entity) -> Entity {
        ent
    }

    #[cfg(feature = "piston")]
    fn add_blend(ent: Entity) -> Entity {
        ent.blend(Blend::Add)
    }

    /// Function to call the same lightning sequence twice, first pass for detecting hit enemy
    /// and second pass for rendering.
    pub fn lightning_branch(
        &mut self,
        seed: u32,
        length: u32,
        f: &mut dyn FnMut(&mut Self, &[f64; 4]) -> bool,
    ) -> u32 {
        // Random walk with momentum
        fn next_lightning(rng: &mut Xor128, a: &mut [f64; 4]) {
            a[2] += LIGHTNING_ACCEL * (rng.gen() - 0.5) - a[2] * LIGHTNING_FEEDBACK;
            a[3] += LIGHTNING_ACCEL * (rng.gen() - 0.5) - a[3] * LIGHTNING_FEEDBACK;
            a[0] += a[2];
            a[1] += a[3];
        }

        let mut rng2 = Xor128::new(seed);
        let mut a = [self.player.base.pos[0], self.player.base.pos[1], 0., -16.];
        for i in 0..length {
            let ox = a[0];
            let oy = a[1];
            next_lightning(&mut rng2, &mut a);
            let segment = [ox, oy, a[0], a[1]];
            if !f(self, &segment) {
                return i;
            }
        }
        length
    }

    pub fn lightning(
        &mut self,
        seed: u32,
        nmax: Option<usize>,
        f: &mut dyn FnMut(&mut Self, u32),
    ) -> usize {
        let nmax = if let Some(v) = nmax {
            v
        } else {
            std::cmp::min(
                (self.player.power_level() as usize + 1 + self.time % 2) / 2,
                31,
            )
        };
        let mut branch_rng = Xor128::new(seed);

        for _ in 0..nmax {
            // Use the same seed twice to reproduce random sequence
            let seed = branch_rng.nexti();

            f(self, seed);
        }

        nmax
    }

    pub fn try_shoot(
        &mut self,
        key_shoot: bool,
        seed: u32,
        add_tent: &mut impl FnMut(TempEntityType, &[f64; 2], &mut ShooterState),
    ) -> usize {
        let weapon = self.player.weapon;
        let shoot_period = if let Weapon::Bullet = weapon { 5 } else { 50 };

        if Weapon::Bullet == weapon || Weapon::Missile == weapon {
            let player = &mut self.player;
            if key_shoot && player.cooldown == 0 {
                let level = player.power_level() as i32;
                player.cooldown += shoot_period;
                for i in -1 - level..2 + level {
                    let speed = if let Weapon::Bullet = weapon {
                        BULLET_SPEED
                    } else {
                        MISSILE_SPEED
                    };
                    let mut ent = Entity::new(player.base.pos, [i as f64, -speed])
                        .rotation((i as f32).atan2(speed as f32));
                    if let Weapon::Bullet = weapon {
                        self.shots_bullet += 1;
                        ent = Self::add_blend(ent);
                        self.bullets.insert(Projectile::Bullet(BulletBase(ent)));
                    } else {
                        self.shots_missile += 1;
                        ent = ent.health(5);
                        self.bullets.insert(Projectile::Missile {
                            base: BulletBase(ent),
                            target: None,
                            trail: vec![],
                        });
                    }
                }
            }
        } else if Weapon::Light == weapon && key_shoot {
            let player = &self.player;
            let level = player.power_level() as i32;
            let beam_rect = [
                player.base.pos[0] - LIGHT_WIDTH,
                0.,
                player.base.pos[0] + LIGHT_WIDTH,
                player.base.pos[1],
            ];

            let mut enemies = std::mem::take(&mut self.enemies);
            for enemy in enemies.iter_mut() {
                if enemy.test_hit(beam_rect) {
                    add_tent(TempEntityType::Explode2, &enemy.pos, self);
                    enemy.damage(1 + level, &beam_rect);
                }
            }
            self.enemies = enemies;
        } else if Weapon::Lightning == weapon && key_shoot {
            return self.lightning(seed, None, &mut |state, seed| {
                state.lightning_branch(
                    seed,
                    LIGHTNING_VERTICES,
                    &mut |state: &mut Self, segment: &[f64; 4]| {
                        let b = [segment[2], segment[3]];
                        let mut res = true;
                        for enemy in state.enemies.iter_mut() {
                            let ebb = enemy.get_bb();
                            if ebb[0] < b[0] + 4.
                                && b[0] - 4. <= ebb[2]
                                && ebb[1] < b[1] + 4.
                                && b[1] - 4. <= ebb[3]
                            {
                                enemy.damage(2 + state.rng.gen_range(0, 3) as i32, &ebb);
                                res = false;
                                // Needs to break this loop before add_tent for borrow checker limitation.
                                break;
                            }
                        }
                        if !res {
                            add_tent(TempEntityType::Explode2, &b, state);
                        }
                        res
                    },
                );
            });
        }
        0
    }

    /// Generate enemies in this frame.
    ///
    /// Returns: wave_period
    pub fn gen_enemies(&mut self) -> usize {
        let wave_period = 1024;
        if !self.paused {
            let dice = 256;
            let wave = self.time % wave_period;
            if wave < wave_period * 3 / 4 {
                let [mut enemy_count, mut boss_count, mut shielded_boss_count, mut spiral_count, mut centipede_count] =
                    [0; 5];
                for e in self.enemies.iter() {
                    match &*e {
                        Enemy::Enemy1(_) => {
                            enemy_count += 1;
                        }
                        Enemy::Boss(_) => {
                            boss_count += 1;
                        }
                        Enemy::ShieldedBoss(_) => {
                            shielded_boss_count += 1;
                        }
                        Enemy::SpiralEnemy(_) => {
                            spiral_count += 1;
                        }
                        Enemy::Centipede(_) => {
                            centipede_count += 1;
                        }
                    }
                }
                let gen_amount = self.player.difficulty_level() * 4 + 8;
                let mut i = self.rng.gen_range(0, dice);
                while i < gen_amount {
                    let weights = [
                        if enemy_count < 128 {
                            if self.player.score < 1024 {
                                64
                            } else {
                                16
                            }
                        } else {
                            0
                        },
                        if boss_count < 32 { 4 } else { 0 },
                        if shielded_boss_count < 32 {
                            std::cmp::min(4, self.player.difficulty_level())
                        } else {
                            0
                        },
                        if spiral_count < 4 { 4 } else { 0 },
                        if centipede_count < 4 { 4 } else { 0 },
                    ];
                    let allweights = weights.iter().sum();
                    let accum = {
                        let mut accum = [0; 5];
                        let mut accumulator = 0;
                        for (i, e) in weights.iter().enumerate() {
                            accumulator += e;
                            accum[i] = accumulator;
                        }
                        accum
                    };

                    if 0 < allweights {
                        let rng = &mut self.rng;
                        let dice = rng.gen_range(0, allweights);
                        let (pos, velo) = match rng.gen_range(0, 3) {
                            0 => {
                                // top
                                (
                                    [rng.gen_rangef(0., WIDTH as f64), 0.],
                                    [rng.gen() - 0.5, rng.gen() * 0.5],
                                )
                            }
                            1 => {
                                // left
                                (
                                    [0., rng.gen_rangef(0., WIDTH as f64)],
                                    [rng.gen() * 0.5, rng.gen() - 0.5],
                                )
                            }
                            2 => {
                                // right
                                (
                                    [WIDTH as f64, rng.gen_rangef(0., WIDTH as f64)],
                                    [-rng.gen() * 0.5, rng.gen() - 0.5],
                                )
                            }
                            _ => panic!("RNG returned out of range"),
                        };
                        if let Some(x) = accum.iter().position(|x| dice < *x) {
                            self.enemies.insert(match x {
                                0 => Enemy::Enemy1(EnemyBase::new(pos, velo).health(3)),
                                1 => Enemy::Boss(EnemyBase::new(pos, velo).health(64)),
                                2 => Enemy::ShieldedBoss(ShieldedBoss::new(pos, velo)),
                                3 => Enemy::new_spiral(pos, velo),
                                _ => Enemy::new_centipede(pos, velo),
                            });
                        }
                    }
                    i += self.rng.gen_range(0, dice);
                }
            }
        }
        wave_period
    }

    #[cfg(feature = "webgl")]
    pub fn draw_items(&self, gl: &GL, assets: &Assets) {
        for item in &self.items {
            item.draw(gl, assets);
        }
    }

    #[cfg(all(not(feature = "webgl"), feature = "piston"))]
    pub fn draw_items(&self, context: &Context, graphics: &mut G2d, assets: &Assets) {
        for item in &self.items {
            item.draw(&context, graphics, &assets);
        }
    }

    pub fn animate_items(&mut self) {
        if self.paused {
            return;
        }
        let mut items = std::mem::take(&mut self.items);
        items.retain_id(|id, e| {
            if e.animate(&mut self.player).is_some() {
                println!("Deleted Item {} / {}", id, self.items.len());
                return false;
            }
            true
        });
        self.items = items;
    }

    #[cfg(feature = "webgl")]
    pub fn draw_enemies(&self, gl: &GL, assets: &Assets) {
        for enemy in &self.enemies {
            enemy.draw(self, gl, assets);
        }
    }

    #[cfg(all(not(feature = "webgl"), feature = "piston"))]
    pub fn draw_enemies(&self, context: &Context, graphics: &mut G2d, assets: &Assets) {
        for enemy in &self.enemies {
            enemy.draw(&context, graphics, &assets);
        }
    }

    pub fn animate_enemies(&mut self, on_killed: &mut impl FnMut(&Enemy, &mut ShooterState)) {
        if self.paused {
            return;
        }
        let mut enemies = std::mem::take(&mut self.enemies);
        enemies.retain_id(|id, enemy| {
            if self.paused {
                return true;
            }
            let ret = {
                if let Some(death_reason) = enemy.animate(self) {
                    if matches!(death_reason, DeathReason::Killed) {
                        on_killed(enemy, self);
                    }
                    println!(
                        "Deleted Enemy {} id={} {}",
                        match enemy {
                            Enemy::Enemy1(_) => "enemy",
                            Enemy::Boss(_) => "boss",
                            Enemy::ShieldedBoss(_) => "ShieldedBoss",
                            Enemy::SpiralEnemy(_) => "SpiralEnemy",
                            Enemy::Centipede(_) => "Centipede",
                        },
                        id,
                        self.enemies.len()
                    );
                    false
                } else {
                    true
                }
            };
            if !ret {
                self.player.kills += 1;
                self.player.score += if enemy.is_boss() { 10 } else { 1 };
                if self.rng.gen_range(0, 100) < 20 {
                    let ent = Entity::new(enemy.pos, [0., 1.]);
                    self.items.insert(enemy.drop_item(ent));
                }
            }
            ret
        });
        self.enemies = enemies;
    }

    #[cfg(feature = "webgl")]
    pub fn draw_bullets(&self, gl: &GL, assets: &Assets) {
        for b in self.bullets.iter() {
            b.draw(gl, assets);
        }
    }

    #[cfg(all(not(feature = "webgl"), feature = "piston"))]
    pub fn draw_bullets(&self, context: &Context, graphics: &mut G2d, assets: &Assets) {
        for b in self.bullets.iter() {
            b.draw(&context, graphics, &assets);
        }
    }

    /// Returns true if player was killed, signaling game over event.
    pub fn animate_bullets(
        &mut self,
        add_tent: &mut impl FnMut(TempEntityType, &[f64; 2], &mut ShooterState),
    ) -> bool {
        if self.paused {
            return false;
        }
        let mut ret = false;
        let mut bullets_to_delete = Vec::new();
        let mut bullets = std::mem::take(&mut self.bullets);
        bullets.retain_id(|i, b| {
            if self.paused {
                return true;
            }
            let Some(death_reason) = b.animate_bullet(&mut self.enemies, &mut self.player) else {
                return true;
            };
            bullets_to_delete.push(i);

            match death_reason {
                DeathReason::Killed | DeathReason::HitPlayer => {
                    let tt = match b {
                        Projectile::Missile { .. } => TempEntityType::Explode2,
                        _ => TempEntityType::Explode,
                    };
                    add_tent(tt, &b.pos, self)
                }
                _ => {}
            }

            if let DeathReason::HitPlayer = death_reason {
                if self.player.invtime == 0 && !self.game_over && 0 < self.player.lives {
                    self.player.lives -= 1;
                    if self.player.lives == 0 {
                        self.game_over = true;
                        ret = true;
                    } else {
                        self.player.invtime = PLAYER_INVINCIBLE_TIME;
                    }
                }
            }

            println!("Deleted {} id={} ({})", b.get_type(), i, self.bullets.len());

            false
        });
        self.bullets = bullets;

        ret
    }

    #[cfg(feature = "webgl")]
    pub fn draw_tents(&self, gl: &GL, assets: &Assets) {
        for tent in &self.tent {
            tent.draw_temp(gl, assets);
        }
    }

    #[cfg(all(not(feature = "webgl"), feature = "piston"))]
    pub fn draw_tents(&self, context: &Context, graphics: &mut G2d) {
        for e in &self.tent {
            e.draw_temp(&context, graphics);
        }
    }

    pub fn animate_tents(&mut self) {
        if self.paused {
            return;
        }
        let paused = self.paused;
        self.tent.retain(|e| {
            if !paused && e.animate_temp().is_some() {
                //println!("Deleted tent {} / {}", *i, bullets.len());
                return false;
            }
            true
        });
    }
}

#[cfg(feature = "webgl")]
/// Initialize a texture and load an image.
/// When the image finished loading copy it into the texture.
fn load_texture(gl: &GL, url: &str) -> Result<Rc<WebGlTexture>, JsValue> {
    fn window() -> Option<web_sys::Window> {
        web_sys::window()
    }

    fn document() -> Option<web_sys::Document> {
        window()?.document()
    }

    fn get_context() -> Result<GL, JsValue> {
        let document = document().ok_or_else(|| js_str!("no document!"))?;
        let canvas = document.get_element_by_id("canvas").unwrap();
        let canvas: web_sys::HtmlCanvasElement =
            canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();

        Ok(canvas
            .get_context("webgl")?
            .ok_or_else(|| js_str!("no context"))?
            .dyn_into::<GL>()?)
    }

    fn is_power_of_2(value: u32) -> bool {
        (value & (value - 1)) == 0
    }

    let texture = Rc::new(gl.create_texture().unwrap());
    gl.bind_texture(GL::TEXTURE_2D, Some(&*texture));

    // Because images have to be downloaded over the internet
    // they might take a moment until they are ready.
    // Until then put a single pixel in the texture so we can
    // use it immediately. When the image has finished downloading
    // we'll update the texture with the contents of the image.
    let level = 0;
    let internal_format = GL::RGBA as i32;
    let width = 1;
    let height = 1;
    let border = 0;
    let src_format = GL::RGBA;
    let src_type = GL::UNSIGNED_BYTE;
    let pixel = [0u8, 255, 255, 255];
    gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
        GL::TEXTURE_2D,
        level,
        internal_format,
        width,
        height,
        border,
        src_format,
        src_type,
        Some(&pixel),
    )?;
    gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_S, GL::REPEAT as i32);
    gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_T, GL::REPEAT as i32);
    gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MIN_FILTER, GL::LINEAR as i32);

    let image = Rc::new(HtmlImageElement::new()?);
    let url_str = url.to_owned();
    let image_clone = image.clone();
    let texture_clone = texture.clone();
    let callback = Closure::wrap(Box::new(move || -> Result<(), JsValue> {
        console_log!("loaded image: {}", url_str);
        // web_sys::console::log_1(Date::new_0().to_locale_string("en-GB", &JsValue::undefined()));

        let gl = get_context()?;

        gl.bind_texture(GL::TEXTURE_2D, Some(&*texture_clone));
        gl.tex_image_2d_with_u32_and_u32_and_image(
            GL::TEXTURE_2D,
            level,
            internal_format,
            src_format,
            src_type,
            &image_clone,
        )?;

        // WebGL1 has different requirements for power of 2 images
        // vs non power of 2 images so check if the image is a
        // power of 2 in both dimensions.
        if is_power_of_2(image_clone.width()) && is_power_of_2(image_clone.height()) {
            // Yes, it's a power of 2. Generate mips.
            gl.generate_mipmap(GL::TEXTURE_2D);
        } else {
            // No, it's not a power of 2. Turn off mips and set
            // wrapping to clamp to edge
            gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_S, GL::CLAMP_TO_EDGE as i32);
            gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_T, GL::CLAMP_TO_EDGE as i32);
            gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MIN_FILTER, GL::LINEAR as i32);
        }
        Ok(())
    }) as Box<dyn FnMut() -> Result<(), JsValue>>);
    image.set_onload(Some(callback.as_ref().unchecked_ref()));
    image.set_src(url);

    callback.forget();

    Ok(texture)
}

#[cfg(feature = "webgl")]
pub fn vertex_buffer_data(context: &GL, vertices: &[f32]) {
    // Note that `Float32Array::view` is somewhat dangerous (hence the
    // `unsafe`!). This is creating a raw view into our module's
    // `WebAssembly.Memory` buffer, but if we allocate more pages for ourself
    // (aka do a memory allocation in Rust) it'll cause the buffer to change,
    // causing the `Float32Array` to be invalid.
    //
    // As a result, after `Float32Array::view` we have to be very careful not to
    // do any memory allocations before it's dropped.
    unsafe {
        let vert_array = js_sys::Float32Array::view(vertices);

        context.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &vert_array, GL::STATIC_DRAW);
    };
}

#[cfg(feature = "webgl")]
pub fn enable_buffer(gl: &GL, buffer: &Option<WebGlBuffer>, elements: i32, vertex_position: u32) {
    gl.bind_buffer(GL::ARRAY_BUFFER, buffer.as_ref());
    gl.vertex_attrib_pointer_with_i32(vertex_position, elements, GL::FLOAT, false, 0, 0);
    gl.enable_vertex_attrib_array(vertex_position);
}

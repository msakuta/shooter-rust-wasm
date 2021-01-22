#[cfg(all(not(feature = "webgl"), feature = "piston"))]
use piston_window::{draw_state::Blend, G2d, *};
#[cfg(feature = "webgl")]
use std::rc::Rc;
use std::{collections::HashMap, vec};
#[cfg(feature = "webgl")]
use wasm_bindgen::{prelude::*, JsCast};
#[cfg(feature = "webgl")]
use web_sys::{
    Document, Element, HtmlImageElement, WebGlBuffer, WebGlProgram, WebGlRenderingContext as GL,
    WebGlShader, WebGlTexture,
};

#[cfg(feature = "webgl")]
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
/// format-like macro that returns Err(js_sys::String)
macro_rules! js_err {
    ($fmt:expr, $($arg1:expr),*) => {
        Err(JsValue::from_str(&format!($fmt, $($arg1),+)))
    };
    ($fmt:expr) => {
        Err(JsValue::from_str($fmt))
    }
}

pub mod consts;
pub mod entity;
pub mod xor128;

use crate::consts::*;
#[cfg(feature = "webgl")]
use crate::entity::ShaderBundle;
use crate::entity::{
    Assets, BulletBase, DeathReason, Enemy, EnemyBase, Entity, Item, Player, Projectile,
    ShieldedBoss, TempEntity, Weapon,
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
    pub id_gen: u32,
    pub player: Player,
    pub enemies: Vec<Enemy>,
    pub items: Vec<Item>,
    pub bullets: HashMap<u32, Projectile>,
    pub tent: Vec<TempEntity>,
    pub rng: Xor128,
    pub shots_bullet: usize,
    pub shots_missile: usize,

    pub shoot_pressed: bool,
    pub left_pressed: bool,
    pub right_pressed: bool,
    pub up_pressed: bool,
    pub down_pressed: bool,

    #[cfg(feature = "webgl")]
    pub assets: Assets,
}

impl ShooterState {
    pub fn new(_assets: Option<Assets>) -> Self {
        let mut id_gen = 0;
        let mut player = Player::new(Entity::new(
            &mut id_gen,
            [FWIDTH / 2., FHEIGHT * 3. / 4.],
            [0., 0.],
        ));
        player.reset();

        ShooterState {
            time: 0,
            disptime: 0,
            paused: false,
            game_over: false,
            id_gen,
            player,
            enemies: vec![],
            items: vec![],
            bullets: HashMap::new(),
            tent: vec![],
            rng: Xor128::new(3232132),
            shots_bullet: 0,
            shots_missile: 0,
            shoot_pressed: false,
            left_pressed: false,
            right_pressed: false,
            up_pressed: false,
            down_pressed: false,
            #[cfg(feature = "webgl")]
            assets: _assets.unwrap(),
        }
    }

    pub fn restart(&mut self) -> Result<(), ShooterError> {
        self.items.clear();
        self.enemies.clear();
        self.bullets.clear();
        #[cfg(feature = "webgl")]
        self.tent.clear();
        self.time = 0;
        self.id_gen = 0;
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
            a[2] += LIGHTNING_ACCEL * (rng.next() - 0.5) - a[2] * LIGHTNING_FEEDBACK;
            a[3] += LIGHTNING_ACCEL * (rng.next() - 0.5) - a[3] * LIGHTNING_FEEDBACK;
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

    pub fn lightning(&mut self, seed: u32, f: &mut dyn FnMut(&mut Self, u32)) {
        let nmax = std::cmp::min(
            (self.player.power_level() as usize + 1 + self.time % 2) / 2,
            31,
        );
        let mut branch_rng = Xor128::new(seed);

        for _ in 0..nmax {
            // Use the same seed twice to reproduce random sequence
            let seed = branch_rng.nexti();

            f(self, seed);
        }
    }

    pub fn try_shoot(
        &mut self,
        key_shoot: bool,
        weapon: &Weapon,
        seed: u32,
        add_tent: &mut impl FnMut(bool, &[f64; 2], &mut ShooterState),
    ) {
        let shoot_period = if let Weapon::Bullet = weapon { 5 } else { 50 };

        if Weapon::Bullet == *weapon || Weapon::Missile == *weapon {
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
                    let mut ent =
                        Entity::new(&mut self.id_gen, player.base.pos, [i as f64, -speed])
                            .rotation((i as f32).atan2(speed as f32));
                    if let Weapon::Bullet = weapon {
                        self.shots_bullet += 1;
                        ent = Self::add_blend(ent);
                        self.bullets
                            .insert(ent.id, Projectile::Bullet(BulletBase(ent)));
                    } else {
                        self.shots_missile += 1;
                        ent = ent.health(5);
                        self.bullets.insert(
                            ent.id,
                            Projectile::Missile {
                                base: BulletBase(ent),
                                target: 0,
                                trail: vec![],
                            },
                        );
                    }
                }
            }
        } else if Weapon::Light == *weapon && key_shoot {
            let player = &self.player;
            let level = player.power_level() as i32;
            let beam_rect = [
                player.base.pos[0] - LIGHT_WIDTH,
                0.,
                player.base.pos[0] + LIGHT_WIDTH,
                player.base.pos[1],
            ];

            let mut enemies = std::mem::take(&mut self.enemies);
            for enemy in &mut enemies {
                if enemy.test_hit(beam_rect) {
                    add_tent(true, &enemy.get_base().pos, self);
                    enemy.damage(1 + level);
                }
            }
            self.enemies = enemies;
        } else if Weapon::Lightning == *weapon && key_shoot {
            self.lightning(seed, &mut |state, seed| {
                state.lightning_branch(
                    seed,
                    LIGHTNING_VERTICES,
                    &mut |state: &mut Self, segment: &[f64; 4]| {
                        let b = [segment[2], segment[3]];
                        for enemy in state.enemies.iter_mut() {
                            let ebb = enemy.get_bb();
                            if ebb[0] < b[0] + 4.
                                && b[0] - 4. <= ebb[2]
                                && ebb[1] < b[1] + 4.
                                && b[1] - 4. <= ebb[3]
                            {
                                enemy.damage(2 + state.rng.gen_range(0, 3) as i32);
                                add_tent(true, &b, state);
                                return false;
                            }
                        }
                        return true;
                    },
                );
            });
        }
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
                let [enemy_count, boss_count, shielded_boss_count, spiral_count] =
                    self.enemies.iter().fold([0; 4], |mut c, e| match e {
                        Enemy::Enemy1(_) => {
                            c[0] += 1;
                            c
                        }
                        Enemy::Boss(_) => {
                            c[1] += 1;
                            c
                        }
                        Enemy::ShieldedBoss(_) => {
                            c[2] += 1;
                            c
                        }
                        Enemy::SpiralEnemy(_) => {
                            c[3] += 1;
                            c
                        }
                    });
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
                    ];
                    let allweights = weights.iter().fold(0, |sum, x| sum + x);
                    let accum = {
                        let mut accum = [0; 4];
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
                                    [rng.next() - 0.5, rng.next() * 0.5],
                                )
                            }
                            1 => {
                                // left
                                (
                                    [0., rng.gen_rangef(0., WIDTH as f64)],
                                    [rng.next() * 0.5, rng.next() - 0.5],
                                )
                            }
                            2 => {
                                // right
                                (
                                    [WIDTH as f64, rng.gen_rangef(0., WIDTH as f64)],
                                    [-rng.next() * 0.5, rng.next() - 0.5],
                                )
                            }
                            _ => panic!("RNG returned out of range"),
                        };
                        if let Some(x) = accum.iter().position(|x| dice < *x) {
                            self.enemies.push(match x {
                                0 => Enemy::Enemy1(
                                    EnemyBase::new(&mut self.id_gen, pos, velo).health(3),
                                ),
                                1 => Enemy::Boss(
                                    EnemyBase::new(&mut self.id_gen, pos, velo).health(64),
                                ),
                                2 => Enemy::ShieldedBoss(ShieldedBoss::new(
                                    &mut self.id_gen,
                                    pos,
                                    velo,
                                )),
                                _ => Enemy::new_spiral(&mut self.id_gen, pos, velo),
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
    pub fn draw_items(&self, gl: &GL) {
        for item in &self.items {
            item.draw(gl, &self.assets);
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
        let mut to_delete = vec![];
        for (i, e) in &mut ((&mut self.items).iter_mut().enumerate()) {
            if !self.paused {
                if let Some(_) = e.animate(&mut self.player) {
                    to_delete.push(i);
                    continue;
                }
            }
        }

        for i in to_delete.iter().rev() {
            let dead = self.items.remove(*i);
            println!(
                "Deleted Item id={}: {} / {}",
                dead.get_base().id,
                *i,
                self.items.len()
            );
        }
    }

    #[cfg(feature = "webgl")]
    pub fn draw_enemies(&self, gl: &GL) {
        for enemy in &self.enemies {
            enemy.draw(self, gl, &self.assets);
        }
    }

    #[cfg(all(not(feature = "webgl"), feature = "piston"))]
    pub fn draw_enemies(&self, context: &Context, graphics: &mut G2d, assets: &Assets) {
        for enemy in &self.enemies {
            enemy.draw(&context, graphics, &assets);
        }
    }

    pub fn animate_enemies(&mut self) {
        if self.paused {
            return;
        }
        let mut to_delete: Vec<usize> = Vec::new();
        let mut enemies = std::mem::take(&mut self.enemies);
        for (i, enemy) in &mut ((&mut enemies).iter_mut().enumerate()) {
            if !self.paused {
                let killed = {
                    if let Some(death_reason) = enemy.animate(self) {
                        to_delete.push(i);
                        if let DeathReason::Killed = death_reason {
                            true
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                };
                if killed {
                    self.player.kills += 1;
                    self.player.score += if enemy.is_boss() { 10 } else { 1 };
                    if self.rng.gen_range(0, 100) < 20 {
                        let ent = Entity::new(&mut self.id_gen, enemy.get_base().pos, [0., 1.]);
                        self.items.push(enemy.drop_item(ent));
                    }
                    continue;
                }
            }
        }
        self.enemies = enemies;

        for i in to_delete.iter().rev() {
            let dead = self.enemies.remove(*i);
            println!(
                "Deleted Enemy {} id={}: {} / {}",
                match dead {
                    Enemy::Enemy1(_) => "enemy",
                    Enemy::Boss(_) => "boss",
                    Enemy::ShieldedBoss(_) => "ShieldedBoss",
                    Enemy::SpiralEnemy(_) => "SpiralEnemy",
                },
                dead.get_id(),
                *i,
                self.enemies.len()
            );
        }
    }

    #[cfg(feature = "webgl")]
    pub fn draw_bullets(&self, gl: &GL) {
        for (_, b) in &self.bullets {
            b.draw(self, gl, &self.assets);
        }
    }

    #[cfg(all(not(feature = "webgl"), feature = "piston"))]
    pub fn draw_bullets(&self, context: &Context, graphics: &mut G2d, assets: &Assets) {
        for (_, b) in &self.bullets {
            b.draw(&context, graphics, &assets);
        }
    }

    pub fn animate_bullets(
        &mut self,
        add_tent: &mut impl FnMut(bool, &[f64; 2], &mut ShooterState),
    ) {
        if self.paused {
            return;
        }
        let mut bullets_to_delete: Vec<u32> = Vec::new();
        let mut bullets = std::mem::take(&mut self.bullets);
        for (i, b) in &mut bullets {
            if !self.paused {
                if let Some(death_reason) = b.animate_bullet(&mut self.enemies, &mut self.player) {
                    bullets_to_delete.push(*i);

                    let base = b.get_base();

                    match death_reason {
                        DeathReason::Killed | DeathReason::HitPlayer => add_tent(
                            if let Projectile::Missile { .. } = b {
                                false
                            } else {
                                true
                            },
                            &base.0.pos,
                            self,
                        ),
                        _ => {}
                    }

                    if let DeathReason::HitPlayer = death_reason {
                        if self.player.invtime == 0 && !self.game_over && 0 < self.player.lives {
                            self.player.lives -= 1;
                            if self.player.lives == 0 {
                                self.game_over = true;
                            } else {
                                self.player.invtime = PLAYER_INVINCIBLE_TIME;
                            }
                        }
                    }
                }
            }
        }
        self.bullets = bullets;

        for i in bullets_to_delete.iter() {
            if let Some(b) = self.bullets.remove(i) {
                println!(
                    "Deleted {} id={}, {} / {}",
                    b.get_type(),
                    b.get_base().0.id,
                    *i,
                    self.bullets.len()
                );
            } else {
                debug_assert!(false, "All keys must exist in bullets");
            }
        }
    }

    #[cfg(feature = "webgl")]
    pub fn draw_tents(&self, gl: &GL) {
        for tent in &self.tent {
            tent.draw_temp(gl, &self.assets);
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
        let mut to_delete = vec![];
        for (i, e) in &mut ((&mut self.tent).iter_mut().enumerate()) {
            if !self.paused {
                if let Some(_) = e.animate_temp() {
                    to_delete.push(i);
                    continue;
                }
            }
        }

        for i in to_delete.iter().rev() {
            self.tent.remove(*i);
            //println!("Deleted tent {} / {}", *i, bullets.len());
        }
    }
}

#[cfg(feature = "webgl")]
//
// Initialize a texture and load an image.
// When the image finished loading copy it into the texture.
//
fn load_texture(gl: &GL, url: &str) -> Result<Rc<WebGlTexture>, JsValue> {
    fn window() -> web_sys::Window {
        web_sys::window().expect("no global `window` exists")
    }

    fn document() -> web_sys::Document {
        window().document().unwrap()
    }

    fn get_context() -> GL {
        let document = document();
        let canvas = document.get_element_by_id("canvas").unwrap();
        let canvas: web_sys::HtmlCanvasElement =
            canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();

        canvas
            .get_context("webgl")
            .unwrap()
            .unwrap()
            .dyn_into::<GL>()
            .unwrap()
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
    )
    .unwrap();
    gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_S, GL::REPEAT as i32);
    gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_T, GL::REPEAT as i32);
    gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MIN_FILTER, GL::LINEAR as i32);

    let image = Rc::new(HtmlImageElement::new().unwrap());
    let url_str = url.to_owned();
    let image_clone = image.clone();
    let texture_clone = texture.clone();
    let callback = Closure::wrap(Box::new(move || {
        console_log!("loaded image: {}", url_str);
        // web_sys::console::log_1(Date::new_0().to_locale_string("en-GB", &JsValue::undefined()));

        let gl = get_context();

        gl.bind_texture(GL::TEXTURE_2D, Some(&*texture_clone));
        gl.tex_image_2d_with_u32_and_u32_and_image(
            GL::TEXTURE_2D,
            level,
            internal_format,
            src_format,
            src_type,
            &image_clone,
        )
        .unwrap();

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
    }) as Box<dyn FnMut()>);
    image.set_onload(Some(callback.as_ref().unchecked_ref()));
    image.set_src(url);

    callback.forget();

    Ok(texture)
}

#[cfg(feature = "webgl")]
fn vertex_buffer_data(context: &GL, vertices: &[f32]) -> Result<(), JsValue> {
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
    Ok(())
}

#[cfg(feature = "webgl")]
fn enable_buffer(gl: &GL, buffer: &Option<WebGlBuffer>, elements: i32, vertex_position: u32) {
    gl.bind_buffer(GL::ARRAY_BUFFER, buffer.as_ref());
    gl.vertex_attrib_pointer_with_i32(vertex_position, elements, GL::FLOAT, false, 0, 0);
    gl.enable_vertex_attrib_array(vertex_position);
}

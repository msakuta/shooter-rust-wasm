use core::f64;

use crate::consts::*;
use crate::xor128::Xor128;
use crate::ShooterState;
#[cfg(feature = "webgl")]
use crate::{enable_buffer, load_texture, vertex_buffer_data};
#[cfg(feature = "webgl")]
use cgmath::{Matrix3, Matrix4, Rad, Vector2, Vector3};
#[cfg(all(not(feature = "webgl"), feature = "piston"))]
use piston_window::{
    draw_state::Blend,
    math::{rotate_radians, scale, translate},
    *,
};
use rotate_enum::RotateEnum;
#[cfg(all(not(feature = "webgl"), feature = "piston"))]
use std::ops::{Add, Mul};
use std::rc::Rc;
use vecmath::{vec2_add, vec2_len, vec2_normalized, vec2_scale, vec2_square_len, vec2_sub};
#[cfg(feature = "webgl")]
use wasm_bindgen::JsValue;
#[cfg(feature = "webgl")]
use web_sys::{
    Document, Element, WebGlBuffer, WebGlProgram, WebGlRenderingContext as GL, WebGlTexture,
    WebGlUniformLocation,
};

/// The base structure of all Entities.  Implements common methods.
pub struct Entity {
    pub id: u32,
    pub pos: [f64; 2],
    pub velo: [f64; 2],
    pub health: i32,
    pub rotation: f32,
    pub angular_velocity: f32,
    #[cfg(all(not(feature = "webgl"), feature = "piston"))]
    pub blend: Option<Blend>,
}

#[derive(Debug)]
pub enum DeathReason {
    RangeOut,
    Killed,
    HitPlayer,
}

#[cfg(all(not(feature = "webgl"), feature = "piston"))]
// We cannot directly define custom operators on external types, so we wrap the matrix
// int a tuple struct.
pub struct Matrix<T>(pub vecmath::Matrix2x3<T>);

#[cfg(all(not(feature = "webgl"), feature = "piston"))]
// This is such a silly way to operator overload to enable matrix multiplication with
// operator *.
impl<T> Mul for Matrix<T>
where
    T: Copy + Add<T, Output = T> + Mul<T, Output = T>,
{
    type Output = Self;
    fn mul(self, o: Self) -> Self {
        Matrix(vecmath::row_mat2x3_mul(self.0, o.0))
    }
}

impl Entity {
    pub fn new(id_gen: &mut u32, pos: [f64; 2], velo: [f64; 2]) -> Self {
        *id_gen += 1;
        Self {
            id: *id_gen,
            pos,
            velo,
            health: 1,
            rotation: 0.,
            angular_velocity: 0.,
            #[cfg(all(not(feature = "webgl"), feature = "piston"))]
            blend: None,
        }
    }

    pub fn health(mut self, health: i32) -> Self {
        self.health = health;
        self
    }

    #[cfg(all(not(feature = "webgl"), feature = "piston"))]
    pub fn blend(mut self, blend: Blend) -> Self {
        self.blend = Some(blend);
        self
    }

    pub fn rotation(mut self, rotation: f32) -> Self {
        self.rotation = rotation;
        self
    }

    /// Returns None if the Entity survived this frame.
    /// Otherwise returns Some(reason) where reason is DeathReason.
    pub fn animate(&mut self) -> Option<DeathReason> {
        let pos = &mut self.pos;
        *pos = vec2_add(*pos, self.velo);
        self.rotation += self.angular_velocity;
        if self.health <= 0 {
            Some(DeathReason::Killed)
        }
        // Only remove if the velocity is going outward
        else if pos[0] < 0. && self.velo[0] < 0.
            || (WIDTH as f64) < pos[0] && 0. < self.velo[0]
            || pos[1] < 0. && self.velo[1] < 0.
            || (HEIGHT as f64) < pos[1] && 0. < self.velo[1]
        {
            Some(DeathReason::RangeOut)
        } else {
            None
        }
    }

    #[cfg(feature = "webgl")]
    pub fn draw_tex(
        &self,
        assets: &Assets,
        context: &GL,
        texture: &WebGlTexture,
        scale: Option<[f64; 2]>,
    ) {
        let shader = assets.sprite_shader.as_ref().unwrap();
        context.bind_texture(GL::TEXTURE_2D, Some(&texture));
        let pos = &self.pos;
        let translation = Matrix4::from_translation(Vector3::new(pos[0], pos[1], 0.));
        let rotation = Matrix4::from_angle_z(Rad(self.rotation as f64));
        let scale = scale.unwrap_or([1., 1.]);
        let scale_mat = Matrix4::from_nonuniform_scale(scale[0], scale[1], 1.);
        let transform = assets.world_transform * translation * rotation * scale_mat;
        context.uniform_matrix4fv_with_f32_array(
            shader.transform_loc.as_ref(),
            false,
            <Matrix4<f32> as AsRef<[f32; 16]>>::as_ref(&transform.cast().unwrap()),
        );

        context.draw_arrays(GL::TRIANGLE_FAN, 0, 4);
    }

    #[cfg(all(not(feature = "webgl"), feature = "piston"))]
    pub fn draw_tex(
        &self,
        context: &Context,
        g: &mut G2d,
        texture: &G2dTexture,
        scale: Option<f64>,
    ) {
        let pos = &self.pos;
        let tex2 = texture;
        let scale_factor = scale.unwrap_or(1.);
        let (width, height) = (
            scale_factor * tex2.get_width() as f64,
            scale_factor * tex2.get_height() as f64,
        );
        let centerize = translate([-(width / 2.), -(height / 2.)]);
        let rotmat = rotate_radians(self.rotation as f64);
        let translate = translate(*pos);
        let draw_state = if let Some(blend_mode) = self.blend {
            context.draw_state.blend(blend_mode)
        } else {
            context.draw_state
        };
        let image = Image::new().rect([0., 0., width, height]);
        image.draw(
            tex2,
            &draw_state,
            (Matrix(context.transform) * Matrix(translate) * Matrix(rotmat) * Matrix(centerize)).0,
            g,
        );
    }

    pub fn hits_player(&self, player: &Self) -> Option<DeathReason> {
        let e = &player;
        if self.pos[0] - BULLET_SIZE < e.pos[0] + ENEMY_SIZE
            && e.pos[0] - ENEMY_SIZE < self.pos[0] + BULLET_SIZE
            && self.pos[1] - BULLET_SIZE < e.pos[1] + ENEMY_SIZE
            && e.pos[1] - ENEMY_SIZE < self.pos[1] + BULLET_SIZE
        {
            Some(DeathReason::HitPlayer)
        } else {
            None
        }
    }
}

#[derive(PartialEq, Clone, Copy, Debug, RotateEnum)]
pub enum Weapon {
    Bullet,
    Light,
    Missile,
    Lightning,
}

impl std::fmt::Display for Weapon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[test]
fn weapon_rotate() {
    use Weapon::*;
    for start in &[Bullet, Light, Missile, Lightning] {
        assert!(start.next().prev() == *start);
    }
    for start in &[Bullet, Light, Missile, Lightning] {
        assert!(start.next().next().prev().prev() == *start);
    }
    for start in &[Bullet, Light, Missile, Lightning] {
        assert!(start.next().next().next().prev().prev().prev() == *start);
    }
}

pub const WEAPON_SET: [(usize, Weapon, [f32; 3]); 4] = [
    (0, Weapon::Bullet, [1., 0.5, 0.]),
    (2, Weapon::Light, [1., 1., 1.]),
    (3, Weapon::Missile, [0., 1., 0.]),
    (4, Weapon::Lightning, [1., 1., 0.]),
];

pub struct Player {
    pub base: Entity,
    pub score: u32,
    pub kills: u32,
    pub power: u32,
    pub lives: u32,
    /// invincibility time caused by death or bomb
    pub invtime: u32,
    pub weapon: Weapon,
    pub cooldown: u32,
}

impl Player {
    pub fn new(base: Entity) -> Self {
        Self {
            base,
            score: 0,
            kills: 0,
            power: 0,
            lives: 3,
            invtime: 0,
            weapon: Weapon::Bullet,
            cooldown: 0,
        }
    }

    pub fn move_up(&mut self) {
        if PLAYER_SIZE <= self.base.pos[1] - PLAYER_SPEED {
            self.base.pos[1] -= PLAYER_SPEED;
        }
    }

    pub fn move_down(&mut self) {
        if self.base.pos[1] + PLAYER_SPEED < HEIGHT as f64 - PLAYER_SIZE {
            self.base.pos[1] += PLAYER_SPEED;
        }
    }

    pub fn move_left(&mut self) {
        if PLAYER_SIZE <= self.base.pos[0] - PLAYER_SPEED {
            self.base.pos[0] -= PLAYER_SPEED;
        }
    }

    pub fn move_right(&mut self) {
        if self.base.pos[0] + PLAYER_SPEED < WIDTH as f64 - PLAYER_SIZE {
            self.base.pos[0] += PLAYER_SPEED;
        }
    }

    pub fn reset(&mut self) {
        self.base.pos = [240., 400.];
        self.score = 0;
        self.kills = 0;
        self.power = 0;
        self.lives = PLAYER_LIVES;
        self.invtime = 0;
    }

    pub fn power_level(&self) -> u32 {
        self.power >> 4
    }

    pub fn difficulty_level(&self) -> u32 {
        self.score / 256
    }
}

pub struct EnemyBase(pub Entity, pub i32);

impl EnemyBase {
    pub fn new(id_gen: &mut u32, pos: [f64; 2], velo: [f64; 2]) -> Self {
        Self(Entity::new(id_gen, pos, velo).health(64), 0)
    }

    pub fn health(mut self, val: i32) -> Self {
        self.0 = self.0.health(val);
        self
    }
}

pub struct ShieldedBoss {
    pub base: EnemyBase,
    pub shield_health: i32,
}

impl ShieldedBoss {
    pub fn new(id_gen: &mut u32, pos: [f64; 2], velo: [f64; 2]) -> Self {
        Self {
            base: EnemyBase(Entity::new(id_gen, pos, velo).health(64), 0),
            shield_health: 64,
        }
    }
}

pub enum Enemy {
    Enemy1(EnemyBase),
    Boss(EnemyBase),
    ShieldedBoss(ShieldedBoss),
    SpiralEnemy(EnemyBase),
}

#[cfg(feature = "webgl")]
pub struct ShaderBundle {
    pub program: WebGlProgram,
    pub vertex_position: u32,
    pub tex_coord_position: u32,
    pub texture_loc: Option<WebGlUniformLocation>,
    pub transform_loc: Option<WebGlUniformLocation>,
    pub tex_transform_loc: Option<WebGlUniformLocation>,
    pub alpha_loc: Option<WebGlUniformLocation>,
}

#[cfg(feature = "webgl")]
impl ShaderBundle {
    pub fn new(gl: &GL, program: WebGlProgram) -> Self {
        let get_uniform = |location: &str| {
            let op: Option<WebGlUniformLocation> = gl.get_uniform_location(&program, location);
            if op.is_none() {
                console_log!("Warning: location {} undefined", location);
            } else {
                console_log!("location {} defined", location);
            }
            op
        };
        let vertex_position = gl.get_attrib_location(&program, "vertexData") as u32;
        let tex_coord_position = gl.get_attrib_location(&program, "vertexData") as u32;
        console_log!("vertex_position: {}", vertex_position);
        console_log!("tex_coord_position: {}", tex_coord_position);
        Self {
            vertex_position,
            tex_coord_position,
            texture_loc: get_uniform("texture"),
            transform_loc: get_uniform("transform"),
            tex_transform_loc: get_uniform("texTransform"),
            alpha_loc: get_uniform("alpha"),
            // Program has to be later than others
            program,
        }
    }
}

#[cfg(feature = "webgl")]
pub struct Assets {
    pub world_transform: Matrix4<f64>,

    pub enemy_tex: Rc<WebGlTexture>,
    pub boss_tex: Rc<WebGlTexture>,
    pub shield_tex: Rc<WebGlTexture>,
    pub spiral_enemy_tex: Rc<WebGlTexture>,
    pub player_texture: Rc<WebGlTexture>,
    pub bullet_texture: Rc<WebGlTexture>,
    pub enemy_bullet_texture: Rc<WebGlTexture>,
    pub phase_bullet_tex: Rc<WebGlTexture>,
    pub spiral_bullet_tex: Rc<WebGlTexture>,
    pub missile_tex: Rc<WebGlTexture>,
    pub red_glow_tex: Rc<WebGlTexture>,
    pub explode_tex: Rc<WebGlTexture>,
    pub explode2_tex: Rc<WebGlTexture>,
    pub trail_tex: Rc<WebGlTexture>,
    pub beam_tex: Rc<WebGlTexture>,
    pub back_tex: Rc<WebGlTexture>,
    pub power_tex: Rc<WebGlTexture>,
    pub power2_tex: Rc<WebGlTexture>,
    pub sphere_tex: Rc<WebGlTexture>,
    pub weapons_tex: Rc<WebGlTexture>,

    pub sprite_shader: Option<ShaderBundle>,
    pub trail_shader: Option<ShaderBundle>,
    pub trail_buffer: Option<WebGlBuffer>,
    pub rect_buffer: Option<WebGlBuffer>,

    pub player_live_icons: Vec<Element>,
}

#[cfg(feature = "webgl")]
impl Assets {
    pub fn new(
        document: &Document,
        context: &GL,
        image_assets: js_sys::Array,
    ) -> Result<Self, JsValue> {
        let side_panel = document.get_element_by_id("sidePanel").unwrap();

        let player_live_icons = (0..3)
            .map(|_| {
                let lives_icon = document.create_element("img")?;
                lives_icon.set_attribute(
                    "src",
                    &js_sys::Array::from(
                        &image_assets
                            .iter()
                            .find(|value| {
                                let array = js_sys::Array::from(value);
                                array.iter().next() == Some(JsValue::from_str("player"))
                            })
                            .unwrap(),
                    )
                    .to_vec()
                    .get(1)
                    .ok_or_else(|| JsValue::from_str("Couldn't find texture"))?
                    .as_string()
                    .unwrap(),
                )?;
                side_panel.append_child(&lives_icon)?;
                Ok(lives_icon)
            })
            .collect::<Result<Vec<Element>, JsValue>>()?;

        let load_texture_local = |path| -> Result<Rc<WebGlTexture>, JsValue> {
            if let Some(value) = image_assets.iter().find(|value| {
                let array = js_sys::Array::from(value);
                array.iter().next() == Some(JsValue::from_str(path))
            }) {
                let array = js_sys::Array::from(&value).to_vec();
                load_texture(
                    &context,
                    &array
                        .get(1)
                        .ok_or_else(|| JsValue::from_str("Couldn't find texture"))?
                        .as_string()
                        .ok_or_else(|| {
                            JsValue::from_str(&format!(
                                "Couldn't convert value to String: {:?}",
                                path
                            ))
                        })?,
                )
            } else {
                Err(JsValue::from_str("Couldn't find texture"))
            }
        };

        Ok(Assets {
            world_transform: Matrix4::from_translation(Vector3::new(-1., 1., 0.))
                * Matrix4::from_nonuniform_scale(2. / FWIDTH, -2. / FHEIGHT, 1.),
            enemy_tex: load_texture_local("enemy")?,
            boss_tex: load_texture_local("boss")?,
            shield_tex: load_texture_local("shield")?,
            spiral_enemy_tex: load_texture_local("spiralEnemy")?,
            player_texture: load_texture_local("player")?,
            bullet_texture: load_texture_local("bullet")?,
            enemy_bullet_texture: load_texture_local("ebullet")?,
            phase_bullet_tex: load_texture_local("phaseBullet")?,
            spiral_bullet_tex: load_texture_local("spiralBullet")?,
            missile_tex: load_texture_local("missile")?,
            red_glow_tex: load_texture_local("redGlow")?,
            explode_tex: load_texture_local("explode")?,
            explode2_tex: load_texture_local("explode2")?,
            trail_tex: load_texture_local("trail")?,
            beam_tex: load_texture_local("beam")?,
            back_tex: load_texture_local("back")?,
            power_tex: load_texture_local("power")?,
            power2_tex: load_texture_local("power2")?,
            sphere_tex: load_texture_local("sphere")?,
            weapons_tex: load_texture_local("weapons")?,
            sprite_shader: None,
            trail_shader: None,
            rect_buffer: None,
            trail_buffer: None,
            player_live_icons,
        })
    }
}

#[cfg(all(not(feature = "webgl"), feature = "piston"))]
pub struct Assets {
    pub bg: Rc<G2dTexture>,
    pub weapons_tex: Rc<G2dTexture>,
    pub boss_tex: Rc<G2dTexture>,
    pub enemy_tex: Rc<G2dTexture>,
    pub spiral_enemy_tex: Rc<G2dTexture>,
    pub player_tex: Rc<G2dTexture>,
    pub shield_tex: Rc<G2dTexture>,
    pub ebullet_tex: Rc<G2dTexture>,
    pub phase_bullet_tex: Rc<G2dTexture>,
    pub spiral_bullet_tex: Rc<G2dTexture>,
    pub bullet_tex: Rc<G2dTexture>,
    pub missile_tex: Rc<G2dTexture>,
    pub explode_tex: Rc<G2dTexture>,
    pub explode2_tex: Rc<G2dTexture>,
    pub sphere_tex: Rc<G2dTexture>,
    pub power_tex: Rc<G2dTexture>,
    pub power2_tex: Rc<G2dTexture>,
}

#[cfg(all(not(feature = "webgl"), feature = "piston"))]
impl Assets {
    pub fn new(window: &mut PistonWindow) -> (Self, Glyphs) {
        let mut exe_folder = std::env::current_exe().unwrap();
        exe_folder.pop();
        println!("exe: {:?}", exe_folder);
        let assets_loader = find_folder::Search::KidsThenParents(1, 3)
            .of(exe_folder)
            .for_folder("assets")
            .unwrap();

        let font = &assets_loader.join("FiraSans-Regular.ttf");
        let factory = window.factory.clone();
        let glyphs = Glyphs::new(font, factory, TextureSettings::new()).unwrap();

        let mut load_texture = |name| {
            Rc::new(
                Texture::from_path(
                    &mut window.factory,
                    &assets_loader.join(name),
                    Flip::None,
                    &TextureSettings::new(),
                )
                .unwrap(),
            )
        };

        (
            Self {
                bg: load_texture("back2.jpg"),
                weapons_tex: load_texture("weapons.png"),
                boss_tex: load_texture("boss.png"),
                enemy_tex: load_texture("enemy.png"),
                spiral_enemy_tex: load_texture("spiral-enemy.png"),
                player_tex: load_texture("player.png"),
                shield_tex: load_texture("shield.png"),
                ebullet_tex: load_texture("ebullet.png"),
                phase_bullet_tex: load_texture("phase-bullet.png"),
                spiral_bullet_tex: load_texture("spiral-bullet.png"),
                bullet_tex: load_texture("bullet.png"),
                missile_tex: load_texture("missile.png"),
                explode_tex: load_texture("explode.png"),
                explode2_tex: load_texture("explode2.png"),
                sphere_tex: load_texture("sphere.png"),
                power_tex: load_texture("power.png"),
                power2_tex: load_texture("power2.png"),
            },
            glyphs,
        )
    }
}

impl Enemy {
    pub fn get_base(&self) -> &Entity {
        match self {
            Enemy::Enemy1(base) | Enemy::Boss(base) | Enemy::SpiralEnemy(base) => &base.0,
            Enemy::ShieldedBoss(boss) => &boss.base.0,
        }
    }

    pub fn get_base_mut(&mut self) -> &mut EnemyBase {
        match self {
            Enemy::Enemy1(ref mut base)
            | Enemy::Boss(ref mut base)
            | Enemy::SpiralEnemy(ref mut base) => base,
            Enemy::ShieldedBoss(ref mut boss) => &mut boss.base,
        }
    }

    pub fn get_id(&self) -> u32 {
        self.get_base().id
    }

    pub fn damage(&mut self, val: i32) {
        match self {
            Enemy::Enemy1(ref mut base)
            | Enemy::Boss(ref mut base)
            | Enemy::SpiralEnemy(ref mut base) => {
                base.0.health -= val;
                console_log!("damaged: {}", base.0.health);
            }
            Enemy::ShieldedBoss(ref mut boss) => {
                if boss.shield_health < 16 {
                    boss.base.0.health -= val
                } else {
                    boss.shield_health -= val
                }
            }
        }
    }

    pub fn predicted_damage(&self) -> i32 {
        match self {
            Enemy::Enemy1(base) | Enemy::Boss(base) | Enemy::SpiralEnemy(base) => base.1,
            Enemy::ShieldedBoss(boss) => boss.base.1,
        }
    }

    pub fn add_predicted_damage(&mut self, val: i32) {
        let e = self.get_base_mut();
        e.1 += val;
    }

    pub fn total_health(&self) -> i32 {
        self.get_base().health
    }

    pub fn drop_item(&self, ent: Entity) -> Item {
        match self {
            Enemy::Enemy1(_) => Item::PowerUp(ent),
            _ => Item::PowerUp10(ent),
        }
    }

    fn gen_bullets(
        &mut self,
        id_gen: &mut u32,
        bullets: &mut std::collections::HashMap<u32, Projectile>,
        rng: &mut Xor128,
        create_fn: impl Fn(BulletBase) -> Projectile,
    ) {
        let x = rng.gen_range(0, 256);
        if x == 0 {
            use std::f64::consts::PI;
            let bullet_count = 10;
            let phase_offset = rng.gen() * PI;
            for i in 0..bullet_count {
                let angle = 2. * PI * i as f64 / bullet_count as f64 + phase_offset;
                let eb = create_fn(BulletBase(
                    Entity::new(
                        id_gen,
                        self.get_base().pos,
                        vec2_scale([angle.cos(), angle.sin()], 1.),
                    )
                    .rotation(angle as f32),
                ));
                bullets.insert(eb.get_id(), eb);
            }
        }
    }

    pub fn animate(&mut self, state: &mut ShooterState) -> Option<DeathReason> {
        if self.is_boss() {
            self.gen_bullets(
                &mut state.id_gen,
                &mut state.bullets,
                &mut state.rng,
                Projectile::new_phase,
            );
        } else if let Enemy::SpiralEnemy(_) = self {
            self.gen_bullets(
                &mut state.id_gen,
                &mut state.bullets,
                &mut state.rng,
                Projectile::new_spiral,
            );
        } else {
            let x: u32 = state.rng.gen_range(0, 64);
            if x == 0 {
                let eb = Projectile::EnemyBullet(BulletBase(Entity::new(
                    &mut state.id_gen,
                    self.get_base().pos,
                    [state.rng.gen() - 0.5, state.rng.gen() - 0.5],
                )));
                state.bullets.insert(eb.get_id(), eb);
            }
        }

        match self {
            Enemy::Enemy1(ref mut base) | Enemy::Boss(ref mut base) => base.0.animate(),
            Enemy::ShieldedBoss(ref mut boss) => {
                if boss.shield_health < 64 && state.time % 8 == 0 {
                    boss.shield_health += 1;
                }
                boss.base.0.animate()
            }
            Enemy::SpiralEnemy(ref mut base) => {
                base.0.rotation -= std::f32::consts::PI * 0.01;
                base.0.animate()
            }
        }
    }

    #[cfg(feature = "webgl")]
    pub fn draw(&self, _state: &ShooterState, gl: &GL, assets: &Assets) {
        self.get_base().draw_tex(
            assets,
            gl,
            match self {
                Enemy::Enemy1(_) => &assets.enemy_tex,
                Enemy::Boss(_) | Enemy::ShieldedBoss(_) => &assets.boss_tex,
                Enemy::SpiralEnemy(_) => &assets.spiral_enemy_tex,
            },
            Some(match self {
                Enemy::Enemy1(_) => [ENEMY_SIZE; 2],
                Enemy::Boss(_) | Enemy::ShieldedBoss(_) | Enemy::SpiralEnemy(_) => [BOSS_SIZE; 2],
            }),
        );

        if let Enemy::ShieldedBoss(boss) = self {
            self.get_base().draw_tex(
                assets,
                gl,
                &assets.shield_tex,
                Some([boss.shield_health as f64; 2]),
            );
        }
    }

    #[cfg(all(not(feature = "webgl"), feature = "piston"))]
    pub fn draw(&self, context: &Context, g: &mut G2d, assets: &Assets) {
        self.get_base().draw_tex(
            context,
            g,
            match self {
                Enemy::Enemy1(_) => &assets.enemy_tex,
                Enemy::Boss(_) | Enemy::ShieldedBoss(_) => &assets.boss_tex,
                Enemy::SpiralEnemy(_) => &assets.spiral_enemy_tex,
            },
            if let Enemy::SpiralEnemy(_) = self {
                Some(0.5)
            } else {
                None
            },
        );
        if let Enemy::ShieldedBoss(ref boss) = self {
            let pos = &boss.base.0.pos;
            let tex2 = &*assets.shield_tex;
            let centerize = translate([
                -(tex2.get_width() as f64 / 2.),
                -(tex2.get_height() as f64 / 2.),
            ]);
            let rotmat = rotate_radians(0 as f64);
            let scalemat = scale(
                boss.shield_health as f64 / 64.,
                boss.shield_health as f64 / 64.,
            );
            let translate = translate(*pos);
            let draw_state = context.draw_state;
            let image =
                Image::new().rect([0., 0., tex2.get_width() as f64, tex2.get_height() as f64]);
            image.draw(
                tex2,
                &draw_state,
                (Matrix(context.transform)
                    * Matrix(translate)
                    * Matrix(scalemat)
                    * Matrix(rotmat)
                    * Matrix(centerize))
                .0,
                g,
            );
        }
    }

    pub fn test_hit(&self, rect: [f64; 4]) -> bool {
        let rect2 = self.get_bb();
        rect[0] < rect2[2] && rect2[0] < rect[2] && rect[1] < rect2[3] && rect2[1] < rect[3]
    }

    pub fn get_bb(&self) -> [f64; 4] {
        let size = if let Enemy::ShieldedBoss(boss) = self {
            boss.shield_health as f64
        } else {
            ENEMY_SIZE
        };
        let e = self.get_base();
        [
            e.pos[0] - size,
            e.pos[1] - size,
            e.pos[0] + size,
            e.pos[1] + size,
        ]
    }

    pub fn is_boss(&self) -> bool {
        matches!(self, Enemy::Boss(_) | Enemy::ShieldedBoss(_))
    }

    pub fn new_spiral(id_gen: &mut u32, pos: [f64; 2], velo: [f64; 2]) -> Enemy {
        Enemy::SpiralEnemy(EnemyBase::new(id_gen, pos, velo))
    }
}

pub struct BulletBase(pub Entity);

pub enum Projectile {
    Bullet(BulletBase),
    EnemyBullet(BulletBase),
    PhaseBullet {
        base: BulletBase,
        velo: [f64; 2],
        phase: f64,
    },
    SpiralBullet {
        base: BulletBase,
        speed: f64,
        traveled: f64,
    },
    Missile {
        base: BulletBase,
        target: u32,
        trail: Vec<[f64; 2]>,
    },
}

const MISSILE_DETECTION_RANGE: f64 = 256.;
const MISSILE_HOMING_SPEED: f64 = 0.25;
#[cfg(feature = "webgl")]
const MISSILE_TRAIL_WIDTH: f64 = 5.;
const MISSILE_TRAIL_LENGTH: usize = 20;
const MISSILE_DAMAGE: i32 = 5;

impl Projectile {
    pub fn new_phase(base: BulletBase) -> Projectile {
        let velo = base.0.velo;
        Projectile::PhaseBullet {
            base,
            velo,
            phase: 0.,
        }
    }

    pub fn new_spiral(base: BulletBase) -> Projectile {
        let speed = vec2_len(base.0.velo);
        Projectile::SpiralBullet {
            base,
            speed,
            traveled: 0.,
        }
    }

    pub fn get_base(&self) -> &BulletBase {
        match &self {
            &Projectile::Bullet(base) | &Projectile::EnemyBullet(base) => base,
            &Projectile::PhaseBullet { base, .. } | &Projectile::SpiralBullet { base, .. } => base,
            &Projectile::Missile { base, .. } => base,
        }
    }

    pub fn get_id(&self) -> u32 {
        self.get_base().0.id
    }

    pub fn get_type(&self) -> &str {
        match &self {
            &Projectile::Bullet(_) | &Projectile::EnemyBullet(_) => "Bullet",
            &Projectile::PhaseBullet { .. } => "PhaseBullet",
            &Projectile::SpiralBullet { .. } => "SpiralBullet",
            &Projectile::Missile { .. } => "Missile",
        }
    }

    fn animate_player_bullet(
        mut base: &mut BulletBase,
        enemies: &mut Vec<Enemy>,
        mut _player: &mut Player,
    ) -> Option<DeathReason> {
        let bbox = Self::get_bb_base(base);
        let &mut BulletBase(ent) = &mut base;
        for enemy in enemies.iter_mut() {
            if enemy.test_hit(bbox) {
                enemy.damage(ent.health);
                ent.health = 0;
                break;
            }
        }
        ent.animate()
    }

    fn animate_enemy_bullet(
        base: &mut BulletBase,
        _enemies: &mut Vec<Enemy>,
        player: &mut Player,
    ) -> Option<DeathReason> {
        let BulletBase(ref mut ent) = base;
        if let Some(death_reason) = ent.hits_player(&player.base) {
            player.base.health -= ent.health;
            return Some(death_reason);
        }
        ent.animate()
    }

    pub fn animate_bullet(
        &mut self,
        enemies: &mut Vec<Enemy>,
        player: &mut Player,
    ) -> Option<DeathReason> {
        match self {
            Projectile::Bullet(base) => Self::animate_player_bullet(base, enemies, player),
            Projectile::EnemyBullet(base) => Self::animate_enemy_bullet(base, enemies, player),
            Projectile::PhaseBullet { base, velo, phase } => {
                base.0.velo = vec2_scale(*velo, (phase.sin() + 1.) / 2.);
                *phase += 0.02 * std::f64::consts::PI;
                Self::animate_enemy_bullet(base, enemies, player)
            }
            Projectile::SpiralBullet {
                base,
                speed,
                traveled,
            } => {
                let rotation =
                    base.0.rotation as f64 - 0.02 * std::f64::consts::PI / (*traveled * 0.05 + 1.);
                base.0.rotation = rotation as f32;
                base.0.velo = vec2_scale([rotation.cos(), rotation.sin()], *speed);
                *traveled += *speed;
                Self::animate_enemy_bullet(base, enemies, player)
            }
            Projectile::Missile {
                base,
                target,
                trail,
            } => {
                if *target == 0 {
                    let best = enemies.iter_mut().fold((0, 1e5, None), |bestpair, enemy| {
                        let e = enemy.get_base();
                        let dist = vec2_len(vec2_sub(base.0.pos, e.pos));
                        if dist < MISSILE_DETECTION_RANGE
                            && dist < bestpair.1
                            && enemy.predicted_damage() < enemy.total_health()
                        {
                            (e.id, dist, Some(enemy))
                        } else {
                            bestpair
                        }
                    });
                    *target = best.0;
                    if let Some(enemy) = best.2 {
                        enemy.add_predicted_damage(MISSILE_DAMAGE);
                        println!(
                            "Add predicted damage: {} -> {}",
                            enemy.predicted_damage() - MISSILE_DAMAGE,
                            enemy.predicted_damage()
                        );
                    }
                } else if let Some(target_enemy) = enemies.iter().find(|e| e.get_id() == *target) {
                    let target_ent = target_enemy.get_base();
                    let norm = vec2_normalized(vec2_sub(target_ent.pos, base.0.pos));
                    let desired_velo = vec2_scale(norm, MISSILE_SPEED);
                    let desired_diff = vec2_sub(desired_velo, base.0.velo);
                    if std::f64::EPSILON < vec2_square_len(desired_diff) {
                        base.0.velo = if vec2_square_len(desired_diff)
                            < MISSILE_HOMING_SPEED * MISSILE_HOMING_SPEED
                        {
                            desired_velo
                        } else {
                            let desired_diff_norm = vec2_normalized(desired_diff);
                            vec2_add(
                                base.0.velo,
                                vec2_scale(desired_diff_norm, MISSILE_HOMING_SPEED),
                            )
                        };
                        let angle = base.0.velo[1].atan2(base.0.velo[0]);
                        base.0.rotation = (angle + std::f64::consts::FRAC_PI_2) as f32;
                        let (s, c) = angle.sin_cos();
                        base.0.velo[0] = MISSILE_SPEED * c;
                        base.0.velo[1] = MISSILE_SPEED * s;
                    }
                } else {
                    *target = 0
                }
                if MISSILE_TRAIL_LENGTH < trail.len() {
                    trail.remove(0);
                }
                trail.push(base.0.pos);
                let res = Self::animate_player_bullet(base, enemies, player);
                if res.is_some() {
                    if let Some(target_enemy) = enemies.iter_mut().find(|e| e.get_id() == *target) {
                        target_enemy.add_predicted_damage(-MISSILE_DAMAGE);
                        println!(
                            "Reduce predicted damage: {} -> {}",
                            target_enemy.predicted_damage() + MISSILE_DAMAGE,
                            target_enemy.predicted_damage()
                        );
                    }
                }
                res
            }
        }
    }

    pub fn get_bb_base(base: &BulletBase) -> [f64; 4] {
        let e = &base.0;
        [
            e.pos[0] - BULLET_SIZE,
            e.pos[1] - BULLET_SIZE,
            e.pos[0] + BULLET_SIZE,
            e.pos[1] + BULLET_SIZE,
        ]
    }

    #[cfg(feature = "webgl")]
    pub fn draw(&self, gl: &GL, assets: &Assets) {
        if let Projectile::Bullet(base) = self {
            if let Some(ref shader) = assets.sprite_shader.as_ref() {
                gl.blend_equation(GL::FUNC_ADD);
                gl.blend_func(GL::SRC_ALPHA, GL::ONE);
                gl.uniform1f(shader.alpha_loc.as_ref(), 0.15);

                base.0.draw_tex(
                    assets,
                    gl,
                    &assets.red_glow_tex,
                    Some([BULLET_SIZE * 4.; 2]),
                );

                gl.blend_equation(GL::FUNC_ADD);
                gl.blend_func(GL::SRC_ALPHA, GL::ONE_MINUS_SRC_ALPHA);
                gl.uniform1f(shader.alpha_loc.as_ref(), 1.);
            }
        }
        if let Projectile::Missile { trail, .. } = self {
            let shader = assets.trail_shader.as_ref().unwrap();
            gl.use_program(Some(&shader.program));

            gl.bind_texture(GL::TEXTURE_2D, Some(&assets.trail_tex));
            gl.enable(GL::BLEND);
            gl.blend_equation(GL::FUNC_ADD);
            gl.blend_func(GL::SRC_ALPHA, GL::ONE_MINUS_SRC_ALPHA);

            enable_buffer(gl, &assets.trail_buffer, 4, shader.vertex_position);

            let vertices = trail.iter().zip(trail.iter().skip(1)).enumerate().fold(
                vec![],
                |mut acc, (i, (prev_node, this_node))| {
                    let delta = vec2_normalized(vec2_sub(*this_node, *prev_node));
                    let perp = vec2_scale(
                        [delta[1], -delta[0]],
                        MISSILE_TRAIL_WIDTH * i as f64 / MISSILE_TRAIL_LENGTH as f64,
                    );
                    let top = vec2_add(*prev_node, perp);
                    let bottom = vec2_sub(*prev_node, perp);
                    acc.extend_from_slice(&[
                        top[0] as f32,
                        top[1] as f32,
                        i as f32 / trail.len() as f32,
                        -0.1,
                    ]);
                    acc.extend_from_slice(&[
                        bottom[0] as f32,
                        bottom[1] as f32,
                        i as f32 / trail.len() as f32,
                        1.1,
                    ]);
                    acc
                },
            );

            vertex_buffer_data(gl, &vertices);

            gl.uniform_matrix4fv_with_f32_array(
                shader.transform_loc.as_ref(),
                false,
                <Matrix4<f32> as AsRef<[f32; 16]>>::as_ref(&assets.world_transform.cast().unwrap()),
            );

            gl.uniform_matrix3fv_with_f32_array(
                shader.tex_transform_loc.as_ref(),
                false,
                <Matrix3<f32> as AsRef<[f32; 9]>>::as_ref(&Matrix3::from_scale(1.)),
            );

            gl.draw_arrays(GL::TRIANGLE_STRIP, 0, (vertices.len() / 4) as i32);

            // Switch back to sprite shader and buffer
            gl.use_program(assets.sprite_shader.as_ref().map(|o| &o.program));
            enable_buffer(gl, &assets.rect_buffer, 2, shader.vertex_position);
        }
        use Projectile::*;
        self.get_base().0.draw_tex(
            assets,
            gl,
            match self {
                Bullet(_) => &assets.bullet_texture,
                EnemyBullet(_) => &assets.enemy_bullet_texture,
                PhaseBullet { .. } => &assets.phase_bullet_tex,
                SpiralBullet { .. } => &assets.spiral_bullet_tex,
                Missile { .. } => &assets.missile_tex,
            },
            Some(match self {
                Bullet(_) | EnemyBullet(_) | Missile { .. } => [BULLET_SIZE; 2],
                PhaseBullet { .. } | SpiralBullet { .. } => LONG_BULLET_SIZE,
            }),
        );
    }

    #[cfg(all(not(feature = "webgl"), feature = "piston"))]
    pub fn draw(&self, c: &Context, g: &mut G2d, assets: &Assets) {
        if let Projectile::Missile {
            base: _,
            target: _,
            trail,
        } = self
        {
            let mut iter = trail.iter().enumerate();
            if let Some(mut prev) = iter.next() {
                for e in iter {
                    line(
                        [0.75, 0.75, 0.75, e.0 as f32 / MISSILE_TRAIL_LENGTH as f32],
                        e.0 as f64 / MISSILE_TRAIL_LENGTH as f64,
                        [prev.1[0], prev.1[1], e.1[0], e.1[1]],
                        c.transform,
                        g,
                    );
                    prev = e;
                }
            }
        }
        self.get_base().0.draw_tex(
            c,
            g,
            match self {
                Projectile::Bullet(_) => &assets.bullet_tex,
                Projectile::EnemyBullet(_) => &assets.ebullet_tex,
                Projectile::PhaseBullet { .. } => &assets.phase_bullet_tex,
                Projectile::SpiralBullet { .. } => &assets.spiral_bullet_tex,
                Projectile::Missile { .. } => &assets.missile_tex,
            },
            None,
        );
    }
}

pub enum Item {
    PowerUp(Entity),
    PowerUp10(Entity),
}

impl Item {
    pub fn get_base(&self) -> &Entity {
        match self {
            Item::PowerUp(ent) | Item::PowerUp10(ent) => ent,
        }
    }

    #[cfg(feature = "webgl")]
    pub fn draw(&self, gl: &GL, assets: &Assets) {
        match self {
            Item::PowerUp(item) => {
                item.draw_tex(&assets, gl, &assets.power_tex, Some([ITEM_SIZE; 2]))
            }
            Item::PowerUp10(item) => {
                item.draw_tex(&assets, gl, &assets.power2_tex, Some([ITEM2_SIZE; 2]))
            }
        }
    }

    #[cfg(all(not(feature = "webgl"), feature = "piston"))]
    pub fn draw(&self, c: &Context, g: &mut G2d, assets: &Assets) {
        match self {
            Item::PowerUp(item) => item.draw_tex(c, g, &assets.power_tex, None),
            Item::PowerUp10(item) => item.draw_tex(c, g, &assets.power2_tex, None),
        }
    }

    pub fn power_value(&self) -> u32 {
        match self {
            Item::PowerUp(_) => 1,
            Item::PowerUp10(_) => 10,
        }
    }

    pub fn animate(&mut self, player: &mut Player) -> Option<DeathReason> {
        match self {
            Item::PowerUp(ent) | Item::PowerUp10(ent) => {
                if ent.hits_player(&player.base).is_some() {
                    player.power += self.power_value();
                    return Some(DeathReason::Killed);
                }
                ent.animate()
            }
        }
    }
}

#[cfg(feature = "webgl")]
pub struct TempEntity {
    pub base: Entity,
    pub texture: Rc<WebGlTexture>,
    pub max_frames: u32,
    pub width: u32,
    pub playback_rate: u32,
    pub image_width: u32,
    pub size: f64,
}

#[cfg(all(not(feature = "webgl"), feature = "piston"))]
pub struct TempEntity {
    pub base: Entity,
    pub texture: Rc<G2dTexture>,
    pub max_frames: u32,
    pub width: u32,
    pub playback_rate: u32,
}

#[cfg(feature = "webgl")]
impl TempEntity {
    #[allow(dead_code)]
    pub fn max_frames(mut self, max_frames: u32) -> Self {
        self.max_frames = max_frames;
        self
    }
    pub fn animate_temp(&mut self) -> Option<DeathReason> {
        self.base.health -= 1;
        self.base.animate()
    }

    pub fn draw_temp(&self, context: &GL, assets: &Assets) {
        let shader = assets.sprite_shader.as_ref().unwrap();
        let pos = &self.base.pos;
        context.bind_texture(GL::TEXTURE_2D, Some(&self.texture));
        let rotation = Matrix4::from_angle_z(Rad(self.base.rotation as f64));
        let translation = Matrix4::from_translation(Vector3::new(pos[0], pos[1], 0.));
        let scale = Matrix4::from_scale(self.size);
        let frame = self.max_frames - (self.base.health as u32 / self.playback_rate) as u32;
        // let image   = Image::new().rect([0f64, 0f64, self.width as f64, tex2.get_height() as f64])
        //     .src_rect([frame as f64 * self.width as f64, 0., self.width as f64, tex2.get_height() as f64]);
        let transform = assets.world_transform * translation * rotation * scale;
        context.uniform_matrix4fv_with_f32_array(
            shader.transform_loc.as_ref(),
            false,
            <Matrix4<f32> as AsRef<[f32; 16]>>::as_ref(&transform.cast().unwrap()),
        );

        let tex_translate = Matrix3::from_translation(Vector2::new(frame as f32, 0.));
        let tex_scale =
            Matrix3::from_nonuniform_scale(self.width as f32 / self.image_width as f32, 1.);
        context.uniform_matrix3fv_with_f32_array(
            shader.tex_transform_loc.as_ref(),
            false,
            <Matrix3<f32> as AsRef<[f32; 9]>>::as_ref(&(tex_scale * tex_translate)),
        );

        context.draw_arrays(GL::TRIANGLE_FAN, 0, 4);
    }
}

#[cfg(all(not(feature = "webgl"), feature = "piston"))]
impl TempEntity {
    #[allow(dead_code)]
    pub fn max_frames(mut self, max_frames: u32) -> Self {
        self.max_frames = max_frames;
        self
    }
    pub fn animate_temp(&mut self) -> Option<DeathReason> {
        self.base.health -= 1;
        self.base.animate()
    }

    pub fn draw_temp(&self, context: &Context, g: &mut G2d) {
        let pos = &self.base.pos;
        let tex2 = &*self.texture;
        let centerize = translate([-(16. / 2.), -(tex2.get_height() as f64 / 2.)]);
        let rotmat = rotate_radians(self.base.rotation as f64);
        let translate = translate(*pos);
        let frame = self.max_frames - (self.base.health as u32 / self.playback_rate) as u32;
        let draw_state = if let Some(blend_mode) = self.base.blend {
            context.draw_state.blend(blend_mode)
        } else {
            context.draw_state
        };
        let image = Image::new()
            .rect([0f64, 0f64, self.width as f64, tex2.get_height() as f64])
            .src_rect([
                frame as f64 * self.width as f64,
                0.,
                self.width as f64,
                tex2.get_height() as f64,
            ]);
        image.draw(
            tex2,
            &draw_state,
            (Matrix(context.transform) * Matrix(translate) * Matrix(rotmat) * Matrix(centerize)).0,
            g,
        );
    }
}

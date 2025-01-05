mod enemy;
mod projectile;
mod temp_entity;

use core::f64;

pub use self::{
    enemy::{Enemy, EnemyBase, ShieldedBoss},
    projectile::{BulletBase, Projectile},
    temp_entity::{TempEntity, TempEntityType},
};
#[cfg(all(not(feature = "webgl"), feature = "piston"))]
use crate::assets_piston::Assets;
#[cfg(feature = "webgl")]
use crate::assets_webgl::Assets;
use crate::consts::*;
#[cfg(feature = "webgl")]
use cgmath::{Matrix4, Rad, Vector3};
#[cfg(all(not(feature = "webgl"), feature = "piston"))]
use piston_window::{
    draw_state::Blend,
    math::{rotate_radians, translate},
    *,
};
use rotate_enum::RotateEnum;
use std::ops::Deref;
#[cfg(all(not(feature = "webgl"), feature = "piston"))]
use std::ops::{Add, Mul};
use vecmath::vec2_add;
#[cfg(feature = "webgl")]
use web_sys::{WebGlRenderingContext as GL, WebGlTexture};

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

#[derive(Clone, Copy, Debug)]
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
        draw_tex(
            &self.pos,
            self.rotation as f64,
            assets,
            context,
            texture,
            scale,
        );
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

#[cfg(feature = "webgl")]
pub fn draw_tex(
    pos: &[f64; 2],
    rotation: f64,
    assets: &Assets,
    context: &GL,
    texture: &WebGlTexture,
    scale: Option<[f64; 2]>,
) {
    let shader = assets.sprite_shader.as_ref().unwrap();
    context.bind_texture(GL::TEXTURE_2D, Some(&texture));
    let translation = Matrix4::from_translation(Vector3::new(pos[0], pos[1], 0.));
    let rotation = Matrix4::from_angle_z(Rad(rotation));
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

pub enum Item {
    PowerUp(Entity),
    PowerUp10(Entity),
}

impl Deref for Item {
    type Target = Entity;
    fn deref(&self) -> &Entity {
        match self {
            Item::PowerUp(ent) | Item::PowerUp10(ent) => ent,
        }
    }
}

impl Item {
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

fn bounding_box(pos: &[f64; 2], size: f64) -> [f64; 4] {
    [pos[0] - size, pos[1] - size, pos[0] + size, pos[1] + size]
}

fn bbox_intersects(rect: &[f64; 4], rect2: &[f64; 4]) -> bool {
    rect[0] < rect2[2] && rect2[0] < rect[2] && rect[1] < rect2[3] && rect2[1] < rect[3]
}

use core::f64;

use crate::consts::*;
use crate::ShooterState;
use cgmath::{Matrix3, Matrix4, Rad, Vector2, Vector3};
use std::rc::Rc;
use web_sys::{
    WebGlBuffer, WebGlProgram, WebGlRenderingContext as GL, WebGlTexture, WebGlUniformLocation,
};

/// The base structure of all Entities.  Implements common methods.
pub struct Entity {
    pub id: u32,
    pub pos: [f64; 2],
    pub velo: [f64; 2],
    pub health: i32,
    pub rotation: f32,
    pub angular_velocity: f32,
}

#[derive(Debug)]
pub enum DeathReason {
    RangeOut,
    Killed,
    HitPlayer,
}

impl Entity {
    pub fn new(id_gen: &mut u32, pos: [f64; 2], velo: [f64; 2]) -> Self {
        *id_gen += 1;
        Self {
            id: *id_gen,
            pos: pos,
            velo: velo,
            health: 1,
            rotation: 0.,
            angular_velocity: 0.,
        }
    }

    pub fn health(mut self, health: i32) -> Self {
        self.health = health;
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
        for (i, size) in (0..2).zip([WIDTH, HEIGHT].iter()) {
            pos[i] += self.velo[i];
        }
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

    pub fn draw_tex(
        &self,
        assets: &Assets,
        context: &GL,
        texture: &WebGlTexture,
        scale: Option<f64>,
    ) {
        context.bind_texture(GL::TEXTURE_2D, Some(&texture));
        let pos = &self.pos;
        let translation = Matrix4::from_translation(Vector3::new(pos[0], pos[1], 0.));
        let scale_mat = Matrix4::from_scale(scale.unwrap_or(1.));
        let rotation = Matrix4::from_angle_z(Rad(self.rotation as f64));
        let transform = assets.world_transform * &translation * &scale_mat * &rotation;
        context.uniform_matrix4fv_with_f32_array(
            assets.transform_loc.as_ref(),
            false,
            <Matrix4<f32> as AsRef<[f32; 16]>>::as_ref(&transform.cast().unwrap()),
        );

        context.draw_arrays(GL::TRIANGLE_FAN, 0, 4);
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

pub struct Player {
    pub base: Entity,
    pub score: u32,
    pub kills: u32,
    pub power: u32,
    // pub lives: u32,
    // /// invincibility time caused by death or bomb
    // pub invtime: u32,
    pub cooldown: u32,
}

impl Player {
    pub fn new(base: Entity) -> Self {
        Self {
            base,
            score: 0,
            kills: 0,
            power: 0,
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
        // self.lives = PLAYER_LIVES;
        // self.invtime = 0;
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

pub enum Enemy {
    Enemy1(EnemyBase),
    Boss(EnemyBase),
}

pub struct Assets {
    pub world_transform: Matrix4<f64>,

    pub enemy_tex: Rc<WebGlTexture>,
    pub boss_tex: Rc<WebGlTexture>,
    pub player_texture: Rc<WebGlTexture>,
    pub bullet_texture: Rc<WebGlTexture>,
    pub enemy_bullet_texture: Rc<WebGlTexture>,
    pub explode_tex: Rc<WebGlTexture>,
    pub explode2_tex: Rc<WebGlTexture>,

    pub sprite_shader: Option<WebGlProgram>,
    pub animated_sprite_shader: Option<WebGlProgram>,
    pub rect_buffer: Option<WebGlBuffer>,
    pub vertex_position: u32,
    pub texture_loc: Option<WebGlUniformLocation>,
    pub transform_loc: Option<WebGlUniformLocation>,
    pub tex_transform_loc: Option<WebGlUniformLocation>,
}

impl Enemy {
    pub fn get_base(&self) -> &Entity {
        match self {
            Enemy::Enemy1(base) | Enemy::Boss(base) => &base.0,
        }
    }

    pub fn get_base_mut(&mut self) -> &mut EnemyBase {
        match self {
            Enemy::Enemy1(ref mut base) | Enemy::Boss(ref mut base) => base,
        }
    }

    pub fn damage(&mut self, val: i32) {
        match self {
            Enemy::Enemy1(ref mut base) | Enemy::Boss(ref mut base) => {
                base.0.health -= val;
                console_log!("damaged: {}", base.0.health);
            }
        }
    }

    pub fn animate(&mut self, state: &mut ShooterState) -> Option<DeathReason> {
        let x: u32 = state.rng.gen_range(0, 64);
        if x == 0 {
            let eb = Projectile::EnemyBullet(BulletBase(Entity::new(
                &mut state.id_gen,
                self.get_base().pos,
                [state.rng.next() - 0.5, state.rng.next() - 0.5],
            )));
            state.bullets.insert(eb.get_id(), eb);
        }

        match self {
            Enemy::Enemy1(ref mut base) | Enemy::Boss(ref mut base) => base.0.animate(),
        }
    }

    pub fn draw(&self, state: &ShooterState, context: &GL, assets: &Assets) {
        self.get_base().draw_tex(
            assets,
            context,
            match self {
                Enemy::Enemy1(_) => &assets.enemy_tex,
                Enemy::Boss(_) => &assets.boss_tex,
            },
            Some(match self {
                Enemy::Enemy1(_) => ENEMY_SIZE,
                Enemy::Boss(_) => BOSS_SIZE,
            }),
        );
    }

    pub fn test_hit(&self, rect: [f64; 4]) -> bool {
        let rect2 = self.get_bb();
        rect[0] < rect2[2] && rect2[0] < rect[2] && rect[1] < rect2[3] && rect2[1] < rect[3]
    }

    pub fn get_bb(&self) -> [f64; 4] {
        let size = ENEMY_SIZE;
        let e = self.get_base();
        [
            e.pos[0] - size,
            e.pos[1] - size,
            e.pos[0] + size,
            e.pos[1] + size,
        ]
    }

    pub fn is_boss(&self) -> bool {
        match self {
            Enemy::Boss(_) => true,
            _ => false,
        }
    }
}

pub struct BulletBase(pub Entity);

pub enum Projectile {
    Bullet(BulletBase),
    EnemyBullet(BulletBase),
}

impl Projectile {
    pub fn get_base<'b>(&'b self) -> &'b BulletBase {
        match &self {
            &Projectile::Bullet(base) | &Projectile::EnemyBullet(base) => base,
        }
    }

    pub fn get_id(&self) -> u32 {
        self.get_base().0.id
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

    pub fn draw(&self, state: &ShooterState, c: &GL, assets: &Assets) {
        self.get_base().0.draw_tex(
            assets,
            c,
            match self {
                Projectile::Bullet(_) => &assets.bullet_texture,
                Projectile::EnemyBullet(_) => &assets.enemy_bullet_texture,
            },
            Some(BULLET_SIZE),
        );
    }
}

pub struct TempEntity {
    pub base: Entity,
    pub texture: Rc<WebGlTexture>,
    pub max_frames: u32,
    pub width: u32,
    pub playback_rate: u32,
}

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
        let pos = &self.base.pos;
        context.bind_texture(GL::TEXTURE_2D, Some(&self.texture));
        let rotation = Matrix4::from_angle_z(Rad(self.base.rotation as f64));
        let translation = Matrix4::from_translation(Vector3::new(pos[0], pos[1], 0.));
        let scale = Matrix4::from_scale(EXPLODE_SIZE);
        let frame = self.max_frames - (self.base.health as u32 / self.playback_rate) as u32;
        // let image   = Image::new().rect([0f64, 0f64, self.width as f64, tex2.get_height() as f64])
        //     .src_rect([frame as f64 * self.width as f64, 0., self.width as f64, tex2.get_height() as f64]);
        let transform = assets.world_transform * &translation * &rotation * &scale;
        context.uniform_matrix4fv_with_f32_array(
            assets.transform_loc.as_ref(),
            false,
            <Matrix4<f32> as AsRef<[f32; 16]>>::as_ref(&transform.cast().unwrap()),
        );

        let tex_translate = Matrix3::from_translation(Vector2::new((frame) as f32, 0.));
        let tex_scale = Matrix3::from_nonuniform_scale(1. / self.max_frames as f32, 1.);
        context.uniform_matrix3fv_with_f32_array(
            assets.tex_transform_loc.as_ref(),
            false,
            <Matrix3<f32> as AsRef<[f32; 9]>>::as_ref(&(tex_scale * tex_translate)),
        );

        context.draw_arrays(GL::TRIANGLE_FAN, 0, 4);
    }
}

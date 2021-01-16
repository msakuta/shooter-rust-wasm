use core::f64;

use crate::consts::*;
use crate::ShooterState;
use cgmath::{Matrix4, Rad, Vector3};
use web_sys::{WebGlRenderingContext as GL, WebGlTexture};

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
        fn wrap(v: f64, size: f64) -> f64 {
            v - (v / size).floor() * size
        }

        let pos = &mut self.pos;
        for (i, size) in (0..2).zip([WIDTH, HEIGHT].iter()) {
            pos[i] = wrap(pos[i] + self.velo[i], *size as f64);
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
        state: &ShooterState,
        context: &GL,
        texture: &WebGlTexture,
        scale: Option<f64>,
    ) {
        let pos = &self.pos;
        let translation = Matrix4::from_translation(Vector3::new(pos[0], pos[1], 0.));
        let scale_mat = Matrix4::from_scale(scale.unwrap_or(1.));
        let rotation = Matrix4::from_angle_z(Rad(self.rotation as f64));
        let flip = Matrix4::from_nonuniform_scale(1., -1., 1.);
        let transform = state.world_transform * &translation * &scale_mat * &rotation * &flip;
        context.uniform_matrix4fv_with_f32_array(
            state.transform_loc.as_ref(),
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
    // pub score: u32,
    // pub kills: u32,
    // pub power: u32,
    // pub lives: u32,
    // /// invincibility time caused by death or bomb
    // pub invtime: u32,
    // pub cooldown: u32,
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
}

type Assets = ();

impl Enemy {
    pub fn get_base(&self) -> &Entity {
        match &self {
            &Enemy::Enemy1(base) => &base.0,
        }
    }

    pub fn get_base_mut(&mut self) -> &mut EnemyBase {
        match self {
            Enemy::Enemy1(ref mut base) => base,
        }
    }

    pub fn draw(&self, state: &ShooterState, context: &GL, assets: &Assets) {
        self.get_base().draw_tex(
            state,
            context,
            match self {
                Enemy::Enemy1(_) => &state.texture,
            },
            Some(ENEMY_SIZE),
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

    pub fn animate(&mut self, state: &ShooterState) -> Option<DeathReason> {
        match self {
            Enemy::Enemy1(ref mut base) => base.0.animate(),
        }
    }
}

pub struct BulletBase(pub Entity);

pub enum Projectile {
    Bullet(BulletBase),
}

impl Projectile {
    pub fn get_base<'b>(&'b self) -> &'b BulletBase {
        match &self {
            &Projectile::Bullet(base) => base,
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
                // enemy.damage(ent.health);
                ent.health = 0;
                break;
            }
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
            state,
            c,
            match self {
                Projectile::Bullet(_) => &state.bullet_texture,
            },
            Some(BULLET_SIZE),
        );
    }
}

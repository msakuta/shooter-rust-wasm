use crate::consts::*;
use crate::ShooterState;
use cgmath::{Matrix4, Rad, Vector3};
use web_sys::{
    WebGlBuffer, WebGlProgram, WebGlRenderingContext as GL, WebGlShader, WebGlTexture,
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
}

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
            let size2 = size * 2.;
            v - ((v + size) / size2).floor() * size2
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
        let scale_mat = Matrix4::from_scale(scale.unwrap_or(1.) as f32);
        let rotation = Matrix4::from_angle_z(Rad(self.rotation));
        let translation = Matrix4::from_translation(Vector3::new(pos[0] as f32, pos[1] as f32, 0.));
        let transform = &scale_mat * &translation * &rotation;
        context.uniform_matrix4fv_with_f32_array(
            state.transform_loc.as_ref(),
            false,
            <Matrix4<f32> as AsRef<[f32; 16]>>::as_ref(&transform),
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
            Some(0.1),
        );
    }

    pub fn animate(&mut self, state: &ShooterState) -> Option<DeathReason> {
        match self {
            Enemy::Enemy1(ref mut base) => base.0.animate(),
        }
    }
}

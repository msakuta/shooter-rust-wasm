#[cfg(all(not(feature = "webgl"), feature = "piston"))]
use crate::assets_piston::Assets;
#[cfg(feature = "webgl")]
use crate::assets_webgl::Assets;
#[cfg(feature = "webgl")]
use cgmath::{Matrix3, Matrix4};
use std::ops::Deref;
#[cfg(feature = "webgl")]
use web_sys::WebGlRenderingContext as GL;

#[cfg(feature = "webgl")]
use crate::{enable_buffer, vertex_buffer_data};
use vecmath::{vec2_add, vec2_len, vec2_normalized, vec2_scale, vec2_square_len, vec2_sub};

#[cfg(all(not(feature = "webgl"), feature = "piston"))]
use piston_window::*;

use super::{DeathReason, Enemy, Entity, Player, BULLET_SIZE, MISSILE_SPEED};

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

impl Deref for Projectile {
    type Target = Entity;
    fn deref(&self) -> &Entity {
        match &self {
            &Projectile::Bullet(base) | &Projectile::EnemyBullet(base) => &base.0,
            &Projectile::PhaseBullet { base, .. } | &Projectile::SpiralBullet { base, .. } => {
                &base.0
            }
            &Projectile::Missile { base, .. } => &base.0,
        }
    }
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

    pub fn get_id(&self) -> u32 {
        self.id
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
                enemy.damage(ent.health, &bbox);
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
                        let dist = vec2_len(vec2_sub(base.0.pos, enemy.pos));
                        if dist < MISSILE_DETECTION_RANGE
                            && dist < bestpair.1
                            && enemy.predicted_damage() < enemy.total_health()
                        {
                            (enemy.id, dist, Some(enemy))
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
                    let norm = vec2_normalized(vec2_sub(target_enemy.pos, base.0.pos));
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

        use crate::LONG_BULLET_SIZE;

        self.draw_tex(
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
        self.draw_tex(
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

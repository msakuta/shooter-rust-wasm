use std::ops::{Deref, DerefMut};

use vecmath::{vec2_add, vec2_len, vec2_scale, vec2_sub};
#[cfg(feature = "webgl")]
use web_sys::WebGlRenderingContext as GL;

#[cfg(all(not(feature = "webgl"), feature = "piston"))]
use crate::assets_piston::Assets;
#[cfg(feature = "webgl")]
use crate::assets_webgl::Assets;

#[cfg(all(not(feature = "webgl"), feature = "piston"))]
use piston_window::{
    math::{rotate_radians, scale, translate},
    *,
};

#[cfg(all(not(feature = "webgl"), feature = "piston"))]
use super::Matrix;

use crate::{xor128::Xor128, ShooterState};

#[cfg(feature = "webgl")]
use super::draw_tex;
use super::{
    bbox_intersects, bounding_box, BulletBase, DeathReason, Entity, EntitySet, Item, Projectile,
    ENEMY_SIZE, SCREEN_RECT,
};

const JOINT_LENGTH: f64 = 20.;
const CENTIPEDE_STRAIGHT_TASK_TIME: u32 = 100;
const CENTIPEDE_TURN_TASK_TIME: u32 = 100;
const TURN_RATE: f64 = 0.4 * std::f64::consts::PI / CENTIPEDE_TURN_TASK_TIME as f64;

pub struct EnemyBase {
    pub base: Entity,
    pub predicted_damage: i32,
}

impl Deref for EnemyBase {
    type Target = Entity;
    fn deref(&self) -> &Entity {
        &self.base
    }
}

impl DerefMut for EnemyBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl EnemyBase {
    pub fn new(pos: [f64; 2], velo: [f64; 2]) -> Self {
        Self {
            base: Entity::new(pos, velo).health(64),
            predicted_damage: 0,
        }
    }

    pub fn health(mut self, health: i32) -> Self {
        self.base.health = health;
        self
    }
}

pub struct ShieldedBoss {
    pub base: EnemyBase,
    pub shield_health: i32,
}

impl ShieldedBoss {
    pub fn new(pos: [f64; 2], velo: [f64; 2]) -> Self {
        Self {
            base: EnemyBase {
                base: Entity::new(pos, velo).health(64),
                predicted_damage: 0,
            },
            shield_health: 64,
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct CentipedeJoint([f64; 2], i32);

#[derive(Debug)]
#[repr(C)]
enum CentipedeTask {
    Straight,
    TurnLeft,
    TurnRight,
}

pub struct CentipedeEnemy {
    base: EnemyBase,
    joints: Vec<CentipedeJoint>,
    task: CentipedeTask,
    task_time: u32,
    heading: f64,
}

pub enum Enemy {
    Enemy1(EnemyBase),
    Boss(EnemyBase),
    ShieldedBoss(ShieldedBoss),
    SpiralEnemy(EnemyBase),
    Centipede(CentipedeEnemy),
}

impl Deref for Enemy {
    type Target = EnemyBase;
    fn deref(&self) -> &EnemyBase {
        match self {
            Enemy::Enemy1(base) | Enemy::Boss(base) | Enemy::SpiralEnemy(base) => &base,
            Enemy::ShieldedBoss(boss) => &boss.base,
            Enemy::Centipede(centipede) => &centipede.base,
        }
    }
}

impl DerefMut for Enemy {
    fn deref_mut(&mut self) -> &mut EnemyBase {
        match self {
            Enemy::Enemy1(ref mut base)
            | Enemy::Boss(ref mut base)
            | Enemy::SpiralEnemy(ref mut base) => base,
            Enemy::ShieldedBoss(ref mut boss) => &mut boss.base,
            Enemy::Centipede(ref mut centipede) => &mut centipede.base,
        }
    }
}

impl Enemy {
    /// Apply damage to this enemy, within specified rectangle area.
    /// The area can be important for patial damages.
    pub fn damage(&mut self, val: i32, rect: &[f64; 4]) -> Option<Enemy> {
        match self {
            Enemy::Enemy1(ref mut base)
            | Enemy::Boss(ref mut base)
            | Enemy::SpiralEnemy(ref mut base) => {
                base.base.health -= val;
                console_log!("damaged: {}", base.health);
            }
            Enemy::ShieldedBoss(ref mut boss) => {
                if boss.shield_health < 16 {
                    boss.base.health -= val
                } else {
                    boss.shield_health -= val
                }
            }
            Enemy::Centipede(ref mut centipede) => {
                let self_velo = centipede.base.velo;

                let damaged_joint = centipede.joints.iter_mut().enumerate().find(|(_, joint)| {
                    let rect2 = bounding_box(&joint.0, ENEMY_SIZE);
                    bbox_intersects(rect, &rect2)
                });

                if let Some((i, joint)) = damaged_joint {
                    joint.1 -= val;
                    if joint.1 <= 0 {
                        let joint_pos = joint.0;
                        if centipede.joints.len() == 1 {
                            centipede.base.health = -1;
                        } else {
                            let heading =
                                self_velo[1].atan2(self_velo[0]) + std::f64::consts::PI / 2.;
                            let speed = vec2_len(self_velo);
                            let velo = [speed * heading.cos(), speed * heading.sin()];
                            let back_joints = if i + 1 < centipede.joints.len() {
                                Some(centipede.joints[i + 1..].to_vec())
                            } else {
                                None
                            };
                            centipede.joints.resize(i, CentipedeJoint::default());
                            return back_joints.map(|back_joints| {
                                Enemy::new_centipede_joints(joint_pos, velo, back_joints)
                            });
                        }
                    }
                } else {
                    centipede.base.health -= 1;
                }
            }
        }
        None
    }

    pub fn predicted_damage(&self) -> i32 {
        match self {
            Enemy::Enemy1(base) | Enemy::Boss(base) | Enemy::SpiralEnemy(base) => {
                base.predicted_damage
            }
            Enemy::ShieldedBoss(boss) => boss.base.predicted_damage,
            Enemy::Centipede(centipede) => centipede.base.predicted_damage,
        }
    }

    pub fn add_predicted_damage(&mut self, val: i32) {
        self.predicted_damage += val;
    }

    pub fn total_health(&self) -> i32 {
        self.health
    }

    pub fn drop_item(&self, ent: Entity) -> Item {
        match self {
            Enemy::Enemy1(_) => Item::PowerUp(ent),
            _ => Item::PowerUp10(ent),
        }
    }

    fn gen_bullets(
        &mut self,
        bullets: &mut EntitySet<Projectile>,
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
                    Entity::new(self.pos, vec2_scale([angle.cos(), angle.sin()], 1.))
                        .rotation(angle as f32),
                ));
                bullets.insert(eb);
            }
        }
    }

    pub fn animate(&mut self, state: &mut ShooterState) -> Option<DeathReason> {
        if self.is_boss() {
            self.gen_bullets(&mut state.bullets, &mut state.rng, Projectile::new_phase);
        } else if let Enemy::SpiralEnemy(_) = self {
            self.gen_bullets(&mut state.bullets, &mut state.rng, Projectile::new_spiral);
        } else {
            let x: u32 = state.rng.gen_range(0, 64);
            if x == 0 {
                let eb = Projectile::EnemyBullet(BulletBase(Entity::new(
                    self.pos,
                    [state.rng.gen() - 0.5, state.rng.gen() - 0.5],
                )));
                state.bullets.insert(eb);
            }
        }

        match self {
            Enemy::Enemy1(ref mut base) | Enemy::Boss(ref mut base) => base.animate(),
            Enemy::ShieldedBoss(ref mut boss) => {
                if boss.shield_health < 64 && state.time % 8 == 0 {
                    boss.shield_health += 1;
                }
                boss.base.animate()
            }
            Enemy::SpiralEnemy(ref mut base) => {
                base.rotation -= std::f32::consts::PI * 0.01;
                base.animate()
            }
            Enemy::Centipede(centipede) => {
                let death = centipede.base.animate();
                if let Some(DeathReason::Killed) = death {
                    return death;
                }
                let mut prev = centipede.base.pos;
                let Some(first_joint) = centipede.joints.first_mut() else {
                    return Some(DeathReason::Killed);
                };

                if centipede.task_time < 1 {
                    centipede.task = match state.rng.gen_range(0, 4) {
                        0..2 => CentipedeTask::Straight,
                        2 => CentipedeTask::TurnLeft,
                        3 => CentipedeTask::TurnRight,
                        _ => unreachable!(),
                    };
                    centipede.task_time = match centipede.task {
                        CentipedeTask::Straight => CENTIPEDE_STRAIGHT_TASK_TIME,
                        _ => CENTIPEDE_TURN_TASK_TIME,
                    };
                    console_log!("Centipede task: {:?}", centipede.task);
                } else {
                    centipede.task_time -= 1;
                }

                match centipede.task {
                    CentipedeTask::Straight => {}
                    CentipedeTask::TurnLeft => {
                        centipede.heading =
                            (centipede.heading + TURN_RATE).rem_euclid(2. * std::f64::consts::PI);
                    }
                    CentipedeTask::TurnRight => {
                        centipede.heading =
                            (centipede.heading - TURN_RATE).rem_euclid(2. * std::f64::consts::PI);
                    }
                }

                let speed = vec2_len(centipede.base.velo);
                centipede.base.velo[0] = centipede.heading.cos() * speed;
                centipede.base.velo[1] = centipede.heading.sin() * speed;

                first_joint.0 = centipede.base.pos;
                let mut ret = false;
                for joint in centipede.joints.iter_mut().skip(1) {
                    let delta = vec2_sub(joint.0, prev);
                    let dist = vec2_len(delta);
                    if JOINT_LENGTH < dist {
                        let normalized = vec2_scale(delta, JOINT_LENGTH / dist);
                        joint.0 = vec2_add(prev, normalized);
                    }
                    prev = joint.0;
                    let joint_rect = bounding_box(&joint.0, ENEMY_SIZE);
                    if bbox_intersects(&joint_rect, &SCREEN_RECT) {
                        ret = true;
                    }
                }
                if !ret {
                    Some(DeathReason::RangeOut)
                } else {
                    None
                }
            }
        }
    }

    #[cfg(feature = "webgl")]
    pub fn draw(&self, _state: &ShooterState, gl: &GL, assets: &Assets) {
        use crate::{BOSS_SIZE, CENTIPEDE_SIZE};

        use super::ENEMY_SIZE;

        // Draw tails behind
        if let Enemy::Centipede(centipede) = self {
            let mut last_pos = None;
            for (i, joint) in centipede.joints.iter().enumerate() {
                let f = i as f64 / centipede.joints.len() as f64;
                let rotation = if let Some(last_pos) = last_pos {
                    let delta = vec2_sub(joint.0, last_pos);
                    delta[1].atan2(delta[0])
                } else {
                    self.velo[1].atan2(self.velo[0])
                };
                last_pos = Some(joint.0);
                draw_tex(
                    &joint.0,
                    rotation,
                    assets,
                    gl,
                    if i == 0 {
                        &assets.centipede_head_tex
                    } else {
                        &assets.centipede_segment_tex
                    },
                    Some([(CENTIPEDE_SIZE * (1. - f) + ENEMY_SIZE * f); 2]),
                );
            }
            return;
        }

        self.draw_tex(
            assets,
            gl,
            match self {
                Enemy::Enemy1(_) => &assets.enemy_tex,
                Enemy::Boss(_) | Enemy::ShieldedBoss(_) => &assets.boss_tex,
                Enemy::SpiralEnemy(_) => &assets.spiral_enemy_tex,
                _ => unreachable!(),
            },
            Some(match self {
                Enemy::Enemy1(_) => [ENEMY_SIZE; 2],
                Enemy::Boss(_) | Enemy::ShieldedBoss(_) | Enemy::SpiralEnemy(_) => [BOSS_SIZE; 2],
                _ => unreachable!(),
            }),
        );

        if let Enemy::ShieldedBoss(boss) = self {
            self.draw_tex(
                assets,
                gl,
                &assets.shield_tex,
                Some([boss.shield_health as f64; 2]),
            );
        }
    }

    #[cfg(all(not(feature = "webgl"), feature = "piston"))]
    pub fn draw(&self, context: &Context, g: &mut G2d, assets: &Assets) {
        self.draw_tex(
            context,
            g,
            match self {
                Enemy::Enemy1(_) | Enemy::Centipede(_) => &assets.enemy_tex,
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
            let pos = &boss.base.pos;
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
        if let Enemy::Centipede(centipede) = self {
            return centipede.joints.iter().any(|joint| {
                let rect2 = bounding_box(&joint.0, ENEMY_SIZE);
                bbox_intersects(&rect, &rect2)
            });
        }
        let rect2 = self.get_bb();
        bbox_intersects(&rect, &rect2)
    }

    pub fn get_bb(&self) -> [f64; 4] {
        let size = if let Enemy::ShieldedBoss(boss) = self {
            boss.shield_health as f64
        } else {
            ENEMY_SIZE
        };
        bounding_box(&self.pos, size)
    }

    pub fn is_boss(&self) -> bool {
        matches!(self, Enemy::Boss(_) | Enemy::ShieldedBoss(_))
    }

    pub fn new_spiral(pos: [f64; 2], velo: [f64; 2]) -> Enemy {
        Enemy::SpiralEnemy(EnemyBase::new(pos, velo))
    }

    pub fn new_centipede(pos: [f64; 2], velo: [f64; 2]) -> Enemy {
        Enemy::Centipede(CentipedeEnemy {
            // The head is particularly tough
            base: EnemyBase::new(pos, velo).health(32),
            joints: vec![CentipedeJoint(pos, 16); 10],
            task: CentipedeTask::Straight,
            task_time: CENTIPEDE_STRAIGHT_TASK_TIME,
            heading: velo[1].atan2(velo[0]),
        })
    }

    fn new_centipede_joints(pos: [f64; 2], velo: [f64; 2], joints: Vec<CentipedeJoint>) -> Enemy {
        Enemy::Centipede(CentipedeEnemy {
            // The head is particularly tough
            base: EnemyBase::new(pos, velo).health(32),
            joints,
            task: CentipedeTask::Straight,
            task_time: CENTIPEDE_STRAIGHT_TASK_TIME,
            heading: velo[1].atan2(velo[0]),
        })
    }
}

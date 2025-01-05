use std::ops::{Deref, DerefMut};

use vecmath::vec2_scale;
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

use crate::{
    xor128::Xor128, ShooterState
};

use super::{BulletBase, DeathReason, Entity, Item, Projectile, ENEMY_SIZE};

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
    pub fn new(id_gen: &mut u32, pos: [f64; 2], velo: [f64; 2]) -> Self {
        Self {
            base: Entity::new(id_gen, pos, velo).health(64),
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
    pub fn new(id_gen: &mut u32, pos: [f64; 2], velo: [f64; 2]) -> Self {
        Self {
            base: EnemyBase {
                base: Entity::new(id_gen, pos, velo).health(64),
                predicted_damage: 0,
            },
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

impl Deref for Enemy {
    type Target = EnemyBase;
    fn deref(&self) -> &EnemyBase {
        match self {
            Enemy::Enemy1(base) | Enemy::Boss(base) | Enemy::SpiralEnemy(base) => &base,
            Enemy::ShieldedBoss(boss) => &boss.base,
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
        }
    }
}

impl Enemy {
    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn damage(&mut self, val: i32) {
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
        }
    }

    pub fn predicted_damage(&self) -> i32 {
        match self {
            Enemy::Enemy1(base) | Enemy::Boss(base) | Enemy::SpiralEnemy(base) => {
                base.predicted_damage
            }
            Enemy::ShieldedBoss(boss) => boss.base.predicted_damage,
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
                    Entity::new(id_gen, self.pos, vec2_scale([angle.cos(), angle.sin()], 1.))
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
                    self.pos,
                    [state.rng.gen() - 0.5, state.rng.gen() - 0.5],
                )));
                state.bullets.insert(eb.get_id(), eb);
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
        }
    }

    #[cfg(feature = "webgl")]
    pub fn draw(&self, _state: &ShooterState, gl: &GL, assets: &Assets) {
        use crate::BOSS_SIZE;

        use super::ENEMY_SIZE;


        self.draw_tex(
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
        let rect2 = self.get_bb();
        rect[0] < rect2[2] && rect2[0] < rect[2] && rect[1] < rect2[3] && rect2[1] < rect[3]
    }

    pub fn get_bb(&self) -> [f64; 4] {
        let size = if let Enemy::ShieldedBoss(boss) = self {
            boss.shield_health as f64
        } else {
            ENEMY_SIZE
        };
        [
            self.pos[0] - size,
            self.pos[1] - size,
            self.pos[0] + size,
            self.pos[1] + size,
        ]
    }

    pub fn is_boss(&self) -> bool {
        matches!(self, Enemy::Boss(_) | Enemy::ShieldedBoss(_))
    }

    pub fn new_spiral(id_gen: &mut u32, pos: [f64; 2], velo: [f64; 2]) -> Enemy {
        Enemy::SpiralEnemy(EnemyBase::new(id_gen, pos, velo))
    }
}

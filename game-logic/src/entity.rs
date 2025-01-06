use core::f64;

#[cfg(all(not(feature = "webgl"), feature = "piston"))]
use crate::assets_piston::Assets;
#[cfg(feature = "webgl")]
use crate::assets_webgl::Assets;
use crate::consts::*;
use crate::xor128::Xor128;
use crate::ShooterState;
#[cfg(feature = "webgl")]
use crate::{enable_buffer, vertex_buffer_data};
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
use std::{
    ops::{Deref, DerefMut},
    rc::Rc, cell::RefCell,
};
use vecmath::{vec2_add, vec2_len, vec2_normalized, vec2_scale, vec2_square_len, vec2_sub};
#[cfg(feature = "webgl")]
use web_sys::{WebGlRenderingContext as GL, WebGlTexture};
use parser::Vm;

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

pub enum EnemyType {
    Enemy1,
    Boss,
    ShieldedBoss,
    SpiralEnemy,
}

pub struct Enemy {
    pub ty: EnemyType,
    pub base: Entity,
    pub predicted_damage: i32,
    pub shield_health: Option<i32>,
}

impl Enemy {
    pub fn new(
        ty: EnemyType,
        id_gen: &mut u32,
        pos: [f64; 2],
        velo: [f64; 2],
        health: i32,
    ) -> Self {
        Self {
            ty,
            base: Entity::new(id_gen, pos, velo).health(health),
            predicted_damage: 0,
            shield_health: None,
        }
    }

    pub fn new_shielded_boss(id_gen: &mut u32, pos: [f64; 2], velo: [f64; 2]) -> Self {
        Self {
            ty: EnemyType::ShieldedBoss,
            base: Entity::new(id_gen, pos, velo).health(64),
            predicted_damage: 0,
            shield_health: Some(64),
        }
    }

    pub fn get_id(&self) -> u32 {
        self.base.id
    }

    pub fn damage(&mut self, val: i32) {
        if let Some(ref mut shield_health) = self.shield_health {
            if *shield_health < 16 {
                self.base.health -= val
            } else {
                *shield_health -= val
            }
        } else {
            self.base.health -= val;
            console_log!("damaged: {}", self.base.health);
        }
    }

    pub fn add_predicted_damage(&mut self, val: i32) {
        self.predicted_damage += val;
    }

    pub fn total_health(&self) -> i32 {
        self.base.health
    }

    pub fn drop_item(&self, ent: Entity) -> Item {
        match self.ty {
            EnemyType::Enemy1 => Item::PowerUp(ent),
            _ => Item::PowerUp10(ent),
        }
    }

    fn gen_bullets(
        &mut self,
        id_gen: &mut u32,
        bullets: &mut std::collections::HashMap<u32, Projectile>,
        rng: &mut Xor128,
        create_fn: impl Fn(Entity) -> Projectile,
    ) {
        let x = rng.gen_range(0, 256);
        if x == 0 {
            use std::f64::consts::PI;
            let bullet_count = 10;
            let phase_offset = rng.gen() * PI;
            for i in 0..bullet_count {
                let angle = 2. * PI * i as f64 / bullet_count as f64 + phase_offset;
                let eb = create_fn(
                    Entity::new(
                        id_gen,
                        self.base.pos,
                        vec2_scale([angle.cos(), angle.sin()], 1.),
                    )
                    .rotation(angle as f32),
                );
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
        } else if let EnemyType::SpiralEnemy = self.ty {
            self.gen_bullets(
                &mut state.id_gen,
                &mut state.bullets,
                &mut state.rng,
                Projectile::new_spiral,
            );
        } else {
            let x: u32 = state.rng.gen_range(0, 64);
            if x == 0 {
                let eb = Projectile::new(
                    ProjectileType::EnemyBullet,
                    Entity::new(
                        &mut state.id_gen,
                        self.base.pos,
                        [state.rng.gen() - 0.5, state.rng.gen() - 0.5],
                    ),
                );
                state.bullets.insert(eb.get_id(), eb);
            }
        }

        match self.ty {
            EnemyType::ShieldedBoss => {
                if let Some(ref mut shield_health) = self.shield_health {
                    if *shield_health < 64 && state.time % 8 == 0 {
                        *shield_health += 1;
                    }
                }
            }
            EnemyType::SpiralEnemy => {
                self.base.rotation -= std::f32::consts::PI * 0.01;
            }
            _ => (),
        }
        self.base.animate()
    }

    #[cfg(feature = "webgl")]
    pub fn draw(&self, _state: &ShooterState, gl: &GL, assets: &Assets) {
        self.base.draw_tex(
            assets,
            gl,
            match self.ty {
                EnemyType::Enemy1 => &assets.enemy_tex,
                EnemyType::Boss | EnemyType::ShieldedBoss => &assets.boss_tex,
                EnemyType::SpiralEnemy => &assets.spiral_enemy_tex,
            },
            Some(match self.ty {
                EnemyType::Enemy1 => [ENEMY_SIZE; 2],
                EnemyType::Boss | EnemyType::ShieldedBoss | EnemyType::SpiralEnemy => {
                    [BOSS_SIZE; 2]
                }
            }),
        );

        if let Some(shield_health) = self.shield_health {
            self.base.draw_tex(
                assets,
                gl,
                &assets.shield_tex,
                Some([shield_health as f64; 2]),
            );
        }
    }

    #[cfg(all(not(feature = "webgl"), feature = "piston"))]
    pub fn draw(&self, context: &Context, g: &mut G2d, assets: &Assets) {
        self.base.draw_tex(
            context,
            g,
            match self.ty {
                EnemyType::Enemy1 => &assets.enemy_tex,
                EnemyType::Boss | EnemyType::ShieldedBoss => &assets.boss_tex,
                EnemyType::SpiralEnemy => &assets.spiral_enemy_tex,
            },
            if let EnemyType::SpiralEnemy = self.ty {
                Some(0.5)
            } else {
                None
            },
        );
        if let Some(shield_health) = self.shield_health {
            let pos = &self.base.pos;
            let tex2 = &*assets.shield_tex;
            let centerize = translate([
                -(tex2.get_width() as f64 / 2.),
                -(tex2.get_height() as f64 / 2.),
            ]);
            let rotmat = rotate_radians(0 as f64);
            let scalemat = scale(shield_health as f64 / 64., shield_health as f64 / 64.);
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
        let size = if let EnemyType::ShieldedBoss = self.ty {
            self.shield_health.unwrap_or(0) as f64
        } else {
            ENEMY_SIZE
        };
        [
            self.base.pos[0] - size,
            self.base.pos[1] - size,
            self.base.pos[0] + size,
            self.base.pos[1] + size,
        ]
    }

    pub fn is_boss(&self) -> bool {
        matches!(self.ty, EnemyType::Boss | EnemyType::ShieldedBoss)
    }

    pub fn new_spiral(id_gen: &mut u32, pos: [f64; 2], velo: [f64; 2]) -> Enemy {
        Enemy {
            ty: EnemyType::SpiralEnemy,
            base: Entity::new(id_gen, pos, velo).health(64),
            predicted_damage: 0,
            shield_health: None,
        }
    }
}

pub enum ProjectileType {
    Bullet,
    EnemyBullet,
    PhaseBullet,
    SpiralBullet,
    Missile,
}

pub struct PhaseBulletComponent {
    velo: [f64; 2],
    phase: f64,
}

pub struct SpiralBulletComponent {
    speed: f64,
    traveled: f64,
}

pub struct MissileComponent {
    target: u32,
    trail: Vec<[f64; 2]>,
}

pub struct Projectile {
    pub ty: ProjectileType,
    pub base: Entity,
    pub phase_bullet_component: Option<PhaseBulletComponent>,
    pub spiral_bullet_component: Option<SpiralBulletComponent>,
    pub missile_component: Option<MissileComponent>,
    vm: Option<Rc<RefCell<Vm>>>,
}

impl Deref for Projectile {
    type Target = Entity;
    fn deref(&self) -> &Entity {
        &self.base
    }
}

const MISSILE_DETECTION_RANGE: f64 = 256.;
const MISSILE_HOMING_SPEED: f64 = 0.25;
#[cfg(feature = "webgl")]
const MISSILE_TRAIL_WIDTH: f64 = 5.;
const MISSILE_TRAIL_LENGTH: usize = 20;
const MISSILE_DAMAGE: i32 = 5;

impl Projectile {
    pub fn new(ty: ProjectileType, base: Entity) -> Self {
        Self {
            ty,
            base,
            phase_bullet_component: None,
            spiral_bullet_component: None,
            missile_component: None,
            vm: None,
        }
    }

    pub fn new_phase(base: Entity) -> Projectile {
        let velo = base.velo;

        let bytecode = match compile_program(&"scripts/spiral.rscl") {
            Ok(bytecode) => bytecode,
            Err(e) => panic!("Compile error: {e}"),
        };
        let program = Rc::new(bytecode);

        Projectile {
            ty: ProjectileType::PhaseBullet,
            base,
            phase_bullet_component: Some(PhaseBulletComponent { velo, phase: 0. }),
            spiral_bullet_component: None,
            missile_component: None,
            vm: Some(program)
        }
    }

    pub fn new_spiral(base: Entity) -> Projectile {
        let speed = vec2_len(base.velo);
        Projectile {
            ty: ProjectileType::SpiralBullet,
            base,
            phase_bullet_component: None,
            spiral_bullet_component: Some(SpiralBulletComponent {
                speed,
                traveled: 0.,
            }),
            missile_component: None,
            vm: None,
        }
    }

    pub fn new_missile(base: Entity) -> Self {
        Self {
            ty: ProjectileType::Missile,
            base,
            phase_bullet_component: None,
            spiral_bullet_component: None,
            missile_component: Some(MissileComponent {
                target: 0,
                trail: vec![],
            }),
            vm: None,
        }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn get_type(&self) -> &str {
        match self.ty {
            ProjectileType::Bullet | ProjectileType::EnemyBullet => "Bullet",
            ProjectileType::PhaseBullet => "PhaseBullet",
            ProjectileType::SpiralBullet => "SpiralBullet",
            ProjectileType::Missile => "Missile",
        }
    }

    fn animate_player_bullet(
        mut base: &mut Entity,
        enemies: &mut Vec<Enemy>,
        mut _player: &mut Player,
    ) -> Option<DeathReason> {
        let bbox = Self::get_bb_base(base);
        for enemy in enemies.iter_mut() {
            if enemy.test_hit(bbox) {
                enemy.damage(base.health);
                base.health = 0;
                break;
            }
        }
        base.animate()
    }

    fn animate_enemy_bullet(
        base: &mut Entity,
        _enemies: &mut Vec<Enemy>,
        player: &mut Player,
    ) -> Option<DeathReason> {
        if let Some(death_reason) = base.hits_player(&player.base) {
            player.base.health -= base.health;
            return Some(death_reason);
        }
        base.animate()
    }

    pub fn animate_bullet(
        &mut self,
        enemies: &mut Vec<Enemy>,
        player: &mut Player,
    ) -> Option<DeathReason> {
        if let Some(ref mut phase_bullet) = self.phase_bullet_component {
            self.base.velo = vec2_scale(phase_bullet.velo, (phase_bullet.phase.sin() + 1.) / 2.);
            phase_bullet.phase += 0.02 * std::f64::consts::PI;
        }

        if let Some(SpiralBulletComponent {
            speed,
            ref mut traveled,
        }) = self.spiral_bullet_component
        {
            let rotation =
                self.base.rotation as f64 - 0.02 * std::f64::consts::PI / (*traveled * 0.05 + 1.);
            self.base.rotation = rotation as f32;
            self.base.velo = vec2_scale([rotation.cos(), rotation.sin()], speed);
            *traveled += speed;
        }

        let pos = &self.base.pos;
        if let Some(MissileComponent {
            ref mut target,
            ref mut trail,
        }) = self.missile_component
        {
            if *target == 0 {
                let best = enemies.iter_mut().fold((0, 1e5, None), |bestpair, enemy| {
                    let dist = vec2_len(vec2_sub(*pos, enemy.base.pos));
                    if dist < MISSILE_DETECTION_RANGE
                        && dist < bestpair.1
                        && enemy.predicted_damage < enemy.total_health()
                    {
                        (enemy.base.id, dist, Some(enemy))
                    } else {
                        bestpair
                    }
                });
                *target = best.0;
                if let Some(enemy) = best.2 {
                    enemy.add_predicted_damage(MISSILE_DAMAGE);
                    println!(
                        "Add predicted damage: {} -> {}",
                        enemy.predicted_damage - MISSILE_DAMAGE,
                        enemy.predicted_damage
                    );
                }
            } else if let Some(target_enemy) = enemies.iter().find(|e| e.get_id() == *target) {
                let norm = vec2_normalized(vec2_sub(target_enemy.base.pos, self.base.pos));
                let desired_velo = vec2_scale(norm, MISSILE_SPEED);
                let desired_diff = vec2_sub(desired_velo, self.base.velo);
                if std::f64::EPSILON < vec2_square_len(desired_diff) {
                    self.base.velo = if vec2_square_len(desired_diff)
                        < MISSILE_HOMING_SPEED * MISSILE_HOMING_SPEED
                    {
                        desired_velo
                    } else {
                        let desired_diff_norm = vec2_normalized(desired_diff);
                        vec2_add(
                            self.base.velo,
                            vec2_scale(desired_diff_norm, MISSILE_HOMING_SPEED),
                        )
                    };
                    let angle = self.base.velo[1].atan2(self.base.velo[0]);
                    self.base.rotation = (angle + std::f64::consts::FRAC_PI_2) as f32;
                    let (s, c) = angle.sin_cos();
                    self.base.velo[0] = MISSILE_SPEED * c;
                    self.base.velo[1] = MISSILE_SPEED * s;
                }
            } else {
                *target = 0
            }
            if MISSILE_TRAIL_LENGTH < trail.len() {
                trail.remove(0);
            }
            trail.push(self.base.pos);
            let res = Self::animate_player_bullet(&mut self.base, enemies, player);
            if res.is_some() {
                if let Some(target_enemy) = enemies.iter_mut().find(|e| e.get_id() == *target) {
                    target_enemy.add_predicted_damage(-MISSILE_DAMAGE);
                    println!(
                        "Reduce predicted damage: {} -> {}",
                        target_enemy.predicted_damage + MISSILE_DAMAGE,
                        target_enemy.predicted_damage
                    );
                }
            }
            return res;
        }

        match self.ty {
            ProjectileType::Bullet | ProjectileType::Missile => {
                Self::animate_player_bullet(&mut self.base, enemies, player)
            }
            _ => Self::animate_enemy_bullet(&mut self.base, enemies, player),
        }
    }

    pub fn get_bb_base(base: &Entity) -> [f64; 4] {
        [
            base.pos[0] - BULLET_SIZE,
            base.pos[1] - BULLET_SIZE,
            base.pos[0] + BULLET_SIZE,
            base.pos[1] + BULLET_SIZE,
        ]
    }

    #[cfg(feature = "webgl")]
    pub fn draw(&self, gl: &GL, assets: &Assets) {
        if let ProjectileType::Bullet = self.ty {
            if let Some(ref shader) = assets.sprite_shader.as_ref() {
                gl.blend_equation(GL::FUNC_ADD);
                gl.blend_func(GL::SRC_ALPHA, GL::ONE);
                gl.uniform1f(shader.alpha_loc.as_ref(), 0.15);

                self.base.draw_tex(
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
        if let Some(MissileComponent { ref trail, .. }) = self.missile_component {
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
        use ProjectileType::*;
        self.draw_tex(
            assets,
            gl,
            match self.ty {
                Bullet => &assets.bullet_texture,
                EnemyBullet => &assets.enemy_bullet_texture,
                PhaseBullet => &assets.phase_bullet_tex,
                SpiralBullet => &assets.spiral_bullet_tex,
                Missile => &assets.missile_tex,
            },
            Some(match self.ty {
                Bullet | EnemyBullet | Missile => [BULLET_SIZE; 2],
                PhaseBullet | SpiralBullet => LONG_BULLET_SIZE,
            }),
        );
    }

    #[cfg(all(not(feature = "webgl"), feature = "piston"))]
    pub fn draw(&self, c: &Context, g: &mut G2d, assets: &Assets) {
        if let Some(MissileComponent { ref trail, .. }) = self.missile_component {
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
            match self.ty {
                ProjectileType::Bullet => &assets.bullet_tex,
                ProjectileType::EnemyBullet => &assets.ebullet_tex,
                ProjectileType::PhaseBullet => &assets.phase_bullet_tex,
                ProjectileType::SpiralBullet => &assets.spiral_bullet_tex,
                ProjectileType::Missile => &assets.missile_tex,
            },
            None,
        );
    }
}

pub(crate) fn compile_program(src: &str) -> Result<ByteCode, Box<dyn Error>> {
    let source = std::fs::read_to_string(src).expect("Source file could be read");
    let ast = parse_program(src, &source).expect("Source parsed");

    let mut type_check_context = TypeCheckContext::new();
    extend_funcs(|name, func| type_check_context.add_fn(name, func));
    match type_check(&ast, &mut type_check_context) {
        Ok(_) => println!("Typecheck Ok"),
        Err(e) => {
            return Err(format!(
                "{}:{}:{}: {}",
                src,
                e.span.location_line(),
                e.span.get_utf8_column(),
                e
            )
            .into())
        }
    }

    let mut compiler = Compiler::new();
    compiler.compile(&ast)?;

    if args.disasm {
        compiler.disasm(&mut std::io::stdout())?;
    }

    let mut bytecode = compiler.into_bytecode();
    extend_funcs(|name, func| bytecode.add_fn(name, func));

    Ok(bytecode)
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

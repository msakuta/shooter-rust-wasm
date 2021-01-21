use core::f64;

use crate::consts::*;
use crate::xor128::Xor128;
use crate::{enable_buffer, vertex_buffer_data, ShooterState};
use cgmath::{Matrix3, Matrix4, Rad, Vector2, Vector3};
use std::rc::Rc;
use vecmath::{vec2_add, vec2_len, vec2_normalized, vec2_scale, vec2_square_len, vec2_sub};
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
        for i in 0..2 {
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
        let shader = assets.sprite_shader.as_ref().unwrap();
        context.bind_texture(GL::TEXTURE_2D, Some(&texture));
        let pos = &self.pos;
        let translation = Matrix4::from_translation(Vector3::new(pos[0], pos[1], 0.));
        let scale_mat = Matrix4::from_scale(scale.unwrap_or(1.));
        let rotation = Matrix4::from_angle_z(Rad(self.rotation as f64));
        let transform = assets.world_transform * &translation * &scale_mat * &rotation;
        context.uniform_matrix4fv_with_f32_array(
            shader.transform_loc.as_ref(),
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

#[derive(PartialEq, Clone, Debug)]
pub enum Weapon {
    Bullet,
    Light,
    Missile,
    Lightning,
}

impl Weapon {
    pub fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}

pub const weapon_set: [(usize, Weapon, [f32; 3]); 4] = [
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

pub struct ShaderBundle {
    pub program: WebGlProgram,
    pub vertex_position: u32,
    pub tex_coord_position: u32,
    pub texture_loc: Option<WebGlUniformLocation>,
    pub transform_loc: Option<WebGlUniformLocation>,
    pub tex_transform_loc: Option<WebGlUniformLocation>,
}

impl ShaderBundle {
    pub fn new(gl: &GL, program: WebGlProgram) -> Self {
        let vertex_position = gl.get_attrib_location(&program, "vertexData") as u32;
        let tex_coord_position = gl.get_attrib_location(&program, "vertexData") as u32;
        let texture_loc = gl.get_uniform_location(&program, "texture");
        let transform_loc = gl.get_uniform_location(&program, "transform");
        let tex_transform_loc = gl.get_uniform_location(&program, "texTransform");
        let check_none = |op: &Option<WebGlUniformLocation>| {
            if op.is_none() {
                console_log!("Warning: location undefined");
            } else {
                console_log!("location defined");
            }
        };
        console_log!("vertex_position: {}", vertex_position);
        console_log!("tex_coord_position: {}", tex_coord_position);
        check_none(&texture_loc);
        check_none(&transform_loc);
        check_none(&tex_transform_loc);
        Self {
            program,
            vertex_position,
            tex_coord_position,
            texture_loc,
            transform_loc,
            tex_transform_loc,
        }
    }
}

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
        match self {
            _ => self.get_base().health,
        }
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
            let phase_offset = rng.next() * PI;
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
                |base| Projectile::new_phase(base),
            );
        } else if let Enemy::SpiralEnemy(_) = self {
            self.gen_bullets(
                &mut state.id_gen,
                &mut state.bullets,
                &mut state.rng,
                |base| Projectile::new_spiral(base),
            );
        } else {
            let x: u32 = state.rng.gen_range(0, 64);
            if x == 0 {
                let eb = Projectile::EnemyBullet(BulletBase(Entity::new(
                    &mut state.id_gen,
                    self.get_base().pos,
                    [state.rng.next() - 0.5, state.rng.next() - 0.5],
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
                Enemy::Enemy1(_) => ENEMY_SIZE,
                Enemy::Boss(_) | Enemy::ShieldedBoss(_) | Enemy::SpiralEnemy(_) => BOSS_SIZE,
            }),
        );

        if let Enemy::ShieldedBoss(boss) = self {
            self.get_base().draw_tex(
                assets,
                gl,
                &assets.shield_tex,
                Some(boss.shield_health as f64),
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
        match self {
            Enemy::Boss(_) | Enemy::ShieldedBoss(_) => true,
            _ => false,
        }
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

    pub fn get_base<'b>(&'b self) -> &'b BulletBase {
        match &self {
            &Projectile::Bullet(base) | &Projectile::EnemyBullet(base) => base,
            &Projectile::PhaseBullet { base, .. } | &Projectile::SpiralBullet { base, .. } => base,
            &Projectile::Missile { base, .. } => base,
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
                if let Some(_) = res {
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

    pub fn draw(&self, state: &ShooterState, gl: &GL, assets: &Assets) {
        if let Projectile::Missile { trail, .. } = self {
            let mut iter = trail.iter().enumerate();
            if let Some(mut prev) = iter.next() {
                for e in iter {
                    prev = e;
                }
            }

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

            vertex_buffer_data(gl, &vertices).unwrap();

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
            gl.use_program(assets.sprite_shader.as_ref().and_then(|o| Some(&o.program)));
            enable_buffer(gl, &assets.rect_buffer, 2, shader.vertex_position);
        }
        self.get_base().0.draw_tex(
            assets,
            gl,
            match self {
                Projectile::Bullet(_) => &assets.bullet_texture,
                Projectile::EnemyBullet(_) => &assets.enemy_bullet_texture,
                Projectile::PhaseBullet { .. } => &assets.phase_bullet_tex,
                Projectile::SpiralBullet { .. } => &assets.spiral_bullet_tex,
                Projectile::Missile { .. } => &assets.missile_tex,
            },
            Some(BULLET_SIZE),
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

    pub fn draw(&self, gl: &GL, assets: &Assets) {
        match self {
            Item::PowerUp(item) => item.draw_tex(&assets, gl, &assets.power_tex, Some(ITEM_SIZE)),
            Item::PowerUp10(item) => {
                item.draw_tex(&assets, gl, &assets.power2_tex, Some(ITEM2_SIZE))
            }
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
                if let Some(_) = ent.hits_player(&player.base) {
                    player.power += self.power_value();
                    return Some(DeathReason::Killed);
                }
                ent.animate()
            }
        }
    }
}

pub struct TempEntity {
    pub base: Entity,
    pub texture: Rc<WebGlTexture>,
    pub max_frames: u32,
    pub width: u32,
    pub playback_rate: u32,
    pub image_width: u32,
    pub size: f64,
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
        let shader = assets.sprite_shader.as_ref().unwrap();
        let pos = &self.base.pos;
        context.bind_texture(GL::TEXTURE_2D, Some(&self.texture));
        let rotation = Matrix4::from_angle_z(Rad(self.base.rotation as f64));
        let translation = Matrix4::from_translation(Vector3::new(pos[0], pos[1], 0.));
        let scale = Matrix4::from_scale(self.size);
        let frame = self.max_frames - (self.base.health as u32 / self.playback_rate) as u32;
        // let image   = Image::new().rect([0f64, 0f64, self.width as f64, tex2.get_height() as f64])
        //     .src_rect([frame as f64 * self.width as f64, 0., self.width as f64, tex2.get_height() as f64]);
        let transform = assets.world_transform * &translation * &rotation * &scale;
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

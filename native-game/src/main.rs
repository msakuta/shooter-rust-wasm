use game_logic::{
    consts::*,
    entity::{Assets, BulletBase, Enemy, Entity, Item, Player, Projectile, TempEntity, Weapon},
    ShooterState,
};
use piston_window::draw_state::Blend;
use piston_window::*;
use rand::prelude::*;
use std::collections::HashMap;

fn main() {
    let mut time = 0;
    let mut disptime = 0;
    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow =
        WindowSettings::new("Shooter Rust", [WINDOW_WIDTH, WINDOW_HEIGHT])
            .exit_on_esc(true)
            .opengl(opengl)
            .build()
            .unwrap();

    // let (assets, mut glyphs) = Assets::new(&mut window);
    let assets = Assets {};

    let mut id_gen = 0;
    let mut player = Player::new(Entity::new(&mut id_gen, [240., 400.], [0., 0.]));

    let mut enemies = Vec::<Enemy>::new();

    let mut items = Vec::<Item>::new();

    let mut bullets = HashMap::new();

    let mut tent = Vec::<TempEntity>::new();

    let mut rng = thread_rng();

    let mut paused = false;
    let mut game_over = true;

    let mut weapon = Weapon::Bullet;

    let shoot_period = if let Weapon::Bullet = weapon { 5 } else { 50 };

    let level = player.power_level() as i32;
    player.cooldown += shoot_period;
    for i in -1 - level..2 + level {
        let speed = if let Weapon::Bullet = weapon {
            BULLET_SPEED
        } else {
            MISSILE_SPEED
        };
        let mut ent = Entity::new(&mut id_gen, player.base.pos, [i as f64, -speed])
            .rotation((i as f32).atan2(speed as f32));
        bullets.insert(ent.id, Projectile::Bullet(BulletBase(ent)));
    }

    while let Some(event) = window.next() {}
}

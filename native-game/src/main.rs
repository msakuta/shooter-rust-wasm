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

    let (assets, mut glyphs) = Assets::new(&mut window);

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

    fn limit_viewport(viewport: &Viewport, ratio: f64, wwidth: u32, wheight: u32) -> Viewport{
        let vp_ratio = (viewport.rect[2] - viewport.rect[0]) as f64 /
            (viewport.rect[3] - viewport.rect[0]) as f64;
        let mut newvp = *viewport;
        newvp.window_size[0] = (wwidth as f64 * (vp_ratio / ratio).max(1.)) as u32;
        newvp.window_size[1] = (wheight as f64 * (ratio / vp_ratio).max(1.)) as u32;
        #[cfg(debug)]
        for (vp, name) in [(viewport, "Old"), (&newvp, "New")].iter() {
            println!("{} Context: ratio: {} vp.rect: {:?} vp.draw: {:?} vp.window: {:?}",
                name, ratio, vp.rect, vp.draw_size, vp.window_size);
        }
        newvp
    }

    while let Some(event) = window.next() {
        if let Some(_) = event.render_args() {
            window.draw_2d(&event, |mut context, graphics| {
                clear([0.0, 0., 0., 1.], graphics);

                if let Some(viewport) = context.viewport {
                    let (fwidth, fheight) = (WINDOW_WIDTH as f64, WINDOW_HEIGHT as f64);
                    let ratio = fwidth / fheight;
    
                    let wnd_context = Context::new_viewport(limit_viewport(&viewport, ratio, WINDOW_WIDTH, WINDOW_HEIGHT));
    
                    wnd_context.trans(-1., -1.);
    
                    image(&assets.bg, wnd_context.transform, graphics);
    
                    context = Context::new_viewport(limit_viewport(&viewport, ratio, WINDOW_WIDTH, WINDOW_HEIGHT));
                }
            });
        }
    }
}

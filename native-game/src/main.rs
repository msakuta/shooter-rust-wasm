use game_logic::{
    consts::*,
    entity::{
        Assets, BulletBase, Enemy, Entity, Item, Matrix, Player, Projectile, TempEntity, Weapon,
    },
    ShooterState,
};
use piston_window::draw_state::Blend;
use piston_window::math::{rotate_radians, scale, translate};
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

    fn limit_viewport(viewport: &Viewport, ratio: f64, wwidth: u32, wheight: u32) -> Viewport {
        let vp_ratio = (viewport.rect[2] - viewport.rect[0]) as f64
            / (viewport.rect[3] - viewport.rect[0]) as f64;
        let mut newvp = *viewport;
        newvp.window_size[0] = (wwidth as f64 * (vp_ratio / ratio).max(1.)) as u32;
        newvp.window_size[1] = (wheight as f64 * (ratio / vp_ratio).max(1.)) as u32;
        #[cfg(debug)]
        for (vp, name) in [(viewport, "Old"), (&newvp, "New")].iter() {
            println!(
                "{} Context: ratio: {} vp.rect: {:?} vp.draw: {:?} vp.window: {:?}",
                name, ratio, vp.rect, vp.draw_size, vp.window_size
            );
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

                    let wnd_context = Context::new_viewport(limit_viewport(
                        &viewport,
                        ratio,
                        WINDOW_WIDTH,
                        WINDOW_HEIGHT,
                    ));

                    wnd_context.trans(-1., -1.);

                    image(&assets.bg, wnd_context.transform, graphics);

                    context = Context::new_viewport(limit_viewport(
                        &viewport,
                        ratio,
                        WINDOW_WIDTH,
                        WINDOW_HEIGHT,
                    ));
                }

                let mut draw_text_pos = |s: &str, pos: [f64; 2], color: [f32; 4], size: u32| {
                    text::Text::new_color(color, size)
                        .draw(
                            s,
                            &mut glyphs,
                            &context.draw_state,
                            context.transform.trans(pos[0], pos[1]),
                            graphics,
                        )
                        .unwrap_or_default();
                };

                let weapon_set = [
                    (0, Weapon::Bullet, [1., 0.5, 0.]),
                    (2, Weapon::Light, [1., 1., 1.]),
                    (3, Weapon::Missile, [0., 1., 0.]),
                    (4, Weapon::Lightning, [1., 1., 0.]),
                ];

                draw_text_pos(
                    "Z",
                    [
                        ((WINDOW_WIDTH + WIDTH) / 2 - weapon_set.len() as u32 * 32 / 2 - 16) as f64,
                        (WINDOW_HEIGHT * 3 / 4) as f64,
                    ],
                    [1.0, 1.0, 0.0, 1.0],
                    14,
                );
                draw_text_pos(
                    "X",
                    [
                        ((WINDOW_WIDTH + WIDTH) / 2 + weapon_set.len() as u32 * 32 / 2 + 6) as f64,
                        (WINDOW_HEIGHT * 3 / 4) as f64,
                    ],
                    [1.0, 1.0, 0.0, 1.0],
                    14,
                );

                // Display weapon selection
                use piston_window::math::translate;
                let centerize = translate([
                    -((assets.sphere_tex.get_width() * weapon_set.len() as u32) as f64 / 2.),
                    -(assets.sphere_tex.get_height() as f64 / 2.),
                ]);
                for (i, v) in weapon_set.iter().enumerate() {
                    let sphere_image = if v.1 == weapon {
                        Image::new_color([v.2[0], v.2[1], v.2[2], 1.])
                    } else {
                        Image::new_color([0.5 * v.2[0], 0.5 * v.2[1], 0.5 * v.2[2], 1.])
                    };
                    let transl = translate([
                        ((WINDOW_WIDTH + WIDTH) / 2 + i as u32 * 32) as f64,
                        (WINDOW_HEIGHT * 3 / 4) as f64,
                    ]);
                    let transform =
                        (Matrix(context.transform) * Matrix(transl) * Matrix(centerize)).0;
                    sphere_image.draw(&assets.sphere_tex, &context.draw_state, transform, graphics);
                    let weapons_image = sphere_image
                        .color(if v.1 == weapon {
                            [1., 1., 1., 1.]
                        } else {
                            [0.5, 0.5, 0.5, 1.]
                        })
                        .src_rect([
                            v.0 as f64 * 32.,
                            0.,
                            32.,
                            assets.weapons_tex.get_height() as f64,
                        ]);
                    weapons_image.draw(
                        &assets.weapons_tex,
                        &context.draw_state,
                        transform,
                        graphics,
                    );
                }

                // Display player lives
                for i in 0..player.lives {
                    let width = assets.player_tex.get_width();
                    let height = assets.player_tex.get_height();
                    let transl = translate([
                        (WINDOW_WIDTH - (i + 1) as u32 * width) as f64,
                        (WINDOW_HEIGHT - height) as f64,
                    ]);
                    let transform = (Matrix(context.transform) * Matrix(transl)).0;
                    image(&assets.player_tex, transform, graphics);
                }
            });
        }
    }
}

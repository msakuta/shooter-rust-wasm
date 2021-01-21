use game_logic::{
    consts::*,
    entity::{
        Assets, BulletBase, DeathReason, Enemy, Entity, Item, Matrix, Player, Projectile,
        TempEntity, Weapon,
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

    let [mut shots_bullet, mut shots_missile] = [0, 0];

    let [mut key_up, mut key_down, mut key_left, mut key_right, mut key_shoot, mut key_change, mut key_pause] =
        [false; 7];

    let mut weapon = Weapon::Bullet;

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

                // id_gen and rng must be passed as arguments since they are mutable
                // borrows and needs to be released for each iteration.
                // These variables are used in between multiple invocation of this closure.
                let mut add_tent =
                    |is_bullet, pos: &[f64; 2], id_gen: &mut u32, rng: &mut ThreadRng| {
                        let mut ent = Entity::new(
                            id_gen,
                            [
                                pos[0] + 4. * (rng.gen::<f64>() - 0.5),
                                pos[1] + 4. * (rng.gen::<f64>() - 0.5),
                            ],
                            [0., 0.],
                        )
                        .rotation(rng.gen::<f32>() * 2. * std::f32::consts::PI);
                        let (playback_rate, max_frames) = if is_bullet { (2, 8) } else { (4, 6) };
                        ent = ent.health((max_frames * playback_rate) as i32);

                        tent.push(TempEntity {
                            base: ent,
                            texture: if is_bullet {
                                &assets.explode_tex
                            } else {
                                &assets.explode2_tex
                            },
                            max_frames,
                            width: if is_bullet { 16 } else { 32 },
                            playback_rate,
                        })
                    };

                if !game_over && !paused {
                    if key_up {
                        player.move_up()
                    }
                    if key_down {
                        player.move_down()
                    }
                    if key_left {
                        player.move_left()
                    }
                    if key_right {
                        player.move_right()
                    }

                    let shoot_period = if let Weapon::Bullet = weapon { 5 } else { 50 };

                    if Weapon::Bullet == weapon || Weapon::Missile == weapon {
                        if key_shoot && player.cooldown == 0 {
                            let level = player.power_level() as i32;
                            player.cooldown += shoot_period;
                            for i in -1 - level..2 + level {
                                let speed = if let Weapon::Bullet = weapon {
                                    BULLET_SPEED
                                } else {
                                    MISSILE_SPEED
                                };
                                let mut ent =
                                    Entity::new(&mut id_gen, player.base.pos, [i as f64, -speed])
                                        .rotation((i as f32).atan2(speed as f32));
                                if let Weapon::Bullet = weapon {
                                    shots_bullet += 1;
                                    ent = ent.blend(Blend::Add);
                                    bullets.insert(ent.id, Projectile::Bullet(BulletBase(ent)));
                                } else {
                                    shots_missile += 1;
                                    ent = ent.health(5);
                                    bullets.insert(
                                        ent.id,
                                        Projectile::Missile {
                                            base: BulletBase(ent),
                                            target: 0,
                                            trail: vec![],
                                        },
                                    );
                                }
                            }
                        }
                    }
                    else if Weapon::Light == weapon && key_shoot {
                        // Apparently Piston doesn't allow vertex colored rectangle, we need to 
                        // draw multiple lines in order to display gradual change in color.
                        for i in -3..4 {
                            let f = (4. - (i as i32).abs() as f32) / 4.;
                            line([f / 3., 0.5 + f / 2., 1., f],
                                1.,
                                [player.base.pos[0] + i as f64, player.base.pos[1],
                                player.base.pos[0] + i as f64, 0.],
                                context.transform, graphics);
                        }
                        for enemy in enemies.iter_mut() {
                            if enemy.test_hit([player.base.pos[0] - LIGHT_WIDTH, 0., player.base.pos[0] + LIGHT_WIDTH, player.base.pos[1]]) {
                                add_tent(true, &enemy.get_base().pos, &mut id_gen, &mut rng);
                                enemy.damage(1 + player.power_level() as i32);
                            }
                        }
                    }
                    else if Weapon::Lightning == weapon && key_shoot {
                        let col = [1.,1.,1.,1.];
                        let col2 = [1.,0.5,1.,0.25];
                        let nmax = std::cmp::min((player.power_level() + 1 + time % 2) / 2, 31);

                        // Random walk with momentum
                        fn next_lightning(rng: &mut SmallRng, a: &mut [f64; 4]){
                            a[2] += LIGHTNING_ACCEL * (rng.gen::<f64>() - 0.5) - a[2] * LIGHTNING_FEEDBACK;
                            a[3] += LIGHTNING_ACCEL * (rng.gen::<f64>() - 0.5) - a[3] * LIGHTNING_FEEDBACK;
                            a[0] += a[2];
                            a[1] += a[3];
                        }

                        for _ in 0..nmax {
                            // Use the same seed twice to reproduce random sequence
                            let seed = {
                                let mut seed: <SmallRng as SeedableRng>::Seed = Default::default();
                                rng.fill_bytes(&mut seed);
                                seed
                            };

                            // Lambda to call the same lightning sequence twice, first pass for detecting hit enemy
                            // and second pass for rendering.
                            let lightning = |seed: &<SmallRng as SeedableRng>::Seed, length: u32, f: &mut dyn FnMut(&[f64; 4]) -> bool| {
                                let mut rng2 = SmallRng::from_seed(*seed);
                                let mut a = [player.base.pos[0], player.base.pos[1], 0., -16.];
                                for i in 0..length {
                                    let ox = a[0];
                                    let oy = a[1];
                                    next_lightning(&mut rng2, &mut a);
                                    let segment = [ox, oy, a[0], a[1]];
                                    if !f(&segment) {
                                        return i;
                                    }
                                }
                                length
                            };

                            let length = lightning(&seed, LIGHTNING_VERTICES, &mut |segment: &[f64; 4]| {
                                let b = [segment[2], segment[3]];
                                for enemy in enemies.iter_mut() {
                                    let ebb = enemy.get_bb();
                                    if ebb[0] < b[0] + 4. && b[0] - 4. <= ebb[2] && ebb[1] < b[1] + 4. && b[1] - 4. <= ebb[3] {
                                        enemy.damage(2 + rng.gen_range(0, 3));
                                        add_tent(true, &b, &mut id_gen, &mut rng);
                                        return false;
                                    }
                                }
                                return true;
                            });
                            let hit = length != LIGHTNING_VERTICES;

                            lightning(&seed, length, &mut |segment: &[f64; 4]| {
                                line(if hit { col } else { col2 }, if hit { 2. } else { 1. }, *segment, context.transform, graphics);
                                true
                            });
                        }
                    }
                    if player.cooldown < 1 {
                        player.cooldown = 0;
                    } else {
                        player.cooldown -= 1;
                    }
                }

                let wave_period = 1024;

                if !game_over {
                    if player.invtime == 0 || disptime % 2 == 0 {
                        player
                            .base
                            .draw_tex(&context, graphics, &assets.player_tex, None);
                    }
                }

                if !paused {
                    time += 1;
                }
                disptime += 1;

                let mut bullets_to_delete: Vec<u32> = Vec::new();
                for (i, b) in &mut bullets.iter_mut() {
                    if !paused {
                        if let Some(death_reason) = b.animate_bullet(&mut enemies, &mut player) {
                            bullets_to_delete.push(*i);

                            let base = b.get_base();

                            match death_reason {
                                DeathReason::Killed | DeathReason::HitPlayer => add_tent(
                                    if let Projectile::Missile { .. } = b {
                                        false
                                    } else {
                                        true
                                    },
                                    &base.0.pos,
                                    &mut id_gen,
                                    &mut rng,
                                ),
                                _ => {}
                            }

                            if let DeathReason::HitPlayer = death_reason {
                                if player.invtime == 0 && !game_over && 0 < player.lives {
                                    player.lives -= 1;
                                    if player.lives == 0 {
                                        game_over = true;
                                    } else {
                                        player.invtime = PLAYER_INVINCIBLE_TIME;
                                    }
                                }
                            }
                        }
                    }

                    b.draw(&context, graphics, &assets);
                }

                for i in bullets_to_delete.iter() {
                    if let Some(b) = bullets.remove(i) {
                        println!(
                            "Deleted {} id={}, {} / {}",
                            b.get_type(),
                            b.get_base().0.id,
                            *i,
                            bullets.len()
                        );
                    } else {
                        debug_assert!(false, "All keys must exist in bullets");
                    }
                }

                bullets_to_delete.clear();

                // Right side bar
                rectangle(
                    [0.20, 0.20, 0.4, 1.],
                    [
                        WIDTH as f64,
                        0.,
                        (WINDOW_WIDTH - WIDTH) as f64,
                        WINDOW_HEIGHT as f64,
                    ],
                    context.transform,
                    graphics,
                );

                rectangle(
                    [0., 0.5, 0.4, 1.],
                    [
                        WIDTH as f64,
                        (3) as f64 * 12.0 + 4.,
                        player.power as f64,
                        8.,
                    ],
                    context.transform,
                    graphics,
                );

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

                if paused {
                    draw_text_pos(
                        "PAUSED",
                        [(WIDTH / 2 - 80) as f64, (HEIGHT / 2) as f64],
                        [1.0, 1.0, 0.0, 1.0],
                        20,
                    );
                }

                if game_over {
                    let color = [1.0, 1.0, 1.0, 1.0];
                    draw_text_pos(
                        "GAME OVER",
                        [(WIDTH / 2 - 80) as f64, (HEIGHT * 3 / 4) as f64],
                        color,
                        20,
                    );
                    draw_text_pos(
                        "Press Space to Start",
                        [(WIDTH / 2 - 110) as f64, (HEIGHT * 3 / 4 + 20) as f64],
                        color,
                        20,
                    );
                }

                let mut draw_text = |s: &str, line: i32| {
                    draw_text_pos(
                        s,
                        [WIDTH as f64, (line + 1) as f64 * 12.0],
                        [0.0, 1.0, 0.0, 1.0],
                        12,
                    )
                };

                draw_text(&format!("Frame: {}", time), 0);
                draw_text(&format!("Score: {}", player.score), 1);
                draw_text(&format!("Kills: {}", player.kills), 2);
                draw_text(
                    &format!("Power: {}, Level: {}", player.power, player.power_level()),
                    3,
                );
                draw_text(
                    &format!(
                        "Wave: {} Level: {}",
                        time / wave_period,
                        player.difficulty_level()
                    ),
                    4,
                );
                draw_text(&format!("shots_bullet: {}", shots_bullet), 5);
                draw_text(&format!("shots_missile: {}", shots_missile), 6);

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
        } else {
            let mut toggle_key = |opt: Option<Button>, tf: bool| {
                if let Some(Button::Keyboard(key)) = opt {
                    match key {
                        Key::Up | Key::W => key_up = tf,
                        Key::Down | Key::S => key_down = tf,
                        Key::Left | Key::A => key_left = tf,
                        Key::Right | Key::D => key_right = tf,
                        Key::C => key_shoot = tf,
                        Key::Z | Key::X => {
                            if !key_change && tf && !game_over {
                                use Weapon::*;
                                let weapon_set = [
                                    ("Bullet", Bullet),
                                    ("Light", Light),
                                    ("Missile", Missile),
                                    ("Lightning", Lightning),
                                ];
                                let (name, next_weapon) = match weapon {
                                    Bullet => {
                                        if key == Key::X {
                                            &weapon_set[1]
                                        } else {
                                            &weapon_set[3]
                                        }
                                    }
                                    Light => {
                                        if key == Key::X {
                                            &weapon_set[2]
                                        } else {
                                            &weapon_set[0]
                                        }
                                    }
                                    Missile => {
                                        if key == Key::X {
                                            &weapon_set[3]
                                        } else {
                                            &weapon_set[1]
                                        }
                                    }
                                    Lightning => {
                                        if key == Key::X {
                                            &weapon_set[0]
                                        } else {
                                            &weapon_set[2]
                                        }
                                    }
                                };
                                weapon = next_weapon.clone();
                                println!("Weapon switched: {}", name);
                            }
                            key_change = tf;
                        }
                        Key::P => {
                            if !key_pause && tf {
                                paused = !paused;
                            }
                            key_pause = tf;
                        }
                        Key::Space => {
                            if tf {
                                items.clear();
                                enemies.clear();
                                bullets.clear();
                                tent.clear();
                                time = 0;
                                id_gen = 0;
                                player.reset();
                                shots_bullet = 0;
                                shots_missile = 0;
                                paused = false;
                                game_over = false;
                            }
                        }
                        Key::G => {
                            if cfg!(debug_assertions) && tf {
                                player.score += 1000;
                            }
                        }
                        Key::H => {
                            if cfg!(debug_assertions) && tf {
                                player.power += 16;
                            }
                        }
                        _ => {}
                    }
                }
            };
            toggle_key(event.press_args(), true);
            toggle_key(event.release_args(), false);
        }
    }
}

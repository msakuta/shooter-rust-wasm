use fps_counter::FPSCounter;
use game_logic::{
    assets_piston::Assets,
    consts::*,
    entity::{Entity, Matrix, TempEntity, Weapon, WEAPON_SET},
    ShooterError, ShooterState,
};
use piston_window::math::translate;
use piston_window::*;

fn main() -> Result<(), ShooterError> {
    let mut disptime = 0;
    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow =
        WindowSettings::new("Shooter Rust", [WINDOW_WIDTH, WINDOW_HEIGHT])
            .exit_on_esc(true)
            .opengl(opengl)
            .build()
            .unwrap();

    window.events.set_max_fps(60);
    window.events.set_ups(60);

    let (assets, mut glyphs) = Assets::new(&mut window);

    let mut state = ShooterState::default();

    let [mut key_up, mut key_down, mut key_left, mut key_right, mut key_shoot, mut key_change, mut key_pause] =
        [false; 7];

    fn limit_viewport(viewport: &Viewport, ratio: f64, wwidth: u32, wheight: u32) -> Viewport {
        let vp_ratio = (viewport.rect[2] - viewport.rect[0]) as f64
            / (viewport.rect[3] - viewport.rect[0]) as f64;
        let mut newvp = *viewport;
        newvp.window_size[0] = (wwidth as f64 * (vp_ratio / ratio).max(1.)) as u32;
        newvp.window_size[1] = (wheight as f64 * (ratio / vp_ratio).max(1.)) as u32;
        #[cfg(debug_assertions)]
        for (vp, name) in [(viewport, "Old"), (&newvp, "New")].iter() {
            println!(
                "{} Context: ratio: {} vp.rect: {:?} vp.draw: {:?} vp.window: {:?}",
                name, ratio, vp.rect, vp.draw_size, vp.window_size
            );
        }
        newvp
    }

    let mut last_lightning = vec![];

    let mut fps_counter = FPSCounter::default();
    let mut ups_counter = FPSCounter::default();
    let mut last_ups = 0;

    while let Some(event) = window.next() {
        match event {
            Event::Loop(Loop::Render(_)) => {
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

                        image(&*assets.bg, wnd_context.transform, graphics);

                        context = Context::new_viewport(limit_viewport(
                            &viewport,
                            ratio,
                            WINDOW_WIDTH,
                            WINDOW_HEIGHT,
                        ));
                    }

                    if !state.game_over && !state.paused {
                        // Use the same seed twice to reproduce random sequence
                        let weapon = state.player.weapon;

                        if Weapon::Light == weapon && key_shoot {
                            // Apparently Piston doesn't allow vertex colored rectangle, we need to
                            // draw multiple lines in order to display gradual change in color.
                            for i in -3..4 {
                                let f = (4. - (i as i32).abs() as f32) / 4.;
                                line(
                                    [f / 3., 0.5 + f / 2., 1., f],
                                    1.,
                                    [
                                        state.player.base.pos[0] + i as f64,
                                        state.player.base.pos[1],
                                        state.player.base.pos[0] + i as f64,
                                        0.,
                                    ],
                                    context.transform,
                                    graphics,
                                );
                            }
                        }
                    }

                    let col = [1., 1., 1., 1.];
                    let col2 = [1., 0.5, 1., 0.25];
                    for (seed, nmax) in &last_lightning {
                        state.lightning(
                            *seed,
                            Some(*nmax),
                            &mut |state: &mut ShooterState, seed| {
                                let length = state.lightning_branch(
                                    seed,
                                    LIGHTNING_VERTICES,
                                    &mut |state: &mut ShooterState, segment: &[f64; 4]| {
                                        let b = [segment[2], segment[3]];
                                        for enemy in state.enemies.iter_mut() {
                                            let ebb = enemy.get_bb();
                                            if ebb[0] < b[0] + 4.
                                                && b[0] - 4. <= ebb[2]
                                                && ebb[1] < b[1] + 4.
                                                && b[1] - 4. <= ebb[3]
                                            {
                                                return false;
                                            }
                                        }
                                        true
                                    },
                                );
                                let hit = length != LIGHTNING_VERTICES;

                                state.lightning_branch(
                                    seed,
                                    length,
                                    &mut |_: &mut ShooterState, segment: &[f64; 4]| {
                                        line(
                                            if hit { col } else { col2 },
                                            if hit { 2. } else { 1. },
                                            *segment,
                                            context.transform,
                                            graphics,
                                        );
                                        true
                                    },
                                );
                            },
                        );
                    }

                    if !state.paused {
                        last_lightning.clear();
                    }

                    let wave_period = state.gen_enemies();

                    if !state.game_over && (state.player.invtime == 0 || disptime % 2 == 0) {
                        state
                            .player
                            .base
                            .draw_tex(&context, graphics, &assets.player_tex, None);
                    }

                    disptime += 1;

                    state.draw_items(&context, graphics, &assets);

                    state.draw_enemies(&context, graphics, &assets);

                    state.draw_bullets(&context, graphics, &assets);

                    state.draw_tents(&context, graphics);

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
                        [WIDTH as f64, 3. * 12.0 + 4., state.player.power as f64, 8.],
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

                    if state.paused {
                        draw_text_pos(
                            "PAUSED",
                            [(WIDTH / 2 - 80) as f64, (HEIGHT / 2) as f64],
                            [1.0, 1.0, 0.0, 1.0],
                            20,
                        );
                    }

                    if state.game_over {
                        let color = [1.0, 1.0, 1.0, 1.0];
                        draw_text_pos(
                            "GAME OVER",
                            [(WIDTH / 2 - 80) as f64, (HEIGHT * 3 / 4) as f64],
                            color,
                            20,
                        );
                        draw_text_pos(
                            "Press N to Restart",
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

                    draw_text(&format!("Frame: {}", state.time), 0);
                    draw_text(&format!("Score: {}", state.player.score), 1);
                    draw_text(&format!("Kills: {}", state.player.kills), 2);
                    draw_text(
                        &format!(
                            "Power: {}, Level: {}",
                            state.player.power,
                            state.player.power_level()
                        ),
                        3,
                    );
                    draw_text(
                        &format!(
                            "Wave: {} Level: {}",
                            state.time / wave_period,
                            state.player.difficulty_level()
                        ),
                        4,
                    );
                    draw_text(&format!("shots_bullet: {}", state.shots_bullet), 5);
                    draw_text(&format!("shots_missile: {}", state.shots_missile), 6);

                    draw_text_pos(
                        "Z",
                        [
                            ((WINDOW_WIDTH + WIDTH) / 2 - WEAPON_SET.len() as u32 * 32 / 2 - 16)
                                as f64,
                            (WINDOW_HEIGHT * 3 / 4) as f64,
                        ],
                        [1.0, 1.0, 0.0, 1.0],
                        14,
                    );
                    draw_text_pos(
                        "X",
                        [
                            ((WINDOW_WIDTH + WIDTH) / 2 + WEAPON_SET.len() as u32 * 32 / 2 + 6)
                                as f64,
                            (WINDOW_HEIGHT * 3 / 4) as f64,
                        ],
                        [1.0, 1.0, 0.0, 1.0],
                        14,
                    );

                    draw_text_pos(
                        &format!("fps: {}", fps_counter.tick()),
                        [WIDTH as f64, (WINDOW_HEIGHT - 36) as f64],
                        [1.0, 1.0, 0.0, 1.0],
                        14,
                    );

                    draw_text_pos(
                        &format!("ups: {}", last_ups),
                        [WIDTH as f64, (WINDOW_HEIGHT - 24) as f64],
                        [1.0, 1.0, 0.0, 1.0],
                        14,
                    );

                    // Display weapon selection
                    let centerize = translate([
                        -((assets.sphere_tex.get_width() * WEAPON_SET.len() as u32) as f64 / 2.),
                        -(assets.sphere_tex.get_height() as f64 / 2.),
                    ]);
                    for (i, v) in WEAPON_SET.iter().enumerate() {
                        let sphere_image = if v.1 == state.player.weapon {
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
                        sphere_image.draw(
                            &*assets.sphere_tex,
                            &context.draw_state,
                            transform,
                            graphics,
                        );
                        let weapons_image = sphere_image
                            .color(if v.1 == state.player.weapon {
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
                            &*assets.weapons_tex,
                            &context.draw_state,
                            transform,
                            graphics,
                        );
                    }

                    // Display player lives
                    for i in 0..state.player.lives {
                        let width = assets.player_tex.get_width();
                        let height = assets.player_tex.get_height();
                        let transl = translate([
                            (WINDOW_WIDTH - (i + 1) as u32 * width) as f64,
                            (WINDOW_HEIGHT - height - 48) as f64,
                        ]);
                        let transform = (Matrix(context.transform) * Matrix(transl)).0;
                        image(&*assets.player_tex, transform, graphics);
                    }
                });
            }
            Event::Loop(Loop::Update(_)) => {
                last_ups = ups_counter.tick();

                // id_gen and rng must be passed as arguments since they are mutable
                // borrows and needs to be released for each iteration.
                // These variables are used in between multiple invocation of this closure.
                let mut add_tent = |is_bullet, pos: &[f64; 2], state: &mut ShooterState| {
                    let mut ent = Entity::new(
                        &mut state.id_gen,
                        [
                            pos[0] + 4. * (state.rng.gen() - 0.5),
                            pos[1] + 4. * (state.rng.gen() - 0.5),
                        ],
                        [0., 0.],
                    )
                    .rotation(state.rng.gen() as f32 * 2. * std::f32::consts::PI);
                    let (playback_rate, max_frames) = if is_bullet { (2, 8) } else { (4, 6) };
                    ent = ent.health((max_frames * playback_rate) as i32);

                    state.tent.push(TempEntity {
                        base: ent,
                        texture: if is_bullet {
                            assets.explode_tex.clone()
                        } else {
                            assets.explode2_tex.clone()
                        },
                        max_frames,
                        width: if is_bullet { 16 } else { 32 },
                        playback_rate,
                    })
                };

                if !state.game_over && !state.paused {
                    if key_up {
                        state.player.move_up()
                    }
                    if key_down {
                        state.player.move_down()
                    }
                    if key_left {
                        state.player.move_left()
                    }
                    if key_right {
                        state.player.move_right()
                    }

                    let seed = state.rng.nexti();

                    last_lightning.push((seed, state.try_shoot(key_shoot, seed, &mut add_tent)));

                    if state.player.cooldown < 1 {
                        state.player.cooldown = 0;
                    } else {
                        state.player.cooldown -= 1;
                    }

                    if 0 < state.player.invtime {
                        state.player.invtime -= 1;
                    }
                }

                if !state.paused {
                    state.time += 1;
                }

                state.animate_items();

                state.animate_enemies();

                state.animate_bullets(&mut add_tent);

                state.animate_tents();
            }
            Event::Input(Input::Button(_)) => {
                let mut toggle_key = |opt: Option<Button>, tf: bool| -> Result<(), ShooterError> {
                    if let Some(Button::Keyboard(key)) = opt {
                        match key {
                            Key::Up | Key::W => key_up = tf,
                            Key::Down | Key::S => key_down = tf,
                            Key::Left | Key::A => key_left = tf,
                            Key::Right | Key::D => key_right = tf,
                            Key::Space => key_shoot = tf,
                            Key::Z | Key::X => {
                                if !key_change && tf && !state.game_over {
                                    state.player.weapon = if key == Key::X {
                                        state.player.weapon.next()
                                    } else {
                                        state.player.weapon.prev()
                                    };
                                    println!("Weapon switched: {}", state.player.weapon);
                                }
                                key_change = tf;
                            }
                            Key::P => {
                                if !key_pause && tf {
                                    state.paused = !state.paused;
                                }
                                key_pause = tf;
                            }
                            Key::N => {
                                if tf {
                                    state.restart()?;
                                    state.shots_bullet = 0;
                                    state.shots_missile = 0;
                                }
                            }
                            Key::G => {
                                if cfg!(debug_assertions) && tf {
                                    state.player.score += 1000;
                                }
                            }
                            Key::H => {
                                if cfg!(debug_assertions) && tf {
                                    state.player.power += 16;
                                }
                            }
                            _ => {}
                        }
                    }
                    Ok(())
                };
                toggle_key(event.press_args(), true)?;
                toggle_key(event.release_args(), false)?;
            }
            _ => (),
        }
    }

    Ok(())
}

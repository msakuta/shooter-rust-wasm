use game_logic::{
    consts::*,
    entity::{
        Assets, BulletBase, DeathReason, Enemy, EnemyBase, Entity, Item, Matrix, Player,
        Projectile, ShieldedBoss, TempEntity, Weapon,
    },
    xor128::Xor128,
    ShooterError, ShooterState,
};
use piston_window::draw_state::Blend;
use piston_window::math::{rotate_radians, scale, translate};
use piston_window::*;
use rand::prelude::*;
use std::collections::HashMap;

fn main() -> Result<(), ShooterError> {
    let mut disptime = 0;
    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow =
        WindowSettings::new("Shooter Rust", [WINDOW_WIDTH, WINDOW_HEIGHT])
            .exit_on_esc(true)
            .opengl(opengl)
            .build()
            .unwrap();

    let (assets, mut glyphs) = Assets::new(&mut window);

    let mut state = ShooterState::new(None);

    let mut enemies = Vec::<Enemy>::new();

    let mut items = Vec::<Item>::new();

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
                    |is_bullet, pos: &[f64; 2], id_gen: &mut u32, rng: &mut Xor128| {
                        let mut ent = Entity::new(
                            id_gen,
                            [
                                pos[0] + 4. * (rng.next() - 0.5),
                                pos[1] + 4. * (rng.next() - 0.5),
                            ],
                            [0., 0.],
                        )
                        .rotation(rng.next() as f32 * 2. * std::f32::consts::PI);
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

                    // Use the same seed twice to reproduce random sequence
                    let seed = state.rng.nexti();

                    state.try_shoot(key_shoot, &weapon, seed, &mut enemies, &mut add_tent);

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
                    } else if Weapon::Lightning == weapon && key_shoot {
                        let col = [1., 1., 1., 1.];
                        let col2 = [1., 0.5, 1., 0.25];
                        state.lightning(seed, &mut |state: &mut ShooterState, seed| {
                            let length = state.lightning_branch(
                                seed,
                                LIGHTNING_VERTICES,
                                &mut |_: &mut ShooterState, segment: &[f64; 4]| {
                                    let b = [segment[2], segment[3]];
                                    for enemy in enemies.iter_mut() {
                                        let ebb = enemy.get_bb();
                                        if ebb[0] < b[0] + 4.
                                            && b[0] - 4. <= ebb[2]
                                            && ebb[1] < b[1] + 4.
                                            && b[1] - 4. <= ebb[3]
                                        {
                                            return false;
                                        }
                                    }
                                    return true;
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
                        });
                    }

                    if state.player.cooldown < 1 {
                        state.player.cooldown = 0;
                    } else {
                        state.player.cooldown -= 1;
                    }
                }

                let wave_period = 1024;
                if !paused {
                    let dice = 256;
                    let wave = state.time % wave_period;
                    if wave < wave_period * 3 / 4 {
                        let [enemy_count, boss_count, shielded_boss_count, spiral_count] =
                            enemies.iter().fold([0; 4], |mut c, e| match e {
                                Enemy::Enemy1(_) => {
                                    c[0] += 1;
                                    c
                                }
                                Enemy::Boss(_) => {
                                    c[1] += 1;
                                    c
                                }
                                Enemy::ShieldedBoss(_) => {
                                    c[2] += 1;
                                    c
                                }
                                Enemy::SpiralEnemy(_) => {
                                    c[3] += 1;
                                    c
                                }
                            });
                        let gen_amount = state.player.difficulty_level() * 4 + 8;
                        let mut i = rng.gen_range(0, dice);
                        while i < gen_amount {
                            let weights = [
                                if enemy_count < 128 {
                                    if state.player.score < 1024 {
                                        64
                                    } else {
                                        16
                                    }
                                } else {
                                    0
                                },
                                if boss_count < 32 { 4 } else { 0 },
                                if shielded_boss_count < 32 {
                                    std::cmp::min(4, state.player.difficulty_level())
                                } else {
                                    0
                                },
                                if spiral_count < 4 { 4 } else { 0 },
                            ];
                            let allweights = weights.iter().fold(0, |sum, x| sum + x);
                            let accum = {
                                let mut accum = [0; 4];
                                let mut accumulator = 0;
                                for (i, e) in weights.iter().enumerate() {
                                    accumulator += e;
                                    accum[i] = accumulator;
                                }
                                accum
                            };

                            if 0 < allweights {
                                let dice = rng.gen_range(0, allweights);
                                let (pos, velo) = match rng.gen_range(0, 3) {
                                    0 => {
                                        // top
                                        (
                                            [rng.gen_range(0., WIDTH as f64), 0.],
                                            [rng.gen::<f64>() - 0.5, rng.gen::<f64>() * 0.5],
                                        )
                                    }
                                    1 => {
                                        // left
                                        (
                                            [0., rng.gen_range(0., WIDTH as f64)],
                                            [rng.gen::<f64>() * 0.5, rng.gen::<f64>() - 0.5],
                                        )
                                    }
                                    2 => {
                                        // right
                                        (
                                            [WIDTH as f64, rng.gen_range(0., WIDTH as f64)],
                                            [-rng.gen::<f64>() * 0.5, rng.gen::<f64>() - 0.5],
                                        )
                                    }
                                    _ => panic!("RNG returned out of range"),
                                };
                                if let Some(x) = accum.iter().position(|x| dice < *x) {
                                    enemies.push(match x {
                                        0 => Enemy::Enemy1(
                                            EnemyBase::new(&mut state.id_gen, pos, velo).health(3),
                                        ),
                                        1 => Enemy::Boss(
                                            EnemyBase::new(&mut state.id_gen, pos, velo).health(64),
                                        ),
                                        2 => Enemy::ShieldedBoss(ShieldedBoss::new(
                                            &mut state.id_gen,
                                            pos,
                                            velo,
                                        )),
                                        _ => Enemy::new_spiral(&mut state.id_gen, pos, velo),
                                    });
                                }
                            }
                            i += rng.gen_range(0, dice);
                        }
                    }
                }

                if !game_over {
                    if state.player.invtime == 0 || disptime % 2 == 0 {
                        state
                            .player
                            .base
                            .draw_tex(&context, graphics, &assets.player_tex, None);
                    }
                }

                if !paused {
                    state.time += 1;
                }
                disptime += 1;

                let mut to_delete: Vec<usize> = Vec::new();

                for (i, e) in &mut ((&mut items).iter_mut().enumerate()) {
                    if !paused {
                        if let Some(_) = e.animate(&mut state.player) {
                            to_delete.push(i);
                            continue;
                        }
                    }
                    e.draw(&context, graphics, &assets);
                }

                for i in to_delete.iter().rev() {
                    let dead = items.remove(*i);
                    println!(
                        "Deleted Item id={}: {} / {}",
                        dead.get_base().id,
                        *i,
                        items.len()
                    );
                }

                let mut to_delete: Vec<usize> = Vec::new();
                for (i, enemy) in &mut ((&mut enemies).iter_mut().enumerate()) {
                    if !paused {
                        let killed = {
                            if let Some(death_reason) = enemy.animate(&mut state) {
                                to_delete.push(i);
                                if let DeathReason::Killed = death_reason {
                                    true
                                } else {
                                    false
                                }
                            } else {
                                false
                            }
                        };
                        if killed {
                            state.player.kills += 1;
                            state.player.score += if enemy.is_boss() { 10 } else { 1 };
                            if rng.gen_range(0, 100) < 20 {
                                let ent =
                                    Entity::new(&mut state.id_gen, enemy.get_base().pos, [0., 1.]);
                                items.push(enemy.drop_item(ent));
                            }
                            continue;
                        }
                    }
                    enemy.draw(&context, graphics, &assets);
                }

                for i in to_delete.iter().rev() {
                    let dead = enemies.remove(*i);
                    println!(
                        "Deleted Enemy {} id={}: {} / {}",
                        match dead {
                            Enemy::Enemy1(_) => "enemy",
                            Enemy::Boss(_) => "boss",
                            Enemy::ShieldedBoss(_) => "ShieldedBoss",
                            Enemy::SpiralEnemy(_) => "SpiralEnemy",
                        },
                        dead.get_id(),
                        *i,
                        enemies.len()
                    );
                }

                let mut bullets_to_delete: Vec<u32> = Vec::new();
                for (i, b) in &mut state.bullets.iter_mut() {
                    if !paused {
                        if let Some(death_reason) =
                            b.animate_bullet(&mut enemies, &mut state.player)
                        {
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
                                    &mut state.id_gen,
                                    &mut state.rng,
                                ),
                                _ => {}
                            }

                            if let DeathReason::HitPlayer = death_reason {
                                if state.player.invtime == 0 && !game_over && 0 < state.player.lives
                                {
                                    state.player.lives -= 1;
                                    if state.player.lives == 0 {
                                        game_over = true;
                                    } else {
                                        state.player.invtime = PLAYER_INVINCIBLE_TIME;
                                    }
                                }
                            }
                        }
                    }

                    b.draw(&context, graphics, &assets);
                }

                for i in bullets_to_delete.iter() {
                    if let Some(b) = state.bullets.remove(i) {
                        println!(
                            "Deleted {} id={}, {} / {}",
                            b.get_type(),
                            b.get_base().0.id,
                            *i,
                            state.bullets.len()
                        );
                    } else {
                        debug_assert!(false, "All keys must exist in bullets");
                    }
                }

                bullets_to_delete.clear();

                to_delete.clear();
                for (i, e) in &mut ((&mut tent).iter_mut().enumerate()) {
                    if !paused {
                        if let Some(_) = e.animate_temp() {
                            to_delete.push(i);
                            continue;
                        }
                    }
                    e.draw_temp(&context, graphics);
                }

                for i in to_delete.iter().rev() {
                    tent.remove(*i);
                    //println!("Deleted tent {} / {}", *i, bullets.len());
                }

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
                        state.player.power as f64,
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
                for i in 0..state.player.lives {
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
            let mut toggle_key = |opt: Option<Button>, tf: bool| -> Result<(), ShooterError> {
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
                                state.restart()?;
                                items.clear();
                                enemies.clear();
                                state.bullets.clear();
                                tent.clear();
                                shots_bullet = 0;
                                shots_missile = 0;
                                paused = false;
                                game_over = false;
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
    }

    Ok(())
}

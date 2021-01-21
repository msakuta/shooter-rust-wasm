use cgmath::{Matrix3, Matrix4, Vector3};
use js_sys::JsString;
use slice_of_array::SliceFlatExt;
use std::rc::Rc;
use std::{collections::HashMap, vec};
use vecmath::{vec2_add, vec2_normalized, vec2_scale, vec2_sub};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{
    Element, HtmlImageElement, WebGlBuffer, WebGlProgram, WebGlRenderingContext as GL, WebGlShader,
    WebGlTexture,
};

macro_rules! console_log {
    ($fmt:expr, $($arg1:expr),*) => {
        crate::log(&format!($fmt, $($arg1),+))
    };
    ($fmt:expr) => {
        crate::log($fmt)
    }
}

/// format-like macro that returns js_sys::String
macro_rules! js_str {
    ($fmt:expr, $($arg1:expr),*) => {
        JsValue::from_str(&format!($fmt, $($arg1),+))
    };
    ($fmt:expr) => {
        JsValue::from_str($fmt)
    }
}

/// format-like macro that returns Err(js_sys::String)
macro_rules! js_err {
    ($fmt:expr, $($arg1:expr),*) => {
        Err(JsValue::from_str(&format!($fmt, $($arg1),+)))
    };
    ($fmt:expr) => {
        Err(JsValue::from_str($fmt))
    }
}

use game_logic::consts::*;
use game_logic::entity::{
    Assets, BulletBase, DeathReason, Enemy, EnemyBase, Entity, Item, Player, Projectile,
    ShaderBundle, ShieldedBoss, TempEntity, Weapon,
};
use game_logic::xor128::Xor128;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub(crate) fn log(s: &str);
}

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn document() -> web_sys::Document {
    window().document().unwrap()
}

fn get_context() -> GL {
    let document = document();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement =
        canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();

    canvas
        .get_context("webgl")
        .unwrap()
        .unwrap()
        .dyn_into::<GL>()
        .unwrap()
}

#[wasm_bindgen]
pub struct ShooterState(game_logic::ShooterState);

#[wasm_bindgen]
impl ShooterState {
    #[wasm_bindgen(constructor)]
    pub fn new(image_assets: js_sys::Array) -> Result<ShooterState, JsValue> {
        let side_panel = document().get_element_by_id("sidePanel").unwrap();
        let player_live_icons = (0..3)
            .map(|_| {
                let lives_icon = document().create_element("img")?;
                lives_icon.set_attribute(
                    "src",
                    &js_sys::Array::from(
                        &image_assets
                            .iter()
                            .find(|value| {
                                let array = js_sys::Array::from(value);
                                array.iter().next() == Some(JsValue::from_str("player"))
                            })
                            .unwrap(),
                    )
                    .to_vec()
                    .get(1)
                    .ok_or_else(|| JsValue::from_str("Couldn't find texture"))?
                    .as_string()
                    .unwrap(),
                )?;
                side_panel.append_child(&lives_icon)?;
                Ok(lives_icon)
            })
            .collect::<Result<Vec<Element>, JsValue>>()?;

        let context = get_context();

        let load_texture_local = |path| -> Result<Rc<WebGlTexture>, JsValue> {
            if let Some(value) = image_assets.iter().find(|value| {
                let array = js_sys::Array::from(value);
                array.iter().next() == Some(JsValue::from_str(path))
            }) {
                let array = js_sys::Array::from(&value).to_vec();
                load_texture(
                    &context,
                    &array
                        .get(1)
                        .ok_or_else(|| JsValue::from_str("Couldn't find texture"))?
                        .as_string()
                        .ok_or_else(|| {
                            JsValue::from_str(&format!(
                                "Couldn't convert value to String: {:?}",
                                path
                            ))
                        })?,
                )
            } else {
                Err(JsValue::from_str("Couldn't find texture"))
            }
        };

        let mut id_gen = 0;
        let mut player = Player::new(Entity::new(
            &mut id_gen,
            [FWIDTH / 2., FHEIGHT / 2.],
            [0., 0.],
        ));
        player.reset();

        Ok(Self(game_logic::ShooterState {
            time: 0,
            disptime: 0,
            paused: false,
            game_over: false,
            id_gen,
            player,
            enemies: vec![],
            items: vec![],
            bullets: HashMap::new(),
            tent: vec![],
            rng: Xor128::new(3232132),
            shots_bullet: 0,
            shots_missile: 0,
            shoot_pressed: false,
            left_pressed: false,
            right_pressed: false,
            up_pressed: false,
            down_pressed: false,
            player_live_icons,
            assets: Assets {
                world_transform: Matrix4::from_translation(Vector3::new(-1., 1., 0.))
                    * &Matrix4::from_nonuniform_scale(2. / FWIDTH, -2. / FHEIGHT, 1.),
                enemy_tex: load_texture_local("enemy")?,
                boss_tex: load_texture_local("boss")?,
                shield_tex: load_texture_local("shield")?,
                spiral_enemy_tex: load_texture_local("spiralEnemy")?,
                player_texture: load_texture_local("player")?,
                bullet_texture: load_texture_local("bullet")?,
                enemy_bullet_texture: load_texture_local("ebullet")?,
                phase_bullet_tex: load_texture_local("phaseBullet")?,
                spiral_bullet_tex: load_texture_local("spiralBullet")?,
                missile_tex: load_texture_local("missile")?,
                explode_tex: load_texture_local("explode")?,
                explode2_tex: load_texture_local("explode2")?,
                trail_tex: load_texture_local("trail")?,
                beam_tex: load_texture_local("beam")?,
                back_tex: load_texture_local("back")?,
                power_tex: load_texture_local("power")?,
                power2_tex: load_texture_local("power2")?,
                sphere_tex: load_texture_local("sphere")?,
                weapons_tex: load_texture_local("weapons")?,
                sprite_shader: None,
                trail_shader: None,
                rect_buffer: None,
                trail_buffer: None,
            },
        }))
    }

    pub fn key_down(&mut self, event: web_sys::KeyboardEvent) -> Result<JsString, JsValue> {
        println!("key: {}", event.key_code());
        match event.key_code() {
            32 => self.0.shoot_pressed = true,
            65 | 37 => self.0.left_pressed = true,
            68 | 39 => self.0.right_pressed = true,
            80 => {
                // P
                self.0.paused = !self.0.paused;
                let paused_element = document().get_element_by_id("paused").unwrap();
                paused_element.set_class_name(if self.0.paused {
                    "noselect"
                } else {
                    "noselect hidden"
                })
            }
            87 | 38 => self.0.up_pressed = true,
            83 | 40 => self.0.down_pressed = true,
            88 | 90 => {
                // Z or X
                use Weapon::*;
                let is_x = event.key_code() == 88;
                let weapon_set = [
                    ("Bullet", Bullet),
                    ("Light", Light),
                    ("Missile", Missile),
                    ("Lightning", Lightning),
                ];
                let (name, next_weapon) = match self.0.player.weapon {
                    Bullet => {
                        if is_x {
                            &weapon_set[1]
                        } else {
                            &weapon_set[3]
                        }
                    }
                    Light => {
                        if is_x {
                            &weapon_set[2]
                        } else {
                            &weapon_set[0]
                        }
                    }
                    Missile => {
                        if is_x {
                            &weapon_set[3]
                        } else {
                            &weapon_set[1]
                        }
                    }
                    Lightning => {
                        if is_x {
                            &weapon_set[0]
                        } else {
                            &weapon_set[2]
                        }
                    }
                };
                self.0.player.weapon = next_weapon.clone();
                println!("Weapon switched: {}", name);
            }
            _ => (),
        }
        Ok(JsString::from(self.0.player.weapon.to_string()))
    }

    pub fn key_up(&mut self, event: web_sys::KeyboardEvent) {
        console_log!("key: {}", event.key_code());
        match event.key_code() {
            32 => self.0.shoot_pressed = false,
            65 | 37 => self.0.left_pressed = false,
            68 | 39 => self.0.right_pressed = false,
            87 | 38 => self.0.up_pressed = false,
            83 | 40 => self.0.down_pressed = false,
            _ => (),
        }
    }

    pub fn restart(&mut self) -> Result<(), JsValue> {
        self.restart();

        for icon in &self.0.player_live_icons {
            icon.set_class_name("");
        }
        let game_over_element = document()
            .get_element_by_id("gameOver")
            .ok_or_else(|| js_str!("Game over element was not found"))?;
        game_over_element.set_class_name("hidden");

        Ok(())
    }

    pub fn start(&mut self) -> Result<(), JsValue> {
        let context = get_context();

        let vert_shader = compile_shader(
            &context,
            GL::VERTEX_SHADER,
            r#"
            attribute vec2 vertexData;
            uniform mat4 transform;
            uniform mat3 texTransform;
            varying vec2 texCoords;
            void main() {
                gl_Position = transform * vec4(vertexData.xy, 0.0, 1.0);

                texCoords = (texTransform * vec3((vertexData.xy - 1.) * 0.5, 1.)).xy;
            }
        "#,
        )?;
        let frag_shader = compile_shader(
            &context,
            GL::FRAGMENT_SHADER,
            r#"
            precision mediump float;

            varying vec2 texCoords;

            uniform sampler2D texture;

            void main() {
                vec4 texColor = texture2D( texture, vec2(texCoords.x, texCoords.y) );
                gl_FragColor = texColor;
            }
        "#,
        )?;
        let program = link_program(&context, &vert_shader, &frag_shader)?;
        context.use_program(Some(&program));

        let shader = ShaderBundle::new(&context, program);

        context.active_texture(GL::TEXTURE0);

        context.uniform1i(shader.texture_loc.as_ref(), 0);

        context.enable(GL::BLEND);
        context.blend_equation(GL::FUNC_ADD);
        context.blend_func(GL::SRC_ALPHA, GL::ONE_MINUS_SRC_ALPHA);

        self.0.assets.sprite_shader = Some(shader);

        let vert_shader = compile_shader(
            &context,
            GL::VERTEX_SHADER,
            r#"
            attribute vec4 vertexData;
            uniform mat4 transform;
            uniform mat3 texTransform;
            varying vec2 texCoords;
            void main() {
                gl_Position = transform * vec4(vertexData.xy, 0.0, 1.0);

                texCoords = (texTransform * vec3(vertexData.zw, 1.)).xy;
            }
        "#,
        )?;
        let frag_shader = compile_shader(
            &context,
            GL::FRAGMENT_SHADER,
            r#"
            precision mediump float;

            varying vec2 texCoords;

            uniform sampler2D texture;

            void main() {
                vec4 texColor = texture2D( texture, vec2(texCoords.x, texCoords.y) );
                gl_FragColor = texColor;
            }
        "#,
        )?;
        let program = link_program(&context, &vert_shader, &frag_shader)?;
        context.use_program(Some(&program));
        self.0.assets.trail_shader = Some(ShaderBundle::new(&context, program));

        context.active_texture(GL::TEXTURE0);
        context.uniform1i(
            self.0
                .assets
                .trail_shader
                .as_ref()
                .and_then(|s| s.texture_loc.as_ref()),
            0,
        );

        self.0.assets.trail_buffer =
            Some(context.create_buffer().ok_or("failed to create buffer")?);

        self.0.assets.rect_buffer = Some(context.create_buffer().ok_or("failed to create buffer")?);
        context.bind_buffer(GL::ARRAY_BUFFER, self.0.assets.rect_buffer.as_ref());
        let rect_vertices: [f32; 8] = [1., 1., -1., 1., -1., -1., 1., -1.];
        vertex_buffer_data(&context, &rect_vertices)?;

        context.clear_color(0.0, 0.0, 0.5, 1.0);

        Ok(())
    }

    pub fn render(&mut self) -> Result<(), JsValue> {
        let context = get_context();

        if self.0.time > 300000 {
            console_log!("All done!");

            // Drop our handle to this closure so that it will get cleaned
            // up once we return.
            // let _ = f.borrow_mut().take();
            return Ok(());
        }

        if !self.0.paused {
            self.0.time += 1;
        }
        self.0.disptime += 1;

        // state must be passed as arguments since they are mutable
        // borrows and needs to be released for each iteration.
        // These variables are used in between multiple invocation of this closure.
        let add_tent = |is_bullet, pos: &[f64; 2], state: &mut game_logic::ShooterState| {
            let mut ent = Entity::new(
                &mut state.id_gen,
                [
                    pos[0] + 4. * (state.rng.next() - 0.5),
                    pos[1] + 4. * (state.rng.next() - 0.5),
                ],
                [0., 0.],
            )
            .rotation(state.rng.next() as f32 * 2. * std::f32::consts::PI);
            let (playback_rate, max_frames) = if is_bullet { (2, 8) } else { (4, 6) };
            ent = ent.health((max_frames * playback_rate) as i32);

            state.tent.push(TempEntity {
                base: ent,
                texture: if is_bullet {
                    state.assets.explode_tex.clone()
                } else {
                    state.assets.explode2_tex.clone()
                },
                max_frames,
                width: if is_bullet { 16 } else { 32 },
                playback_rate,
                image_width: if is_bullet { 128 } else { 256 },
                size: if is_bullet {
                    EXPLODE_SIZE
                } else {
                    EXPLODE2_SIZE
                },
            });
        };

        if !self.0.paused {
            let dice = 256;
            let rng = &mut self.0.rng;
            let mut i = rng.gen_range(0, dice);
            let [enemy_count, boss_count, shielded_boss_count, spiral_count] =
                self.0.enemies.iter().fold([0; 4], |mut c, e| match e {
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
            let gen_amount = self.0.player.difficulty_level() * 2 + 4;
            while i < gen_amount {
                let weights = [
                    if enemy_count < 128 {
                        if self.0.player.score < 1024 {
                            64
                        } else {
                            16
                        }
                    } else {
                        0
                    },
                    if boss_count < 32 { 4 } else { 0 },
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
                                [rng.gen_rangef(0., WIDTH as f64), 0.],
                                [rng.next() - 0.5, rng.next() * 0.5],
                            )
                        }
                        1 => {
                            // left
                            (
                                [0., rng.gen_rangef(0., WIDTH as f64)],
                                [rng.next() * 0.5, rng.next() - 0.5],
                            )
                        }
                        2 => {
                            // right
                            (
                                [WIDTH as f64, rng.gen_rangef(0., WIDTH as f64)],
                                [-rng.next() * 0.5, rng.next() - 0.5],
                            )
                        }
                        _ => panic!("RNG returned out of range"),
                    };
                    if let Some(x) = accum.iter().position(|x| dice < *x) {
                        self.0.enemies.push(match x {
                            0 => Enemy::Enemy1(
                                EnemyBase::new(&mut self.0.id_gen, pos, velo).health(3),
                            ),
                            1 => Enemy::Boss(
                                EnemyBase::new(&mut self.0.id_gen, pos, velo).health(64),
                            ),
                            2 => Enemy::ShieldedBoss(ShieldedBoss::new(
                                &mut self.0.id_gen,
                                pos,
                                velo,
                            )),
                            _ => Enemy::new_spiral(&mut self.0.id_gen, pos, velo),
                        });
                    }
                }
                i += rng.gen_range(0, dice);
            }
        }

        context.clear(GL::COLOR_BUFFER_BIT);

        context.uniform_matrix4fv_with_f32_array(
            self.0
                .assets
                .sprite_shader
                .as_ref()
                .unwrap()
                .transform_loc
                .as_ref(),
            false,
            <Matrix4<f32> as AsRef<[f32; 16]>>::as_ref(&Matrix4::from_scale(1.)),
        );
        context.bind_texture(GL::TEXTURE_2D, Some(&self.0.assets.back_tex));
        context.draw_arrays(GL::TRIANGLE_FAN, 0, 4);

        if !self.0.game_over && !self.0.paused {
            if self.0.up_pressed {
                self.0.player.move_up()
            }
            if self.0.down_pressed {
                self.0.player.move_down()
            }
            if self.0.left_pressed {
                self.0.player.move_left()
            }
            if self.0.right_pressed {
                self.0.player.move_right()
            }

            if self.0.shoot_pressed && self.0.player.cooldown == 0 {
                let weapon = &self.0.player.weapon;
                let shoot_period = if let Weapon::Bullet = weapon { 5 } else { 50 };

                if Weapon::Bullet == *weapon || Weapon::Missile == *weapon {
                    let level = self.0.player.power_level() as i32;
                    self.0.player.cooldown += shoot_period;
                    for i in -1 - level..2 + level {
                        let speed = if let Weapon::Bullet = weapon {
                            BULLET_SPEED
                        } else {
                            MISSILE_SPEED
                        };
                        let mut ent = Entity::new(
                            &mut self.0.id_gen,
                            self.0.player.base.pos,
                            [i as f64, -speed],
                        )
                        .rotation((i as f32).atan2(speed as f32));
                        if let Weapon::Bullet = weapon {
                            self.0.shots_bullet += 1;
                            self.0
                                .bullets
                                .insert(ent.id, Projectile::Bullet(BulletBase(ent)));
                        } else {
                            self.0.shots_missile += 1;
                            ent = ent.health(5);
                            self.0.bullets.insert(
                                ent.id,
                                Projectile::Missile {
                                    base: BulletBase(ent),
                                    target: 0,
                                    trail: vec![],
                                },
                            );
                        }
                    }
                } else if Weapon::Light == *weapon {
                    let gl = &context;
                    let assets = &self.0.assets;
                    let player = &self.0.player;
                    let level = player.power_level() as i32;

                    gl.use_program(Some(&self.0.assets.trail_shader.as_ref().unwrap().program));
                    let shader = assets.trail_shader.as_ref().unwrap();

                    gl.uniform1i(shader.texture_loc.as_ref(), 0);
                    gl.bind_texture(GL::TEXTURE_2D, Some(&assets.beam_tex));

                    enable_buffer(gl, &assets.trail_buffer, 4, shader.vertex_position);

                    let left = player.base.pos[0] as f32 - level as f32 - 3.;
                    let right = player.base.pos[0] as f32 + level as f32 + 3.;
                    let vertices = [
                        [left, player.base.pos[1] as f32, 0., 0.],
                        [right, player.base.pos[1] as f32, 0., 1.],
                        [left, 0., 1., 0.],
                        [right, 0., 1., 1.],
                    ];

                    vertex_buffer_data(gl, &vertices.flat()).unwrap();

                    gl.uniform_matrix4fv_with_f32_array(
                        shader.transform_loc.as_ref(),
                        false,
                        <Matrix4<f32> as AsRef<[f32; 16]>>::as_ref(
                            &assets.world_transform.cast().unwrap(),
                        ),
                    );

                    gl.uniform_matrix3fv_with_f32_array(
                        shader.tex_transform_loc.as_ref(),
                        false,
                        <Matrix3<f32> as AsRef<[f32; 9]>>::as_ref(&Matrix3::from_scale(1.)),
                    );

                    gl.draw_arrays(GL::TRIANGLE_STRIP, 0, vertices.len() as i32);

                    enable_buffer(
                        gl,
                        &assets.rect_buffer,
                        2,
                        assets.sprite_shader.as_ref().unwrap().vertex_position,
                    );

                    let beam_rect = [
                        player.base.pos[0] - LIGHT_WIDTH,
                        0.,
                        player.base.pos[0] + LIGHT_WIDTH,
                        player.base.pos[1],
                    ];
                    let mut enemies = std::mem::take(&mut self.0.enemies);
                    for enemy in &mut enemies {
                        if enemy.test_hit(beam_rect) {
                            add_tent(true, &enemy.get_base().pos, &mut self.0);
                            enemy.damage(1 + level);
                        }
                    }
                    self.0.enemies = enemies;
                } else if Weapon::Lightning == *weapon {
                    let nmax = std::cmp::min(
                        (self.0.player.power_level() + 1 + self.0.time as u32 % 2) / 2,
                        31,
                    );

                    // Random walk with momentum
                    fn next_lightning(rng: &mut Xor128, a: &mut [f64; 4]) {
                        a[2] += LIGHTNING_ACCEL * (rng.next() - 0.5) - a[2] * LIGHTNING_FEEDBACK;
                        a[3] += LIGHTNING_ACCEL * (rng.next() - 0.5) - a[3] * LIGHTNING_FEEDBACK;
                        a[0] += a[2];
                        a[1] += a[3];
                    }

                    let gl = &context;

                    for _ in 0..nmax {
                        // Use the same seed twice to reproduce random sequence
                        let seed = self.0.rng.nexti();

                        // Lambda to call the same lightning sequence twice, first pass for detecting hit enemy
                        // and second pass for rendering.
                        let lightning = |state: &mut Self,
                                         seed: u32,
                                         length: u32,
                                         f: &mut dyn FnMut(
                            &mut game_logic::ShooterState,
                            &[f64; 4],
                        ) -> bool| {
                            let mut rng2 = Xor128::new(seed);
                            let mut a = [
                                state.0.player.base.pos[0],
                                state.0.player.base.pos[1],
                                0.,
                                -16.,
                            ];
                            for i in 0..length {
                                let ox = a[0];
                                let oy = a[1];
                                next_lightning(&mut rng2, &mut a);
                                let segment = [ox, oy, a[0], a[1]];
                                if !f(&mut state.0, &segment) {
                                    return i;
                                }
                            }
                            length
                        };

                        let length = lightning(
                            self,
                            seed,
                            LIGHTNING_VERTICES,
                            &mut |state: &mut game_logic::ShooterState, segment: &[f64; 4]| {
                                let b = [segment[2], segment[3]];
                                for enemy in state.enemies.iter_mut() {
                                    let ebb = enemy.get_bb();
                                    if ebb[0] < b[0] + 4.
                                        && b[0] - 4. <= ebb[2]
                                        && ebb[1] < b[1] + 4.
                                        && b[1] - 4. <= ebb[3]
                                    {
                                        enemy.damage(2 + state.rng.gen_range(0, 3) as i32);
                                        add_tent(true, &b, state);
                                        return false;
                                    }
                                }
                                return true;
                            },
                        );
                        let hit = length != LIGHTNING_VERTICES;

                        gl.use_program(Some(&self.0.assets.trail_shader.as_ref().unwrap().program));
                        let shader = self.0.assets.trail_shader.as_ref().unwrap();

                        gl.uniform1i(shader.texture_loc.as_ref(), 0);
                        gl.bind_texture(GL::TEXTURE_2D, Some(&self.0.assets.beam_tex));

                        enable_buffer(gl, &self.0.assets.trail_buffer, 4, shader.vertex_position);

                        let mut vertices = vec![];
                        let mut prev_node_opt = None;

                        lightning(self, seed, length, &mut |state, segment: &[f64; 4]| {
                            // line(if hit { col } else { col2 }, if hit { 2. } else { 1. }, *segment, context.transform, graphics);
                            let prev_node = if let Some(node) = prev_node_opt {
                                node
                            } else {
                                prev_node_opt = Some([segment[0], segment[1]]);
                                return true;
                            };
                            let width = if hit { 2. } else { 1. };
                            let this_node = [segment[0], segment[1]];
                            let delta = vec2_normalized(vec2_sub(this_node, prev_node));
                            let perp = vec2_scale([delta[1], -delta[0]], width);
                            let top = vec2_add(prev_node, perp);
                            let bottom = vec2_sub(prev_node, perp);
                            vertices.extend_from_slice(&[top[0] as f32, top[1] as f32, 0., -0.1]);
                            vertices.extend_from_slice(&[
                                bottom[0] as f32,
                                bottom[1] as f32,
                                0.,
                                1.1,
                            ]);
                            prev_node_opt = Some([segment[0], segment[1]]);
                            true
                        });

                        vertex_buffer_data(gl, &vertices).unwrap();

                        let shader = self.0.assets.trail_shader.as_ref().unwrap();
                        gl.uniform_matrix4fv_with_f32_array(
                            shader.transform_loc.as_ref(),
                            false,
                            <Matrix4<f32> as AsRef<[f32; 16]>>::as_ref(
                                &self.0.assets.world_transform.cast().unwrap(),
                            ),
                        );

                        gl.uniform_matrix3fv_with_f32_array(
                            shader.tex_transform_loc.as_ref(),
                            false,
                            <Matrix3<f32> as AsRef<[f32; 9]>>::as_ref(&Matrix3::from_scale(1.)),
                        );

                        gl.draw_arrays(GL::TRIANGLE_STRIP, 0, (vertices.len() / 4) as i32);

                        enable_buffer(
                            gl,
                            &self.0.assets.rect_buffer,
                            2,
                            self.0
                                .assets
                                .sprite_shader
                                .as_ref()
                                .unwrap()
                                .vertex_position,
                        );
                    }
                }
            }
            if self.0.player.cooldown < 1 {
                self.0.player.cooldown = 0;
            } else {
                self.0.player.cooldown -= 1;
            }

            if 0 < self.0.player.invtime {
                self.0.player.invtime -= 1;
            }
        }

        context.use_program(Some(&self.0.assets.sprite_shader.as_ref().unwrap().program));

        enable_buffer(
            &context,
            &self.0.assets.rect_buffer,
            2,
            self.0
                .assets
                .sprite_shader
                .as_ref()
                .unwrap()
                .vertex_position,
        );

        let load_identity = |state: &Self| {
            context.uniform_matrix3fv_with_f32_array(
                state
                    .0
                    .assets
                    .sprite_shader
                    .as_ref()
                    .unwrap()
                    .tex_transform_loc
                    .as_ref(),
                false,
                <Matrix3<f32> as AsRef<[f32; 9]>>::as_ref(&Matrix3::from_scale(1.)),
            );
        };

        load_identity(self);

        let mut to_delete: Vec<usize> = Vec::new();

        for (i, e) in &mut ((&mut self.0.items).iter_mut().enumerate()) {
            if !self.0.paused {
                if let Some(_) = e.animate(&mut self.0.player) {
                    to_delete.push(i);
                    continue;
                }
            }
            e.draw(&context, &self.0.assets);
        }

        for i in to_delete.iter().rev() {
            let dead = self.0.items.remove(*i);
            console_log!(
                "Deleted Item id={}: {} / {}",
                dead.get_base().id,
                *i,
                self.0.items.len()
            );
        }
        to_delete.clear();

        for enemy in &self.0.enemies {
            enemy.draw(&self.0, &context, &self.0.assets);
        }

        if !self.0.paused {
            self.0.enemies = std::mem::take(&mut self.0.enemies)
                .into_iter()
                .filter_map(|mut enemy| {
                    if let Some(death_reason) = enemy.animate(&mut self.0) {
                        if let DeathReason::Killed = death_reason {
                            self.0.player.kills += 1;
                            self.0.player.score += if enemy.is_boss() { 10 } else { 1 };
                            if self.0.rng.gen_range(0, 100) < 20 {
                                let ent =
                                    Entity::new(&mut self.0.id_gen, enemy.get_base().pos, [0., 1.]);
                                self.0.items.push(enemy.drop_item(ent));
                                console_log!("item dropped: {:?}", self.0.items.len());
                            }
                        }
                        None
                    } else {
                        Some(enemy)
                    }
                })
                .collect();
        }

        for (_, bullet) in &self.0.bullets {
            bullet.draw(&self.0, &context, &self.0.assets);
        }

        if !self.0.paused {
            self.0.bullets = std::mem::take(&mut self.0.bullets)
                .into_iter()
                .filter_map(|(id, mut bullet)| {
                    if let Some(reason) =
                        bullet.animate_bullet(&mut self.0.enemies, &mut self.0.player)
                    {
                        match reason {
                            DeathReason::Killed | DeathReason::HitPlayer => add_tent(
                                if let Projectile::Missile { .. } = bullet {
                                    false
                                } else {
                                    true
                                },
                                &bullet.get_base().0.pos,
                                &mut self.0,
                            ),
                            _ => {}
                        }

                        if let DeathReason::HitPlayer = reason {
                            if self.0.player.invtime == 0
                                && !self.0.game_over
                                && 0 < self.0.player.lives
                            {
                                self.0.player.lives -= 1;
                                self.0.player_live_icons[self.0.player.lives as usize]
                                    .set_class_name("hidden");
                                if self.0.player.lives == 0 {
                                    self.0.game_over = true;
                                    let game_over_element =
                                        document().get_element_by_id("gameOver")?;
                                    game_over_element.set_class_name("");
                                } else {
                                    self.0.player.invtime = PLAYER_INVINCIBLE_TIME;
                                }
                            }
                        }

                        None
                    } else {
                        Some((id, bullet))
                    }
                })
                .collect::<HashMap<_, _>>();
        }

        let mut to_delete = vec![];
        for (i, e) in &mut ((&mut self.0.tent).iter_mut().enumerate()) {
            if !self.0.paused {
                if let Some(_) = e.animate_temp() {
                    to_delete.push(i);
                    continue;
                }
            }
            e.draw_temp(&context, &self.0.assets);
        }

        for i in to_delete.iter().rev() {
            self.0.tent.remove(*i);
            //println!("Deleted tent {} / {}", *i, bullets.len());
        }

        load_identity(self);

        if !self.0.game_over {
            if self.0.player.invtime == 0 || self.0.disptime % 2 == 0 {
                self.0.player.base.draw_tex(
                    &self.0.assets,
                    &context,
                    &self.0.assets.player_texture,
                    Some(PLAYER_SIZE),
                );
            }
        }

        fn set_text(id: &str, text: &str) {
            let frame_element = document().get_element_by_id(id).unwrap();
            frame_element.set_inner_html(text);
        }

        set_text("frame", &format!("Frame: {}", self.0.time));
        set_text("score", &format!("Score: {}", self.0.player.score));
        set_text("kills", &format!("Kills: {}", self.0.player.kills));
        set_text(
            "difficulty",
            &format!("Difficulty Level: {}", self.0.player.difficulty_level()),
        );
        set_text(
            "power",
            &format!(
                "Power: {} Level: {}",
                self.0.player.power,
                self.0.player.power_level()
            ),
        );
        set_text(
            "shots",
            &format!("Shots {}/{}", self.0.shots_bullet, self.0.shots_missile),
        );
        set_text("weapon", &format!("Weapon: {:#?}", self.0.player.weapon));

        Ok(())
    }
}

pub fn compile_shader(context: &GL, shader_type: u32, source: &str) -> Result<WebGlShader, String> {
    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context
        .get_shader_parameter(&shader, GL::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(context
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader")))
    }
}

pub fn link_program(
    context: &GL,
    vert_shader: &WebGlShader,
    frag_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
    let program = context
        .create_program()
        .ok_or_else(|| String::from("Unable to create shader object"))?;

    context.attach_shader(&program, vert_shader);
    context.attach_shader(&program, frag_shader);
    context.link_program(&program);

    if context
        .get_program_parameter(&program, GL::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(context
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error creating program object")))
    }
}

//
// Initialize a texture and load an image.
// When the image finished loading copy it into the texture.
//
fn load_texture(gl: &GL, url: &str) -> Result<Rc<WebGlTexture>, JsValue> {
    let texture = Rc::new(gl.create_texture().unwrap());
    gl.bind_texture(GL::TEXTURE_2D, Some(&*texture));

    // Because images have to be downloaded over the internet
    // they might take a moment until they are ready.
    // Until then put a single pixel in the texture so we can
    // use it immediately. When the image has finished downloading
    // we'll update the texture with the contents of the image.
    let level = 0;
    let internal_format = GL::RGBA as i32;
    let width = 1;
    let height = 1;
    let border = 0;
    let src_format = GL::RGBA;
    let src_type = GL::UNSIGNED_BYTE;
    let pixel = [0u8, 255, 255, 255];
    gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
        GL::TEXTURE_2D,
        level,
        internal_format,
        width,
        height,
        border,
        src_format,
        src_type,
        Some(&pixel),
    )
    .unwrap();
    gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_S, GL::REPEAT as i32);
    gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_T, GL::REPEAT as i32);
    gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MIN_FILTER, GL::LINEAR as i32);

    let image = Rc::new(HtmlImageElement::new().unwrap());
    let url_str = url.to_owned();
    let image_clone = image.clone();
    let texture_clone = texture.clone();
    let callback = Closure::wrap(Box::new(move || {
        console_log!("loaded image: {}", url_str);
        // web_sys::console::log_1(Date::new_0().to_locale_string("en-GB", &JsValue::undefined()));

        let gl = get_context();

        gl.bind_texture(GL::TEXTURE_2D, Some(&*texture_clone));
        gl.tex_image_2d_with_u32_and_u32_and_image(
            GL::TEXTURE_2D,
            level,
            internal_format,
            src_format,
            src_type,
            &image_clone,
        )
        .unwrap();

        // WebGL1 has different requirements for power of 2 images
        // vs non power of 2 images so check if the image is a
        // power of 2 in both dimensions.
        if is_power_of_2(image_clone.width()) && is_power_of_2(image_clone.height()) {
            // Yes, it's a power of 2. Generate mips.
            gl.generate_mipmap(GL::TEXTURE_2D);
        } else {
            // No, it's not a power of 2. Turn off mips and set
            // wrapping to clamp to edge
            gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_S, GL::CLAMP_TO_EDGE as i32);
            gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_T, GL::CLAMP_TO_EDGE as i32);
            gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MIN_FILTER, GL::LINEAR as i32);
        }
    }) as Box<dyn FnMut()>);
    image.set_onload(Some(callback.as_ref().unchecked_ref()));
    image.set_src(url);

    callback.forget();

    Ok(texture)
}

fn is_power_of_2(value: u32) -> bool {
    (value & (value - 1)) == 0
}

fn vertex_buffer_data(context: &GL, vertices: &[f32]) -> Result<(), JsValue> {
    // Note that `Float32Array::view` is somewhat dangerous (hence the
    // `unsafe`!). This is creating a raw view into our module's
    // `WebAssembly.Memory` buffer, but if we allocate more pages for ourself
    // (aka do a memory allocation in Rust) it'll cause the buffer to change,
    // causing the `Float32Array` to be invalid.
    //
    // As a result, after `Float32Array::view` we have to be very careful not to
    // do any memory allocations before it's dropped.
    unsafe {
        let vert_array = js_sys::Float32Array::view(vertices);

        context.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &vert_array, GL::STATIC_DRAW);
    };
    Ok(())
}

fn enable_buffer(gl: &GL, buffer: &Option<WebGlBuffer>, elements: i32, vertex_position: u32) {
    gl.bind_buffer(GL::ARRAY_BUFFER, buffer.as_ref());
    gl.vertex_attrib_pointer_with_i32(vertex_position, elements, GL::FLOAT, false, 0, 0);
    gl.enable_vertex_attrib_array(vertex_position);
}

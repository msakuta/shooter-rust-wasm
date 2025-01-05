use cgmath::{Matrix3, Matrix4};
use js_sys::JsString;
use slice_of_array::SliceFlatExt;
use vecmath::{vec2_add, vec2_normalized, vec2_scale, vec2_sub};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGlProgram, WebGlRenderingContext as GL, WebGlShader};

use game_logic::{
    assets_webgl::{Assets, ShaderBundle},
    console_log,
    consts::*,
    enable_buffer,
    entity::{Enemy, Entity, TempEntity, TempEntityType, Weapon},
    js_str, vertex_buffer_data,
};

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

#[derive(Default)]
struct InputState {
    pub shoot_pressed: bool,
    pub left_pressed: bool,
    pub right_pressed: bool,
    pub up_pressed: bool,
    pub down_pressed: bool,
}

#[wasm_bindgen]
pub struct ShooterState {
    state: game_logic::ShooterState,
    input_state: InputState,
    assets: Assets,
}

#[wasm_bindgen]
impl ShooterState {
    #[wasm_bindgen(constructor)]
    pub fn new(image_assets: js_sys::Array) -> Result<ShooterState, JsValue> {
        let context = get_context();

        Ok(Self {
            state: game_logic::ShooterState::default(),
            input_state: InputState::default(),
            assets: Assets::new(&document(), &context, image_assets)?,
        })
    }

    pub fn key_down(&mut self, event: web_sys::KeyboardEvent) -> Result<JsString, JsValue> {
        println!("key: {}", event.key_code());
        match event.key_code() {
            32 => self.input_state.shoot_pressed = true,
            65 | 37 => self.input_state.left_pressed = true,
            68 | 39 => self.input_state.right_pressed = true,
            80 => {
                // P
                self.state.paused = !self.state.paused;
                let paused_element = document().get_element_by_id("paused").unwrap();
                paused_element.set_class_name(if self.state.paused {
                    "noselect"
                } else {
                    "noselect hidden"
                })
            }
            87 | 38 => self.input_state.up_pressed = true,
            83 | 40 => self.input_state.down_pressed = true,
            88 | 90 => {
                // Z or X
                let is_x = event.key_code() == 88;
                self.state.player.weapon = if is_x {
                    self.state.player.weapon.next()
                } else {
                    self.state.player.weapon.prev()
                };
                console_log!("Weapon switched: {}", self.state.player.weapon);
            }
            78 => {
                // N
                self.state.restart()?;
                for (name, class_name) in &[("gameOver", "hidden"), ("paused", "hidden noselect")] {
                    let elem = document()
                        .get_element_by_id(name)
                        .ok_or_else(|| js_str!("HTML element not found"))?;
                    elem.set_class_name(class_name);
                }
            }
            _ => (),
        }
        Ok(JsString::from(self.state.player.weapon.to_string()))
    }

    pub fn key_up(&mut self, event: web_sys::KeyboardEvent) {
        console_log!("key: {}", event.key_code());
        match event.key_code() {
            32 => self.input_state.shoot_pressed = false,
            65 | 37 => self.input_state.left_pressed = false,
            68 | 39 => self.input_state.right_pressed = false,
            87 | 38 => self.input_state.up_pressed = false,
            83 | 40 => self.input_state.down_pressed = false,
            _ => (),
        }
    }

    pub fn restart(&mut self) -> Result<(), JsValue> {
        self.state.restart()?;

        for icon in &self.assets.player_live_icons {
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
            uniform float alpha;

            void main() {
                vec4 texColor = texture2D( texture, vec2(texCoords.x, texCoords.y) );
                gl_FragColor = vec4(texColor.rgb, texColor.a * alpha);
            }
        "#,
        )?;
        let program = link_program(&context, &vert_shader, &frag_shader)?;
        context.use_program(Some(&program));

        let shader = ShaderBundle::new(&context, program);

        context.active_texture(GL::TEXTURE0);

        context.uniform1i(shader.texture_loc.as_ref(), 0);
        context.uniform1f(shader.alpha_loc.as_ref(), 1.);

        context.enable(GL::BLEND);
        context.blend_equation(GL::FUNC_ADD);
        context.blend_func(GL::SRC_ALPHA, GL::ONE_MINUS_SRC_ALPHA);

        self.assets.sprite_shader = Some(shader);

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
        self.assets.trail_shader = Some(ShaderBundle::new(&context, program));

        context.active_texture(GL::TEXTURE0);
        context.uniform1i(
            self.assets
                .trail_shader
                .as_ref()
                .and_then(|s| s.texture_loc.as_ref()),
            0,
        );

        self.assets.trail_buffer = Some(context.create_buffer().ok_or("failed to create buffer")?);

        self.assets.rect_buffer = Some(context.create_buffer().ok_or("failed to create buffer")?);
        context.bind_buffer(GL::ARRAY_BUFFER, self.assets.rect_buffer.as_ref());
        let rect_vertices: [f32; 8] = [1., 1., -1., 1., -1., -1., 1., -1.];
        vertex_buffer_data(&context, &rect_vertices);

        context.clear_color(0.0, 0.0, 0.5, 1.0);

        Ok(())
    }

    pub fn render(&mut self) -> Result<(), JsValue> {
        let context = get_context();

        if !self.state.paused {
            self.state.time += 1;
        }
        self.state.disptime += 1;

        let assets = &self.assets;

        type TT = TempEntityType;

        // state must be passed as arguments since they are mutable
        // borrows and needs to be released for each iteration.
        // These variables are used in between multiple invocation of this closure.
        let add_tent =
            |ty: TT, pos: &[f64; 2], velo: &[f64; 2], state: &mut game_logic::ShooterState| {
                let mut ent = Entity::new(
                    &mut state.id_gen,
                    [
                        pos[0] + 4. * (state.rng.gen() - 0.5),
                        pos[1] + 4. * (state.rng.gen() - 0.5),
                    ],
                    *velo,
                )
                .rotation(state.rng.gen() as f32 * 2. * std::f32::consts::PI);
                let (playback_rate, max_frames, repeats, width, shrink) = match ty {
                    TT::Explode => (2, 8, 1, 16, 0.),
                    TT::Explode2 => (4, 6, 1, 32, 0.),
                    TT::Blood => (2, 4, state.rng.gen_range(2, 5), 16, 1.),
                };
                ent = ent.health((max_frames * playback_rate * repeats) as i32);

                state.tent.push(TempEntity {
                    base: ent,
                    texture: match ty {
                        TT::Explode => assets.explode_tex.clone(),
                        TT::Explode2 => assets.explode2_tex.clone(),
                        TT::Blood => assets.blood_tex.clone(),
                    },
                    max_frames,
                    width,
                    playback_rate,
                    image_width: match ty {
                        TT::Explode => 128,
                        TT::Explode2 => 256,
                        TT::Blood => 64,
                    },
                    size: match ty {
                        TT::Explode | TT::Blood => EXPLODE_SIZE,
                        _ => EXPLODE2_SIZE,
                    },
                    shrink_rate: shrink,
                });
            };

        let wave_period = self.state.gen_enemies();

        context.clear(GL::COLOR_BUFFER_BIT);

        context.uniform_matrix4fv_with_f32_array(
            self.assets
                .sprite_shader
                .as_ref()
                .unwrap()
                .transform_loc
                .as_ref(),
            false,
            <Matrix4<f32> as AsRef<[f32; 16]>>::as_ref(&Matrix4::from_scale(1.)),
        );
        context.bind_texture(GL::TEXTURE_2D, Some(&self.assets.back_tex));
        context.draw_arrays(GL::TRIANGLE_FAN, 0, 4);

        if !self.state.game_over && !self.state.paused {
            if self.input_state.up_pressed {
                self.state.player.move_up()
            }
            if self.input_state.down_pressed {
                self.state.player.move_down()
            }
            if self.input_state.left_pressed {
                self.state.player.move_left()
            }
            if self.input_state.right_pressed {
                self.state.player.move_right()
            }

            if self.input_state.shoot_pressed && self.state.player.cooldown == 0 {
                let weapon = self.state.player.weapon;

                // Use the same seed twice to reproduce random sequence
                let seed = self.state.rng.nexti();

                self.state.try_shoot(
                    self.input_state.shoot_pressed,
                    seed,
                    &mut |ty, pos, state| add_tent(ty, pos, &[0.; 2], state),
                );

                if Weapon::Light == weapon {
                    let gl = &context;
                    let assets = &self.assets;
                    let player = &self.state.player;
                    let level = player.power_level() as i32;

                    gl.use_program(Some(&self.assets.trail_shader.as_ref().unwrap().program));
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

                    vertex_buffer_data(gl, &vertices.flat());

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
                } else if Weapon::Lightning == weapon {
                    let gl = &context;

                    self.state.lightning(
                        seed,
                        None,
                        &mut |state: &mut game_logic::ShooterState, seed| {
                            let length = state.lightning_branch(
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
                                            add_tent(TempEntityType::Explode2, &b, &[0.; 2], state);
                                            return false;
                                        }
                                    }
                                    true
                                },
                            );
                            let hit = length != LIGHTNING_VERTICES;

                            gl.use_program(Some(&assets.trail_shader.as_ref().unwrap().program));
                            let shader = assets.trail_shader.as_ref().unwrap();

                            gl.uniform1i(shader.texture_loc.as_ref(), 0);
                            gl.bind_texture(GL::TEXTURE_2D, Some(&assets.beam_tex));

                            enable_buffer(gl, &assets.trail_buffer, 4, shader.vertex_position);

                            let mut vertices = vec![];
                            let mut prev_node_opt = None;

                            state.lightning_branch(
                                seed,
                                length,
                                &mut |_state, segment: &[f64; 4]| {
                                    // line(if hit { col } else { col2 }, if hit { 2. } else { 1. }, *segment, context.transform, graphics);
                                    let prev_node = if let Some(node) = prev_node_opt {
                                        node
                                    } else {
                                        prev_node_opt = Some([segment[0], segment[1]]);
                                        return true;
                                    };
                                    let width = if hit { 5. } else { 1. };
                                    let this_node = [segment[0], segment[1]];
                                    let delta = vec2_normalized(vec2_sub(this_node, prev_node));
                                    let perp = vec2_scale([delta[1], -delta[0]], width);
                                    let top = vec2_add(prev_node, perp);
                                    let bottom = vec2_sub(prev_node, perp);
                                    vertices.extend_from_slice(&[
                                        top[0] as f32,
                                        top[1] as f32,
                                        0.,
                                        -0.1,
                                    ]);
                                    vertices.extend_from_slice(&[
                                        bottom[0] as f32,
                                        bottom[1] as f32,
                                        0.,
                                        1.1,
                                    ]);
                                    prev_node_opt = Some([segment[0], segment[1]]);
                                    true
                                },
                            );

                            vertex_buffer_data(gl, &vertices);

                            let shader = assets.trail_shader.as_ref().unwrap();
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

                            gl.draw_arrays(GL::TRIANGLE_STRIP, 0, (vertices.len() / 4) as i32);

                            enable_buffer(
                                gl,
                                &assets.rect_buffer,
                                2,
                                assets.sprite_shader.as_ref().unwrap().vertex_position,
                            );
                        },
                    );
                }
            }
            if self.state.player.cooldown < 1 {
                self.state.player.cooldown = 0;
            } else {
                self.state.player.cooldown -= 1;
            }

            if 0 < self.state.player.invtime {
                self.state.player.invtime -= 1;
            }
        }

        context.use_program(Some(&self.assets.sprite_shader.as_ref().unwrap().program));

        enable_buffer(
            &context,
            &self.assets.rect_buffer,
            2,
            self.assets.sprite_shader.as_ref().unwrap().vertex_position,
        );

        let load_identity = |state: &Self| {
            context.uniform_matrix3fv_with_f32_array(
                state
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

        self.state.draw_items(&context, &self.assets);

        self.state.animate_items();

        self.state.draw_enemies(&context, &self.assets);

        const BLOOD_SPEED: f64 = 2.;

        self.state.animate_enemies(&mut |enemy, state| {
            let blood_count = match enemy {
                Enemy::Boss(_) | Enemy::SpiralEnemy(_) => 15,
                _ => 5,
            };
            for _i in 0..blood_count {
                let velo = [
                    state.rng.gen_rangef(-BLOOD_SPEED, BLOOD_SPEED),
                    state.rng.gen_rangef(-BLOOD_SPEED, BLOOD_SPEED),
                ];
                add_tent(TT::Blood, &enemy.pos, &velo, state);
            }
        });

        self.state.draw_bullets(&context, &self.assets);

        if self
            .state
            .animate_bullets(&mut |ty, pos, state| add_tent(ty, pos, &[0.; 2], state))
        {
            let game_over_elem = document()
                .get_element_by_id("gameOver")
                .ok_or_else(|| js_str!("game over elem not found"))?;
            game_over_elem.set_class_name("");
        };

        self.state.draw_tents(&context, &self.assets);

        self.state.animate_tents();

        load_identity(self);

        if !self.state.game_over && (self.state.player.invtime == 0 || self.state.disptime % 2 == 0)
        {
            self.state.player.base.draw_tex(
                &self.assets,
                &context,
                &self.assets.player_texture,
                Some([PLAYER_SIZE; 2]),
            );
        }

        fn set_text(id: &str, text: &str) {
            let frame_element = document().get_element_by_id(id).unwrap();
            frame_element.set_inner_html(text);
        }

        set_text("frame", &format!("Frame: {}", self.state.time));
        set_text("score", &format!("Score: {}", self.state.player.score));
        set_text("kills", &format!("Kills: {}", self.state.player.kills));
        set_text(
            "power",
            &format!(
                "Power: {} Level: {}",
                self.state.player.power,
                self.state.player.power_level()
            ),
        );
        set_text(
            "waves",
            &format!(
                "Wave: {} Level: {}",
                self.state.time / wave_period,
                self.state.player.difficulty_level()
            ),
        );
        set_text(
            "shots",
            &format!(
                "Shots {}/{}",
                self.state.shots_bullet, self.state.shots_missile
            ),
        );
        set_text(
            "weapon",
            &format!("Weapon: {:#?}", self.state.player.weapon),
        );

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

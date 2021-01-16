use cgmath::{Matrix4, Vector3};
use std::collections::HashMap;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{
    HtmlImageElement, WebGlBuffer, WebGlProgram, WebGlRenderingContext, WebGlShader, WebGlTexture,
};

macro_rules! console_log {
    ($fmt:expr, $($arg1:expr),*) => {
        crate::log(&format!($fmt, $($arg1),+))
    };
    ($fmt:expr) => {
        crate::log($fmt)
    }
}

mod consts;
mod entity;
mod xor128;

use crate::consts::*;
use crate::entity::{
    Assets, BulletBase, DeathReason, Enemy, EnemyBase, Entity, Player, Projectile,
};
use crate::xor128::Xor128;

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

fn get_context() -> WebGlRenderingContext {
    let document = document();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement =
        canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();

    canvas
        .get_context("webgl")
        .unwrap()
        .unwrap()
        .dyn_into::<WebGlRenderingContext>()
        .unwrap()
}

#[wasm_bindgen]
pub struct ShooterState {
    time: usize,
    id_gen: u32,
    player: Player,
    enemies: Vec<Enemy>,
    bullets: HashMap<u32, Projectile>,
    rng: Xor128,
    shots_bullet: usize,

    shoot_pressed: bool,
    left_pressed: bool,
    right_pressed: bool,
    up_pressed: bool,
    down_pressed: bool,

    assets: Assets,
}

#[wasm_bindgen]
impl ShooterState {
    #[wasm_bindgen(constructor)]
    pub fn new(image_assets: js_sys::Array) -> Result<ShooterState, JsValue> {
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
        let player = Player::new(Entity::new(
            &mut id_gen,
            [FWIDTH / 2., FHEIGHT / 2.],
            [0., 0.],
        ));

        Ok(Self {
            time: 0,
            id_gen,
            player,
            enemies: vec![],
            bullets: HashMap::new(),
            rng: Xor128::new(3232132),
            shots_bullet: 0,
            shoot_pressed: false,
            left_pressed: false,
            right_pressed: false,
            up_pressed: false,
            down_pressed: false,
            assets: Assets {
                world_transform: Matrix4::from_translation(Vector3::new(-1., 1., 0.))
                    * &Matrix4::from_nonuniform_scale(2. / FWIDTH, -2. / FHEIGHT, 1.),
                enemy_tex: load_texture_local("enemy")?,
                boss_tex: load_texture_local("boss")?,
                player_texture: load_texture_local("player")?,
                bullet_texture: load_texture_local("bullet")?,
                enemy_bullet_texture: load_texture_local("ebullet")?,
                rect_buffer: None,
                vertex_position: 0,
                texture_loc: None,
                transform_loc: None,
            },
        })
    }

    pub fn key_down(&mut self, event: web_sys::KeyboardEvent) {
        println!("key: {}", event.key_code());
        match event.key_code() {
            32 => self.shoot_pressed = true,
            65 => self.left_pressed = true,
            68 => self.right_pressed = true,
            87 => self.up_pressed = true,
            83 => self.down_pressed = true,
            _ => (),
        }
    }

    pub fn key_up(&mut self, event: web_sys::KeyboardEvent) {
        console_log!("key: {}", event.key_code());
        match event.key_code() {
            32 => self.shoot_pressed = false,
            65 => self.left_pressed = false,
            68 => self.right_pressed = false,
            87 => self.up_pressed = false,
            83 => self.down_pressed = false,
            _ => (),
        }
    }

    pub fn start(&mut self) -> Result<(), JsValue> {
        let context = get_context();

        let vert_shader = compile_shader(
            &context,
            WebGlRenderingContext::VERTEX_SHADER,
            r#"
            attribute vec2 vertexData;
            uniform mat4 transform;
            varying vec2 texCoords;
            void main() {
                gl_Position = transform * vec4(vertexData.xy, 0.0, 1.0);

                texCoords = (vertexData.xy - 1.) * 0.5;
            }
        "#,
        )?;
        let frag_shader = compile_shader(
            &context,
            WebGlRenderingContext::FRAGMENT_SHADER,
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

        self.assets.texture_loc = context.get_uniform_location(&program, "texture");
        self.assets.transform_loc = context.get_uniform_location(&program, "transform");
        console_log!(
            "assets.transform_loc: {}",
            self.assets.transform_loc.is_some()
        );

        // Tell WebGL we want to affect texture unit 0
        context.active_texture(WebGlRenderingContext::TEXTURE0);

        context.uniform1i(self.assets.texture_loc.as_ref(), 0);

        context.enable(WebGlRenderingContext::BLEND);
        context.blend_equation(WebGlRenderingContext::FUNC_ADD);
        context.blend_func(
            WebGlRenderingContext::SRC_ALPHA,
            WebGlRenderingContext::ONE_MINUS_SRC_ALPHA,
        );

        self.assets.vertex_position = context.get_attrib_location(&program, "vertexData") as u32;
        console_log!("vertex_position: {}", self.assets.vertex_position);

        let rect_vertices: [f32; 8] = [1., 1., -1., 1., -1., -1., 1., -1.];

        let vertex_buffer_data = |vertices: &[f32]| -> Result<WebGlBuffer, JsValue> {
            let buffer = context.create_buffer().ok_or("failed to create buffer")?;
            context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));

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

                context.buffer_data_with_array_buffer_view(
                    WebGlRenderingContext::ARRAY_BUFFER,
                    &vert_array,
                    WebGlRenderingContext::STATIC_DRAW,
                );
            }
            Ok(buffer)
        };

        self.assets.rect_buffer = Some(vertex_buffer_data(&rect_vertices)?);

        context.clear_color(0.0, 0.0, 0.5, 1.0);

        Ok(())
    }

    pub fn render(&mut self) -> Result<(), JsValue> {
        let context = get_context();

        if self.time > 300000 {
            console_log!("All done!");

            // Drop our handle to this closure so that it will get cleaned
            // up once we return.
            // let _ = f.borrow_mut().take();
            return Ok(());
        }

        // Set the body's text content to how many times this
        // requestAnimationFrame callback has fired.
        self.time += 1;
        // console_log!("requestAnimationFrame has been called {} times.", i);

        let dice = 256;
        let rng = &mut self.rng;
        let mut i = rng.gen_range(0, dice);
        let [enemy_count, boss_count] = self.enemies.iter().fold([0; 2], |mut c, e| match e {
            Enemy::Enemy1(_) => {
                c[0] += 1;
                c
            }
            Enemy::Boss(_) => {
                c[1] += 1;
                c
            }
        });
        let gen_amount = 4;
        while i < gen_amount {
            let weights = [
                if enemy_count < 128 {
                    if self.player.score < 1024 {
                        64
                    } else {
                        16
                    }
                } else {
                    0
                },
                if boss_count < 32 { 4 } else { 0 },
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
                    self.enemies.push(match x {
                        0 => Enemy::Enemy1(EnemyBase::new(&mut self.id_gen, pos, velo).health(3)),
                        _ => Enemy::Boss(EnemyBase::new(&mut self.id_gen, pos, velo).health(64)),
                    });
                }
            }
            i += rng.gen_range(0, dice);
        }

        if self.up_pressed {
            self.player.move_up()
        }
        if self.down_pressed {
            self.player.move_down()
        }
        if self.left_pressed {
            self.player.move_left()
        }
        if self.right_pressed {
            self.player.move_right()
        }

        if self.shoot_pressed && self.player.cooldown == 0 {
            let shoot_period = 5;

            if self.time % 5 == 0 {
                let level = self.player.power_level() as i32;
                self.player.cooldown += shoot_period;
                for i in -1 - level..2 + level {
                    let speed = BULLET_SPEED;
                    let ent =
                        Entity::new(&mut self.id_gen, self.player.base.pos, [i as f64, -speed])
                            .rotation((i as f32).atan2(speed as f32));
                    self.shots_bullet += 1;
                    self.bullets
                        .insert(ent.id, Projectile::Bullet(BulletBase(ent)));
                }
            }
        }
        if self.player.cooldown < 1 {
            self.player.cooldown = 0;
        } else {
            self.player.cooldown -= 1;
        }

        context.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

        context.bind_buffer(
            WebGlRenderingContext::ARRAY_BUFFER,
            Some(self.assets.rect_buffer.as_ref().unwrap()),
        );
        context.vertex_attrib_pointer_with_i32(
            self.assets.vertex_position,
            2,
            WebGlRenderingContext::FLOAT,
            false,
            0,
            0,
        );
        context.enable_vertex_attrib_array(self.assets.vertex_position);

        for enemy in &self.enemies {
            enemy.draw(self, &context, &self.assets);
        }

        self.enemies = std::mem::take(&mut self.enemies)
            .into_iter()
            .filter_map(|mut enemy| {
                if let Some(death_reason) = enemy.animate(self) {
                    if let DeathReason::Killed = death_reason {
                        self.player.kills += 1;
                        self.player.score += if enemy.is_boss() { 10 } else { 1 };
                    }
                    None
                } else {
                    Some(enemy)
                }
            })
            .collect();

        self.bullets = std::mem::take(&mut self.bullets)
            .into_iter()
            .filter_map(|(id, mut bullet)| {
                bullet.draw(self, &context, &self.assets);
                if let Some(_reason) = bullet.animate_bullet(&mut self.enemies, &mut self.player) {
                    None
                } else {
                    Some((id, bullet))
                }

                // bullet.pos[0] = bullet.pos[0] + bullet.velo[0];
                // bullet.pos[1] = bullet.pos[1] + bullet.velo[1];
                // if -size < bullet.pos[0]
                //     && bullet.pos[0] < size
                //     && -size < bullet.pos[1]
                //     && bullet.pos[1] < size
                // {
                //     Some(bullet)
                // } else {
                //     None
                // }
            })
            .collect::<HashMap<_, _>>();

        self.player.base.draw_tex(
            &self.assets,
            &context,
            &self.assets.player_texture,
            Some(PLAYER_SIZE),
        );

        fn set_text(id: &str, text: &str) {
            let frame_element = document().get_element_by_id(id).unwrap();
            frame_element.set_inner_html(text);
        }

        set_text("frame", &format!("Frame {}", self.time));
        set_text("score", &format!("Score {}", self.player.score));
        set_text("kills", &format!("Kills {}", self.player.kills));
        set_text("shots", &format!("Shots {}", self.shots_bullet));

        Ok(())
    }
}

pub fn compile_shader(
    context: &WebGlRenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context
        .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
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
    context: &WebGlRenderingContext,
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
        .get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS)
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
fn load_texture(gl: &WebGlRenderingContext, url: &str) -> Result<Rc<WebGlTexture>, JsValue> {
    let texture = Rc::new(gl.create_texture().unwrap());
    gl.bind_texture(WebGlRenderingContext::TEXTURE_2D, Some(&*texture));

    // Because images have to be downloaded over the internet
    // they might take a moment until they are ready.
    // Until then put a single pixel in the texture so we can
    // use it immediately. When the image has finished downloading
    // we'll update the texture with the contents of the image.
    let level = 0;
    let internal_format = WebGlRenderingContext::RGBA as i32;
    let width = 1;
    let height = 1;
    let border = 0;
    let src_format = WebGlRenderingContext::RGBA;
    let src_type = WebGlRenderingContext::UNSIGNED_BYTE;
    let pixel = [0u8, 255, 255, 255];
    gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
        WebGlRenderingContext::TEXTURE_2D,
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
    gl.tex_parameteri(
        WebGlRenderingContext::TEXTURE_2D,
        WebGlRenderingContext::TEXTURE_WRAP_S,
        WebGlRenderingContext::REPEAT as i32,
    );
    gl.tex_parameteri(
        WebGlRenderingContext::TEXTURE_2D,
        WebGlRenderingContext::TEXTURE_WRAP_T,
        WebGlRenderingContext::REPEAT as i32,
    );
    gl.tex_parameteri(
        WebGlRenderingContext::TEXTURE_2D,
        WebGlRenderingContext::TEXTURE_MIN_FILTER,
        WebGlRenderingContext::LINEAR as i32,
    );

    let image = Rc::new(HtmlImageElement::new().unwrap());
    let url_str = url.to_owned();
    let image_clone = image.clone();
    let texture_clone = texture.clone();
    let callback = Closure::wrap(Box::new(move || {
        console_log!("loaded image: {}", url_str);
        // web_sys::console::log_1(Date::new_0().to_locale_string("en-GB", &JsValue::undefined()));

        let gl = get_context();

        gl.bind_texture(WebGlRenderingContext::TEXTURE_2D, Some(&*texture_clone));
        gl.tex_image_2d_with_u32_and_u32_and_image(
            WebGlRenderingContext::TEXTURE_2D,
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
            gl.generate_mipmap(WebGlRenderingContext::TEXTURE_2D);
        } else {
            // No, it's not a power of 2. Turn off mips and set
            // wrapping to clamp to edge
            gl.tex_parameteri(
                WebGlRenderingContext::TEXTURE_2D,
                WebGlRenderingContext::TEXTURE_WRAP_S,
                WebGlRenderingContext::CLAMP_TO_EDGE as i32,
            );
            gl.tex_parameteri(
                WebGlRenderingContext::TEXTURE_2D,
                WebGlRenderingContext::TEXTURE_WRAP_T,
                WebGlRenderingContext::CLAMP_TO_EDGE as i32,
            );
            gl.tex_parameteri(
                WebGlRenderingContext::TEXTURE_2D,
                WebGlRenderingContext::TEXTURE_MIN_FILTER,
                WebGlRenderingContext::LINEAR as i32,
            );
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

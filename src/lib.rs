use cgmath::{Matrix4, Rad, Vector3};
use std::collections::HashMap;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{
    HtmlImageElement, WebGlBuffer, WebGlProgram, WebGlRenderingContext, WebGlShader, WebGlTexture,
    WebGlUniformLocation,
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
use crate::entity::{BulletBase, Enemy, EnemyBase, Entity, Projectile};
use crate::xor128::Xor128;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub(crate) fn log(s: &str);
}

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn get_context() -> WebGlRenderingContext {
    let window = window();
    let document = window.document().unwrap();
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

struct Bullet {
    pub pos: [f64; 2],
    pub velo: [f64; 2],
    pub rotation: f32,
}

#[wasm_bindgen]
pub struct ShooterState {
    time: usize,
    id_gen: u32,
    player: [f64; 2],
    enemies: Vec<Enemy>,
    bullets: HashMap<u32, Projectile>,

    shoot_pressed: bool,
    left_pressed: bool,
    right_pressed: bool,
    up_pressed: bool,
    down_pressed: bool,

    world_transform: Matrix4<f64>,
    texture: Rc<WebGlTexture>,
    player_texture: Rc<WebGlTexture>,
    bullet_texture: Rc<WebGlTexture>,
    rect_buffer: Option<WebGlBuffer>,
    vertex_position: u32,
    texture_loc: Option<WebGlUniformLocation>,
    transform_loc: Option<WebGlUniformLocation>,
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

        Ok(Self {
            time: 0,
            id_gen: 0,
            player: [FWIDTH / 2., FHEIGHT / 2.],
            enemies: vec![],
            bullets: HashMap::new(),
            shoot_pressed: false,
            left_pressed: false,
            right_pressed: false,
            up_pressed: false,
            down_pressed: false,
            world_transform: Matrix4::from_translation(Vector3::new(-1., -1., 0.))
                * &Matrix4::from_nonuniform_scale(2. / FWIDTH, 2. / FHEIGHT, 1.),
            texture: load_texture_local("enemy")?,
            player_texture: load_texture_local("player")?,
            bullet_texture: load_texture_local("bullet")?,
            rect_buffer: None,
            vertex_position: 0,
            texture_loc: None,
            transform_loc: None,
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

        self.texture_loc = context.get_uniform_location(&program, "texture");
        self.transform_loc = context.get_uniform_location(&program, "transform");
        console_log!("transform_loc: {}", self.transform_loc.is_some());

        // Tell WebGL we want to affect texture unit 0
        context.active_texture(WebGlRenderingContext::TEXTURE0);

        context.uniform1i(self.texture_loc.as_ref(), 0);

        context.enable(WebGlRenderingContext::BLEND);
        context.blend_equation(WebGlRenderingContext::FUNC_ADD);
        context.blend_func(
            WebGlRenderingContext::SRC_ALPHA,
            WebGlRenderingContext::ONE_MINUS_SRC_ALPHA,
        );

        self.vertex_position = context.get_attrib_location(&program, "vertexData") as u32;
        console_log!("vertex_position: {}", self.vertex_position);

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

        self.rect_buffer = Some(vertex_buffer_data(&rect_vertices)?);

        let mut random = Xor128::new(3232132);

        for _ in 0..10 {
            let mut enemy = Enemy::Enemy1(EnemyBase::new(
                &mut self.id_gen,
                [random.next(), random.next()],
                [(random.next() - 0.5) * 1., (random.next() - 0.5) * 1.],
            ));
            enemy.get_base_mut().0.rotation = random.next() as f32 * std::f32::consts::PI * 2.;
            enemy.get_base_mut().0.angular_velocity =
                (random.next() - 0.5) as f32 * std::f32::consts::PI * 0.1;
            self.enemies.push(enemy);
        }

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

        if self.shoot_pressed {
            if self.time % 5 == 0 {
                let speed = BULLET_SPEED;
                let ent = Entity::new(&mut self.id_gen, self.player, [0., speed]).rotation(0.);
                self.bullets
                    .insert(ent.id, Projectile::Bullet(BulletBase(ent)));
            }
        }

        let scale = 0.1;
        let size = 1. / scale;

        if self.left_pressed && 0. < self.player[0] - PLAYER_SPEED {
            self.player[0] -= PLAYER_SPEED;
        }
        if self.right_pressed && self.player[0] + PLAYER_SPEED < FWIDTH {
            self.player[0] += PLAYER_SPEED;
        }
        if self.down_pressed && 0. < self.player[1] - PLAYER_SPEED {
            self.player[1] -= PLAYER_SPEED;
        }
        if self.up_pressed && self.player[1] + PLAYER_SPEED < FHEIGHT {
            self.player[1] += PLAYER_SPEED;
        }

        context.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

        // Bind the texture to texture unit 0
        context.bind_texture(WebGlRenderingContext::TEXTURE_2D, Some(&*self.texture));

        context.bind_buffer(
            WebGlRenderingContext::ARRAY_BUFFER,
            Some(self.rect_buffer.as_ref().unwrap()),
        );
        context.vertex_attrib_pointer_with_i32(
            self.vertex_position,
            2,
            WebGlRenderingContext::FLOAT,
            false,
            0,
            0,
        );
        context.enable_vertex_attrib_array(self.vertex_position);

        for enemy in &self.enemies {
            enemy.draw(self, &context, &());
        }

        let mut enemies = std::mem::take(&mut self.enemies);
        for enemy in &mut enemies {
            enemy.animate(self);
        }
        self.enemies = enemies;

        context.bind_texture(
            WebGlRenderingContext::TEXTURE_2D,
            Some(&*self.bullet_texture),
        );

        self.bullets = std::mem::take(&mut self.bullets)
            .into_iter()
            .filter_map(|(id, mut bullet)| {
                bullet.draw(self, &context, &());
                if let Some(reason) = bullet.animate_bullet(&mut self.enemies, &mut self.player) {
                    console_log!(
                        "deathreason: {:?}, pos: {:?}",
                        reason,
                        bullet.get_base().0.pos
                    );
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

        context.bind_texture(
            WebGlRenderingContext::TEXTURE_2D,
            Some(&*self.player_texture),
        );
        let translation =
            Matrix4::from_translation(Vector3::new(self.player[0], self.player[1], 0.));
        let scale_mat = Matrix4::from_nonuniform_scale(PLAYER_SIZE, PLAYER_SIZE, 1.);
        let rotation = Matrix4::from_angle_z(Rad(0.));
        let transform = &self.world_transform
            * &translation
            * &rotation
            * &scale_mat
            * &Matrix4::from_nonuniform_scale(1., -1., 1.);
        context.uniform_matrix4fv_with_f32_array(
            self.transform_loc.as_ref(),
            false,
            <Matrix4<f32> as AsRef<[f32; 16]>>::as_ref(&transform.cast().unwrap()),
        );
        context.draw_arrays(WebGlRenderingContext::TRIANGLE_FAN, 0, 4);

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

        let f = || -> Result<(), JsValue> {
            let window = web_sys::window().unwrap();
            let document = window.document().unwrap();
            let canvas = document.get_element_by_id("canvas").unwrap();
            let canvas: web_sys::HtmlCanvasElement =
                canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;
            let context = canvas
                .get_context("webgl")?
                .unwrap()
                .dyn_into::<WebGlRenderingContext>()?;

            context.bind_texture(WebGlRenderingContext::TEXTURE_2D, Some(&*texture_clone));
            context.tex_image_2d_with_u32_and_u32_and_image(
                WebGlRenderingContext::TEXTURE_2D,
                level,
                internal_format,
                src_format,
                src_type,
                &image_clone,
            )?;
            Ok(())
        };

        f().ok();

        //   // WebGL1 has different requirements for power of 2 images
        //   // vs non power of 2 images so check if the image is a
        //   // power of 2 in both dimensions.
        //   if (is_power_of_2(image.width) && is_power_of_2(image.height)) {
        //      // Yes, it's a power of 2. Generate mips.
        //      gl.generateMipmap(gl.TEXTURE_2D);
        //   } else {
        //      // No, it's not a power of 2. Turn off mips and set
        //      // wrapping to clamp to edge
        //      gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE);
        //      gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE);
        //      gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.LINEAR);
        //   }
    }) as Box<dyn FnMut()>);
    image.set_onload(Some(callback.as_ref().unchecked_ref()));
    image.set_src(url);

    callback.forget();

    Ok(texture)
}

fn _is_power_of_2(value: usize) -> bool {
    (value & (value - 1)) == 0
}

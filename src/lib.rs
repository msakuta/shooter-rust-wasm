mod xor128;

use crate::xor128::Xor128;
use cgmath::{Matrix4, Rad, Vector3};
use std::cell::RefCell;
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

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub(crate) fn log(s: &str);
}

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

struct Enemy {
    position: [f64; 2],
    velocity: [f64; 2],
    angle: f64,
    angular_velocity: f64,
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let window = window();
    let document = window.document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

    let context = canvas
        .get_context("webgl")?
        .unwrap()
        .dyn_into::<WebGlRenderingContext>()?;

    let vert_shader = compile_shader(
        &context,
        WebGlRenderingContext::VERTEX_SHADER,
        r#"
        attribute vec2 vertexData;
        uniform mat4 transform;
        varying vec2 texCoords;
        void main() {
            gl_Position = transform * vec4(vertexData.xy, 0.0, 1.0);

            texCoords = vertexData.xy + 0.5;
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

    let texture_loc = context.get_uniform_location(&program, "texture");
    console_log!("texture_loc: {}", texture_loc.is_some());
    let texture = load_texture(&context, "./assets/enemy.png")?;

    let transform_loc = context.get_uniform_location(&program, "transform");
    console_log!("transform_loc: {}", transform_loc.is_some());

    // Tell WebGL we want to affect texture unit 0
    context.active_texture(WebGlRenderingContext::TEXTURE0);

    // Bind the texture to texture unit 0
    context.bind_texture(WebGlRenderingContext::TEXTURE_2D, Some(&*texture));
    context.uniform1i(texture_loc.as_ref(), 0);

    context.enable(WebGlRenderingContext::BLEND);
    context.blend_equation(WebGlRenderingContext::FUNC_ADD);
    context.blend_func(
        WebGlRenderingContext::SRC_ALPHA,
        WebGlRenderingContext::ONE_MINUS_SRC_ALPHA,
    );

    let vertex_position = context.get_attrib_location(&program, "vertexData") as u32;
    console_log!("vertex_position: {}", vertex_position);

    let rect_vertices: [f32; 8] = [0.5, 0.5, -0.5, 0.5, -0.5, -0.5, 0.5, -0.5];

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

    let rect_buffer = vertex_buffer_data(&rect_vertices)?;

    let mut random = Xor128::new(3232132);

    let mut enemies = vec![];
    for _ in 0..10 {
        enemies.push(Enemy {
            position: [random.next(), random.next()],
            velocity: [(random.next() - 0.5) * 0.1, (random.next() - 0.5) * 0.1],
            angle: random.next() * std::f64::consts::PI * 2.,
            angular_velocity: (random.next() - 0.5) * std::f64::consts::PI * 0.1,
        });
    }

    context.clear_color(0.0, 0.0, 0.5, 1.0);

    // Here we want to call `requestAnimationFrame` in a loop, but only a fixed
    // number of times. After it's done we want all our resources cleaned up. To
    // achieve this we're using an `Rc`. The `Rc` will eventually store the
    // closure we want to execute on each frame, but to start out it contains
    // `None`.
    //
    // After the `Rc` is made we'll actually create the closure, and the closure
    // will reference one of the `Rc` instances. The other `Rc` reference is
    // used to store the closure, request the first frame, and then is dropped
    // by this function.
    //
    // Inside the closure we've got a persistent `Rc` reference, which we use
    // for all future iterations of the loop
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    let mut i = 0;
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        if i > 300000 {
            console_log!("All done!");

            // Drop our handle to this closure so that it will get cleaned
            // up once we return.
            let _ = f.borrow_mut().take();
            return;
        }

        // Set the body's text content to how many times this
        // requestAnimationFrame callback has fired.
        i += 1;
        console_log!("requestAnimationFrame has been called {} times.", i);

        context.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

        for enemy in &mut enemies {
            let scale = 0.1;
            let size = 1. / scale;
            let angle = enemy.angle as f32;
            let scale_mat = Matrix4::from_scale(scale as f32);
            let rotation = Matrix4::from_angle_z(Rad(angle));
            let translation = Matrix4::from_translation(Vector3::new(
                enemy.position[0] as f32,
                enemy.position[1] as f32,
                0.,
            ));
            let transform = &scale_mat * &translation * &rotation;
            context.uniform_matrix4fv_with_f32_array(
                transform_loc.as_ref(),
                false,
                <Matrix4<f32> as AsRef<[f32; 16]>>::as_ref(&transform),
            );

            context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&rect_buffer));
            context.vertex_attrib_pointer_with_i32(
                vertex_position,
                2,
                WebGlRenderingContext::FLOAT,
                false,
                0,
                0,
            );
            context.enable_vertex_attrib_array(vertex_position);
            context.draw_arrays(WebGlRenderingContext::TRIANGLE_FAN, 0, 4);

            fn wrap(v: f64, size: f64) -> f64 {
                let size2 = size * 2.;
                v - ((v + size) / size2).floor() * size2
            }

            enemy.position[0] = wrap(enemy.position[0] + enemy.velocity[0], size as f64);
            enemy.position[1] = wrap(enemy.position[1] + enemy.velocity[1], size as f64);
            enemy.angle += enemy.angular_velocity;
        }

        // Schedule ourself for another requestAnimationFrame callback.
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());
    Ok(())
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

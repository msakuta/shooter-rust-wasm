
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlShader, HtmlImageElement, WebGlTexture};

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

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
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
        // attribute vec4 position;
        attribute vec4 vertexData; // <vec2 position, vec2 texCoords>
        varying vec2 texCoords;
        void main() {
            // gl_Position = position;
            gl_Position = vec4(vertexData.xy, 0.0, 1.0);

            texCoords = vertexData.xy * 5.;
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
            // gl_FragColor = vec4(0.5, 1.0, 1.0, 1.0);
            gl_FragColor = texture2D( texture, vec2(texCoords.x, texCoords.y) );
            // gl_FragColor = texture2D( texture, vec2(0.5, 0.5) );
        }
    "#,
    )?;
    let program = link_program(&context, &vert_shader, &frag_shader)?;
    context.use_program(Some(&program));

    let loc = context.get_uniform_location(&program, "texture");
    console_log!("loc: {}", loc.is_some());
    let texture = load_texture(&context, "");

    // Tell WebGL we want to affect texture unit 0
    context.active_texture(WebGlRenderingContext::TEXTURE0);

    // Bind the texture to texture unit 0
    context.bind_texture(WebGlRenderingContext::TEXTURE_2D, texture.as_ref());
    context.uniform1i(loc.as_ref(), 0);

    let vertices: [f32; 9] = [-0.7, -0.7, 0.0, 0.7, -0.7, 0.0, 0.0, 0.7, 0.0];

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
        let vert_array = js_sys::Float32Array::view(&vertices);

        context.buffer_data_with_array_buffer_view(
            WebGlRenderingContext::ARRAY_BUFFER,
            &vert_array,
            WebGlRenderingContext::STATIC_DRAW,
        );
    }

    context.vertex_attrib_pointer_with_i32(0, 3, WebGlRenderingContext::FLOAT, false, 0, 0);
    context.enable_vertex_attrib_array(0);

    context.clear_color(0.0, 0.0, 0.0, 1.0);
    context.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

    context.draw_arrays(
        WebGlRenderingContext::TRIANGLES,
        0,
        (vertices.len() / 3) as i32,
    );
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
fn load_texture(gl: &WebGlRenderingContext, url: &str) -> Option<WebGlTexture> {
    let texture = gl.create_texture();
    gl.bind_texture(WebGlRenderingContext::TEXTURE_2D, texture.as_ref());
  
    // Because images have to be downloaded over the internet
    // they might take a moment until they are ready.
    // Until then put a single pixel in the texture so we can
    // use it immediately. When the image has finished downloading
    // we'll update the texture with the contents of the image.
    let level = 0;
    let internal_format = WebGlRenderingContext::RGBA as i32;
    let width = 2;
    let height = 2;
    let border = 0;
    let src_format = WebGlRenderingContext::RGBA;
    let src_type = WebGlRenderingContext::UNSIGNED_BYTE;
    let pixel = [
        0u8, 255, 255, 255,
        255, 0, 0, 255,
        0, 255, 0, 255,
        255,255,255,255,
    ];
    gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
        WebGlRenderingContext::TEXTURE_2D, level, internal_format,
                  width, height, border, src_format, src_type,
                  Some(&pixel)).unwrap();
    gl.tex_parameteri(WebGlRenderingContext::TEXTURE_2D, WebGlRenderingContext::TEXTURE_WRAP_S, WebGlRenderingContext::REPEAT as i32);
    gl.tex_parameteri(WebGlRenderingContext::TEXTURE_2D, WebGlRenderingContext::TEXTURE_WRAP_T, WebGlRenderingContext::REPEAT as i32);
    gl.tex_parameteri(WebGlRenderingContext::TEXTURE_2D, WebGlRenderingContext::TEXTURE_MIN_FILTER, WebGlRenderingContext::LINEAR as i32);

    let image = HtmlImageElement::new().unwrap();
    // image.set_onload(|| {
    //   gl.bind_texture(WebGlRenderingContext::TEXTURE_2D, texture);
    //   gl.texImage2D(WebGlRenderingContext::TEXTURE_2D, level, internal_format,
    //                 srcFormat, srcType, image);
  
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
    // });
    image.set_src(url);
  
    return texture;
  }

  fn is_power_of_2(value: usize) -> bool {
    (value & (value - 1)) == 0
  }
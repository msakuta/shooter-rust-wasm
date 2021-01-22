#[cfg(all(not(feature = "webgl"), feature = "piston"))]
use piston_window::{
    draw_state::Blend,
    math::{rotate_radians, scale, translate},
    *,
};
#[cfg(feature = "webgl")]
use std::rc::Rc;
use std::{collections::HashMap, vec};
#[cfg(feature = "webgl")]
use wasm_bindgen::{prelude::*, JsCast};
#[cfg(feature = "webgl")]
use web_sys::{
    Document, Element, HtmlImageElement, WebGlBuffer, WebGlProgram, WebGlRenderingContext as GL,
    WebGlShader, WebGlTexture,
};

#[cfg(feature = "webgl")]
macro_rules! console_log {
    ($fmt:expr, $($arg1:expr),*) => {
        crate::log(&format!($fmt, $($arg1),+))
    };
    ($fmt:expr) => {
        crate::log($fmt)
    }
}

#[cfg(not(feature = "webgl"))]
macro_rules! console_log {
    ($fmt:expr, $($arg1:expr),*) => {
        println!($fmt, $($arg1),+)
    };
    ($fmt:expr) => {
        println!($fmt)
    }
}

#[cfg(feature = "webgl")]
/// format-like macro that returns js_sys::String
macro_rules! js_str {
    ($fmt:expr, $($arg1:expr),*) => {
        JsValue::from_str(&format!($fmt, $($arg1),+))
    };
    ($fmt:expr) => {
        JsValue::from_str($fmt)
    }
}

#[cfg(feature = "webgl")]
/// format-like macro that returns Err(js_sys::String)
macro_rules! js_err {
    ($fmt:expr, $($arg1:expr),*) => {
        Err(JsValue::from_str(&format!($fmt, $($arg1),+)))
    };
    ($fmt:expr) => {
        Err(JsValue::from_str($fmt))
    }
}

pub mod consts;
pub mod entity;
pub mod xor128;

use crate::consts::*;
#[cfg(feature = "webgl")]
use crate::entity::ShaderBundle;
use crate::entity::{
    Assets, BulletBase, DeathReason, Enemy, EnemyBase, Entity, Item, Player, Projectile,
    ShieldedBoss, TempEntity, Weapon,
};
use xor128::Xor128;

#[cfg(feature = "webgl")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub(crate) fn log(s: &str);
}

#[cfg(feature = "webgl")]
pub type ShooterError = JsValue;

#[cfg(not(feature = "webgl"))]
pub type ShooterError = std::io::Error;

pub struct ShooterState {
    pub time: usize,
    pub disptime: usize,
    pub paused: bool,
    pub game_over: bool,
    pub id_gen: u32,
    pub player: Player,
    pub enemies: Vec<Enemy>,
    pub items: Vec<Item>,
    pub bullets: HashMap<u32, Projectile>,
    #[cfg(feature = "webgl")]
    pub tent: Vec<TempEntity>,
    pub rng: Xor128,
    pub shots_bullet: usize,
    pub shots_missile: usize,

    pub shoot_pressed: bool,
    pub left_pressed: bool,
    pub right_pressed: bool,
    pub up_pressed: bool,
    pub down_pressed: bool,

    #[cfg(feature = "webgl")]
    pub assets: Assets,
}

impl ShooterState {
    pub fn new(assets: Option<Assets>) -> Self {
        let mut id_gen = 0;
        let mut player = Player::new(Entity::new(
            &mut id_gen,
            [FWIDTH / 2., FHEIGHT / 2.],
            [0., 0.],
        ));
        player.reset();

        ShooterState {
            time: 0,
            disptime: 0,
            paused: false,
            game_over: false,
            id_gen,
            player,
            enemies: vec![],
            items: vec![],
            bullets: HashMap::new(),
            #[cfg(feature = "webgl")]
            tent: vec![],
            rng: Xor128::new(3232132),
            shots_bullet: 0,
            shots_missile: 0,
            shoot_pressed: false,
            left_pressed: false,
            right_pressed: false,
            up_pressed: false,
            down_pressed: false,
            #[cfg(feature = "webgl")]
            assets: assets.unwrap(),
        }
    }
    pub fn restart(&mut self) -> Result<(), ShooterError> {
        self.items.clear();
        self.enemies.clear();
        self.bullets.clear();
        #[cfg(feature = "webgl")]
        self.tent.clear();
        self.time = 0;
        self.id_gen = 0;
        self.player.reset();
        self.shots_bullet = 0;
        self.shots_missile = 0;
        self.paused = false;
        self.game_over = false;
        Ok(())
    }
}

#[cfg(feature = "webgl")]
//
// Initialize a texture and load an image.
// When the image finished loading copy it into the texture.
//
fn load_texture(gl: &GL, url: &str) -> Result<Rc<WebGlTexture>, JsValue> {
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

    fn is_power_of_2(value: u32) -> bool {
        (value & (value - 1)) == 0
    }

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

#[cfg(feature = "webgl")]
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

#[cfg(feature = "webgl")]
fn enable_buffer(gl: &GL, buffer: &Option<WebGlBuffer>, elements: i32, vertex_position: u32) {
    gl.bind_buffer(GL::ARRAY_BUFFER, buffer.as_ref());
    gl.vertex_attrib_pointer_with_i32(vertex_position, elements, GL::FLOAT, false, 0, 0);
    gl.enable_vertex_attrib_array(vertex_position);
}

#[cfg(feature = "webgl")]
use js_sys::Atomics::xor;
#[cfg(feature = "webgl")]
use web_sys::{
    Element, HtmlImageElement, WebGlBuffer, WebGlProgram, WebGlRenderingContext as GL, WebGlShader,
    WebGlTexture,
};
#[cfg(feature = "webgl")]
use wasm_bindgen::{prelude::*, JsCast};
use std::{collections::HashMap, vec};

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
use crate::entity::{
    Assets, BulletBase, DeathReason, Enemy, EnemyBase, Entity, Item, Player, Projectile,
    ShieldedBoss, TempEntity, Weapon,
};
#[cfg(feature = "webgl")]
use crate::entity::ShaderBundle;
use xor128::Xor128;

#[cfg(feature = "webgl")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub(crate) fn log(s: &str);
}

#[cfg(feature = "webgl")]
type ShooterError = JsValue;

#[cfg(not(feature = "webgl"))]
type ShooterError = std::io::Error;

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
    pub player_live_icons: Vec<Element>,

    pub assets: Assets,
}

impl ShooterState {
    pub fn restart(&mut self) -> Result<(), ShooterError> {
        self.items.clear();
        self.enemies.clear();
        self.bullets.clear();
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

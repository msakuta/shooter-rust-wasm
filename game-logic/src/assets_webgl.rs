use crate::{consts::*, load_texture};
use cgmath::{Matrix4, Vector3};
use std::rc::Rc;
use wasm_bindgen::JsValue;
use web_sys::{
    Document, Element, WebGlBuffer, WebGlProgram, WebGlRenderingContext as GL, WebGlTexture,
    WebGlUniformLocation,
};

pub struct ShaderBundle {
    pub program: WebGlProgram,
    pub vertex_position: u32,
    pub tex_coord_position: u32,
    pub texture_loc: Option<WebGlUniformLocation>,
    pub transform_loc: Option<WebGlUniformLocation>,
    pub tex_transform_loc: Option<WebGlUniformLocation>,
    pub alpha_loc: Option<WebGlUniformLocation>,
}

impl ShaderBundle {
    pub fn new(gl: &GL, program: WebGlProgram) -> Self {
        let get_uniform = |location: &str| {
            let op: Option<WebGlUniformLocation> = gl.get_uniform_location(&program, location);
            if op.is_none() {
                console_log!("Warning: location {} undefined", location);
            } else {
                console_log!("location {} defined", location);
            }
            op
        };
        let vertex_position = gl.get_attrib_location(&program, "vertexData") as u32;
        let tex_coord_position = gl.get_attrib_location(&program, "vertexData") as u32;
        console_log!("vertex_position: {}", vertex_position);
        console_log!("tex_coord_position: {}", tex_coord_position);
        Self {
            vertex_position,
            tex_coord_position,
            texture_loc: get_uniform("texture"),
            transform_loc: get_uniform("transform"),
            tex_transform_loc: get_uniform("texTransform"),
            alpha_loc: get_uniform("alpha"),
            // Program has to be later than others
            program,
        }
    }
}

#[cfg(feature = "webgl")]
pub struct Assets {
    pub world_transform: Matrix4<f64>,

    pub enemy_tex: Rc<WebGlTexture>,
    pub boss_tex: Rc<WebGlTexture>,
    pub shield_tex: Rc<WebGlTexture>,
    pub spiral_enemy_tex: Rc<WebGlTexture>,
    pub centipede_head_tex: Rc<WebGlTexture>,
    pub centipede_segment_tex: Rc<WebGlTexture>,
    pub player_texture: Rc<WebGlTexture>,
    pub bullet_texture: Rc<WebGlTexture>,
    pub enemy_bullet_texture: Rc<WebGlTexture>,
    pub phase_bullet_tex: Rc<WebGlTexture>,
    pub spiral_bullet_tex: Rc<WebGlTexture>,
    pub missile_tex: Rc<WebGlTexture>,
    pub red_glow_tex: Rc<WebGlTexture>,
    pub explode_tex: Rc<WebGlTexture>,
    pub explode2_tex: Rc<WebGlTexture>,
    pub blood_tex: Rc<WebGlTexture>,
    pub trail_tex: Rc<WebGlTexture>,
    pub beam_tex: Rc<WebGlTexture>,
    pub back_tex: Rc<WebGlTexture>,
    pub power_tex: Rc<WebGlTexture>,
    pub power2_tex: Rc<WebGlTexture>,
    pub sphere_tex: Rc<WebGlTexture>,
    pub weapons_tex: Rc<WebGlTexture>,

    pub sprite_shader: Option<ShaderBundle>,
    pub trail_shader: Option<ShaderBundle>,
    pub trail_buffer: Option<WebGlBuffer>,
    pub rect_buffer: Option<WebGlBuffer>,

    pub player_live_icons: Vec<Element>,
}

impl Assets {
    pub fn new(
        document: &Document,
        context: &GL,
        image_assets: js_sys::Array,
    ) -> Result<Self, JsValue> {
        let side_panel = document.get_element_by_id("sidePanel").unwrap();

        let player_live_icons = (0..3)
            .map(|_| {
                let lives_icon = document.create_element("img")?;
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

        Ok(Assets {
            world_transform: Matrix4::from_translation(Vector3::new(-1., 1., 0.))
                * Matrix4::from_nonuniform_scale(2. / FWIDTH, -2. / FHEIGHT, 1.),
            enemy_tex: load_texture_local("enemy")?,
            boss_tex: load_texture_local("boss")?,
            shield_tex: load_texture_local("shield")?,
            spiral_enemy_tex: load_texture_local("spiralEnemy")?,
            centipede_head_tex: load_texture_local("centipedeHead")?,
            centipede_segment_tex: load_texture_local("centipedeSegment")?,
            player_texture: load_texture_local("player")?,
            bullet_texture: load_texture_local("bullet")?,
            enemy_bullet_texture: load_texture_local("ebullet")?,
            phase_bullet_tex: load_texture_local("phaseBullet")?,
            spiral_bullet_tex: load_texture_local("spiralBullet")?,
            missile_tex: load_texture_local("missile")?,
            red_glow_tex: load_texture_local("redGlow")?,
            explode_tex: load_texture_local("explode")?,
            explode2_tex: load_texture_local("explode2")?,
            blood_tex: load_texture_local("blood")?,
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
            player_live_icons,
        })
    }
}

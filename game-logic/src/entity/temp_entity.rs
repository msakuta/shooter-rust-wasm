use std::rc::Rc;

#[cfg(feature = "webgl")]
use crate::assets_webgl::Assets;

use super::{DeathReason, Entity};
#[cfg(feature = "webgl")]
use cgmath::{Matrix3, Matrix4, Rad, Vector2, Vector3};
#[cfg(feature = "webgl")]
use web_sys::{WebGlRenderingContext as GL, WebGlTexture};

#[cfg(all(not(feature = "webgl"), feature = "piston"))]
use piston_window::{
    math::{rotate_radians, translate},
    *,
};

#[cfg(all(not(feature = "webgl"), feature = "piston"))]
use super::Matrix;

#[cfg(feature = "webgl")]
pub struct TempEntity {
    pub base: Entity,
    pub texture: Rc<WebGlTexture>,
    pub max_frames: u32,
    pub width: u32,
    pub playback_rate: u32,
    pub image_width: u32,
    pub size: f64,
}

#[cfg(all(not(feature = "webgl"), feature = "piston"))]
pub struct TempEntity {
    pub base: Entity,
    pub texture: Rc<G2dTexture>,
    pub max_frames: u32,
    pub width: u32,
    pub playback_rate: u32,
}

#[cfg(feature = "webgl")]
impl TempEntity {
    #[allow(dead_code)]
    pub fn max_frames(mut self, max_frames: u32) -> Self {
        self.max_frames = max_frames;
        self
    }
    pub fn animate_temp(&mut self) -> Option<DeathReason> {
        self.base.health -= 1;
        self.base.animate()
    }

    pub fn draw_temp(&self, context: &GL, assets: &Assets) {
        let shader = assets.sprite_shader.as_ref().unwrap();
        let pos = &self.base.pos;
        context.bind_texture(GL::TEXTURE_2D, Some(&self.texture));
        let rotation = Matrix4::from_angle_z(Rad(self.base.rotation as f64));
        let translation = Matrix4::from_translation(Vector3::new(pos[0], pos[1], 0.));
        let scale = Matrix4::from_scale(self.size);
        let frame = self.max_frames - (self.base.health as u32 / self.playback_rate) as u32;
        // let image   = Image::new().rect([0f64, 0f64, self.width as f64, tex2.get_height() as f64])
        //     .src_rect([frame as f64 * self.width as f64, 0., self.width as f64, tex2.get_height() as f64]);
        let transform = assets.world_transform * translation * rotation * scale;
        context.uniform_matrix4fv_with_f32_array(
            shader.transform_loc.as_ref(),
            false,
            <Matrix4<f32> as AsRef<[f32; 16]>>::as_ref(&transform.cast().unwrap()),
        );

        let tex_translate = Matrix3::from_translation(Vector2::new(frame as f32, 0.));
        let tex_scale =
            Matrix3::from_nonuniform_scale(self.width as f32 / self.image_width as f32, 1.);
        context.uniform_matrix3fv_with_f32_array(
            shader.tex_transform_loc.as_ref(),
            false,
            <Matrix3<f32> as AsRef<[f32; 9]>>::as_ref(&(tex_scale * tex_translate)),
        );

        context.draw_arrays(GL::TRIANGLE_FAN, 0, 4);
    }
}

#[cfg(all(not(feature = "webgl"), feature = "piston"))]
impl TempEntity {
    #[allow(dead_code)]
    pub fn max_frames(mut self, max_frames: u32) -> Self {
        self.max_frames = max_frames;
        self
    }
    pub fn animate_temp(&mut self) -> Option<DeathReason> {
        self.base.health -= 1;
        self.base.animate()
    }

    pub fn draw_temp(&self, context: &Context, g: &mut G2d) {
        let pos = &self.base.pos;
        let tex2 = &*self.texture;
        let centerize = translate([-(16. / 2.), -(tex2.get_height() as f64 / 2.)]);
        let rotmat = rotate_radians(self.base.rotation as f64);
        let translate = translate(*pos);
        let frame = self.max_frames - (self.base.health as u32 / self.playback_rate) as u32;
        let draw_state = if let Some(blend_mode) = self.base.blend {
            context.draw_state.blend(blend_mode)
        } else {
            context.draw_state
        };
        let image = Image::new()
            .rect([0f64, 0f64, self.width as f64, tex2.get_height() as f64])
            .src_rect([
                frame as f64 * self.width as f64,
                0.,
                self.width as f64,
                tex2.get_height() as f64,
            ]);
        image.draw(
            tex2,
            &draw_state,
            (Matrix(context.transform) * Matrix(translate) * Matrix(rotmat) * Matrix(centerize)).0,
            g,
        );
    }
}

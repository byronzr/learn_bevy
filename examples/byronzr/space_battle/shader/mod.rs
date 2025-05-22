use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy::sprite::{AlphaMode2d, Material2d};

#[derive(AsBindGroup, Asset, TypePath, Debug, Clone)]
pub struct MaterialEngineFlame {
    #[texture(0)]
    #[sampler(1)]
    pub my_texture: Handle<Image>,
    #[uniform(2)]
    pub lumina: LinearRgba,
    #[uniform(3)]
    pub time: f32,
    pub lumina_value: f32,
}

impl Material2d for MaterialEngineFlame {
    fn fragment_shader() -> ShaderRef {
        "space_battle/engine_flame.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
        //AlphaMode2d::Opaque
    }
}

impl MaterialEngineFlame {
    pub fn start(&mut self) {
        if self.time < 1.0 {
            self.time += 0.2;
        }
        if self.lumina_value < 80.0 {
            self.lumina_value += 40.;
            self.lumina.set_alpha(self.lumina_value);
        }
    }
    pub fn stop(&mut self) {
        if self.time > -1.0 {
            self.time -= 0.2;
        }
        if self.lumina_value > 0.0 {
            self.lumina_value -= 20.;
            self.lumina.set_alpha(self.lumina_value);
        }
    }
}

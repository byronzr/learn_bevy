#import bevy_sprite::{
    mesh2d_view_bindings::globals,      // 都有一个 globals
    mesh2d_vertex_output::VertexOutput,
}

// #import bevy_pbr::{
//     mesh_view_bindings::globals,     // 也有一个 globals
//     forward_io::VertexOutput,
// }

// we can import items from shader modules in the assets folder with a quoted path
#import "shaders/custom_material_import.wgsl"::COLOR_MULTIPLIER

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> material_color: vec4<f32>;
// 原始的纹理
@group(#{MATERIAL_BIND_GROUP}) @binding(1) var base_color_texture: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(2) var base_color_sampler: sampler;
// 用于 dithering 的纹理和采样器
@group(#{MATERIAL_BIND_GROUP}) @binding(3) var dither_texture: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(4) var dither_sampler: sampler;
// 用于fract函数比例,简单将两个图像都设为正方形
@group(#{MATERIAL_BIND_GROUP}) @binding(5) var<uniform> ratio: vec2<f32>; 



@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let dither_uv = fract(mesh.uv * ratio);
    let dither_color = textureSample(dither_texture, dither_sampler, dither_uv);
    let base_color = textureSample(base_color_texture, base_color_sampler, mesh.uv);

    // 来自于 ITU-R BT.601标准,人类眼部神经对于亮度的感知权重
    let weight = vec3<f32>(0.299, 0.587, 0.114);
    let luminance_image = dot(base_color.rgb, weight);
    let luminance_dither = dot(dither_color.rgb, weight);

    let pen_color = vec4f(1.0,0.0,0.0,1.0);
    let background = vec4f(1.0,1.0,0.0,1.0);

    // 视觉上的细腻感是从两个维度来体现的：
    // 1. dither 本身的纹理体现(但更多的时候是用来体现笔触)
    // 2. dither 本身的亮度可以作为阈值来使用(大多数情况下)
    let dither_threshold = textureSample(dither_texture, dither_sampler, dither_uv).r; 
    // 使用时间来体现阈值动画
    if (luminance_image * abs(sin(globals.time)) > luminance_dither) {
        return background;
    } else {
        return pen_color;
    }
}

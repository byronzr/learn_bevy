#import bevy_sprite::{
    mesh2d_view_bindings::globals,      // 都有一个 globals
    mesh2d_vertex_output::VertexOutput,
}

// search => #define_import_path bevy_render::globals
// path => /Users/byronzr/rProjects/bevy/crates/bevy_render/src/globals.wgsl
// struct Globals {
//     time: f32,
//     delta_time: f32,
//     frame_count: u32,
// #ifdef SIXTEEN_BYTE_ALIGNMENT
//     _webgl2_padding: f32
// #endif
// };

// search => #define_import_path bevy_sprite::mesh2d_vertex_output     
// path=> /Users/byronzr/rProjects/bevy/crates/bevy_sprite_render/src/mesh2d/mesh2d_vertex_output.wgsl
// struct VertexOutput {
//     @builtin(position) position: vec4<f32>,
//     @location(0) world_position: vec4<f32>,
//     @location(1) world_normal: vec3<f32>,
//     @location(2) uv: vec2<f32>,
//     #ifdef VERTEX_TANGENTS
//     @location(3) world_tangent: vec4<f32>,
//     #endif
//     #ifdef VERTEX_COLORS
//     @location(4) color: vec4<f32>,
//     #endif
// }

// we can import items from shader modules in the assets folder with a quoted path
#import "shaders/custom_material_import.wgsl"::COLOR_MULTIPLIER

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> draw_color: vec4<f32>;
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
    // 因为 uv 的原始区间为 0.->1.
    // fract(uv * ratio) 取小数部分，相当于放大(分段) ratio 取当前分段区间
    // 这是一种常用的分段（映射）手法
    let dither_uv = fract(mesh.uv * ratio);
    let dither_color = textureSample(dither_texture, dither_sampler, dither_uv);
    let base_color = textureSample(base_color_texture, base_color_sampler, mesh.uv);

    // * 提取亮度信息
    // * 来自于 ITU-R BT.601标准,人类眼部神经对于亮度的感知权重
    // let weight = vec3<f32>(0.299, 0.587, 0.114);
    // let luminance_image = dot(base_color.rgb, weight);
    // let luminance_dither = dot(dither_color.rgb, weight);

    // * 提取亮度信息
    // * 线性空间的 luma（BT.709 / sRGB）
    let weight = vec3<f32>(0.2126, 0.7152, 0.0722);
    let luminance_image = dot(base_color.rgb, weight); // image 亮度
    let luminance_dither = dot(dither_color.rgb, weight); // dither 亮度

    // 背景色(画布)
    let canvas_color = vec4f(1.0,1.0,1.0,1.0);

    // 使用时间来体现阈值动画
    // 我们并没有实现视觉上的扩散效果，只是每个 dither 影响的分段区间的明度
    // GPU运行足够快
    if (luminance_image * abs(sin(globals.time)) < luminance_dither) {
        return draw_color;
    } else {
        return canvas_color;
    }
}

#import bevy_sprite::{
    mesh2d_view_bindings::globals,
    mesh2d_vertex_output::VertexOutput,
}
// #import bevy_pbr::mesh_view_bindings::globals

// #import bevy_pbr::{
//     mesh_view_bindings::globals,
//     forward_io::VertexOutput,
// }

@group(2) @binding(0) var my_texture: texture_2d<f32>;
@group(2) @binding(1) var my_sampler: sampler;
@group(2) @binding(2) var<uniform> lumina: vec4<f32>;
@group(2) @binding(3) var<uniform> time: f32;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let v = sin(globals.time)*0.2;
    let uv = in.uv;
    let center = 0.5;
    let amplitude = 0.2; // 最大收缩幅度（0~0.5之间，越大收缩越明显）
    //let scale = 1.0 - amplitude * sin(globals.time); // scale 在 [1-amplitude, 1+amplitude] 之间变化
    let scale = 1.0 - amplitude * sin(time)-v; // scale 在 [1-amplitude, 1+amplitude] 之间变化
    let new_x = (uv.x - center) * scale + center;
    let new_uv = vec2<f32>(new_x, uv.y);
    
    // 标准亮度加权值
    var raw_color = textureSample(my_texture, my_sampler, new_uv); 
    let luminance = dot(raw_color.rgb, vec3<f32>(0.2126, 0.7152, 0.0722));

    // 伪随机
    let rnd = fract(sin(dot(vec2<f32>(uv.x, uv.y) + globals.time, vec2<f32>(12.9898, 78.233))) * 43758.5453);
    let rnd2 = fract(sin(dot(vec2<f32>(uv.x, uv.y) + globals.time, vec2<f32>(12.9898, 78.233))) * 10000.5453);

    if raw_color.a > 0.5  {
        raw_color =  lumina * textureSample(my_texture, my_sampler, new_uv); 
    }

    if time == -1 {
        raw_color = vec4<f32>(0.0, 0.0, 0.0, 1.0);
    }

    return rnd * rnd * raw_color;
}
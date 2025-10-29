
#import bevy_sprite::{
    mesh2d_view_bindings::globals,
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

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> material_color: vec4<f32>;



@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let base_line = 0.5;
    let max_offset = 0.2;
    let noise_scale = 10.0;

    let c = noise(mesh.uv.x * noise_scale + globals.time);
    let noise_value = c * max_offset;
    // step 返回对应的 0/1 浮点结果
    // step(edge,v)
    let edge_value = step(base_line+noise_value,mesh.uv.y);

    // wgsl 对于 mix 参数会自动推导与扩展，相对宽松,
    // mix 实际是对 向量进行插值
    // let color = mix(vec4(1.0),vec4(vec3(0.0),1.0),vec4(edge_value));
    // 由于 edge_value 通过 step 返回 0|1，所以这里确实可以用 if 代替 mix 来方便理解
    // 然后 GPU 更倾向于"无分支"的数据流，往往 mix 是更好的选择
    let color = mix(vec4(1.0),material_color,edge_value);
    return color;
}

fn random(x:f32)->f32 {
    return fract(sin(x*1000.0)*5323.1323);
}

fn noise(x:f32)->f32 {
    // 整数部分
    let i = floor(x);
    // 小数部分
    let f = fract(x);
    // 整数区间
    let a = random(i);
    let b = random(i+1.0);

    // 缓动曲线
    let u = f * f * (3.0 - 2.0*f);
    return mix(a,b,u);
}

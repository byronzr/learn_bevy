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
//     @builtin(position) position: vec4f,
//     @location(0) world_position: vec4f,
//     @location(1) world_normal: vec3<f32>,
//     @location(2) uv: vec2f,
//     #ifdef VERTEX_TANGENTS
//     @location(3) world_tangent: vec4f,
//     #endif
//     #ifdef VERTEX_COLORS
//     @location(4) color: vec4f,
//     #endif
// }

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> material_color: vec4f;

// vec2 p=(FC.xy*2.-r)/r.y,l,v=p*(1.-(l+=abs(.7-dot(p,p))))/.2;for(float i;i++<8.;
// o+=(sin(v.xyyx)+1.)*abs(v.x-v.y)*.2)v+=cos(v.yx*i+vec2(0,i)+t)/i+.7;
// o=tanh(exp(p.y*vec4(1,-1,-2,0))*exp(-4.*l.x)/o);

// 向量版 tanh
fn tanh_vec4(x: vec4f) -> vec4f {
    let e2x = exp(2.0 * x);
    return (e2x - vec4f(1.0)) / (e2x + vec4f(1.0));
}

// ...existing code...
@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4f {
    let t = globals.time;

    // Shadertoy 风格的 FC 与 r（此处用 uv 归一）
    let uv = mesh.uv;
    let FC = vec2f(uv.x, uv.y);
    let r  = vec2f(1.0, 1.0);

    var o: vec4f = vec4f(0.0);

    // vec2 p=(FC.xy*2.-r)/r.y;
    var p: vec2f = (FC * 2.0 - r) / r.y;

    // l 为标量累积项（原式里 l.x 被使用，等价于把 l 当标量）
    var l: f32 = 0.0;

    // v=p*(1.-(l+=abs(.7-dot(p,p))))/.2;
    l = l + abs(0.7 - dot(p, p));
    var v: vec2f = p * (1.0 - l) / 0.2;

    // for(float i;i++<8.; o+=(sin(v.xyyx)+1.)*abs(v.x-v.y)*.2) v+=cos(v.yx*i+vec2(0,i)+t)/i+.7;
    for (var k: i32 = 0; k < 8; k = k + 1) {
        let i = f32(k + 1); // i 从 1..8

        // 累积 v
        v = v + cos(vec2f(v.y, v.x) * i + vec2f(0.0, i) + vec2f(t, t)) / i + 0.7;

        // 累积 o
        let sx = sin(vec4f(v.x, v.y, v.y, v.x));
        o = o + (sx + vec4f(1.0)) * abs(v.x - v.y) * 0.2;
    }

    // o=tanh(exp(p.y*vec4(1,-1,-2,0))*exp(-4.*l.x)/o);
    // 由于 l 是标量，这里等价于使用 l
    let a = exp(p.y * vec4f(1.0, -1.0, -2.0, 0.0));
    let b = exp(vec4f(-4.0 * l));
    // 避免除零
    let o_safe = max(o, vec4f(1e-4));
    o = tanh_vec4((a * b) / o_safe);

    let c = clamp(o, vec4f(0.0), vec4f(1.0));
    return vec4f(c.xyz, 1.0);
}
// ...existing code...

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

// for(float i,z,d;i++<1e2;o+=(cos(z+t+vec4(6,1,2,3))+1.)/d){vec3 p=z*normalize(FC.rgb*2.-r.xyy);
// p.z-=t;for(d=1.;d<9.;d/=.7)p+=cos(p.yzx*d+z*.2)/d;
// z+=d=.02+.1*abs(3.-length(p.xy));}o=tanh(o/3e3);

// 向量版 tanh
fn tanh_vec4(x: vec4f) -> vec4f {
    let e2x = exp(2.0 * x);
    return (e2x - vec4f(1.0)) / (e2x + vec4f(1.0));
}

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4f {
    let t = globals.time;
    let uv = mesh.uv;
    let FC = vec3f(uv, 0.0);
    let r  = vec2f(1.0, 1.0);

    var o: vec4f = vec4f(0.0);
    var z: f32 = 0.0;
    var d: f32 = 0.0;

    // for(float i,z,d; i++<1e2; o+=(cos(z+t+vec4(6,1,2,3))+1.)/d) { ... }
    for (var iter: i32 = 0; iter < 100; iter = iter + 1) {
        // vec3 p = z*normalize(FC.rgb*2.-r.xyy); p.z -= t;
        var p: vec3f = z * normalize(FC * 2.0 - vec3f(r.x, r.y, r.y));
        p.z = p.z - t;

        // for(d=1.; d<9.; d/=.7) p += cos(p.yzx*d + z*.2)/d;
        d = 1.0;
        loop {
            if !(d < 9.0) { break; }
            p = p + cos(p.yzx * d + z * 0.2) / d;
            d = d / 0.7;
        }

        // z += d = .02 + .1*abs(3.-length(p.xy));
        d = 0.02 + 0.1 * abs(3.0 - length(p.xy));
        z = z + d;

        // 累加 o
        o = o + (cos(z + t + vec4f(6.0, 1.0, 2.0, 3.0)) + 1.0) / d;
    }

    // o = tanh(o/3e3);
    o = tanh_vec4(o / 3000.0);

    // 可使用 material_color 影响输出（若需要）
    return o;
}
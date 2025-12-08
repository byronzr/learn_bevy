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

// source from XorDev
// for(float z,d,i;i++<2e1;){vec3 p=z*normalize(FC.rgb*2.-r.xyx);
// p=vec3(atan(p.y/.2,p.x)*2.,p.z/3.,length(p.xy)-5.-z*.2);
// for(d=1.;d<7.;d++)p+=sin(p.yzx*d+t+.3*i)/d;
// z+=d=length(vec4(.4*cos(p)-.4,p.z));
// o+=(cos(p.x+i*.4+z+vec4(6,1,2,0))+1.)/d;}o=tanh(o*o/4e2);

// 向量版 tanh
fn tanh_vec4(x: vec4f) -> vec4f {
    let e2x = exp(2.0 * x);
    return (e2x - vec4f(1.0)) / (e2x + vec4f(1.0));
}

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4f {
    let t = globals.time;

    // 构造 Shadertoy 风格的 FC 与 r（此处用 uv 归一化版本）
    let uv = mesh.uv;
    let FC = vec3f(uv, 0.0);
    let r  = vec3f(1.0, 1.0, 1.0);

    var o: vec4f = vec4f(0.0);
    var z: f32 = 0.0;
    var d: f32 = 0.0;

    // for(float z,d,i; i++ < 2e1;)
    for (var k: i32 = 0; k < 20; k = k + 1) {
        let i = f32(k + 1); // 循环体内的 i 从 1..20

        // vec3 p = z * normalize(FC*2 - r.xyx);
        var p: vec3f = z * normalize(FC * 2.0 - r.xyx);

        // p = vec3(atan(p.y/.2, p.x)*2, p.z/3, length(p.xy)-5 - z*.2)
        p = vec3f(
            atan2(p.y / 0.2, p.x) * 2.0,
            p.z / 3.0,
            length(p.xy) - 5.0 - z * 0.2
        );

        // for (d=1; d<7; d++) p += sin(p.yzx*d + t + .3*i) / d;
        for (var j: i32 = 1; j < 7; j = j + 1) {
            let jf = f32(j);
            p = p + sin(p.yzx * jf + t + 0.3 * i) / jf;
        }

        // z += d = length(vec4(.4*cos(p) - .4, p.z));
        let c3 = 0.4 * cos(p) - vec3f(0.4);
        let tmp = length(vec4f(c3, p.z));
        d = tmp;
        z = z + d;

        // o += (cos(p.x + i*.4 + z + vec4(6,1,2,0)) + 1.) / d;
        let base = p.x + i * 0.4 + z;
        let v4   = vec4f(base, base, base, base) + vec4f(6.0, 1.0, 2.0, 0.0);
        o = o + (cos(v4) + vec4f(1.0)) / max(d, 1e-4);
    }

    // o = tanh(o*o/4e2);
    // hyperbolic tangent
    o = tanh_vec4((o * o) / 400.0);

    let c = clamp(o, vec4f(0.0), vec4f(1.0));
    return vec4f(c.xyz, 1.0);
}

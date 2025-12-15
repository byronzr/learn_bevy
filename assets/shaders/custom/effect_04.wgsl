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

// for(float z,d,i;i++<1e2;o+=(1.+cos(i*.7+t+vec4(6,1,2,0)))/d/i){vec3 p=z*normalize(FC.rgb*2.-r.xyx);p=vec3(atan(p.y,p.x)*2.,p.z/3.,length(p.xy)-6.);for(d=1.;d<9.;d++)p+=sin(p.yzx*d-t+.2*i)/d;z+=d=.2*length(vec4(.1*cos(p*3.)-.1,p.z));}o=tanh(o*o/9e2);

// 向量版 tanh
fn tanh_vec4(x: vec4f) -> vec4f {
    let e2x = exp(2.0 * x);
    return (e2x - vec4f(1.0)) / (e2x + vec4f(1.0));
}


@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4f {
    let t = globals.time;

    // Shadertoy 风格的 FC 与 r（此处用 uv 归一）
    let uv = mesh.uv;
    let FC = vec3f(uv,0.0);
    let r  = vec2f(1.0, 1.0);

    var o: vec4<f32> = vec4<f32>(0.0);
    var z: f32 = 0.0;
    var d: f32 = 0.0;
    var i: f32 = 0.0;

    // 迭代 100 次
    for (var iter: i32 = 0; iter < 100; iter = iter + 1) {
        i = f32(iter + 1);

        var p: vec3<f32> = z * normalize(FC.rgb * 2.0 - r.xyx);
        p = vec3<f32>(
            atan2(p.y, p.x) * 2.0,
            p.z / 3.0,
            length(p.xy) - 6.0
        );

        // 使用同一个 d 变量进行内部循环
        d = 1.0;
        for (; d < 9.0; d = d + 1.0) {
            p = p + sin(p.yzx * d - t + 0.2 * i) / d;
        }

        d = 0.2 * length(vec4<f32>(0.1 * cos(p * 3.0) - 0.1, p.z));
        z = z + d;

        o = o + (1.0 + cos(i * 0.7 + t + vec4<f32>(6.0, 1.0, 2.0, 0.0))) / d / i;
    }

    o = tanh_vec4((o * o) / 900.0);
    return o;
}


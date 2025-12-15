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

// from XorDev 
// source url: https://x.com/XorDev
// vec2 p=(FC.xy-r*.5)/r.y*mat2(8,-6,6,8),v;
// for(float i,f=3.+snoise2D(p+vec2(t*7.,0));
// i++<50.;
// o+=(cos(sin(i)*vec4(1,2,3,1))+1.)*exp(sin(i*i+t))/length(max(v,vec2(v.x*f*.02,v.y))))v=p+cos(i*i+(t+p.x*.1)*.03+i*vec2(11,9))*5.;
// o=tanh(pow(o/1e2,vec4(1.5)));



// 伪随机（哈希）
fn random2d(p: vec2f) -> f32 {
    return fract(sin(dot(p, vec2f(127.1, 311.7))) * 43758.5453123);
}



// 二维噪声函数
fn noise2(p: vec2f) -> f32 {
    let i = floor(p);
    let f = fract(p);
    let a = random2d(i);
    let b = random2d(i + vec2f(1.0, 0.0));
    let c = random2d(i + vec2f(0.0, 1.0));
    let d = random2d(i + vec2f(1.0, 1.0));
    let u = f * f * (3.0 - 2.0 * f);
    let nx0 = mix(a,b,u.x);
    let nx1 = mix(c,d,u.x);
    return mix(nx0, nx1, u.y);
}

// 向量版 tanh 近似
fn tanh_vec4(x: vec4f) -> vec4f {
    let e2x = exp(2.0 * x);
    return (e2x - vec4f(1.0)) / (e2x + vec4f(1.0));
}

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4f {
    // 时间
    let t = globals.time;

    // 当前（点）距离中心点的向量
    let centered = mesh.uv - vec2f(0.5, 0.5);

    // 缩放与旋转（控制尾焰方向）
    // @ NOTE: 涉及到矩阵变化的识知点
    let M = mat2x2<f32>(vec2f(8.0, -6.0), vec2f(6.0, 8.0));
    var p = M * centered;

    // 初始值(尾焰累积)
    var o: vec4f = vec4f(0.0);
    var v: vec2f = p;

    // 控制尾焰扰动效果
    let f = 3.0 + noise2(p + vec2f(t * 7.0, 0.0));
    

    // (流星？)数量
    var i: f32 = 0.0;
    loop {
        if (i >= 10.0) { break; }

        let phase = i * i + (t + p.x * 0.1) * 0.03;

        // 因子： 干涉纹理
        v = p + cos(vec2f(phase, phase) + i * vec2f(11.0, 9.0)) * 5.0;

        // 因子： 颜色倾向
        let col_scale = cos(sin(i) * vec4f(1.0, 2.0, 3.0, 1.0)) + vec4f(1.0);

        // 权重： 放大差异
        let weight = exp(sin(i * i + t));

        // 增加扰动的方向性
        let denom_vec = max(v, vec2f(v.x * f * 0.02, v.y));
        let denom = length(denom_vec);
        o = o + col_scale * weight / denom;
        i = i + 1.0;
    }

    // 提升暗部细节，抑制高亮溢出，形成柔和饱和的尾焰外观
    // hyperbolic tangent (双曲正切)
    o = tanh_vec4(pow(o / 100.0, vec4f(1.5)));

    // 最终颜色（alpha 固定为 1）
    let c = clamp(o, vec4f(0.0), vec4f(1.0));
    return vec4f(c.xyz, 1.0);
}
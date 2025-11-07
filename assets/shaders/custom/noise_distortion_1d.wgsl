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


// ! 这个函数说明了，
// ! 为什么即想随机又不想太过[抖动]，random 的随机确定性提供了一个可预见的能力
@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let base_line = 0.5;
    let max_offset = 0.2;

    // 这里对噪声密度进行了放大
    // 原始 uv 的区间在 0/1 之间,这使得噪声在生成时波动被限制在一个很小的区间
    let noise_scale = 10.0;

    // 当 UV 值用尽时，time 提供了持续的步进值，使得动画发生
    // noise 受到 random 的影响，返回的任然是 0~1 区间
    var noise_value = noise(mesh.uv.x * noise_scale + globals.time);

    // ! 在同一桢中，globals.time 是相同的，mesh.uv.x 在确定的情况下。
    // ! noise_value 返回的值是确定的，它的随机性，已经在 random 中被确定.
    // ! -- 在接下来的代码中 
    // ! base_line + noise_value, 在同 x 时提供了 [确定性] 
    // ! 而 y 值，确会因为趋势发生变化,这种 [确定性] 使得我们不需要去[问寻]周边的像素点状态
    // ! 将原[计算管线](compute_line)要作的状态查询，[综合]到了[fragment]中

    // 因为 base_line 与 噪声值配合判断，
    // 显然太容易超出 1.0 了，所以适当的缩小噪音范围，
    // 使得每个 uv.y 值都有机会
    noise_value = noise_value * max_offset;

    // step 返回对应的 0/1 浮点结果
    // step(edge,v), 可以把 step 理解为 is_inner 比较方便理解
    let up_down = step(base_line+noise_value,mesh.uv.y);

    // mix 就是 lerp, 
    // 但在这里等于 if 因为 up_down 只会是 0与1
    let color = mix(vec4(1.0),material_color,up_down);
    return color;
}


// 无论如何变换 fract 都使得最后的随机值是小于 1.0 的
fn random(x:f32)->f32 {
    return fract(sin(x*1000.0)*5323.1323);
}

// 虽然参数 x 由 uv.x 与 globals.time 等众多因素够成会超过 1.0
// 但是 经过 random 函数后，再次回来了 0.0～1.0的区间
fn noise(x:f32)->f32 {
    // 整数部分
    let part_integer = floor(x);
    // 小数部分
    let part_fraction = fract(x);

    // 整数区间 (随机)
    let integer_start = random(part_integer);
    let integer_end = random(part_integer+1.0);

    // (小数部分)缓动曲线
    // For scalar T, the result is t * t * (3.0 - 2.0 * t),
    // where t = clamp((x - edge0) / (edge1 - edge0), 0.0, 1.0).
    // smoothstep 与 三次缓动 等价
    // let curve = part_fraction * part_fraction * (3.0 - 2.0*part_fraction);
    let curve = smoothstep(0.0,1.0,part_fraction);

    // 插值
    // mix(a, b, t) = a + (b - a) * t = (1 - t) * a + t * b
    return mix(integer_start,integer_end,curve);
}

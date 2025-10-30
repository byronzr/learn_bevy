
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
    // 直接观察噪声采样图可以看到一定的重复性,但并不平滑
    // let color = random_vec2(mesh.uv);
    // return vec4f(vec3f(color),1.0);

    // 平滑过后的噪声图
    // let color = noise_vec2(mesh.uv*100 );
    // return vec4f(vec3f(color),1.0);

    // 使纹理更加丰富(更像云层)
    // let color = noise_multiple(mesh.uv);
    // return vec4f(vec3f(color),1.0);

    // 画(正圆)圆
    // let vn = mesh.uv - vec2(0.5,0.5);
    // let vr = length(vn);
    // let radius = 0.4;
    // let alpha = step(radius,vr);
    // let bg = vec4f(1.0);
    // return mix(material_color,bg,alpha);


    // 加噪声
    let vn = mesh.uv - vec2(0.5,0.5);
    let vr = length(vn);
    let radius = 0.2;
    
    // 变化率（柔捏力度）
    let change_rate_factor = 0.5;
    // 柔捏速度（频率）
    let speed_factor = 1.0;
    // 最后整体(大小)的控制
    let radius_buffer = 0.1;

    // wgsl 的坐标系使得我们已经使用 vr 求出了距离圆心的距离
    // vr 代替 dis
    // let dis = distance(mesh.uv,vec2(0.5));

    let dir = normalize(mesh.uv - vec2f(0.5));

    // 如果使用 multiple (类似火焰的碎片效果)
    var noise_value = noise_multiple(mesh.uv + dir * change_rate_factor + vec2f(globals.time)*speed_factor);

    // 注意这里使用的是 noise_vec2 (类似面团柔捏)
    // var noise_value = noise_vec2(mesh.uv + dir * change_rate_factor + vec2f(globals.time)*speed_factor);
    noise_value *= radius_buffer;
    
    // let color = step(dis,radius+noise_value);
    // vr 带替 dis
    let color = step(vr,radius+noise_value);
    let bg = vec4f(1.0);
    
    
    return mix(bg,material_color,color);

}

// fn random(x:f32)->f32 {
//     return fract(sin(x*1000.0)*5323.1323);
// }
// 2维的伪随机
fn random_vec2(v:vec2f)->f32 {
    let m1 = 14.7258;
    let m2 = 36.9323;
    let m3 = 5323.1323;
    return fract(sin(dot(v.xy,vec2(m1,m2)))*m3);
}

// fn noise(x:f32)->f32 {
//     let i = floor(x);
//     let f = fract(x);
//     let a = random(i);
//     let b = random(i+1.0);
//     let u = f * f * (3.0 - 2.0*f);
//     return mix(a,b,u);
// }
fn noise_vec2(v:vec2f)->f32 {
    
    // 整数部分
    let i = floor(v);
    // 小数部分
    let f = fract(v);


    // [四点]随机,
    // 因为 random_vec2 是伪随机，v 值的随机返回是确定的
    // 所以此处可以 [预先] 获得采样结果进行差值
    let bl = random_vec2(i);
    let br = random_vec2(i+vec2f(1.0,0.0));
    let tl = random_vec2(i+vec2f(0.0,1.0));
    let tr = random_vec2(i+vec2f(1.0,1.0));

    // 缓动
    let u = f * f * (3.0 - 2.0*f);

    // 以x轴进行插值
    let b = mix(bl,br,u.x);
    let t = mix(tl,tr,u.x);
    // 以y轴进行插值
    let ret = mix(b,t,u.y);

    return ret;
}

fn noise_multiple(v:vec2f)->f32 {
    var m = noise_vec2(v * 8.0);
    m += noise_vec2(v * 16.0) * 0.5;
    m += noise_vec2(v * 32.0) * 0.25;
    m += noise_vec2(v * 64.0) * 0.125;
    //m += noise_vec2(v * 128.0) * 0.0625;
    // 不断的叠加后 m的区间趋近于 2，所以除以 2
    m /= 2.0;
    return m;
}

#import bevy_sprite::{
    mesh2d_view_bindings::globals,
    mesh2d_vertex_output::VertexOutput,
}

// search => #define_import_path bevy_render::globals
// path => /Users/byronzr/rProjects/bevy/crates/bevy_render/src/globals.wgsl
// search => #define_import_path bevy_sprite::mesh2d_vertex_output     
// path=> /Users/byronzr/rProjects/bevy/crates/bevy_sprite_render/src/mesh2d/mesh2d_vertex_output.wgsl

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> material_color: vec4<f32>;


// ! 较为通用抗锯齿函数
fn aa_width(d: f32) -> f32 {
    // 基于导数估算边缘宽度用于 smoothstep 抗锯齿
    let w = abs(dpdx(d)) + abs(dpdy(d));
    return max(w, 1e-4);
}

// ! 更通用的区间重映射 [a,b] -> [c,d]（不夹取）
fn remap(x: f32, a: f32, b: f32, c: f32, d: f32) -> f32 {
    let t = (x - a) / (b - a);
    return c + t * (d - c); // 需要夹取可对 t 使用 clamp
}

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {

    // wgsl 没有默认的 PI 常量
    const PI = 3.141592653589793;
    // var 变量需要调整颜色与
    var color = material_color;

    /// * vn 与 vr 作为一个输入判断的条件变量来决定颜色的走向
    /// * 实际上图型(shape)还是依赖于初始的定义（radius/innser_radius/center）

    /// ! --- 第一个外圆 --- 
    // 距离中心的向量
    let vn = mesh.uv - vec2(0.5,0.5);
    // 当前向量长度
    let vr = length(vn);
    // 需要控制的半径阈值
    // 中心点是 （0.5，0.5）半径最大可视值为 0.5
    let radius = 0.4;
    // 抗锯齿的值
    let aa = aa_width(vr);
    // 外部着色,内部透明
    let outer_corlor = smoothstep(radius-aa,radius+aa,vr);

    // * 如果 outer_color 是有颜色的，说明在外圆范围之外

    /// ! --- 第二个内圆 ---
    // 内圆的半径相对于外圆进行缩减
    let inner_radius = radius - 0.2;
    let inner_corlor = smoothstep(inner_radius-aa,inner_radius+aa,vr);


    /// ! 抠像蒙版
    let mask = inner_corlor-outer_corlor;
    // * 如果 inner_color 有颜色，说明在内圆之外
    // 1. 当 uv 在外圆之外时，outer = 1 / inner = 1 
    //      mask = nner - outer = 0 
    // 2. 当 uv 在 [圆环内部时] outer = 0 / inner = 1
    //      mask = inner - outer = 1
    // 3. 当 uv 在 [内圆之中时] outer = 0 / inner = 0
    //      mask = inner - outer = 0
    // * 完成圆环的绘制
    

    // ! 环形渐变
    let deg = 0.;
    // 起始向量平行向右
    let upvec = vec2(cos(radians(deg)),sin(radians(deg)));
    // 起始向量垂直向下
    // let upvec = vec2(sin(radians(deg)),cos(radians(deg)));
    let nv1 = normalize(vn);
    // * 获得一个无法区别补角的角度值
    // * theta 现在是弧度值，所以不透明通道是溢出的
    // * 0 -> PI
    var theta = acos(dot(nv1,upvec)); 
    

    // * 将 0 -> PI 扩展为 -PI -> PI
    let n = cross(vec3(upvec,0),vec3(nv1,0));
    theta *=sign(n.z);

    // * 对 theta 进行映射
    // * 从视觉上来看，未映射前是
    // * - 下部为右黑（0.), 左白（1.）
    // * - 上部为右黑（0.)，左黑（-1.）
    // * 结果
    // * - 右边为灰（0.5），左上为白（1.），左下为黑（0.）
    theta = remap(theta,-PI,PI,0.,1.);

    // ! 遮罩与渐变的叠加
    // * theta = f32 .rgb = vec3f
    // * color.rgb *= theta; // 单独相乘是不可行的
    // * 用 vec4f 进行重构变量可行
    // * 注意 Bevy Material2d 的实现中，alpha_mode 需要是
    // * AlphaMode2d::Blend 才能使透明生效
    color = vec4f(color.rgb*theta,mask);

    // ! 如果需要对 360 度进圆弧起始可控
    // let annulus_start = 0.1;
    // let annulus_end = 0.7;
    // @ 同样 smoothstep 是为了保护切线平滑
    // let start_theta = smoothstep(annulus_start,annulus_start+0.001,theta);
    // let end_theta = smoothstep(annulus_end,annulus_end+0.001,theta);
    // * 再次与透明度相乘就可以将圆环变更为有起始与结束的圆弧
    // color.a *= start_theta - end_theta;

    
    // ! 分段
    let count = 10;
    let inc = 360./f32(count);
    var line:f32;
    for (var i:i32=0;i<=count;i++) {
        let deg = inc * f32(i);
        line += segment_line(vn,deg,0.01);
    }

    // 将遮罩取反相乘就减出去了边距
    // 此时 shape 的基础型态已经完成,但是最左侧的测变在同一个片段中开始与结束，视觉上观感不好
    // 这时候再回顾环型渐变的参考向量为什么要使用 deg 角度值进行创建的原因了。
    // 通过角度(deg)可以后期与分段数(count)进行调整
    color.a *= 1.-line;

    return color;
    //return color;
}

// ! 分段(放射不等距切割)
fn scatter_line(count:i32,theta:f32) -> f32{
    // * 当 0 -> 1 被放大 10 倍时,那么 0 -> 0.1 就被映射成了 0 -> 1
    // * 当前 theta * 分段（放大倍数） 的小数部分，都是一个新的 0 -> 1 区间
    let mul = fract(theta*f32(count));
    // * 分段剪多少边距
    let start = 0.1;
    let offset = 0.001;
    // * 一个分段图形的起始与结束位子的剪切
    // * 起始位置
    let start_mul = smoothstep(start,start+offset,mul);
    // * 结束位置（反剪）
    let end_mul = smoothstep(start,start+offset,1.-mul);
    return start_mul*end_mul;
}


// ! 分段（平行等距切割）
// * 利用投影绘制出一个从中心点开始向上的平行四边形
// * （重复）并旋转，然后(组合)相减
fn segment_line(vn:vec2f, deg:f32, width:f32) -> f32 {
    // upvec 可以是以中心点为起点任意角度的直线
    // vn 是当前向量，投影在 v 上的值在upvec不变的情况下是固定的。
    // 这样就绘制了一个从中心开始，从左(0.)向右(1.)的渐变值,
    // 而从中心开始从右（0.）向左(-1.)的渐变值是溢出的，所以取 abs
    let upvec = vec2(cos(radians(deg)),sin(radians(deg)));
    let shadow = abs(dot(upvec,vn));
    // 使用 step 将渐变转换为实体
    // ! 根据计算得到的结果往往是一个负型（想要区域实际的透明的）
    // ! 通常会用 1. - line 的方式
    // ! 但更简便的方式是将 step 中的参数调转
    //var line = step(0.01,v);
    let line = step(shadow,width);
    let side = signside(upvec,vn);
    return line*side;
}

// ! 以v2作为参考，获得 v1 所在 side 封装函数
fn signside(v2:vec2f,v1:vec2f) -> f32{
    let upvec = normalize(v2);
    let nv1 = normalize(v1);
    let n = cross(vec3(upvec,0),vec3(nv1,0));
    return clamp(1 * sign(n.z),0,1);
}

// @ step(edge:T, x:T) -> T
// Returns 1.0 if edge ≤ x, and 0.0 otherwise. Component-wise when T is a vector.
// 当 x >= edge 时返回 1，否则 0

// @ smoothstep(edge0: T, edge1: T, x: T) -> T
// step 会有很强的锯感，而 smooth 可以在边缘增加阈值确定到返回一个平滑的值
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

    

    // 需要切分的数量
    let count = 10;
    // 每次角度的步进值
    let degree_per_second = 360./f32(count);
    // wgsl 没有默认的 PI 常量
    const PI = 3.141592653589793;
    // var 变量需要调整颜色
    var color = material_color;
    // divide
    let divide_width = 0.02;
    // 最小的小数 [1e-6] 必须连写-
    let epsilon = 1e-6;


    let c = f32(count);
    // 每个段的持续秒数（你设定的步长）
    let step_seconds: f32 = 1.0; 
    // 运行时长总周期
    let cycle_seconds = c * step_seconds; 

    // ! 同一相位，小数点精度在这种纯数学运算的图形中，误差被放大得很严重
    // ! 不要带着传统思维认为多次 360. / 60. 永远是同一个值
    // phase ∈ [0,1)
    let phase = fract(globals.time / cycle_seconds);

    /// ! --- 第一个外圆 ---
    // 距离中心的向量 
    // 因为 (0,0),在单位向量的左上角,这与其它的 shader 语言不同 
    let center = vec2(0.5,0.5);
    let vn = mesh.uv - center;
    // 当前向量长度
    let vr = length(vn);
    // 需要控制的半径阈值
    // 中心点是 （0.5，0.5）半径最大可视值为 0.3
    let radius = 0.3;
    // 抗锯齿的值
    let aa = aa_width(vr);
    // 外部着色,内部透明
    let outer_color = smoothstep(radius-aa,radius+aa,vr);

    // 如果 outer_color 是有颜色的，说明在外圆范围之外
    // outer_color 实际通过 smoothstep 只会是 0.0 / 1.0
    // 所以在后期可以用加减法进行（抠图）

    /// ! --- 第二个内圆 ---
    // 内圆的半径相对于外圆进行缩减
    let inner_radius = radius - 0.2;
    let inner_color = smoothstep(inner_radius-aa,inner_radius+aa,vr);

    /// ! -- 高起扇区 -- 
    let dynamic_height = remap(globals.time % step_seconds,0.,step_seconds - epsilon,0.,0.1);
    let sector_radius = radius + dynamic_height;
    let sector_color = smoothstep(sector_radius-aa,sector_radius+aa,vr);

    // ! 常规遮罩与扇区庶罩
    var normal_mask = inner_color - outer_color;
    var sector_mask = inner_color - sector_color;


    

    // ! 环形渐变(动画)
    // 使用角度生成起始参考边的好处，就在于后期角度的调整
    // (使用同一相位计算) 每秒度数

    // ! 旋转渐变
    //var deg = phase * 360.;

    // ! 静态渐变
    var deg = 0.;

    // -- 关于 upvec -- 的推导
    // cos(theta) = x/r;
    // sin(theta) = y/r;
    // -- 
    // let point = (px,py);
    // let px = r * cos(theta);
    // let py = r * sin(theta);
    // 因为条件是单位向量斜边(Hypotenuse) r = 1
    // let px = cos(theta);
    // let py = sin(theta);
    // let point = vec2(cos(theta),sin(theta));
    // 因为大多数三角函数的参数是弧度值
    // let point = vec2(cos(radians(theta)),sin(theta));

    // 用于与当前 uv 构成平角的参考向量
    // 起始向量平行向右(顺时针)
    let upvec = vec2(cos(radians(deg)),sin(radians(deg)));

    // let upvec0 = vec2(cos(radians(0.0)),   sin(radians(0.0)));   // (1,0)  右（数学）
    // let upvec90 = vec2(cos(radians(90.0)),  sin(radians(90.0)));  // (0,1)  上（数学）,下(屏幕)
    // let upvec180 = vec2(cos(radians(180.0)), sin(radians(180.0))); // (-1,0) 左（数学）

    // 将 vn 转换成一个单位向量 (因为)
    // nv1 => vn / vr 
    //     => (x/vr, y/vr)
    let nv1 = normalize(vn);

    // 获得一个无法区别 [补角] 的角度值(永远小于180度的夹角)
    // theta 现在是弧度值，所以不透明通道是溢出的
    // 0 -> PI
    var theta = acos(dot(nv1,upvec));

    
    // -- 关于坐标系 --
    // 右手比成一把枪，中指申直,中指为X，拇指为Y，拾指为Z
    // 这把枪指向自已（Bevy）
    // 这把枪拽拽的（逆时针旋转180度）指向别人（屏幕坐标）
    // +Z,就是指向的方向
    // 当 cross 求值时，Z的正负值决定了你观察 xy 这个平面的视角

    // -- cross 求值-- 
    // 当左上角(0.0,0.0)A，旋转至(1.0,0.5)B [屏幕坐标]转换成距离中心的向量
    // 左上角(-0.5,-0.5)A,旋转至(0.5,0.0)B, 从 +Z 方向去看是[逆时针旋转]
    // 左下角(-0.5,0.5)A,,旋转至(0.5,0.0)B, 从 -Z 方向去看是[逆时针旋转]

    // -- cross 的一个比较不正确的理解 --
    // 从z轴的哪个方向去观察平面xy中，A旋转到B的方向是[逆时针]

    // -- cross 较专业的解释 --
    // cross 的 z 符号并不“决定”你从哪侧观察，而是提供了一个带方向的法向量；
    // 你以哪侧去“看”这个法向量，就能解释 a 到 b 的旋转是顺时针还是逆时针。




    // 使用 叉乘(cross) 基于 参考向量(upvec)B 求 当前归一化向量(nv1)A 的相对位置
    // cross 函数本身遵循右手坐标系法则(与引擎和WGSL本身无关)
    // 将 0 -> PI 扩展为 -PI -> PI
    // 所以在:
    // 上半部分从左到右旋转（顺时针）所以Z是负值, ( -PI~0 )
    // 下半部分从左到右旋转（逆时针）所以Z是正值，( PI~0 )
    let n = cross(vec3(upvec,0),vec3(nv1,0));
    theta *=sign(n.z);

    // 对 theta 进行映射 
    // 上部左中最黑，下部左中最白
    // * 这里，要追述一下 deg 与 theta 的逻辑关系 -- 
    // * deg 使得 upvec 这条参考线不断的进行旋转，随之使得渐变映射发生改变
    // * upvec 是 [中性] 灰的起点，也就是黑与白反方向
    // * 在 [数学坐标系] （+Y向上）中应当向 [逆时针] 旋转，
    // * 但在 [屏幕/像素坐标系] （+Y向下） 中就变成了 [顺时针] 旋转
    // * theta 的值 描述的是 从（-X）的单位向量 [旋转] 360 度的区间
    theta = remap(theta,-PI,PI,0.,1.);


    // ! 高亮扇区
    let mask = highlight_sector(phase,theta,normal_mask,sector_mask,c);

    // ! 常用的渐变灰阶权重，让视觉中性灰居中
    let k: f32 = 0.5;                       
    var w = pow(clamp(theta, 0.0, 1.0), k);

    // 与视神经的权重是有区别的,视神经的明度权重目标是彩色（rgb）
    // w = vec3<f32>(0.2126, 0.7152, 0.0722);

    // 没有权重的效果(中性灰感觉会被压缩得很厉害)
    // w = 1.0;

    // ! 遮罩与渐变的叠加
    // 注意 Bevy Material2d 的实现中，alpha_mode 需要是 AlphaMode2d::Blend 才能使透明生效
    color = vec4f(color.rgb * theta * w,mask);
    

    // ! -- 如果想从[圆环]变成[圆弧] --
    // let annulus_start = 0.1;
    // let annulus_end = 0.7;
    // // 同样 smoothstep 是为了保护切线平滑
    // let start_theta = smoothstep(annulus_start,annulus_start+0.001,theta);
    // let end_theta = smoothstep(annulus_end,annulus_end+0.001,theta);
    // // 再次与透明度相乘就可以将圆环变更为有起始与结束的圆弧
    // color.a *= start_theta - end_theta;


    var line:f32;   
    // ! 等距分段
    for (var i:i32=0;i<=count;i++) {
        let deg = degree_per_second * f32(i);
        line += segment_line(vn,deg,divide_width);
    }

    // ! 放射分段
    //line = 1.0 - scatter_line(c,theta);

    // // 将遮罩取反相乘就减出去了边距
    // // 此时 shape 的基础型态已经完成,但是最左侧的测变在同一个片段中开始与结束，视觉上观感不好
    // // 这时候再回顾环型渐变的参考向量为什么要使用 deg 角度值进行创建的原因了。
    // // 通过角度(deg)可以后期与分段数(count)进行调整
    color.a *= 1.-line;

    return color;
    //return color;
}

// ! 高亮扇区
fn highlight_sector(phase:f32,theta:f32,normal_mask:f32,sector_mask:f32,c:f32) -> f32{

    // ! 偏移中，为什么不增加 [间距](divide_width) 的偏移量？
    // ! 因为最终会以 遮罩 的方式减去，并没有存在真正的 [绘制] 扇区，完全是在作面的加减法
    let offset = (1./c) / 2.;

    // 最小的小数 [1e-6] 必须连写-
    let epsilon = 1e-6;

    // 时间进度
    // 0..count-1
    var time_index = i32(floor(phase * (c-epsilon))); 
    // 扇区进度

    // (bug) 未偏移
    // let sector_index = i32(floor(theta * (c-epsilon)));

    // (bug) 左一半
    let sector_index = i32(floor((theta + offset) * (c-epsilon)));

    // (fixed) 纯粹给自已加难度
    // let fixed = false;
    let fixed = (time_index == 0 && (sector_index == 0 || theta+offset>1.));
    
    // -- 并不推荐使用 if 除非代码块中确实能够得到性能提升，不然（无分支的） select / mix 是最优解
    // if  time_index==sector_index  {
    //     mask = inner_color - convex_corlor;
    // }

    // -- 1.select(B,A,Cond) 
    // select, 要求条件为 bool
    let mask = select(normal_mask,sector_mask,time_index == sector_index || fixed);

    // -- 2.mix(B,A,Cond(0/1))
    // mix, 进行无分支时，要求 cond 限定在  0/1 之间，否则产生插值
    //let mask = mix(normal_mask,sector_mask,current);

    return mask;
}

// ! 分段(放射不等距切割)
fn scatter_line(c:f32,theta:f32) -> f32{
    // 0. -> 0.99~
    let part = fract(theta*c);

    // 间距起始
    let start = 1./c;
    let end =  start + 0.001;

    // head_mask 是一个“低通/阈值”型遮罩
    // part ≤ start 时返回 0
    // part ≥ end 时返回 1
    // 在 [start, end] 内从 0 平滑过渡到 1
    // - 例 -
    // +X 向量在旋转时 theta 值在达到 start 以前，都是 0 
    // 在[描边]时平滑过度
    // 在超过 end 以后为 1
    // 于是看到 [大约] 0.-> 0.1 区间的中空（upvec顺时针） 
    let head_mask = smoothstep(start,end,part);

    // 这里实现了 [遮罩的最后一半] 大约也是中空的，但是从[尾部开始计算]
    // 当 part = 0.90+ 时 tail_mask 就进入了一个 [反向] 检测试的过程
    let tail = 1. - part;
    let tail_mask = smoothstep(start,end,tail);

    // 可以看看只有一半的效果
    // let tail_mask = 1.0;
    
    // 两个中空的 [头尾] 结合在了一起
    // 这个间距实际是[别人的尾]和[自已的头]
    return head_mask*tail_mask;
}


// ! 分段（平行等距切割）
// 利用投影绘制出一个从中心点开始向上的平行四边形
// (重复)并旋转，然后(组合)相减
// 如果将圆用矩型切割成 2 份，那么实际上需要画四个矩型(围绕中心点)
fn segment_line(vn:vec2f, deg:f32, width:f32) -> f32 {
    // 需要垂直的参考线
    let upvec = vec2(cos(radians(deg)),sin(radians(deg)));

    // 投影
    // 本质上是计算向量（vn）的 [垂足] 距离中心点的距离,
    // abs 的使用原因是 deg 到左边两个象限时距离中心点是[负的]
    let shadow_length = abs(dot(upvec,vn));

    // 所以当第一次运行时，实际上是在[右上]（数学）象限,[绘制] 矩形
    // 如果 shadown_length > width / 2 说明超出绘制范围

    // 根据计算得到的结果往往是一个 [负型]（想要区域实际的透明的）
    // 使用 step 将渐变转换为实体
    // 通常会用 1. - line 的方式
    // var line = 1. - step(width/2., shadow_length);
    
    // 但更简便的方式是将 step 中的参数调转
    let line = step(shadow_length,width/2.);

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

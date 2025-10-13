// // The Vertex output of the default vertex shader for the Ui Material pipeline.
// struct UiVertexOutput {
//     @location(0) uv: vec2<f32>,
//     ! The size of the borders in UV space. Order is Left, Right, Top, Bottom.
//     @location(1) border_widths: vec4<f32>,
//     ! The size of the borders in pixels. Order is top left, top right, bottom right, bottom left.
//     @location(2) border_radius: vec4<f32>,
//     ! The size of the node in pixels. Order is width, height.
//     @location(3) @interpolate(flat) size: vec2<f32>,
//     @builtin(position) position: vec4<f32>,
// };


// Draws a progress bar with properties defined in CustomUiMaterial
#import bevy_ui::ui_vertex_output::UiVertexOutput

// ! UI使用的group居然是在第1组
@group(1) @binding(0) var<uniform> color: vec4<f32>;
@group(1) @binding(1) var<uniform> slider: vec4<f32>;
@group(1) @binding(2) var material_color_texture: texture_2d<f32>;
@group(1) @binding(3) var material_color_sampler: sampler;
@group(1) @binding(4) var<uniform> border_color: vec4<f32>;


@fragment
fn fragment(in: UiVertexOutput) -> @location(0) vec4<f32> {
    let output_color = textureSample(material_color_texture, material_color_sampler, in.uv) * color;

    // half size of the UI node
    let half_size = 0.5 * in.size;

    // position relative to the center of the UI node
    // UV 是 NTC (Normalized Texture Coordinates) 坐标系
    // ! 当前渲染像素相对于于 UI 节点中心的偏移(从NTC转换到像素坐标系)
    let p = in.uv * in.size - half_size;

    // ! 验证 select
    // var b = vec2(0.0,0.0);
    // if (0.<p.x) {
    //     b.x = in.border_widths.z; 
    // }else{
    //     b.x = in.border_widths.x;
    // }

    // if (0. <p.y){
    //     b.y = in.border_widths.w;
    // }else{
    //     b.y = in.border_widths.y;
    // }
    // thickness of the border closest to the current position
    // p 在第一象限时,x值为正,y值为负(+x,-y),b值为 (z,y) => (上,右)
    // p 在第二象限时,x值为负,y值为负(-x,-y),b值为 (x,y) => (左,上)
    // p 在第三象限时,x值为负,y值为正(-x,+y),b值为 (x,w) => (左,下)
    // p 在第四象限时,x值为正,y值为正(+x,+y),b值为 (z,w) => (上,下)
    let b = vec2(
        // ! cond ? b : a
        // ! 如果 p.x 为负数,说明当前像素在左边,返回 x(LEFT),反之返回 z(TOP)
        select(in.border_widths.x, in.border_widths.z, 0. < p.x),
        // ! 如果 p.y 为负数,说明当前像素在上边,返回 y(RIGHT),反之返回 w(BOTTOM)
        select(in.border_widths.y, in.border_widths.w, 0. < p.y)
    );

    // select radius for the nearest corner
    // ! xy = (上左,上右) wz = (下左,下右)
    let rs = select(in.border_radius.xy, in.border_radius.wz, 0.0 < p.y);
    let radius = select(rs.x, rs.y, 0.0 < p.x);

    // distance along each axis from the corner
    // ! 计算当前像素到边框的距离
    let d = half_size - abs(p);


    // ! 绘制边框
    // ! 如果当前像素到边框的距离小于边框宽度,则
    // ! 说明当前像素在边框内,返回边框颜色(进入)
    // if the distance to the edge from the current position on any axis 
    // is less than the border width on that axis then the position is within 
    // the border and we return the border color
    if d.x < b.x || d.y < b.y {
        // ! 如果当前像素到边框的距离小于圆角半径,则说明当前像素在圆角内
        // determine if the point is inside the curved corner and return the corresponding color
        let q = radius - d;
        
        if radius < min(max(q.x, q.y), 0.0) + length(vec2(max(q.x, 0.0), max(q.y, 0.0))) {
            // ! 圆角外(透明)
            return vec4(0.0);
        } else {
            // ! 圆角内(边框颜色)
            return border_color;
        }
    }

    // sample the texture at this position if it's to the left of the slider value
    // otherwise return a fully transparent color
    // ! 如果当前像素在滑块左边,则返回纹理颜色(正片叠底 multiply/调制 modulate)
    // @ 其实接管 Node 绘制最主要的部份就在此,之前的都是绘制边框与圆角(可以硬抄)
    if in.uv.x < slider.x {
        let output_color = textureSample(material_color_texture, material_color_sampler, in.uv) * color;
        return output_color;
    } else {
        return vec4(0.0);
    }
}


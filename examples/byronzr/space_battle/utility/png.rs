use std::borrow::Cow;

use bevy::asset::RenderAssetUsages;
use bevy::prelude::*;
use bevy::render::mesh::{Indices, Mesh, PrimitiveTopology, VertexAttributeValues};
use bevy_image;
use contour;
use image::ImageReader;
use lyon::math::{Point, point};
use lyon::path::Path;
use lyon::tessellation::{BuffersBuilder, FillOptions, FillTessellator, FillVertex, VertexBuffers};

// 通过加载 PNG 绘制 Mesh
pub fn load<'a>(
    file_path: impl Into<Cow<'a, str>>,
    asset_server: &mut AssetServer,
) -> Result<(Mesh, Handle<bevy_image::Image>, Vec<Vec2>)> {
    let file_path = file_path.into();
    // 1. 加载纹理
    let texture_handle: Handle<bevy_image::Image> = asset_server.load(format!("{}", file_path));

    // 实际加载图片地址(服务于ImageReader)
    let file_asset_path = format!("assets/{}", file_path);
    // 加载图片
    let img = ImageReader::open(file_asset_path)?.decode()?.into_rgba8();
    let (width, height) = img.dimensions();
    // 图片两极化(透明与不透明)
    let mut mask = vec![vec![0f32; width as usize]; height as usize];
    for x in 0..width {
        for y in 0..height {
            let pixel = img.get_pixel(x, y);
            if pixel[3] > 0 {
                // alpha > 0
                mask[y as usize][x as usize] = 1.0;
            }
        }
    }
    // 转换成一维数组
    let mut mask = mask
        .into_iter()
        .flat_map(|v| {
            let mut nv = v.clone();
            nv.reverse();
            nv.into_iter()
        })
        .collect::<Vec<f32>>();
    mask.reverse(); // 反转,因为图片是从左下角开始的

    // 找到轮廓,并使 mesh 居中
    let contour = contour::ContourBuilder::new(width as usize, height as usize, false)
        .x_origin(-((width / 2) as f32))
        .y_origin(-((height / 2) as f32));

    let res = contour.lines(&mask, &[0.1])?;
    //println!("contour: {:?}", res[0].to_geojson());

    // 三角化
    let mut first = true;
    let mut builder = Path::builder();

    for line in res[0].geometry() {
        for p in line.points() {
            if first {
                builder.begin(point(p.x(), p.y()));
                first = false;
            } else {
                builder.line_to(point(p.x(), p.y()));
            }
        }
    }
    builder.end(true);
    let path = builder.build();
    //println!("path: {:?}", path);

    // 2. 创建 tessellator 和存储三角形的缓冲区
    let mut tessellator = FillTessellator::new();
    let mut geometry: VertexBuffers<Point, u16> = VertexBuffers::new();

    // 3. 执行三角化
    tessellator.tessellate_path(
        &path,
        &FillOptions::default(),
        &mut BuffersBuilder::new(&mut geometry, |vertex: FillVertex| vertex.position()),
    )?;

    // 4. 输出三角形顶点和索引
    let vertices = VertexAttributeValues::Float32x3(
        geometry
            .vertices
            .iter()
            .map(|v| [v.x, v.y, 0.])
            .collect::<Vec<[f32; 3]>>(),
    );

    let vertices_2d = geometry
        .vertices
        .iter()
        .map(|v| Vec2::new(v.x, v.y))
        .collect::<Vec<Vec2>>();

    let indices = Indices::U16(
        geometry
            .indices
            .iter()
            .map(|v| *v as u16)
            .collect::<Vec<u16>>(),
    );

    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices); // Vec<[f32; 3]>
    mesh.insert_indices(indices);

    Ok((mesh, texture_handle, vertices_2d))
}

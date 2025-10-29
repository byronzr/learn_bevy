# 个人笔记

一步步，迈向游戏国度.集成个人零游戏开发经验的探索过程,在没有完整策案前,会不断累游戏场景,应用(桌面)场景,随着版本迭代,可能会出现"造轮子"的案例.

# 官方用例 
请查看官方用例 [bevy](https://bevyengine.org/),中文注释的代码 [高级伴读](https://github.com/byronzr/learn_bevy/tree/main/examples),(从 0.15 开始的注释,如果没有出现 panic 将不会主动优化代码,仅作来理解使用)

# Shader(WGSL)
配合的 [learn-wgpu-zh](https://jinleili.github.io/learn-wgpu-zh/) 教程的流程梳理分享 [keynote](https://github.com/byronzr/learn_bevy/tree/main/keynote/wgsl)

* ### 基础
    * [01] `wgsl` WebGPU 接管绘制
    * [02] `wgsl` pipeline 渲染管线
    * [03] `wgsl` vertex 顶点缓冲与索引
    * [04] `wgsl` image 图像与绑定组
    * [05] `wgsl` uniform 统一缓存
    * [06] `wgsl` instance 实例渐进 `Bevy` / [shader_material_2d](https://github.com/bevyengine/bevy/blob/main/examples/shader/shader_material_2d.rs) / [animate_shader](https://github.com/bevyengine/bevy/blob/main/examples/shader/animate_shader.rs)

* ### 扩展
    * [07] `Bevy` custom_vertex bevy中增加vertex [custom_vertex_attribute](https://github.com/bevyengine/bevy/blob/main/examples/shader/custom_vertex_attribute.rs)
    * [08] `Bevy` storage 存储缓冲区 [storage_buffer](https://github.com/bevyengine/bevy/blob/main/examples/shader/storage_buffer.rs)
    * [09] `Bevy` shader_defs 宏定义 [shader_defs](https://github.com/bevyengine/bevy/blob/main/examples/shader/shader_defs.rs)

* ### 实践
    * [10] `wgsl` 高斯模糊 计算管线(compute pipeline)
    * [11] `Bevy` 生命游戏 [game_of_life](https://github.com/bevyengine/bevy/blob/main/examples/shader/compute_shader_game_of_life.rs)
    * [12] `Bevy` 像素抖动 [dither](https://github.com/byronzr/learn_bevy/tree/main/examples/custom/shader/dither.rs)
    * [13] `Bevy` 分切圆环 [annulus](https://github.com/byronzr/learn_bevy/tree/main/examples/custom/shader/annulus.rs)
    * [14] `Bevy` 噪声扰动
        * 一维 [noise_distortion_1d](https://github.com/byronzr/learn_bevy/tree/main/examples/custom/shader/noise_distortion_1d.rs)
        * 二维 [noise_distortion_2d](https://github.com/byronzr/learn_bevy/tree/main/examples/custom/shader/noise_distortion_2d.rs)

# 实践型案例
* **[camera_renderlayer](https://github.com/byronzr/learn_bevy/tree/main/examples/byronzr/camera_renderlayer/main.rs):** 关于 camera 中的 RenderLayers / TaregetCamera 的相关实践
    * [0.16] 变更了部分函数名称
* **[inventory](https://github.com/byronzr/learn_bevy/tree/main/examples/byronzr/inventory/main.rs):** 利用 `Trigger<Pointer<T>>` 实现物品栏堆叠
    * [0.16] 支持了透明区域无法选择的直觉操作
* **[hexagon_tile](https://github.com/byronzr/learn_bevy/tree/main/examples/byronzr/hexagon_tile):** 六边形走地图与迷雾(三种区域测试方式)
    * 以纯数学函数实现
    * MeshPickingPlugin 加入后,支持 Mesh2d 的 picking 事件
    * Rapier2d 实现 Collider Interscetion Test
* **[space_battle](https://github.com/byronzr/learn_bevy/tree/main/examples/byronzr/space_battle)** 类 starsector 的太空战
    * 弹道,寻敌,护盾,伤害,炮塔转向,规避,幅能冷却
* **[ffui](https://github.com/byronzr/learn_bevy/tree/main/examples/byronzr/ffui)** ffmpeg 套壳
    * 基于 bevy / tokio / ffmpeg 
    * clipboard 导入(连续导入/单次导入/锁定导入)
    * ffmpeg 随机截图,视频转换(videotoolbox/libx265)
    * json 保存文件处理状态
    * 一些 bevy 开发 desktop application 可能会遇到的基本实现
        * scrollable (可滚动区域)
        * button (按钮)
        * multipage (多页面)
        * preview (显示截图预览)
        * toast (渐进提示窗)

    
# Rapier (bevy_rapier2d) 

以下是一些用于观察 rapier 物理特性的实践用例,更多资料请查阅官网 [rapier](https://rapier.rs/)

* **[rapier_rigid_type](https://github.com/byronzr/learn_bevy/tree/main/examples/rapier2d/rigid_type.rs):** Rapier2d 的 rigid 类型介绍
* **[rapier_rigid_related](https://github.com/byronzr/learn_bevy/tree/main/examples/rapier2d/rigid_related.rs):** Rapier2d 的 rigid 与之相关的一些 Component 用例说明
* **[rapier_rigid_kinematic](https://github.com/byronzr/learn_bevy/tree/main/examples/rapier2d/rigid_kinematic.rs):** 对于 Rigidbody::Kinematic* 用例说明
* **[rapier_collider_type](https://github.com/byronzr/learn_bevy/tree/main/examples/rapier2d/collider_type.rs):** Rapier2d 的 collider 类型介绍(实体与传感器)
* **[rapier_collider_related](https://github.com/byronzr/learn_bevy/tree/main/examples/rapier2d/collider_related.rs):** Rapier2d 的 collider 相关的 Component 用例
* **[rapier_collider_event](https://github.com/byronzr/learn_bevy/tree/main/examples/rapier2d/collider_event.rs):** 关于 Event  用例
* **[rapier_scene_queries_cast](https://github.com/byronzr/learn_bevy/tree/main/examples/rapier2d/scene_queries_cast.rs):** 关于 场景查询 (Scene Queries) cast(ray/shape) 用例
* **[rapier_scene_queries_projection](https://github.com/byronzr/learn_bevy/tree/main/examples/rapier2d/scene_queries_projection.rs):** 关于 场景查询 (point projection) 用例
* **[rapier_scene_queries_intersections](https://github.com/byronzr/learn_bevy/tree/main/examples/rapier2d/scene_queries_intersections.rs):** 关于 场景查询 (intersection) 用例
* **[rapier_joints](https://github.com/byronzr/learn_bevy/tree/main/examples/rapier2d/joints.rs):** 关于 (joints) 用例
* **[rapier_joints_motor](https://github.com/byronzr/learn_bevy/tree/main/examples/rapier2d/joints_motor.rs):** 关于 (joint_motor) 关节(马达)电机用例
* **[rapier_advanced_detection](https://github.com/byronzr/learn_bevy/tree/main/examples/rapier2d/advanced_detection.rs):** 进阶测试用例

	
	






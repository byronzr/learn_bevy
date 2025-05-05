# 个人笔记

一步步，迈向游戏国度.集成个人零游戏开发经验的探索过程,在没有完整策案前,会不断累游戏场景,应用(桌面)场景,随着版本迭代,可能会出现"造轮子"的案例.

# 官方用例 
请查看官方用例 [bevy](https://bevyengine.org/),中文注释的代码 [高级伴读](https://github.com/byronzr/learn_bevy/tree/main/examples),(从 0.15 开始的注释,如果没有出现 panic 将不会主动优化代码,仅作来理解使用)

# 实践型案例
* **[camera_renderlayer](https://github.com/byronzr/learn_bevy/tree/main/examples/byronzr/camera_renderlayer.rs):** 关于 camera 中的 RenderLayers / TaregetCamera 的相关实践
    * [0.16] 变更了部分函数名称
* **[inventory](https://github.com/byronzr/learn_bevy/tree/main/examples/byronzr/inventory.rs):** 利用 `Trigger<Pointer<T>>` 实现物品栏堆叠
    * [0.16] 支持了透明区域无法选择的直觉操作
* **[hexagon_tile](https://github.com/byronzr/learn_bevy/tree/main/examples/byronzr/hexagon_tile):** 六边形走地图与迷雾(三种区域测试方式)
    * 以纯数学函数实现
    * MeshPickingPlugin 加入后,支持 Mesh2d 的 picking 事件
    * Rapier2d 实现 Collider Interscetion Test
    
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
* **[rapier_joints](https://github.com/byronzr/learn_bevy/tree/main/examples/rapier2d/joints.rs):** 关于 场景查询 (intersection) 用例
	
	






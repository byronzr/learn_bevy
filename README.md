# 官方用例跟读
一步步，迈向游戏国度

## hello_bevy
	一个终端显示的调度与基础运行构建模型,间隔 2 秒不断打印 greeting
	

## Bevy Examples
* time
  * (✓) timers / 计时器的应用,一个文本进度报告终端. 
  * (✓) virtual_time / 在同一环境中两个 Sprite 分别互不影响的应用 Time<Real> 与 Time<Virtual> 速度.
  * (✓) time / 在控制台用短字符交互,描述一个 FixedUpdate / Update 还有各种 Time 的关系
* ECS	
  * (✓) ecs_guide /	更完整 ECS 用例,一个模拟游戏得分的终端用例,包括 SystemSet 对 Update 的细分.
  * (✓) observer_propagation /Entity 嵌套,事件 (Event) 的监视(observe)与传播(propagate),模拟一个身着盔甲的哥布林,承受伤害的过程
  * (✓) fixed_timestep / 每个 system 都有一个 Local<T> 局布变量
  * (✓) change_detection / component 与 resource 变更侦测,通过 Query 得到 Component 的引用
  * (✓) component_hooks / component hook,组件的 "勾子",一种在组织结构上优于 event 的监视模式
  * (✓) custom_query_param /	自定义 Query 结构,实现结果与条件集成简单化
  * (✓) custom_schedule / 自定义 schedule 
  * (✓) dynamic / 动态的 component,entity,query
  * (✓) fallible_params / 不可靠的参数,除了Query以外,还有 Populated 与 Single 两个极为有用的查询
  * (✓) generic_system / 泛型 system 的用例
  * (✓) hierarchy / Entity 的组织嵌套与实际影响
  * (✓) iter_combinations / Query 结果成对组合
  * (□) nondeterministic_system_order / system 并行的不确定性
  * (✓) event / 简单的 event 发送与接收
  * (✓) observers / 一个随机分部多枚地雷,连琐爆炸的例子,使用 Trigger 触发器
  * (✓) one_shot_systems / 展示了如何将 system 包装到 Component 中等待用户输入触发
  * (✓) parallel_query / 高性能的并行迭代器(多用于物理性,巨量的 entity 运算)
  * (✓) removal_detection / Bevy 内建的 Component OnRemove 事件触发器的用例
  * (✓) run_conditions / system 运行条件(run_if) 与闭包的结合
  * (✓) send_and_receive_events / 在一个相同的 system 作用域内,完成 Event 的读写
  * (□) startup_system
  * (✓) system_closure / 如何用闭包构建 system
  * (✓) system_param / 为 system 添加可直接访问的自定义的参数
  * (✓) system_piping / system 的串联
  * (✓) system_stepping / system 的执行细节 Stepping 的应用(单步调试)
* 2D
  * (✓) 2d_shapes / 绘制基础几何图形
  * (✓) 2d_viewport_to_world / 坐标系转换的意义
  * (□) bloom_2d.rs / 光晕细节
  * (✓) bounding_2d.rs / 碰撞与切线的用例
  * (□) cpu_draw.rs / CPU纹理绘制
  * (□) custom_gltf_vertex_attribute.rs / WGSL渲染用例
  * (□) mesh2d_alpha_mode / 透明与混合
  * (□) mesh2d_arcs.rs / UV贴图的一些细节
  * (□) mesh2d_manual.rs / 自定义渲染
  * (□) mesh2d_vertex_color_texture.rs 顶点渲染与纹理混合
  * (✓) mesh2d.rs / mesh2d与material2d的基本用法
  * (✓) move_sprite.rs / Sprite 移动基本用法
  * (□) pixel_grid_snap / 像素网格渲染
  * (✓) rotation / 两种常用的旋转索敌用例(咬住敌人与炮塔瞄准)
  * (✓) sprite_animation / 计时器在 Sprite 动画中的应用
  * (✓) sprite_flipping / Sprite 图片镜像(翻转)
  * (✓) sprite_sheet / Sprite 动画实现
  * (✓) sprite_slice / 九宫贴片模式
  * (✓) sprite_tile / 平铺模式
  * (□) sprite / 构建精灵的例子
  * (✓) text2d / 适用于游戏场景的字体文本
  * (✓) texture_atlas / 纹理集的基本用法
  * (✓) transparency_2d / (Sprite) 如何使用透明度
  * (□) wireframe_2d.rs / 调试 2d 渲染的用例
* scene
  * (✓) scene 场景载入与保存
* state
  * (✓) states / 关于 state 的基本用法
  * (✓) computed_states / state 的转换运算与 entity 自动回收
  * (□) custom_transitions / 自定义 state 转换触发点
  * (✓) sub_states / state 的嵌套与扩展
* input
  * char_input_events / 简单的字符输入事件示例
  * keyboard_modifiers /修饰键与组合键的用法
  * mouse_grab / 鼠标(截取)隐藏与释放
  * text_input / 输入法(可中文)事件的示例
* picking
  * sprite_picking / sprite与鼠标联动事件


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
  * (□) custom_schedule / 自定义 schedule 
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


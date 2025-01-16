# learn_bevy
一步步，迈向游戏国度

## lesson 1
> 一个终端显示的调度与基础运行构建模型
> 间隔 2 秒不断打印 greeting
`cargo run --example lesson1`

### lesson 1_1
> 更完整 ECS 用例
> 一个模拟游戏得分的终端用例
> 包括 SystemSet 对 Update 的细分

### lesson 1_2
> timer 计时器的应用
> 一个文本进度报告终端,模拟: 
> 一个实体(PrintOnCompletionTimer) 
> 一个超时间 Countdown.main_timer 
> 一个节点时间 Countdown.percent_trigger

## lesson 2
> 事件 (Event) 的监视(observe)与传播(propagate)
> 模拟一个身着盔甲的哥布林,承受伤害的过程

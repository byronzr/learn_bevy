# 基于 StarSector 太空对战的场景复现学习

* ### effects 视效
    * EffectsPlugin 特效
    * explode.rs
        * [system] `enemy_explode()` 敌人爆炸
        * [system] `projectile_explode()` 投射物爆炸
    * projectile.rs
        * [system] `projectile_trail()` 投射物(炮弹)尾焰轨迹
    * engine.rs
        * [system] `engine_flame()` 引擎尾焰
		
* ### strategies 策略
    * StrategiesPlugin 策略
        * player.rs
            * [system] `player_detection()` 玩家寻敌
        * turret.rs
            * [system] `turret_detection()` 炮塔寻敌
        * projectile.rs
            * [system] `projectile_detection()` 投射物(missile)寻敌
			
			
* ### ui 界面
    * UiPlugin 界面管理
		* mod.rs
			* [system] `setup()`, 初始化
			* [system] `perspective_interaction()` 场景缩放
			* [system] `show_grid` 辅助网格
			* [fn] `button()` 快速 button 函数
		* main.rs
			* [system] `ui_main_setup()` 主控菜单
			* [system] `main_menu_interaction()` 
		* game.rs
			* [system] `ui_game_setup()` 游戏菜单
            * [system] `game_menu_interaction()` 
		* detect.rs 
			* 调试信息
			
    
* ### resources 资源
	* state.rs
        * [Resource] `MainMenu` 主控菜单
            (调试,绘制,开始释放敌人)
        * [Resource] `GameMenu` 游戏菜单
            (增减推力,扭力,当前武器,目标敌人锁定)
    * projectile.rs 抛射物
        * [Component] `ProjectileState` 投射物本身就是个小飞船
		* [Component] `ProjectileType`
            * Missile, 导弹长射程,有追踪,无幅能,有爆炸范围,有限数量
            * Bullet, 普通弹丸,低幅能,一次泄弹
            * Beam, 立即命中,高幅能,高伤害,充能久
            * 
    * turret.rs 武器挂载点
        * [Resource] `TurretResource`
            * 朝向与可旋角度
	* weapon.rs 
		* 武器种类与初始化
    * player.rs
        * [Resource] `PlayerShipResource`
            * 玩家旗舰信息
    * enemy.rs
        * [Component] `EnemyShipResource`
            * 敌舰信息
			
* ### components 组件
	* 所有组件统一管理
			
* ### utility 实用函数
    * track.rs 向量与角度计算封装
        * [fn] `rotate_to()` 计算可旋角度与方向
        * [fn] `forward()` 朝向(向量)与距离
    * png.rs 图像加载
        * [fn] `load()` 加载 PNG 图片,生成 Collider 所需要 vertices
	* curve.rs 曲线函数
		* [fn] `ease_in()` 缓入
		* [fn] `ease_out()` 缓出
		* [fn] `ease_in_out()` 缓入缓出

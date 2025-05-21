# 基于 StarSector 太空对战的场景复现学习

* ### effects 视效
    * EffectsPlugin 特效
    * explode.rs
        * [system] //TODO// `enemy_explode()` 敌人爆炸
        * [system] //TODO// `projectile_explode()` 投射物爆炸
    * projectile.rs
        * [system] //TODO// `projectile_trail()` 投射物(炮弹)尾焰轨迹
    * engine.rs
        * [system] //TODO// `engine_flame()` 引擎尾焰
		
* ### strategies 策略
    * StrategiesPlugin 策略
		* enemy.rs
			* [system] `random_enemies()` 随机生成敌舰
			* [system] //TODO// `enemies_movement` 敌人向中心靠拢
			* [system] `enemy_collision()` 敌般碰撞事件
        * player.rs
			* [system] `generate_player_ship()` 初始化旗舰
			* [system] `drift()` idle 漂移状态
            * [system] `player_detection()` 玩家寻敌
        * turret.rs
            * [system] `turret_detection()` 炮塔寻敌
        * projectile.rs
			* [observer] `emit_observer()` 发射监视器
            * [system] //TODO// `projectile_detection()` 投射物(missile)寻敌
		* weapon.rs
			* [system] ``weapon_maintenance` 武器上弹,充能,等计算
			
* ### ui 界面
    * UiPlugin 界面管理
		* mod.rs
			* [system] `setup()`, 初始化
			* [system] //TODO// `zoom()` 场景缩放
			* [system] `show_grid` 辅助网格
			* [fn] `button()` 快速 button 函数
		* main.rs
			* [system] `ui_main_setup()` 主控菜单
			* [system] `main_menu_interaction()` 
		* game.rs
			* [system] `ui_game_setup()` 游戏菜单
            * [system] `game_menu_interaction()` 
		* detect.rs 
			* [system] `direct_test()` 以中心点为起点,以Y轴为朝向的示例
		* hud.rs
			* // TODO // 仪表
			
    
* ### resources 资源
	* menu.rs 菜单状态
    * turret.rs 武器挂载点
    * player.rs 玩家旗舰
    * enemy.rs 敌舰统一资源
       
			
* ### components 组件
	* mod.rs
	* ship.rs 舰体组件
	* weapon.rs 武器组件
	
			
* ### utility 实用函数
    * track.rs 向量与角度计算封装
        * [fn] `rotate_to()` 计算可旋角度与方向
        * [fn] `forward()` 朝向(向量)与距离
    * png.rs 图像加载
        * [fn] `load()` 加载 PNG 图片,生成 Collider 所需要 vertices
	* curve.rs 曲线函数
		* [fn] //TODO// `ease_in()` 缓入
		* [fn] //TODO// `ease_out()` 缓出
		* [fn] //TODO// `ease_in_out()` 缓入缓出

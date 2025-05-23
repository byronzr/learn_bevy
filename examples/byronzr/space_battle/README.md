# 基于 StarSector 太空对战的场景复现学习

* ### effects 视效
    * EffectsPlugin 特效
    * explode.rs
        * [system] //Deferred// `enemy_explode()` 敌人爆炸
        * [system] //Deferred// `projectile_explode()` 投射物爆炸
    * projectile.rs
        * [system] //Deferred// `projectile_trail()` 投射物(炮弹)尾焰轨迹
    * engine.rs
        * [system] `engine_flame()` 引擎尾焰
		
* ### strategies 策略
    * StrategiesPlugin 策略
		* enemy/
			* random.rs
				* [system] `random_enemies()` 随机生成敌舰
			* movement.rs
				* [system] `enemy_movement` 敌人向中心靠拢
				* [system] `enemy_locked` 炮塔锁定着色
			* collision.rs
				* [system] `enemy_collision()` 敌般碰撞事件
        * player/
			* generate.rs
				* [system] `generate_player_ship()` 初始化旗舰
			* drift.rs
				* [system] `drift()` idle 漂移状态
			* player_detection.rs
				* [system] `player_detection()` 玩家寻敌
				* [fn] `track_target()` 
        * turret.rs
            * [system] `turret_detection()` 炮塔寻敌
        * projectile.rs
			* [observer] `emit_observer()` 发射监视器
			* [observer] `seek_observer` 炮塔索敌监视器
			* [system] `seek_target_clean` 炮塔标记清理
            * [system] //Deferred// `Projectile_detection()` 投射物(missile)寻敌
		* weapon.rs
			* [system] ``weapon_maintenance` 武器上弹,充能,等计算
			
* ### ui 界面
    * UiPlugin 界面管理
		* mod.rs
			* [system] `setup()`, 初始化
			* [system] `zoom()` 场景缩放
			* [system] `lock_player` 锁定跟随玩家
			* [system] `show_grid` 辅助网格
			* [fn] `button()` 快速 button 函数
		* panel/ 
			* setup.rs
				* [system] `ui_main_setup()` 主控菜单
			* interaction.rs
				* [system] `main_menu_interaction()` 
		* game/
			* interaction.rs
	            * [system] `game_menu_interaction()` 
			* setup.rs
				* [system] `ui_game_setup()` 游戏菜单
		* detect.rs 
			* [system] `direct_test()` 以中心点为起点,以Y轴为朝向的示例
		* hud/
			* init.rs
				* [system] `init_hud` 初始化 hud
			* sync.rs
				* [system] `sync_hud` 同步 hud 数据
			
    
* ### resources 资源
	* menu.rs 菜单状态
    * turret.rs 武器挂载点
    * player.rs 玩家旗舰
    * enemy.rs 敌舰统一资源

* ### events 事件
* ### shader 着色器
	* [struct] WGSL shder 尾焰效果
       
			
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
		* [fn] //Deferred// `ease_in()` 缓入
		* [fn] //Deferred// `ease_out()` 缓出
		* [fn] //Deferred// `ease_in_out()` 缓入缓出

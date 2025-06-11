use bevy::prelude::*;
use bevy::input::mouse::MouseWheel;
use rand::{thread_rng, Rng};
use crate::components::*;
use crate::resources::*;
use crate::states::*;
use crate::audio::AudioEvent;

// ===== 设置和清理系统 =====

pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

pub fn load_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    // 加载所有小鸟纹理
    let bird_textures = BirdCharacter::all_characters()
        .iter()
        .map(|character| asset_server.load(character.get_texture_path()))
        .collect();
    
    // 加载所有管道纹理
    let pipe_textures = PipeType::all_types()
        .iter()
        .map(|pipe_type| asset_server.load(pipe_type.get_texture_path()))
        .collect();

    // 加载数字纹理 (0-9)
    let number_textures = (0..10)
        .map(|i| asset_server.load(format!("numbers/{}.png", i)))
        .collect();

    commands.insert_resource(GameAssets {
        bird_textures,
        pipe_textures,
        ground_texture: asset_server.load("ground.png"),
        cloud_texture: asset_server.load("cloud_1.png"),
        mountain_texture: asset_server.load("mountain.png"),
        font: asset_server.load("fonts/NotoSansSC-Regular.ttf"),
        number_textures,
    });
}

pub fn setup_menu_when_ready(
    mut commands: Commands,
    assets: Option<Res<GameAssets>>,
    game_data: Res<GameData>,
    existing_menu: Query<&MenuText>,
) {
    // 检查菜单是否已经设置
    if !existing_menu.is_empty() {
        return;
    }
    
    // 检查资源是否已加载
    let Some(assets) = assets else {
        return;
    };
    
    // 游戏标题
    commands.spawn((
        Text2d::new("Flappy Bevy"),
        TextFont {
            font: assets.font.clone(),
            font_size: 48.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Transform::from_translation(Vec3::new(0.0, 150.0, 1.0)),
        MenuText,
    ));
    
    // 当前角色显示
    commands.spawn((
        Text2d::new(format!("当前角色: {}", game_data.selected_character.get_name())),
        TextFont {
            font: assets.font.clone(),
            font_size: 28.0,
            ..default()
        },
        TextColor(Color::srgb(0.9, 0.9, 0.9)),
        Transform::from_translation(Vec3::new(0.0, 80.0, 1.0)),
        MenuText,
    ));
    
    // 角色预览
    commands.spawn((
        Sprite::from_image(assets.get_bird_texture(game_data.selected_character)),
        Transform::from_translation(Vec3::new(0.0, 20.0, 1.0))
            .with_scale(Vec3::splat(game_data.selected_character.get_scale())),
        CharacterPreview,
        MenuText,
    ));
    
    // 控制说明
    commands.spawn((
        Text2d::new("← → 或滚轮切换角色\n\n空格键或鼠标左键开始游戏"),
        TextFont {
            font: assets.font.clone(),
            font_size: 24.0,
            ..default()
        },
        TextColor(Color::srgb(0.8, 0.8, 0.8)),
        Transform::from_translation(Vec3::new(0.0, -80.0, 1.0)),
        MenuText,
    ));
}

pub fn setup_game(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut game_data: ResMut<GameData>,
) {
    game_data.score = 0;

    // 生成小鸟 - 使用选中的角色和对应的缩放
    commands.spawn((
        Sprite::from_image(assets.get_bird_texture(game_data.selected_character)),
        Transform::from_translation(Vec3::new(-200.0, 0.0, 1.0))
            .with_scale(Vec3::splat(game_data.selected_character.get_scale())),
        Bird {
            character: game_data.selected_character,
        },
        Velocity { y: 0.0 },
        Gravity(980.0),
        Collider,
    ));

    // 生成背景山脉
    for i in 0..5 {
        commands.spawn((
            Sprite::from_image(assets.mountain_texture.clone()),
            Transform::from_translation(Vec3::new(
                i as f32 * 200.0 - 400.0,
                -250.0,
                -1.0,
            )),
            Mountain,
            Scrolling { speed: 50.0 },
        ));
    }

    // 生成云朵
    for i in 0..3 {
        commands.spawn((
            Sprite::from_image(assets.cloud_texture.clone()),
            Transform::from_translation(Vec3::new(
                i as f32 * 300.0 - 300.0,
                200.0,
                -0.5,
            ))
            .with_scale(Vec3::splat(0.8)),
            Cloud,
            Scrolling { speed: 30.0 },
        ));
    }
}

pub fn setup_game_over(
    mut commands: Commands,
    assets: Res<GameAssets>,
    game_data: Res<GameData>,
) {
    commands.spawn((
        Text2d::new(format!(
            "游戏结束！\n\n分数: {}\n最高分: {}\n\n按 R 键重新开始\n按 ESC 键返回菜单选择角色",
            game_data.score, game_data.high_score
        )),
        TextFont {
            font: assets.font.clone(),
            font_size: 30.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
        GameOverText,
    ));
}

// ===== 清理系统 =====

pub fn cleanup_menu(mut commands: Commands, query: Query<Entity, With<MenuText>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn cleanup_game(
    mut commands: Commands,
    bird_query: Query<Entity, With<Bird>>,
    pipe_query: Query<Entity, With<Pipe>>,
    mountain_query: Query<Entity, With<Mountain>>,
    cloud_query: Query<Entity, With<Cloud>>,
    score_query: Query<Entity, With<ScoreText>>,
    score_digits: Query<Entity, With<ScoreDigit>>,
) {
    for entity in bird_query.iter() {
        commands.entity(entity).despawn();
    }
    for entity in pipe_query.iter() {
        commands.entity(entity).despawn();
    }
    for entity in mountain_query.iter() {
        commands.entity(entity).despawn();
    }
    for entity in cloud_query.iter() {
        commands.entity(entity).despawn();
    }
    for entity in score_query.iter() {
        commands.entity(entity).despawn();
    }
    for entity in score_digits.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn cleanup_game_over(mut commands: Commands, query: Query<Entity, With<GameOverText>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

// ===== 菜单系统 =====

pub fn menu_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut audio_events: EventWriter<AudioEvent>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) || mouse_input.just_pressed(MouseButton::Left) {
        audio_events.write(AudioEvent::Swoosh);
        next_state.set(GameState::Playing);
    }
}

pub fn character_selection_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut game_data: ResMut<GameData>,
    assets: Option<Res<GameAssets>>,
    mut scroll_events: EventReader<MouseWheel>,
    mut text_query: Query<&mut Text2d, With<MenuText>>,
    mut preview_query: Query<(&mut Sprite, &mut Transform), With<CharacterPreview>>,
) {
    let mut character_changed = false;
    
    // 键盘输入
    if keyboard_input.just_pressed(KeyCode::ArrowLeft) {
        game_data.selected_character = game_data.selected_character.previous();
        character_changed = true;
    } else if keyboard_input.just_pressed(KeyCode::ArrowRight) {
        game_data.selected_character = game_data.selected_character.next();
        character_changed = true;
    }
    
    // 鼠标滚轮输入
    for scroll in scroll_events.read() {
        if scroll.y > 0.0 {
            game_data.selected_character = game_data.selected_character.next();
            character_changed = true;
        } else if scroll.y < 0.0 {
            game_data.selected_character = game_data.selected_character.previous();
            character_changed = true;
        }
    }
    
    // 更新UI
    if character_changed {
        // 检查assets是否可用
        if let Some(assets) = assets {
            // 更新角色名称文本
            for mut text in text_query.iter_mut() {
                if text.0.contains("当前角色") {
                    **text = format!("当前角色: {}", game_data.selected_character.get_name());
                }
            }
            
            // 更新角色预览
            for (mut sprite, mut transform) in preview_query.iter_mut() {
                *sprite = Sprite::from_image(assets.get_bird_texture(game_data.selected_character));
                transform.scale = Vec3::splat(game_data.selected_character.get_scale());
            }
            
        }
    }
}

// ===== 游戏逻辑系统 =====

pub fn bird_input_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut bird_query: Query<&mut Velocity, With<Bird>>,
    config: Res<GameConfig>,
    mut audio_events: EventWriter<AudioEvent>,  // 添加音频事件写入器
) {
    if keyboard_input.just_pressed(KeyCode::Space) || mouse_input.just_pressed(MouseButton::Left) {
        for mut velocity in bird_query.iter_mut() {
            velocity.y = config.jump_force;
            audio_events.write(AudioEvent::Jump);  // 添加跳跃音效
        }
    }
}

pub fn bird_physics_system(
    time: Res<Time>,
    mut bird_query: Query<(&mut Transform, &mut Velocity, &Gravity), With<Bird>>,
) {
    for (mut transform, mut velocity, gravity) in bird_query.iter_mut() {
        // 应用重力
        velocity.y -= gravity.0 * time.delta_secs();
        
        // 更新位置
        transform.translation.y += velocity.y * time.delta_secs();
        
        // 限制小鸟旋转角度
        let angle = (velocity.y / 300.0).clamp(-1.0, 1.0) * 0.5;
        transform.rotation = Quat::from_rotation_z(angle);
    }
}

pub fn pipe_spawn_system(
    time: Res<Time>,
    mut commands: Commands,
    mut config: ResMut<GameConfig>,
    assets: Res<GameAssets>,
) {
    config.pipe_spawn_timer.tick(time.delta());
    
    if config.pipe_spawn_timer.just_finished() {
        let mut rng = thread_rng();
        let gap_y = rng.gen_range(-100.0..100.0);
        
        // 随机选择管道类型
        let pipe_types = PipeType::all_types();
        let selected_pipe_type = pipe_types[rng.gen_range(0..pipe_types.len())];
        let pipe_texture = assets.get_pipe_texture(selected_pipe_type);
        let pipe_scale = selected_pipe_type.get_scale();
        
        // 使用标准间距
        let adjusted_gap = config.pipe_gap;
        
        // 上管道
        commands.spawn((
            Sprite::from_image(pipe_texture.clone()),
            Transform::from_translation(Vec3::new(
                500.0,
                gap_y + adjusted_gap / 2.0 + 150.0,
                0.0,
            ))
            .with_rotation(Quat::from_rotation_z(std::f32::consts::PI))
            .with_scale(Vec3::splat(pipe_scale)),
            Pipe { pipe_type: selected_pipe_type },
            Scrolling { speed: config.pipe_speed },
            Collider,
        ));
        
        // 下管道
        commands.spawn((
            Sprite::from_image(pipe_texture),
            Transform::from_translation(Vec3::new(
                500.0,
                gap_y - adjusted_gap / 2.0 - 150.0,
                0.0,
            ))
            .with_scale(Vec3::splat(pipe_scale)),
            Pipe { pipe_type: selected_pipe_type },
            Scrolling { speed: config.pipe_speed },
            Collider,
        ));
    }
}

pub fn scrolling_system(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &Scrolling), (Without<Mountain>, Without<Cloud>)>,
    mut mountain_query: Query<&mut Transform, (With<Mountain>, Without<Cloud>)>,
    mut cloud_query: Query<&mut Transform, (With<Cloud>, Without<Mountain>)>,
) {
    // 处理普通滚动实体（管道等）
    for (entity, mut transform, scrolling) in query.iter_mut() {
        transform.translation.x -= scrolling.speed * time.delta_secs();
        
        // 移除超出屏幕的实体
        if transform.translation.x < -600.0 {
            commands.entity(entity).despawn();
        }
    }
    
    // 处理山脉循环滚动
    for mut transform in mountain_query.iter_mut() {
        transform.translation.x -= 50.0 * time.delta_secs();
        
        // 当山脉移出左侧时，移动到屏幕右侧外并添加随机间隔
        if transform.translation.x < -600.0 {
            let mut rng = rand::thread_rng();
            let random_gap = rng.gen_range(100.0..400.0); // 随机间隔100-400像素
            // 移动到屏幕右侧外（600像素外）+ 基础间距 + 随机间隔
            transform.translation.x = 600.0 + 200.0 + random_gap;
        }
    }
    
    // 处理云朵循环滚动
    for mut transform in cloud_query.iter_mut() {
        transform.translation.x -= 30.0 * time.delta_secs();
        
        // 当云朵移出左侧时，移动到屏幕右侧外并添加随机间隔
        if transform.translation.x < -600.0 {
            let mut rng = rand::thread_rng();
            let random_gap = rng.gen_range(200.0..600.0); // 随机间隔200-600像素
            // 移动到屏幕右侧外（600像素外）+ 基础间距 + 随机间隔
            transform.translation.x = 600.0 + 300.0 + random_gap;
        }
    }
}

pub fn collision_system(
    bird_query: Query<(&Transform, &Bird), With<Collider>>,
    pipe_query: Query<(&Transform, &Pipe), (With<Collider>, Without<Bird>)>,
    mut next_state: ResMut<NextState<GameState>>,
    mut audio_events: EventWriter<AudioEvent>,  // 添加音频事件写入器
) {
    for (bird_transform, bird) in bird_query.iter() {
        let bird_radius = bird.character.get_collision_radius();
        
        // 检查边界碰撞
        if bird_transform.translation.y - bird_radius < -280.0 
            || bird_transform.translation.y + bird_radius > 280.0 {
            audio_events.write(AudioEvent::Hit);  // 添加碰撞音效
            next_state.set(GameState::GameOver);
            return;
        }
        
        // 改进的管道碰撞检测
        for (pipe_transform, pipe) in pipe_query.iter() {
            if check_pipe_collision(bird_transform, bird, pipe_transform, pipe) {
                audio_events.write(AudioEvent::Hit);  // 添加碰撞音效
                next_state.set(GameState::GameOver);
                return;
            }
        }
    }
}

// 新增：专门的管道碰撞检测函数
fn check_pipe_collision(
    bird_transform: &Transform,
    bird: &Bird,
    pipe_transform: &Transform,
    pipe: &Pipe,
) -> bool {
    let bird_radius = bird.character.get_collision_radius();
    let pipe_scale = pipe.pipe_type.get_scale();
    let (base_offset_x, base_offset_y) = pipe.pipe_type.get_collision_offset();
    
    // 小鸟边界
    let bird_left = bird_transform.translation.x - bird_radius;
    let bird_right = bird_transform.translation.x + bird_radius;
    let bird_bottom = bird_transform.translation.y - bird_radius;
    let bird_top = bird_transform.translation.y + bird_radius;
    
    // 管道碰撞中心（考虑偏移）
    let pipe_center_x = pipe_transform.translation.x + base_offset_x * pipe_scale;
    let pipe_center_y = pipe_transform.translation.y + base_offset_y * pipe_scale;
    
    // 首先检查X轴是否重叠
    let (width_factor, _) = pipe.pipe_type.get_collision_bounds();
    let pipe_width = 52.0 * pipe_scale * width_factor;
    let pipe_left = pipe_center_x - pipe_width / 2.0;
    let pipe_right = pipe_center_x + pipe_width / 2.0;
    
    // 如果X轴没有重叠，直接返回false
    if bird_right <= pipe_left || bird_left >= pipe_right {
        return false;
    }
    
    // X轴重叠时，使用基于高度区间的碰撞检测
    if pipe.pipe_type.use_precise_collision() {
        // 使用多段精确碰撞检测
        let segments = pipe.pipe_type.get_collision_segments();
        
        for (seg_offset_x, seg_offset_y, width_factor, height_factor) in segments {
            // 计算每个碰撞段的实际位置和尺寸
            let segment_width = 52.0 * pipe_scale * width_factor;
            let segment_height = 320.0 * pipe_scale * height_factor;
            
            // 段的中心位置（基础偏移 + 段偏移）
            // 修正：seg_offset应该直接乘以pipe_scale，而不是先乘以320.0
            let segment_center_x = pipe_transform.translation.x + 
                (base_offset_x + seg_offset_x * 52.0) * pipe_scale;
            let segment_center_y = pipe_transform.translation.y + 
                (base_offset_y + seg_offset_y * 160.0) * pipe_scale;
            
            // 段的边界
            let segment_left = segment_center_x - segment_width / 2.0;
            let segment_right = segment_center_x + segment_width / 2.0;
            let segment_bottom = segment_center_y - segment_height / 2.0;
            let segment_top = segment_center_y + segment_height / 2.0;
            
            // 检查与当前段的碰撞（基于高度区间）
            if bird_right > segment_left && bird_left < segment_right {
                // X轴重叠时，检查Y轴是否在障碍物的高度范围内
                if bird_top > segment_bottom && bird_bottom < segment_top {
                    return true; // 在障碍物高度范围内，发生碰撞
                }
            }
        }
        false // 不在任何障碍物段的高度范围内，可以通过
    } else {
        // 传统管道使用简单AABB碰撞检测
        let (_, height_factor) = pipe.pipe_type.get_collision_bounds();
        
        // 计算管道的实际碰撞区域
        let pipe_height = 320.0 * pipe_scale * height_factor;
        
        // 管道边界
        let pipe_bottom = pipe_center_y - pipe_height / 2.0;
        let pipe_top = pipe_center_y + pipe_height / 2.0;
        
        // 基于高度区间的碰撞检测
        bird_top > pipe_bottom && bird_bottom < pipe_top
    }
}

pub fn score_system(
    bird_query: Query<&Transform, With<Bird>>,
    pipe_query: Query<&Transform, (With<Pipe>, Without<Bird>)>,
    mut game_data: ResMut<GameData>,
    mut audio_events: EventWriter<AudioEvent>,  // 添加音频事件写入器
) {
    for bird_transform in bird_query.iter() {
        for pipe_transform in pipe_query.iter() {
            // 如果小鸟通过了管道，增加分数
            if pipe_transform.translation.x < bird_transform.translation.x - 50.0
                && pipe_transform.translation.x > bird_transform.translation.x - 55.0
            {
                game_data.score += 1;
                audio_events.write(AudioEvent::Score);  // 添加得分音效
            }
        }
    }
}

pub fn number_score_display(
    mut commands: Commands,
    game_data: Res<GameData>,
    game_assets: Res<GameAssets>,
    score_digits: Query<Entity, With<ScoreDigit>>,
) {
    // 清除现有的数字精灵
    for entity in score_digits.iter() {
        commands.entity(entity).despawn();
    }
    
    // 将分数转换为字符串
    let score_str = game_data.score.to_string();
    let digit_count = score_str.len();
    
    // 优化后的数字布局参数
    let digit_width = 24.0;
    let digit_spacing = 4.0;
    let total_width = (digit_count as f32) * digit_width + (digit_count.saturating_sub(1) as f32) * digit_spacing;
    let start_x = -total_width / 2.0 + digit_width / 2.0;
    
    // 为每个数字创建精灵
    for (i, digit_char) in score_str.chars().enumerate() {
        if let Some(digit) = digit_char.to_digit(10) {
            let x_position = start_x + (i as f32) * (digit_width + digit_spacing);
            
            commands.spawn((
                Sprite {
                    image: game_assets.number_textures[digit as usize].clone(),
                    ..default()
                },
                Transform {
                    translation: Vec3::new(x_position, 200.0, 10.0),
                    scale: Vec3::splat(0.8),
                    ..default()
                },
                ScoreDigit,
            ));
        }
    }
}

pub fn game_over_system(
    mut game_data: ResMut<GameData>,
) {
    if game_data.score > game_data.high_score {
        game_data.high_score = game_data.score;
    }
}

pub fn restart_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    current_state: Res<State<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyR) {
        match current_state.get() {
            GameState::GameOver => next_state.set(GameState::Playing),
            _ => {}
        }
    }
    
    if keyboard_input.just_pressed(KeyCode::Escape) {
        match current_state.get() {
            GameState::Playing | GameState::GameOver => next_state.set(GameState::Menu),
            _ => {}
        }
    }
}

// 添加新的系统函数
pub fn on_game_over(
    mut audio_events: EventWriter<AudioEvent>,
) {
    audio_events.write(AudioEvent::Die);
}

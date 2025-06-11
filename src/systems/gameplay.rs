use bevy::prelude::*;
use rand::{thread_rng, Rng};
use crate::components::*;
use crate::resources::*;
use crate::states::*;
use crate::audio::AudioEvent;

// ===== 游戏逻辑系统 =====

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

pub fn bird_input_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut bird_query: Query<&mut Velocity, With<Bird>>,
    config: Res<GameConfig>,
    mut audio_events: EventWriter<AudioEvent>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) || mouse_input.just_pressed(MouseButton::Left) {
        for mut velocity in bird_query.iter_mut() {
            velocity.y = config.jump_force;
            audio_events.write(AudioEvent::Jump);
        }
    }
}

pub fn pipe_spawn_system(
    time: Res<Time>,
    mut commands: Commands,
    mut config: ResMut<GameConfig>,
    assets: Res<GameAssets>,
    game_data: Res<GameData>,
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
        
        // 根据分数动态调整通道间隙大小
        // 基础间隙150，每5分减少10像素，最小保持80像素
        let gap_reduction = (game_data.score / 5) as f32 * 10.0;
        let adjusted_gap = (config.pipe_gap - gap_reduction).max(80.0);
        
        // 上管道 - 调整Y位置确保覆盖到屏幕顶部
        commands.spawn((
            Sprite::from_image(pipe_texture.clone()),
            Transform::from_translation(Vec3::new(
                500.0,
                gap_y + adjusted_gap / 2.0 + 200.0, // 增加偏移量确保覆盖屏幕顶部
                0.0,
            ))
            .with_rotation(Quat::from_rotation_z(std::f32::consts::PI))
            .with_scale(Vec3::splat(pipe_scale)),
            Pipe { pipe_type: selected_pipe_type },
            Scrolling { speed: config.pipe_speed },
            Collider,
        ));
        
        // 下管道 - 调整Y位置确保覆盖到屏幕底部
        commands.spawn((
            Sprite::from_image(pipe_texture),
            Transform::from_translation(Vec3::new(
                500.0,
                gap_y - adjusted_gap / 2.0 - 200.0, // 增加偏移量确保覆盖屏幕底部
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

pub fn score_system(
    bird_query: Query<&Transform, With<Bird>>,
    pipe_query: Query<&Transform, (With<Pipe>, Without<Bird>)>,
    mut game_data: ResMut<GameData>,
    mut audio_events: EventWriter<AudioEvent>,
) {
    for bird_transform in bird_query.iter() {
        // 收集所有通过的管道x坐标，去重后计分
        let mut scored_x_positions = std::collections::HashSet::new();
        
        for pipe_transform in pipe_query.iter() {
            // 如果小鸟通过了管道
            if pipe_transform.translation.x < bird_transform.translation.x - 50.0
                && pipe_transform.translation.x > bird_transform.translation.x - 55.0
            {
                // 将x坐标四舍五入到整数，确保同一对管道有相同的x坐标
                let pipe_x = pipe_transform.translation.x.round() as i32;
                scored_x_positions.insert(pipe_x);
            }
        }
        
        // 为每个唯一的x位置增加分数
        for _ in scored_x_positions {
            game_data.score += 1;
            audio_events.write(AudioEvent::Score);
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
            GameState::Playing | GameState::GameOver | GameState::Leaderboard => next_state.set(GameState::Menu),
            _ => {}
        }
    }
}

pub fn on_game_over(
    mut audio_events: EventWriter<AudioEvent>,
) {
    audio_events.write(AudioEvent::Die);
}
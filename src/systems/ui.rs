use bevy::prelude::*;
use crate::components::*;
use crate::resources::*;
use crate::states::*;
use crate::audio::AudioEvent;

// ===== UI和显示系统 =====

pub fn setup_game_over(
    mut commands: Commands,
    assets: Res<GameAssets>,
    game_data: Res<GameData>,
) {
    // 添加半透明黑色蒙版背景
    commands.spawn((
        Sprite {
            color: Color::srgba(0.0, 0.0, 0.0, 0.7), // 70%透明度的黑色蒙版
            custom_size: Some(Vec2::new(800.0, 600.0)), // 覆盖整个屏幕
            ..default()
        },
        Transform::from_translation(Vec3::new(0.0, 0.0, 0.5)), // 在背景之上，文字之下
        GameOverText, // 使用相同的组件标记，方便清理
    ));

    // 游戏结束标题
    commands.spawn((
        Text2d::new("游戏结束"),
        TextFont {
            font: assets.font.clone(),
            font_size: 48.0,
            ..default()
        },
        TextColor(Color::srgb(1.0, 0.8, 0.2)), // 金黄色标题
        Transform::from_translation(Vec3::new(0.0, 100.0, 1.0)),
        GameOverText,
    ));

    // 分数显示
    commands.spawn((
        Text2d::new(format!("本次分数: {}", game_data.score)),
        TextFont {
            font: assets.font.clone(),
            font_size: 36.0,
            ..default()
        },
        TextColor(Color::srgb(0.9, 0.9, 1.0)), // 淡蓝色
        Transform::from_translation(Vec3::new(0.0, 30.0, 1.0)),
        GameOverText,
    ));

    // 最高分显示
    commands.spawn((
        Text2d::new(format!("最高分: {}", game_data.high_score)),
        TextFont {
            font: assets.font.clone(),
            font_size: 32.0,
            ..default()
        },
        TextColor(Color::srgb(1.0, 0.7, 0.7)), // 淡红色
        Transform::from_translation(Vec3::new(0.0, -20.0, 1.0)),
        GameOverText,
    ));

    // 操作提示
    commands.spawn((
        Text2d::new("按 R 键重新开始"),
        TextFont {
            font: assets.font.clone(),
            font_size: 28.0,
            ..default()
        },
        TextColor(Color::srgb(0.8, 1.0, 0.8)), // 淡绿色
        Transform::from_translation(Vec3::new(0.0, -80.0, 1.0)),
        GameOverText,
    ));

    commands.spawn((
        Text2d::new("按 ESC 键返回菜单"),
        TextFont {
            font: assets.font.clone(),
            font_size: 28.0,
            ..default()
        },
        TextColor(Color::srgb(0.9, 0.9, 0.9)), // 浅灰色
        Transform::from_translation(Vec3::new(0.0, -120.0, 1.0)),
        GameOverText,
    ));
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

pub fn save_game_data(
    mut game_data: ResMut<GameData>,
    save_manager: Res<SaveManager>,
) {
    // 将当前分数添加到排行榜
    let updated_save_data = save_manager.add_score_to_leaderboard(
        game_data.save_data.clone(),
        game_data.score,
        game_data.selected_character,
    );
    
    // 保存数据到文件
    if let Err(e) = save_manager.save_data(&updated_save_data) {
        eprintln!("保存数据失败: {}", e);
    }
    
    // 更新游戏数据
    game_data.save_data = updated_save_data.clone();
    game_data.high_score = updated_save_data.high_score;
}

pub fn setup_leaderboard(
    mut commands: Commands,
    assets: Res<GameAssets>,
    game_data: Res<GameData>,
) {
    // 主标题
    commands.spawn((
        Text2d::new("🏆 排行榜 🏆"),
        TextFont {
            font: assets.font.clone(),
            font_size: 48.0,
            ..default()
        },
        TextColor(Color::srgb(1.0, 0.8, 0.0)),
        Transform::from_translation(Vec3::new(0.0, 280.0, 1.0)),
        LeaderboardText,
    ));
    
    // 左侧：排行榜标题
    commands.spawn((
        Text2d::new("前10名最高分"),
        TextFont {
            font: assets.font.clone(),
            font_size: 32.0,
            ..default()
        },
        TextColor(Color::srgb(0.9, 0.9, 1.0)),
        Transform::from_translation(Vec3::new(-300.0, 220.0, 1.0)),
        LeaderboardText,
    ));
    
    // 左侧：排行榜条目
    let leaderboard = &game_data.save_data.leaderboard;
    if leaderboard.is_empty() {
        commands.spawn((
            Text2d::new("暂无记录\n\n开始游戏创建\n你的第一个记录吧！"),
            TextFont {
                font: assets.font.clone(),
                font_size: 24.0,
                ..default()
            },
            TextColor(Color::srgb(0.7, 0.7, 0.7)),
            Transform::from_translation(Vec3::new(-300.0, 100.0, 1.0)),
            LeaderboardText,
        ));
    } else {
        for (i, entry) in leaderboard.iter().enumerate().take(10) {
            let rank_color = match i {
                0 => Color::srgb(1.0, 0.8, 0.0), // 金色
                1 => Color::srgb(0.8, 0.8, 0.8), // 银色
                2 => Color::srgb(0.8, 0.5, 0.2), // 铜色
                _ => Color::WHITE,
            };
            
            let rank_symbol = match i {
                0 => "🥇",
                1 => "🥈",
                2 => "🥉",
                _ => &format!("{:2}.", i + 1),
            };
            
            // 格式化时间戳
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            let time_diff = now.saturating_sub(entry.timestamp);
            let formatted_time = if time_diff < 60 {
                "刚刚".to_string()
            } else if time_diff < 3600 {
                format!("{}分钟前", time_diff / 60)
            } else if time_diff < 86400 {
                format!("{}小时前", time_diff / 3600)
            } else {
                format!("{}天前", time_diff / 86400)
            };
            
            commands.spawn((
                Text2d::new(format!(
                    "{} {} - {} 分\n    {}",
                    rank_symbol,
                    entry.character.get_name(),
                    entry.score,
                    formatted_time
                )),
                TextFont {
                    font: assets.font.clone(),
                    font_size: 20.0,
                    ..default()
                },
                TextColor(rank_color),
                Transform::from_translation(Vec3::new(-300.0, 170.0 - i as f32 * 40.0, 1.0)),
                LeaderboardText,
            ));
        }
    }
    
    // 右侧：统计信息
    commands.spawn((
        Text2d::new(format!(
            "📊 游戏统计\n\n总游戏次数: {}\n总得分: {}\n平均分数: {:.1}\n最高分: {}",
            game_data.save_data.total_games,
            game_data.save_data.total_score,
            if game_data.save_data.total_games > 0 {
                game_data.save_data.total_score as f32 / game_data.save_data.total_games as f32
            } else {
                0.0
            },
            game_data.save_data.high_score
        )),
        TextFont {
            font: assets.font.clone(),
            font_size: 24.0,
            ..default()
        },
        TextColor(Color::srgb(0.8, 0.9, 1.0)),
        Transform::from_translation(Vec3::new(300.0, 100.0, 1.0)),
        StatisticsText,
    ));
    
    // 返回提示
    commands.spawn((
        Text2d::new("按 ESC 键返回主菜单"),
        TextFont {
            font: assets.font.clone(),
            font_size: 24.0,
            ..default()
        },
        TextColor(Color::srgb(0.8, 0.8, 0.8)),
        Transform::from_translation(Vec3::new(0.0, -250.0, 1.0)),
        LeaderboardText,
    ));
}

pub fn leaderboard_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut audio_events: EventWriter<AudioEvent>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        audio_events.write(AudioEvent::Swoosh);
        next_state.set(GameState::Menu);
    }
}
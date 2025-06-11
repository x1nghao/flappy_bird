use bevy::prelude::*;
use bevy::input::mouse::MouseWheel;
use crate::components::*;
use crate::resources::*;
use crate::states::*;
use crate::audio::AudioEvent;

// ===== 菜单系统 =====

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
        Text2d::new("Flappy Bird"),
        TextFont {
            font: assets.font.clone(),
            font_size: 50.0,
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
    
    // 右侧排行榜标题
    commands.spawn((
        Text2d::new("排行榜"),
        TextFont {
            font: assets.font.clone(),
            font_size: 30.0,
            ..default()
        },
        TextColor(Color::srgb(1.0, 0.8, 0.0)),
        Transform::from_translation(Vec3::new(280.0, 110.0, 1.0)),
        MenuText,
    ));
    
    // 右侧排行榜内容
    let leaderboard = &game_data.save_data.leaderboard;
    if leaderboard.is_empty() {
        commands.spawn((
            Text2d::new("暂无记录\n开始游戏创建记录吧!"),
            TextFont {
                font: assets.font.clone(),
                font_size: 20.0,
                ..default()
            },
            TextColor(Color::srgb(0.8, 0.8, 0.8)),
            Transform::from_translation(Vec3::new(280.0, 30.0, 1.0)),
            MenuText,
        ));
    } else {
        for (i, entry) in leaderboard.iter().enumerate().take(5) {
            let rank_color = match i {
                0 => Color::srgb(1.0, 0.8, 0.0), // 金色
                1 => Color::srgb(0.8, 0.8, 0.8), // 银色
                2 => Color::srgb(0.8, 0.5, 0.2), // 铜色
                _ => Color::srgb(0.9, 0.9, 0.9),
            };
            
            let rank_text = match i {
                0 => "第1名".to_string(),
                1 => "第2名".to_string(),
                2 => "第3名".to_string(),
                _ => format!("第{}名", i + 1),
            };
            
            commands.spawn((
                Text2d::new(format!(
                    "{} {} - {} 分",
                    rank_text,
                    entry.character.get_name(),
                    entry.score
                )),
                TextFont {
                    font: assets.font.clone(),
                    font_size: 19.0,
                    ..default()
                },
                TextColor(rank_color),
                Transform::from_translation(Vec3::new(280.0, 65.0 - i as f32 * 30.0, 1.0)),
                MenuText,
            ));
        }
    }
    
    // 控制说明
    commands.spawn((
        Text2d::new("← → 或滚轮切换角色\n\n空格键或鼠标左键开始游戏"),
        TextFont {
            font: assets.font.clone(),
            font_size: 24.0,
            ..default()
        },
        TextColor(Color::srgb(0.8, 0.8, 0.8)),
        Transform::from_translation(Vec3::new(0.0, -120.0, 1.0)),
        MenuText,
    ));
}

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
    
    if keyboard_input.just_pressed(KeyCode::KeyL) {
        audio_events.write(AudioEvent::Swoosh);
        next_state.set(GameState::Leaderboard);
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
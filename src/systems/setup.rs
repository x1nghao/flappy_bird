use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::winit::WinitWindows;
use crate::components::*;
use crate::resources::*;

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
    
    // 加载所有小鸟的动画帧
    let bird_animation_frames = BirdCharacter::all_characters()
        .iter()
        .map(|character| {
            character.get_animation_frames()
                .iter()
                .map(|frame_path| asset_server.load(*frame_path))
                .collect()
        })
        .collect();
    
    // 加载所有管道纹理
    let pipe_textures: Vec<Handle<Image>> = PipeType::all_types()
        .iter()
        .map(|pipe_type| asset_server.load(pipe_type.get_texture_path()))
        .collect();

    // 加载数字纹理 (0-9)
    let number_textures = (0..10)
        .map(|i| asset_server.load(format!("numbers/{}.png", i)))
        .collect();

    

    commands.insert_resource(GameAssets {
        bird_textures,
        bird_animation_frames,
        pipe_texture: pipe_textures[0].clone(),
        ground_texture: asset_server.load("mountain.png"), // 暂时使用mountain.png替代
        cloud_texture: asset_server.load("cloud_1.png"),
        mountain_texture: asset_server.load("mountain.png"),
        font: asset_server.load("fonts/NotoSansSC-Regular.ttf"),
        number_textures,
    });
}

// 设置窗口图标的系统
pub fn set_window_icon(
    windows: NonSend<WinitWindows>,
) {
    // 使用image库直接加载图标文件
    if let Ok(image) = image::open("assets/icon.png") {
        let image = image.into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        
        // 创建winit图标
        if let Ok(icon) = winit::window::Icon::from_rgba(rgba, width, height) {
            // 为所有窗口设置图标
            for window in windows.windows.values() {
                window.set_window_icon(Some(icon.clone()));
            }
        }
    }
}

pub fn cleanup_menu(mut commands: Commands, query: Query<Entity, With<MenuText>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn cleanup_game(
    mut commands: Commands,
    bird_query: Query<Entity, With<Bird>>,
    pipe_query: Query<Entity, With<Pipe>>,
    score_query: Query<Entity, With<ScoreDigit>>,
    scrolling_query: Query<Entity, With<Scrolling>>,
) {
    // 清理小鸟
    for entity in bird_query.iter() {
        commands.entity(entity).despawn();
    }
    
    // 清理管道
    for entity in pipe_query.iter() {
        commands.entity(entity).despawn();
    }
    
    // 清理分数显示
    for entity in score_query.iter() {
        commands.entity(entity).despawn();
    }
    
    // 清理滚动背景元素
    for entity in scrolling_query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn cleanup_pipes(mut commands: Commands, query: Query<Entity, With<Pipe>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn cleanup_background(mut commands: Commands, query: Query<Entity, With<Scrolling>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn cleanup_game_over(mut commands: Commands, query: Query<Entity, With<GameOverText>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn cleanup_leaderboard(
    mut commands: Commands,
    leaderboard_query: Query<Entity, With<LeaderboardText>>,
    statistics_query: Query<Entity, With<StatisticsText>>,
) {
    for entity in leaderboard_query.iter() {
        commands.entity(entity).despawn();
    }
    for entity in statistics_query.iter() {
        commands.entity(entity).despawn();
    }
}
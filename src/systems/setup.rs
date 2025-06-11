use bevy::prelude::*;
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
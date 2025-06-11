mod audio;
mod states;
mod components;
mod resources;
mod systems;

use audio::*;
use states::GameState;
use components::*;
use resources::*;
use systems::*;
use bevy::prelude::*;

fn main() {
    // 初始化数据持久化管理器
    let save_manager = SaveManager::new();
    let save_data = save_manager.load_data();
    
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Flappy Bird".into(),
                resolution: (800.0, 600.0).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }).set(ImagePlugin::default_nearest()))
        .init_state::<GameState>()
        .insert_resource(ClearColor(Color::srgb(0.34, 0.75, 0.79)))
        .insert_resource(GameData {
            score: 0,
            high_score: save_data.high_score,
            selected_character: save_data.selected_character,
            save_data: save_data.clone(),
        })
        .insert_resource(save_manager)
        .insert_resource(GameConfig {
            jump_force: 400.0,
            pipe_speed: 200.0,
            pipe_gap: 150.0,
            pipe_spawn_timer: Timer::from_seconds(2.0, TimerMode::Repeating),
        })
        .add_systems(Startup, (setup_camera, load_assets, set_window_icon))
        .add_systems(
            Update,
            (
                setup_menu_when_ready.run_if(in_state(GameState::Menu)),
                menu_system.run_if(in_state(GameState::Menu)),
                character_selection_system.run_if(in_state(GameState::Menu)),
                (
                    bird_input_system,
                    bird_physics_system,
                    wing_animation_system,
                    pipe_spawn_system,
                    scrolling_system,
                    collision_system,
                    score_system,
                    number_score_display,
                )
                    .run_if(in_state(GameState::Playing)),
                game_over_system.run_if(in_state(GameState::GameOver)),
                leaderboard_system.run_if(in_state(GameState::Leaderboard)),
                restart_system,
            ),
        )
        .add_systems(OnExit(GameState::Menu), cleanup_menu)
        .add_systems(OnEnter(GameState::Playing), setup_game)
        .add_systems(OnExit(GameState::Playing), cleanup_game)
        .add_systems(OnEnter(GameState::GameOver), (setup_game_over, on_game_over, save_game_data))
        .add_systems(OnExit(GameState::GameOver), cleanup_game_over)
        .add_systems(OnEnter(GameState::Leaderboard), setup_leaderboard)
        .add_systems(OnExit(GameState::Leaderboard), cleanup_leaderboard)
        .add_plugins(AudioPlugin)
        .run();
}
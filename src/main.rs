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
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Flappy Bird".into(),
                resolution: (800.0, 600.0).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .init_state::<GameState>()
        .insert_resource(ClearColor(Color::srgb(0.34, 0.75, 0.79)))
        .insert_resource(GameData {
            score: 0,
            high_score: 0,
            selected_character: BirdCharacter::YellowBird,
        })
        .insert_resource(GameConfig {
            jump_force: 400.0,
            pipe_speed: 200.0,
            pipe_gap: 150.0,
            pipe_spawn_timer: Timer::from_seconds(2.0, TimerMode::Repeating),
        })
        .add_systems(Startup, (setup_camera, load_assets))
        .add_systems(
            Update,
            (
                setup_menu_when_ready.run_if(in_state(GameState::Menu)),
                menu_system.run_if(in_state(GameState::Menu)),
                character_selection_system.run_if(in_state(GameState::Menu)),
                (
                    bird_input_system,
                    bird_physics_system,
                    pipe_spawn_system,
                    scrolling_system,
                    collision_system,
                    score_system,
                    number_score_display,
                )
                    .run_if(in_state(GameState::Playing)),
                game_over_system.run_if(in_state(GameState::GameOver)),
                restart_system,
            ),
        )
        .add_systems(OnExit(GameState::Menu), cleanup_menu)
        .add_systems(OnEnter(GameState::Playing), setup_game)
        .add_systems(OnExit(GameState::Playing), cleanup_game)
        .add_systems(OnEnter(GameState::GameOver), (setup_game_over, on_game_over))
        .add_systems(OnExit(GameState::GameOver), cleanup_game_over)
        .add_plugins(AudioPlugin)
        .run();
}
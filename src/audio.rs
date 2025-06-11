use bevy::prelude::*;
use crate::states::GameState;
use crate::embedded_assets::AssetLoader;

#[derive(Resource)]
pub struct AudioAssets {
    pub wing: Handle<AudioSource>,
    pub point: Handle<AudioSource>,
    pub hit: Handle<AudioSource>,
    pub die: Handle<AudioSource>,
    pub swoosh: Handle<AudioSource>,
}

#[derive(Event)]
pub enum AudioEvent {
    Jump,
    Score,
    Hit,
    Die,
    Swoosh,
}

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AudioEvent>()
            .add_systems(Startup, load_audio_assets)
            .add_systems(Update, handle_audio_events);
    }
}

fn load_audio_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let audio_assets = AudioAssets {
        wing: AssetLoader::load_audio(&asset_server, "audio/wing.ogg"),
        point: AssetLoader::load_audio(&asset_server, "audio/point.ogg"),
        hit: AssetLoader::load_audio(&asset_server, "audio/hit.ogg"),
        die: AssetLoader::load_audio(&asset_server, "audio/die.ogg"),
        swoosh: AssetLoader::load_audio(&asset_server, "audio/swoosh.ogg"),
    };
    
    commands.insert_resource(audio_assets);
}

fn handle_audio_events(
    mut commands: Commands,
    mut audio_events: EventReader<AudioEvent>,
    audio_assets: Res<AudioAssets>,
) {
    for event in audio_events.read() {
        let audio_source = match event {
            AudioEvent::Jump => &audio_assets.wing,
            AudioEvent::Score => &audio_assets.point,
            AudioEvent::Hit => &audio_assets.hit,
            AudioEvent::Die => &audio_assets.die,
            AudioEvent::Swoosh => &audio_assets.swoosh,
        };
        
        commands.spawn((
            AudioPlayer::new(audio_source.clone()),
            PlaybackSettings::DESPAWN,
        ));
    }
}
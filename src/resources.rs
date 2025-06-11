use bevy::prelude::*;
use crate::components::{BirdCharacter, PipeType};

// 资源定义
#[derive(Resource)]
pub struct GameData {
    pub score: u32,
    pub high_score: u32,
    pub selected_character: BirdCharacter,
}

#[derive(Resource)]
pub struct GameConfig {
    pub jump_force: f32,
    pub pipe_speed: f32,
    pub pipe_gap: f32,
    pub pipe_spawn_timer: Timer,
}

#[derive(Resource)]
pub struct GameAssets {
    pub bird_textures: Vec<Handle<Image>>,  // 存储所有小鸟纹理
    pub pipe_textures: Vec<Handle<Image>>,  // 存储多个管道纹理
    pub ground_texture: Handle<Image>,
    pub cloud_texture: Handle<Image>,
    pub mountain_texture: Handle<Image>,
    pub font: Handle<Font>,
    pub number_textures: Vec<Handle<Image>>,
}

impl GameAssets {
    pub fn get_bird_texture(&self, character: BirdCharacter) -> Handle<Image> {
        let index = match character {
            BirdCharacter::YellowBird => 0,
            BirdCharacter::RedBird => 1,
            BirdCharacter::BlueBird => 2,
            BirdCharacter::WuSaQi => 3,
            BirdCharacter::JiYi => 4,
            BirdCharacter::XiaoBa => 5,
        };
        self.bird_textures[index].clone()
    }
    
    pub fn get_pipe_texture(&self, pipe_type: PipeType) -> Handle<Image> {
        let index = match pipe_type {
            PipeType::Green => 0,
            PipeType::Red => 1,
        };
        self.pipe_textures[index].clone()
    }
}
use bevy::prelude::*;
use crate::components::{BirdCharacter, PipeType};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

// 排行榜条目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaderboardEntry {
    pub score: u32,
    pub character: BirdCharacter,
    pub timestamp: u64, // Unix时间戳
    pub player_name: String,
}

// 持久化数据结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveData {
    pub high_score: u32,
    pub selected_character: BirdCharacter,
    pub leaderboard: Vec<LeaderboardEntry>,
    pub total_games: u32,
    pub total_score: u32,
}

impl Default for SaveData {
    fn default() -> Self {
        Self {
            high_score: 0,
            selected_character: BirdCharacter::YellowBird,
            leaderboard: Vec::new(),
            total_games: 0,
            total_score: 0,
        }
    }
}

// 游戏数据资源
#[derive(Resource)]
pub struct GameData {
    pub score: u32,
    pub high_score: u32,
    pub selected_character: BirdCharacter,
    pub save_data: SaveData,
}

// 数据持久化管理器
#[derive(Resource)]
pub struct SaveManager {
    pub save_path: PathBuf,
}

impl SaveManager {
    pub fn new() -> Self {
        let mut save_path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
        save_path.push("flappy_bird");
        save_path.push("save_data.json");
        
        // 确保目录存在
        if let Some(parent) = save_path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        
        Self { save_path }
    }
    
    pub fn load_data(&self) -> SaveData {
        match fs::read_to_string(&self.save_path) {
            Ok(content) => {
                serde_json::from_str(&content).unwrap_or_default()
            }
            Err(_) => SaveData::default(),
        }
    }
    
    pub fn save_data(&self, data: &SaveData) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(data)?;
        fs::write(&self.save_path, json)?;
        Ok(())
    }
    
    pub fn add_score_to_leaderboard(&self, mut save_data: SaveData, score: u32, character: BirdCharacter) -> SaveData {
        let entry = LeaderboardEntry {
            score,
            character,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            player_name: format!("玩家{}", character.get_name()),
        };
        
        save_data.leaderboard.push(entry);
        
        // 按分数排序，保留前10名
        save_data.leaderboard.sort_by(|a, b| b.score.cmp(&a.score));
        save_data.leaderboard.truncate(10);
        
        // 更新统计数据
        save_data.total_games += 1;
        save_data.total_score += score;
        
        if score > save_data.high_score {
            save_data.high_score = score;
        }
        
        save_data
    }
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
    pub bird_animation_frames: Vec<Vec<Handle<Image>>>,  // 存储所有小鸟的动画帧
    pub pipe_texture: Handle<Image>,
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
    
    pub fn get_bird_animation_frames(&self, character: BirdCharacter) -> Vec<Handle<Image>> {
        let index = match character {
            BirdCharacter::YellowBird => 0,
            BirdCharacter::RedBird => 1,
            BirdCharacter::BlueBird => 2,
            BirdCharacter::WuSaQi => 3,
            BirdCharacter::JiYi => 4,
            BirdCharacter::XiaoBa => 5,
        };
        self.bird_animation_frames[index].clone()
    }
    
    pub fn get_pipe_texture(&self, pipe_type: PipeType) -> Handle<Image> {
        self.pipe_texture.clone()
    }
}
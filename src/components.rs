use bevy::prelude::*;

// 小鸟角色枚举 - 支持所有6个角色
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BirdCharacter {
    YellowBird,
    RedBird,
    BlueBird,
    WuSaQi,     // 乌撒奇
    JiYi,       // 吉伊
    XiaoBa,     // 小八
}

// 小鸟组件
#[derive(Component)]
pub struct Bird {
    pub character: BirdCharacter,
}

// 管道类型枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PipeType {
    Green,
    Red,
    ColorfulGourd3,
    ColorfulGourd4,
    Lantern2,
    Lantern3,
    Gourd3,
    Gourd5,
}

// 管道组件
#[derive(Component)]
pub struct Pipe {
    pub pipe_type: PipeType,
}

// 环境组件
#[derive(Component)]
pub struct Ground;

#[derive(Component)]
pub struct Cloud;

#[derive(Component)]
pub struct Mountain;

// 物理组件 - 移除未使用的 x 字段
#[derive(Component)]
pub struct Velocity {
    pub y: f32,  // 只保留 y 轴速度，因为 x 轴未使用
}

#[derive(Component)]
pub struct Gravity(pub f32);

#[derive(Component)]
pub struct Collider;

#[derive(Component)]
pub struct Scrolling {
    pub speed: f32,
}

// UI组件
#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub struct MenuText;

#[derive(Component)]
pub struct CharacterPreview;

#[derive(Component)]
pub struct GameOverText;

#[derive(Component)]
pub struct ScoreDigit;

impl PipeType {
    pub fn get_texture_path(&self) -> &'static str {
        match self {
            PipeType::Green => "pipes/pipe-green.png",
            PipeType::Red => "pipes/pipe-red.png",
            PipeType::ColorfulGourd3 => "pipes/彩葫芦串3.png",
            PipeType::ColorfulGourd4 => "pipes/彩葫芦串4.png",
            PipeType::Lantern2 => "pipes/灯笼2.png",
            PipeType::Lantern3 => "pipes/灯笼3.png",
            PipeType::Gourd3 => "pipes/葫芦串3.png",
            PipeType::Gourd5 => "pipes/葫芦串5.png",
        }
    }
    
    pub fn get_scale(&self) -> f32 {
        match self {
            // 英文管道使用标准缩放
            PipeType::Green | PipeType::Red => 1.0,
            // 中文装饰物需要调整缩放比例 - 进一步缩小
            PipeType::ColorfulGourd3 | PipeType::ColorfulGourd4 => 0.5,  // 从0.8改为0.5
            PipeType::Lantern2 | PipeType::Lantern3 => 0.4,  // 从0.6改为0.4
            PipeType::Gourd3 | PipeType::Gourd5 => 0.5,  // 从0.7改为0.5
        }
    }
    
    pub fn get_collision_bounds(&self) -> (f32, f32) {
        // 返回 (width_factor, height_factor) 相对于原始尺寸的比例
        match self {
            PipeType::Green | PipeType::Red => (0.8, 0.9), // 英文管道，减少透明区域
            PipeType::ColorfulGourd3 | PipeType::ColorfulGourd4 => (0.6, 0.7), // 糖葫芦形状不规则
            PipeType::Lantern2 | PipeType::Lantern3 => (0.7, 0.8), // 灯笼中间较宽
            PipeType::Gourd3 | PipeType::Gourd5 => (0.6, 0.8), // 葫芦串形状
        }
    }
    
    pub fn get_collision_offset(&self) -> (f32, f32) {
        // 返回碰撞中心相对于图片中心的偏移
        match self {
            PipeType::Green | PipeType::Red => (0.0, 0.0), // 管道居中
            PipeType::ColorfulGourd3 | PipeType::ColorfulGourd4 => (0.0, -5.0), // 糖葫芦重心偏下
            PipeType::Lantern2 | PipeType::Lantern3 => (0.0, 0.0), // 灯笼居中
            PipeType::Gourd3 | PipeType::Gourd5 => (0.0, -3.0), // 葫芦串重心偏下
        }
    }
    
    pub fn all_types() -> [PipeType; 8] {
        [
            PipeType::Green,
            PipeType::Red,
            PipeType::ColorfulGourd3,
            PipeType::ColorfulGourd4,
            PipeType::Lantern2,
            PipeType::Lantern3,
            PipeType::Gourd3,
            PipeType::Gourd5,
        ]
    }
}

impl BirdCharacter {
    pub fn get_texture_path(&self) -> &'static str {
        match self {
            BirdCharacter::YellowBird => "birds/yellowbird-midflap.png",
            BirdCharacter::RedBird => "birds/redbird-midflap.png",
            BirdCharacter::BlueBird => "birds/bluebird-midflap.png",
            BirdCharacter::WuSaQi => "birds/乌撒奇.png",
            BirdCharacter::JiYi => "birds/吉伊.png",
            BirdCharacter::XiaoBa => "birds/小八.png",
        }
    }
    
    pub fn get_name(&self) -> &'static str {
        match self {
            BirdCharacter::YellowBird => "Yellow Bird",
            BirdCharacter::RedBird => "Red Bird",
            BirdCharacter::BlueBird => "Blue Bird",
            BirdCharacter::WuSaQi => "乌撒奇",
            BirdCharacter::JiYi => "吉伊",
            BirdCharacter::XiaoBa => "小八",
        }
    }
    
    pub fn get_scale(&self) -> f32 {
        match self {
            // 英文小鸟使用更大的缩放
            BirdCharacter::YellowBird | BirdCharacter::RedBird | BirdCharacter::BlueBird => 2.0,
            // 中文角色使用更小的缩放
            BirdCharacter::WuSaQi | BirdCharacter::JiYi | BirdCharacter::XiaoBa => 0.3,
        }
    }
    
    pub fn previous(&self) -> BirdCharacter {
        match self {
            BirdCharacter::YellowBird => BirdCharacter::XiaoBa,
            BirdCharacter::RedBird => BirdCharacter::YellowBird,
            BirdCharacter::BlueBird => BirdCharacter::RedBird,
            BirdCharacter::WuSaQi => BirdCharacter::BlueBird,
            BirdCharacter::JiYi => BirdCharacter::WuSaQi,
            BirdCharacter::XiaoBa => BirdCharacter::JiYi,
        }
    }
    
    pub fn next(&self) -> BirdCharacter {
        match self {
            BirdCharacter::YellowBird => BirdCharacter::RedBird,
            BirdCharacter::RedBird => BirdCharacter::BlueBird,
            BirdCharacter::BlueBird => BirdCharacter::WuSaQi,
            BirdCharacter::WuSaQi => BirdCharacter::JiYi,
            BirdCharacter::JiYi => BirdCharacter::XiaoBa,
            BirdCharacter::XiaoBa => BirdCharacter::YellowBird,
        }
    }
    
    pub fn all_characters() -> [BirdCharacter; 6] {
        [
            BirdCharacter::YellowBird,
            BirdCharacter::RedBird,
            BirdCharacter::BlueBird,
            BirdCharacter::WuSaQi,
            BirdCharacter::JiYi,
            BirdCharacter::XiaoBa,
        ]
    }
    
    pub fn get_collision_radius(&self) -> f32 {
        match self {
            // 英文小鸟使用标准碰撞半径
            BirdCharacter::YellowBird | BirdCharacter::RedBird | BirdCharacter::BlueBird => 12.0,
            // 中文角色可能需要不同的碰撞半径
            BirdCharacter::WuSaQi | BirdCharacter::JiYi | BirdCharacter::XiaoBa => 10.0,
        }
    }
}
use bevy::prelude::*;
use crate::components::*;
use crate::states::*;
use crate::audio::AudioEvent;

// ===== 物理和碰撞系统 =====

pub fn bird_physics_system(
    time: Res<Time>,
    mut bird_query: Query<(&mut Transform, &mut Velocity, &Gravity), With<Bird>>,
) {
    for (mut transform, mut velocity, gravity) in bird_query.iter_mut() {
        // 应用重力
        velocity.y -= gravity.0 * time.delta_secs();
        
        // 更新位置
        transform.translation.y += velocity.y * time.delta_secs();
        
        // 限制小鸟旋转角度
        let angle = (velocity.y / 300.0).clamp(-1.0, 1.0) * 0.5;
        transform.rotation = Quat::from_rotation_z(angle);
    }
}

pub fn collision_system(
    bird_query: Query<(&Transform, &Bird), With<Collider>>,
    pipe_query: Query<(&Transform, &Pipe), (With<Collider>, Without<Bird>)>,
    mut next_state: ResMut<NextState<GameState>>,
    mut audio_events: EventWriter<AudioEvent>,
) {
    for (bird_transform, bird) in bird_query.iter() {
        let bird_radius = bird.character.get_collision_radius();
        
        // 检查边界碰撞
        if bird_transform.translation.y - bird_radius < -280.0 
            || bird_transform.translation.y + bird_radius > 280.0 {
            audio_events.write(AudioEvent::Hit);
            next_state.set(GameState::GameOver);
            return;
        }
        
        // 改进的管道碰撞检测
        for (pipe_transform, pipe) in pipe_query.iter() {
            if check_pipe_collision(bird_transform, bird, pipe_transform, pipe) {
                audio_events.write(AudioEvent::Hit);
                next_state.set(GameState::GameOver);
                return;
            }
        }
    }
}

// 专门的管道碰撞检测函数
fn check_pipe_collision(
    bird_transform: &Transform,
    bird: &Bird,
    pipe_transform: &Transform,
    pipe: &Pipe,
) -> bool {
    let bird_radius = bird.character.get_collision_radius();
    let pipe_scale = pipe.pipe_type.get_scale();
    let (base_offset_x, base_offset_y) = pipe.pipe_type.get_collision_offset();
    
    // 小鸟边界
    let bird_left = bird_transform.translation.x - bird_radius;
    let bird_right = bird_transform.translation.x + bird_radius;
    let bird_bottom = bird_transform.translation.y - bird_radius;
    let bird_top = bird_transform.translation.y + bird_radius;
    
    // 管道碰撞中心（考虑偏移）
    let pipe_center_x = pipe_transform.translation.x + base_offset_x * pipe_scale;
    let pipe_center_y = pipe_transform.translation.y + base_offset_y * pipe_scale;
    
    // 首先检查X轴是否重叠
    let (width_factor, _) = pipe.pipe_type.get_collision_bounds();
    let pipe_width = 52.0 * pipe_scale * width_factor;
    let pipe_left = pipe_center_x - pipe_width / 2.0;
    let pipe_right = pipe_center_x + pipe_width / 2.0;
    
    // 如果X轴没有重叠，直接返回false
    if bird_right <= pipe_left || bird_left >= pipe_right {
        return false;
    }
    
    // X轴重叠时，使用基于高度区间的碰撞检测
    if pipe.pipe_type.use_precise_collision() {
        // 使用多段精确碰撞检测
        let segments = pipe.pipe_type.get_collision_segments();
        
        for (seg_offset_x, seg_offset_y, width_factor, height_factor) in segments {
            // 计算每个碰撞段的实际位置和尺寸
            let segment_width = 52.0 * pipe_scale * width_factor;
            let segment_height = 320.0 * pipe_scale * height_factor;
            
            // 段的中心位置（基础偏移 + 段偏移）
            // 修正：seg_offset应该直接乘以pipe_scale，而不是先乘以320.0
            let segment_center_x = pipe_transform.translation.x + 
                (base_offset_x + seg_offset_x * 52.0) * pipe_scale;
            let segment_center_y = pipe_transform.translation.y + 
                (base_offset_y + seg_offset_y * 160.0) * pipe_scale;
            
            // 段的边界
            let segment_left = segment_center_x - segment_width / 2.0;
            let segment_right = segment_center_x + segment_width / 2.0;
            let segment_bottom = segment_center_y - segment_height / 2.0;
            let segment_top = segment_center_y + segment_height / 2.0;
            
            // 检查与当前段的碰撞（基于高度区间）
            if bird_right > segment_left && bird_left < segment_right {
                // X轴重叠时，检查Y轴是否在障碍物的高度范围内
                if bird_top > segment_bottom && bird_bottom < segment_top {
                    return true; // 在障碍物高度范围内，发生碰撞
                }
            }
        }
        false // 不在任何障碍物段的高度范围内，可以通过
    } else {
        // 传统管道使用简单AABB碰撞检测
        let (_, height_factor) = pipe.pipe_type.get_collision_bounds();
        
        // 计算管道的实际碰撞区域
        let pipe_height = 320.0 * pipe_scale * height_factor;
        
        // 管道边界
        let pipe_bottom = pipe_center_y - pipe_height / 2.0;
        let pipe_top = pipe_center_y + pipe_height / 2.0;
        
        // 基于高度区间的碰撞检测
        bird_top > pipe_bottom && bird_bottom < pipe_top
    }
}
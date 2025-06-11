//! 嵌入式资源加载模块
//! 为Windows平台提供资源嵌入功能，其他平台使用文件系统加载

use bevy::prelude::*;

#[cfg(target_os = "windows")]
use rust_embed::RustEmbed;

#[cfg(target_os = "windows")]
#[derive(RustEmbed)]
#[folder = "assets/"]
struct EmbeddedAssets;

/// 跨平台资源加载器
pub struct AssetLoader;

impl AssetLoader {
    /// 加载图像资源
    pub fn load_image(asset_server: &AssetServer, path: &str) -> Handle<Image> {
        #[cfg(target_os = "windows")]
        {
            // Windows平台：尝试从嵌入资源加载
            if let Some(embedded_data) = EmbeddedAssets::get(path) {
                // 从嵌入的字节数据创建图像
                if let Ok(image) = image::load_from_memory(&embedded_data.data) {
                    let image_rgba8 = image.to_rgba8();
                    let (width, height) = image_rgba8.dimensions();
                    
                    let bevy_image = Image::new(
                        bevy::render::render_resource::Extent3d {
                            width,
                            height,
                            depth_or_array_layers: 1,
                        },
                        bevy::render::render_resource::TextureDimension::D2,
                        image_rgba8.into_raw(),
                        bevy::render::render_resource::TextureFormat::Rgba8UnormSrgb,
                        bevy::render::render_asset::RenderAssetUsages::RENDER_WORLD,
                    );
                    
                    return asset_server.add(bevy_image);
                }
            }
            
            // 回退到文件系统加载
            asset_server.load(path)
        }
        
        #[cfg(not(target_os = "windows"))]
        {
            // 其他平台：直接从文件系统加载
            asset_server.load(path)
        }
    }
    
    /// 加载字体资源
    pub fn load_font(asset_server: &AssetServer, path: &str) -> Handle<Font> {
        #[cfg(target_os = "windows")]
        {
            // Windows平台：尝试从嵌入资源加载
            if let Some(embedded_data) = EmbeddedAssets::get(path) {
                return asset_server.add(Font::try_from_bytes(embedded_data.data.to_vec()).unwrap());
            }
            
            // 回退到文件系统加载
            asset_server.load(path)
        }
        
        #[cfg(not(target_os = "windows"))]
        {
            // 其他平台：直接从文件系统加载
            asset_server.load(path)
        }
    }
    
    /// 加载音频资源
    pub fn load_audio(asset_server: &AssetServer, path: &str) -> Handle<AudioSource> {
        #[cfg(target_os = "windows")]
        {
            // Windows平台：尝试从嵌入资源加载
            if let Some(_embedded_data) = EmbeddedAssets::get(path) {
                // 注意：Bevy的音频系统可能需要特殊处理
                // 这里先回退到文件系统加载
                // 未来可以考虑实现自定义音频源
            }
            
            // 回退到文件系统加载
            asset_server.load(path)
        }
        
        #[cfg(not(target_os = "windows"))]
        {
            // 其他平台：直接从文件系统加载
            asset_server.load(path)
        }
    }
    
    /// 获取嵌入的原始数据（用于图标等特殊用途）
    pub fn get_raw_data(path: &str) -> Option<Vec<u8>> {
        #[cfg(target_os = "windows")]
        {
            EmbeddedAssets::get(path).map(|data| data.data.to_vec())
        }
        
        #[cfg(not(target_os = "windows"))]
        {
            // 其他平台：从文件系统读取
            std::fs::read(format!("assets/{}", path)).ok()
        }
    }
    
    /// 检查资源是否存在
    pub fn exists(path: &str) -> bool {
        #[cfg(target_os = "windows")]
        {
            EmbeddedAssets::get(path).is_some() || std::path::Path::new(&format!("assets/{}", path)).exists()
        }
        
        #[cfg(not(target_os = "windows"))]
        {
            std::path::Path::new(&format!("assets/{}", path)).exists()
        }
    }
}

/// 便捷宏：加载图像资源
#[macro_export]
macro_rules! load_image {
    ($asset_server:expr, $path:expr) => {
        crate::embedded_assets::AssetLoader::load_image($asset_server, $path)
    };
}

/// 便捷宏：加载字体资源
#[macro_export]
macro_rules! load_font {
    ($asset_server:expr, $path:expr) => {
        crate::embedded_assets::AssetLoader::load_font($asset_server, $path)
    };
}

/// 便捷宏：加载音频资源
#[macro_export]
macro_rules! load_audio {
    ($asset_server:expr, $path:expr) => {
        crate::embedded_assets::AssetLoader::load_audio($asset_server, $path)
    };
}
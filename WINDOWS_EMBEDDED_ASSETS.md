# Windows 嵌入式资源功能说明

本项目已实现了 Windows 平台的资源嵌入功能，可以将所有游戏资源（图片、字体、音频等）打包到可执行文件中，实现单文件分发。

## 功能特性

### 跨平台支持
- **Windows**: 资源嵌入到可执行文件中，支持单文件分发
- **macOS/Linux**: 继续使用外部文件系统加载资源

### 自动回退机制
- Windows 平台优先从嵌入资源加载
- 如果嵌入资源不存在，自动回退到文件系统加载
- 其他平台直接使用文件系统加载

## 技术实现

### 核心组件

1. **嵌入式资源模块** (`src/embedded_assets.rs`)
   - 使用 `rust-embed` 库嵌入 `assets/` 目录
   - 提供统一的资源加载接口
   - 支持图片、字体、音频资源

2. **条件编译**
   ```toml
   [target.'cfg(target_os = "windows")'.dependencies]
   rust-embed = "8.0"
   ```

3. **资源加载器** (`AssetLoader`)
   - `load_image()`: 加载图片资源
   - `load_font()`: 加载字体资源
   - `load_audio()`: 加载音频资源
   - `get_raw_data()`: 获取原始字节数据

### 使用方法

#### 在代码中使用
```rust
use crate::embedded_assets::AssetLoader;

// 加载图片
let texture = AssetLoader::load_image(&asset_server, "bird/yellow_bird.png");

// 加载字体
let font = AssetLoader::load_font(&asset_server, "fonts/NotoSansSC-Regular.ttf");

// 加载音频
let audio = AssetLoader::load_audio(&asset_server, "audio/wing.ogg");

// 获取原始数据（如图标）
if let Some(icon_data) = AssetLoader::get_raw_data("icon.png") {
    // 处理图标数据
}
```

#### 便捷宏
```rust
// 使用宏简化调用
let texture = load_image!(&asset_server, "bird/yellow_bird.png");
let font = load_font!(&asset_server, "fonts/NotoSansSC-Regular.ttf");
let audio = load_audio!(&asset_server, "audio/wing.ogg");
```

## 构建说明

### 本地构建

#### Windows 平台
```bash
# 确保 assets 目录存在
ls assets/

# 构建（资源会自动嵌入）
cargo build --release

# 生成的可执行文件包含所有资源
./target/release/flappy_bird.exe
```

#### 其他平台
```bash
# 正常构建，使用外部资源文件
cargo build --release

# 需要确保 assets 目录与可执行文件在同一位置
cp -r assets target/release/
./target/release/flappy_bird
```

### CI/CD 构建

GitHub Actions 工作流已更新，支持：
- Windows: 自动嵌入资源，生成单文件可执行程序
- macOS: 创建 `.app` 包，包含资源和图标
- Linux: 创建 tar.gz 包，包含可执行文件和资源目录

## 资源管理

### 支持的资源类型
- **图片**: PNG, JPG, JPEG（通过 `image` 库）
- **字体**: TTF, OTF（通过 Bevy 字体系统）
- **音频**: OGG, WAV, MP3（通过 Bevy 音频系统）

### 资源路径
所有资源路径相对于 `assets/` 目录：
```
assets/
├── bird/
│   ├── yellow_bird.png
│   └── ...
├── pipes/
│   └── ...
├── fonts/
│   └── NotoSansSC-Regular.ttf
├── audio/
│   ├── wing.ogg
│   └── ...
└── icon.png
```

## 性能考虑

### 内存使用
- 嵌入的资源在编译时包含在二进制文件中
- 运行时按需加载到内存
- 对于大型资源，建议考虑压缩

### 文件大小
- Windows 可执行文件会包含所有资源
- 当前资源总大小约 2-5MB
- 可以通过优化图片和音频格式减小体积

## 故障排除

### 常见问题

1. **编译错误：找不到 assets 目录**
   ```bash
   # 确保 assets 目录存在
   mkdir -p assets
   ```

2. **资源加载失败**
   - 检查资源路径是否正确
   - 确认资源文件存在于 `assets/` 目录中
   - 查看控制台错误信息

3. **Windows 平台资源未嵌入**
   - 确认 `rust-embed` 依赖已正确添加
   - 检查条件编译配置
   - 重新构建项目

### 调试方法

```rust
// 检查资源是否存在
if AssetLoader::exists("bird/yellow_bird.png") {
    println!("资源存在");
} else {
    println!("资源不存在");
}

// 获取原始数据进行调试
if let Some(data) = AssetLoader::get_raw_data("icon.png") {
    println!("图标大小: {} 字节", data.len());
}
```

## 未来改进

1. **压缩支持**: 添加资源压缩以减小文件大小
2. **选择性嵌入**: 允许配置哪些资源需要嵌入
3. **加密支持**: 对嵌入的资源进行加密保护
4. **热重载**: 开发模式下支持资源热重载

## 相关文件

- `src/embedded_assets.rs`: 嵌入式资源加载器
- `src/systems/setup.rs`: 资源加载系统
- `src/audio.rs`: 音频资源加载
- `Cargo.toml`: 依赖配置
- `.github/workflows/release.yml`: CI/CD 配置
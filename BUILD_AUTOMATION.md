# 构建自动化说明

本项目已配置了完整的跨平台自动化构建流程，包括应用图标的自动生成和打包。

## 自动化功能

### 1. 跨平台构建
- **Linux (x86_64)**: 标准可执行文件 + assets
- **Windows (x86_64)**: 嵌入图标的可执行文件 + assets
- **macOS (x86_64 & ARM64)**: 完整的 .app 应用包

### 2. 图标自动化处理

#### macOS 平台
- 自动从 `assets/favicon.ico` 生成 PNG 格式图标
- 创建完整的 iconset（包含所有必需尺寸）
- 生成标准的 .icns 文件
- 创建符合 macOS 标准的应用包结构

#### Windows 平台
- 使用 `winres` 将图标嵌入到可执行文件中
- 支持 .ico 格式图标

#### Linux 平台
- 运行时通过 Bevy/winit 设置窗口图标
- 使用 PNG 格式图标

### 3. 触发条件

自动化构建在以下情况下触发：
- 推送到 `main` 或 `master` 分支
- 创建以 `v` 开头的标签（如 `v1.0.0`）
- 创建 Pull Request

### 4. 发布流程

当推送标签时（如 `git tag v1.0.0 && git push origin v1.0.0`）：
1. 自动构建所有平台的版本
2. 生成对应的图标和应用包
3. 创建 GitHub Release
4. 上传所有构建产物

## 文件结构

构建完成后的文件结构：

```
artifacts/
├── flappy_bird-linux-x86_64.tar.gz     # Linux 版本
├── flappy_bird-windows-x86_64.zip      # Windows 版本
├── flappy_bird-macos-x86_64.tar.gz     # macOS Intel 版本
└── flappy_bird-macos-aarch64.tar.gz    # macOS Apple Silicon 版本
```

### macOS 应用包内容
```
FlappyBird.app/
├── Contents/
│   ├── Info.plist              # 应用元数据
│   ├── MacOS/
│   │   ├── flappy_bird         # 可执行文件
│   │   └── assets/             # 游戏资源
│   └── Resources/
│       └── AppIcon.icns        # 应用图标
```

## 本地测试

如果需要在本地测试自动化流程：

```bash
# 测试 macOS 应用包创建
./scripts/build_macos_app.sh

# 测试 Windows 图标嵌入
cargo build --release --target x86_64-pc-windows-msvc
```

## 自定义配置

### 修改应用信息
编辑 `.github/workflows/release.yml` 中的 Info.plist 部分：
- `CFBundleIdentifier`: 应用包标识符
- `CFBundleName`: 应用显示名称
- `CFBundleVersion`: 版本号

### 更换图标
替换 `assets/favicon.ico` 文件，自动化流程会处理格式转换。

## 依赖说明

- **macOS**: 使用系统自带的 `sips` 和 `iconutil` 工具
- **Windows**: 使用 `winres` crate 进行图标嵌入
- **Linux**: 运行时通过 `image` crate 加载图标
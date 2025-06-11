#!/bin/bash

# macOS 应用包构建脚本
# 用于本地测试自动化构建流程

set -e

echo "🚀 开始构建 macOS 应用包..."

# 检查是否在 macOS 上运行
if [[ "$OSTYPE" != "darwin"* ]]; then
    echo "❌ 此脚本只能在 macOS 上运行"
    exit 1
fi

# 构建项目
echo "📦 编译项目..."
cargo build --release

# 转换图标格式
echo "🎨 处理应用图标..."
if [ ! -f assets/icon.png ]; then
    echo "   转换 favicon.ico 到 PNG 格式..."
    sips -s format png assets/favicon.ico --out assets/icon.png
fi

# 创建 iconset
echo "   创建 iconset..."
rm -rf AppIcon.iconset
mkdir -p AppIcon.iconset

# 生成不同尺寸的图标
sips -z 16 16 assets/icon.png --out AppIcon.iconset/icon_16x16.png
sips -z 32 32 assets/icon.png --out AppIcon.iconset/icon_32x32.png
sips -z 32 32 assets/icon.png --out AppIcon.iconset/icon_16x16@2x.png
sips -z 64 64 assets/icon.png --out AppIcon.iconset/icon_32x32@2x.png
sips -z 128 128 assets/icon.png --out AppIcon.iconset/icon_128x128.png
sips -z 256 256 assets/icon.png --out AppIcon.iconset/icon_128x128@2x.png
sips -z 256 256 assets/icon.png --out AppIcon.iconset/icon_256x256.png
sips -z 512 512 assets/icon.png --out AppIcon.iconset/icon_256x256@2x.png
sips -z 512 512 assets/icon.png --out AppIcon.iconset/icon_512x512.png
sips -z 1024 1024 assets/icon.png --out AppIcon.iconset/icon_512x512@2x.png

# 转换为 icns
echo "   生成 ICNS 文件..."
iconutil -c icns AppIcon.iconset

# 创建应用包结构
echo "📱 创建应用包结构..."
rm -rf FlappyBird.app
mkdir -p FlappyBird.app/Contents/MacOS
mkdir -p FlappyBird.app/Contents/Resources

# 复制可执行文件
echo "   复制可执行文件..."
cp target/release/flappy_bird FlappyBird.app/Contents/MacOS/

# 复制资源文件
echo "   复制游戏资源..."
cp -r assets FlappyBird.app/Contents/MacOS/

# 复制图标
echo "   复制应用图标..."
cp AppIcon.icns FlappyBird.app/Contents/Resources/

# 创建 Info.plist
echo "   创建 Info.plist..."
cat > FlappyBird.app/Contents/Info.plist << EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleExecutable</key>
    <string>flappy_bird</string>
    <key>CFBundleIdentifier</key>
    <string>com.example.flappybird</string>
    <key>CFBundleName</key>
    <string>Flappy Bird</string>
    <key>CFBundleVersion</key>
    <string>1.0</string>
    <key>CFBundleShortVersionString</key>
    <string>1.0</string>
    <key>CFBundleInfoDictionaryVersion</key>
    <string>6.0</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleIconFile</key>
    <string>AppIcon</string>
    <key>LSMinimumSystemVersion</key>
    <string>10.12</string>
</dict>
</plist>
EOF

# 设置可执行权限
chmod +x FlappyBird.app/Contents/MacOS/flappy_bird

echo "✅ macOS 应用包创建完成！"
echo "📍 位置: $(pwd)/FlappyBird.app"
echo "🎮 运行: open FlappyBird.app"
echo "📦 打包: tar -czf FlappyBird-macos.tar.gz FlappyBird.app"

# 清理临时文件
echo "🧹 清理临时文件..."
rm -rf AppIcon.iconset

echo "🎉 构建完成！"
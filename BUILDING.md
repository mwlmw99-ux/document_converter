# 构建说明

当前平板 Termux/PRoot 环境仅用于编辑，不能作为发布构建机。

## Android
在 Linux/Windows/macOS 正式开发机安装 Flutter stable、Android SDK/Build Tools、NDK、CMake、JDK 17 和 Rust stable；配置 Android Rust targets 后，在 `apps/flutter_app` 执行 `flutter build apk` 或 `flutter build appbundle`。

## 桌面
Windows 在 Windows + Visual Studio 构建；macOS 必须在 macOS + Xcode 构建；Linux 建议 Ubuntu LTS + GCC/CMake 构建。

构建前必须生成完整 Flutter 平台目录（`flutter create .`），并接入 Rust FFI；当前仓库保留轻量骨架，避免在无工具链环境中产生未经验证的生成文件。

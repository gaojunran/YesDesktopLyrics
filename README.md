## YesDesktopLyrics

一个极简的命令行工具 + Hammerspoon 解决方案，用于在 macOS 上显示 [YesPlayMusic](https://github.com/qier222/YesPlayMusic) 的歌词。

![screenshot](<screenshot.png>)

### 二进制文件发布

如果您是 ARM 架构的 MacOS ，可以直接在 Release 中下载二进制文件并加入 PATH 中。

接着参阅[使用方法](#使用方法)。

### 手动编译

若您电脑中有 Rust 环境，我们更推荐手动编译。克隆源码后运行：

```bash
cargo install --path .
```
测试一下，在 YesPlayMusic 播放歌曲时，在 bash 中运行`yes line`，如果正确地输出了当前歌词，则说明工具运转正常。

### 使用方法

1. 安装[hammerspoon](https://www.hammerspoon.org/)。如您使用 Homebrew，则运行：

```bash
brew install hammerspoon
```
2. 将本仓库中的 [mac-menu-bar.lua](./mac-menu-bar.lua) 文件复制到 `~/.hammerspoon/` 目录下，并重命名为`lyrics.lua`。在`~/.hammerspoon/init.lua`中添加以下代码：

```lua
require("lyrics")
```

3. 在 Hammerspoon 工具中刷新配置（Reload Config）。

### 贡献代码

本项目设计为一个命令行工具，运行`yes line`可以从 YesPlayMusic 开放的本地 API 中得到当前歌词，可以轻松在各个工具中集成。

如您有 Windows 或 Linux 平台上类似 Hammerspoon 工具的脚本，可以在这两个平台的状态栏上显示歌词，欢迎给我提交 Pull Request！

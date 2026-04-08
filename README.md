<!-- markdownlint-disable MD033 MD041 MD045 -->
<p align="center">
    <img style="height:240px;width:240px" src="https://s2.loli.net/2025/03/10/GSsjOcHqdtBkyu9.png" alt="OsynicPad"/>
</p>

<div align="center">

# 🎮 Osynic Pad

**Rust 编写的游戏手柄到键盘映射工具**

![Rust](https://img.shields.io/badge/Rust-1.85+-blue?style=flat-square)
![License](https://img.shields.io/badge/License-MIT-green?style=flat-square)
![Crates.io](https://img.shields.io/crates/v/osynic_pad?style=flat-square)

[English](README_EN.md) | 中文

</div>

---

## 📋 简介

**Osynic Pad** 是一个功能强大的游戏手柄到键盘映射工具库，使用 Rust 编写。它支持 Xbox、PlayStation 等多种主流游戏手柄，通过交互式菜单和灵活的配置系统，让你轻松将手柄按键映射到键盘事件。

### 🎯 核心特性

- 🎮 **广泛的手柄支持** - 兼容 Xbox、PlayStation、Nintendo Switch 等主流游戏手柄
- 📋 **灵活的配置管理** - 三种配置获取方式：使用现有配置、导入外部配置、创建新配置
- ⚡ **高性能映射** - 响应延迟 < 5ms
- 🐛 **调试模式** - 详细的事件日志便于诊断测试
- 🎯 **多模式支持** - 支持自定义按键映射和多个映射模式切换
- 🌈 **友好的 CLI 界面** - 使用 `inquire` 库提供直观的交互式菜单

## 🚀 快速开始

### 安装

**假设你已安装 Rust 1.85+**

```bash
# 从源码编译
git clone https://github.com/osynicite/osynic_pad
cd osynic_pad
cargo build --release

# 运行程序
./target/release/osynic-pad
```

或直接使用 cargo install：

```bash
cargo install osynic_pad
```

### 基本使用

启动程序后，会自动弹出交互式菜单：

#### 1️⃣ 配置选择步骤

```
🎮 Osynic Pad 配置选择

请选择配置来源:
→ 📂 使用现有配置
  📥 导入配置（从指定位置）
  ✨ 新建配置（交互式）
```

**三种选择说明：**

- **📂 使用现有配置** - 选择 `configs/` 目录下已有的配置文件
- **📥 导入配置** - 从任意位置导入 JSON 配置文件（自动复制到项目）
- **✨ 新建配置** - 交互式创建新配置文件

#### 2️⃣ Debug 模式选择

```
🐛 启用 Debug 模式? (y/n)
```

#### 3️⃣ 开始映射

配置完成后，程序即开始监听手柄输入，将按键映射到键盘事件。

## ⚙️ 配置文件格式

配置文件放在 `configs/` 目录，采用 JSON 格式。

### 配置文件示例

```json
{
  "default_mode": "Default",
  "mappings": {
    "Default": {
      "A": "Space",
      "B": "Escape",
      "X": "w",
      "Y": "s",
      "LB": "q",
      "RB": "e",
      "Start": "Return",
      "Back": "Tab"
    },
    "Alternative": {
      "A": "z",
      "B": "x",
      "X": "c",
      "Y": "v"
    }
  }
}
```

### 支持的按键

#### 手柄按键
- `A`, `B`, `X`, `Y` - 四个主要按键
- `Start`, `Back` - 开始和返回按键
- `LB`, `RB` - 左右 bumper
- `LT`, `RT` - 左右 trigger（模拟量，转化为离散值）
- `LS`, `RS` - 左右摇杆按键
- `DPadUp`, `DPadDown`, `DPadLeft`, `DPadRight` - 方向键

#### 键盘输出
支持所有标准键盘按键，如：`a-z`, `0-9`, `Space`, `Return`, `Escape` 等。

## 📚 库的使用

将 `osynic_pad` 作为库集成到你的 Rust 项目：

```toml
[dependencies]
osynic_pad = "0.1.0"
tokio = { version = "1.51", features = ["sync", "rt-multi-thread"] }
```

### 代码示例

```rust
use osynic_pad::{Config, GamepadMapper, MappingMode};
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 加载配置
    let config = Config::load_from_path(&"configs/my_config.json".into())?;
    
    // 创建键盘输入句柄
    let enigo = Arc::new(Mutex::new(
        enigo::Enigo::new(&enigo::Settings::default())?
    ));
    
    // 创建映射器
    let mapper = Arc::new(GamepadMapper::new(
        config,
        enigo,
        MappingMode::Default,
        false, // debug 模式
    ));
    
    // 现在可以使用 mapper 处理手柄事件...
    Ok(())
}
```

## 🏗️ 项目结构

```
osynic_pad/
├── src/
│   ├── lib.rs          # 库入口
│   ├── main.rs         # CLI 二进制程序入口
│   ├── cli.rs          # 交互式 CLI 菜单
│   ├── config.rs       # 配置文件处理
│   ├── events.rs       # 手柄事件定义
│   ├── mapper.rs       # 核心映射逻辑
│   └── error.rs        # 错误类型定义
├── configs/            # 配置文件目录
│   ├── pad_config.json
│   └── ...
├── examples/           # 使用示例
└── Cargo.toml         # 项目配置
```

## 📚 API 文档

### 主要类型和函数

- `Config` - 配置文件结构
- `GamepadMapper` - 手柄事件映射器
- `MappingMode` - 映射模式枚举
- `PadEvent` - 手柄事件类型

详细 API 文档可通过 `cargo doc --open` 查看。

## 🔧 开发

### 前置要求

- Rust 1.85.0 或更高版本
- Cargo

### 本地构建

```bash
cargo build
cargo build --release
```

### 运行测试

```bash
cargo test
```

### 代码检查

```bash
cargo clippy
cargo fmt
```

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

请确保：
- 遵循 Rust 社区编码规范
- 通过所有 clippy 检查
- 代码已格式化

## 📜 协议

本项目采用 [MIT License](LICENSE) 开源，详见 LICENSE 文件。

## 🙏 致谢

本项目依赖以下优秀的开源库：

- [gilrs](https://gitlab.com/gilrs-project/gilrs) - 手柄输入
- [enigo](https://github.com/enigo-rs/enigo) - 键盘输出模拟
- [inquire](https://github.com/mikaelmello/inquire) - 交互式 CLI
- [serde](https://serde.rs/) - 序列化/反序列化
- [tokio](https://tokio.rs/) - 异步运行时

## 📞 联系方式

- 提交问题：[GitHub Issues](https://github.com/osynicite/osynic_pad/issues)
- 项目地址：[GitHub Repository](https://github.com/osynicite/osynic_pad)

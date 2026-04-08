# 🎮 Osynic Pad 使用指南

## 项目概述

Osynic Pad 是一个功能完整的游戏手柄到键盘映射工具，支持交互式配置选择、Debug 模式，以及实时键盘事件映射。

## 主要特性

✨ **核心功能**
- 🕹️ 支持多种游戏手柄（Xbox、PlayStation 等）
- 🔄 实时按键映射到键盘事件
- 📝 灵活的 JSON 配置文件支持
- 🔧 双映射模式（Default 和 Alternative）

🎯 **CLI 优化**
- 📋 配置文件自动扫描和交互式选择
- ⌨️ 上下箭头键导航，Enter 键确认
- 🐛 可选的 Debug 模式用于诊断
- 🎛️ 彩色界面和直观的用户提示
- 🚀 优雅的启动和关闭流程

## 编译和运行

### 前置需求
- Rust 1.85.0 或更高版本
- 游戏手柄连接到系统

### 编译

```bash
# Debug 模式构建（开发用）
cargo build

# Release 模式构建（推荐，性能更好）
cargo build --release
```

### 运行

```bash
# 直接运行（从源代码）
cargo run --release

# 或者运行编译后的二进制文件
./target/release/osynic-pad  # Linux/macOS
target\release\osynic-pad.exe  # Windows PowerShell
```

## 使用流程

### 1️⃣ 启动程序

运行二进制文件后，你会看到欢迎屏幕：

```
   ✨ 欢迎使用 Osynic Pad 手柄映射工具！
   按任意键开始配置...
```

按任意键继续。

### 2️⃣ 选择配置文件

程序会自动扫描 `configs/` 目录下的所有 `.json` 文件，并展示交互式菜单：

```
═══════════════════════════════════════
   🎮 Osynic Pad 配置选择
═══════════════════════════════════════

请选择配置文件（↑↓ 移动，Enter 确认）：

→ pad_config.json
  pad_config_em.json
  pad_config_mid.json
  pad_config_top.json

═══════════════════════════════════════
```

**操作方法：**
- `↑` - 上移选择
- `↓` - 下移选择  
- `Enter` - 确认选择
- `Esc` - 退出程序

### 3️⃣ 选择 Debug 模式

选择配置后，会进入 Debug 模式选择：

```
═══════════════════════════════════════
   🐛 Debug 模式选择
═══════════════════════════════════════

选择模式（↑↓ 移动，Enter 确认）：

→ 关闭 Debug
  启用 Debug

═══════════════════════════════════════
```

**说明：**
- **关闭 Debug**：正常模式，按键映射无日志输出
- **启用 Debug** ✓：输出详细日志，用于诊断和测试

### 4️⃣ 开始映射

选择完成后，程序启动映射功能：

```
╔═══════════════════════════════════════╗
║   🎮 Osynic Pad 正在运行             ║
╠═══════════════════════════════════════╣
║ 配置文件: pad_config.json             ║
║ 映射模式: Default                     ║
║ Debug 模式: 关闭                       ║
╚═══════════════════════════════════════╝

可用的手柄:
  0 【Xbox 360 Controller】

⚙️  启动中... 按 Ctrl+C 退出
```

现在可以按动手柄按键，程序会将其映射为对应的键盘事件。

### 5️⃣ 停止程序

按 `Ctrl+C` 优雅地停止程序：

```
👋 正在关闭...
```

## 配置文件格式

配置文件采用 JSON 格式，位于 `configs/` 目录。

### 完整配置示例

```json
{
  "mapping_mode": "default",
  "button_mappings": {
    "South": "Space",
    "East": "E",
    "North": "W",
    "West": "Q",
    "Start": "Escape",
    "Select": "Enter",
    "DPadUp": "Up",
    "DPadDown": "Down",
    "DPadLeft": "Left",
    "DPadRight": "Right",
    "LeftTrigger2": "A",
    "RightTrigger2": "D",
    "LeftThumb": "C",
    "RightThumb": "V"
  },
  "alternative_mappings": {
    "South": "Enter",
    "East": "Escape",
    "North": "F",
    "West": "Z"
  }
}
```

### 配置说明

| 字段                   | 说明                 | 示例                           |
| ---------------------- | -------------------- | ------------------------------ |
| `mapping_mode`         | 初始映射模式（可选） | `"default"` 或 `"alternative"` |
| `button_mappings`      | 默认按键映射         | `{"South": "Space"}`           |
| `alternative_mappings` | 备选按键映射         | `{"South": "Enter"}`           |

### 支持的手柄按键

#### 数字按键
```
South, East, North, West
LeftTrigger, RightTrigger
LeftTrigger2, RightTrigger2
LeftThumb, RightThumb
Select, Start, Mode
DPadUp, DPadDown, DPadLeft, DPadRight
```

### 支持的键盘按键

```
Escape, Enter, Return
Space, Tab
Left, Right, Up, Down (箭头键)
A-Z (所有字母键)
F2 等函数键
```

## Debug 模式详解

启用 Debug 模式后，程序会输出详细日志：

```
[DEBUG] 等待手柄事件中...
[DEBUG] 手柄: Xbox 360 Controller (已连接: true)
[DEBUG] 按下按钮: South -> Space
[DEBUG] 释放按钮: South -> Space
[DEBUG] 触发器事件: LeftTrigger2 值变化: 0.85
```

**何时使用 Debug 模式：**
- ✅ 测试新配置文件
- ✅ 诊断按键映射问题
- ✅ 验证手柄连接状态
- ✅ 调试触发器灵敏度

## 特殊功能

### Space 按键智能处理

程序对 Space 键有特殊优化，支持多键同时按下时的正确处理：

```rust
// 使用计数器防止多按键重复触发
if key == Key::Space {
    let mut count = SPACE_COUNT.write().await;
    *count += 1;
    if *count == 1 {
        enigo_guard.key(key, Press)?;  // 仅第一个按键时触发
    }
}
```

### 触发器映射

L/R 触发器可以映射为按键，当压力值达到阈值时触发：

```json
{
  "button_mappings": {
    "LeftTrigger2": "A",
    "RightTrigger2": "D"
  }
}
```

## 项目结构

```
osynic_pad/
├── src/
│   ├── main.rs           # 主程序入口（CLI 实现）
│   ├── lib.rs            # 库代码（可选）
│   └── error.rs          # 错误类型定义（可选）
├── configs/
│   ├── pad_config.json           # 主配置
│   ├── pad_config_em.json        # 紧急配置
│   ├── pad_config_mid.json       # 中等配置
│   └── pad_config_top.json       # 高性能配置
├── examples/
│   ├── agilo.rs          # 原始实现（参考用）
│   └── a.rs              # 其他示例
├── Cargo.toml            # 项目配置
└── README.md             # 项目说明
```

## 常见问题

### ❓ 手柄无法识别

**症状：** 程序启动时显示"没有检测到任何手柄"

**解决方案：**
1. 检查 USB 连接（如果是有线）
2. 检查蓝牙配对（如果是无线）
3. 在系统游戏控制器设置中验证手柄状态
4. 重新连接手柄，再次运行程序

### ❓ 按键映射无反应

**症状：** 按下手柄按键但键盘没有响应

**解决方案：**
1. 启用 Debug 模式验证事件是否被接收
2. 检查配置文件中的按键名称是否正确
3. 确保目标应用程序窗口获得焦点
4. 检查是否有按键冲突或被系统拦截

### ❓ 程序启动很慢

**症状：** 程序从执行到显示菜单耗时较长

**解决方案：**
1. 使用 Release 模式编译（`cargo build --release`）
2. 确保系统中没有过多后台进程
3. 检查网络连接（首次运行可能需要下载依赖）

## 开发相关

### 添加新的配置文件

1. 在 `configs/` 目录下创建新的 JSON 文件
2. 遵循配置格式
3. 重新运行程序，新配置会自动出现在菜单中

### 修改 CLI 界面

所有 CLI 界面代码在 `src/main.rs` 中的以下函数：
- `show_config_selector()` - 整体选择流程
- `select_from_list()` - 配置选择菜单
- `select_debug_mode()` - Debug 模式选择
- `clear_screen()` - 清屏函数

### 扩展支持的按键

在 `Config::get_key_for_button()` 方法中添加新的按键映射：

```rust
fn get_key_for_button(&self, button: &str, mode: &MappingMode) -> Option<Key> {
    // 添加新的按键支持
    match key_str.as_str() {
        "F1" => Some(Key::F1),
        "F3" => Some(Key::F3),
        // ...
    }
}
```

## 许可证

MIT License - 详见 LICENSE 文件

## 技术栈

- 🦀 **Rust 1.85.0+** - 编程语言
- 📦 **Tokio 1.51** - 异步运行时
- ⌨️ **enigo 0.6.1** - 键盘控制库
- 🎮 **gilrs 0.11** - 手柄驱动库
- 💻 **crossterm 0.28** - 终端控制库

## 性能指标

| 指标       | 数值              |
| ---------- | ----------------- |
| 二进制大小 | ~2.1 MB (Release) |
| 启动时间   | < 100ms           |
| 事件延迟   | < 5ms             |
| 内存占用   | ~15-20 MB         |

---

**最后更新：** 2026年4月5日  
**版本：** 0.1.0

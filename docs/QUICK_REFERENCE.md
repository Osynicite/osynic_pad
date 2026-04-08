# 🎮 Osynic Pad 快速参考

## 快速启动

### 编译
```bash
cargo build --release
```

### 运行
```bash
# 从源码运行
cargo run --release

# 或直接运行二进制文件
target\release\osynic-pad.exe       # Windows
./target/release/osynic-pad         # Linux/macOS
```

---

## 菜单操作

### 配置选择
```
↑ / ↓  →  上下导航
Enter  →  确认选择
Esc    →  退出程序
```

### Debug 模式
```
↑ / ↓  →  上下导航
Enter  →  确认选择
Esc    →  退出程序
```

---

## 支持的手柄按键

### 主要按钮
| 按键    | 说明                         |
| ------- | ---------------------------- |
| `South` | 下按钮（Xbox 的 A，PS 的 ✕） |
| `North` | 上按钮（Xbox 的 Y，PS 的 △） |
| `East`  | 右按钮（Xbox 的 B，PS 的 ◯） |
| `West`  | 左按钮（Xbox 的 X，PS 的 ◻） |

### 肩部按钮
| 按键            | 说明       |
| --------------- | ---------- |
| `LeftTrigger`   | 左肩浅按钮 |
| `RightTrigger`  | 右肩浅按钮 |
| `LeftTrigger2`  | 左肩深按钮 |
| `RightTrigger2` | 右肩深按钮 |

### 菜单和摇杆
| 按键         | 说明       |
| ------------ | ---------- |
| `Start`      | 开始按钮   |
| `Select`     | 选择按钮   |
| `Mode`       | 模式按钮   |
| `LeftThumb`  | 左摇杆按下 |
| `RightThumb` | 右摇杆按下 |

### 方向键
| 按键        | 说明     |
| ----------- | -------- |
| `DPadUp`    | 方向键上 |
| `DPadDown`  | 方向键下 |
| `DPadLeft`  | 方向键左 |
| `DPadRight` | 方向键右 |

---

## 支持的键盘按键

### 字母
```
A, B, C, ... Z  (所有大写字母)
```

### 特殊键
```
Space      →  空格键
Enter      →  回车键
Return     →  返回键
Escape     →  Esc 键
Tab        →  制表符
```

### 方向键
```
Left       →  左箭头
Right      →  右箭头
Up         →  上箭头
Down       →  下箭头
```

### 功能键
```
F2, F3, ... (支持函数键)
```

---

## 配置示例

### 最小配置
```json
{
  "button_mappings": {
    "South": "Space",
    "East": "E",
    "Start": "Escape"
  },
  "alternative_mappings": {}
}
```

### 完整配置
```json
{
  "mapping_mode": "default",
  "button_mappings": {
    "South": "Space",
    "East": "E",
    "North": "W",
    "West": "Q",
    "Start": "Escape",
    "Select": "Tab",
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
    "East": "F2",
    "North": "Z",
    "West": "X"
  }
}
```

---

## 常见映射方案

### 游戏方案（WASD + Space）
```json
{
  "mapping_mode": "default",
  "button_mappings": {
    "North": "W",
    "South": "Space",
    "East": "D",
    "West": "A",
    "RightTrigger2": "Q"
  }
}
```

### 菜单导航方案
```json
{
  "mapping_mode": "default",
  "button_mappings": {
    "North": "Up",
    "South": "Down",
    "East": "Right",
    "West": "Left",
    "Start": "Enter",
    "Select": "Escape"
  }
}
```

---

## Debug 模式查看

启用 Debug 模式后的日志示例：

```
[DEBUG] 等待手柄事件中...
[DEBUG] 手柄: Xbox 360 Controller (已连接: true)
[DEBUG] 按下按钮: South -> Space
[DEBUG] 释放按钮: South -> Space
[DEBUG] 触发器 LeftTrigger2 值变化: 0.85
```

---

## 故障排除速查

### 手柄无法识别
- ✓ 检查 USB/蓝牙连接
- ✓ 验证系统游戏控制器设置
- ✓ 尝试重新连接

### 按键无反应
- ✓ 启用 Debug 模式检查
- ✓ 验证配置文件拼写
- ✓ 确保应用窗口获得焦点

### 程序启动慢
- ✓ 使用 Release 构建版本
- ✓ 关闭后台程序
- ✓ 检查网络连接

---

## 文件位置

| 文件           | 位置                            |
| -------------- | ------------------------------- |
| 主程序         | `src/main.rs`                   |
| 配置文件       | `configs/*.json`                |
| Release 二进制 | `target/release/osynic-pad.exe` |
| 使用说明       | `USAGE.md`                      |
| 项目信息       | `PROJECT_COMPLETION.md`         |

---

## 环境变量（可选）

暂无环境变量配置需求

---

## 建议的按键映射

### FPS 游戏
```
North: W, South: Space, East: D, West: A
RightTrigger2: LMB, LeftTrigger2: RMB
```

### 平台游戏
```
North: Jump, South: Dash, East: Interact, West: Pause
DPad: Movement
```

### RPG 游戏
```
North: Magic, South: Attack, East: Interact, West: Menu
DPad: Navigation
```

---

## 性能参考

| 指标       | 数值     |
| ---------- | -------- |
| 启动时间   | < 100ms  |
| 事件延迟   | < 5ms    |
| 内存占用   | 15-20 MB |
| CPU 占用   | < 2%     |
| 二进制大小 | 1.24 MB  |

---

## 更多帮助

- 详细使用指南：见 `USAGE.md`
- 开发信息：见 `BUILD_SUMMARY.md`
- 项目详情：见 `PROJECT_COMPLETION.md`

---

## 库化集成（新增）

### 作为库导入
```toml
[dependencies]
osynic_pad = { path = "../osynic_pad" }
```

### 基础使用
```rust
use osynic_pad::{Config, GamepadMapper, MappingMode};

let config = Config::load("config.json")?;
let mapper = GamepadMapper::new(config, enigo, MappingMode::Default, true);
```

### 完整示例参考
见 `examples/library_usage.rs`，包含 5 种使用模式

### 公开 API 列表
```
Config              配置结构体
MappingMode         映射模式枚举
GamepadMapper       映射引擎
PadEvent            事件类型
button_to_string()  按钮转文字
scan_config_files() 扫描配置文件
string_to_key()     文字转按键
[CLI 函数集]        UI 交互函数
```

---

**最后更新：** 2026年4月5日  
版本：0.1.0 Release (库化版)

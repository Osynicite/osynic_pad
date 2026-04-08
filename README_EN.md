<!-- markdownlint-disable MD033 MD041 MD045 -->
<p align="center">
    <img style="height:240px;width:240px" src="https://s2.loli.net/2025/03/10/GSsjOcHqdtBkyu9.png" alt="OsynicPad"/>
</p>

<div align="center">

# 🎮 Osynic Pad

**A gamepad-to-keyboard mapper written in Rust**

![Rust](https://img.shields.io/badge/Rust-1.85+-blue?style=flat-square)
![License](https://img.shields.io/badge/License-MIT-green?style=flat-square)
![Crates.io](https://img.shields.io/crates/v/osynic_pad?style=flat-square)

English | [中文](README.md)

</div>

---

## 📋 Overview

**Osynic Pad** is a powerful gamepad-to-keyboard mapping library written in Rust. It supports various mainstream gaming controllers including Xbox, PlayStation, and Nintendo Switch. Through an interactive menu system and flexible configuration, you can easily map gamepad buttons to keyboard events.

### 🎯 Key Features

- 🎮 **Wide Controller Support** - Compatible with Xbox, PlayStation, Nintendo Switch, and other mainstream gaming controllers
- 📋 **Flexible Configuration** - Three ways to obtain configurations: use existing configs, import from external locations, or create new ones interactively
- ⚡ **High Performance** - Response latency < 5ms
- 🐛 **Debug Mode** - Detailed event logging for troubleshooting and testing
- 🎯 **Multi-mode Support** - Support for custom key mappings and mode switching
- 🌈 **User-friendly CLI** - Interactive menus powered by the `inquire` library

## 🚀 Getting Started

### Installation

**Requires Rust 1.85.0 or later**

```bash
# Build from source
git clone https://github.com/osynicite/osynic_pad
cd osynic_pad
cargo build --release

# Run the program
./target/release/osynic-pad
```

Or use cargo install:

```bash
cargo install osynic_pad
```

### Basic Usage

When you start the program, an interactive menu will appear:

#### 1️⃣ Configuration Selection

```
🎮 Osynic Pad Configuration Selection

Please select configuration source:
→ 📂 Use existing configuration
  📥 Import configuration from path
  ✨ Create new configuration
```

**Three options explained:**

- **📂 Use existing configuration** - Select from configuration files in the `configs/` directory
- **📥 Import configuration** - Import a JSON configuration file from any location (automatically copied to project)
- **✨ Create new configuration** - Interactively create a new configuration file

#### 2️⃣ Debug Mode Selection

```
🐛 Enable debug mode? (y/n)
```

#### 3️⃣ Start Mapping

After configuration is complete, the program will listen to controller input and map buttons to keyboard events.

## ⚙️ Configuration File Format

Configuration files should be placed in the `configs/` directory in JSON format.

### Configuration File Example

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

### Supported Keys

#### Controller Buttons
- `A`, `B`, `X`, `Y` - Four main buttons
- `Start`, `Back` - Start and back buttons
- `LB`, `RB` - Left and right bumper
- `LT`, `RT` - Left and right triggers (analog, converted to discrete values)
- `LS`, `RS` - Left and right stick buttons
- `DPadUp`, `DPadDown`, `DPadLeft`, `DPadRight` - Direction pad

#### Keyboard Output
Supports all standard keyboard keys, such as: `a-z`, `0-9`, `Space`, `Return`, `Escape`, etc.

## 📚 Using as a Library

Add `osynic_pad` as a library dependency to your Rust project:

```toml
[dependencies]
osynic_pad = "0.1.0"
tokio = { version = "1.51", features = ["sync", "rt-multi-thread"] }
```

### Code Example

```rust
use osynic_pad::{Config, GamepadMapper, MappingMode};
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration
    let config = Config::load_from_path(&"configs/my_config.json".into())?;
    
    // Create keyboard input handler
    let enigo = Arc::new(Mutex::new(
        enigo::Enigo::new(&enigo::Settings::default())?
    ));
    
    // Create mapper
    let mapper = Arc::new(GamepadMapper::new(
        config,
        enigo,
        MappingMode::Default,
        false, // debug mode
    ));
    
    // Now you can use mapper to handle gamepad events...
    Ok(())
}
```

## 🏗️ Project Structure

```
osynic_pad/
├── src/
│   ├── lib.rs          # Library entry point
│   ├── main.rs         # CLI binary entry point
│   ├── cli.rs          # Interactive CLI menus
│   ├── config.rs       # Configuration file handling
│   ├── events.rs       # Gamepad event definitions
│   ├── mapper.rs       # Core mapping logic
│   └── error.rs        # Error type definitions
├── configs/            # Configuration files directory
│   ├── pad_config.json
│   └── ...
├── examples/           # Usage examples
└── Cargo.toml         # Project configuration
```

## 📚 API Documentation

### Main Types and Functions

- `Config` - Configuration file structure
- `GamepadMapper` - Gamepad event mapper
- `MappingMode` - Mapping mode enumeration
- `PadEvent` - Gamepad event types

For detailed API documentation, run `cargo doc --open`.

## 🔧 Development

### Prerequisites

- Rust 1.85.0 or later
- Cargo

### Local Build

```bash
cargo build
cargo build --release
```

### Running Tests

```bash
cargo test
```

### Code Quality Checks

```bash
cargo clippy
cargo fmt
```

## 🤝 Contributing

Issues and pull requests are welcome!

Please ensure:
- Follow Rust community coding standards
- Pass all clippy checks
- Code is properly formatted

## 📜 License

This project is open source under the [MIT License](LICENSE). See the LICENSE file for details.

## 🙏 Acknowledgments

This project depends on these excellent open source libraries:

- [gilrs](https://gitlab.com/gilrs-project/gilrs) - Gamepad input
- [enigo](https://github.com/enigo-rs/enigo) - Keyboard output simulation
- [inquire](https://github.com/mikaelmello/inquire) - Interactive CLI
- [serde](https://serde.rs/) - Serialization/Deserialization
- [tokio](https://tokio.rs/) - Async runtime

## 📞 Contact

- Report issues: [GitHub Issues](https://github.com/osynicite/osynic_pad/issues)
- Project repository: [GitHub Repository](https://github.com/osynicite/osynic_pad)

// Osynic Pad 库 - 游戏手柄到键盘映射工具
// 允许外部应用调用核心功能模块
#[cfg(feature = "cli")]
pub mod cli;
pub mod config;
pub mod error;
pub mod events;
pub mod mapper;

// 公共 API 导出
#[cfg(feature = "cli")]
pub use cli::{
    select_debug_mode, select_from_list, show_config_selector, show_gamepads, show_startup_info,
    show_welcome_screen,
};
pub use config::{Config, MappingMode, scan_config_files, string_to_key};
pub use events::{PadEvent, button_to_string};
pub use mapper::GamepadMapper;

/// 库版本
pub const VERSION: &str = "0.1.0";

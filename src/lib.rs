// Osynic Pad 库 - 游戏手柄到键盘映射工具
// 允许外部应用调用核心功能模块

pub mod config;
pub mod events;
pub mod mapper;
pub mod cli;
pub mod error;

// 公共 API 导出
pub use config::{ Config, MappingMode, scan_config_files, string_to_key };
pub use events::{ PadEvent, button_to_string };
pub use mapper::GamepadMapper;
pub use cli::{
    show_config_selector,
    select_from_list,
    select_debug_mode,
    show_welcome_screen,
    show_startup_info,
    show_gamepads,
};

/// 库版本
pub const VERSION: &str = "0.1.0";

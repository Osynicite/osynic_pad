use crate::config::scan_config_files;
use inquire::{Confirm, Select, Text};
use std::error::Error;
use std::fs;
use std::path::PathBuf;

/// 整体配置选择流程
pub fn show_config_selector() -> Result<(PathBuf, bool), Box<dyn Error>> {
    println!("🎮 Osynic Pad 配置选择\n");

    // 第一步：选择配置源
    let config_path = choose_config_source()?;

    // 第二步：选择 Debug 模式
    let debug = select_debug_mode()?;

    Ok((config_path, debug))
}

/// 选择配置来源：使用现有配置、导入配置或新建配置
pub fn choose_config_source() -> Result<PathBuf, Box<dyn Error>> {
    let options = vec![
        "📂 使用现有配置",
        "📥 导入配置（从指定位置）",
        "✨ 新建配置（交互式）",
    ];

    let choice = Select::new("请选择配置来源:", options)
        .with_help_message("使用 ↑↓ 导航，Enter 确认")
        .prompt()?;

    match choice {
        "📂 使用现有配置" => use_existing_config(),
        "📥 导入配置（从指定位置）" => import_config_from_path(),
        "✨ 新建配置（交互式）" => create_new_config(),
        _ => Err("未知的选择".into()),
    }
}

/// 使用现有配置
fn use_existing_config() -> Result<PathBuf, Box<dyn Error>> {
    let configs = scan_config_files()?;

    if configs.is_empty() {
        eprintln!("❌ 错误：没有找到任何配置文件！");
        eprintln!("📁 请在 configs 目录下放置 JSON 配置文件。");
        std::process::exit(1);
    }

    let config_names: Vec<String> = configs
        .iter()
        .map(|p| {
            p.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("Unknown")
                .to_string()
        })
        .collect();

    select_from_list(config_names, &configs)
}

/// 导入配置文件
fn import_config_from_path() -> Result<PathBuf, Box<dyn Error>> {
    let path_str = Text::new("请输入配置文件的完整路径:")
        .with_help_message("例如: C:\\path\\to\\config.json 或 /home/user/config.json")
        .prompt()?;

    let path = PathBuf::from(&path_str);

    // 检查文件是否存在
    if !path.exists() {
        eprintln!("❌ 错误：文件不存在: {}", path_str);
        std::process::exit(1);
    }

    // 检查是否是 JSON 文件
    if path.extension().and_then(|s| s.to_str()) != Some("json") {
        eprintln!("⚠️  警告：文件似乎不是 JSON 格式");
    }

    // 尝试复制到 configs 目录
    let filename = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("imported_config.json");

    let configs_dir = PathBuf::from("configs");
    if !configs_dir.exists() {
        fs::create_dir_all(&configs_dir)?;
    }

    let dest_path = configs_dir.join(filename);

    // 如果文件已存在，询问是否覆盖
    if dest_path.exists() {
        let should_overwrite = Confirm::new(&format!("配置文件 {} 已存在。是否覆盖?", filename))
            .with_default(false)
            .prompt()?;

        if !should_overwrite {
            println!("✅ 使用现有文件: {}", dest_path.display());
            return Ok(dest_path);
        }
    }

    fs::copy(&path, &dest_path)?;
    println!(
        "✅ 已导入配置: {} -> {}",
        path.display(),
        dest_path.display()
    );

    Ok(dest_path)
}

/// 创建新配置（交互式）
fn create_new_config() -> Result<PathBuf, Box<dyn Error>> {
    println!("\n📝 新建配置（交互式向导）\n");

    let config_name = Text::new("请输入配置名称（不含 .json）:")
        .with_default("my_config")
        .prompt()?;

    let config_filename = format!("{}.json", config_name);
    let configs_dir = PathBuf::from("configs");

    if !configs_dir.exists() {
        fs::create_dir_all(&configs_dir)?;
    }

    let config_path = configs_dir.join(&config_filename);

    // 如果文件已存在
    if config_path.exists() {
        let should_overwrite =
            Confirm::new(&format!("配置文件 {} 已存在。是否覆盖?", config_filename))
                .with_default(false)
                .prompt()?;

        if !should_overwrite {
            println!("✅ 使用现有文件: {}", config_path.display());
            return Ok(config_path);
        }
    }

    // 创建默认配置
    let default_config = r#"{
  "default_mode": "Default",
  "mappings": {
    "Default": {
      "A": "Space",
      "B": "Escape",
      "X": "w",
      "Y": "s"
    }
  }
}
"#;

    fs::write(&config_path, default_config)?;
    println!("✅ 已创建新配置: {}", config_path.display());
    println!("💡 你可以编辑此文件来自定义按键映射。\n");

    Ok(config_path)
}

/// 使用 inquire 选择配置文件
pub fn select_from_list(
    names: Vec<String>,
    configs: &[PathBuf],
) -> Result<PathBuf, Box<dyn Error>> {
    let answer = Select::new("请选择配置文件:", names)
        .with_page_size(10)
        .with_help_message("使用 ↑↓ 导航，Enter 确认")
        .prompt()?;

    // 找到选中的配置文件
    let selected_idx = configs
        .iter()
        .position(|p| p.file_name().and_then(|n| n.to_str()).unwrap_or("") == answer)
        .ok_or("Configuration not found")?;

    Ok(configs[selected_idx].clone())
}

/// 选择 Debug 模式
pub fn select_debug_mode() -> Result<bool, Box<dyn Error>> {
    let confirm = Confirm::new("🐛 启用 Debug 模式?")
        .with_default(false)
        .with_help_message("y/n")
        .prompt()?;

    Ok(confirm)
}

/// 显示欢迎屏幕
pub fn show_welcome_screen() -> Result<(), Box<dyn Error>> {
    println!("\n   ✨ 欢迎使用 Osynic Pad 手柄映射工具！\n");
    Ok(())
}

/// 显示启动信息
pub fn show_startup_info(config_filename: &str, mode_str: &str, debug: bool) {
    println!("\n╔═══════════════════════════════════════╗");
    println!("║   🎮 Osynic Pad 正在运行             ║");
    println!("╠═══════════════════════════════════════╣");
    println!("║ 配置文件: {:<26} ║", config_filename);
    println!("║ 映射模式: {:<26} ║", mode_str);
    println!(
        "║ Debug 模式: {:<24} ║",
        if debug { "启用 ✓" } else { "关闭" }
    );
    println!("╚═══════════════════════════════════════╝\n");
}

/// 显示可用的手柄列表
pub fn show_gamepads(gilrs: &gilrs::Gilrs) -> bool {
    println!("可用的手柄:");
    let mut found = false;
    for (id, gamepad) in gilrs.gamepads() {
        found = true;
        println!("  {} 【{}】", id, gamepad.name());
    }

    if !found {
        println!("❌ 没有检测到任何手柄！");
        println!("   请检查：");
        println!("   1. 手柄是否已正确连接");
        println!("   2. 系统是否识别到手柄");
        println!("   3. 驱动是否正确安装");
    }

    found
}

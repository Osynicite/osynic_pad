use crossterm::{
    event::{self, KeyCode, KeyEvent},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use enigo::{
    Direction::{Press, Release},
    Enigo, Key, Keyboard, Settings,
};
use gilrs::{Button, Event, EventType, Gilrs};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, LazyLock};
use std::time::{Duration, Instant};
use tokio::sync::{Mutex, RwLock, mpsc};

// 全局变量：Space 按键计数
static SPACE_COUNT: LazyLock<RwLock<u32>> = LazyLock::new(|| RwLock::new(0));

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
enum MappingMode {
    Default,
    Alternative,
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    mapping_mode: Option<MappingMode>,
    button_mappings: HashMap<String, String>,
    alternative_mappings: HashMap<String, String>,
}

impl Config {
    fn load(path: &str) -> Result<Self, Box<dyn Error>> {
        let config_str = fs::read_to_string(path)?;
        let config: Config = serde_json::from_str(&config_str)?;
        Ok(config)
    }

    fn get_key_for_button(&self, button: &str, mode: &MappingMode) -> Option<Key> {
        let key_str = match mode {
            MappingMode::Default => self.button_mappings.get(button),
            MappingMode::Alternative => self.alternative_mappings.get(button),
        };

        key_str.and_then(|key_str| match key_str.as_str() {
            "Escape" => Some(Key::Escape),
            "Enter" => Some(Key::Return),
            "Left" => Some(Key::LeftArrow),
            "Right" => Some(Key::RightArrow),
            "Space" => Some(Key::Space),
            "F2" => Some(Key::F2),
            "A" => Some(Key::A),
            "B" => Some(Key::B),
            "C" => Some(Key::C),
            "D" => Some(Key::D),
            "E" => Some(Key::E),
            "F" => Some(Key::F),
            "G" => Some(Key::G),
            "H" => Some(Key::H),
            "I" => Some(Key::I),
            "J" => Some(Key::J),
            "K" => Some(Key::K),
            "L" => Some(Key::L),
            "M" => Some(Key::M),
            "N" => Some(Key::N),
            "O" => Some(Key::O),
            "P" => Some(Key::P),
            "Q" => Some(Key::Q),
            "R" => Some(Key::R),
            "S" => Some(Key::S),
            "T" => Some(Key::T),
            "U" => Some(Key::U),
            "V" => Some(Key::V),
            "W" => Some(Key::W),
            "X" => Some(Key::X),
            "Y" => Some(Key::Y),
            "Z" => Some(Key::Z),
            _ => None,
        })
    }
}

#[derive(Debug)]
enum PadEvent {
    ButtonPress(String),
    ButtonRelease(String),
    TriggerChanged(String, f32),
}

struct GamepadMapper {
    config: Config,
    enigo: Arc<Mutex<Enigo>>,
    mode: MappingMode,
    debug: bool,
}

impl GamepadMapper {
    fn new(config: Config, enigo: Arc<Mutex<Enigo>>, mode: MappingMode, debug: bool) -> Self {
        Self {
            config,
            enigo,
            mode,
            debug,
        }
    }

    async fn handle_event(&self, event: PadEvent) -> Result<(), Box<dyn Error>> {
        if let Ok(mut enigo_guard) = self.enigo.try_lock() {
            match event {
                PadEvent::ButtonPress(button) => {
                    if let Some(key) = self.config.get_key_for_button(&button, &self.mode) {
                        if self.debug {
                            println!("[DEBUG] 按下按钮: {} -> {:?}", button, key);
                        }
                        if key == Key::Space {
                            let mut count = SPACE_COUNT.write().await;
                            *count += 1;
                            if *count == 1 {
                                enigo_guard.key(key, Press)?;
                            }
                        } else {
                            enigo_guard.key(key, Press)?;
                        }
                    } else if self.debug {
                        println!("[DEBUG] 未找到按钮 {} 的映射", button);
                    }
                }
                PadEvent::ButtonRelease(button) => {
                    if let Some(key) = self.config.get_key_for_button(&button, &self.mode) {
                        if self.debug {
                            println!("[DEBUG] 释放按钮: {} -> {:?}", button, key);
                        }
                        if key == Key::Space {
                            let mut count = SPACE_COUNT.write().await;
                            *count = count.saturating_sub(1);
                            if *count == 0 {
                                enigo_guard.key(key, Release)?;
                            }
                        } else {
                            enigo_guard.key(key, Release)?;
                        }
                    }
                }
                PadEvent::TriggerChanged(trigger, value) => {
                    let threshold = 1.0;
                    if let Some(key) = self.config.get_key_for_button(&trigger, &self.mode) {
                        if self.debug && value > 0.0 {
                            println!("[DEBUG] 触发器 {} 值变化: {}", trigger, value);
                        }
                        if value >= threshold {
                            enigo_guard.key(key, Press)?;
                        } else if value < threshold {
                            enigo_guard.key(key, Release)?;
                        }
                    }
                }
            }
        }
        Ok(())
    }
}

fn button_to_string(button: Button) -> String {
    match button {
        Button::South => "South".to_string(),
        Button::East => "East".to_string(),
        Button::North => "North".to_string(),
        Button::West => "West".to_string(),
        Button::LeftTrigger => "LeftTrigger".to_string(),
        Button::LeftTrigger2 => "LeftTrigger2".to_string(),
        Button::RightTrigger => "RightTrigger".to_string(),
        Button::RightTrigger2 => "RightTrigger2".to_string(),
        Button::Select => "Select".to_string(),
        Button::Start => "Start".to_string(),
        Button::Mode => "Mode".to_string(),
        Button::LeftThumb => "LeftThumb".to_string(),
        Button::RightThumb => "RightThumb".to_string(),
        Button::DPadUp => "DPadUp".to_string(),
        Button::DPadDown => "DPadDown".to_string(),
        Button::DPadLeft => "DPadLeft".to_string(),
        Button::DPadRight => "DPadRight".to_string(),
        _ => "Unknown".to_string(),
    }
}

/// 扫描 configs 目录找到所有可用的配置文件
fn scan_config_files() -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let config_dir = "configs";
    let mut configs = Vec::new();

    if let Ok(entries) = fs::read_dir(config_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                configs.push(path);
            }
        }
    }

    configs.sort();
    Ok(configs)
}

/// CLI 界面：让用户选择配置和 Debug 模式
fn show_config_selector() -> Result<(PathBuf, bool), Box<dyn Error>> {
    let configs = scan_config_files()?;

    if configs.is_empty() {
        eprintln!("错误：没有找到任何配置文件！");
        eprintln!("请在 configs 目录下放置 JSON 配置文件。");
        std::process::exit(1);
    }

    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let result = select_from_list(&configs);

    disable_raw_mode()?;
    execute!(stdout, LeaveAlternateScreen)?;

    let selected_config = result?;

    // 选择 Debug 模式
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let debug = select_debug_mode()?;

    disable_raw_mode()?;
    execute!(stdout, LeaveAlternateScreen)?;

    Ok((selected_config, debug))
}

/// 使用上下键选择配置
fn select_from_list(configs: &[PathBuf]) -> Result<PathBuf, Box<dyn Error>> {
    let mut selected = 0;

    loop {
        clear_screen();
        println!("\n═══════════════════════════════════════");
        println!("   🎮 Osynic Pad 配置选择");
        println!("═══════════════════════════════════════\n");
        println!("请选择配置文件（↑↓ 移动，Enter 确认）：\n");

        for (i, config) in configs.iter().enumerate() {
            let filename = config
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("Unknown");
            let marker = if i == selected { "→" } else { " " };
            let style = if i == selected {
                format!("\x1b[1;32m{}\x1b[0m", filename)
            } else {
                filename.to_string()
            };
            println!("{} {}", marker, style);
        }

        println!("\n═══════════════════════════════════════");

        if event::poll(Duration::from_millis(100))? {
            if let event::Event::Key(KeyEvent { code, .. }) = event::read()? {
                match code {
                    KeyCode::Up => {
                        if selected > 0 {
                            selected -= 1;
                        }
                    }
                    KeyCode::Down => {
                        if selected < configs.len() - 1 {
                            selected += 1;
                        }
                    }
                    KeyCode::Enter => {
                        return Ok(configs[selected].clone());
                    }
                    KeyCode::Esc => {
                        std::process::exit(0);
                    }
                    _ => {}
                }
            }
        }
    }
}

/// 选择 Debug 模式
fn select_debug_mode() -> Result<bool, Box<dyn Error>> {
    let mut selected = 0;
    let options = vec!["关闭 Debug", "启用 Debug"];

    loop {
        clear_screen();
        println!("\n═══════════════════════════════════════");
        println!("   🐛 Debug 模式选择");
        println!("═══════════════════════════════════════\n");
        println!("选择模式（↑↓ 移动，Enter 确认）：\n");

        for (i, option) in options.iter().enumerate() {
            let marker = if i == selected { "→" } else { " " };
            let style = if i == selected {
                format!("\x1b[1;32m{}\x1b[0m", option)
            } else {
                option.to_string()
            };
            println!("{} {}", marker, style);
        }

        println!("\n═══════════════════════════════════════");

        if event::poll(Duration::from_millis(100))? {
            if let event::Event::Key(KeyEvent { code, .. }) = event::read()? {
                match code {
                    KeyCode::Up => {
                        if selected > 0 {
                            selected -= 1;
                        }
                    }
                    KeyCode::Down => {
                        if selected < options.len() - 1 {
                            selected += 1;
                        }
                    }
                    KeyCode::Enter => {
                        return Ok(selected == 1);
                    }
                    KeyCode::Esc => {
                        std::process::exit(0);
                    }
                    _ => {}
                }
            }
        }
    }
}

/// 清空屏幕
fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

#[tokio::main]
async fn main() {
    match run().await {
        Ok(_) => (),
        Err(err) => {
            eprintln!("错误: {}", err);
            std::process::exit(1);
        }
    }
}

async fn run() -> Result<(), Box<dyn Error>> {
    // 显示欢迎信息
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    clear_screen();
    println!("\n   ✨ 欢迎使用 Osynic Pad 手柄映射工具！");
    println!("   按任意键开始配置...\n");

    // 等待用户输入
    event::read()?;

    disable_raw_mode()?;
    execute!(stdout, LeaveAlternateScreen)?;

    // 选择配置和 Debug 模式
    let (config_path, debug) = show_config_selector()?;

    let config_filename = config_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("Unknown");

    let config = Config::load(config_path.to_str().unwrap())?;
    let enigo = Arc::new(Mutex::new(Enigo::new(&Settings::default())?));

    let mode = config.mapping_mode.clone().unwrap_or(MappingMode::Default);

    let mapper = Arc::new(GamepadMapper::new(
        config,
        Arc::clone(&enigo),
        mode.clone(),
        debug,
    ));

    // 创建 channel 传递手柄事件
    let (tx, mut rx) = mpsc::channel::<PadEvent>(1000);

    // 初始化 Gilrs
    let mut gilrs = Gilrs::new()?;

    // 显示启动信息
    println!("\n╔═══════════════════════════════════════╗");
    println!("║   🎮 Osynic Pad 正在运行             ║");
    println!("╠═══════════════════════════════════════╣");
    println!("║ 配置文件: {:<26} ║", config_filename);
    println!("║ 映射模式: {:<26} ║", format!("{:?}", mode));
    println!(
        "║ Debug 模式: {:<24} ║",
        if debug { "启用 ✓" } else { "关闭" }
    );
    println!("╚═══════════════════════════════════════╝\n");

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

    println!("\n⚙️  启动中... 按 Ctrl+C 退出\n");

    // 创建手柄事件监听任务
    let tx_clone = tx.clone();
    let gamepad_handler = tokio::spawn(async move {
        let mut counter = 0;
        let mut last_event_time = Instant::now();

        loop {
            counter += 1;
            if counter % 3000 == 0 && debug {
                println!("[DEBUG] 等待手柄事件中...");
                for (_id, gamepad) in gilrs.gamepads() {
                    println!(
                        "[DEBUG] 手柄: {} (已连接: {})",
                        gamepad.name(),
                        gamepad.is_connected()
                    );
                }
            }

            while let Some(Event {
                id: _,
                event,
                time: _,
                ..
            }) = gilrs.next_event()
            {
                match event {
                    EventType::ButtonPressed(button, _) => {
                        let _ = tx_clone.try_send(PadEvent::ButtonPress(button_to_string(button)));
                    }
                    EventType::ButtonReleased(button, _) => {
                        let _ =
                            tx_clone.try_send(PadEvent::ButtonRelease(button_to_string(button)));
                    }
                    EventType::AxisChanged(axis, value, _) => match axis {
                        gilrs::Axis::LeftZ => {
                            let _ = tx_clone.try_send(PadEvent::TriggerChanged(
                                "LeftTrigger2".to_string(),
                                value,
                            ));
                        }
                        gilrs::Axis::RightZ => {
                            let _ = tx_clone.try_send(PadEvent::TriggerChanged(
                                "RightTrigger2".to_string(),
                                value,
                            ));
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }

            let elapsed = last_event_time.elapsed();
            let sleep_time = if elapsed < Duration::from_millis(2) {
                Duration::from_millis(2) - elapsed
            } else {
                Duration::ZERO
            };
            tokio::time::sleep(sleep_time).await;
            last_event_time = Instant::now();
        }
    });

    // 创建事件处理任务
    let event_handler = tokio::spawn({
        let mapper = Arc::clone(&mapper);
        async move {
            while let Some(event) = rx.recv().await {
                if let Err(e) = mapper.handle_event(event).await {
                    eprintln!("处理事件错误: {}", e);
                }
            }
        }
    });

    // 等待中断信号
    tokio::signal::ctrl_c().await?;

    println!("\n\n👋 正在关闭...");
    drop(tx);
    gamepad_handler.abort();
    event_handler.abort();

    Ok(())
}

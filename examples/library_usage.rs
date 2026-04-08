// 外部应用示例：演示如何使用 osynic_pad 库
// 这个示例展示了如何作为库来调用核心功能

use std::error::Error;
use std::sync::Arc;
use std::time::{ Duration, Instant };

use tokio::sync::{ mpsc, Mutex };
use enigo::{ Enigo, Settings };
use gilrs::{ Gilrs, Event, EventType };

// 导入 osynic_pad 库
use osynic_pad::{ Config, GamepadMapper, MappingMode, PadEvent, button_to_string, show_gamepads };

/// 示例 1: 基础使用 - 直接创建映射器并处理事件
#[allow(dead_code)]
async fn example_basic_usage() -> Result<(), Box<dyn Error>> {
    println!("\n=== 示例 1: 基础使用 ===\n");

    // 加载配置
    let config = Config::load("configs/pad_config.json")?;

    // 创建映射器
    let enigo = Arc::new(Mutex::new(Enigo::new(&Settings::default())?));
    let mapper = GamepadMapper::new(
        config,
        enigo,
        MappingMode::Default,
        true // 启用 debug 模式
    );

    // 处理事件示例
    let event = PadEvent::ButtonPress("South".to_string());
    mapper.handle_event(event).await?;

    Ok(())
}

/// 示例 2: 使用配置扫描 - 列出所有可用配置
#[allow(dead_code)]
async fn example_config_scanning() -> Result<(), Box<dyn Error>> {
    println!("\n=== 示例 2: 配置扫描 ===\n");

    // 扫描所有配置文件
    let configs = osynic_pad::scan_config_files()?;

    println!("找到 {} 个配置文件：", configs.len());
    for (i, config_path) in configs.iter().enumerate() {
        let filename = config_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("Unknown");
        println!("  {}. {}", i + 1, filename);
    }

    // 使用第一个配置
    if let Some(first_config) = configs.first() {
        let config = Config::load_from_path(first_config)?;
        println!("\n已加载配置: {:?}", first_config.file_name());
        println!("映射模式: {:?}", config.mapping_mode);
    }

    Ok(())
}

/// 示例 3: 手柄集成 - 完整的手柄监听和映射流程
#[allow(dead_code)]
async fn example_gamepad_integration() -> Result<(), Box<dyn Error>> {
    println!("\n=== 示例 3: 手柄集成 ===\n");

    // 加载配置
    let config = Config::load("configs/pad_config.json")?;
    let enigo = Arc::new(Mutex::new(Enigo::new(&Settings::default())?));
    let mode = config.mapping_mode.clone().unwrap_or(MappingMode::Default);
    let mapper = Arc::new(GamepadMapper::new(config, enigo, mode, true));

    // 初始化手柄驱动
    let mut gilrs = Gilrs::new()?;

    // 显示可用手柄
    println!("检测到的手柄：");
    show_gamepads(&gilrs);

    // 创建事件管道
    let (tx, mut rx) = mpsc::channel::<PadEvent>(1000);

    // 手柄监听任务
    let tx_clone = tx.clone();
    let listen_task = tokio::spawn(async move {
        println!("\n监听手柄事件 (10 秒后停止)...");
        let start = Instant::now();

        loop {
            if start.elapsed() > Duration::from_secs(10) {
                break;
            }

            while let Some(Event { id: _, event, .. }) = gilrs.next_event() {
                match event {
                    EventType::ButtonPressed(button, _) => {
                        let button_name = button_to_string(button);
                        println!("  [EVENT] 按下: {}", button_name);
                        let _ = tx_clone.try_send(PadEvent::ButtonPress(button_name));
                    }
                    EventType::ButtonReleased(button, _) => {
                        let button_name = button_to_string(button);
                        println!("  [EVENT] 释放: {}", button_name);
                        let _ = tx_clone.try_send(PadEvent::ButtonRelease(button_name));
                    }
                    _ => {}
                }
            }

            tokio::time::sleep(Duration::from_millis(16)).await;
        }

        println!("\n监听任务结束");
    });

    // 事件处理任务
    let mapper_clone = Arc::clone(&mapper);
    let process_task = tokio::spawn(async move {
        while let Some(event) = rx.recv().await {
            if let Err(e) = mapper_clone.handle_event(event).await {
                eprintln!("处理事件失败: {}", e);
            }
        }
    });

    // 等待任务完成
    listen_task.await?;
    drop(tx);
    process_task.await?;

    Ok(())
}

/// 示例 4: 多配置切换 - 支持多个配置文件
#[allow(dead_code)]
async fn example_multi_config_switching() -> Result<(), Box<dyn Error>> {
    println!("\n=== 示例 4: 多配置切换 ===\n");

    // 扫描所有配置
    let configs = osynic_pad::scan_config_files()?;

    // 为每个配置创建映射器
    let enigo = Arc::new(Mutex::new(Enigo::new(&Settings::default())?));

    for config_path in configs.iter().take(2) {
        let config = Config::load_from_path(config_path)?;
        let filename = config_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("Unknown");

        let mode = config.mapping_mode.clone().unwrap_or(MappingMode::Default);
        let _mapper = GamepadMapper::new(config, Arc::clone(&enigo), mode, false);

        println!("✓ 已加载配置: {}", filename);
    }

    Ok(())
}

/// 示例 5: 自定义映射模式 - 在运行时切换映射模式
#[allow(dead_code)]
async fn example_custom_mapping_mode() -> Result<(), Box<dyn Error>> {
    println!("\n=== 示例 5: 自定义映射模式 ===\n");

    let config = Config::load("configs/pad_config.json")?;
    let enigo = Arc::new(Mutex::new(Enigo::new(&Settings::default())?));

    // 使用 Default 模式
    {
        let mapper = GamepadMapper::new(
            config.clone(),
            Arc::clone(&enigo),
            MappingMode::Default,
            false
        );
        println!("✓ Default 模式映射器已创建");
        println!(
            "  South 按钮映射到: {:?}",
            config.get_key_for_button("South", &MappingMode::Default)
        );
    }

    // 使用 Alternative 模式
    {
        let mapper = GamepadMapper::new(
            config.clone(),
            Arc::clone(&enigo),
            MappingMode::Alternative,
            false
        );
        println!("✓ Alternative 模式映射器已创建");
        println!(
            "  South 按钮映射到: {:?}",
            config.get_key_for_button("South", &MappingMode::Alternative)
        );
    }

    Ok(())
}

/// 主函数 - 运行所有示例
#[tokio::main]
async fn main() {
    println!("\n╔════════════════════════════════════════╗");
    println!("║   osynic_pad 库使用示例                ║");
    println!("╚════════════════════════════════════════╝");

    // 示例 1: 基础使用
    if let Err(e) = example_basic_usage().await {
        eprintln!("示例 1 失败: {}", e);
    } else {
        println!("✓ 示例 1 完成");
    }

    // 示例 2: 配置扫描
    if let Err(e) = example_config_scanning().await {
        eprintln!("示例 2 失败: {}", e);
    } else {
        println!("✓ 示例 2 完成");
    }

    // 示例 3: 手柄集成 (需要实际手柄)
    if let Err(e) = example_gamepad_integration().await {
        eprintln!("示例 3 失败: {}", e);
    } else {
        println!("✓ 示例 3 完成");
    }

    // 示例 4: 多配置切换
    if let Err(e) = example_multi_config_switching().await {
        eprintln!("示例 4 失败: {}", e);
    } else {
        println!("✓ 示例 4 完成");
    }

    // 示例 5: 自定义映射模式
    if let Err(e) = example_custom_mapping_mode().await {
        eprintln!("示例 5 失败: {}", e);
    } else {
        println!("✓ 示例 5 完成");
    }

    println!("\n╚════════════════════════════════════════╝");
    println!("所有示例执行完毕！\n");
}

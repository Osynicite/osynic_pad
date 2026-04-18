!#[cfg(feature = "cli")]
// Osynic Pad - 手柄到键盘映射工具
// CLI 二进制程序入口

use std::error::Error;
use std::sync::Arc;
use std::time::{Duration, Instant};

use enigo::{Enigo, Settings};
use gilrs::{Event, EventType, Gilrs};
use tokio::sync::{Mutex, mpsc};

// 导入库模块
use osynic_pad::{Config, GamepadMapper, MappingMode, PadEvent, cli, events};

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
    // 显示欢迎屏幕
    cli::show_welcome_screen()?;

    // 选择配置和 Debug 模式
    let (config_path, debug) = cli::show_config_selector()?;

    let config_filename = config_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("Unknown");

    // 加载配置
    let config = Config::load_from_path(&config_path)?;

    // 创建 enigo 实例
    let enigo = Arc::new(Mutex::new(Enigo::new(&Settings::default())?));

    // 确定映射模式
    let mode = config.mapping_mode.clone().unwrap_or(MappingMode::Default);

    // 创建映射器实例
    let mapper = Arc::new(GamepadMapper::new(
        config,
        Arc::clone(&enigo),
        mode.clone(),
        debug,
    ));

    // 创建事件管道
    let (tx, mut rx) = mpsc::channel::<PadEvent>(1000);

    // 初始化手柄
    let mut gilrs = Gilrs::new()?;

    // 显示启动信息
    cli::show_startup_info(config_filename, &format!("{:?}", mode), debug);
    cli::show_gamepads(&gilrs);

    // 注意：不在主循环中输出任何内容，以避免 TUI 闪屏
    // 如需 debug 输出，请重定向到日志文件

    // 创建手柄事件监听任务
    let tx_clone = tx.clone();
    let gamepad_handler = tokio::spawn(async move {
        let mut last_event_time = Instant::now();

        loop {
            // 处理手柄事件
            while let Some(Event {
                id: _,
                event,
                time: _,
                ..
            }) = gilrs.next_event()
            {
                match event {
                    EventType::ButtonPressed(button, _) => {
                        let _ = tx_clone
                            .try_send(PadEvent::ButtonPress(events::button_to_string(button)));
                    }
                    EventType::ButtonReleased(button, _) => {
                        let _ = tx_clone
                            .try_send(PadEvent::ButtonRelease(events::button_to_string(button)));
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

            // 轮询控制（低 CPU 占用）
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

    // 等待 Ctrl+C 信号
    tokio::signal::ctrl_c().await?;

    println!("\n\n👋 正在关闭...");
    drop(tx);
    gamepad_handler.abort();
    event_handler.abort();

    Ok(())
}

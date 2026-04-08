use std::sync::{ Arc, LazyLock };
use tokio::sync::{ Mutex, RwLock };
use std::error::Error;
use enigo::{ Direction::{ Press, Release }, Enigo, Key, Keyboard };
use crate::config::{ Config, MappingMode };
use crate::events::PadEvent;

/// 全局变量：Space 按键计数器，用于防止多个按键同时按下时重复触发
static SPACE_COUNT: LazyLock<RwLock<u32>> = LazyLock::new(|| RwLock::new(0));

/// 手柄映射核心引擎
pub struct GamepadMapper {
    pub config: Config,
    pub enigo: Arc<Mutex<Enigo>>,
    pub mode: MappingMode,
    pub debug: bool,
}

impl GamepadMapper {
    /// 创建新的手柄映射器
    pub fn new(config: Config, enigo: Arc<Mutex<Enigo>>, mode: MappingMode, debug: bool) -> Self {
        Self { config, enigo, mode, debug }
    }

    /// 处理手柄事件，将其转换为键盘事件
    pub async fn handle_event(&self, event: PadEvent) -> Result<(), Box<dyn Error>> {
        if let Ok(mut enigo_guard) = self.enigo.try_lock() {
            match event {
                PadEvent::ButtonPress(button) => {
                    if let Some(key) = self.config.get_key_for_button(&button, &self.mode) {
                        if self.debug {
                            println!("[DEBUG] 按下按钮: {} -> {:?}", button, key);
                        }
                        // Space 键特殊处理：使用计数器防止多个按键同时按下时的重复触发
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
                        // Space 键特殊处理：计数器减一，仅当计数为 0 时才释放按键
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

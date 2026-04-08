use std::error::Error;
use std::io::stdin;
use std::sync::{Arc, LazyLock};
use tokio::sync::{mpsc,RwLock,Mutex};
// use std::sync::Mutex;
use std::time::{Duration, Instant};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use std::{fs, env};
use gilrs::{Gilrs, Button, Event, EventType};
use enigo::{
    Direction::{Press, Release},
    Enigo, Settings, Key, Keyboard,
};

// 一个全局变量，用rwlock，计数Space是否正在被几个按键按着
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
    button_mappings: HashMap<String, String>,    // 按钮到键盘按键的映射
    alternative_mappings: HashMap<String, String>, // 备选映射方案
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

        key_str.and_then(|key_str| {
            match key_str.as_str() {
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
            }
        })
    }
}

#[derive(Debug)]
enum PadEvent {
    ButtonPress(String),    // 按钮名称
    ButtonRelease(String),  // 按钮名称
    TriggerChanged(String, f32),  // 触发器名称和压力值（0.0-1.0）
}

struct GamepadMapper {
    config: Config,
    enigo: Arc<Mutex<Enigo>>,
    mode: MappingMode,
}

impl GamepadMapper {
    fn new(config: Config, enigo: Arc<Mutex<Enigo>>, mode: MappingMode) -> Self {
        Self { config, enigo, mode }
    }

    async fn handle_event(&self, event: PadEvent) -> Result<(), Box<dyn Error>> {
        if let Ok(mut enigo_guard) = self.enigo.try_lock() {
            // println!("尝试处理事件: {:?}", event);
            match event {
                PadEvent::ButtonPress(button) => {
                    // println!("尝试处理按钮按下: {}", button);
                    if let Some(key) = self.config.get_key_for_button(&button, &self.mode) {
                        // println!("按下按键: {:?}", key);
                        // 特别判断一下Space键，需要增加一个计数
                        
                        // enigo_guard.key(key, Press)?;
                        if key == Key::Space {
                            let mut count = SPACE_COUNT.write().await;
                            *count += 1;
                            if *count == 1 {
                                enigo_guard.key(key, Press)?;
                            }
                        } else {
                            enigo_guard.key(key, Press)?;
                        }
                    } else {
                        println!("未找到按钮 {} 的映射", button);
                    }
                },
                PadEvent::ButtonRelease(button) => {
                    // println!("尝试处理按钮释放: {}", button);
                    if let Some(key) = self.config.get_key_for_button(&button, &self.mode) {
                        // println!("释放按键: {:?}", key);
                        // 特别判断一下Space键，如果是两个按键同时按下，就不释放，只去减少一个按键的计数
                        // enigo_guard.key(key, Release)?;
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
                },
                PadEvent::TriggerChanged(trigger, value) => {
                    // 当触发器压力超过阈值时触发按键
                    let threshold = 1.0;
                    // println!("触发器 {} 值变化: {}", trigger, value);
                    
                    if let Some(key) = self.config.get_key_for_button(&trigger, &self.mode) {
                        if value >= threshold {
                            // println!("触发器达到阈值，按下按键: {:?}", key);
                            enigo_guard.key(key, Press)?;
                        } else if value < threshold {
                            // println!("触发器低于阈值，释放按键: {:?}", key);
                            enigo_guard.key(key, Release)?;
                        }
                    }
                }
            }
        } else {
            println!("无法获取 enigo 锁");
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

#[tokio::main]
async fn main() {
    match run().await {
        Ok(_) => (),
        Err(err) => println!("错误: {}", err)
    }
}

async fn run() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    let enigo = Arc::new(Mutex::new(Enigo::new(&Settings::default())?));
    
    // 加载配置文件
    let mut config = Config::load("configs/pad_config.json")?;

    println!("配置文件加载 OK");
    
    // 确定映射模式
    let mode = env::args().nth(1)
        .and_then(|arg| match arg.to_lowercase().as_str() {
            "alternative" => Some(MappingMode::Alternative),
            "default" => Some(MappingMode::Default),
            _ => None
        })
        .or(config.mapping_mode.take())
        .unwrap_or(MappingMode::Default);

    println!("使用映射模式: {:?}", mode);
    
    let mapper = Arc::new(GamepadMapper::new(config, Arc::clone(&enigo), mode));
    
    // 创建channel用于传递手柄事件
    let (tx, mut rx) = mpsc::channel::<PadEvent>(1000);
    
    // 初始化Gilrs
    let mut gilrs = Gilrs::new()?;
    
    // 显示已连接的手柄和详细信息
    println!("\n可用的手柄:");
    let mut found = false;
    for (id, gamepad) in gilrs.gamepads() {
        found = true;
        println!("{}: {} 已连接", id, gamepad.name());
        println!("- 电源状态: {:?}", gamepad.power_info());
        println!("- 是否已连接: {}", gamepad.is_connected());
        println!("- UUID: {:?}", gamepad.uuid());
    }
    
    if !found {
        println!("没有检测到任何手柄！");
        println!("请检查:");
        println!("1. 手柄是否已正确连接并开启");
        println!("2. 系统是否识别到手柄（可在系统游戏控制器设置中查看）");
        println!("3. 驱动是否正确安装");
    }
    
    // 创建手柄事件监听任务
    let tx_clone = tx.clone();
    let gamepad_handler = tokio::spawn(async move {
        println!("开始监听手柄事件...");
        let mut counter = 0;
        
        let mut last_event_time = Instant::now();
        loop {
            counter += 1;
            if counter % 3000 == 0 {  // 每3000次循环打印一次（约5秒）
                println!("正在等待手柄事件...");
                // 重新检查连接状态
                for (_id, gamepad) in gilrs.gamepads() {
                    println!("手柄 {} 状态: 已连接={}", gamepad.name(), gamepad.is_connected());
                }
            }
            while let Some(Event { id:_, event, time: _,.. }) = gilrs.next_event() {
                // println!("收到事件: {:?} 来自手柄 {}", event, id);
                match event {
                    EventType::ButtonPressed(button, _) => {
                        // println!("按下按钮: {}", button_to_string(button));
                        // 判断是不是trigger2，
                        if let Err(e) = tx_clone.try_send(PadEvent::ButtonPress(button_to_string(button))) {
                            eprintln!("发送事件错误: {}", e);
                        }
                    }
                    EventType::ButtonReleased(button, _) => {
                        // println!("释放按钮: {}", button_to_string(button));
                        if let Err(e) = tx_clone.try_send(PadEvent::ButtonRelease(button_to_string(button))) {
                            eprintln!("发送事件错误: {}", e);
                        }
                    }
                    EventType::AxisChanged(axis, value, _) => {
                        match axis {
                            gilrs::Axis::LeftZ => {  // Left Trigger
                                // println!("左触发器值变化: {}", value);
                                let _ = tx_clone.try_send(PadEvent::TriggerChanged("LeftTrigger2".to_string(), value));
                            }
                            gilrs::Axis::RightZ => {  // Right Trigger
                                // println!("右触发器值变化: {}", value);
                                let _ = tx_clone.try_send(PadEvent::TriggerChanged("RightTrigger2".to_string(), value));
                            }
                            _ => {}
                        }
                    }
                    _ => {}  // 忽略其他类型的事件
                }
            }
            // tokio::time::sleep(std::time::Duration::from_millis(16)).await;  // 约60fps的轮询率
            // 
            // tokio::time::sleep(std::time::Duration::from_millis(8)).await; // 约120fps的轮询率
            //
            // tokio::time::sleep(std::time::Duration::from_millis(4)).await; // 约240fps的轮询率
            //
            // tokio::time::sleep(std::time::Duration::from_millis(2)).await; // 约480fps的轮询率
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
        println!("开始事件处理任务...");  // 没问题
        async move {
            println!("event_handler 任务开始");
            while let Some(event) = rx.recv().await {
                // println!("event_handler 收到事件: {:?}", event); // 一直就没收到过事件，咋回事？
                if let Err(e) = mapper.handle_event(event).await {
                    eprintln!("处理事件错误: {}", e);
                }
            }
            println!("event_handler 任务结束");
        } 
    });

    println!("连接已建立。按回车键退出。");                
    input.clear();
    stdin().read_line(&mut input)?;
    
    // 清理和关闭
    drop(tx);
    gamepad_handler.abort();
    event_handler.abort();
    
    Ok(())
}


// use std::error::Error;
// use std::io::stdin;
// use std::sync::Arc;
// use tokio::sync::mpsc;
// use std::sync::Mutex;
// use std::collections::HashMap;
// use serde::{Deserialize, Serialize};
// use std::{fs, env};
// use gilrs::{Gilrs, Button, Event, EventType};
// use enigo::{
//     Direction::{Press, Release},
//     Enigo, Settings, Key, Keyboard,
// };

// #[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
// #[serde(rename_all = "lowercase")]
// enum MappingMode {
//     Default,
//     Alternative,
// }

// #[derive(Debug, Serialize, Deserialize)]
// struct Config {
//     mapping_mode: Option<MappingMode>,
//     button_mappings: HashMap<String, String>,    // 按钮到键盘按键的映射
//     alternative_mappings: HashMap<String, String>, // 备选映射方案
// }

// impl Config {
//     fn load(path: &str) -> Result<Self, Box<dyn Error>> {
//         let config_str = fs::read_to_string(path)?;
//         let config: Config = serde_json::from_str(&config_str)?;
//         Ok(config)
//     }

//     fn get_key_for_button(&self, button: &str, mode: &MappingMode) -> Option<Key> {
//         let key_str = match mode {
//             MappingMode::Default => self.button_mappings.get(button),
//             MappingMode::Alternative => self.alternative_mappings.get(button),
//         };

//         key_str.and_then(|key_str| {
//             match key_str.as_str() {
//                 "Space" => Some(Key::Space),
//                 "A" => Some(Key::A),
//                 "B" => Some(Key::B),
//                 "C" => Some(Key::C),
//                 "D" => Some(Key::D),
//                 "E" => Some(Key::E),
//                 "F" => Some(Key::F),
//                 "G" => Some(Key::G),
//                 "H" => Some(Key::H),
//                 "I" => Some(Key::I),
//                 "J" => Some(Key::J),
//                 "K" => Some(Key::K),
//                 "L" => Some(Key::L),
//                 "M" => Some(Key::M),
//                 "N" => Some(Key::N),
//                 "O" => Some(Key::O),
//                 "P" => Some(Key::P),
//                 "Q" => Some(Key::Q),
//                 "R" => Some(Key::R),
//                 "S" => Some(Key::S),
//                 "T" => Some(Key::T),
//                 "U" => Some(Key::U),
//                 "V" => Some(Key::V),
//                 "W" => Some(Key::W),
//                 "X" => Some(Key::X),
//                 "Y" => Some(Key::Y),
//                 "Z" => Some(Key::Z),
//                 _ => None,
//             }
//         })
//     }
// }

// #[derive(Debug)]
// enum PadEvent {
//     ButtonPress(String),    // 按钮名称
//     ButtonRelease(String),  // 按钮名称
//     TriggerChanged(String, f32),  // 触发器名称和压力值（0.0-1.0）
// }

// struct GamepadMapper {
//     config: Config,
//     enigo: Arc<Mutex<Enigo>>,
//     mode: MappingMode,
// }

// impl GamepadMapper {
//     fn new(config: Config, enigo: Arc<Mutex<Enigo>>, mode: MappingMode) -> Self {
//         Self { config, enigo, mode }
//     }

//     fn handle_event(&self, event: PadEvent) -> Result<(), Box<dyn Error>> {
//         if let Ok(mut enigo_guard) = self.enigo.lock() {
//             match event {
//                 PadEvent::ButtonPress(button) => {
//                     if let Some(key) = self.config.get_key_for_button(&button, &self.mode) {
//                         enigo_guard.key(key, Press)?;
//                     }
//                 },
//                 PadEvent::ButtonRelease(button) => {
//                     if let Some(key) = self.config.get_key_for_button(&button, &self.mode) {
//                         enigo_guard.key(key, Release)?;
//                     }
//                 },
//                 PadEvent::TriggerChanged(trigger, value) => {
//                     // 当触发器压力超过80%时触发按键
//                     if value >= 0.8 {
//                         if let Some(key) = self.config.get_key_for_button(&trigger, &self.mode) {
//                             enigo_guard.key(key, Press)?;
//                         }
//                     } else {
//                         if let Some(key) = self.config.get_key_for_button(&trigger, &self.mode) {
//                             enigo_guard.key(key, Release)?;
//                         }
//                     }
//                 }
//             }
//         }
//         Ok(())
//     }
// }

// fn button_to_string(button: Button) -> String {
//     match button {
//         Button::South => "South".to_string(),
//         Button::East => "East".to_string(),
//         Button::North => "North".to_string(),
//         Button::West => "West".to_string(),
//         Button::LeftTrigger => "LeftTrigger".to_string(),
//         Button::LeftTrigger2 => "LeftTrigger2".to_string(),
//         Button::RightTrigger => "RightTrigger".to_string(),
//         Button::RightTrigger2 => "RightTrigger2".to_string(),
//         Button::Select => "Select".to_string(),
//         Button::Start => "Start".to_string(),
//         Button::Mode => "Mode".to_string(),
//         Button::LeftThumb => "LeftThumb".to_string(),
//         Button::RightThumb => "RightThumb".to_string(),
//         Button::DPadUp => "DPadUp".to_string(),
//         Button::DPadDown => "DPadDown".to_string(),
//         Button::DPadLeft => "DPadLeft".to_string(),
//         Button::DPadRight => "DPadRight".to_string(),
//         _ => "Unknown".to_string(),
//     }
// }

// #[tokio::main]
// async fn main() {
//     match run().await {
//         Ok(_) => (),
//         Err(err) => println!("错误: {}", err)
//     }
// }

// async fn run() -> Result<(), Box<dyn Error>> {
//     let mut input = String::new();
//     let enigo = Arc::new(Mutex::new(Enigo::new(&Settings::default())?));
    
//     // 加载配置文件
//     let mut config = Config::load("pad_config.json")?;
    
//     // 确定映射模式
//     let mode = env::args().nth(1)
//         .and_then(|arg| match arg.to_lowercase().as_str() {
//             "alternative" => Some(MappingMode::Alternative),
//             "default" => Some(MappingMode::Default),
//             _ => None
//         })
//         .or(config.mapping_mode.take())
//         .unwrap_or(MappingMode::Default);

//     println!("使用映射模式: {:?}", mode);
    
//     let mapper = Arc::new(GamepadMapper::new(config, Arc::clone(&enigo), mode));
    
//     // 创建channel用于传递手柄事件
//     let (tx, mut rx) = mpsc::channel::<PadEvent>(32);
    
//     // 初始化Gilrs
//     let mut gilrs = Gilrs::new()?;
    
//     // 显示已连接的手柄和详细信息
//     println!("\n可用的手柄:");
//     let mut found = false;
//     for (id, gamepad) in gilrs.gamepads() {
//         found = true;
//         println!("{}: {} 已连接", id, gamepad.name());
//         println!("- 电源状态: {:?}", gamepad.power_info());
//         println!("- 是否已连接: {}", gamepad.is_connected());
//         println!("- UUID: {:?}", gamepad.uuid());
//     }
    
//     if !found {
//         println!("没有检测到任何手柄！");
//         println!("请检查:");
//         println!("1. 手柄是否已正确连接并开启");
//         println!("2. 系统是否识别到手柄（可在系统游戏控制器设置中查看）");
//         println!("3. 驱动是否正确安装");
//     }
    
//     // 创建手柄事件监听任务
//     let tx_clone = tx.clone();
//     let gamepad_handler = tokio::spawn(async move {
//         println!("开始监听手柄事件...");
//         let mut counter = 0;
//         loop {
//             counter += 1;
//             if counter % 300 == 0 {  // 每300次循环打印一次（约5秒）
//                 println!("正在等待手柄事件...");
//                 // 重新检查连接状态
//                 for (_id, gamepad) in gilrs.gamepads() {
//                     println!("手柄 {} 状态: 已连接={}", gamepad.name(), gamepad.is_connected());
//                 }
//             }
//             while let Some(Event { id, event, time: _ ,..}) = gilrs.next_event() {
//                 println!("收到事件: {:?} 来自手柄 {}", event, id);
//                 match event {
//                     EventType::ButtonPressed(button, _) => {
//                         let _ = tx_clone.try_send(PadEvent::ButtonPress(button_to_string(button)));
//                     }
//                     EventType::ButtonReleased(button, _) => {
//                         let _ = tx_clone.try_send(PadEvent::ButtonRelease(button_to_string(button)));
//                     }
//                     EventType::AxisChanged(axis, value, _) => {
//                         match axis {
//                             gilrs::Axis::LeftZ => {  // Left Trigger
//                                 let _ = tx_clone.try_send(PadEvent::TriggerChanged("LeftTrigger2".to_string(), value));
//                             }
//                             gilrs::Axis::RightZ => {  // Right Trigger
//                                 let _ = tx_clone.try_send(PadEvent::TriggerChanged("RightTrigger2".to_string(), value));
//                             }
//                             _ => {}
//                         }
//                     }
//                     _ => {}  // 忽略其他类型的事件
//                 }
//             }
//             std::thread::sleep(std::time::Duration::from_millis(16));  // 约60fps的轮询率
//         }
//     });

//     // 创建事件处理任务
//     let event_handler = tokio::spawn({
//         let mapper = Arc::clone(&mapper);
//         async move {
//             while let Some(event) = rx.recv().await {
//                 if let Err(e) = mapper.handle_event(event) {
//                     eprintln!("处理事件错误: {}", e);
//                 }
//             }
//         }
//     });

//     println!("连接已建立。按回车键退出。");
//     input.clear();
//     stdin().read_line(&mut input)?;
    
//     // 清理和关闭
//     drop(tx);
//     gamepad_handler.abort();
//     event_handler.abort();
    
//     Ok(())
// }


// use std::error::Error;
// use std::io::stdin;
// use std::sync::Arc;
// use tokio::sync::mpsc;
// use std::sync::Mutex;
// use std::collections::HashMap;
// use serde::{Deserialize, Serialize};
// use std::{fs, env};
// use gilrs::{Gilrs, Button, Event, EventType};
// use enigo::{
//     Direction::{Press, Release},
//     Enigo, Settings, Key, Keyboard,
// };

// #[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
// #[serde(rename_all = "lowercase")]
// enum MappingMode {
//     Default,
//     Alternative,
// }

// #[derive(Debug, Serialize, Deserialize)]
// struct Config {
//     mapping_mode: Option<MappingMode>,
//     button_mappings: HashMap<String, String>,    // 按钮到键盘按键的映射
//     alternative_mappings: HashMap<String, String>, // 备选映射方案
// }

// impl Config {
//     fn load(path: &str) -> Result<Self, Box<dyn Error>> {
//         let config_str = fs::read_to_string(path)?;
//         let config: Config = serde_json::from_str(&config_str)?;
//         Ok(config)
//     }

//     fn get_key_for_button(&self, button: &str, mode: &MappingMode) -> Option<Key> {
//         let key_str = match mode {
//             MappingMode::Default => self.button_mappings.get(button),
//             MappingMode::Alternative => self.alternative_mappings.get(button),
//         };

//         key_str.and_then(|key_str| {
//             match key_str.as_str() {
//                 "Space" => Some(Key::Space),
//                 "A" => Some(Key::A),
//                 "B" => Some(Key::B),
//                 "C" => Some(Key::C),
//                 "D" => Some(Key::D),
//                 "E" => Some(Key::E),
//                 "F" => Some(Key::F),
//                 "G" => Some(Key::G),
//                 "H" => Some(Key::H),
//                 "I" => Some(Key::I),
//                 "J" => Some(Key::J),
//                 "K" => Some(Key::K),
//                 "L" => Some(Key::L),
//                 "M" => Some(Key::M),
//                 "N" => Some(Key::N),
//                 "O" => Some(Key::O),
//                 "P" => Some(Key::P),
//                 "Q" => Some(Key::Q),
//                 "R" => Some(Key::R),
//                 "S" => Some(Key::S),
//                 "T" => Some(Key::T),
//                 "U" => Some(Key::U),
//                 "V" => Some(Key::V),
//                 "W" => Some(Key::W),
//                 "X" => Some(Key::X),
//                 "Y" => Some(Key::Y),
//                 "Z" => Some(Key::Z),
//                 _ => None,
//             }
//         })
//     }
// }

// #[derive(Debug)]
// enum PadEvent {
//     ButtonPress(String),    // 按钮名称
//     ButtonRelease(String),  // 按钮名称
//     TriggerChanged(String, f32),  // 触发器名称和压力值（0.0-1.0）
// }

// struct GamepadMapper {
//     config: Config,
//     enigo: Arc<Mutex<Enigo>>,
//     mode: MappingMode,
// }

// impl GamepadMapper {
//     fn new(config: Config, enigo: Arc<Mutex<Enigo>>, mode: MappingMode) -> Self {
//         Self { config, enigo, mode }
//     }

//     fn handle_event(&self, event: PadEvent) -> Result<(), Box<dyn Error>> {
//         if let Ok(mut enigo_guard) = self.enigo.lock() {
//             match event {
//                 PadEvent::ButtonPress(button) => {
//                     if let Some(key) = self.config.get_key_for_button(&button, &self.mode) {
//                         enigo_guard.key(key, Press)?;
//                     }
//                 },
//                 PadEvent::ButtonRelease(button) => {
//                     if let Some(key) = self.config.get_key_for_button(&button, &self.mode) {
//                         enigo_guard.key(key, Release)?;
//                     }
//                 },
//                 PadEvent::TriggerChanged(trigger, value) => {
//                     // 当触发器压力超过80%时触发按键
//                     if value >= 0.8 {
//                         if let Some(key) = self.config.get_key_for_button(&trigger, &self.mode) {
//                             enigo_guard.key(key, Press)?;
//                         }
//                     } else {
//                         if let Some(key) = self.config.get_key_for_button(&trigger, &self.mode) {
//                             enigo_guard.key(key, Release)?;
//                         }
//                     }
//                 }
//             }
//         }
//         Ok(())
//     }
// }

// fn button_to_string(button: Button) -> String {
//     match button {
//         Button::South => "South".to_string(),
//         Button::East => "East".to_string(),
//         Button::North => "North".to_string(),
//         Button::West => "West".to_string(),
//         Button::LeftTrigger => "LeftTrigger".to_string(),
//         Button::LeftTrigger2 => "LeftTrigger2".to_string(),
//         Button::RightTrigger => "RightTrigger".to_string(),
//         Button::RightTrigger2 => "RightTrigger2".to_string(),
//         Button::Select => "Select".to_string(),
//         Button::Start => "Start".to_string(),
//         Button::Mode => "Mode".to_string(),
//         Button::LeftThumb => "LeftThumb".to_string(),
//         Button::RightThumb => "RightThumb".to_string(),
//         Button::DPadUp => "DPadUp".to_string(),
//         Button::DPadDown => "DPadDown".to_string(),
//         Button::DPadLeft => "DPadLeft".to_string(),
//         Button::DPadRight => "DPadRight".to_string(),
//         _ => "Unknown".to_string(),
//     }
// }

// #[tokio::main]
// async fn main() {
//     match run().await {
//         Ok(_) => (),
//         Err(err) => println!("错误: {}", err)
//     }
// }

// async fn run() -> Result<(), Box<dyn Error>> {
//     let mut input = String::new();
//     let enigo = Arc::new(Mutex::new(Enigo::new(&Settings::default())?));
    
//     // 加载配置文件
//     let mut config = Config::load("pad_config.json")?;
    
//     // 确定映射模式
//     let mode = env::args().nth(1)
//         .and_then(|arg| match arg.to_lowercase().as_str() {
//             "alternative" => Some(MappingMode::Alternative),
//             "default" => Some(MappingMode::Default),
//             _ => None
//         })
//         .or(config.mapping_mode.take())
//         .unwrap_or(MappingMode::Default);

//     println!("使用映射模式: {:?}", mode);
    
//     let mapper = Arc::new(GamepadMapper::new(config, Arc::clone(&enigo), mode));
    
//     // 创建channel用于传递手柄事件
//     let (tx, mut rx) = mpsc::channel::<PadEvent>(32);
    
//     // 初始化Gilrs
//     let mut gilrs = Gilrs::new()?;
    
//     // 显示已连接的手柄
//     println!("\n可用的手柄:");
//     for (id, gamepad) in gilrs.gamepads() {
//         println!("{}: {} 已连接", id, gamepad.name());
//     }
    
//     // 创建手柄事件监听任务
//     let tx_clone = tx.clone();
//     let gamepad_handler = tokio::spawn(async move {
//         loop {
//             while let Some(Event { id:_, event, time: _  ,..}) = gilrs.next_event() {
//                 match event {
//                     EventType::ButtonPressed(button, _) => {
//                         let _ = tx_clone.try_send(PadEvent::ButtonPress(button_to_string(button)));
//                     }
//                     EventType::ButtonReleased(button, _) => {
//                         let _ = tx_clone.try_send(PadEvent::ButtonRelease(button_to_string(button)));
//                     }
//                     EventType::AxisChanged(axis, value, _) => {
//                         match axis {
//                             gilrs::Axis::LeftZ => {  // Left Trigger
//                                 let _ = tx_clone.try_send(PadEvent::TriggerChanged("LeftTrigger2".to_string(), value));
//                             }
//                             gilrs::Axis::RightZ => {  // Right Trigger
//                                 let _ = tx_clone.try_send(PadEvent::TriggerChanged("RightTrigger2".to_string(), value));
//                             }
//                             _ => {}
//                         }
//                     }
//                     _ => {}  // 忽略其他类型的事件
//                 }
//             }
//             std::thread::sleep(std::time::Duration::from_millis(16));  // 约60fps的轮询率
//         }
//     });

//     // 创建事件处理任务
//     let event_handler = tokio::spawn({
//         let mapper = Arc::clone(&mapper);
//         async move {
//             while let Some(event) = rx.recv().await {
//                 if let Err(e) = mapper.handle_event(event) {
//                     eprintln!("处理事件错误: {}", e);
//                 }
//             }
//         }
//     });

//     println!("连接已建立。按回车键退出。");
//     input.clear();
//     stdin().read_line(&mut input)?;
    
//     // 清理和关闭
//     drop(tx);
//     gamepad_handler.abort();
//     event_handler.abort();
    
//     Ok(())
// }
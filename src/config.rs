use serde::{ Deserialize, Serialize };
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use enigo::Key;

/// 映射模式：Default 默认模式，Alternative 备选模式
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum MappingMode {
    Default,
    Alternative,
}

/// 手柄配置结构
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub mapping_mode: Option<MappingMode>,
    pub button_mappings: HashMap<String, String>,
    pub alternative_mappings: HashMap<String, String>,
}

impl Config {
    /// 从文件路径加载配置
    pub fn load(path: &str) -> Result<Self, Box<dyn Error>> {
        let config_str = fs::read_to_string(path)?;
        let config: Config = serde_json::from_str(&config_str)?;
        Ok(config)
    }

    /// 从 PathBuf 加载配置
    pub fn load_from_path(path: &PathBuf) -> Result<Self, Box<dyn Error>> {
        let config_str = fs::read_to_string(path)?;
        let config: Config = serde_json::from_str(&config_str)?;
        Ok(config)
    }

    /// 根据按钮和模式获取对应的键盘按键
    pub fn get_key_for_button(&self, button: &str, mode: &MappingMode) -> Option<Key> {
        let key_str = match mode {
            MappingMode::Default => self.button_mappings.get(button),
            MappingMode::Alternative => self.alternative_mappings.get(button),
        };

        key_str.and_then(|key_str| string_to_key(key_str))
    }
}

/// 将字符串转换为 Key 枚举
pub fn string_to_key(key_str: &str) -> Option<Key> {
    match key_str {
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
}

/// 扫描 configs 目录找到所有可用的配置文件
pub fn scan_config_files() -> Result<Vec<PathBuf>, Box<dyn Error>> {
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

use enigo::Key;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

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
        "A" => Some(Key::Unicode('A')),
        "B" => Some(Key::Unicode('B')),
        "C" => Some(Key::Unicode('C')),
        "D" => Some(Key::Unicode('D')),
        "E" => Some(Key::Unicode('E')),
        "F" => Some(Key::Unicode('F')),
        "G" => Some(Key::Unicode('G')),
        "H" => Some(Key::Unicode('H')),
        "I" => Some(Key::Unicode('I')),
        "J" => Some(Key::Unicode('J')),
        "K" => Some(Key::Unicode('K')),
        "L" => Some(Key::Unicode('L')),
        "M" => Some(Key::Unicode('M')),
        "N" => Some(Key::Unicode('N')),
        "O" => Some(Key::Unicode('O')),
        "P" => Some(Key::Unicode('P')),
        "Q" => Some(Key::Unicode('Q')),
        "R" => Some(Key::Unicode('R')),
        "S" => Some(Key::Unicode('S')),
        "T" => Some(Key::Unicode('T')),
        "U" => Some(Key::Unicode('U')),
        "V" => Some(Key::Unicode('V')),
        "W" => Some(Key::Unicode('W')),
        "X" => Some(Key::Unicode('X')),
        "Y" => Some(Key::Unicode('Y')),
        "Z" => Some(Key::Unicode('Z')),
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

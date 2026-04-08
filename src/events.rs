/// 手柄事件枚举类型
#[derive(Debug, Clone)]
pub enum PadEvent {
    /// 按钮按下事件
    ButtonPress(String),
    /// 按钮释放事件
    ButtonRelease(String),
    /// 触发器变化事件（触发器名称，压力值 0.0-1.0）
    TriggerChanged(String, f32),
}

/// 将手柄按钮枚举转换为字符串
pub fn button_to_string(button: gilrs::Button) -> String {
    match button {
        gilrs::Button::South => "South".to_string(),
        gilrs::Button::East => "East".to_string(),
        gilrs::Button::North => "North".to_string(),
        gilrs::Button::West => "West".to_string(),
        gilrs::Button::LeftTrigger => "LeftTrigger".to_string(),
        gilrs::Button::LeftTrigger2 => "LeftTrigger2".to_string(),
        gilrs::Button::RightTrigger => "RightTrigger".to_string(),
        gilrs::Button::RightTrigger2 => "RightTrigger2".to_string(),
        gilrs::Button::Select => "Select".to_string(),
        gilrs::Button::Start => "Start".to_string(),
        gilrs::Button::Mode => "Mode".to_string(),
        gilrs::Button::LeftThumb => "LeftThumb".to_string(),
        gilrs::Button::RightThumb => "RightThumb".to_string(),
        gilrs::Button::DPadUp => "DPadUp".to_string(),
        gilrs::Button::DPadDown => "DPadDown".to_string(),
        gilrs::Button::DPadLeft => "DPadLeft".to_string(),
        gilrs::Button::DPadRight => "DPadRight".to_string(),
        _ => "Unknown".to_string(),
    }
}

use enigo::*;
use std::{thread, time::Duration};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut enigo = Enigo::new(&Settings::default())?;
    loop {
        enigo.button(Button::Left, Direction::Click)?;
        thread::sleep(Duration::from_secs(2));
    }
}
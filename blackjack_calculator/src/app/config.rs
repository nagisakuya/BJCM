use std::io::Write;

use super::*;

pub mod rule_setting_window;
pub use rule_setting_window::*;

pub mod key_setting_window;
pub use key_setting_window::*;

#[derive(Default,serde::Serialize,serde::Deserialize)]
pub struct Config {
    pub rule: Rule,
    pub kyes: Keys,
}
impl Config{
    pub fn load() -> Self{
        if let Ok(bin) = std::fs::read(SETTING_FILE_PATH) {
            bincode::deserialize(&bin).unwrap()
        }else{
            Default::default()
        }
    }
    pub fn save(& self){
        let mut file = std::fs::File::create(SETTING_FILE_PATH).unwrap();
        file.write_all(&bincode::serialize(self).unwrap()).unwrap();
    }
}

#[derive(Clone,serde::Serialize,serde::Deserialize)]
pub struct Keys{
    pub card: [Key; 10],
    pub undo: Key,
    pub next: Key,
    pub reset: Key,
    pub split: Key,
    pub up:Key,
    pub down:Key,
    pub right:Key,
    pub left:Key,
}

impl Default for Keys{
    fn default() -> Self {
        Keys {
            card:[
                Key::Num1,
                Key::Num2,
                Key::Num3,
                Key::Num4,
                Key::Num5,
                Key::Num6,
                Key::Num7,
                Key::Num8,
                Key::Num9,
                Key::Num0,
            ],
            undo:Key::Z,
            next:Key::Enter,
            reset:Key::R,
            split:Key::S,
            up:Key::ArrowUp,
            down:Key::ArrowDown,
            right:Key::ArrowRight,
            left:Key::ArrowLeft,
        }
    }
}
use egui::mutex::RwLock;
use once_cell::sync::Lazy;
use std::{collections::HashMap, io::Write};

use super::*;

pub mod rule_setting_window;
pub use rule_setting_window::*;

pub mod key_setting;
pub use key_setting::*;

pub mod general_setting;
pub use general_setting::*;

pub mod texts;
pub use texts::*;

//configとして色々なデータが一体化しているから色んなところでconfigを渡している可能性がある
//もっと細分化するという解決方法があるかもしれない
pub static CONFIG: Lazy<RwLock<Config>> = Lazy::new(|| RwLock::new(Config::load()));
pub fn get_text(key: TextKey) -> &'static str {
    CONFIG.read().get_text(key)
}

static TEXTS: Lazy<HashMap<TextKey, Vec<&'static str>>> = Lazy::new(load_texts);

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Config {
    pub rule: Rule,
    pub kyes: Keys,
    pub general: GeneralSetting,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            rule: DEMO_RULE,
            kyes: Default::default(),
            general: Default::default(),
        }
    }
}
impl Config {
    pub fn load() -> Self {
        if let Ok(bin) = std::fs::read(SETTING_FILE_PATH) {
            if let Ok(o) = bincode::deserialize(&bin) {
                return o;
            }
        }

        Default::default()
    }
    pub fn save(&self) {
        let mut file = std::fs::File::create(SETTING_FILE_PATH).unwrap();
        file.write_all(&bincode::serialize(self).unwrap()).unwrap();
    }
    fn get_text(&self, key: TextKey) -> &'static str {
        let Some(Some(&text)) = TEXTS.get(&key).map(|i|i.get(self.general.language as usize)) else {
            return "[MISSING_TEXT]";
        };
        text
    }
}

const DEMO_RULE: Rule = Rule {
    NUMBER_OF_DECK: 1,
    LATE_SURRENDER: false,
    DOUBLE_AFTER_SPLIT: false,
    RE_SPLIT: false,
    ACTION_AFTER_SPLITTING_ACE: false,
    DEALER_PEEKS_ACE: true,
    DEALER_PEEKS_TEN: false,
    BJ_PAYBACK: 1.5,
    BJ_AFTER_SPLIT: false,
    DEALER_SOFT_17_STAND: true,
    CHARLIE: None,
    INSUALANCE: true,
};

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn lang_test() {
        let mut config = Config::default();
        config.general.language = Language::Japanese;
        let text = config.get_text(TextKey::BuyWindowName);
        println!("{text}");
    }
}

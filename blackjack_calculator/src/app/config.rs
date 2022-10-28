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

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Config {
    pub rule: Rule,
    pub kyes: Keys,
    pub general: GeneralSetting,
}
impl Default for Config{
    fn default() -> Self {
        Self { rule: DEMO_RULE, kyes: Default::default(), general: Default::default() }
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
    const TEXTS: Lazy<HashMap<TextKey, Vec<&'static str>>> = Lazy::new(|| load_texts());
    pub fn get_text(&self, key: TextKey) -> &'static str {
        if let Some(o) = Self::TEXTS.get(&key) {
            if let Some(i) = o.get(self.general.language as usize) {
                return i;
            }
        }
        "text_not_found"
    }
}

const DEMO_RULE:Rule = Rule{
    NUMBER_OF_DECK: 1,
    LATE_SURRENDER: false,
    DOUBLE_AFTER_SPLIT : false,
    RE_SPLIT : false,
    ACTION_AFTER_SPLITTING_ACE : false,
    DEALER_PEEKS_ACE : true, 
    DEALER_PEEKS_TEN : false,
    BJ_PAYBACK : 1.5,
    BJ_AFTER_SPLIT : false,
    DEALER_SOFT_17_STAND : false,
    CHARLIE : None,
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

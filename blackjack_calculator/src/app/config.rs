use std::{io::Write, collections::HashMap};
use once_cell::sync::Lazy;

use super::*;

pub mod rule_setting_window;
pub use rule_setting_window::*;

pub mod key_setting;
pub use key_setting::*;

pub mod general_setting;
pub use general_setting::*;

pub mod texts;
pub use texts::*;


#[derive(Default,serde::Serialize,serde::Deserialize)]
pub struct Config {
    pub rule: Rule,
    pub kyes: Keys,
    pub general: GeneralSetting,
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
    const TEXTS:Lazy<HashMap<TextKey,Vec<&'static str>>> = Lazy::new(||{
        load_texts()
    });
    pub fn get_text(&self,key:TextKey) -> &'static str{
        if let Some(o) = Self::TEXTS.get(&key){
            if let Some(i) = o.get(self.general.language as usize){
                return i
            }
        }
        "text_not_found"
    }
}


#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn lang_test(){
        let mut config = Config::default();
        config.general.language = Language::Japanese;
        let text = config.get_text(TextKey::BuyWindowName);
        println!("{text}");
    }
}

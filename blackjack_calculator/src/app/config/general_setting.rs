use std::collections::HashMap;
//use super::*;

mod texts;
pub use texts::*;


#[derive(PartialEq,Eq,Hash,Clone,serde::Serialize,serde::Deserialize)]
pub struct General{
    pub language: Language,
}
impl Default for General{
    fn default() -> Self {
        General{
            language:Language::English,
        }
    }
}

#[derive(PartialEq,Eq,Hash,Clone,Copy,serde::Serialize,serde::Deserialize)]
#[repr(usize)]
pub enum Language{
    English,
    Japanese,
}
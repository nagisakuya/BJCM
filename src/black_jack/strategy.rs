
use super::*;

pub mod basic_strategy;

pub trait Strategy{
    fn get(&self,player:&Player,dealer:&Dealer) -> Action;
}
pub enum Action {
    Hit,
    Stand,
    Split,
    Double,
    Surrender,
}
impl fmt::Display for Action{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self{
            Action::Hit => write!(f,"Hit")?,
            Action::Stand => write!(f,"Stand")?,
            Action::Split => write!(f,"Split")?,
            Action::Double => write!(f,"Double")?,
            Action::Surrender => write!(f,"Surrender")?,
        }
        Ok(())
    }
}


    
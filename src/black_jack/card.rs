use std::fmt;


#[derive(Clone,PartialEq)]
pub struct Card {
    pub suit: u8,//1がA ... 10がT
}

impl Card {
    pub fn new(i: usize) -> Result<Self, String> {
        match i {
            1..=10 => Ok(Card { suit: i as u8 }),
            _ => Err(String::from("Cardを1~10以外で定義しようとしました")),
        }
    }
    pub fn to_usize(&self) -> usize{
        self.suit as usize - 1
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.suit {
            1 => write!(f,"A")?,
            2..=9 => write!(f,"{}",self.suit)?,
            10 => write!(f,"T")?,
            _ => write!(f,"_")?,
        }
        Ok(())
    }
}

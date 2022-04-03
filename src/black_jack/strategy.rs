use std::fs;
use std::str::FromStr;
use super::*;
use super::rule;

const HARD_PATH:&str = "./Strategy/BasicStrategy/hard.txt";
const SOFT_PATH:&str = "./Strategy/BasicStrategy/soft.txt";
const SPLIT_PATH:&str = "./Strategy/BasicStrategy/split.txt";

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

#[derive(Clone, Copy)]
enum Strategy {
    Hit,
    Stand,
    Split,
    NoSplit,
    SplitIfDoubleable,
    Double,
    DoubleElseStand,
    Surrender,
    SurrenderElseStand,
}
impl FromStr for Strategy {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "H" => Ok(Strategy::Hit),
            "S" => Ok(Strategy::Stand),
            "P" => Ok(Strategy::Split),
            "N" => Ok(Strategy::NoSplit),
            "Ph" => Ok(Strategy::SplitIfDoubleable),
            "D" => Ok(Strategy::Double),
            "Ds" => Ok(Strategy::DoubleElseStand),
            "Rh" => Ok(Strategy::Surrender),
            "Rs" => Ok(Strategy::SurrenderElseStand),
            _ => Err(String::from("Wrong Strategy text")),
        }
    }
}

fn load_strategy_file(path: &str, array: &mut [[Strategy;10]]) {
    let mut string = fs::read_to_string(path).expect("Wrong path");
    string.retain(|c| c != '\r');
    let lines = string.split("\n");
    for (i, line) in lines.enumerate() {
        for (j, item) in line.split("\t").enumerate() {
            array[i][j] = Strategy::from_str(item).unwrap();
        }
    }
}

struct HardHandStrategy {
    content: [[Strategy; 10]; 10],//content[player][dealer]
}
impl HardHandStrategy {
    fn new() -> Self {
        HardHandStrategy {
            content: [[Strategy::Hit; 10]; 10],
        }
    }
    fn load_strategy(path: &str) -> Self {
        let mut temp = Self::new();
        load_strategy_file(path,&mut temp.content);
        temp
    }
}

struct SoftHandStrategy {
    content: [[Strategy; 10]; 10],
}
impl SoftHandStrategy {
    fn new() -> Self {
        SoftHandStrategy {
            content: [[Strategy::Hit; 10]; 10],
        }
    }
    fn load_strategy(path: &str) -> Self {
        let mut temp = Self::new();
        load_strategy_file(path,&mut temp.content);
        temp
    }
}

struct SplittableHandStrategy {
    content: [[Strategy; 10]; 10],
}
impl SplittableHandStrategy {
    fn new() -> Self {
        SplittableHandStrategy {
            content: [[Strategy::Hit; 10]; 10],
        }
    }
    fn load_strategy(path: &str) -> Self {
        let mut temp = Self::new();
        load_strategy_file(path,&mut temp.content);
        temp
    }
}

pub struct BasicStrategy{
    hard:HardHandStrategy,
    soft:SoftHandStrategy,
    split:SplittableHandStrategy,
}
impl BasicStrategy{
    pub fn new() -> Self{
        BasicStrategy{
            hard:HardHandStrategy::load_strategy(HARD_PATH),
            soft:SoftHandStrategy::load_strategy(SOFT_PATH),
            split:SplittableHandStrategy::load_strategy(SPLIT_PATH),
        }
    }
    pub fn get(&self,player:&Player,dealer:&Dealer) -> Action{
        let dealer_upcard:usize = dealer.hand[0].to_usize();
        if player.splittable(){
            match self.split.content[player.hand[0].to_usize()][dealer_upcard]{
                Strategy::Split => return Action::Split,
                Strategy::SplitIfDoubleable => {
                    if rule::DOUBLE_AFTER_SPLIT {return Action::Split;} 
                },
                Strategy::NoSplit => {},
                _ => panic!("something wrong"),
            }
        }
        let stra:Strategy; 
        let (player_sum,player_soft) = player.hand.status();
        //if player_sum > 21 {return Action::Stand}

        if player_soft {
            stra = self.soft.content[player_sum as usize - 12][dealer_upcard];
        }
        else{
            let mut temp = player_sum as usize;
            if temp<=8 {temp = 0}
            else if temp>=17 {temp = 9}
            else {temp -= 8}
            stra = self.hard.content[temp][dealer_upcard];
        }
        
        match stra{
            Strategy::Hit => Action::Hit,
            Strategy::Stand => Action::Stand,
            Strategy::Double =>{
                if player.doubleable() {Action::Double}
                else {Action::Hit}
            },
            Strategy::DoubleElseStand =>{
                if player.doubleable() {Action::Double}
                else {Action::Stand}
            },
            Strategy::Surrender =>{
                if rule::SURRENDER {Action::Surrender}
                else {Action::Hit}
            },
            Strategy::SurrenderElseStand =>{
                if rule::SURRENDER {Action::Surrender}
                else {Action::Stand}
            },
            _ => {panic!("something wrong")}
        }
    }
}

#[cfg(test)]
#[allow(unused_mut)]
pub mod tests {
    use super::*;
    #[test]
    fn strategy_test() {
        let stra = BasicStrategy::new();
        for i in 1..=10{
            let mut player = Player::from_arr(&[7,9]);
            //player.splitted = true;
            let dealer = Dealer::from_arr(&[i]);
            let a = stra.get(&player,&dealer);
            println!("{}",a);
        }
    }
}

use std::fs;
use std::str::FromStr;
use super::*;

const HARD_PATH:&str = "./Strategy/BasicStrategy/hard.txt";
const SOFT_PATH:&str = "./Strategy/BasicStrategy/soft.txt";
const SPLIT_PATH:&str = "./Strategy/BasicStrategy/split.txt";

#[derive(Clone, Copy)]
enum BasicStrategyAction {
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
impl FromStr for BasicStrategyAction {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "H" => Ok(BasicStrategyAction::Hit),
            "S" => Ok(BasicStrategyAction::Stand),
            "P" => Ok(BasicStrategyAction::Split),
            "N" => Ok(BasicStrategyAction::NoSplit),
            "Ph" => Ok(BasicStrategyAction::SplitIfDoubleable),
            "D" => Ok(BasicStrategyAction::Double),
            "Ds" => Ok(BasicStrategyAction::DoubleElseStand),
            "Rh" => Ok(BasicStrategyAction::Surrender),
            "Rs" => Ok(BasicStrategyAction::SurrenderElseStand),
            _ => Err(String::from("Wrong Strategy text")),
        }
    }
}

fn load_strategy_file(path: &str, array: &mut [[BasicStrategyAction;10]]) {
    let mut string = fs::read_to_string(path).expect("Wrong path");
    string.retain(|c| c != '\r');
    let lines = string.split("\n");
    for (i, line) in lines.enumerate() {
        for (j, item) in line.split("\t").enumerate() {
            array[i][j] = BasicStrategyAction::from_str(item).unwrap();
        }
    }
}

struct HardHandStrategy {
    content: [[BasicStrategyAction; 10]; 10],//content[player][dealer]
}
impl HardHandStrategy {
    fn new() -> Self {
        HardHandStrategy {
            content: [[BasicStrategyAction::Hit; 10]; 10],
        }
    }
    fn load_strategy(path: &str) -> Self {
        let mut temp = Self::new();
        load_strategy_file(path,&mut temp.content);
        temp
    }
}

struct SoftHandStrategy {
    content: [[BasicStrategyAction; 10]; 10],
}
impl SoftHandStrategy {
    fn new() -> Self {
        SoftHandStrategy {
            content: [[BasicStrategyAction::Hit; 10]; 10],
        }
    }
    fn load_strategy(path: &str) -> Self {
        let mut temp = Self::new();
        load_strategy_file(path,&mut temp.content);
        temp
    }
}

struct SplittableHandStrategy {
    content: [[BasicStrategyAction; 10]; 10],
}
impl SplittableHandStrategy {
    fn new() -> Self {
        SplittableHandStrategy {
            content: [[BasicStrategyAction::Hit; 10]; 10],
        }
    }
    fn load_strategy(path: &str) -> Self {
        let mut temp = Self::new();
        load_strategy_file(path,&mut temp.content);
        temp
    }
}

pub struct BasicStrategy8{
    hard:HardHandStrategy,
    soft:SoftHandStrategy,
    split:SplittableHandStrategy,
}
impl BasicStrategy8{
    pub fn new() -> Self{
        BasicStrategy8{
            hard:HardHandStrategy::load_strategy(HARD_PATH),
            soft:SoftHandStrategy::load_strategy(SOFT_PATH),
            split:SplittableHandStrategy::load_strategy(SPLIT_PATH),
        }
    }
}

impl Strategy for BasicStrategy8{
    fn get(&self,player:&Player,dealer:&Dealer) -> Action{
        let dealer_upcard:usize = dealer.hand[0].to_usize();
        if player.splittable(){
            match self.split.content[player.hand[0].to_usize()][dealer_upcard]{
                BasicStrategyAction::Split => return Action::Split,
                BasicStrategyAction::SplitIfDoubleable => {
                    if rule::DOUBLE_AFTER_SPLIT {return Action::Split;} 
                },
                BasicStrategyAction::NoSplit => {},
                _ => panic!("something wrong"),
            }
        }
        let stra:BasicStrategyAction; 
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
            BasicStrategyAction::Hit => Action::Hit,
            BasicStrategyAction::Stand => Action::Stand,
            BasicStrategyAction::Double =>{
                if player.doubleable() {Action::Double}
                else {Action::Hit}
            },
            BasicStrategyAction::DoubleElseStand =>{
                if player.doubleable() {Action::Double}
                else {Action::Stand}
            },
            BasicStrategyAction::Surrender =>{
                if rule::SURRENDER {Action::Surrender}
                else {Action::Hit}
            },
            BasicStrategyAction::SurrenderElseStand =>{
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
        let stra = BasicStrategy8::new();
        for i in 1..=10{
            let mut player = Player::from_arr(&[7,9]);
            //player.splitted = true;
            let dealer = Dealer::from_arr(&[i]);
            let a = stra.get(&player,&dealer);
            println!("{}",a);
        }
    }
}

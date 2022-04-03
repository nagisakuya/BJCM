pub mod card;
pub mod deck;
pub mod hand;
pub mod rule;
pub mod strategy;

use std::fmt;

use deck::*;
use hand::*;

pub enum BJResult{
    BJ,
    Win,
    Push,
    Lose,
    Surrender,
}

pub fn judge(player:&Player,dealer:&Dealer) -> BJResult{
    if player.surrender {return BJResult::Surrender;}

    let (dealer_sum,is_dealer_bj) = dealer.status();
    let (player_sum,is_player_bj) = player.status();

    if is_dealer_bj && is_player_bj {return BJResult::Push;}
    if is_dealer_bj {return BJResult::Lose;}
    if is_player_bj {return BJResult::BJ;}

    let is_player_bust = player_sum > 21;
    if is_player_bust {return BJResult::Lose;}

    let is_dealer_bust = dealer_sum > 21;
    if is_dealer_bust {return BJResult::Win;}
    
    if player_sum > dealer_sum {return BJResult::Win;}
    else if player_sum < dealer_sum {return BJResult::Lose;}
    else {return BJResult::Push;}
}


#[derive(Clone)]
pub struct Dealer {
    hand: Hand,
}
impl Dealer {
    fn new() -> Self{
        Dealer {
            hand: Hand::new()
        }
    }
    pub fn create(deck:&mut Deck) -> Self{
        let mut temp = Self::new();
        temp.hand.add(deck.draw_random());
        if rule::DEALER_HOLE_CARD {temp.hand.add(deck.draw_random());}
        temp
    }
    pub fn from_arr(arr:&[usize]) -> Self{
        Dealer {
            hand: Hand::from_arr(arr).unwrap(),
        }
    }
    pub fn drow(&mut self, deck: &mut Deck) {
        loop {
            let (sum,_) = self.hand.status();
            if sum>= 17 {break;}
            self.hand.add(deck.draw_random());
        }
    }
    pub fn status(&self) -> (u8,bool){
        let (sum,_) = self.hand.status();
        let is_bj = sum == 21 && self.hand.len() == 2;
        (sum,is_bj)
    }
}

#[derive(Clone)]
pub struct Player {
    hand: Hand,
    splitted: bool,
    pub doubled: bool,
    surrender: bool,
}
impl Player{
    fn new() -> Self{
        Player {
            hand: Hand::new(),
            splitted: false,
            doubled: false,
            surrender: false,
        }
    }
    pub fn create(deck:&mut Deck) -> Self{
        let mut temp = Self::new();
        temp.hand.add(deck.draw_random());
        temp.hand.add(deck.draw_random());
        temp
    }
    pub fn from_arr(arr:&[usize]) -> Self{
        Player {
            hand: Hand::from_arr(arr).unwrap(),
            splitted: false,
            doubled: false,
            surrender: false,
        }
    }
    pub fn hit(&mut self, deck: &mut Deck) {
        self.hand.add(deck.draw_random());
    }
    pub fn double(&mut self, deck: &mut Deck) {
        self.hand.add(deck.draw_random());
        self.doubled = true;
    }
    pub fn split(&mut self, deck: &mut Deck) -> (Player,bool) {
        self.hand.pop();
        self.splitted = true;
        let mut clone = self.clone();
        self.hit(deck);
        clone.hit(deck);
        (clone,if self.hand[0].suit == 1 {true} else {false})
    }
    pub fn surrender(&mut self) {
        self.surrender = true
    }
    pub fn doubleable(&self) ->bool{
        self.hand.len()==2 && (!self.splitted || rule::DOUBLE_AFTER_SPLIT)
    }
    pub fn splittable(& self) -> bool{
        self.hand.len()==2 && self.hand[0] == self.hand[1] && (rule::RE_SPLIT || !self.splitted)
    }
    pub fn status(&self) -> (u8,bool){
        let (sum,_) = self.hand.status();
        let is_bj = sum == 21 && self.hand.len() == 2 && !self.splitted;
        (sum,is_bj)
    }
}

impl fmt::Display for Player{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"{} splitted={} doubled={} ",self.hand,self.splitted,self.doubled).unwrap();
        Ok(())
    }
}

#[cfg(test)]
pub mod tests{
    use rand::Rng;

    use super::*;

    #[test]
    fn dealer_test(){
        let mut rng = rand::thread_rng();
        let mut deck = Deck::new(4);
        let mut d = Dealer::from_arr(&[rng.gen_range(1..=10)]);
        d.drow(&mut deck);
        println!("{}\n{:?}",d.hand,d.hand.status());
    }
    #[test]
    fn split_test(){
        let mut deck = Deck::new(4);
        let mut p = Player::from_arr(&[3,3]);
        let (m,flag) = p.split(&mut deck);
        println!("{}",p);
        println!("{}",m);
        println!("{}",flag);
    }
}

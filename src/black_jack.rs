pub mod card;
pub mod deck;
pub mod hand;
pub mod rule;
pub mod strategy;

use std::fmt;

use deck::*;
use hand::*;

enum BJResult{
    BJ,
    Win,
    Push,
    Lose,
}

fn judge(player:&Player,dealer:&Dealer) -> BJResult{
    let (dealer_sum,_) = dealer.hand.status();
    let (player_sum,_) = player.hand.status();
    let is_dealer_bj = dealer_sum == 21 && dealer.hand.len() == 2;
    let is_player_bj = player_sum == 21 && player.hand.len() == 2;

    if is_dealer_bj && is_player_bj {return BJResult::Push;}
    if is_dealer_bj {return BJResult::Lose;}
    if is_player_bj {return BJResult::BJ;}

    let is_dealer_bust = dealer_sum > 21;
    if is_dealer_bust {return BJResult::Win;}
    
    let is_player_bust = player_sum > 21;
    if is_player_bust {return BJResult::Lose;}

    if player_sum > dealer_sum {return BJResult::Win;}
    else if player_sum < dealer_sum {return BJResult::Lose;}
    else {return BJResult::Push;}
}

#[derive(Clone)]
pub struct Dealer {
    hand: Hand,
}

impl Dealer {
    pub fn new(arr:&[usize]) -> Self{
        Dealer {
            hand: Hand::from_arr(arr).unwrap(),
        }
    }
    fn drow(&mut self, deck: &mut Deck,rng:&mut rand::rngs::ThreadRng) {
        loop {
            self.hand.add(deck.draw_random(rng));
            let (sum,_) = self.hand.status();
            if sum>= 17 {break;}
        }
    }
}

#[derive(Clone)]
pub struct Player {
    hand: Hand,
    splitted: bool,
    doubled: bool,
}
impl Player{
    pub fn new(arr:&[usize]) -> Self{
        Player {
            hand: Hand::from_arr(arr).unwrap(),
            splitted: false,
            doubled: false,
        }
    }
    pub fn hit(&mut self, deck: &mut Deck,rng:&mut rand::rngs::ThreadRng) {
        self.hand.add(deck.draw_random(rng));
    }
    pub fn double(&mut self, deck: &mut Deck,rng:&mut rand::rngs::ThreadRng) {
        self.hand.add(deck.draw_random(rng));
        self.doubled = true;
    }
    pub fn split(&mut self, deck: &mut Deck,rng:&mut rand::rngs::ThreadRng) -> Player {
        self.hand.pop();
        self.splitted = true;
        let mut clone = self.clone();
        self.hit(deck,rng);
        clone.hit(deck,rng);
        clone
    }
    pub fn doubleable(&self) ->bool{
        !self.splitted || rule::DOUBLE_AFTER_SPLIT
    }
    pub fn splittable(& self) -> bool{
        self.hand.len()==2 && self.hand[0] == self.hand[1] && (rule::RE_SPLIT || !self.splitted)
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
        let mut d = Dealer::new(&[rng.gen_range(1..=10)]);
        d.drow(&mut deck, &mut rng);
        println!("{}\n{:?}",d.hand,d.hand.status());
    }
    #[test]
    fn split_test(){
        let mut rng = rand::thread_rng();
        let mut deck = Deck::new(4);
        let mut p = Player::new(&[10,10]);
        let m = p.split(&mut deck,&mut rng);
        println!("{}",p);
        println!("{}",m);
    }
}

use std::ops::{
    Index, 
    IndexMut
};
use std::fmt;
use rand::Rng;
use super::card;


#[derive(Clone)]
pub struct Deck{
    content: [u16;10],
    rng:rand::rngs::ThreadRng
}

impl Deck{
    pub fn new(deck_size:u16) -> Self{
        let mut temp = Deck{
            content: [deck_size*4;10],
            rng: rand::thread_rng(),
        };
        temp.content[9] = deck_size * 16;
        temp
    }
    pub fn sum(&self) -> u16{
        self.content.iter().sum()
    }
    pub fn draw_random(&mut self) -> card::Card{
        let f:f32 = self.rng.gen();
        let temp:u16 = (f * (self.sum() as f32)) as u16;
        let mut  pos = 0;
        for i in (0..10).rev(){
            pos += self.content[i];
            if pos > temp {
                self.content[i] -= 1;
                return card::Card::new(i+1).unwrap();
            }
        };
        return card::Card::new(10).unwrap();
    }
}

impl Index<usize> for Deck{
    type Output = u16;
    fn index(&self, index:usize) -> &Self::Output{
        &self.content[index - 1]
    }
}

impl IndexMut<usize> for Deck{
    fn index_mut(&mut self, index:usize) -> &mut Self::Output{
        &mut self.content[index - 1]
    }
}

impl fmt::Display for Deck{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..10{
            write!(f,"{} ",self.content[i]).unwrap();
        }
        Ok(())
    }
}

#[cfg(test)]
mod decktest{
    use crate::black_jack::deck;

    #[test]
    fn deck_test1(){
        let mut result = [0;10];
        let times = 1000000;
        for _ in 0..times{
            let mut deck = deck::Deck::new(1);
            let temp = deck.draw_random();
            result[temp.suit as usize - 1] += 1;
        }
        for &item in result.iter(){
            println!("{}",item as f32/times as f32);
        }
    }
    
    #[test]
    fn deck_test2(){
        let mut deck = deck::Deck::new(1);
        for _ in 0..52 {
            deck.draw_random();
        }
        println!("{}",deck);
    }
}
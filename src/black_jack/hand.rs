use super::card;
use card::*;
use std::ops::{
    Index
};

use std::fmt;
pub struct Hand{
    content:Vec<Card>,
}
impl Hand{
    pub fn new() -> Self{
        Hand{
            content: Vec::new(),
        }
    }
    pub fn add(&mut self,c:Card){
        self.content.push(c);
    }
    pub fn pop(&mut self){
        self.content.pop();
    }
    pub fn len(&self)->usize{
        self.content.len()
    }
    pub fn from_arr(arr:&[usize]) -> Result<Self,String>{
        let mut temp = Hand::new();
        for &item in arr{
            match Card::new(item){
                Ok(o) => temp.add(o),
                Err(e) => return Err(e),
            };
        }
        Ok(temp)
    }
    pub fn status(&self) -> (u8,bool){
        let mut has_ace = false;
        let mut sum = 0;
        let mut softhand = false;
        for item in self.content.iter(){
            sum += item.suit;
            if item.suit == 1 {
                has_ace = true;
            }
        }
        if sum<=11 && has_ace {
            sum += 10;
            softhand = true;
        }
        (sum,softhand)
    }
}

impl Clone for Hand{
    fn clone(&self) -> Hand{
        Hand { 
            content: self.content.to_vec() ,
        }
    }
}

impl fmt::Display for Hand{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for item in self.content.iter(){
            write!(f,"{} ",item)?;
        }
        Ok(())
    }
}

impl Index<usize> for Hand{
    type Output = Card;
    fn index(&self, index:usize) -> &Self::Output{
        &self.content[index]
    }
}
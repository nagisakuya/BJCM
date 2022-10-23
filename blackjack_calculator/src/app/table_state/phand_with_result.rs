use std::thread::JoinHandle;
use blackjack_lib::phand_for_play::PhandForPlay;

use super::*;

pub(super) enum CalculationResult<T> {
    Calculating(Option<JoinHandle<T>>),
    Result(Option<T>),
}
impl<T> CalculationResult<T> {
    pub(super) fn check(&mut self) {
        match self {
            CalculationResult::Calculating(x) => {
                if x.as_ref().unwrap().is_finished() {
                    let temp = CalculationResult::Result(Some(x.take().unwrap().join().unwrap()));
                    *self = temp;
                }
            }
            CalculationResult::Result(_) => (),
        }
    }
}
impl<T> Clone for CalculationResult<T> {
    fn clone(&self) -> Self {
        CalculationResult::Result(None)
    }
}

#[derive(Clone)]
pub(super) struct PhandWithResult{
    phand:PhandForPlay,
    pub result:CalculationResult<Action>,
    pub is_player:bool,
}

impl Default for PhandWithResult{
    fn default() -> Self {
        PhandWithResult{
            phand:PhandForPlay::new(),
            result:CalculationResult::Result(None),
            is_player:false,
        }
    }
}
impl PhandWithResult{
    pub fn new(is_player:bool) -> Self{
        PhandWithResult{
            phand:PhandForPlay::new(),
            result:CalculationResult::Result(None),
            is_player
        }
    }
    pub fn get_phand(&self) -> &Phand{
        &self.phand
    }
    pub fn _clear(&mut self){
        self.phand = PhandForPlay::new();
    }
    pub fn divide(&mut self) -> Self{
        PhandWithResult{
            phand:self.phand.divide(),
            result:CalculationResult::Result(None),
            is_player:self.is_player,
        }
    }
    pub fn _actionable(&self) -> bool{
        self.phand.actionable()
    }
}
impl std::ops::Deref for PhandWithResult{
    type Target = PhandForPlay;
    fn deref(&self) -> &Self::Target {
        &self.phand
    }
}
impl std::ops::DerefMut for PhandWithResult{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.phand
    }
}
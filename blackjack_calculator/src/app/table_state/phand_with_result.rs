use std::thread::JoinHandle;
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
}

impl PhandWithResult{
    pub fn new() -> Self{
        PhandWithResult{
            phand:PhandForPlay::new(),
            result:CalculationResult::Result(None),
        }
    }
    pub fn divide(&mut self) -> Self{
        PhandWithResult{
            phand:self.phand.divide(),
            result:CalculationResult::Result(None),
        }
    }
    pub fn _actionable(&self) -> bool{
        self.phand.actionable()
    }
}

impl PhandTrait for PhandWithResult{
    fn as_mut_phand(&mut self) ->&mut Phand {
        self.phand.as_mut_phand()
    }
    fn as_phand(&self) ->&Phand {
        self.phand.as_phand()
    }
}
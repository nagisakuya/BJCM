use super::*;

#[derive(Clone, PartialEq, Eq)]
pub enum Selected {
    Player(usize),
    Dealer,
    Discard,
}
impl Selected {
    pub fn is_player(&self, i: usize) -> bool {
        if let Selected::Player(t) = self {
            return i.eq(t);
        }
        false
    }
}

#[derive(Clone,PartialEq, Eq)]
pub enum StepperElements {
    DealToDealer,
    _DealToPlayerFromLeft,
    DealToPlayerFromRight,
    PlayerPlaysFromRight,
    _PlayerPlaysFromLeft,
    DiscardIfExist,
    DealerPlays,
}

use StepperElements::*;

#[derive(Clone)]
pub struct Stepper {
    vec: Vec<StepperElements>,
    current: usize,
}

impl Default for Stepper {
    fn default() -> Self {
        Stepper {
            vec: vec![
                DealToPlayerFromRight,
                DealToDealer,
                DealToPlayerFromRight,
                PlayerPlaysFromRight,
                DiscardIfExist,
                DealerPlays,
            ],
            current: 0,
        }
    }
}

impl Stepper {
    pub fn get(&self) -> Option<&StepperElements> {
        self.vec.get(self.current)
    }
    pub fn reset(&mut self, players: usize) -> Option<Selected> {
        self.current = 0;
        self.get_entry_current(players)
    }
    fn get_entry(&self, players: usize, index: usize) -> Option<Selected> {
        if let Some(x) = self.vec.get(index){
            match x {
                DealToDealer | DealerPlays => Some(Selected::Dealer),
                DealToPlayerFromRight | PlayerPlaysFromRight => Some(Selected::Player(players - 1)),
                _DealToPlayerFromLeft | _PlayerPlaysFromLeft => Some(Selected::Player(0)),
                DiscardIfExist => Some(Selected::Discard),
            }
        }else{
            None
        }
    }
    fn get_entry_current(&self, players: usize) -> Option<Selected> {
        self.get_entry(players, self.current)
    }
}

impl TableState {
    fn stepper_move(&mut self) {
        self.stepper.current += 1;
        if self.stepper.get().is_some() && *self.stepper.get().unwrap() == DiscardIfExist && !CONFIG.read().general.infinite{
            self.stepper.current += 1;
        }
        if let Some(x) = self.stepper.get_entry_current(self.players.len()){
            self.selected = x;
        }
    }
    pub fn step(&mut self) {
        if let Some(o) = self.stepper.get() {
            match o {
                DealToDealer | _DealToPlayerFromLeft | DealToPlayerFromRight => self.step_force(),
                _ => {}
            }
        }
    }
    pub fn step_force(&mut self) {
        if let Some(o) = self.stepper.get() {
            match o {
                DealToDealer | DealerPlays | DiscardIfExist => self.stepper_move(),
                _DealToPlayerFromLeft | _PlayerPlaysFromLeft => {
                    if let Selected::Player(ref mut i) = self.selected {
                        *i += 1;
                        if *i == self.players.len() {
                            self.stepper_move()
                        }
                    }
                }
                DealToPlayerFromRight | PlayerPlaysFromRight => {
                    if let Selected::Player(ref mut i) = self.selected {
                        if *i == 0 {
                            self.stepper_move()
                        } else {
                            *i -= 1;
                        }
                    }
                }
            }
        }
    }
}

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

#[derive(Clone)]
enum StepperElements {
    DealToDealer,
    _DealToPlayerFromLeft,
    DealToPlayerFromRight,
    PlayerPlaysFromRight,
    _PlayerPlaysFromLeft,
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
                DealerPlays,
            ],
            current: 0,
        }
    }
}

impl Stepper {
    fn get(&self) -> Option<&StepperElements> {
        self.vec.get(self.current)
    }
    pub fn reset(&mut self, players: usize) -> Selected {
        self.current = 0;
        self.get_entry_current(players)
    }
    fn get_entry(&self, players: usize, index: usize) -> Selected {
        match self.vec.get(index).unwrap() {
            DealToDealer | DealerPlays => Selected::Dealer,
            DealToPlayerFromRight | PlayerPlaysFromRight => Selected::Player(players - 1),
            _DealToPlayerFromLeft | _PlayerPlaysFromLeft => Selected::Player(0),
        }
    }
    fn get_entry_current(&self, players: usize) -> Selected {
        self.get_entry(players, self.current)
    }
}

impl TableState {
    fn stepper_move(&mut self) {
        self.stepper.current += 1;
        self.selected_base = self.stepper.get_entry_current(self.players.len());
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
                DealToDealer | DealerPlays => self.stepper_move(),
                _DealToPlayerFromLeft | _PlayerPlaysFromLeft => {
                    if let Selected::Player(ref mut i) = self.selected_base {
                        *i += 1;
                        if *i == self.players.len() {
                            self.stepper_move()
                        }
                    }
                }
                DealToPlayerFromRight | PlayerPlaysFromRight => {
                    if let Selected::Player(ref mut i) = self.selected_base {
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

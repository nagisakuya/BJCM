use super::*;

use std::io::Read;
use std::os::windows::process::CommandExt;
use std::thread;

mod draw;
mod phand_with_result;
mod selected;
use eframe::epaint::TextShape;
use phand_with_result::*;
use selected::*;

#[derive(Clone)]
pub struct TableState {
    pub(super) deck: Deck,
    players: VecDeque<PhandWithResult>,
    base_players: VecDeque<PhandWithResult>,
    pub(super) dealer: Dealer,
    stepper: Stepper,
    selected: Selected,
    discard: Vec<Card>,
    betsize: Option<u32>,
}
impl TableState {
    pub fn new(config: &Config) -> Self {
        Self {
            deck: Deck::new(config.rule.NUMBER_OF_DECK),
            players: VecDeque::from(vec![PhandWithResult::new(true)]),
            base_players: VecDeque::from(vec![PhandWithResult::new(true)]),
            dealer: Dealer::new(),
            selected: Selected::Player(0),
            betsize: None,
            stepper: Default::default(),
            discard: Vec::new(),
        }
    }
}
impl TableState {
    pub fn update(
        &mut self,
        ctx: &Context,
        config: &Config,
        history: &mut VecDeque<TableState>,
        betsize: u32,
        asset: &mut AssetManager,
        total_ev_handler: &mut TotalEvHandler,
    ) {
        const HISTORY_LIMIT: usize = 100;
        self.check_join_result();
        let previous = self.clone();
        let mut updated = false;
        let mut update_hand_ev = false;
        for i in 0..10 {
            if ctx.input().key_pressed(config.kyes.card[i]) {
                if self.betsize == None {
                    self.betsize = Some(betsize);
                }
                if !self.deck.drawable(i) {
                    continue;
                }
                updated = true;
                update_hand_ev = true;
                match self.selected {
                    Selected::Player(pos) => {
                        let player = self.players.get_mut(pos).unwrap();
                        if let CalculationResult::Result(Some(Action::Double)) = player.result {
                            player.doubled = true;
                        }
                        player.push(Card::new(i).unwrap());

                        self.deck.draw(i);
                    }
                    Selected::Dealer => {
                        self.dealer.push(Card::new(i).unwrap());
                        self.deck.draw(i);
                    }
                    Selected::Discard => {
                        self.discard.push(Card::new(i).unwrap());
                        self.deck.draw(i);
                    }
                }
                self.step(config);
            }
        }
        if ctx.input().key_pressed(config.kyes.remove) {
            updated = true;
            update_hand_ev = true;
            match self.selected {
                Selected::Player(pos) => {
                    let player = self.players.get_mut(pos).unwrap();
                    if let Some(o) = player.pop() {
                        self.deck.add(o);
                    }
                }
                Selected::Dealer => {
                    if let Some(o) = self.dealer.pop() {
                        self.deck.add(o);
                    }
                }
                Selected::Discard => {
                    if let Some(o) = self.discard.pop() {
                        self.deck.add(o);
                    }
                }
            }
        }
        if ctx.input().key_pressed(config.kyes.next) {
            updated = true;
            asset.add_current(self.next());
        }
        if ctx.input().key_pressed(config.kyes.reset) {
            updated = true;
            asset.add_current(self.reset(&config));
            total_ev_handler.reset();
        }
        if ctx.input().key_pressed(config.kyes.step) {
            updated = true;
            self.step_force(config);
        }
        if ctx.input().key_pressed(config.kyes.split) {
            if let Selected::Player(ref mut pos) = self.selected {
                let hand = self.players.get_mut(*pos).unwrap();
                if hand.is_twin() {
                    let temp = hand.divide();
                    self.players.insert(*pos + 1, temp);
                    *pos += 1;
                    updated = true;
                    update_hand_ev = true;
                }
            }
        };
        self.update_selected(ctx, config, &mut updated);
        if updated {
            history.push_back(previous);
            if history.len() > HISTORY_LIMIT {
                history.pop_front();
            }
        }
        if update_hand_ev{
            self.update_hand_ev(&config.rule);
        }
        if ctx.input().key_pressed(config.kyes.undo) {
            if history.len() > 0 {
                *self = history.pop_back().unwrap();
            }
            self.update_hand_ev(&config.rule);
        }
    }

    pub fn reset(&mut self, config: &Config) -> i32 {
        self.deck = Deck::new(config.rule.NUMBER_OF_DECK);
        self.next()
    }

    pub fn next(&mut self) -> i32 {
        let mut profit = 0;
        if let Some(b) = self.betsize {
            for phand in self.players.iter() {
                if self.dealer.len() >= 2 && phand.is_player {
                    profit += (b as f64 * phand.calc_payout(&self.dealer)) as i32;
                }
            }
        }
        self.dealer = Dealer::new();
        self.players = self.base_players.clone();
        self.discard.clear();
        self.betsize = None;
        if let Some(x) = self.stepper.reset(self.players.len()) {
            self.selected = x;
        }

        profit
    }

    fn update_selected(&mut self, ctx: &Context, config: &Config, updated: &mut bool) {
        if ctx.input().key_pressed(config.kyes.right) {
            match self.selected {
                Selected::Player(pos) => {
                    if pos == self.players.len() - 1 {
                        self.selected = Selected::Player(0);
                    } else {
                        self.selected = Selected::Player(pos + 1);
                    }
                    *updated = true;
                }
                Selected::Dealer => {
                    if config.general.infinite {
                        self.selected = Selected::Discard
                    }
                }
                Selected::Discard => self.selected = Selected::Dealer,
            }
        };
        if ctx.input().key_pressed(config.kyes.left) {
            match self.selected {
                Selected::Player(pos) => {
                    if pos == 0 {
                        if config.general.infinite {
                            self.selected = Selected::Discard
                        } else {
                            self.selected = Selected::Player(self.players.len() - 1);
                        }
                    } else {
                        self.selected = Selected::Player(pos - 1);
                    }
                    *updated = true;
                }
                Selected::Dealer => {
                    if config.general.infinite {
                        self.selected = Selected::Discard
                    }
                }
                Selected::Discard => self.selected = Selected::Dealer,
            }
        };
        if ctx.input().key_pressed(config.kyes.up) {
            match self.selected {
                Selected::Player(_) => self.selected = Selected::Dealer,
                Selected::Dealer => {
                    self.selected = Selected::Player(self.players.len() / 2)
                }
                Selected::Discard =>{
                    self.selected = Selected::Dealer
                }
            }
            *updated = true;
        };
        if ctx.input().key_pressed(config.kyes.down) {
            match self.selected {
                Selected::Player(_) => self.selected = Selected::Dealer,
                Selected::Dealer | Selected::Discard => {
                    self.selected = Selected::Player(self.players.len() / 2)
                }
            }
            *updated = true;
        };
    }

    fn update_hand_ev(&mut self, rule: &Rule) {
        for phand in self.players.iter_mut() {
            if self.dealer.len() == 1 && phand.len() >= 2 && phand.is_player && phand.actionable() {
                let phand_str =
                    io_util::bytes_to_string(&bincode::serialize(&phand.get_phand()).unwrap());
                let dealer = io_util::bytes_to_string(&bincode::serialize(&self.dealer).unwrap());
                let deck = io_util::bytes_to_string(&bincode::serialize(&self.deck).unwrap());
                let rule = io_util::bytes_to_string(&bincode::serialize(&rule).unwrap());
                let closure = move || {
                    let process = std::process::Command::new(SUBPROCESS_PATH)
                        .arg(deck)
                        .arg(phand_str)
                        .arg(dealer)
                        .env("RULE", &rule)
                        .stdout(std::process::Stdio::piped())
                        .creation_flags(0x08000000)
                        .spawn()
                        .unwrap();
                    let mut string = String::new();
                    process.stdout.unwrap().read_to_string(&mut string).unwrap();
                    let t: usize = string.parse().unwrap();
                    Action::from_usize(t)
                };
                phand.result = CalculationResult::Calculating(Some(thread::spawn(closure)));
            } else {
                phand.result = CalculationResult::Result(None)
            }
        }
    }
    fn check_join_result(&mut self) {
        for phand in self.players.iter_mut() {
            phand.result.check();
        }
    }
}

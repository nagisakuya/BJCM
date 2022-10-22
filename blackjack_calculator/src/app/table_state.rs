use super::*;

use std::io::Read;
use std::os::windows::process::CommandExt;
use std::thread;

mod phand_with_result;
mod draw;
mod selected;
use selected::*;
use eframe::epaint::TextShape;
use phand_with_result::*;


#[derive(Clone)]
pub struct TableState {
    pub(super) deck: Deck,
    players: VecDeque<PhandWithResult>,
    base_players: VecDeque<PhandWithResult>,
    dealer: Dealer,
    stepper: Stepper,
    selected_current: Selected,
    selected_base: Selected,
    discard: Vec<Card>,
    card_texture: [Option<TextureHandle>; 10],
}
impl TableState {
    pub fn new(config: &Config) -> Self {
        Self {
            deck: Deck::new(config.rule.NUMBER_OF_DECK),
            players: VecDeque::from(vec![PhandWithResult::default()]),
            base_players: VecDeque::from(vec![PhandWithResult::default()]),
            dealer: Dealer::new(),
            selected_current: Selected::Player(0),
            selected_base: Selected::Player(0),
            discard: Vec::new(),
            card_texture: Default::default(),
            stepper: Default::default(),
        }
    }
}
impl TableState {
    pub fn setup(&mut self, cc: &eframe::CreationContext<'_>) {
        for i in 0..10 {
            let path = &format!("{}/{}.png", IMAGE_FOLDER_PATH, i + 1);
            let image = load_image_from_path(path).unwrap();
            self.card_texture[i] = Some(cc.egui_ctx.load_texture(&format!("card_{}", i), image));
        }
    }

    pub fn update(&mut self, ctx: &Context, config: &Config, history: &mut VecDeque<TableState>) {
        const HISTORY_LIMIT:usize = 100;
        self.check_join_result();
        let previous = self.clone();
        let mut updated = false;
        for i in 0..10 {
            if ctx.input().key_pressed(config.kyes.card[i]) {
                if !self.deck.drawable(i) {
                    continue;
                }
                updated = true;
                match self.selected_current {
                    Selected::Player(pos) => {
                        self.players.get_mut(pos).unwrap().push(Card::new(i).unwrap());
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
                self.step();
            }
        }
        if ctx.input().key_pressed(config.kyes.next) {
            updated = true;
            self.next();
        }
        if ctx.input().key_pressed(config.kyes.reset) {
            updated = true;
            self.reset(&config);
        }
        if ctx.input().key_pressed(config.kyes.step) {
            self.step_force();
        }
        if ctx.input().key_pressed(config.kyes.split) {
            if let Selected::Player(pos) = self.selected_current {
                let hand = self.players.get_mut(pos).unwrap();
                if hand.is_twin() {
                    let temp = hand.divide();
                    self.players.insert(pos + 1, temp);
                    updated = true;
                }
            }
        };
        if updated {
            history.push_back(previous);
            if history.len() > HISTORY_LIMIT {
                history.pop_front();
            }
            self.update_hand_ev(&config.rule);
        }
        if ctx.input().key_pressed(config.kyes.undo) {
            if history.len() > 0 {
                *self = history.pop_back().unwrap();
            }
            self.update_hand_ev(&config.rule);
        }
        self.update_selected(ctx, config);
    }

    pub fn reset(&mut self, config: &Config) {
        self.next();
        self.deck = Deck::new(config.rule.NUMBER_OF_DECK);
    }

    pub fn next(&mut self) {
        self.dealer = Dealer::new();
        self.players = self.base_players.clone();
        self.discard = Vec::new();
        self.selected_base = Selected::Dealer;
        self.stepper.reset(self.players.len());
    }
    
    fn update_selected(&mut self, ctx: &Context, config: &Config) {
        if ctx.input().key_pressed(config.kyes.right) {
            match self.selected_base {
                Selected::Player(pos) => {
                    if pos == self.players.len() - 1 {
                        self.selected_base = Selected::Player(0);
                    } else {
                        self.selected_base = Selected::Player(pos + 1);
                    }
                }
                Selected::Dealer => self.selected_base = Selected::Discard,
                Selected::Discard => self.selected_base = Selected::Dealer,
            }
        };
        if ctx.input().key_pressed(config.kyes.left) {
            match self.selected_base {
                Selected::Player(pos) => {
                    if pos == 0 {
                        self.selected_base = Selected::Player(self.players.len() - 1);
                    } else {
                        self.selected_base = Selected::Player(pos - 1);
                    }
                }
                Selected::Dealer => self.selected_base = Selected::Discard,
                Selected::Discard => self.selected_base = Selected::Dealer,
            }
        };
        if ctx.input().key_pressed(config.kyes.up) || ctx.input().key_pressed(config.kyes.down) {
            match self.selected_base {
                Selected::Player(_) => self.selected_base = Selected::Dealer,
                Selected::Dealer => self.selected_base = Selected::Player(0),
                Selected::Discard => self.selected_base = Selected::Player(0),
            }
        };

        //更新
        if ctx.input().key_down(config.kyes.dealer) {
            self.selected_current = Selected::Dealer;
        } else if ctx.input().key_down(config.kyes.discard) {
            self.selected_current = Selected::Discard;
        } else {
            self.selected_current = self.selected_base.clone();
        }
    }

    fn update_hand_ev(&mut self, rule: &Rule) {
        for phand in self.players.iter_mut(){
            if self.dealer.len() == 1 && phand.len() >= 2 {
                let phand_str =
                    io_util::bytes_to_string(&bincode::serialize(&phand.as_phand()).unwrap());
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
        for phand in self.players.iter_mut(){
            phand.result.check();
        }
    }
    
}

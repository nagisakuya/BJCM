use super::*;

use std::thread;
use std::os::windows::process::CommandExt;
use std::io::Read;

mod phand_with_result;
use phand_with_result::*;

#[derive(Clone, PartialEq, Eq)]
enum Selected {
    Player(usize),
    Dealer,
    Discard,
}
impl Selected {
    fn is_player(&self, i: usize) -> bool {
        if let Selected::Player(t) = self {
            return i.eq(t);
        }
        false
    }
}
#[derive(Clone)]
pub struct TableState {
    pub(super) deck: Deck,
    player: Player<PhandWithResult>,
    dealer: Dealer,
    selected: Selected,
    discard: Vec<Card>,
    card_texture: [Option<TextureHandle>; 10],
}
impl TableState{
    pub fn new(rule:&Rule) -> Self{
        Self {
            deck: Deck::new(rule.NUMBER_OF_DECK),
            player: Player::Single(PhandWithResult::new()),
            dealer: Dealer::new(),
            selected: Selected::Dealer,
            discard: Vec::new(),
            card_texture: Default::default(),
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
    pub fn update(&mut self, ctx: &Context,config:&Config,history: &mut VecDeque<TableState>) {
        self.check_join_result();
        let previous = self.clone();
        let mut updated = false;
        for i in 0..10 {
            if ctx.input().key_pressed(config.kyes.card[i]) {
                if !self.deck.drawable(i) {
                    continue;
                }
                updated = true;
                match self.selected {
                    Selected::Player(pos) => {
                        self.player
                            .get_mut(pos)
                            .push(Card::new(i).unwrap());
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
            }
        }
        if ctx.input().key_pressed(config.kyes.next) {
            updated = true;
            self.next(&config.rule);
        }
        if ctx.input().key_pressed(config.kyes.reset) {
            updated = true;
            self.reset(&config.rule);
        }
        if ctx.input().key_pressed(config.kyes.split) {
            if let Selected::Player(pos) = self.selected {
                let hand = self.player.get_mut(pos);
                if hand.is_twin() {
                    let temp = hand.divide();
                    self.player.insert(pos + 1, temp);
                    updated = true;
                }
            }
        };
        if updated {
            history.push_back(previous);
            if history.len() > 10 {
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
        self.updare_selected(ctx,config);
    }
    pub fn reset(&mut self,rule:&Rule){
        *self = Self{
            card_texture:self.card_texture.clone(),
            ..Self::new(rule)
        }
    }
    fn next(&mut self,rule:&Rule){
        *self = Self{
            card_texture:self.card_texture.clone(),
            deck:self.deck.clone(),
            ..Self::new(rule)
        }
    }
    fn updare_selected(&mut self,ctx:&Context,config:&Config){
        if ctx.input().key_pressed(config.kyes.right) {
            match self.selected {
                Selected::Player(pos) => {
                    if pos == self.player.len() - 1 {
                        self.selected = Selected::Player(0);
                    } else {
                        self.selected = Selected::Player(pos + 1);
                    }
                }
                Selected::Dealer => self.selected = Selected::Discard,
                Selected::Discard => self.selected = Selected::Dealer,
            }
        };
        if ctx.input().key_pressed(config.kyes.left) {
            match self.selected {
                Selected::Player(pos) => {
                    if pos == 0 {
                        self.selected =
                            Selected::Player(self.player.len() - 1);
                    } else {
                        self.selected = Selected::Player(pos - 1);
                    }
                }
                Selected::Dealer => self.selected = Selected::Discard,
                Selected::Discard => self.selected = Selected::Dealer,
            }
        };
        if ctx.input().key_pressed(config.kyes.up) || ctx.input().key_pressed(config.kyes.down) {
            match self.selected {
                Selected::Player(_) => self.selected = Selected::Dealer,
                Selected::Dealer => self.selected = Selected::Player(0),
                Selected::Discard => self.selected = Selected::Player(0),
            }
        };
    }
    fn update_hand_ev(&mut self, rule: &Rule) {
        self.player.for_each_mut(|phand, _| {
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
        });
    }
    fn check_join_result(&mut self) {
        self.player.for_each_mut(|phand, _| {
            phand.result.check();
        });
    }
    const CARD_SIZE: Vec2 = vec2(75.0, 135.0);
    pub fn draw_table(&self, ui: &mut Ui) {
        let available_size = ui.available_size();
        //draw dealer
        {
            let dealer_root = pos2(available_size.x / 2.0 - Self::CARD_SIZE.x / 2.0, 80.0);
            let highlight = self.selected == Selected::Dealer;
            self.draw_hand(ui, self.dealer.as_slice(), dealer_root, highlight);
        }

        //draw player
        {
            self.player.for_each(|p, i| {
                let space_bottom = 35.0;
                let space = available_size.x / (self.player.len() + 1) as f32;
                let root_position = pos2(
                    space * (i + 1) as f32 - Self::CARD_SIZE.x / 2.0,
                    available_size.y - space_bottom - Self::CARD_SIZE.y,
                );
                let highlight = self.selected.is_player(i);
                self.draw_hand(ui, p.as_slice(), root_position, highlight);

                let widget_rect = Rect::from_min_size(
                    root_position + vec2(0.0, Self::CARD_SIZE.y + 5.0),
                    vec2(Self::CARD_SIZE.x, space_bottom),
                );
                match p.result {
                    CalculationResult::Calculating(_) => {
                        ui.put(widget_rect, Spinner::new().size(15.0));
                    },
                    CalculationResult::Result(Some(x)) =>{
                        ui.put(
                            widget_rect,
                            Label::new(&format!("{:?}", x)),
                        );
                    },
                    _ => (),
                };
            });
        }

        //draw discard area
        {
            let root_position = pos2(available_size.x - Self::CARD_SIZE.x - 40.0, 80.0);
            let rect = Rect::from_min_size(
                pos2(root_position.x - 20.0, root_position.y - 40.0),
                vec2(
                    Self::CARD_SIZE.x + 50.0,
                    Self::CARD_SIZE.y + 60.0,
                ),
            );
            ui.painter()
                .rect_filled(rect, Rounding::same(5.0), Color32::from_rgb(75, 0, 0));
            let highlight = self.selected == Selected::Discard;
            self.draw_hand(ui, self.discard.as_slice(), root_position, highlight);
        }
    }
    pub fn draw_deck(&self, ui: &mut Ui){
        for i in 0..10 {
            ui.label(
                RichText::new(format!(
                    "{}={:>2}",
                    Card::new(i).unwrap(),
                    self.deck[i]
                )),
            );
        }
    }
    fn draw_hand(&self, ui: &mut Ui, cards: &[Card], root_position: Pos2, highlight: bool) {
        if highlight {
            let rect = Rect::from_min_size(
                pos2(
                    root_position.x - 10.0,
                    root_position.y - 10.0 * cards.len() as f32,
                ),
                vec2(
                    Self::CARD_SIZE.x + 10.0 * cards.len() as f32 + 10.0,
                    Self::CARD_SIZE.y + cards.len() as f32 * 10.0 + 10.0,
                ),
            );
            ui.painter()
                .rect_filled(rect, Rounding::same(5.0), Color32::from_rgb(100, 100, 100));
        }
        for (i, item) in cards.iter().enumerate() {
            let upper_right_pos = root_position + vec2(10.0 * i as f32, -10.0 * i as f32);
            let widget_rect = Rect::from_min_size(upper_right_pos, Self::CARD_SIZE);
            ui.put(
                widget_rect,
                Image::new(
                    self.card_texture[item.to_usize()].as_ref().unwrap(),
                    Self::CARD_SIZE,
                ),
            );
        }
    }
}

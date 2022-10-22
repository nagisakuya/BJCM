use super::*;

use std::io::Read;
use std::os::windows::process::CommandExt;
use std::thread;

mod phand_with_result;
use eframe::epaint::TextShape;
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
    selected_current: Selected,
    selected_base: Selected,
    discard: Vec<Card>,
    card_texture: [Option<TextureHandle>; 10],
}
impl TableState {
    pub fn new(config: &Config) -> Self {
        Self {
            deck: Deck::new(config.rule.NUMBER_OF_DECK),
            player: match config.general.number_of_player {
                1 => Player::Single(PhandWithResult::new()),
                x => Player::Splitted(vec![PhandWithResult::new(); x]),
            },
            dealer: Dealer::new(),
            selected_current: Selected::Player(0),
            selected_base: Selected::Player(0),
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
    pub fn update(&mut self, ctx: &Context, config: &Config, history: &mut VecDeque<TableState>) {
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
                        self.player.get_mut(pos).push(Card::new(i).unwrap());
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
            self.next(&config);
        }
        if ctx.input().key_pressed(config.kyes.reset) {
            updated = true;
            self.reset(&config);
        }
        if ctx.input().key_pressed(config.kyes.split) {
            if let Selected::Player(pos) = self.selected_current {
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
        self.updare_selected(ctx, config);
    }
    pub fn reset(&mut self, config: &Config) {
        *self = Self {
            card_texture: self.card_texture.clone(),
            ..Self::new(config)
        }
    }
    pub fn next(&mut self, config: &Config) {
        *self = Self {
            card_texture: self.card_texture.clone(),
            deck: self.deck.clone(),
            ..Self::new(config)
        }
    }
    fn updare_selected(&mut self, ctx: &Context, config: &Config) {
        if ctx.input().key_pressed(config.kyes.right) {
            match self.selected_base {
                Selected::Player(pos) => {
                    if pos == self.player.len() - 1 {
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
                        self.selected_base = Selected::Player(self.player.len() - 1);
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
    
    pub fn show_deck(&self, ui: &mut Ui, config: &Config) {
        const GRIFFIN_ULTIMATE: [f64; 10] =
            [-60.0, 37.0, 45.0, 52.0, 70.0, 46.0, 27.0, 0.0, -17.0, -50.0];
        const GRIFFIN_ULTIMATE_MULTIPLER: f64 = 0.008;
        const DEFAULT_RTP: f64 = -0.554;
        let mut count = 0.0;
        for i in 0..10 {
            ui.label(RichText::new(format!(
                "{}={:>2}",
                Card::new(i).unwrap(),
                self.deck[i]
            )));
            count += ((config.rule.NUMBER_OF_DECK * if i != 9 { 4 } else { 16 }) - self.deck[i])
                as f64
                * GRIFFIN_ULTIMATE[i];
        }
        count = count * GRIFFIN_ULTIMATE_MULTIPLER / config.rule.NUMBER_OF_DECK as f64;
        ui.label(RichText::new(format!(
            "{}={:4>+1.3}%",
            config.get_text(TextKey::EstimatedRTPLabel),
            (count + DEFAULT_RTP)
        )));
    }

    pub fn draw_table(&self, ui: &mut Ui) {
        let available_rect = ui.min_rect();

        //draw discard area
        {
            let root_position = pos2(40.0, 80.0);
            let rect = Rect::from_min_size(
                pos2(root_position.x - 20.0, root_position.y - 40.0),
                vec2(Self::CARD_SIZE.x + 50.0, Self::CARD_SIZE.y + 60.0),
            );
            ui.painter()
                .rect_filled(rect, Rounding::same(5.0), Color32::from_rgb(75, 0, 0));
            let highlight = self.selected_current == Selected::Discard;
            self.draw_hand(ui, self.discard.as_slice(), root_position, Vec2::new(25.0, 0.0),highlight);
        }

        //draw dealer
        {
            const TOP_MARGIN:f32 = 20.0;
            let dealer_root = pos2(available_rect.width() / 2.0 - Self::CARD_SIZE.x/2.0 - 25.0, available_rect.top() + TOP_MARGIN);
            let highlight = self.selected_current == Selected::Dealer;
            self.draw_hand(ui, self.dealer.as_slice(), dealer_root,Vec2::new(25.0, 0.0), highlight);
        }

        //draw player
        {
            self.player.for_each(|p, i| {
                const BOTTOM_MARGIN:f32 = 20.0;
                let space_width = available_rect.width() - Self::CARD_SIZE.x * self.player.len() as f32;
                let space_per_hand = space_width / (self.player.len() + 1) as f32;
                let pos = pos2(
                    space_per_hand * (i + 1) as f32 + Self::CARD_SIZE.x * i as f32,
                    available_rect.bottom() - BOTTOM_MARGIN - Self::CARD_SIZE.y,
                );
                let highlight = self.selected_current.is_player(i);
                self.draw_hand(ui, p.as_slice(), pos,Vec2::new(0.0, -30.0), highlight);

                let result_area_rect = Rect::from_min_size(
                    pos + vec2(0.0, Self::CARD_SIZE.y + 5.0),
                    vec2(Self::CARD_SIZE.x, BOTTOM_MARGIN),
                );
                match p.result {
                    CalculationResult::Calculating(_) => {
                        ui.put(result_area_rect, Spinner::new().size(15.0));
                    }
                    CalculationResult::Result(Some(x)) => {
                        ui.put(result_area_rect, Label::new(&format!("{:?}", x)));
                    }
                    _ =>  {
                        ui.put(result_area_rect, Label::new(&format!("◆")));
                    },
                };
            });
        }
    }
    fn draw_hand(&self, ui: &mut Ui, cards: &[Card], pos: Pos2, step:Vec2 , highlight: bool) {
        const OUTER_MARGINE: f32 = 5.0;
        let upper_limit = ui.ctx().available_rect().top() + 10.0;
        if highlight {
            let bottom_left_pos = pos2(
                pos.x - OUTER_MARGINE,
                pos.y + Self::CARD_SIZE.y + OUTER_MARGINE,
            );
            let mut upper_right_pos = pos2(
                pos.x + Self::CARD_SIZE.x + OUTER_MARGINE,
                pos.y - OUTER_MARGINE,
            );
            if cards.len() >= 2 {
                upper_right_pos += step * (cards.len() - 1) as f32;
            }
            let rect = Rect::from_two_pos(bottom_left_pos, upper_right_pos);
            ui.painter()
                .rect_filled(rect, Rounding::same(5.0), Color32::from_rgb(100, 100, 100));
        }
        for (i, item) in cards.iter().enumerate() {
            let mut pos_temp = pos + step * i as f32;
            let mut size = Self::CARD_SIZE;
            if pos_temp.y < upper_limit {
                let temp = upper_limit - pos_temp.y;
                pos_temp.y = upper_limit;
                size.y -= temp;
            }
            Self::draw_card(ui, item, pos_temp);
        }
    }
    const CARD_SIZE: Vec2 = vec2(70.0, 100.0);
    fn draw_card(ui: &mut Ui, card: &Card, pos: Pos2) -> Rect{
        const MARGIN:Vec2 = Vec2::new(0.0, 0.0);
        const TEXT_SIZE:f32 = 35.0;
        const TEXT_WIDTH:f32 = TEXT_SIZE * 0.8;
        let rect = Rect::from_min_size(pos, Self::CARD_SIZE);
        ui.painter()
            .rect_filled(rect, Rounding::same(5.0), Color32::from_rgb(255, 255, 255));
        ui.painter().rect_stroke(
            rect,
            Rounding::same(5.0),
            Stroke::new(1.0, Color32::from_rgb(0, 0, 0)),
        );
        let galley = ui.painter().layout(
            card.to_string(),
            FontId::new(TEXT_SIZE, FontFamily::Name("times_new_roman".into())),
            Color32::BLACK,
            0.0
        );
        
        let margin = Vec2::new((TEXT_WIDTH - galley.size().x) / 2.0,0.0) + MARGIN;

        let upper_text = TextShape::new(rect.min + margin, galley.clone());
        ui.painter().add(upper_text);

        let mut bottom_text = TextShape::new(rect.max - margin, galley);
        bottom_text.angle = std::f32::consts::PI;
        ui.painter().add(bottom_text);

        //center rect
        ui.painter().rect_filled(Rect::from_center_size(rect.center(), Vec2::new(20.0,20.0)), Rounding::none(), Color32::from_rgb(200, 200, 200));

        rect
    }
}

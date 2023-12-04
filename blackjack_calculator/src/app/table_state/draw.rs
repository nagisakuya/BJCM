use super::*;
use config::*;

impl TableState {
    pub fn show_deck(&self, ui: &mut Ui) {
        let max_rect = ui.max_rect();
        const HEIGHT: f32 = 20.0;
        const MARGIN: f32 = 1.0;
        for i in 0..10 {
            let original =
                4.0 * CONFIG.read().rule.NUMBER_OF_DECK as f32 * if i == 9 { 4.0 } else { 1.0 };
            let width = max_rect.width() * self.deck.get(Card::new(i)) as f32 / original;
            let rect = Rect::from_min_size(
                max_rect.min + Vec2::new(0.0, i as f32 * (HEIGHT + MARGIN)),
                Vec2::new(width, HEIGHT),
            );
            ui.painter().rect_filled(rect, 0.0, Color32::from_gray(100));
            let text_shape = Shape::text(
                &ui.fonts(),
                rect.left_bottom(),
                Align2::LEFT_BOTTOM,
                format!("{}:{:>2}", Card::new(i), self.deck[i]),
                FontId::new(HEIGHT - 6.0, FontFamily::Name("noto_sans".into())),
                Color32::from_gray(200),
            );
            ui.painter().add(text_shape);
        }
        ui.add_space(10.0 * (HEIGHT + MARGIN));
    }

    pub fn draw_table(&mut self, ui: &mut Ui) {
        let available_rect = ui.min_rect();
        const TOP_MARGIN: f32 = 20.0;

        //draw dealer
        {
            let dealer_root = pos2(
                available_rect.left() + available_rect.width() / 2.0
                    - Self::CARD_SIZE.x / 2.0
                    - 25.0,
                available_rect.top() + TOP_MARGIN,
            );
            let highlight = self.selected == Selected::Dealer;
            Self::draw_hand(
                ui,
                self.dealer.as_slice(),
                dealer_root,
                Vec2::new(25.0, 0.0),
                highlight,
            );
            if CONFIG.read().rule.INSUALANCE
                && self.dealer.len() == 1
                && self.dealer.get_first().is_ace()
            {
                let insualance = self.deck.insualance_odd() > 0.0;
                let label = Label::new(&format!(
                    "Insualance={}",
                    if insualance { "YES" } else { "NO" }
                ))
                .wrap(false);
                let insualance_text_rect = Rect::from_center_size(
                    dealer_root + vec2(Self::CARD_SIZE.x * 0.5, Self::CARD_SIZE.y + 10.0),
                    vec2(Self::CARD_SIZE.x * 2.0, 20.0),
                );
                ui.put(insualance_text_rect, label);
            }
        }

        //draw discard
        {
            let discard_root = pos2(
                available_rect.left() + Self::CARD_SIZE.x / 2.0,
                available_rect.top() + 100.0,
            );
            let highlight = self.selected == Selected::Discard;
            Self::draw_hand(
                ui,
                self.discard.as_slice(),
                discard_root,
                Vec2::new(0.0, 20.0),
                highlight,
            );
        }

        //draw player
        {
            let players_len = self.players.len() as f32;
            let mut updated = false;
            for (i, phand) in self.players.iter_mut().enumerate() {
                const BOTTOM_MARGIN: f32 = 30.0;
                let space_width = available_rect.width()
                    - Self::CARD_SIZE.x * players_len
                    - SIDE_BUTTON_SIZE * 2.0;
                let space_per_hand = space_width / (players_len + 1.0);
                let pos = pos2(
                    available_rect.left()
                        + SIDE_BUTTON_SIZE
                        + space_per_hand * (i + 1) as f32
                        + Self::CARD_SIZE.x * i as f32,
                    available_rect.bottom() - BOTTOM_MARGIN - Self::CARD_SIZE.y,
                );
                let highlight = self.selected.is_player(i);
                Self::draw_hand(ui, phand.as_slice(), pos, Vec2::new(0.0, -30.0), highlight);

                let result_area_rect = Rect::from_min_size(
                    pos + vec2(0.0, Self::CARD_SIZE.y + 5.0),
                    vec2(Self::CARD_SIZE.x, BOTTOM_MARGIN),
                );
                match phand.result {
                    CalculationResult::Calculating(_) => {
                        ui.put(result_area_rect, Spinner::new().size(15.0));
                    }
                    CalculationResult::Result(Some(x)) => {
                        ui.put(result_area_rect, Label::new(&format!("{:?}", x)));
                    }
                    _ => {
                        let rect = Rect::from_center_size(
                            result_area_rect.center(),
                            Vec2::new(25.0, 25.0),
                        );
                        if phand.is_player {
                            if ui.put(rect, Button::new("◆".to_string())).clicked() {
                                phand.is_player = false;
                                if !phand.splitted {
                                    self.base_players.get_mut(i).unwrap().is_player = false;
                                }
                            };
                        } else if ui.put(rect, Button::new("◇".to_string())).clicked() {
                            phand.is_player = true;
                            if !phand.splitted {
                                self.base_players.get_mut(i).unwrap().is_player = true;
                            }
                            updated = true;
                        };
                    }
                };
            }
            if updated {
                self.update_hand_ev();
            }
        }

        //draw buttons
        const SIDE_BUTTON_SIZE: f32 = 25.0;
        {
            const SPACE: f32 = 7.0;
            let size = Vec2::new(SIDE_BUTTON_SIZE, SIDE_BUTTON_SIZE);
            let rect_right_bottom = Rect::from_min_size(
                available_rect.right_bottom() - Vec2::new(SIDE_BUTTON_SIZE, SIDE_BUTTON_SIZE),
                size,
            );
            let rect_left_bottom = Rect::from_min_size(
                available_rect.left_bottom() - Vec2::new(0.0, SIDE_BUTTON_SIZE),
                size,
            );
            let rect_right_top = Rect::from_min_size(
                available_rect.right_bottom()
                    - Vec2::new(SIDE_BUTTON_SIZE, SIDE_BUTTON_SIZE * 2.0 + SPACE),
                size,
            );
            let rect_left_top = Rect::from_min_size(
                available_rect.left_bottom() - Vec2::new(0.0, SIDE_BUTTON_SIZE * 2.0 + SPACE),
                size,
            );
            if ui.put(rect_right_bottom, Button::new("-")).clicked() {
                let _ = self.players.pop_back();
                let _ = self.base_players.pop_back();
                if let Selected::Player(ref mut i) = self.selected {
                    if *i == self.players.len() && *i != 0 {
                        *i -= 1;
                    }
                }
            };
            if ui.put(rect_left_bottom, Button::new("-")).clicked() {
                let _ = self.players.pop_front();
                let _ = self.base_players.pop_front();
                if let Selected::Player(ref mut i) = self.selected {
                    if *i != 0 {
                        *i -= 1;
                    }
                }
            };
            if ui.put(rect_right_top, Button::new("+")).clicked() {
                if let Selected::Player(ref mut i) = self.selected {
                    if *i + 1 == self.players.len() {
                        *i += 1;
                    }
                }
                self.players.push_back(PhandWithResult::default());
                self.base_players.push_back(PhandWithResult::default());
            };
            if ui.put(rect_left_top, Button::new("+")).clicked() {
                self.players.push_front(PhandWithResult::default());
                self.base_players.push_back(PhandWithResult::default());
                if let Selected::Player(ref mut i) = self.selected {
                    if self.players.len() != 1 {
                        *i += 1;
                    }
                }
            };
        }
    }

    fn draw_hand(ui: &mut Ui, cards: &[Card], pos: Pos2, step: Vec2, highlight: bool) {
        const OUTER_MARGINE: f32 = 5.0;
        let upper_limit = ui.ctx().available_rect().top() + 10.0;
        if highlight {
            let mut bottom_left_pos = pos2(
                pos.x - OUTER_MARGINE,
                pos.y + Self::CARD_SIZE.y + OUTER_MARGINE,
            );
            let mut upper_right_pos = pos2(
                pos.x + Self::CARD_SIZE.x + OUTER_MARGINE,
                pos.y - OUTER_MARGINE,
            );
            if cards.len() >= 2 {
                if step.x > 0.0 {
                    upper_right_pos.x += step.x * (cards.len() - 1) as f32;
                } else {
                    bottom_left_pos.x += step.x * (cards.len() - 1) as f32;
                }
                if step.y < 0.0 {
                    upper_right_pos.y += step.y * (cards.len() - 1) as f32;
                } else {
                    bottom_left_pos.y += step.y * (cards.len() - 1) as f32;
                }
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
    fn draw_card(ui: &mut Ui, card: &Card, pos: Pos2) -> Rect {
        const MARGIN: Vec2 = Vec2::new(0.0, 0.0);
        const TEXT_SIZE: f32 = 35.0;
        const TEXT_WIDTH: f32 = TEXT_SIZE * 0.8;
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
            0.0,
        );

        //let underline = card.get() == 6 || card.get() == 9;
        //const underline_weight:f32 = 2.0;
        let margin = Vec2::new((TEXT_WIDTH - galley.size().x) / 2.0, 0.0) + MARGIN;

        let upper_text = TextShape::new(rect.min + margin, galley.clone());
        ui.painter().add(upper_text);

        if CONFIG.read().general.rotate_num {
            let mut bottom_text = TextShape::new(rect.max - margin, galley);
            bottom_text.angle = std::f32::consts::PI;
            ui.painter().add(bottom_text);
        } else {
            let bottom_text = TextShape::new(rect.max - margin - galley.rect.size(), galley);
            ui.painter().add(bottom_text);
        }

        //center rect
        ui.painter().rect_filled(
            Rect::from_center_size(rect.center(), Vec2::new(20.0, 20.0)),
            Rounding::none(),
            Color32::from_rgb(200, 200, 200),
        );

        rect
    }
}

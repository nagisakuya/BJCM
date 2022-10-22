use super::*;

impl TableState{
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

    pub fn draw_table(&mut self, ui: &mut Ui) {
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
            Self::draw_hand(ui, self.discard.as_slice(), root_position, Vec2::new(25.0, 0.0),highlight);
        }

        //draw dealer
        {
            const TOP_MARGIN:f32 = 20.0;
            let dealer_root = pos2(available_rect.width() / 2.0 - Self::CARD_SIZE.x/2.0 - 25.0, available_rect.top() + TOP_MARGIN);
            let highlight = self.selected_current == Selected::Dealer;
            Self::draw_hand(ui, self.dealer.as_slice(), dealer_root,Vec2::new(25.0, 0.0), highlight);
        }

        //draw player
        {
            let players_len = self.players.len() as f32;
            for (i,phand) in self.players.iter_mut().enumerate(){
                const BOTTOM_MARGIN:f32 = 30.0;
                let space_width = available_rect.width() - Self::CARD_SIZE.x * players_len - SIDE_BUTTON_SIZE * 2.0;
                let space_per_hand = space_width / (players_len + 1.0);
                let pos = pos2(
                    available_rect.left() + SIDE_BUTTON_SIZE + space_per_hand * (i + 1) as f32 + Self::CARD_SIZE.x * i as f32,
                    available_rect.bottom() - BOTTOM_MARGIN - Self::CARD_SIZE.y,
                );
                let highlight = self.selected_current.is_player(i);
                Self::draw_hand(ui, phand.as_slice(), pos,Vec2::new(0.0, -30.0), highlight);

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
                    _ =>  {
                        if self.dealer.len() == 0 && phand.len() == 0{
                            let rect = Rect::from_center_size(result_area_rect.center(),Vec2::new(25.0,25.0));
                            if phand.is_player{
                                if ui.put(rect, Button::new(&format!("◆"))).clicked(){
                                    phand.is_player = false;
                                    self.base_players.get_mut(i).unwrap().is_player = false;
                                };
                            }
                            else{
                                if ui.put(rect, Button::new(&format!("◇"))).clicked(){
                                    phand.is_player = true;
                                    self.base_players.get_mut(i).unwrap().is_player = true;
                                };
                            }
                        }else{
                            if phand.is_player{
                                ui.put(result_area_rect, Label::new("◆"));
                            }else{
                                ui.put(result_area_rect, Label::new("◇"));
                            }
                        }
                    },
                };
            }
        }

        //draw buttons
        const SIDE_BUTTON_SIZE:f32 = 25.0;
        {
            const SPACE:f32 = 7.0;
            let size = Vec2::new(SIDE_BUTTON_SIZE,SIDE_BUTTON_SIZE);
            let rect_right_bottom = Rect::from_min_size(available_rect.right_bottom() - Vec2::new(SIDE_BUTTON_SIZE,SIDE_BUTTON_SIZE),size);
            let rect_left_bottom = Rect::from_min_size(available_rect.left_bottom() - Vec2::new(0.0,SIDE_BUTTON_SIZE),size);
            let rect_right_top = Rect::from_min_size(available_rect.right_bottom() - Vec2::new(SIDE_BUTTON_SIZE,SIDE_BUTTON_SIZE*2.0 + SPACE),size);
            let rect_left_top = Rect::from_min_size(available_rect.left_bottom() - Vec2::new(0.0,SIDE_BUTTON_SIZE*2.0 + SPACE),size);
            if ui.put(rect_right_bottom,Button::new("-")).clicked(){
                let _ = self.players.pop_back();
                let _ = self.base_players.pop_back();
                if let Selected::Player(ref mut i) = self.selected_base{
                    if *i == self.players.len() && *i != 0{
                        *i -= 1;
                    }
                }
            };
            if ui.put(rect_left_bottom,Button::new("-")).clicked(){
                let _ = self.players.pop_front();
                let _ = self.base_players.pop_front();
                if let Selected::Player(ref mut i) = self.selected_base{
                    if *i != 0{
                        *i -= 1;
                    }
                }
            };
            if ui.put(rect_right_top,Button::new("+")).clicked(){
                self.players.push_back(PhandWithResult::default());
                self.base_players.push_back(PhandWithResult::default());
            };
            if ui.put(rect_left_top,Button::new("+")).clicked(){
                self.players.push_front(PhandWithResult::default());
                self.base_players.push_back(PhandWithResult::default());
                if let Selected::Player(ref mut i) = self.selected_base{
                    if self.players.len() != 1{
                        *i += 1;
                    }
                }
            };
        }

    }

    fn draw_hand(ui: &mut Ui, cards: &[Card], pos: Pos2, step:Vec2 , highlight: bool) {
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
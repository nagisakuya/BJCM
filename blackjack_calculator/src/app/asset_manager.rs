use super::*;

use super::ASSET_FILE_PATH;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct AssetManager {
    total_asset: u32,
    min_bet: u32,
    max_bet: u32,
    bet_step: u32,
    round_up: f32,
    pub opened: bool,
}

impl Default for AssetManager {
    fn default() -> Self {
        AssetManager {
            total_asset: 0,
            min_bet: 0,
            max_bet: 1000,
            bet_step: 10,
            round_up: 0.5,
            opened: false,
        }
    }
}

impl AssetManager {
    fn _save(&self) {
        let mut file = std::fs::File::create(ASSET_FILE_PATH).unwrap();
        std::io::Write::write_all(&mut file, &bincode::serialize(self).unwrap()).unwrap();
    }
    pub fn load() -> Self {
        if let Ok(bin) = std::fs::read(ASSET_FILE_PATH) {
            bincode::deserialize(&bin).unwrap()
        } else {
            Default::default()
        }
    }
    pub fn show_bet_text(&self, ui: &mut Ui, ev: Option<f32>) {
        let mut text = String::from("bet:");
        if let Some(ev) = ev {
            text += &self.calc_betsize(ev).to_string()
        } else {
            text += &format!("{}", self.min_bet)
        };

        ui.label(RichText::new(text).size(20.0));
    }
    pub fn calc_betsize(&self, ev: f32) -> u32 {
        let bet = self.total_asset as f32 * ev;
        let mut a = (bet / self.bet_step as f32) as u32;
        let b = bet % self.bet_step as f32;
        if b >= (self.bet_step as f32 * self.round_up) {
            a += 1;
        }

        let mut bet = a * self.bet_step;
        if bet > self.max_bet {
            bet = self.max_bet
        }
        if bet < self.min_bet {
            bet = self.min_bet
        }

        bet
    }
    pub fn show(&mut self, ctx: &Context, config: &Config) -> bool {
        let mut disable_key_input = false;
        Window::new(config.get_text(TextKey::AssetWindowName))
            .auto_sized()
            .collapsible(false)
            .open(&mut self.opened)
            .show(ctx, |ui| {
                let mut add_num_textedit = |ui: &mut Ui, num: &mut u32| {
                    let mut text = num.to_string();
                    let resp = ui.add(TextEdit::singleline(&mut text).desired_width(140.0));
                    if resp.changed() {
                        if text.is_empty(){
                            *num = 0;
                        }else if let Ok(o) = text.parse() {
                            *num = o;
                        }
                    }
                    if resp.has_focus() {
                        disable_key_input = true;
                    }
                };

                const SPACE:f32 = 3.0;
                ui.label("◇asset");
                add_num_textedit(ui, &mut self.total_asset);
                ui.add_space(SPACE);

                ui.label("◇minimum bet");
                add_num_textedit(ui, &mut self.min_bet);
                ui.add_space(SPACE);

                ui.label("◇Maximum bet");
                add_num_textedit(ui, &mut self.max_bet);
                ui.add_space(SPACE);

                ui.label("◇bet step");
                add_num_textedit(ui, &mut self.bet_step);
                ui.add_space(SPACE);

                ui.label("◇round-up threshold");
                ui.add(Slider::new(&mut self.round_up, 0.05..=1.00).step_by(0.05));
            });
        disable_key_input
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let mut temp = AssetManager::default();
        temp.total_asset = 10000;
        let a = temp.calc_betsize(0.0554);
        println!("{a}");
    }
}

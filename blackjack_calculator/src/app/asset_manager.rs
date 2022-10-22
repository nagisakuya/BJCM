use std::str::FromStr;

use super::*;

use super::ASSET_FILE_PATH;

#[derive(Clone, serde::Serialize, serde::Deserialize ,PartialEq)]
pub struct AssetManager {
    total_asset: i32,
    diff_between_current_asset: i32,
    current_asset: i32,
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
            diff_between_current_asset: 0,
            current_asset: 0,
            min_bet: 0,
            max_bet: 1000,
            bet_step: 10,
            round_up: 0.5,
            opened: false,
        }
    }
}

impl AssetManager {
    fn save(&self) {
        let mut file = std::fs::File::create(ASSET_FILE_PATH).unwrap();
        std::io::Write::write_all(&mut file, &bincode::serialize(self).unwrap()).unwrap();
    }
    pub fn add_current(&mut self, i: i32) {
        self.current_asset += i;
        self.save()
    }
    pub fn load() -> Self {
        if let Ok(bin) = std::fs::read(ASSET_FILE_PATH) {
            bincode::deserialize(&bin).unwrap()
        } else {
            Default::default()
        }
    }
    pub fn draw_compornents(&mut self, ui: &mut Ui, ev: Option<f32>, input_flag: &mut bool) -> u32 {
        let betsize = if let Some(ev) = ev {
            self.calc_betsize(ev)
        } else {
            self.min_bet
        };

        let text = format!("bet:{betsize}");

        ui.label(RichText::new(text).size(20.0));

        ui.horizontal(|ui|{
            ui.label("asset:");
            Self::add_numonly_textedit(ui, &mut self.current_asset, input_flag, 100.0);
            let temp = self.current_asset + self.diff_between_current_asset;
            if self.total_asset != temp{
                self.total_asset = temp;
                self.save();
            }
        });

        betsize
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
    pub fn show_window(&mut self, ctx: &Context, config: &Config, input_flag: &mut bool) {
        let temp = self.clone();
        Window::new(config.get_text(TextKey::AssetWindowName))
            .auto_sized()
            .collapsible(false)
            .open(&mut self.opened)
            .show(ctx, |ui| {
                let mut add_textedit = |ui: &mut Ui, num| {
                    Self::add_numonly_textedit(ui, num, input_flag, 140.0);
                };
                const SPACE: f32 = 3.0;
                ui.label("◇asset");
                add_textedit(ui, &mut self.total_asset);
                ui.add_space(SPACE);

                ui.label("◇asset in casino");
                add_textedit(ui, &mut self.current_asset);
                ui.add_space(SPACE);
                self.diff_between_current_asset = self.total_asset - self.current_asset;

                let mut add_textedit = |ui: &mut Ui, num| {
                    Self::add_numonly_textedit(ui, num, input_flag, 140.0);
                };
                ui.label("◇minimum bet");
                add_textedit(ui, &mut self.min_bet);
                ui.add_space(SPACE);

                ui.label("◇Maximum bet");
                add_textedit(ui, &mut self.max_bet);
                ui.add_space(SPACE);

                ui.label("◇bet step");
                add_textedit(ui, &mut self.bet_step);
                ui.add_space(SPACE);

                ui.label("◇round-up threshold");
                ui.add(Slider::new(&mut self.round_up, 0.05..=1.00).step_by(0.05));
            });
        if !(*self).eq(&temp){
            self.save();
        }
    }
    pub fn add_numonly_textedit<T: ToString + FromStr + Default>(
        ui: &mut Ui,
        num: &mut T,
        key_input_flag: &mut bool,
        width: f32,
    ) {
        let mut text = num.to_string();
        let resp = ui.add(TextEdit::singleline(&mut text).desired_width(width));
        if resp.changed() {
            if text.is_empty() {
                *num = Default::default();
            } else if let Ok(o) = text.parse() {
                *num = o;
            }
        }
        if resp.has_focus() {
            *key_input_flag = true;
        }
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

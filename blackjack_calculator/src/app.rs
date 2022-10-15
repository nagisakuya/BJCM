pub use blackjack_lib::*;
pub use eframe::egui::*;

use std::collections::VecDeque;
use std::process::Command;
use std::time::{Duration, Instant};

mod table_state;
use table_state::*;

mod total_ev_handler;
use total_ev_handler::*;

mod config;
pub use config::TextKey;
use config::*;

mod activator;
use activator::*;

mod buy_window;
use buy_window::*;

const IMAGE_FOLDER_PATH: &str = "./data/images";
const SUBPROCESS_PATH: &str = "./data/calc_ev_subprocess.exe";
const SETTING_FILE_PATH: &str = "./data/setting.bin";
const ACTIVATION_CODE_PATH: &str = "./data/activation_code.txt";

pub struct AppMain {
    config: Config,
    activator: Activator,
    table_state: TableState,
    table_history: VecDeque<TableState>,
    total_ev_handler: TotalEvHandler,
    rule_setting_window: Option<RuleSettingWindow>,
    key_setting_window: Option<KeySettingWindow>,
    general_setting_window: Option<GeneralSettingWindow>,
    buy_window: BuyWindow,
}
impl AppMain {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut activator = Activator::new();
        let config = if activator.check_activated() {
            Config::load()
        } else {
            Config::default()
        };

        let mut _self = Self {
            table_state: TableState::new(&config),
            table_history: VecDeque::new(),
            total_ev_handler: Default::default(),
            rule_setting_window: None,
            key_setting_window: None,
            general_setting_window: None,
            buy_window: BuyWindow::new(),
            activator,
            config,
        };

        _self.total_ev_handler.setup(cc);
        _self.table_state.setup(cc);

        cc.egui_ctx.set_visuals(egui::Visuals::dark());

        //font
        {
            let mut fonts = FontDefinitions::default();
            let mut font = FontData::from_static(include_bytes!("../fonts/NotoSansJP-Regular.otf"));
            font.tweak.scale = 1.5;
            fonts.font_data.insert("noto_sans".to_owned(), font);
            fonts
                .families
                .get_mut(&FontFamily::Proportional)
                .unwrap()
                .insert(0, "noto_sans".to_owned());
            cc.egui_ctx.set_fonts(fonts);
        }

        _self
    }
    pub fn unactivate(mut self) -> Self {
        self.activator.unactivate();
        self
    }
}
impl eframe::App for AppMain {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        TopBottomPanel::top("menu")
            .resizable(false)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    if ui
                        .button(self.config.get_text(TextKey::GeneralSettingButton))
                        .clicked()
                    {
                        if self.general_setting_window.is_none() {
                            self.general_setting_window = Some(GeneralSettingWindow::new(
                                &self.config.general,
                                self.activator.check_activated(),
                            ));
                        }
                    }
                    if ui
                        .button(self.config.get_text(TextKey::RuleSettingWindowButton))
                        .clicked()
                    {
                        if self.rule_setting_window.is_none() {
                            self.rule_setting_window = Some(RuleSettingWindow::new(
                                &self.config.rule,
                                self.activator.check_activated(),
                            ));
                        }
                    }
                    if ui
                        .button(self.config.get_text(TextKey::KeySettingWindowButton))
                        .clicked()
                    {
                        if self.key_setting_window.is_none() {
                            self.key_setting_window = Some(KeySettingWindow::new(
                                &self.config.kyes,
                                self.activator.check_activated(),
                            ));
                        }
                    }
                    if ui
                        .button(self.config.get_text(TextKey::HowToUseButton))
                        .clicked()
                    {
                        Command::new("cmd").args(["/c","start",self.config.get_text(TextKey::HowToUseURL)]).status().unwrap();
                    }
                    let text = RichText::new(self.config.get_text(TextKey::BuyWindowButton))
                        .color(Color32::from_gray(20));
                    let temp = Button::new(text).fill(Color32::from_rgb(255, 200, 30));
                    if !self.activator.activated && ui.add(temp).clicked() {
                        self.buy_window.opened = !self.buy_window.opened;
                    }
                })
            });
        CentralPanel::default().show(ctx, |ui| {
            SidePanel::right("side_panel")
                .resizable(false)
                .show_inside(ui, |ui| {
                    self.table_state.draw_deck(ui);
                    ui.with_layout(Layout::bottom_up(Align::Center), |ui| {
                        ui.add_space(10.0);
                        self.total_ev_handler.draw_contents(ui, &self.table_state);
                    });
                });
            self.table_state.draw_table(ui);
        });
        self.total_ev_handler
            .update(&self.config, &self.table_state.deck);
        self.table_state
            .update(ctx, &self.config, &mut self.table_history);
        if let Some(ref mut o) = self.rule_setting_window {
            let result = o.show(ctx, &self.config);
            if result.0 {
                self.rule_setting_window = None;
                if self.activator.check_activated() {
                    if let Some(o) = result.1 {
                        self.config.rule = o;
                        self.config.save();
                        self.total_ev_handler.reset();
                        self.table_state.reset(&self.config);
                        self.table_history = Default::default();
                    }
                }
            }
        }
        if let Some(ref mut o) = self.key_setting_window {
            let result = o.show(ctx, &self.config);
            if result.0 {
                self.key_setting_window = None;
                if let Some(o) = result.1 {
                    self.config.kyes = o;
                    if self.activator.check_activated() {
                        self.config.save();
                    }
                }
            }
        }

        if let Some(ref mut o) = self.general_setting_window {
            let result = o.show(ctx, &self.config);
            if result.0 {
                self.general_setting_window = None;
                if let Some(o) = result.1 {
                    self.config.general = o;
                    self.table_state.next(&self.config);
                }
            }
        }
        self.buy_window.show(ctx, &self.config, &mut self.activator);
    }
}

fn load_image_from_path(path: &str) -> Result<ColorImage, image::ImageError> {
    let path = std::path::Path::new(path);
    let image = image::io::Reader::open(path)?.decode()?;
    let size = [image.width() as _, image.height() as _];
    let image_buffer = image.to_rgba8();
    let pixels = image_buffer.as_flat_samples();
    Ok(ColorImage::from_rgba_unmultiplied(size, pixels.as_slice()))
}

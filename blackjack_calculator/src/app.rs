pub use blackjack_lib::*;
pub use eframe::egui::*;

use std::collections::{BTreeMap, VecDeque};
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

mod asset_manager;
use asset_manager::*;

mod buy_window;
use buy_window::*;

//const IMAGE_FOLDER_PATH: &str = "./data/images";
const SUBPROCESS_PATH: &str = "./data/calc_ev_subprocess.exe";
const SUBPROCESS_WSL_PATH: &str = "./data/calc_ev_subprocess";
const ACTIVATION_CODE_PATH: &str = "./data/activation_code.txt";
const SETTING_FILE_PATH: &str = "./data/setting.bin";
const ASSET_FILE_PATH: &str = "./data/asset.bin";

pub struct AppMain {
    config: Config,
    activator: Activator,
    table_state: TableState,
    table_history: VecDeque<TableState>,
    total_ev_handler: TotalEvHandler,
    rule_setting_window: RuleSettingWindow,
    key_setting_window: KeySettingWindow,
    general_setting_window: GeneralSettingWindow,
    asset_manager: AssetManager,
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

        rule::override_rule(config.rule.clone());

        let mut _self = Self {
            table_state: TableState::new(&config),
            table_history: VecDeque::new(),
            total_ev_handler: Default::default(),
            rule_setting_window: RuleSettingWindow::new(&config.rule, activator.check_activated()),
            key_setting_window: KeySettingWindow::new(&config.kyes, activator.check_activated()),
            general_setting_window: GeneralSettingWindow::new(config.general.clone()),
            asset_manager: AssetManager::load(),
            buy_window: BuyWindow::new(),
            activator,
            config,
        };

        _self.total_ev_handler.setup(cc);

        cc.egui_ctx.set_visuals(egui::Visuals::dark());

        //font
        {
            let mut fonts = FontDefinitions::default();
            let mut sans = FontData::from_static(include_bytes!("../fonts/NotoSansJP-Regular.otf"));
            sans.tweak.scale = 1.5;
            fonts.font_data.insert("noto_sans".to_string(), sans);

            let times = FontData::from_static(include_bytes!("../fonts/times new roman.ttf"));
            fonts.font_data.insert("times_new_roman".to_string(), times);

            let priority = fonts.families.get_mut(&FontFamily::Proportional).unwrap();
            priority.insert(0, "noto_sans".to_string());

            let mut btree: BTreeMap<FontFamily, Vec<_>> = BTreeMap::new();
            btree.insert(
                FontFamily::Name("times_new_roman".into()),
                vec!["times_new_roman".to_string()],
            );
            btree.insert(
                FontFamily::Name("noto_sans".into()),
                vec!["noto_sans".to_string()],
            );

            fonts.families.append(&mut btree);

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
        let mut disable_key_input_flag = false;
        let mut betsize = 0;
        TopBottomPanel::top("menu")
            .resizable(false)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    if ui
                        .button(self.config.get_text(TextKey::GeneralSettingButton))
                        .clicked()
                    {
                        self.general_setting_window.switch(&self.config);
                    }
                    if ui
                        .button(self.config.get_text(TextKey::RuleSettingWindowButton))
                        .clicked()
                    {
                        self.rule_setting_window.switch(&self.config);
                    }
                    if ui
                        .button(self.config.get_text(TextKey::KeySettingWindowButton))
                        .clicked()
                    {
                        self.key_setting_window.switch(&self.config);
                    }
                    if ui
                        .button(self.config.get_text(TextKey::HowToUseButton))
                        .clicked()
                    {
                        Command::new("cmd")
                            .args(["/c", "start", self.config.get_text(TextKey::HowToUseURL)])
                            .status()
                            .unwrap();
                    }
                    if ui
                        .button(self.config.get_text(TextKey::AssetButton))
                        .clicked()
                    {
                        self.asset_manager.opened = !self.asset_manager.opened;
                    }
                    let text = RichText::new(self.config.get_text(TextKey::BuyWindowButton))
                        .color(Color32::from_gray(20));
                    let temp = Button::new(text).fill(Color32::from_rgb(255, 200, 30));
                    if !self.activator.activated && ui.add(temp).clicked() {
                        self.buy_window.opened = !self.buy_window.opened;
                    }
                })
            });
        self.total_ev_handler
            .update(&self.config, &self.table_state,ctx);
        SidePanel::right("side_panel")
            .resizable(false)
            .max_width(140.0)
            .show(ctx, |ui| {
                TopBottomPanel::bottom("total_ev_handler")
                    .resizable(false)
                    .frame(Frame::default().outer_margin(style::Margin::same(0.0)))
                    .show_inside(ui, |ui| {
                        ui.vertical_centered(|ui| {
                            self.total_ev_handler.draw_text(ui, &self.table_state);
                            betsize = self.asset_manager.draw_text(
                                ui,
                                self.total_ev_handler.get_optimal_betsize(),
                                &self.config,
                            );
                            self.total_ev_handler.draw_controller(ui,&self.config,&self.table_state.deck);
                            self.asset_manager.show_balance(
                                ui,
                                &mut disable_key_input_flag,
                                &self.config,
                            );
                            ui.add_space(10.0);
                        });
                    });
                self.table_state.show_deck(ui, &self.config);
            });
        CentralPanel::default().show(ctx, |ui| {
            self.table_state.draw_table(ui, &self.config);
        });
        let result = self.rule_setting_window.show(ctx, &self.config);
        if result.0 {
            self.rule_setting_window.close();
            if self.activator.check_activated() {
                if let Some(o) = result.1 {
                    rule::override_rule(o.clone());
                    self.config.rule = o;
                    self.config.save();
                    self.total_ev_handler.reset();
                    //self.table_state.reset(&self.config);
                    //self.table_history = Default::default();
                }
            }
        }
        let result = self.key_setting_window.show(ctx, &self.config);
        if result.0 {
            self.key_setting_window.close();
            if let Some(o) = result.1 {
                self.config.kyes = o;
                if self.activator.check_activated() {
                    self.config.save();
                }
            }
        }

        let result = self.general_setting_window.show(ctx, &self.config);
        if result.0 {
            self.general_setting_window.close();
            if let Some(o) = result.1 {
                self.config.general = o;
                if self.activator.check_activated() {
                    self.config.save();
                }
            }
        }
        self.buy_window.show(ctx, &self.config, &mut self.activator);

        self.asset_manager
            .show_window(ctx, &self.config, &mut disable_key_input_flag);

        if !disable_key_input_flag {
            self.table_state.update(
                ctx,
                &self.config,
                &mut self.table_history,
                betsize,
                &mut self.asset_manager,
                &mut self.total_ev_handler,
            );
        }
    }
}

fn _load_image_from_path(path: &str) -> Result<ColorImage, image::ImageError> {
    let path = std::path::Path::new(path);
    let image = image::io::Reader::open(path)?.decode()?;
    let size = [image.width() as _, image.height() as _];
    let image_buffer = image.to_rgba8();
    let pixels = image_buffer.as_flat_samples();
    Ok(ColorImage::from_rgba_unmultiplied(size, pixels.as_slice()))
}

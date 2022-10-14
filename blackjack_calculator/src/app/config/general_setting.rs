use super::*;
use std::collections::HashMap;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

mod texts;
pub use texts::*;

#[derive(PartialEq, Eq, Hash, Clone, serde::Serialize, serde::Deserialize)]
pub struct GeneralSetting {
    pub language: Language,
}
impl Default for GeneralSetting {
    fn default() -> Self {
        GeneralSetting {
            language: Language::Japanese,
        }
    }
}

pub struct GeneralSettingWindow {
    pub general: GeneralSetting,
    _is_activated: bool,
}
impl GeneralSettingWindow {
    pub fn new(general: &GeneralSetting, activated: bool) -> GeneralSettingWindow {
        GeneralSettingWindow {
            general: general.clone(),
            _is_activated: activated,
        }
    }
    pub fn show(&mut self, ctx: &Context, config: &Config) -> (bool, Option<GeneralSetting>) {
        let mut result = (false, None);
        Window::new(config.get_text(TextKey::GeneralSettingWindowName))
            .auto_sized()
            .collapsible(false)
            .show(ctx, |ui| {
                egui::ComboBox::from_label(config.get_text(TextKey::GeneralSettingLanguage))
                    .selected_text(format!("{:?}", self.general.language))
                    .show_ui(ui, |ui| {
                        for item in Language::iter() {
                            ui.selectable_value(
                                &mut self.general.language,
                                item,
                                format!("{:?}", item),
                            );
                        }
                    });

                ui.horizontal(|ui| {
                    if ui.button("cancel").clicked() {
                        result.0 = true;
                    }
                    if ui.button("apply").clicked() {
                        result.0 = true;
                        result.1 = Some(self.general.clone());
                    }
                });
            });
        result
    }
}

#[derive(
    EnumIter, PartialEq, Eq, Hash, Clone, Copy, Debug, serde::Serialize, serde::Deserialize,
)]
#[repr(usize)]
pub enum Language {
    English,
    Japanese,
}

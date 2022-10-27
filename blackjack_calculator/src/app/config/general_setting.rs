use super::*;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(PartialEq, Eq, Hash, Clone, serde::Serialize, serde::Deserialize)]
pub struct GeneralSetting {
    pub language: Language,
    pub infinite: bool,
    pub rotate_num: bool,
}
impl Default for GeneralSetting {
    fn default() -> Self {
        GeneralSetting {
            language: Language::Japanese,
            infinite: false,
            rotate_num: true,
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
                ui.label("◇".to_owned() + config.get_text(TextKey::GeneralSettingLanguage));
                egui::ComboBox::from_label("")
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

                ui.add_space(10.0);
                ui.add(Checkbox::new(&mut self.general.infinite, config.get_text(TextKey::GeneralSettingDiscard)));
                ui.add(Checkbox::new(&mut self.general.rotate_num, config.get_text(TextKey::GeneralSettingRotateNum)));
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

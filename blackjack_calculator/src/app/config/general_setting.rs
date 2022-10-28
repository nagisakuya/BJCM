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
    general: GeneralSetting,
    opened: bool,
}
impl GeneralSettingWindow {
    pub fn new(general: GeneralSetting) -> GeneralSettingWindow {
        GeneralSettingWindow {
            opened: false,
            general: general.clone(),
        }
    }
    pub fn switch(&mut self, config: &Config){
        if self.opened{
            self.try_close(config);
        }else{
            self.opened = true;
        }
    }
    pub fn try_close(&mut self, config: &Config){
        if config.general == self.general{
            self.close();
        }
    }
    pub fn close(&mut self){
        self.opened = false;
    }
    pub fn show(&mut self, ctx: &Context, config: &Config) -> (bool, Option<GeneralSetting>) {
        let mut result = (false, None);
        if !self.opened {return result}
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

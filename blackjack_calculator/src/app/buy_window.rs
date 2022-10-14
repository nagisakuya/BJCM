use super::*;

pub struct BuyWindow {
    pub opened: bool,
    pcid: String,
    activation_code: String,
    activation_text: Option<String>,
}
impl BuyWindow {
    pub fn new() -> Self {
        BuyWindow {
            opened: false,
            pcid: activator::Activator::get_pcid(),
            activation_code: String::new(),
            activation_text: None,
        }
    }
    pub fn show(&mut self, ctx: &Context,config:&Config,activator:&mut Activator) {
        Window::new(config.get_text(TextKey::BuyWindowName)).open(&mut self.opened)
        .show(ctx, |ui|{
            ui.label(RichText::new(config.get_text(TextKey::BuyWindowH1)).heading().color(Color32::from_gray(230)));
            ui.label(RichText::new(config.get_text(TextKey::BuyWindowT1)));
            ui.add_space(20.0);
            ui.label(RichText::new(config.get_text(TextKey::BuyWindowH2)).heading().color(Color32::from_gray(230)));
            ui.label(RichText::new(config.get_text(TextKey::BuyWindowT2)));
            ui.add(Hyperlink::new(config.get_text(TextKey::PurchaseLink)));
            ui.add_space(20.0);
            ui.label(config.get_text(TextKey::BuyWindowUserID));
            ui.add(TextEdit::singleline(&mut self.pcid.clone()));
            ui.add_space(20.0);
            ui.label(config.get_text(TextKey::BuyWindowActivationFormDescription));
            ui.add(TextEdit::singleline(&mut self.activation_code));
            if ui.button("activate").clicked(){
                match activator.activate(&self.activation_code) {
                    Ok(_) => {self.activation_text = Some(String::from("Activation success"))},
                    Err(e) => {self.activation_text = Some(e);},
                }
            }
            if let Some(ref o) = self.activation_text{
                ui.label(o);
            }
        });
    }
}

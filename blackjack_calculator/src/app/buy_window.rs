use super::*;

pub struct BuyWindow {
    pub opened: bool,
    pcid: String,
}
impl BuyWindow {
    pub fn new() -> Self {
        BuyWindow {
            opened: false,
            pcid: activator::Activator::get_pcid(),
        }
    }
    pub fn show(&mut self, ctx: &Context,config:&Config) {
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
            let mut temp = self.pcid.clone();
            ui.add(TextEdit::singleline(&mut temp));
        });
    }
}

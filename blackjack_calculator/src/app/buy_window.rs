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
    pub fn show(&mut self, ctx: &Context) {
        Window::new("製品版購入").open(&mut self.opened)
        .show(ctx, |ui|{
            ui.label(RichText::new("試用版と製品版の違い").heading());
            ui.label(RichText::new("試用版では、テーブルルールの変更が出来ません。\n製品版では、テーブルルールを変更し、テーブルルールに対応した最適手と期待値を計算することができます。"));
            ui.label(RichText::new("\nご購入方法").heading());
            ui.label(RichText::new("こちらのwebサイトからご購入ください。"));
            ui.add(Hyperlink::new("https://www.youtube.com/"));
            ui.label("\nユーザーID:");
            let mut temp = self.pcid.clone();
            ui.add(TextEdit::singleline(&mut temp));
        });
    }
}

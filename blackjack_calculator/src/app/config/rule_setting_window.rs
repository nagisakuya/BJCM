use super::*;

pub struct RuleSettingWindow{
    rule:Rule,
    charlie_enable:bool,
    charlie:usize,
    bj_odd:f32,
    is_activated:bool,
}
impl RuleSettingWindow{
    pub fn new(rule:&Rule,activated:bool) -> Self{
        let charlie = if let Some(t) = rule.CHARLIE{
            t
        }else {5};
        RuleSettingWindow{
            rule:rule.clone(),
            charlie_enable:rule.CHARLIE.is_some(),
            charlie,
            bj_odd:rule.BJ_PAYBACK + 1.0,
            is_activated:activated,
        }
    }
    fn to_rule(&self) -> Rule{
        let mut rule = self.rule.clone();
        rule.CHARLIE = if self.charlie_enable{
            Some(self.charlie)
        }else{None};
        rule.BJ_PAYBACK = self.bj_odd - 1.0;
        rule
    }
    pub fn show(&mut self,ctx:&Context) -> (bool,Option<Rule>){
        let mut result = (false,None);
        Window::new("rule setting")
        .auto_sized()
        .collapsible(false)
        .show(ctx, |ui|{
            ui.label("◇number of deck");
            ui.add(Slider::new(&mut self.rule.NUMBER_OF_DECK,1..=16));
            ui.label("◇Blackjack odds");
            ui.add(Slider::new(&mut self.bj_odd,2.00..=3.00).step_by(0.05));
            ui.add(Checkbox::new(&mut self.rule.LATE_SURRENDER, "Surrender"));
            ui.add(Checkbox::new(&mut self.rule.DEALER_SOFT_17_STAND, "dealer soft 17 stands"));
            ui.add(Checkbox::new(&mut self.charlie_enable, "Charlie"));
            if self.charlie_enable{
                ui.add(Slider::new(&mut self.charlie,4..=9));
            }
            ui.add(Checkbox::new(&mut self.rule.DOUBLE_AFTER_SPLIT, "double after split"));
            ui.add(Checkbox::new(&mut self.rule.BJ_AFTER_SPLIT, "blackjack after split"));
            ui.add(Checkbox::new(&mut self.rule.RE_SPLIT, "resplit"));
            ui.add(Checkbox::new(&mut self.rule.ACTION_AFTER_SPLITTING_ACE, "splitting ace stands"));
            ui.add(Checkbox::new(&mut self.rule.DEALER_PEEKS_ACE, "dealer peeks when Ace"));
            ui.add(Checkbox::new(&mut self.rule.DEALER_PEEKS_TEN, "dealer peeks when Ten"));
            if !self.is_activated{
                ui.label(RichText::new("試用版ではルール設定の\n変更ができません！").color(Color32::from_rgb(200, 0, 0)));
            }
            ui.horizontal(|ui|{
                if ui.button("cancel").clicked(){
                    result.0 = true;
                }
                if self.is_activated && ui.button("apply").clicked(){
                    result.0 = true;
                    result.1 = Some(self.to_rule());
                }
            });
         });
         result
    }
}
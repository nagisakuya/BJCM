use super::*;


pub struct RuleSettingWindow{
    rule:Rule,
    charlie_enable:bool,
    charlie:usize,
    bj_odd:f64,
    is_activated:bool,
    opened:bool,
}
impl RuleSettingWindow{
    pub fn new(rule:&Rule,activated:bool) -> Self{
        let charlie = if let Some(t) = rule.CHARLIE{
            t
        }else {6};
        RuleSettingWindow{
            rule:rule.clone(),
            charlie_enable:rule.CHARLIE.is_some(),
            charlie,
            bj_odd:rule.BJ_PAYBACK + 1.0,
            is_activated:activated,
            opened:false,
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
        if config.rule == self.to_rule(){
            self.close();
        }
    }
    pub fn close(&mut self){
        self.opened = false;
    }
    fn to_rule(&self) -> Rule{
        let mut rule = self.rule.clone();
        rule.CHARLIE = if self.charlie_enable{
            Some(self.charlie)
        }else{None};
        rule.BJ_PAYBACK = self.bj_odd - 1.0;
        rule
    }
    pub fn show(&mut self,ctx:&Context,config:&Config) -> (bool,Option<Rule>){
        let mut result = (false,None);
        if !self.opened {return result}
        Window::new(config.get_text(TextKey::RuleSettingWindowName))
        .auto_sized()
        .collapsible(false)
        .show(ctx, |ui|{
            ui.label("◇number of deck");
            ui.add(Slider::new(&mut self.rule.NUMBER_OF_DECK,1..=16));
            ui.label("◇Blackjack odds");
            ui.add(Slider::new(&mut self.bj_odd,2.00..=3.00).step_by(0.05));
            ui.add(Checkbox::new(&mut self.rule.LATE_SURRENDER, "Surrender"));
            let mut temp = !self.rule.DEALER_SOFT_17_STAND;
            ui.add(Checkbox::new(&mut temp, "dealer soft 17 hits"));
            self.rule.DEALER_SOFT_17_STAND = !temp;
            ui.add(Checkbox::new(&mut self.charlie_enable, "Charlie"));
            if self.charlie_enable{
                ui.add(Slider::new(&mut self.charlie,4..=9));
            }
            ui.add(Checkbox::new(&mut self.rule.DOUBLE_AFTER_SPLIT, "double after split"));
            ui.add(Checkbox::new(&mut self.rule.BJ_AFTER_SPLIT, "blackjack after split"));
            ui.add(Checkbox::new(&mut self.rule.RE_SPLIT, "resplit"));
            let mut temp = !self.rule.ACTION_AFTER_SPLITTING_ACE;
            ui.add(Checkbox::new(&mut temp, "splitting ace stands"));
            self.rule.DEALER_SOFT_17_STAND = !temp;
            ui.add(Checkbox::new(&mut self.rule.INSUALANCE, "Insualance"));
            if self.rule.INSUALANCE{
                self.rule.DEALER_PEEKS_ACE = true;
            }else{
                ui.add(Checkbox::new(&mut self.rule.DEALER_PEEKS_ACE, "dealer peeks with Ace"));
            }
            ui.add(Checkbox::new(&mut self.rule.DEALER_PEEKS_TEN, "dealer peeks with Ten"));
            if !self.is_activated{
                ui.label(RichText::new(config.get_text(TextKey::TrialVersionRuleSettingMessage)).color(Color32::from_rgb(200, 0, 0)));
            }
            ui.horizontal(|ui|{
                if ui.button(config.get_text(TextKey::Cancel)).clicked(){
                    result.0 = true;
                }
                if self.is_activated && ui.button(config.get_text(TextKey::Apply)).clicked(){
                    result.0 = true;
                    result.1 = Some(self.to_rule());
                }
            });
         });
         result
    }
}
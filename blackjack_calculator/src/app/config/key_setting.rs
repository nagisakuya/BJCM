use once_cell::sync::Lazy;
use super::*;


#[derive(Clone,serde::Serialize,serde::Deserialize)]
pub struct Keys{
    pub card: [Key; 10],
    pub undo: Key,
    pub next: Key,
    pub reset: Key,
    pub split: Key,
    pub up:Key,
    pub down:Key,
    pub right:Key,
    pub left:Key,
}

impl Default for Keys{
    fn default() -> Self {
        Keys {
            card:[
                Key::Num1,
                Key::Num2,
                Key::Num3,
                Key::Num4,
                Key::Num5,
                Key::Num6,
                Key::Num7,
                Key::Num8,
                Key::Num9,
                Key::Num0,
            ],
            undo:Key::Z,
            next:Key::Enter,
            reset:Key::R,
            split:Key::S,
            up:Key::ArrowUp,
            down:Key::ArrowDown,
            right:Key::ArrowRight,
            left:Key::ArrowLeft,
        }
    }
}

macro_rules! generate_combobox {
    ($label:expr,$ui:expr,$i:expr) => {{
        let text = format!("{:?}",$i);
        let closure = |ui:&mut Ui|{
            for &item in KEY_LIST.iter(){
                let text = format!{"{:?}",item};
                ui.selectable_value(&mut $i, item, text);
            };
        };
        ComboBox::from_label($label).selected_text(text).width(120.0).show_ui($ui, closure);
    }};
}

pub struct KeySettingWindow{
    keys:Keys,
    is_activated:bool,
}
impl KeySettingWindow{
    pub fn new(keys:&Keys,is_activated:bool) -> Self{
        KeySettingWindow{
            keys:keys.clone(),
            is_activated,
        }
    }
    pub fn show(&mut self,ctx:&Context,config:&Config) -> (bool,Option<Keys>){
        let mut result = (false,None);
        Window::new(config.get_text(TextKey::KeySettingWindowName))
        .auto_sized()
        .collapsible(false)
        .show(ctx, |ui|{
            ScrollArea::vertical().auto_shrink([true,true]).max_height(300.0)
            .show(ui, |ui|{
                generate_combobox!("UP",ui,self.keys.up);
                generate_combobox!("DOWN",ui,self.keys.down);
                generate_combobox!("RIGHT",ui,self.keys.right);
                generate_combobox!("LEFT",ui,self.keys.left);
                generate_combobox!("Split",ui,self.keys.split);
                generate_combobox!("Next",ui,self.keys.next);
                generate_combobox!("Undo",ui,self.keys.undo);
                generate_combobox!("Reset",ui,self.keys.reset);
                generate_combobox!("Ace",ui,self.keys.card[0]);
                generate_combobox!("2",ui,self.keys.card[1]);
                generate_combobox!("3",ui,self.keys.card[2]);
                generate_combobox!("4",ui,self.keys.card[3]);
                generate_combobox!("5",ui,self.keys.card[4]);
                generate_combobox!("6",ui,self.keys.card[5]);
                generate_combobox!("7",ui,self.keys.card[6]);
                generate_combobox!("8",ui,self.keys.card[7]);
                generate_combobox!("9",ui,self.keys.card[8]);
                generate_combobox!("Ten",ui,self.keys.card[9]);
            });
            if !self.is_activated{
                ui.label(RichText::new(config.get_text(TextKey::TrialVersionKeySettingMessage)).color(Color32::from_rgb(200, 0, 0)));
            }
            ui.horizontal(|ui|{
                if ui.button("cancel").clicked(){
                    result.0 = true;
                }
                if ui.button("apply").clicked(){
                    result.0 = true;
                    result.1 = Some(self.keys.clone());
                }
            });
         });
         result
    }
}

static KEY_LIST:Lazy<Vec<Key>> = Lazy::new(||vec![
    Key::Num0,
    Key::Num1,
    Key::Num2,
    Key::Num3,
    Key::Num4,
    Key::Num5,
    Key::Num6,
    Key::Num7,
    Key::Num8,
    Key::Num9,
    
    Key::A,
    Key::B,
    Key::C,
    Key::D,
    Key::E,
    Key::F,
    Key::G,
    Key::H,
    Key::I,
    Key::J,
    Key::K,
    Key::L,
    Key::M,
    Key::N,
    Key::O,
    Key::P,
    Key::Q,
    Key::R,
    Key::S,
    Key::T,
    Key::U,
    Key::V,
    Key::W,
    Key::X,
    Key::Y,
    Key::Z,
    
    Key::ArrowDown,
    Key::ArrowLeft,
    Key::ArrowRight,
    Key::ArrowUp,
    
    Key::Escape,
    Key::Tab,
    Key::Backspace,
    Key::Enter,
    Key::Space,
    
    Key::Insert,
    Key::Delete,
    Key::Home,
    Key::End,
    Key::PageUp,
    Key::PageDown,
    ]);
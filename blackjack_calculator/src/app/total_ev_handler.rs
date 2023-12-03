use super::*;
use std::io::Read;
use std::os::windows::process::CommandExt;
use std::sync::mpsc::{self, Receiver};
use std::{
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
};

const CREATE_NO_WINDOW: u32 = 0x08000000;

#[derive(PartialEq, Eq, Clone)]
enum CalcMode {
    Idle,
    Endless,
    DealerStands,
}

pub struct TotalEvHandler {
    process: Option<(Arc<Mutex<std::process::Child>>, Deck, Instant)>,
    calculate: CalcMode,
    total_ev: Option<(f32, Instant, Deck)>,
    optimal_betsize: Option<f32>,
    total_ev_resiever: Option<Receiver<(f32, f32)>>,
    progless: f32,
    progless_resiever: Option<Receiver<f32>>,
    stdout_resiever: Option<JoinHandle<()>>,
    //textures: [Option<TextureHandle>; 2],
    wsl_installed: bool,
}
impl Default for TotalEvHandler {
    fn default() -> Self {
        let wsl = {
            let output = std::process::Command::new("wsl")
                .arg("echo")
                .arg("1")
                .creation_flags(CREATE_NO_WINDOW)
                .output()
                .expect("failed to start process");
            output.status.success()
        };
        TotalEvHandler {
            progless: 0.0,
            process: None,
            total_ev: None,
            optimal_betsize: None,
            calculate: CalcMode::Idle,
            stdout_resiever: None,
            progless_resiever: None,
            total_ev_resiever: None,
            //textures: Default::default(),
            wsl_installed: wsl,
        }
    }
}
impl TotalEvHandler {
    //unused
    pub fn _get_ev(&self) -> Option<f32> {
        self.total_ev.as_ref().map(|x| x.0)
    }
    pub fn get_optimal_betsize(&self) -> Option<f32> {
        self.optimal_betsize
    }
    pub fn setup(&mut self, _cc: &eframe::CreationContext<'_>) {
        /*let image = load_image_from_path(&format!("{}/play.png", IMAGE_FOLDER_PATH)).unwrap();
        self.textures[0] = Some(cc.egui_ctx.load_texture("play", image));
        let image = load_image_from_path(&format!("{}/stop.png", IMAGE_FOLDER_PATH)).unwrap();
        self.textures[1] = Some(cc.egui_ctx.load_texture("stop", image));*/
    }
    pub fn update(&mut self, table: &TableState, ctx: &Context) {
        let deck = &table.deck;
        let dealer = &table.dealer;

        let start_condition = {
            self.calculate == CalcMode::Endless
                || self.calculate == CalcMode::DealerStands
                    && (dealer.stand() || ctx.input().key_pressed(CONFIG.read().kyes.next))
        };
        let not_calculated = self.total_ev.is_none() || !self.total_ev.as_ref().unwrap().2.eq(deck);
        
        if self.process.is_none() && start_condition && not_calculated {
            self.spawn(deck);
        }

        if let Some(ref x) = self.progless_resiever {
            if let Ok(o) = x.try_recv() {
                self.progless = o;
            }
        }
        if let Some(ref x) = self.total_ev_resiever {
            if let Ok((ev, betsize)) = x.try_recv() {
                self.total_ev = Some((
                    ev,
                    self.process.as_ref().unwrap().2,
                    self.process.as_ref().unwrap().1.clone(),
                ));
                self.optimal_betsize = Some(betsize);
                self.process = None;
                self.progless = 0.0;
            }
        }
    }
    fn spawn(&mut self, deck: &Deck) {
        //privateであるRULEやDECKをbinに変換しないといけない時点でセキュリティになってるとは思う
        let process = if self.wsl_installed && !CONFIG.read().general.disable_wsl {
            std::process::Command::new("wsl")
                .arg(
                    "RULE=".to_string() + &io_util::bytes_to_string(&CONFIG.read().rule.to_bytes()),
                )
                .arg(SUBPROCESS_WSL_PATH)
                .arg(&io_util::bytes_to_string(&deck.to_bytes()))
                .stdout(std::process::Stdio::piped())
                .creation_flags(CREATE_NO_WINDOW)
                .spawn()
                .unwrap()
        } else {
            std::process::Command::new(SUBPROCESS_PATH)
                .arg(&io_util::bytes_to_string(&deck.to_bytes()))
                .env(
                    "RULE",
                    &io_util::bytes_to_string(&CONFIG.read().rule.to_bytes()),
                )
                .stdout(std::process::Stdio::piped())
                .creation_flags(CREATE_NO_WINDOW)
                .spawn()
                .unwrap()
        };
        self.process = Some((Arc::new(Mutex::new(process)), deck.clone(), Instant::now()));
        self.stdout_resiever = Some(thread::spawn({
            let process = self.process.as_ref().unwrap().clone();
            let (total_ev_sender, total_ev_resiever) = mpsc::channel::<(f32, f32)>();
            let (progless_sender, progless_resiever) = mpsc::channel::<f32>();
            self.total_ev_resiever = Some(total_ev_resiever);
            self.progless_resiever = Some(progless_resiever);
            move || {
                let mut buffer = Vec::new();
                let process = &mut process.0.lock().unwrap();
                loop {
                    let mut temp_buf = vec![0; 10000];
                    if process.stdout.as_mut().unwrap().read(&mut temp_buf).is_ok() {
                        for item in temp_buf {
                            buffer.push(item);
                        }
                        let string = String::from_utf8(buffer.clone()).unwrap();
                        let strings: Vec<&str> = string.split('\n').collect();
                        if process.try_wait().unwrap().is_some() {
                            let temp: Vec<_> = strings.last().unwrap().split(',').collect();
                            let total_ev = (
                                temp.first().unwrap().parse().unwrap(),
                                temp.last().unwrap().parse().unwrap(),
                            );
                            total_ev_sender.send(total_ev).unwrap();
                            break;
                        } else if strings.len() >= 2 {
                            let temp = strings.get(strings.len() - 2).unwrap();
                            progless_sender.send(temp.parse().unwrap()).unwrap();
                        }
                    }
                    thread::sleep(Duration::from_millis(40));
                }
            }
        }));
    }
    fn stop(&mut self) {
        if let Some(p) = self.process.take() {
            thread::spawn(move || {
                let _ = p.0.lock().unwrap().kill();
            });
        }
    }
    pub fn reset(&mut self) {
        self.stop();
        *self = TotalEvHandler {
            //textures: self.textures.clone(),
            calculate: self.calculate.clone(),
            ..Default::default()
        };
    }
    pub fn draw_text(&mut self, ui: &mut Ui, table_state: &TableState) {
        ui.vertical_centered(|ui| {
            ui.label(
                RichText::new(match self.total_ev {
                    Some(ref mut s) => {
                        let percent = s.0 * 100.0;
                        let ago = if table_state.deck.eq(&s.2) {
                            s.1 = Instant::now();
                            0
                        } else {
                            (Instant::now() - s.1).as_secs() / 5 * 5
                        };
                        format!("{:4>+1.3}%\n({:>2}秒前)", percent, ago)
                    }
                    None => "\n".to_owned(),
                })
                .size(20.0),
            );
        });
    }
    pub fn draw_controller(&mut self, ui: &mut Ui, deck: &Deck) {
        ui.vertical_centered(|ui| {
            ui.add(ProgressBar::new(self.progless).animate(self.process.is_some()));
            ui.horizontal(|ui| {
                let color = if self.calculate == CalcMode::DealerStands {
                    Color32::GOLD
                } else {
                    Color32::DARK_GRAY
                };
                ui.add_space(3.0);
                if ui.add(Button::new("     ▶     ").fill(color)).clicked() {
                    match self.calculate {
                        CalcMode::Idle => {
                            self.calculate = CalcMode::DealerStands;
                            self.spawn(deck);
                        }
                        CalcMode::DealerStands => {
                            self.calculate = CalcMode::Idle;
                            self.stop();
                        }
                        CalcMode::Endless => self.calculate = CalcMode::DealerStands,
                    }
                }
                let color = if self.calculate == CalcMode::Endless {
                    Color32::GOLD
                } else {
                    Color32::DARK_GRAY
                };
                if ui.add(Button::new("     ∞     ").fill(color)).clicked() {
                    match self.calculate {
                        CalcMode::Idle => self.calculate = CalcMode::Endless,
                        CalcMode::DealerStands => self.calculate = CalcMode::Endless,
                        CalcMode::Endless => {
                            self.calculate = CalcMode::Idle;
                            self.stop();
                        }
                    }
                }
            });
        });
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let deck = Deck::new(8);
        let mut rule = Rule::default();
        rule.BJ_PAYBACK = 1.5;
        println!("{}", io_util::bytes_to_string(&rule.to_bytes()));
        println!("{}", SUBPROCESS_WSL_PATH);
        println!("{}", io_util::bytes_to_string(&deck.to_bytes()));
        println!(
            "wsl RULE=\"{}\" {} {}",
            io_util::bytes_to_string(&rule.to_bytes()),
            SUBPROCESS_WSL_PATH,
            io_util::bytes_to_string(&deck.to_bytes())
        );
    }
}

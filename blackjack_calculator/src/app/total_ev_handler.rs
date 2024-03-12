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
enum CalcState {
    Idle,
    Endless,
    Once,
}

pub struct TotalEvHandler {
    process: Option<(Arc<Mutex<std::process::Child>>, Deck)>,
    state: CalcState,
    calculation_result: Option<(f32, Deck)>,
    optimal_betsize: Option<f32>,
    total_ev_resiever: Option<Receiver<(f32, f32)>>,
    progress: f32,
    progless_resiever: Option<Receiver<f32>>,
    stdout_resiever: Option<JoinHandle<()>>,
    //textures: [Option<TextureHandle>; 2],
    pub wsl_installed: bool,
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
            progress: 0.0,
            process: None,
            calculation_result: None,
            optimal_betsize: None,
            state: CalcState::Idle,
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
        self.calculation_result.as_ref().map(|x| x.0)
    }
    pub fn get_optimal_betsize(&self) -> Option<f32> {
        self.optimal_betsize
    }
    pub fn update(&mut self, table: &TableState, ctx: &Context) {
        let deck = &table.deck;
        let dealer = &table.dealer;

        let start_condition = match self.state {
            CalcState::Endless => {
                dealer.stand()
                    || ctx.input(|input| {
                        input.key_pressed(CONFIG.read().kyes.next)
                            || input.key_pressed(CONFIG.read().kyes.reset)
                    })
            }
            _ => false,
        };
        let not_calculated = self.calculation_result.is_none()
            || !self.calculation_result.as_ref().unwrap().1.eq(deck);

        if start_condition && not_calculated {
            self.try_spawn(deck);
        }

        if let Some(ref x) = self.progless_resiever {
            if let Ok(o) = x.try_recv() {
                self.progress = o;
            }
        }
        if let Some(ref x) = self.total_ev_resiever {
            if let Ok((ev, betsize)) = x.try_recv() {
                let process = self.process.take();
                self.calculation_result = Some((ev, process.unwrap().1));
                self.optimal_betsize = Some(betsize);
                self.progress = 0.0;
                self.progless_resiever = None;
                if self.state == CalcState::Once {
                    self.state = CalcState::Idle
                }
                ctx.request_repaint_after(Duration::ZERO);
            }
        }
    }
    fn try_spawn(&mut self, deck: &Deck) -> Option<()> {
        if self.process.is_some() {
            return None;
        }
        self.spawn(deck);
        Some(())
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
        self.process = Some((Arc::new(Mutex::new(process)), deck.clone()));
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
                    let mut temp_buf = [0; 1000];
                    let process_finished = process.try_wait().unwrap().is_some();
                    let readed = process
                        .stdout
                        .as_mut()
                        .unwrap()
                        .read(&mut temp_buf)
                        .unwrap();
                    for item in temp_buf {
                        if item != 0 {
                            buffer.push(item);
                        }
                    }
                    if !process_finished || readed == 0 {
                        let string = String::from_utf8(buffer.clone()).unwrap();
                        let strings: Vec<&str> = string.split('\n').collect();
                        if process_finished {
                            let temp: Vec<_> = strings.last().unwrap().split(',').collect();
                            let total_ev = (
                                temp.first().unwrap().parse().unwrap_or_else(|_| {
                                    panic!("ParseFailed:{}", temp.first().unwrap())
                                }),
                                temp.last().unwrap().parse().unwrap_or_else(|_| {
                                    panic!("ParseFailed:{}", temp.last().unwrap())
                                }),
                            );
                            let _ = total_ev_sender.send(total_ev);
                            break;
                        } else if strings.len() >= 2 {
                            let temp = strings.get(strings.len() - 2).unwrap();
                            let _ = progless_sender.send(temp.parse().unwrap());
                        }
                    }
                    thread::sleep(Duration::from_millis(10));
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
            state: self.state.clone(),
            ..Default::default()
        };
    }
    pub fn draw_text(&mut self, ui: &mut Ui, table_state: &TableState) {
        ui.vertical_centered(|ui| {
            if let Some((ev, ref deck)) = self.calculation_result {
                let percent = ev * 100.0;
                let mut text = format!("{:4>+1.3}%", percent);

                let is_latest = table_state.deck.eq(deck);
                let color = if is_latest {
                    if ev > 0.0 {
                        Color32::LIGHT_GREEN
                    }else {
                        Color32::LIGHT_RED
                    }
                } else {
                    text = format!("({})", text);
                    if ev > 0.0 {
                        Color32::DARK_GREEN
                    }else {
                        Color32::DARK_RED
                    }
                };
                ui.label(RichText::new(text).size(20.0).color(color));
            } else {
                //print dummy
                let rich_text = RichText::new("").size(20.0);
                ui.label(rich_text);
            }
        });
    }
    pub fn draw_controller(&mut self, ui: &mut Ui, deck: &Deck) {
        ui.vertical_centered(|ui| {
            ui.add(ProgressBar::new(self.progress).animate(self.process.is_some()));
            ui.horizontal(|ui| {
                if ui
                    .add_enabled(
                        self.process.is_none(),
                        Button::new("     ▶     ").fill(Color32::DARK_GRAY),
                    )
                    .clicked()
                {
                    self.state = CalcState::Once;
                    self.try_spawn(deck);
                }

                let color = if self.state == CalcState::Endless {
                    Color32::GOLD
                } else {
                    Color32::DARK_GRAY
                };
                ui.add_space(3.0);
                if ui.add(Button::new("     ∞     ").fill(color)).clicked() {
                    match self.state {
                        CalcState::Idle => {
                            self.state = CalcState::Endless;
                            if self.calculation_result.is_none() {
                                self.try_spawn(deck);
                            }
                        }
                        CalcState::Endless => {
                            self.state = CalcState::Idle;
                            self.stop();
                        }
                        CalcState::Once => {
                            self.state = CalcState::Endless;
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

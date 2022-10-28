use super::*;
use std::{thread::{self, JoinHandle}, sync::{Arc,Mutex}};
use std::os::windows::process::CommandExt;
use std::io::Read;
use std::sync::mpsc::{self, Receiver, Sender};

pub struct TotalEvHandler {
    process: Option<(Arc<Mutex<std::process::Child>>, Deck, Instant)>,
    calculate: bool,
    total_ev: Option<(f32, Instant, Deck)>,
    total_ev_resiever: Option<Receiver<f32>>,
    progless: f32,
    progless_resiever: Option<Receiver<f32>>,
    stdout_resiever: Option<JoinHandle<()>>,
    textures: [Option<TextureHandle>; 2],
}
impl Default for TotalEvHandler {
    fn default() -> Self {
        TotalEvHandler {
            progless: 0.0,
            process: None,
            total_ev: None,
            calculate: false,
            stdout_resiever: None,
            progless_resiever: None,
            total_ev_resiever: None,
            textures: Default::default(),
        }
    }
}
impl TotalEvHandler {
    pub fn get_ev(&self) -> Option<f32>{
        match &self.total_ev {
            Some(x) => {Some(x.0)},
            None => {None},
        }
    }
    pub fn setup(&mut self, cc: &eframe::CreationContext<'_>) {
        let image = load_image_from_path(&format!("{}/play.png",IMAGE_FOLDER_PATH)).unwrap();
        self.textures[0] = Some(cc.egui_ctx.load_texture("play", image));
        let image = load_image_from_path(&format!("{}/stop.png",IMAGE_FOLDER_PATH)).unwrap();
        self.textures[1] = Some(cc.egui_ctx.load_texture("stop", image));
    }
    pub fn update(&mut self, config:&Config, deck: &Deck) {
        if self.calculate && self.process.is_none() {
            if self.total_ev.is_none() || !self.total_ev.as_ref().unwrap().2.eq(deck) {
                self.spawn(deck, &config.rule);
            }
        }
        if let Some(ref x) = self.progless_resiever {
            if let Ok(o) = x.try_recv() {
                self.progless = o;
            }
        }
        if let Some(ref x) = self.total_ev_resiever {
            if let Ok(o) = x.try_recv() {
                self.total_ev = Some((
                    o,
                    self.process.as_ref().unwrap().2,
                    self.process.as_ref().unwrap().1.clone(),
                ));
                self.process = None;
                self.progless = 0.0;
            }
        }
    }
    fn spawn(&mut self, deck: &Deck, rule: &Rule) {
        let process = std::process::Command::new(SUBPROCESS_PATH)
            .arg(&io_util::bytes_to_string(&deck.to_bytes()))
            .env("RULE", &io_util::bytes_to_string(&rule.to_bytes()))
            .stdout(std::process::Stdio::piped())
            .creation_flags(0x08000000)
            .spawn()
            .unwrap();
        self.process = Some((Arc::new(Mutex::new(process)), deck.clone(), Instant::now()));
        self.stdout_resiever = Some(thread::spawn({
            let process = self.process.as_ref().unwrap().clone();
            let (total_ev_sender, total_ev_resiever): (Sender<f32>, Receiver<f32>) =
                mpsc::channel();
            let (progless_sender, progless_resiever): (Sender<f32>, Receiver<f32>) =
                mpsc::channel();
            self.total_ev_resiever = Some(total_ev_resiever);
            self.progless_resiever = Some(progless_resiever);
            move || {
                let mut buffer = Vec::new();
                let process = &mut process.0.lock().unwrap();
                loop {
                    let mut temp_buf = vec![0; 10000];
                    if let Ok(readed) = process.stdout.as_mut().unwrap().read(&mut temp_buf) {
                        for i in 0..readed {
                            buffer.push(temp_buf[i]);
                        }
                        let string = String::from_utf8(buffer.clone()).unwrap();
                        let strings: Vec<&str> = string.split("\n").collect();
                        if process.try_wait().unwrap().is_some() {
                            let total_ev = strings.last().unwrap().parse().unwrap();
                            total_ev_sender.send(total_ev).unwrap();
                            break;
                        } else {
                            if strings.len() >= 2 {
                                let temp = strings.get(strings.len() - 2).unwrap();
                                progless_sender.send(temp.parse().unwrap()).unwrap();
                            }
                        }
                    }
                    thread::sleep(Duration::from_millis(40));
                }
            }
        }));
    }
    fn stop(&mut self) {
        let prev_ev = self.total_ev.clone();
        self.reset();
        self.total_ev = prev_ev;
    }
    pub fn reset(&mut self){
        if let Some(p) = self.process.take(){
            thread::spawn(move||{
                let _ = p.0.lock().unwrap().kill();
            });
        }
        *self = TotalEvHandler{
            textures:self.textures.clone(),
            ..Default::default()
        };
    }
    pub fn draw_contents(&mut self, ui: &mut Ui, table_state: &TableState) {
        ui.vertical_centered(|ui|{
            if self.calculate {
                if ui
                    .add(ImageButton::new(
                        self.textures[1].as_ref().unwrap(),
                        vec2(50.0, 50.0),
                    ))
                    .clicked()
                {
                    self.stop();
                };
            } else {
                if ui
                    .add(ImageButton::new(
                        self.textures[0].as_ref().unwrap(),
                        vec2(50.0, 50.0),
                    ))
                    .clicked()
                {
                    self.calculate = true;
                };
            }
            ui.add(ProgressBar::new(self.progless).animate(self.process.is_some()));
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
}
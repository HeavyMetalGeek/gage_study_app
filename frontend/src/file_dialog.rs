use eframe::{egui, epi};
use rfd;

pub enum Message {
    FileOpen(std::path::PathBuf),
    // Other messages
}

pub struct FileApp {
    message_channel: (
        std::sync::mpsc::Sender<Message>,
        std::sync::mpsc::Receiver<Message>,
    )
}

impl Default for FileApp {
    fn default() -> Self {
        Self {
            message_channel: std::sync::mpsc::channel(),
        }
    }
}

impl epi::App for FileApp {
    fn name(&self) -> &str {
        "file dialog app"
    }
    fn update(&mut self, ctx: &egui::CtxRef, _frame: &mut epi::Frame<'_>) {
        // This is important, otherwise file dialog can hang
        // and messages are not processed
        ctx.request_repaint();

        loop {
            match self.message_channel.1.try_recv() {
                Ok(_message) => {
                    // Process FileOpen and other messages
                }
                Err(_) => {
                    break;
                }
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            let open_button = ui.add(egui::widgets::Button::new("Open..."));

            if open_button.clicked() {
                let task = rfd::AsyncFileDialog::new()
                    .add_filter("Text files", &["txt"])
                    .set_directory("/")
                    .pick_file();

                let message_sender = self.message_channel.0.clone();

                execute(async move {
                    let file = task.await;

                    if let Some(file) = file {
                        //let file_path = file;
                        let _tentative_file = file;
                        let file_path = std::path::PathBuf::from("idk");
                        message_sender.send(Message::FileOpen(file_path)).ok();
                    }
                });

            }
        });
    }
}

use std::future::Future;

#[cfg(not(target_arch = "wasm32"))]
fn execute<F: Future<Output = ()> + Send + 'static>(f: F) {
    // this is stupid... use any executor of your choice instead
    std::thread::spawn(move || futures::executor::block_on(f));
}
#[cfg(target_arch = "wasm32")]
fn execute<F: Future<Output = ()> + 'static>(f: F) {
    wasm_bindgen_futures::spawn_local(f);
}

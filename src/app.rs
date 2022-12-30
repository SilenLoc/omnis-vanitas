use std::sync::mpsc::{Receiver, Sender};
use translation_server_client_silen::ready;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
pub struct TemplateApp {
    tx: Sender<String>,

    rx: Receiver<String>,

    open: bool,
    translation_server_ready: String,
}

impl Default for TemplateApp {
    fn default() -> Self {
        let (tx, rx) = std::sync::mpsc::channel();

        Self {
            tx,
            rx,
            open: false,
            translation_server_ready: "nothing".to_owned(),
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new() -> Self {
        Default::default()
    }
}

impl eframe::App for TemplateApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self {
            tx: _,
            rx: _,
            open: _,
            translation_server_ready,
        } = self;

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Side Panel");

            if ui.button("open a window").clicked() {
                self.open = true;
            }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("powered by ");
                    ui.hyperlink_to("me", "https://github.com/SilenLoc");
                    ui.label(".");
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(&*translation_server_ready);
        });

        if self.open {
            egui::Window::new("Window").show(ctx, |ui| {
                if ui.button("Close").clicked() {
                    self.open = false;
                }

                if ui.button("is server ready?").clicked() {
                    send_req(self.tx.clone(), ctx.clone());
                }
            });
        }

        if let Ok(ready_status) = self.rx.try_recv() {
            self.translation_server_ready = ready_status;
        }
    }
}

fn send_req(tx: Sender<String>, ctx: egui::Context) {
    tokio::spawn(async move {
        // Send a request with an increment value.
        let response = ready().await;

        let result = match response {
            Ok(v) => v.text().await.map_err(|e| e.to_string()).unwrap(),
            Err(e) => e,
        };
        let _ = tx.send(result);

        ctx.request_repaint();
    });
}

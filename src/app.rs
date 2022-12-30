pub(crate) mod native {

    use std::sync::mpsc::{Receiver, Sender};

    use translation_server_dtos_silen::{TransErr, TransResponse};

    use crate::{check_ready, outcalls::translate_from_to};

    /// We derive Deserialize/Serialize so we can persist app state on shutdown.
    pub struct App {
        tx: Sender<String>,
        rx: Receiver<String>,

        tx_trans: Sender<Result<TransResponse, TransErr>>,
        rx_trans: Receiver<Result<TransResponse, TransErr>>,

        translate: String,
        from: String,
        to: String,
        translate_to: String,

        open: bool,
        translation_server_ready: String,
    }

    impl Default for App {
        fn default() -> Self {
            let (tx, rx) = std::sync::mpsc::channel();
            let (tx_trans, rx_trans) = std::sync::mpsc::channel();

            Self {
                tx,
                rx,
                tx_trans,
                rx_trans,
                translate: "to trans".to_owned(),
                from: "en".to_owned(),
                to: "elb".to_owned(),
                translate_to: "".to_owned(),
                open: false,
                translation_server_ready: "nothing".to_owned(),
            }
        }
    }

    impl App {
        /// Called once before the first frame.
        pub fn new() -> Self {
            Default::default()
        }
    }

    impl eframe::App for App {
        fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
            let Self {
                tx: _,
                rx: _,
                tx_trans,
                rx_trans,
                translate,
                from,
                to,
                translate_to,
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

                if ui.button("is server ready?").clicked() {
                    check_ready(self.tx.clone(), ctx.clone());
                }

                if ui.button("translate").clicked() {
                    translate_from_to(
                        translate.clone(),
                        from.clone(),
                        to.clone(),
                        tx_trans.clone(),
                        ctx.clone(),
                    )
                }

                ui.label("to trans");
                ui.text_edit_multiline(translate);
                ui.label("from");
                ui.text_edit_multiline(from);
                ui.label("to");
                ui.text_edit_multiline(to);

                ui.label("result");
                ui.text_edit_multiline(translate_to);
            });

            if self.open {
                egui::Window::new("Window").show(ctx, |ui| {
                    if ui.button("Close").clicked() {
                        self.open = false;
                    }
                });
            }

            if let Ok(ready_status) = self.rx.try_recv() {
                self.translation_server_ready = ready_status;
            }
            if let Ok(answer) = self.rx_trans.try_recv() {
                self.translate_to = match answer {
                    Ok(v) => v.content,
                    Err(e) => e.content,
                }
            }
        }
    }
}

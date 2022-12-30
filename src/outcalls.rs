use std::sync::mpsc::Sender;

use translation_server_client_silen::ready;
use translation_server_dtos_silen::{TransErr, TransResponse};

pub fn check_ready(tx: Sender<String>, ctx: egui::Context) {
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

pub fn translate_from_to(
    content: String,
    from: String,
    to: String,
    tx_trans: Sender<Result<TransResponse, TransErr>>,
    ctx: egui::Context,
) {
    tokio::spawn(async move {
        // Send a request with an increment value.
        let response: Result<TransResponse, TransErr> = translation_server_client_silen::translate(
            content.as_str(),
            from.as_str(),
            to.as_str(),
        )
        .await;

        let _ = tx_trans.send(response);

        ctx.request_repaint();
    });
}

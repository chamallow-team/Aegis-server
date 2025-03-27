use rocket::futures::{SinkExt, StreamExt};
use rocket::get;
use rocket_ws::{Channel as WsChannel, WebSocket};
use serde::{Deserialize, Serialize};

#[get("/")]
pub(crate) fn ws_listener(ws: WebSocket) -> WsChannel<'static> {
    ws.channel(move |mut stream| {
        Box::pin(async move {
            while let Some(message) = stream.next().await {
                println!("{:?}", &message);
                let _ = stream.send(message?).await;
            }

            Ok(())
        })
    })
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct SocketMessage {
    identity: (),
    body: (),
}

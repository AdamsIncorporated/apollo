use actix_web::{rt, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_ws::AggregatedMessage;
use futures_util::StreamExt as _;
use log::info;

async fn echo(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let (res, mut session, stream) = actix_ws::handle(&req, stream)?;

    // Log when WebSocket connection is established
    if let Some(peer_addr) = req.peer_addr() {
        info!("WebSocket connection established for: {}", peer_addr);
    } else {
        info!("WebSocket connection established, but peer address could not be determined");
    }

    let mut stream = stream
        .aggregate_continuations()
        .max_continuation_size(2_usize.pow(20));

    // start task but don't wait for it
    rt::spawn(async move {
        // receive messages from WebSocket
        while let Some(msg) = stream.next().await {
            match msg {
                Ok(AggregatedMessage::Text(text)) => {
                    // Log received text message
                    info!("Received text message: {}", text);
                    // echo text message
                    if let Err(e) = session.text(text).await {
                        info!("Error sending text message: {}", e);
                    }
                }

                Ok(AggregatedMessage::Binary(bin)) => {
                    // Log received binary message
                    info!("Received binary message with size: {}", bin.len());
                    // echo binary message
                    if let Err(e) = session.binary(bin).await {
                        info!("Error sending binary message: {}", e);
                    }
                }

                Ok(AggregatedMessage::Ping(msg)) => {
                    // Log received ping message
                    info!("Received PING message: {:?}", msg);
                    // respond to PING frame with PONG frame
                    if let Err(e) = session.pong(&msg).await {
                        info!("Error sending PONG message: {}", e);
                    }
                }

                _ => {
                    // Log unhandled message type
                    info!("Received an unsupported message type");
                }
            }
        }
    });

    Ok(res)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Log server start
    info!("Starting server on http://127.0.0.1:8080");

    HttpServer::new(|| App::new().route("/echo", web::get().to(echo)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

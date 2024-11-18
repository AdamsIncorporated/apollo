use actix_web::{rt, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_ws::AggregatedMessage;
use futures_util::StreamExt as _;
use log::{info, error};  // Import error level for logging
use std::sync::{Arc, Mutex};
use env_logger;  // Import env_logger

#[derive(Clone)]
struct AppState {
    counter: Arc<Mutex<u32>>,  // Shared counter state
}

async fn echo(req: HttpRequest, stream: web::Payload, data: web::Data<AppState>) -> Result<HttpResponse, Error> {
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

    // Start task but don't wait for it
    rt::spawn(async move {
        // Receive messages from WebSocket
        while let Some(msg) = stream.next().await {
            match msg {
                Ok(AggregatedMessage::Text(text)) => {
                    // Log received text message
                    info!("Received text message: {}", text);
                    // Increment counter
                    let mut counter = data.counter.lock().unwrap();
                    *counter += 1;
                    let counter_value = *counter;
                    
                    // Send updated counter back to the client
                    let response_message = format!("{} | Counter: {}", text, counter_value);
                    if let Err(e) = session.text(response_message).await {
                        error!("Error sending text message: {}", e);  // Use error level for failures
                    }
                }

                Ok(AggregatedMessage::Binary(bin)) => {
                    // Log received binary message
                    info!("Received binary message with size: {}", bin.len());
                    // Echo binary message
                    if let Err(e) = session.binary(bin).await {
                        error!("Error sending binary message: {}", e);  // Use error level for failures
                    }
                }

                Ok(AggregatedMessage::Ping(msg)) => {
                    // Log received ping message
                    info!("Received PING message: {:?}", msg);
                    // Respond to PING frame with PONG frame
                    if let Err(e) = session.pong(&msg).await {
                        error!("Error sending PONG message: {}", e);  // Use error level for failures
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
    // Initialize the logger
    env_logger::init();

    // Shared application state with a counter
    let app_state = web::Data::new(AppState {
        counter: Arc::new(Mutex::new(0)),  // Initial counter value
    });

    // Log server start
    info!("Starting server on http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())  // Pass the state to the route
            .route("/echo", web::get().to(echo))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

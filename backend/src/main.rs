use actix::*;
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_cors::Cors;
use actix_web_actors::ws;
use log::{error, info};
use std::time::{Duration, Instant};

/// Define a WebSocket actor
struct CounterWebSocket {
    counter: u64,
    hb: Instant, // for tracking the heartbeat
}

impl CounterWebSocket {
    fn new() -> Self {
        info!("Creating new WebSocket actor"); // Log when a new WebSocket actor is created
        CounterWebSocket {
            counter: 0,
            hb: Instant::now(),
        }
    }

    /// Start a recurring task to send the counter to the client
    fn start_counter(&mut self, ctx: &mut ws::WebsocketContext<Self>) {
        info!("Starting counter task"); // Log when the counter task starts
        ctx.run_interval(Duration::from_secs(1), |act, ctx| {
            act.counter += 1;
            ctx.text(act.counter.to_string());
        });
    }
}

impl Actor for CounterWebSocket {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        info!("WebSocket connection started"); // Log when the connection starts
        self.start_counter(ctx); // Start the counter when the WebSocket is connected
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        info!("WebSocket connection stopped"); // Log when the connection stops
        Running::Stop
    }
}

/// Handle WebSocket messages
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for CounterWebSocket {
    fn handle(
        &mut self,
        msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut ws::WebsocketContext<Self>,
    ) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                info!("Received Ping message: {:?}", msg); // Log Ping message
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
                info!("Received Pong message"); // Log Pong message
            }
            Ok(ws::Message::Text(text)) => {
                info!("Received Text message: {}", text); // Log received Text message
            }
            Ok(ws::Message::Binary(_)) => {
                info!("Received Binary message"); // Log received Binary message
            }
            Ok(ws::Message::Close(reason)) => {
                info!("Received Close message: {:?}", reason); // Log Close message
                ctx.close(reason);
                ctx.stop();
            }
            Ok(ws::Message::Continuation(_)) => {
                info!("Received Continuation message"); // Log Continuation message (if needed)
            }
            Err(e) => {
                error!("Error in WebSocket message: {:?}", e); // Log any error
            }
            _ => (), // Ignore NOP messages
        }
    }
}

/// WebSocket handler
async fn ws_handler(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    info!("WebSocket request received: {:?}", req); // Log incoming WebSocket request
    let ws = CounterWebSocket::new();
    ws::start(ws, &req, stream)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    info!("Starting server...");

    HttpServer::new(|| {
        App::new()
            .wrap(Cors::default().allowed_origin("http://localhost:3000"))
            .route("/echo/", web::get().to(ws_handler))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

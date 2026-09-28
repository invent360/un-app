//! WebSocket connection handler for /ws/jobs

use actix_web::{web, HttpRequest, HttpResponse};
use actix_ws::Message;
use futures_util::StreamExt;
use std::time::{Duration, Instant};

use super::subscribe;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(30);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(60);

/// WebSocket endpoint handler for job updates
pub async fn ws_jobs_handler(
    req: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, actix_web::Error> {
    let (response, mut session, mut msg_stream) = actix_ws::handle(&req, stream)?;

    // Subscribe to job events
    let rx = match subscribe() {
        Some(rx) => rx,
        None => {
            tracing::error!("Broadcaster not initialized");
            return Ok(HttpResponse::InternalServerError().finish());
        }
    };

    tracing::info!("New WebSocket client connected");

    // Spawn WebSocket actor task
    actix_web::rt::spawn(async move {
        let mut rx = rx;
        let mut last_heartbeat = Instant::now();
        let mut interval = actix_web::rt::time::interval(HEARTBEAT_INTERVAL);

        loop {
            tokio::select! {
                // Handle incoming client messages
                Some(msg) = msg_stream.next() => {
                    match msg {
                        Ok(Message::Ping(bytes)) => {
                            last_heartbeat = Instant::now();
                            if session.pong(&bytes).await.is_err() {
                                tracing::debug!("Failed to send pong, closing connection");
                                break;
                            }
                        }
                        Ok(Message::Pong(_)) => {
                            last_heartbeat = Instant::now();
                        }
                        Ok(Message::Text(text)) => {
                            // Client can send commands if needed
                            tracing::debug!("Received text from client: {}", text);
                        }
                        Ok(Message::Close(_)) => {
                            tracing::debug!("Client sent close frame");
                            break;
                        }
                        Err(e) => {
                            tracing::debug!("WebSocket error: {:?}", e);
                            break;
                        }
                        _ => {}
                    }
                }
                // Handle job events from broadcaster
                result = rx.recv() => {
                    match result {
                        Ok(event) => {
                            let json = match serde_json::to_string(&event) {
                                Ok(j) => j,
                                Err(e) => {
                                    tracing::error!("Failed to serialize event: {}", e);
                                    continue;
                                }
                            };
                            if session.text(json).await.is_err() {
                                tracing::debug!("Failed to send message, closing connection");
                                break;
                            }
                        }
                        Err(e) => {
                            tracing::debug!("Broadcast receiver error: {:?}", e);
                            // Channel lagged or closed, continue
                        }
                    }
                }
                // Send periodic pings
                _ = interval.tick() => {
                    if Instant::now().duration_since(last_heartbeat) > CLIENT_TIMEOUT {
                        tracing::debug!("WebSocket client timeout, closing");
                        break;
                    }
                    if session.ping(b"").await.is_err() {
                        tracing::debug!("Failed to send ping, closing connection");
                        break;
                    }
                }
            }
        }

        tracing::info!("WebSocket client disconnected");
        let _ = session.close(None).await;
    });

    Ok(response)
}

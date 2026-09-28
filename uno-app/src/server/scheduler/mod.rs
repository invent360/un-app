//! Background scheduler for automated content operations

use std::time::Duration;
use tokio::time::interval;
use crate::server::app::ServiceFactory;

/// Default scheduler interval in seconds
const SCHEDULER_INTERVAL_SECS: u64 = 60;

/// Start the content scheduler background task
///
/// This function spawns a background task that periodically:
/// - Publishes content that has reached its scheduled publish time
/// - Unpublishes (archives) content that has reached its scheduled unpublish time
pub async fn start_content_scheduler(factory: ServiceFactory) {
    let interval_secs = std::env::var("SCHEDULER_INTERVAL_SECS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(SCHEDULER_INTERVAL_SECS);

    tracing::info!(
        "Starting content scheduler (interval: {}s)",
        interval_secs
    );

    let mut ticker = interval(Duration::from_secs(interval_secs));

    loop {
        ticker.tick().await;

        match factory.content_service.process_scheduled_content().await {
            Ok((published, unpublished)) => {
                if published > 0 || unpublished > 0 {
                    tracing::info!(
                        "Content scheduler: published={}, unpublished={}",
                        published, unpublished
                    );
                }
            }
            Err(e) => {
                tracing::error!("Content scheduler error: {}", e);
            }
        }
    }
}

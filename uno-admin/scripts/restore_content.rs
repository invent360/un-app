//! CLI tool to restore task and guide content to uno-app CMS
//!
//! Usage: cargo run --bin restore-content --features ssr

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use uno_admin::api::content_client::{ContentClient, UpsertContentRequest, PublishContentRequest};

/// Task description from body.json
#[derive(Debug, Deserialize)]
struct TaskBody {
    title: String,
    description: String,
    #[serde(default)]
    images: Vec<String>,
    #[serde(default)]
    status: Option<String>,
    #[serde(default)]
    difficulty: Option<String>,
    #[serde(default)]
    duration: Option<String>,
    #[serde(default)]
    earnings: Vec<EarningsTier>,
    #[serde(default)]
    requirements: Vec<String>,
}

/// Guide from body.json
#[derive(Debug, Deserialize)]
struct GuideBody {
    title: String,
    description: String,
    #[serde(default)]
    difficulty: Option<String>,
    #[serde(default)]
    duration_minutes: Option<i32>,
    #[serde(default)]
    steps: Vec<GuideStep>,
}

/// Earnings tier
#[derive(Debug, Deserialize, Serialize)]
struct EarningsTier {
    name: String,
    min_earnings: f64,
    max_earnings: f64,
    period: String,
    features: Vec<String>,
    is_popular: bool,
}

/// Guide step
#[derive(Debug, Deserialize, Serialize)]
struct GuideStep {
    order: i32,
    title: String,
    description: String,
    image: Option<String>,
}

const TASKS_DIR: &str = "/Users/admin/Documents/unetwork/docs/content/01_tasks/sections";
const GUIDES_DIR: &str = "/Users/admin/Documents/unetwork/docs/content/02_guides/sections";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    // Initialize tracing
    tracing_subscriber::fmt::init();

    println!("=== UNO Content Restore Tool ===\n");

    // Create content client
    let client = ContentClient::from_env()?;

    // Restore tasks
    println!("=== Restoring Tasks ===\n");
    restore_tasks(&client).await?;

    // Restore guides
    println!("\n=== Restoring Guides ===\n");
    restore_guides(&client).await?;

    println!("\n=== Restore Complete ===");
    Ok(())
}

async fn restore_tasks(client: &ContentClient) -> Result<(), Box<dyn std::error::Error>> {
    let tasks_path = Path::new(TASKS_DIR);

    if !tasks_path.exists() {
        println!("Tasks directory not found: {}", TASKS_DIR);
        return Ok(());
    }

    // Map directory names to slugs
    let task_slugs = vec![
        ("01_telemetry", "telemetry"),
        ("02_caller_id_testing", "caller-id-testing"),
        ("03_sms_testing", "sms-testing"),
        ("04_connectivity_verification", "connectivity-verification"),
        ("05_entropy_generation", "entropy-generation"),
    ];

    for (dir_name, slug) in task_slugs {
        let body_path = tasks_path.join(dir_name).join("body.json");

        if !body_path.exists() {
            println!("Skipping task {} - no body.json found at {:?}", slug, body_path);
            continue;
        }

        println!("Restoring task: {}", slug);

        let content_str = fs::read_to_string(&body_path)?;
        let task: TaskBody = serde_json::from_str(&content_str)?;

        // Build image URLs
        let image_base = format!("/assets/tasks/{}", dir_name);
        let images: Vec<String> = task.images.iter()
            .map(|img| {
                if img.starts_with('/') {
                    img.clone()
                } else {
                    format!("{}/{}", image_base, img.trim_start_matches("images/"))
                }
            })
            .collect();

        let front_image = if images.is_empty() {
            format!("{}/cover/0.png", image_base)
        } else {
            images.first().cloned().unwrap_or(format!("{}/cover/0.png", image_base))
        };

        // Build content JSON
        let content = serde_json::json!({
            "title": task.title,
            "description": task.description,
            "image": front_image,
            "images": images,
            "status": task.status.unwrap_or_else(|| "active".to_string()),
            "difficulty": task.difficulty,
            "duration": task.duration,
            "earnings_estimate": task.earnings,
            "requirements": task.requirements,
        });

        let request = UpsertContentRequest {
            content_type: "task".to_string(),
            slug: slug.to_string(),
            content,
            translations: None,
            display_order: None,
            is_featured: Some(false),
            change_summary: Some("Restored from backup".to_string()),
        };

        match client.create_content(request.clone()).await {
            Ok(response) => {
                println!("  Created task {} (id: {})", slug, response.id);

                // Publish the content
                let publish_req = PublishContentRequest {
                    published_by: Some("restore-script".to_string()),
                };
                if let Err(e) = client.publish_content(response.id, publish_req).await {
                    eprintln!("  Warning: Failed to publish task: {}", e);
                } else {
                    println!("  Published task {}", slug);
                }
            }
            Err(e) => {
                let err_str = e.to_string();
                if err_str.contains("already exists") || err_str.contains("duplicate") {
                    println!("  Task {} already exists, skipping...", slug);
                } else {
                    eprintln!("  Error creating task {}: {}", slug, e);
                }
            }
        }
    }

    Ok(())
}

async fn restore_guides(client: &ContentClient) -> Result<(), Box<dyn std::error::Error>> {
    let guides_path = Path::new(GUIDES_DIR);

    if !guides_path.exists() {
        println!("Guides directory not found: {}", GUIDES_DIR);
        return Ok(());
    }

    // Map directory names to slugs
    let guide_slugs = vec![
        ("01_installation", "installation"),
        ("02_setup", "app-setup-guide"),
        ("03_telemetry", "telemetry-activation-guide"),
        ("04_caller_id_testing", "caller-id-testing-activation-guide"),
        ("05_connectivity_verification", "connectivity-verification-activation-guide"),
        ("06_sms_testing", "sms-testing-activation-guide"),
        ("07_entropy_generation", "entropy-generation-activation-guide"),
        ("08_crypto_incentive_withdrawal", "crypto-incentive-withdrawal"),
        ("09_bank_incentive_withdrawal", "bank-incentive-withdrawal"),
    ];

    for (dir_name, slug) in guide_slugs {
        let body_path = guides_path.join(dir_name).join("body.json");

        if !body_path.exists() {
            println!("Skipping guide {} - no body.json found at {:?}", slug, body_path);
            continue;
        }

        println!("Restoring guide: {}", slug);

        let content_str = fs::read_to_string(&body_path)?;
        let guide: GuideBody = serde_json::from_str(&content_str)?;

        // Build image URLs for steps
        let image_base = format!("/assets/guides/{}", dir_name);
        let steps: Vec<serde_json::Value> = guide.steps.iter()
            .map(|step| {
                let image = step.image.as_ref().map(|img| {
                    if img.starts_with('/') {
                        img.clone()
                    } else {
                        format!("{}/steps/{}", image_base, img)
                    }
                });
                serde_json::json!({
                    "order": step.order,
                    "title": step.title,
                    "description": step.description,
                    "image": image,
                })
            })
            .collect();

        // Get thumbnail
        let thumbnail = format!("{}/cover/0.png", image_base);

        // Build content JSON
        let content = serde_json::json!({
            "title": guide.title,
            "description": guide.description,
            "thumbnail": thumbnail,
            "steps": steps,
            "duration_minutes": guide.duration_minutes,
            "difficulty": guide.difficulty,
        });

        let request = UpsertContentRequest {
            content_type: "guide".to_string(),
            slug: slug.to_string(),
            content,
            translations: None,
            display_order: None,
            is_featured: Some(false),
            change_summary: Some("Restored from backup".to_string()),
        };

        match client.create_content(request.clone()).await {
            Ok(response) => {
                println!("  Created guide {} (id: {})", slug, response.id);

                // Publish the content
                let publish_req = PublishContentRequest {
                    published_by: Some("restore-script".to_string()),
                };
                if let Err(e) = client.publish_content(response.id, publish_req).await {
                    eprintln!("  Warning: Failed to publish guide: {}", e);
                } else {
                    println!("  Published guide {}", slug);
                }
            }
            Err(e) => {
                let err_str = e.to_string();
                if err_str.contains("already exists") || err_str.contains("duplicate") {
                    println!("  Guide {} already exists, skipping...", slug);
                } else {
                    eprintln!("  Error creating guide {}: {}", slug, e);
                }
            }
        }
    }

    Ok(())
}

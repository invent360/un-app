//! CLI tool to upload task and guide content to uno-app CMS
//!
//! Usage: cargo run --bin upload-content --features ssr

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use uno_admin::api::content_client::{ContentClient, UpsertContentRequest, PublishContentRequest};

/// Task description from JSON file
#[derive(Debug, Deserialize)]
struct TaskDescription {
    title: String,
    description: String,
    images: Vec<String>,
    status: String,
    difficulty: Option<String>,
    duration: Option<String>,
    earnings: Vec<EarningsTier>,
    requirements: Vec<String>,
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

/// Activation guide from JSON file
#[derive(Debug, Deserialize)]
struct ActivationGuide {
    title: String,
    description: String,
    difficulty: Option<String>,
    duration_minutes: Option<i32>,
    steps: Vec<GuideStep>,
}

/// Guide step
#[derive(Debug, Deserialize, Serialize)]
struct GuideStep {
    order: i32,
    title: String,
    description: String,
    image: Option<String>,
}

const TASKS_DIR: &str = "/Users/admin/Documents/unetwork/docs/content/04_tasks";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    // Initialize tracing
    tracing_subscriber::fmt::init();

    println!("=== UNO Content Upload Tool ===\n");

    // Create content client
    let client = ContentClient::from_env()?;

    // Define task directories and their slugs
    let tasks = vec![
        ("01_telemetry", "telemetry"),
        ("02_caller_id_testing", "caller-id-testing"),
        ("03_sms_testing", "sms-testing"),
        ("04_connectivity_verification", "connectivity-verification"),
        ("05_entropy_generation", "entropy-generation"),
    ];

    // Upload each task
    for (dir_name, slug) in &tasks {
        let task_dir = Path::new(TASKS_DIR).join(dir_name);

        // Upload task description
        if let Err(e) = upload_task(&client, &task_dir, slug).await {
            eprintln!("Failed to upload task {}: {}", slug, e);
        }

        // Upload activation guide
        if let Err(e) = upload_guide(&client, &task_dir, slug).await {
            eprintln!("Failed to upload guide for {}: {}", slug, e);
        }
    }

    println!("\n=== Upload Complete ===");
    Ok(())
}

async fn upload_task(
    client: &ContentClient,
    task_dir: &Path,
    slug: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let desc_path = task_dir.join("description/description.json");

    if !desc_path.exists() {
        println!("Skipping task {} - no description.json found", slug);
        return Ok(());
    }

    println!("Uploading task: {}", slug);

    let desc_content = fs::read_to_string(&desc_path)?;
    let desc: TaskDescription = serde_json::from_str(&desc_content)?;

    // Build image URLs (assuming images will be served from a static path)
    let image_base = format!("/assets/tasks/{}", task_dir.file_name().unwrap().to_str().unwrap());
    let images: Vec<String> = desc.images.iter()
        .map(|img| format!("{}/{}", image_base, img))
        .collect();

    // Get front image (0.png)
    let front_image = format!("{}/0.png", image_base);

    // Build task content
    let content = serde_json::json!({
        "title": desc.title,
        "description": desc.description,
        "image": front_image,
        "images": images,
        "status": desc.status,
        "difficulty": desc.difficulty,
        "duration": desc.duration,
        "earnings_estimate": desc.earnings,
        "requirements": desc.requirements,
    });

    let request = UpsertContentRequest {
        content_type: "task".to_string(),
        slug: slug.to_string(),
        content,
        translations: None,
        display_order: None,
        is_featured: Some(false),
        change_summary: Some("Initial upload from task data".to_string()),
    };

    match client.create_content(request.clone()).await {
        Ok(response) => {
            println!("  Created task {} (id: {})", slug, response.id);

            // Publish the content
            let publish_req = PublishContentRequest {
                published_by: Some("upload-script".to_string()),
            };
            if let Err(e) = client.publish_content(response.id, publish_req).await {
                eprintln!("  Warning: Failed to publish task: {}", e);
            } else {
                println!("  Published task {}", slug);
            }
        }
        Err(e) => {
            // If it already exists, try to update it
            let err_str = e.to_string();
            if err_str.contains("already exists") || err_str.contains("duplicate") {
                println!("  Task {} already exists, skipping...", slug);
            } else {
                return Err(e.into());
            }
        }
    }

    Ok(())
}

async fn upload_guide(
    client: &ContentClient,
    task_dir: &Path,
    task_slug: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let guide_path = task_dir.join("activation/activation.json");

    if !guide_path.exists() {
        println!("Skipping guide for {} - no activation.json found", task_slug);
        return Ok(());
    }

    let guide_slug = format!("{}-activation-guide", task_slug);
    println!("Uploading guide: {}", guide_slug);

    let guide_content = fs::read_to_string(&guide_path)?;
    let guide: ActivationGuide = serde_json::from_str(&guide_content)?;

    // Build image URLs for steps
    let image_base = format!("/assets/tasks/{}", task_dir.file_name().unwrap().to_str().unwrap());
    let steps: Vec<serde_json::Value> = guide.steps.iter()
        .map(|step| {
            let image = step.image.as_ref().map(|img| format!("{}/{}", image_base, img));
            serde_json::json!({
                "order": step.order,
                "title": step.title,
                "description": step.description,
                "image": image,
            })
        })
        .collect();

    // Get thumbnail (use task front image)
    let thumbnail = format!("{}/0.png", image_base);

    // Build guide content
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
        slug: guide_slug.clone(),
        content,
        translations: None,
        display_order: None,
        is_featured: Some(false),
        change_summary: Some("Initial upload from activation data".to_string()),
    };

    match client.create_content(request.clone()).await {
        Ok(response) => {
            println!("  Created guide {} (id: {})", guide_slug, response.id);

            // Publish the content
            let publish_req = PublishContentRequest {
                published_by: Some("upload-script".to_string()),
            };
            if let Err(e) = client.publish_content(response.id, publish_req).await {
                eprintln!("  Warning: Failed to publish guide: {}", e);
            } else {
                println!("  Published guide {}", guide_slug);
            }
        }
        Err(e) => {
            let err_str = e.to_string();
            if err_str.contains("already exists") || err_str.contains("duplicate") {
                println!("  Guide {} already exists, skipping...", guide_slug);
            } else {
                return Err(e.into());
            }
        }
    }

    Ok(())
}

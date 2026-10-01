//! Support server functions for ticket submission and exit flow
//!
//! Implements F1-A and F1-B: Connect UI to actual API endpoints.

use leptos::prelude::*;
use serde::{Deserialize, Serialize};

/// Input for creating a support ticket from the UI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTicketRequest {
    pub category: String,
    pub subject: String,
    pub description: String,
}

/// Response from ticket creation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTicketResponse {
    pub success: bool,
    pub ticket_number: Option<String>,
    pub error: Option<String>,
}

/// Input for initiating an exit from the UI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitiateExitRequest {
    pub license_id: String,
    pub exit_reason: Option<String>,
}

/// Response from exit initiation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitiateExitResponse {
    pub success: bool,
    pub exit_id: Option<String>,
    pub confirmation_number: Option<String>,
    pub error: Option<String>,
}

/// Summary of user's support tickets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TicketSummary {
    pub id: String,
    pub ticket_number: String,
    pub subject: String,
    pub status: String,
    pub created_at: String,
    pub has_unread_reply: bool,
}

/// Submit a new support ticket
#[server(SubmitTicket, "/api")]
pub async fn submit_ticket(
    request: CreateTicketRequest,
) -> Result<CreateTicketResponse, ServerFnError> {
    use actix_web::web::Data;
    use actix_web::HttpRequest;
    use leptos_actix::extract;
    use crate::server::app::ServiceFactory;
    use crate::server::extractors::auth::get_actor_id;
    use crate::server::repositories::{
        CreateTicketInput, TicketCategory, TicketPriority,
    };

    let factory: Data<ServiceFactory> = extract().await?;
    let http_req: HttpRequest = extract().await?;

    // Get authenticated user ID
    let user_id = get_actor_id(&http_req)
        .map_err(|e| ServerFnError::new(format!("Authentication required: {}", e)))?;

    // Parse category
    let category = match request.category.as_str() {
        "general" | "other" => TicketCategory::Other,
        "technical" => TicketCategory::Technical,
        "payment" | "billing" => TicketCategory::Billing,
        "account" => TicketCategory::Account,
        _ => TicketCategory::Other,
    };

    let input = CreateTicketInput {
        user_id,
        license_id: None, // Could be added if needed
        category,
        priority: TicketPriority::Normal,
        subject: request.subject,
        description: Some(request.description),
        locale: None,
        country_code: None,
        tags: vec![],
    };

    match factory.support_repository.create_ticket(input).await {
        Ok(ticket) => Ok(CreateTicketResponse {
            success: true,
            ticket_number: Some(ticket.ticket_number),
            error: None,
        }),
        Err(e) => Ok(CreateTicketResponse {
            success: false,
            ticket_number: None,
            error: Some(e.to_string()),
        }),
    }
}

/// Get user's support tickets
#[server(GetUserTickets, "/api")]
pub async fn get_user_tickets() -> Result<Vec<TicketSummary>, ServerFnError> {
    use actix_web::web::Data;
    use actix_web::HttpRequest;
    use leptos_actix::extract;
    use crate::server::app::ServiceFactory;
    use crate::server::extractors::auth::get_actor_id;
    use crate::server::repositories::TicketFilters;

    let factory: Data<ServiceFactory> = extract().await?;
    let http_req: HttpRequest = extract().await?;

    // Get authenticated user ID
    let user_id = get_actor_id(&http_req)
        .map_err(|e| ServerFnError::new(format!("Authentication required: {}", e)))?;

    let filters = TicketFilters {
        user_id: Some(user_id),
        ..Default::default()
    };

    let tickets = factory.support_repository
        .search_tickets(&filters, 50, 0)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(tickets.into_iter().map(|t| TicketSummary {
        id: t.id.to_string(),
        ticket_number: t.ticket_number,
        subject: t.subject,
        status: t.status.as_str().to_string(),
        created_at: t.created_at.format("%Y-%m-%d %H:%M").to_string(),
        has_unread_reply: false, // Would need message tracking
    }).collect())
}

/// Initiate a voluntary exit
#[server(InitiateExit, "/api")]
pub async fn initiate_exit(
    request: InitiateExitRequest,
) -> Result<InitiateExitResponse, ServerFnError> {
    use actix_web::web::Data;
    use actix_web::HttpRequest;
    use leptos_actix::extract;
    use crate::server::app::ServiceFactory;
    use crate::server::extractors::auth::get_actor_id;
    use crate::server::repositories::{InitiateExitInput, ExitType};

    let factory: Data<ServiceFactory> = extract().await?;
    let http_req: HttpRequest = extract().await?;

    // Get authenticated user ID
    let user_id = get_actor_id(&http_req)
        .map_err(|e| ServerFnError::new(format!("Authentication required: {}", e)))?;

    let input = InitiateExitInput {
        user_id: user_id.clone(),
        license_id: request.license_id,
        exit_type: ExitType::Voluntary,
        exit_reason: request.exit_reason,
        exit_details: None,
        requested_by: user_id,
        requested_by_type: "user".to_string(),
    };

    match factory.exit_repository.initiate_exit(input).await {
        Ok(exit) => {
            // Generate confirmation number from exit ID
            let confirmation = format!("EXIT-{}", exit.id.to_string()[..8].to_uppercase());
            Ok(InitiateExitResponse {
                success: true,
                exit_id: Some(exit.id.to_string()),
                confirmation_number: Some(confirmation),
                error: None,
            })
        }
        Err(e) => Ok(InitiateExitResponse {
            success: false,
            exit_id: None,
            confirmation_number: None,
            error: Some(e.to_string()),
        }),
    }
}

/// Get existing exit request for user's license
#[server(GetExitStatus, "/api")]
pub async fn get_exit_status(license_id: String) -> Result<Option<InitiateExitResponse>, ServerFnError> {
    use actix_web::web::Data;
    use leptos_actix::extract;
    use crate::server::app::ServiceFactory;

    let factory: Data<ServiceFactory> = extract().await?;

    match factory.exit_repository.get_exit_by_license(&license_id).await {
        Ok(Some(exit)) => {
            let confirmation = format!("EXIT-{}", exit.id.to_string()[..8].to_uppercase());
            Ok(Some(InitiateExitResponse {
                success: true,
                exit_id: Some(exit.id.to_string()),
                confirmation_number: Some(confirmation),
                error: None,
            }))
        }
        Ok(None) => Ok(None),
        Err(e) => Err(ServerFnError::new(e.to_string())),
    }
}

/// Register support server functions explicitly (for debugging)
#[cfg(feature = "ssr")]
pub fn register_support_server_fns() {
    use server_fn::ServerFn;
    println!("Registering support server functions:");
    println!("  SubmitTicket: {}", SubmitTicket::url());
    println!("  GetUserTickets: {}", GetUserTickets::url());
    println!("  InitiateExit: {}", InitiateExit::url());
    println!("  GetExitStatus: {}", GetExitStatus::url());
}

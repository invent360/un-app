//! Support ticket handlers for Phase 7
//!
//! Endpoints for:
//! - Creating and managing support tickets
//! - Agent queue management
//! - Ticket messaging

use actix_web::{web, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::server::repositories::{
    DynSupportRepository, TicketCategory, TicketPriority, TicketStatus,
    CreateTicketInput, AddMessageInput, TicketFilters,
};
use crate::types::AppError;

// ============================================
// REQUEST/RESPONSE TYPES
// ============================================

#[derive(Debug, Deserialize)]
pub struct CreateTicketRequest {
    pub user_id: String,
    pub license_id: Option<String>,
    pub category: String,
    pub priority: Option<String>,
    pub subject: String,
    pub description: Option<String>,
    pub locale: Option<String>,
    pub country_code: Option<String>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct AddMessageRequest {
    pub sender_type: String,
    pub sender_id: String,
    pub message: String,
    pub is_internal: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct SearchTicketsRequest {
    pub user_id: Option<String>,
    pub agent_id: Option<Uuid>,
    pub status: Option<String>,
    pub category: Option<String>,
    pub priority: Option<String>,
    pub country_code: Option<String>,
    pub unassigned_only: Option<bool>,
    pub page: Option<i32>,
    pub page_size: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct AssignTicketRequest {
    pub agent_id: Uuid,
    pub assigned_by: String,
}

#[derive(Debug, Deserialize)]
pub struct EscalateTicketRequest {
    pub reason: String,
    pub escalated_by: String,
}

#[derive(Debug, Deserialize)]
pub struct ResolveTicketRequest {
    pub resolution_notes: String,
    pub resolved_by: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateStatusRequest {
    pub status: String,
    pub changed_by: String,
    pub changed_by_type: String,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct RecordSatisfactionRequest {
    pub score: i32,
    pub feedback: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct TicketListResponse<T> {
    pub items: Vec<T>,
    pub total: i64,
    pub page: i32,
    pub page_size: i32,
}

// ============================================
// PUBLIC ENDPOINTS (User-facing)
// ============================================

/// POST /api/v1/support/tickets
/// Create a new support ticket
pub async fn create_ticket(
    repo: web::Data<DynSupportRepository>,
    body: web::Json<CreateTicketRequest>,
) -> Result<HttpResponse, AppError> {
    let category = TicketCategory::from_str(&body.category)
        .ok_or_else(|| AppError::BadRequest(format!("Invalid category: {}", body.category)))?;

    let priority = body.priority
        .as_ref()
        .map(|p| TicketPriority::from_str(p))
        .flatten()
        .unwrap_or(TicketPriority::Normal);

    let input = CreateTicketInput {
        user_id: body.user_id.clone(),
        license_id: body.license_id.clone(),
        category,
        priority,
        subject: body.subject.clone(),
        description: body.description.clone(),
        locale: body.locale.clone(),
        country_code: body.country_code.clone(),
        tags: body.tags.clone().unwrap_or_default(),
    };

    let ticket = repo.create_ticket(input).await?;
    Ok(HttpResponse::Created().json(ticket))
}

/// GET /api/v1/support/tickets/{id}
/// Get ticket by ID
pub async fn get_ticket(
    repo: web::Data<DynSupportRepository>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();
    let ticket = repo.get_ticket(id).await?
        .ok_or_else(|| AppError::NotFound(format!("Ticket {} not found", id)))?;
    Ok(HttpResponse::Ok().json(ticket))
}

/// GET /api/v1/support/tickets/number/{ticket_number}
/// Get ticket by ticket number
pub async fn get_ticket_by_number(
    repo: web::Data<DynSupportRepository>,
    path: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let ticket_number = path.into_inner();
    let ticket = repo.get_ticket_by_number(&ticket_number).await?
        .ok_or_else(|| AppError::NotFound(format!("Ticket {} not found", ticket_number)))?;
    Ok(HttpResponse::Ok().json(ticket))
}

/// GET /api/v1/support/tickets/user/{user_id}
/// Get tickets for a user
pub async fn get_user_tickets(
    repo: web::Data<DynSupportRepository>,
    path: web::Path<String>,
    query: web::Query<SearchTicketsRequest>,
) -> Result<HttpResponse, AppError> {
    let user_id = path.into_inner();
    let page = query.page.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(20).min(100);
    let offset = (page - 1) * page_size;

    let filters = TicketFilters {
        user_id: Some(user_id.clone()),
        status: query.status.as_ref().and_then(|s| TicketStatus::from_str(s)),
        ..Default::default()
    };

    let tickets = repo.search_tickets(&filters, page_size, offset).await?;
    let total = repo.count_tickets(&filters).await?;

    Ok(HttpResponse::Ok().json(TicketListResponse {
        items: tickets,
        total,
        page,
        page_size,
    }))
}

/// POST /api/v1/support/tickets/{id}/messages
/// Add a message to a ticket
pub async fn add_message(
    repo: web::Data<DynSupportRepository>,
    path: web::Path<Uuid>,
    body: web::Json<AddMessageRequest>,
) -> Result<HttpResponse, AppError> {
    let ticket_id = path.into_inner();

    // Verify ticket exists
    repo.get_ticket(ticket_id).await?
        .ok_or_else(|| AppError::NotFound(format!("Ticket {} not found", ticket_id)))?;

    let input = AddMessageInput {
        ticket_id,
        sender_type: body.sender_type.clone(),
        sender_id: body.sender_id.clone(),
        message: body.message.clone(),
        is_internal: body.is_internal.unwrap_or(false),
        attachments: None,
    };

    let message = repo.add_message(input).await?;
    Ok(HttpResponse::Created().json(message))
}

/// GET /api/v1/support/tickets/{id}/messages
/// Get messages for a ticket
pub async fn get_messages(
    repo: web::Data<DynSupportRepository>,
    path: web::Path<Uuid>,
    _req: HttpRequest, // Used to determine if internal messages should be shown
) -> Result<HttpResponse, AppError> {
    let ticket_id = path.into_inner();

    // TODO: Check if requester is agent/admin to show internal messages
    let include_internal = false;

    let messages = repo.get_messages(ticket_id, include_internal).await?;
    Ok(HttpResponse::Ok().json(messages))
}

/// POST /api/v1/support/tickets/{id}/satisfaction
/// Record satisfaction feedback
pub async fn record_satisfaction(
    repo: web::Data<DynSupportRepository>,
    path: web::Path<Uuid>,
    body: web::Json<RecordSatisfactionRequest>,
) -> Result<HttpResponse, AppError> {
    let ticket_id = path.into_inner();

    if body.score < 1 || body.score > 5 {
        return Err(AppError::BadRequest("Score must be between 1 and 5".to_string()));
    }

    let ticket = repo.record_satisfaction(
        ticket_id,
        body.score,
        body.feedback.as_deref(),
    ).await?;

    Ok(HttpResponse::Ok().json(ticket))
}

// ============================================
// ADMIN ENDPOINTS (Agent/Support staff)
// ============================================

/// POST /api/v1/admin/support/tickets/search
/// Search tickets with filters
pub async fn search_tickets(
    repo: web::Data<DynSupportRepository>,
    body: web::Json<SearchTicketsRequest>,
) -> Result<HttpResponse, AppError> {
    let page = body.page.unwrap_or(1);
    let page_size = body.page_size.unwrap_or(20).min(100);
    let offset = (page - 1) * page_size;

    let filters = TicketFilters {
        user_id: body.user_id.clone(),
        agent_id: body.agent_id,
        status: body.status.as_ref().and_then(|s| TicketStatus::from_str(s)),
        category: body.category.as_ref().and_then(|c| TicketCategory::from_str(c)),
        priority: body.priority.as_ref().and_then(|p| TicketPriority::from_str(p)),
        country_code: body.country_code.clone(),
        unassigned_only: body.unassigned_only.unwrap_or(false),
    };

    let tickets = repo.search_tickets(&filters, page_size, offset).await?;
    let total = repo.count_tickets(&filters).await?;

    Ok(HttpResponse::Ok().json(TicketListResponse {
        items: tickets,
        total,
        page,
        page_size,
    }))
}

/// GET /api/v1/admin/support/queue/{agent_id}
/// Get agent's ticket queue
pub async fn get_agent_queue(
    repo: web::Data<DynSupportRepository>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let agent_id = path.into_inner();
    let tickets = repo.get_agent_queue(agent_id).await?;
    Ok(HttpResponse::Ok().json(tickets))
}

/// GET /api/v1/admin/support/unassigned
/// Get unassigned tickets
pub async fn get_unassigned_tickets(
    repo: web::Data<DynSupportRepository>,
    query: web::Query<SearchTicketsRequest>,
) -> Result<HttpResponse, AppError> {
    let category = query.category
        .as_ref()
        .and_then(|c| TicketCategory::from_str(c));
    let limit = query.page_size.unwrap_or(50).min(100);

    let tickets = repo.get_unassigned_tickets(
        category,
        query.country_code.as_deref(),
        limit,
    ).await?;

    Ok(HttpResponse::Ok().json(tickets))
}

/// POST /api/v1/admin/support/tickets/{id}/assign
/// Assign ticket to agent
pub async fn assign_ticket(
    repo: web::Data<DynSupportRepository>,
    path: web::Path<Uuid>,
    body: web::Json<AssignTicketRequest>,
) -> Result<HttpResponse, AppError> {
    let ticket_id = path.into_inner();
    let ticket = repo.assign_ticket(ticket_id, body.agent_id, &body.assigned_by).await?;
    Ok(HttpResponse::Ok().json(ticket))
}

/// POST /api/v1/admin/support/tickets/{id}/auto-assign
/// Auto-assign ticket to available agent
pub async fn auto_assign_ticket(
    repo: web::Data<DynSupportRepository>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let ticket_id = path.into_inner();
    let agent_id = repo.auto_assign_ticket(ticket_id).await?;

    match agent_id {
        Some(id) => Ok(HttpResponse::Ok().json(serde_json::json!({
            "assigned": true,
            "agent_id": id
        }))),
        None => Ok(HttpResponse::Ok().json(serde_json::json!({
            "assigned": false,
            "message": "No available agents"
        }))),
    }
}

/// POST /api/v1/admin/support/tickets/{id}/escalate
/// Escalate ticket
pub async fn escalate_ticket(
    repo: web::Data<DynSupportRepository>,
    path: web::Path<Uuid>,
    body: web::Json<EscalateTicketRequest>,
) -> Result<HttpResponse, AppError> {
    let ticket_id = path.into_inner();
    let ticket = repo.escalate_ticket(ticket_id, &body.reason, &body.escalated_by).await?;
    Ok(HttpResponse::Ok().json(ticket))
}

/// POST /api/v1/admin/support/tickets/{id}/resolve
/// Resolve ticket
pub async fn resolve_ticket(
    repo: web::Data<DynSupportRepository>,
    path: web::Path<Uuid>,
    body: web::Json<ResolveTicketRequest>,
) -> Result<HttpResponse, AppError> {
    let ticket_id = path.into_inner();
    let ticket = repo.resolve_ticket(ticket_id, &body.resolution_notes, &body.resolved_by).await?;
    Ok(HttpResponse::Ok().json(ticket))
}

/// POST /api/v1/admin/support/tickets/{id}/status
/// Update ticket status
pub async fn update_status(
    repo: web::Data<DynSupportRepository>,
    path: web::Path<Uuid>,
    body: web::Json<UpdateStatusRequest>,
) -> Result<HttpResponse, AppError> {
    let ticket_id = path.into_inner();
    let status = TicketStatus::from_str(&body.status)
        .ok_or_else(|| AppError::BadRequest(format!("Invalid status: {}", body.status)))?;

    let ticket = repo.update_status(
        ticket_id,
        status,
        &body.changed_by,
        &body.changed_by_type,
    ).await?;

    Ok(HttpResponse::Ok().json(ticket))
}

/// GET /api/v1/admin/support/tickets/{id}/history
/// Get ticket history
pub async fn get_ticket_history(
    repo: web::Data<DynSupportRepository>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let ticket_id = path.into_inner();
    let history = repo.get_history(ticket_id).await?;
    Ok(HttpResponse::Ok().json(history))
}

/// GET /api/v1/admin/support/stats
/// Get queue statistics
pub async fn get_queue_stats(
    repo: web::Data<DynSupportRepository>,
    query: web::Query<SearchTicketsRequest>,
) -> Result<HttpResponse, AppError> {
    let stats = repo.get_queue_stats(query.country_code.as_deref()).await?;
    Ok(HttpResponse::Ok().json(stats))
}

/// GET /api/v1/admin/support/canned-responses
/// Get canned responses
pub async fn get_canned_responses(
    repo: web::Data<DynSupportRepository>,
    query: web::Query<CannedResponsesQuery>,
) -> Result<HttpResponse, AppError> {
    let category = TicketCategory::from_str(&query.category)
        .ok_or_else(|| AppError::BadRequest(format!("Invalid category: {}", query.category)))?;

    let responses = repo.get_canned_responses(category, &query.locale).await?;
    Ok(HttpResponse::Ok().json(responses))
}

#[derive(Debug, Deserialize)]
pub struct CannedResponsesQuery {
    pub category: String,
    pub locale: String,
}

// ============================================
// ROUTE CONFIGURATION
// ============================================

/// Configure public support routes
pub fn configure_public_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/support")
            .route("/tickets", web::post().to(create_ticket))
            .route("/tickets/{id}", web::get().to(get_ticket))
            .route("/tickets/number/{ticket_number}", web::get().to(get_ticket_by_number))
            .route("/tickets/user/{user_id}", web::get().to(get_user_tickets))
            .route("/tickets/{id}/messages", web::post().to(add_message))
            .route("/tickets/{id}/messages", web::get().to(get_messages))
            .route("/tickets/{id}/satisfaction", web::post().to(record_satisfaction)),
    );
}

/// Configure admin support routes
pub fn configure_admin_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/support")
            .route("/tickets/search", web::post().to(search_tickets))
            .route("/queue/{agent_id}", web::get().to(get_agent_queue))
            .route("/unassigned", web::get().to(get_unassigned_tickets))
            .route("/tickets/{id}/assign", web::post().to(assign_ticket))
            .route("/tickets/{id}/auto-assign", web::post().to(auto_assign_ticket))
            .route("/tickets/{id}/escalate", web::post().to(escalate_ticket))
            .route("/tickets/{id}/resolve", web::post().to(resolve_ticket))
            .route("/tickets/{id}/status", web::post().to(update_status))
            .route("/tickets/{id}/history", web::get().to(get_ticket_history))
            .route("/stats", web::get().to(get_queue_stats))
            .route("/canned-responses", web::get().to(get_canned_responses)),
    );
}

//! Communication preferences handlers (Phase 8)
//!
//! Provides endpoints for managing user communication preferences,
//! message templates, suppressions, and scheduled messages.

use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::server::repositories::{
    DynCommunicationRepository, UpdatePreferencesInput, AddSuppressionInput,
    CreateTemplateInput, UpdateTemplateInput, ScheduleMessageInput, RegisterDeviceInput,
    CommChannel, MessageCategory, SuppressionType, TemplateStatus,
};
use crate::server::services::DynCommunicationService;
use crate::types::AppError;

// ============================================================================
// Request/Response Types
// ============================================================================

#[derive(Debug, Deserialize)]
pub struct GetPreferencesQuery {
    pub user_id: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePreferencesRequest {
    pub email_enabled: Option<bool>,
    pub push_enabled: Option<bool>,
    pub sms_enabled: Option<bool>,
    pub marketing_enabled: Option<bool>,
    pub transactional_enabled: Option<bool>,
    pub support_enabled: Option<bool>,
    pub cohort_reminders_enabled: Option<bool>,
    pub timezone: Option<String>,
    pub quiet_hours_start: Option<String>,
    pub quiet_hours_end: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct OptOutRequest {
    pub reason: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CheckSuppressionQuery {
    pub channel: String,
    pub identifier: String,
}

#[derive(Debug, Deserialize)]
pub struct AddSuppressionRequest {
    pub channel: String,
    pub identifier: String,
    pub suppression_type: String,
    pub source: Option<String>,
    pub reason: Option<String>,
    pub permanent: Option<bool>,
    pub expires_in_days: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct ListTemplatesQuery {
    pub channel: Option<String>,
    pub category: Option<String>,
    pub status: Option<String>,
    pub locale: Option<String>,
    pub limit: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct CreateTemplateRequest {
    pub template_code: String,
    pub channel: String,
    pub locale: Option<String>,
    pub subject: Option<String>,
    pub title: Option<String>,
    pub body_template: String,
    pub html_template: Option<String>,
    pub action_url: Option<String>,
    pub category: String,
    pub description: Option<String>,
    pub variables: Option<Vec<String>>,
    pub sample_data: Option<serde_json::Value>,
    pub created_by: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTemplateRequest {
    pub subject: Option<String>,
    pub body_template: Option<String>,
    pub html_template: Option<String>,
    pub variables: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct ApproveTemplateRequest {
    pub approved_by: String,
}

#[derive(Debug, Deserialize)]
pub struct ScheduleMessageRequest {
    pub user_id: String,
    pub channel: String,
    pub template_code: Option<String>,
    pub subject: Option<String>,
    pub body: String,
    pub scheduled_at: chrono::DateTime<chrono::Utc>,
    pub local_time_preference: Option<bool>,
    pub variables: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct SendImmediateRequest {
    pub user_id: String,
    pub channel: String,
    pub template_code: Option<String>,
    pub subject: Option<String>,
    pub body: Option<String>,
    pub variables: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct RegisterDeviceRequest {
    pub user_id: String,
    pub device_token: String,
    pub platform: String,
    pub device_name: Option<String>,
    pub app_version: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ListScheduledQuery {
    pub user_id: Option<String>,
    pub status: Option<String>,
    pub limit: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct ListSuppressionsQuery {
    pub channel: Option<String>,
    pub permanent: Option<bool>,
    pub limit: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct PreferencesResponse {
    pub id: Uuid,
    pub user_id: String,
    pub email_enabled: bool,
    pub push_enabled: bool,
    pub sms_enabled: bool,
    pub marketing_enabled: bool,
    pub transactional_enabled: bool,
    pub support_enabled: bool,
    pub cohort_reminders_enabled: bool,
    pub timezone: String,
    pub quiet_hours_start: Option<String>,
    pub quiet_hours_end: Option<String>,
    pub global_opt_out: bool,
    pub opt_out_reason: Option<String>,
    pub opted_out_at: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize)]
pub struct SuppressionResponse {
    pub id: Uuid,
    pub channel: String,
    pub identifier: String,
    pub suppression_type: String,
    pub source: String,
    pub reason: Option<String>,
    pub permanent: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Serialize)]
pub struct TemplateResponse {
    pub id: Uuid,
    pub template_code: String,
    pub channel: String,
    pub locale: String,
    pub subject: Option<String>,
    pub body_template: String,
    pub html_template: Option<String>,
    pub category: String,
    pub variables: Vec<String>,
    pub status: String,
    pub approved_by: Option<String>,
    pub approved_at: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize)]
pub struct ScheduledMessageResponse {
    pub id: Uuid,
    pub user_id: String,
    pub channel: String,
    pub template_id: Option<Uuid>,
    pub scheduled_at: chrono::DateTime<chrono::Utc>,
    pub local_time_preference: bool,
    pub subject: Option<String>,
    pub body: String,
    pub status: String,
    pub sent_at: Option<chrono::DateTime<chrono::Utc>>,
    pub external_id: Option<String>,
    pub error_message: Option<String>,
    pub attempt_count: i32,
    pub max_attempts: i32,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize)]
pub struct DeviceTokenResponse {
    pub id: Uuid,
    pub user_id: String,
    pub device_token: String,
    pub platform: String,
    pub device_name: Option<String>,
    pub app_version: Option<String>,
    pub active: bool,
    pub last_used_at: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize)]
pub struct SendResultResponse {
    pub success: bool,
    pub external_id: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct SendStatsResponse {
    pub scheduled: i64,
    pub sent: i64,
    pub failed: i64,
    pub cancelled: i64,
}

#[derive(Debug, Serialize)]
pub struct CanReceiveResponse {
    pub can_receive: bool,
    pub reason: Option<String>,
}

// ============================================================================
// User Handlers
// ============================================================================

/// GET /api/v1/communication/preferences
/// Get communication preferences for current user
pub async fn get_preferences(
    service: web::Data<DynCommunicationService>,
    query: web::Query<GetPreferencesQuery>,
) -> Result<impl Responder, AppError> {
    let prefs = service.get_preferences(&query.user_id).await?;

    Ok(HttpResponse::Ok().json(PreferencesResponse {
        id: prefs.id,
        user_id: prefs.user_id,
        email_enabled: prefs.email_enabled,
        push_enabled: prefs.push_enabled,
        sms_enabled: prefs.sms_enabled,
        marketing_enabled: prefs.marketing_enabled,
        transactional_enabled: prefs.transactional_enabled,
        support_enabled: prefs.support_enabled,
        cohort_reminders_enabled: prefs.cohort_reminders_enabled,
        timezone: prefs.timezone,
        quiet_hours_start: prefs.quiet_hours_start.map(|t| t.to_string()),
        quiet_hours_end: prefs.quiet_hours_end.map(|t| t.to_string()),
        global_opt_out: prefs.global_opt_out,
        opt_out_reason: prefs.opt_out_reason,
        opted_out_at: prefs.opted_out_at,
        created_at: prefs.created_at,
        updated_at: prefs.updated_at,
    }))
}

/// PUT /api/v1/communication/preferences
/// Update communication preferences
pub async fn update_preferences(
    service: web::Data<DynCommunicationService>,
    query: web::Query<GetPreferencesQuery>,
    body: web::Json<UpdatePreferencesRequest>,
) -> Result<impl Responder, AppError> {
    // Parse quiet hours if provided
    let quiet_hours_start = body.quiet_hours_start.as_ref().and_then(|s| {
        chrono::NaiveTime::parse_from_str(s, "%H:%M:%S")
            .or_else(|_| chrono::NaiveTime::parse_from_str(s, "%H:%M"))
            .ok()
    });
    let quiet_hours_end = body.quiet_hours_end.as_ref().and_then(|s| {
        chrono::NaiveTime::parse_from_str(s, "%H:%M:%S")
            .or_else(|_| chrono::NaiveTime::parse_from_str(s, "%H:%M"))
            .ok()
    });

    let prefs = service
        .update_preferences(
            &query.user_id,
            UpdatePreferencesInput {
                email_enabled: body.email_enabled,
                push_enabled: body.push_enabled,
                sms_enabled: body.sms_enabled,
                in_app_enabled: None,
                marketing_enabled: body.marketing_enabled,
                support_enabled: body.support_enabled,
                cohort_reminders_enabled: body.cohort_reminders_enabled,
                digest_frequency: None,
                max_messages_per_day: None,
                timezone: body.timezone.clone(),
                quiet_hours_enabled: None,
                quiet_hours_start,
                quiet_hours_end,
                preferred_locale: None,
                notification_email: None,
                notification_phone: None,
            },
        )
        .await?;

    Ok(HttpResponse::Ok().json(PreferencesResponse {
        id: prefs.id,
        user_id: prefs.user_id,
        email_enabled: prefs.email_enabled,
        push_enabled: prefs.push_enabled,
        sms_enabled: prefs.sms_enabled,
        marketing_enabled: prefs.marketing_enabled,
        transactional_enabled: prefs.transactional_enabled,
        support_enabled: prefs.support_enabled,
        cohort_reminders_enabled: prefs.cohort_reminders_enabled,
        timezone: prefs.timezone,
        quiet_hours_start: prefs.quiet_hours_start.map(|t| t.to_string()),
        quiet_hours_end: prefs.quiet_hours_end.map(|t| t.to_string()),
        global_opt_out: prefs.global_opt_out,
        opt_out_reason: prefs.opt_out_reason,
        opted_out_at: prefs.opted_out_at,
        created_at: prefs.created_at,
        updated_at: prefs.updated_at,
    }))
}

/// POST /api/v1/communication/opt-out
/// Global opt-out
pub async fn opt_out(
    service: web::Data<DynCommunicationService>,
    query: web::Query<GetPreferencesQuery>,
    body: web::Json<OptOutRequest>,
) -> Result<impl Responder, AppError> {
    service
        .opt_out(&query.user_id, body.reason.as_deref())
        .await?;
    Ok(HttpResponse::NoContent().finish())
}

/// POST /api/v1/communication/devices
/// Register a device for push notifications
pub async fn register_device(
    service: web::Data<DynCommunicationService>,
    body: web::Json<RegisterDeviceRequest>,
) -> Result<impl Responder, AppError> {
    let device = service
        .register_device(RegisterDeviceInput {
            user_id: body.user_id.clone(),
            token: body.device_token.clone(),
            platform: body.platform.clone(),
            platform_version: None,
            device_id: None,
            device_name: body.device_name.clone(),
            app_version: body.app_version.clone(),
        })
        .await?;

    Ok(HttpResponse::Created().json(DeviceTokenResponse {
        id: device.id,
        user_id: device.user_id,
        device_token: device.token,
        platform: device.platform,
        device_name: device.device_name,
        app_version: device.app_version,
        active: device.active,
        last_used_at: device.last_used_at,
        created_at: device.created_at,
    }))
}

// ============================================================================
// Admin Handlers
// ============================================================================

/// GET /api/v1/admin/communication/templates
/// List message templates
pub async fn list_templates(
    service: web::Data<DynCommunicationService>,
    query: web::Query<ListTemplatesQuery>,
) -> Result<impl Responder, AppError> {
    let channel = query.channel.as_ref().and_then(|c| parse_channel(c));
    let category = query.category.as_ref().and_then(|c| parse_category(c));
    // Note: status and locale filtering is done client-side (service only supports channel + category)

    let templates = service
        .list_templates(channel, category)
        .await?;

    let response: Vec<TemplateResponse> = templates
        .into_iter()
        .map(|t| TemplateResponse {
            id: t.id,
            template_code: t.template_code,
            channel: format!("{:?}", t.channel).to_lowercase(),
            locale: t.locale,
            subject: t.subject,
            body_template: t.body_template,
            html_template: t.html_template,
            category: format!("{:?}", t.category).to_lowercase(),
            variables: t.variables,
            status: format!("{:?}", t.status).to_lowercase(),
            approved_by: t.approved_by,
            approved_at: t.approved_at,
            created_at: t.created_at,
            updated_at: t.updated_at,
        })
        .collect();

    Ok(HttpResponse::Ok().json(response))
}

/// POST /api/v1/admin/communication/templates
/// Create a message template
pub async fn create_template(
    service: web::Data<DynCommunicationService>,
    body: web::Json<CreateTemplateRequest>,
) -> Result<impl Responder, AppError> {
    let channel = parse_channel(&body.channel).ok_or_else(|| {
        AppError::BadRequest("Invalid channel".to_string())
    })?;
    let category = parse_category(&body.category).ok_or_else(|| {
        AppError::BadRequest("Invalid category".to_string())
    })?;

    let template = service
        .create_template(CreateTemplateInput {
            template_code: body.template_code.clone(),
            channel,
            locale: body.locale.clone().unwrap_or_else(|| "en".to_string()),
            subject: body.subject.clone(),
            title: body.title.clone(),
            body_template: body.body_template.clone(),
            html_template: body.html_template.clone(),
            action_url: body.action_url.clone(),
            template_engine: "handlebars".to_string(),
            category,
            description: body.description.clone(),
            variables: body.variables.clone().unwrap_or_default(),
            sample_data: body.sample_data.clone(),
            created_by: body.created_by.clone().unwrap_or_else(|| "system".to_string()),
        })
        .await?;

    Ok(HttpResponse::Created().json(TemplateResponse {
        id: template.id,
        template_code: template.template_code,
        channel: format!("{:?}", template.channel).to_lowercase(),
        locale: template.locale,
        subject: template.subject,
        body_template: template.body_template,
        html_template: template.html_template,
        category: format!("{:?}", template.category).to_lowercase(),
        variables: template.variables,
        status: format!("{:?}", template.status).to_lowercase(),
        approved_by: template.approved_by,
        approved_at: template.approved_at,
        created_at: template.created_at,
        updated_at: template.updated_at,
    }))
}

// NOTE: get_template_by_id not in service trait - use get_template(code, channel, locale) instead

/// PUT /api/v1/admin/communication/templates/{id}
/// Update a template
pub async fn update_template(
    service: web::Data<DynCommunicationService>,
    path: web::Path<Uuid>,
    body: web::Json<UpdateTemplateRequest>,
) -> Result<impl Responder, AppError> {
    let id = path.into_inner();
    let template = service
        .update_template(
            id,
            UpdateTemplateInput {
                subject: body.subject.clone(),
                title: None,
                body_template: body.body_template.clone(),
                html_template: body.html_template.clone(),
                action_url: None,
                description: None,
                variables: body.variables.clone(),
                sample_data: None,
                updated_by: "admin".to_string(),
            },
        )
        .await?;

    Ok(HttpResponse::Ok().json(TemplateResponse {
        id: template.id,
        template_code: template.template_code,
        channel: format!("{:?}", template.channel).to_lowercase(),
        locale: template.locale,
        subject: template.subject,
        body_template: template.body_template,
        html_template: template.html_template,
        category: format!("{:?}", template.category).to_lowercase(),
        variables: template.variables,
        status: format!("{:?}", template.status).to_lowercase(),
        approved_by: template.approved_by,
        approved_at: template.approved_at,
        created_at: template.created_at,
        updated_at: template.updated_at,
    }))
}

/// POST /api/v1/admin/communication/templates/{id}/approve
/// Approve a template
pub async fn approve_template(
    service: web::Data<DynCommunicationService>,
    path: web::Path<Uuid>,
    body: web::Json<ApproveTemplateRequest>,
) -> Result<impl Responder, AppError> {
    let id = path.into_inner();
    let template = service.approve_template(id, &body.approved_by).await?;

    Ok(HttpResponse::Ok().json(TemplateResponse {
        id: template.id,
        template_code: template.template_code,
        channel: format!("{:?}", template.channel).to_lowercase(),
        locale: template.locale,
        subject: template.subject,
        body_template: template.body_template,
        html_template: template.html_template,
        category: format!("{:?}", template.category).to_lowercase(),
        variables: template.variables,
        status: format!("{:?}", template.status).to_lowercase(),
        approved_by: template.approved_by,
        approved_at: template.approved_at,
        created_at: template.created_at,
        updated_at: template.updated_at,
    }))
}

// NOTE: archive_template not in service trait - templates can be rejected instead
// /// DELETE /api/v1/admin/communication/templates/{id}
// /// Archive a template
// pub async fn archive_template(
//     service: web::Data<DynCommunicationService>,
//     path: web::Path<Uuid>,
// ) -> Result<impl Responder, AppError> {
//     let id = path.into_inner();
//     service.archive_template(id).await?;
//     Ok(HttpResponse::NoContent().finish())
// }

/// GET /api/v1/admin/communication/suppressions
/// List suppressions
pub async fn list_suppressions(
    service: web::Data<DynCommunicationService>,
    query: web::Query<ListSuppressionsQuery>,
) -> Result<impl Responder, AppError> {
    let channel = query.channel.as_ref().and_then(|c| parse_channel(c));
    // Note: permanent filtering is done client-side (service only supports channel + limit)
    let suppressions = service
        .list_suppressions(channel, query.limit.unwrap_or(100))
        .await?;

    let response: Vec<SuppressionResponse> = suppressions
        .into_iter()
        .map(|s| SuppressionResponse {
            id: s.id,
            channel: format!("{:?}", s.channel).to_lowercase(),
            identifier: s.identifier,
            suppression_type: format!("{:?}", s.suppression_type).to_lowercase(),
            source: s.source,
            reason: s.reason,
            permanent: s.permanent,
            created_at: s.created_at,
            expires_at: s.expires_at,
        })
        .collect();

    Ok(HttpResponse::Ok().json(response))
}

/// POST /api/v1/admin/communication/suppressions
/// Add a suppression
pub async fn add_suppression(
    service: web::Data<DynCommunicationService>,
    body: web::Json<AddSuppressionRequest>,
) -> Result<impl Responder, AppError> {
    let channel = parse_channel(&body.channel).ok_or_else(|| {
        AppError::BadRequest("Invalid channel".to_string())
    })?;
    let suppression_type = parse_suppression_type(&body.suppression_type).ok_or_else(|| {
        AppError::BadRequest("Invalid suppression type".to_string())
    })?;

    let expires_at = body.expires_in_days.map(|days| {
        chrono::Utc::now() + chrono::Duration::days(days as i64)
    });

    let suppression = service
        .add_suppression(AddSuppressionInput {
            channel,
            identifier: body.identifier.clone(),
            suppression_type,
            source: body.source.clone().unwrap_or_else(|| "manual".to_string()),
            reason: body.reason.clone(),
            bounce_type: None,
            complaint_type: None,
            permanent: body.permanent.unwrap_or(false),
            user_id: None,
            expires_at,
        })
        .await?;

    Ok(HttpResponse::Created().json(SuppressionResponse {
        id: suppression.id,
        channel: format!("{:?}", suppression.channel).to_lowercase(),
        identifier: suppression.identifier,
        suppression_type: format!("{:?}", suppression.suppression_type).to_lowercase(),
        source: suppression.source,
        reason: suppression.reason,
        permanent: suppression.permanent,
        created_at: suppression.created_at,
        expires_at: suppression.expires_at,
    }))
}

/// DELETE /api/v1/admin/communication/suppressions
/// Remove a suppression by channel and identifier
pub async fn remove_suppression(
    service: web::Data<DynCommunicationService>,
    query: web::Query<CheckSuppressionQuery>,
) -> Result<impl Responder, AppError> {
    let channel = parse_channel(&query.channel).ok_or_else(|| {
        AppError::BadRequest("Invalid channel".to_string())
    })?;
    service.remove_suppression(channel, &query.identifier).await?;
    Ok(HttpResponse::NoContent().finish())
}

/// GET /api/v1/admin/communication/suppressions/check
/// Check if an identifier is suppressed
pub async fn check_suppression(
    service: web::Data<DynCommunicationService>,
    query: web::Query<CheckSuppressionQuery>,
) -> Result<impl Responder, AppError> {
    let channel = parse_channel(&query.channel).ok_or_else(|| {
        AppError::BadRequest("Invalid channel".to_string())
    })?;

    let suppressed = service
        .is_suppressed(channel, &query.identifier)
        .await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({ "suppressed": suppressed })))
}

/// POST /api/v1/admin/communication/send
/// Send an immediate message
pub async fn send_immediate(
    service: web::Data<DynCommunicationService>,
    body: web::Json<SendImmediateRequest>,
) -> Result<impl Responder, AppError> {
    let channel = parse_channel(&body.channel).ok_or_else(|| {
        AppError::BadRequest("Invalid channel".to_string())
    })?;

    // Default category to transactional for immediate sends
    let category = MessageCategory::Transactional;

    // Get recipient from user preferences (would need to be looked up in practice)
    // For now, use user_id as recipient placeholder
    let recipient = &body.user_id;

    let message_body = body.body.as_deref().unwrap_or("");

    let result = service
        .send_immediate(
            &body.user_id,
            channel,
            category,
            recipient,
            body.subject.as_deref(),
            message_body,
            None, // html
        )
        .await?;

    Ok(HttpResponse::Ok().json(SendResultResponse {
        success: result.success,
        external_id: result.message_id,
        error: result.error,
    }))
}

/// GET /api/v1/admin/communication/scheduled
/// List scheduled messages for a user
pub async fn list_scheduled(
    service: web::Data<DynCommunicationService>,
    query: web::Query<ListScheduledQuery>,
) -> Result<impl Responder, AppError> {
    // Service only supports fetching by user_id
    let user_id = query.user_id.as_deref().ok_or_else(|| {
        AppError::BadRequest("user_id is required".to_string())
    })?;

    let messages = service
        .get_user_messages(user_id, query.limit.unwrap_or(50))
        .await?;

    let response: Vec<ScheduledMessageResponse> = messages
        .into_iter()
        .map(|m| ScheduledMessageResponse {
            id: m.id,
            user_id: m.user_id,
            channel: format!("{:?}", m.channel).to_lowercase(),
            template_id: m.template_id,
            scheduled_at: m.scheduled_at,
            local_time_preference: m.local_time_preference,
            subject: m.subject,
            body: m.body,
            status: format!("{:?}", m.status).to_lowercase(),
            sent_at: m.sent_at,
            external_id: m.external_id,
            error_message: m.error_message,
            attempt_count: m.attempt_count,
            max_attempts: m.max_attempts,
            created_at: m.created_at,
        })
        .collect();

    Ok(HttpResponse::Ok().json(response))
}

/// POST /api/v1/admin/communication/scheduled
/// Schedule a message
pub async fn schedule_message(
    service: web::Data<DynCommunicationService>,
    body: web::Json<ScheduleMessageRequest>,
) -> Result<impl Responder, AppError> {
    let channel = parse_channel(&body.channel).ok_or_else(|| {
        AppError::BadRequest("Invalid channel".to_string())
    })?;

    let message = service
        .schedule_message(ScheduleMessageInput {
            user_id: body.user_id.clone(),
            channel,
            category: MessageCategory::Transactional,
            recipient_address: body.user_id.clone(), // Use user_id as recipient placeholder
            recipient_name: None,
            subject: body.subject.clone(),
            title: None,
            body: body.body.clone(),
            html_body: None,
            action_url: None,
            template_id: None,
            template_code: body.template_code.clone(),
            template_data: body.variables.clone(),
            scheduled_at: body.scheduled_at,
            local_time_preference: body.local_time_preference.unwrap_or(true),
            priority: 0,
            correlation_id: None,
            triggered_by: None,
            entity_type: None,
            entity_id: None,
        })
        .await?;

    Ok(HttpResponse::Created().json(ScheduledMessageResponse {
        id: message.id,
        user_id: message.user_id,
        channel: format!("{:?}", message.channel).to_lowercase(),
        template_id: message.template_id,
        scheduled_at: message.scheduled_at,
        local_time_preference: message.local_time_preference,
        subject: message.subject,
        body: message.body,
        status: format!("{:?}", message.status).to_lowercase(),
        sent_at: message.sent_at,
        external_id: message.external_id,
        error_message: message.error_message,
        attempt_count: message.attempt_count,
        max_attempts: message.max_attempts,
        created_at: message.created_at,
    }))
}

/// DELETE /api/v1/admin/communication/scheduled/{id}
/// Cancel a scheduled message
pub async fn cancel_scheduled(
    service: web::Data<DynCommunicationService>,
    path: web::Path<Uuid>,
) -> Result<impl Responder, AppError> {
    let id = path.into_inner();
    service.cancel_message(id).await?;
    Ok(HttpResponse::NoContent().finish())
}

/// POST /api/v1/admin/communication/process
/// Process pending scheduled messages
pub async fn process_scheduled(
    service: web::Data<DynCommunicationService>,
) -> Result<impl Responder, AppError> {
    let stats = service.process_scheduled_messages(100).await?;

    Ok(HttpResponse::Ok().json(SendStatsResponse {
        scheduled: stats.scheduled,
        sent: stats.sent,
        failed: stats.failed,
        cancelled: stats.cancelled,
    }))
}

/// GET /api/v1/admin/communication/can-receive
/// Check if a user can receive a message
pub async fn can_receive(
    service: web::Data<DynCommunicationService>,
    query: web::Query<GetPreferencesQuery>,
    channel_query: web::Query<CheckSuppressionQuery>,
) -> Result<impl Responder, AppError> {
    let channel = parse_channel(&channel_query.channel).ok_or_else(|| {
        AppError::BadRequest("Invalid channel".to_string())
    })?;
    let category = parse_category(&channel_query.identifier).unwrap_or(MessageCategory::Transactional);

    // Recipient address is the identifier for suppression check purposes
    let recipient = &channel_query.identifier;

    let result = service
        .can_receive(&query.user_id, channel, category, recipient)
        .await?;

    Ok(HttpResponse::Ok().json(CanReceiveResponse {
        can_receive: result.can_send,
        reason: result.reason,
    }))
}

// ============================================================================
// Helper Functions
// ============================================================================

fn parse_channel(s: &str) -> Option<CommChannel> {
    match s.to_lowercase().as_str() {
        "email" => Some(CommChannel::Email),
        "push" => Some(CommChannel::Push),
        "sms" => Some(CommChannel::Sms),
        _ => None,
    }
}

fn parse_category(s: &str) -> Option<MessageCategory> {
    match s.to_lowercase().as_str() {
        "transactional" => Some(MessageCategory::Transactional),
        "marketing" => Some(MessageCategory::Marketing),
        "support" => Some(MessageCategory::Support),
        "cohort" => Some(MessageCategory::Cohort),
        _ => None,
    }
}

fn parse_status(s: &str) -> Option<TemplateStatus> {
    match s.to_lowercase().as_str() {
        "draft" => Some(TemplateStatus::Draft),
        "approved" => Some(TemplateStatus::Approved),
        "archived" => Some(TemplateStatus::Archived),
        _ => None,
    }
}

fn parse_suppression_type(s: &str) -> Option<SuppressionType> {
    match s.to_lowercase().as_str() {
        "bounce" => Some(SuppressionType::Bounce),
        "complaint" => Some(SuppressionType::Complaint),
        "unsubscribe" => Some(SuppressionType::Unsubscribe),
        "manual" => Some(SuppressionType::Manual),
        _ => None,
    }
}

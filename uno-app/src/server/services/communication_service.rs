//! Communication service for user messaging
//!
//! Provides communication capabilities with:
//! - User preference management
//! - Suppression list handling
//! - Message template rendering
//! - Scheduled message delivery
//! - Multi-channel support (email, push, SMS, in-app)

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use std::sync::Arc;
use uuid::Uuid;
use regex::Regex;

use crate::server::repositories::{
    DynCommunicationRepository, CommunicationRepository,
    CommunicationPreferences, CommunicationSuppression, MessageTemplate, ScheduledMessage, DeviceToken,
    CommChannel, MessageCategory, SuppressionType, TemplateStatus,
    UpdatePreferencesInput, AddSuppressionInput, CreateTemplateInput, UpdateTemplateInput,
    ScheduleMessageInput, RegisterDeviceInput, CanReceiveResult, SendStats,
};
use crate::types::AppError;

// ============================================
// EMAIL/PUSH PROVIDER ABSTRACTION
// ============================================

/// Email send result
#[derive(Debug, Clone)]
pub struct EmailSendResult {
    pub success: bool,
    pub message_id: Option<String>,
    pub error: Option<String>,
}

/// Push send result
#[derive(Debug, Clone)]
pub struct PushSendResult {
    pub success: bool,
    pub message_id: Option<String>,
    pub error: Option<String>,
}

/// Email provider trait
#[async_trait]
pub trait EmailProvider: Send + Sync {
    async fn send_email(
        &self,
        to: &str,
        subject: &str,
        body: &str,
        html: Option<&str>,
    ) -> Result<EmailSendResult, AppError>;
}

/// Push notification provider trait
#[async_trait]
pub trait PushProvider: Send + Sync {
    async fn send_push(
        &self,
        device_token: &str,
        platform: &str,
        title: &str,
        body: &str,
        data: Option<serde_json::Value>,
    ) -> Result<PushSendResult, AppError>;
}

/// Null email provider (for testing/disabled state)
pub struct NullEmailProvider;

#[async_trait]
impl EmailProvider for NullEmailProvider {
    async fn send_email(
        &self,
        _to: &str,
        _subject: &str,
        _body: &str,
        _html: Option<&str>,
    ) -> Result<EmailSendResult, AppError> {
        Ok(EmailSendResult {
            success: true,
            message_id: Some(format!("null-{}", Uuid::new_v4())),
            error: None,
        })
    }
}

/// Null push provider (for testing/disabled state)
pub struct NullPushProvider;

#[async_trait]
impl PushProvider for NullPushProvider {
    async fn send_push(
        &self,
        _device_token: &str,
        _platform: &str,
        _title: &str,
        _body: &str,
        _data: Option<serde_json::Value>,
    ) -> Result<PushSendResult, AppError> {
        Ok(PushSendResult {
            success: true,
            message_id: Some(format!("null-{}", Uuid::new_v4())),
            error: None,
        })
    }
}

// ============================================
// SERVICE TRAIT
// ============================================

/// Dynamic type alias for CommunicationService trait object
pub type DynCommunicationService = Arc<dyn CommunicationService + Send + Sync>;

/// Send message result
#[derive(Debug, Clone)]
pub struct SendMessageResult {
    pub success: bool,
    pub channel: CommChannel,
    pub message_id: Option<String>,
    pub error: Option<String>,
}

/// Rendered message content
#[derive(Debug, Clone)]
pub struct RenderedMessage {
    pub subject: Option<String>,
    pub title: Option<String>,
    pub body: String,
    pub html_body: Option<String>,
}

/// Communication service trait
#[async_trait]
pub trait CommunicationService: Send + Sync {
    // --- Preferences ---
    async fn get_preferences(&self, user_id: &str) -> Result<CommunicationPreferences, AppError>;
    async fn update_preferences(&self, user_id: &str, input: UpdatePreferencesInput) -> Result<CommunicationPreferences, AppError>;
    async fn opt_out(&self, user_id: &str, reason: Option<&str>) -> Result<CommunicationPreferences, AppError>;
    async fn can_receive(&self, user_id: &str, channel: CommChannel, category: MessageCategory, recipient: &str) -> Result<CanReceiveResult, AppError>;

    // --- Suppressions ---
    async fn add_suppression(&self, input: AddSuppressionInput) -> Result<CommunicationSuppression, AppError>;
    async fn is_suppressed(&self, channel: CommChannel, identifier: &str) -> Result<bool, AppError>;
    async fn remove_suppression(&self, channel: CommChannel, identifier: &str) -> Result<(), AppError>;
    async fn list_suppressions(&self, channel: Option<CommChannel>, limit: i32) -> Result<Vec<CommunicationSuppression>, AppError>;
    async fn process_bounce(&self, channel: CommChannel, identifier: &str, bounce_type: &str, source: &str) -> Result<CommunicationSuppression, AppError>;
    async fn process_complaint(&self, channel: CommChannel, identifier: &str, complaint_type: &str, source: &str) -> Result<CommunicationSuppression, AppError>;

    // --- Templates ---
    async fn create_template(&self, input: CreateTemplateInput) -> Result<MessageTemplate, AppError>;
    async fn get_template(&self, code: &str, channel: CommChannel, locale: &str) -> Result<Option<MessageTemplate>, AppError>;
    async fn list_templates(&self, channel: Option<CommChannel>, category: Option<MessageCategory>) -> Result<Vec<MessageTemplate>, AppError>;
    async fn update_template(&self, id: Uuid, input: UpdateTemplateInput) -> Result<MessageTemplate, AppError>;
    async fn submit_template_for_approval(&self, id: Uuid) -> Result<MessageTemplate, AppError>;
    async fn approve_template(&self, id: Uuid, approved_by: &str) -> Result<MessageTemplate, AppError>;
    async fn reject_template(&self, id: Uuid, reason: &str) -> Result<MessageTemplate, AppError>;
    async fn render_template(&self, template: &MessageTemplate, data: &serde_json::Value) -> Result<RenderedMessage, AppError>;

    // --- Sending ---
    async fn schedule_message(&self, input: ScheduleMessageInput) -> Result<ScheduledMessage, AppError>;
    async fn send_immediate(&self, user_id: &str, channel: CommChannel, category: MessageCategory,
                            recipient: &str, subject: Option<&str>, body: &str, html: Option<&str>) -> Result<SendMessageResult, AppError>;
    async fn process_scheduled_messages(&self, limit: i32) -> Result<SendStats, AppError>;
    async fn cancel_message(&self, id: Uuid) -> Result<ScheduledMessage, AppError>;
    async fn get_user_messages(&self, user_id: &str, limit: i32) -> Result<Vec<ScheduledMessage>, AppError>;

    // --- Device Tokens ---
    async fn register_device(&self, input: RegisterDeviceInput) -> Result<DeviceToken, AppError>;
    async fn get_user_devices(&self, user_id: &str) -> Result<Vec<DeviceToken>, AppError>;
    async fn invalidate_device(&self, token_hash: &str, reason: &str) -> Result<(), AppError>;

    // --- Stats ---
    async fn get_send_stats(&self) -> Result<SendStats, AppError>;
}

// ============================================
// SERVICE IMPLEMENTATION
// ============================================

pub struct CommunicationServiceImpl<E: EmailProvider = NullEmailProvider, P: PushProvider = NullPushProvider> {
    repo: DynCommunicationRepository,
    email_provider: Arc<E>,
    push_provider: Arc<P>,
}

impl CommunicationServiceImpl<NullEmailProvider, NullPushProvider> {
    pub fn new(repo: DynCommunicationRepository) -> Self {
        Self {
            repo,
            email_provider: Arc::new(NullEmailProvider),
            push_provider: Arc::new(NullPushProvider),
        }
    }
}

impl<E: EmailProvider, P: PushProvider> CommunicationServiceImpl<E, P> {
    pub fn with_providers(repo: DynCommunicationRepository, email: E, push: P) -> Self {
        Self {
            repo,
            email_provider: Arc::new(email),
            push_provider: Arc::new(push),
        }
    }

    /// Validate template syntax (simple {{variable}} format)
    fn validate_template_syntax(template: &str) -> Result<(), AppError> {
        // Check for unmatched braces
        let open_count = template.matches("{{").count();
        let close_count = template.matches("}}").count();
        if open_count != close_count {
            return Err(AppError::ValidationError(
                "Unmatched template braces: {{ and }} counts don't match".to_string()
            ));
        }
        Ok(())
    }

    /// Render a template with simple {{variable}} substitution
    fn render_template(&self, template: &str, data: &serde_json::Value) -> Result<String, AppError> {
        let re = Regex::new(r"\{\{([^}]+)\}\}")
            .map_err(|e| AppError::InternalServerError(format!("Regex error: {}", e)))?;

        let result = re.replace_all(template, |caps: &regex::Captures| {
            let key = caps.get(1).map_or("", |m| m.as_str().trim());
            match data.get(key) {
                Some(serde_json::Value::String(s)) => s.clone(),
                Some(serde_json::Value::Number(n)) => n.to_string(),
                Some(serde_json::Value::Bool(b)) => b.to_string(),
                Some(v) => v.to_string(),
                None => format!("{{{{{}}}}}", key), // Keep original if not found
            }
        });

        Ok(result.to_string())
    }
}

#[async_trait]
impl<E: EmailProvider + 'static, P: PushProvider + 'static> CommunicationService for CommunicationServiceImpl<E, P> {
    async fn get_preferences(&self, user_id: &str) -> Result<CommunicationPreferences, AppError> {
        self.repo.get_or_create_preferences(user_id).await
    }

    async fn update_preferences(&self, user_id: &str, input: UpdatePreferencesInput) -> Result<CommunicationPreferences, AppError> {
        // Ensure preferences exist first
        self.repo.get_or_create_preferences(user_id).await?;
        self.repo.update_preferences(user_id, input).await
    }

    async fn opt_out(&self, user_id: &str, reason: Option<&str>) -> Result<CommunicationPreferences, AppError> {
        // Ensure preferences exist first
        self.repo.get_or_create_preferences(user_id).await?;
        self.repo.opt_out(user_id, reason).await
    }

    async fn can_receive(&self, user_id: &str, channel: CommChannel, category: MessageCategory, recipient: &str) -> Result<CanReceiveResult, AppError> {
        self.repo.can_receive_message(user_id, channel, category, recipient).await
    }

    async fn add_suppression(&self, input: AddSuppressionInput) -> Result<CommunicationSuppression, AppError> {
        self.repo.add_suppression(input).await
    }

    async fn is_suppressed(&self, channel: CommChannel, identifier: &str) -> Result<bool, AppError> {
        self.repo.is_suppressed(channel, identifier).await
    }

    async fn remove_suppression(&self, channel: CommChannel, identifier: &str) -> Result<(), AppError> {
        self.repo.remove_suppression(channel, identifier).await
    }

    async fn list_suppressions(&self, channel: Option<CommChannel>, limit: i32) -> Result<Vec<CommunicationSuppression>, AppError> {
        self.repo.list_suppressions(channel, limit).await
    }

    async fn process_bounce(&self, channel: CommChannel, identifier: &str, bounce_type: &str, source: &str) -> Result<CommunicationSuppression, AppError> {
        let permanent = bounce_type == "hard" || bounce_type == "permanent";

        self.repo.add_suppression(AddSuppressionInput {
            channel,
            identifier: identifier.to_string(),
            suppression_type: SuppressionType::Bounce,
            source: source.to_string(),
            reason: Some(format!("Bounce: {}", bounce_type)),
            bounce_type: Some(bounce_type.to_string()),
            complaint_type: None,
            permanent,
            user_id: None,
            expires_at: if permanent { None } else { Some(Utc::now() + chrono::Duration::days(7)) },
        }).await
    }

    async fn process_complaint(&self, channel: CommChannel, identifier: &str, complaint_type: &str, source: &str) -> Result<CommunicationSuppression, AppError> {
        self.repo.add_suppression(AddSuppressionInput {
            channel,
            identifier: identifier.to_string(),
            suppression_type: SuppressionType::Complaint,
            source: source.to_string(),
            reason: Some(format!("Complaint: {}", complaint_type)),
            bounce_type: None,
            complaint_type: Some(complaint_type.to_string()),
            permanent: true, // Complaints are always permanent
            user_id: None,
            expires_at: None,
        }).await
    }

    async fn create_template(&self, input: CreateTemplateInput) -> Result<MessageTemplate, AppError> {
        // Validate template syntax (simple {{variable}} format)
        CommunicationServiceImpl::<E, P>::validate_template_syntax(&input.body_template)?;

        if let Some(ref html) = input.html_template {
            CommunicationServiceImpl::<E, P>::validate_template_syntax(html)?;
        }

        self.repo.create_template(input).await
    }

    async fn get_template(&self, code: &str, channel: CommChannel, locale: &str) -> Result<Option<MessageTemplate>, AppError> {
        // Try exact locale first
        if let Some(template) = self.repo.get_template(code, channel, locale).await? {
            return Ok(Some(template));
        }

        // Fallback to base locale (e.g., "en-US" -> "en")
        if locale.contains('-') {
            let base_locale = locale.split('-').next().unwrap_or("en");
            if let Some(template) = self.repo.get_template(code, channel, base_locale).await? {
                return Ok(Some(template));
            }
        }

        // Fallback to default "en"
        if locale != "en" {
            return self.repo.get_template(code, channel, "en").await;
        }

        Ok(None)
    }

    async fn list_templates(&self, channel: Option<CommChannel>, category: Option<MessageCategory>) -> Result<Vec<MessageTemplate>, AppError> {
        self.repo.list_templates(channel, category).await
    }

    async fn update_template(&self, id: Uuid, input: UpdateTemplateInput) -> Result<MessageTemplate, AppError> {
        // Validate new template syntax if provided (simple {{variable}} format)
        if let Some(ref body) = input.body_template {
            CommunicationServiceImpl::<E, P>::validate_template_syntax(body)?;
        }

        if let Some(ref html) = input.html_template {
            CommunicationServiceImpl::<E, P>::validate_template_syntax(html)?;
        }

        // Reset to draft status on update
        let template = self.repo.get_template_by_id(id).await?
            .ok_or_else(|| AppError::NotFound(format!("Template {} not found", id)))?;

        if template.status == TemplateStatus::Approved {
            return Err(AppError::ValidationError("Cannot update approved template. Create a new version instead.".to_string()));
        }

        self.repo.update_template(id, input).await
    }

    async fn submit_template_for_approval(&self, id: Uuid) -> Result<MessageTemplate, AppError> {
        self.repo.submit_template_for_approval(id).await
    }

    async fn approve_template(&self, id: Uuid, approved_by: &str) -> Result<MessageTemplate, AppError> {
        self.repo.approve_template(id, approved_by).await
    }

    async fn reject_template(&self, id: Uuid, reason: &str) -> Result<MessageTemplate, AppError> {
        self.repo.reject_template(id, reason).await
    }

    async fn render_template(&self, template: &MessageTemplate, data: &serde_json::Value) -> Result<RenderedMessage, AppError> {
        let body = self.render_template(&template.body_template, data)?;

        let subject = if let Some(ref subj) = template.subject {
            Some(self.render_template(subj, data)?)
        } else {
            None
        };

        let title = if let Some(ref t) = template.title {
            Some(self.render_template(t, data)?)
        } else {
            None
        };

        let html_body = if let Some(ref html) = template.html_template {
            Some(self.render_template(html, data)?)
        } else {
            None
        };

        Ok(RenderedMessage {
            subject,
            title,
            body,
            html_body,
        })
    }

    async fn schedule_message(&self, input: ScheduleMessageInput) -> Result<ScheduledMessage, AppError> {
        // Check if user can receive this message
        let can_receive = self.can_receive(&input.user_id, input.channel, input.category, &input.recipient_address).await?;

        if !can_receive.can_send {
            return Err(AppError::ValidationError(format!(
                "Cannot schedule message: {}",
                can_receive.reason.unwrap_or_else(|| "unknown".to_string())
            )));
        }

        self.repo.schedule_message(input).await
    }

    async fn send_immediate(&self, user_id: &str, channel: CommChannel, category: MessageCategory,
                            recipient: &str, subject: Option<&str>, body: &str, html: Option<&str>) -> Result<SendMessageResult, AppError> {
        // Check if user can receive
        let can_receive = self.can_receive(user_id, channel, category, recipient).await?;

        if !can_receive.can_send {
            return Ok(SendMessageResult {
                success: false,
                channel,
                message_id: None,
                error: can_receive.reason,
            });
        }

        // Check suppression
        if self.is_suppressed(channel, recipient).await? {
            return Ok(SendMessageResult {
                success: false,
                channel,
                message_id: None,
                error: Some("Recipient is suppressed".to_string()),
            });
        }

        // Send based on channel
        match channel {
            CommChannel::Email => {
                let result = self.email_provider
                    .send_email(recipient, subject.unwrap_or(""), body, html)
                    .await?;

                Ok(SendMessageResult {
                    success: result.success,
                    channel,
                    message_id: result.message_id,
                    error: result.error,
                })
            }
            CommChannel::Push => {
                // For push, recipient is the device token
                let result = self.push_provider
                    .send_push(recipient, "unknown", subject.unwrap_or(""), body, None)
                    .await?;

                Ok(SendMessageResult {
                    success: result.success,
                    channel,
                    message_id: result.message_id,
                    error: result.error,
                })
            }
            CommChannel::Sms => {
                // SMS would require an SMS provider
                Ok(SendMessageResult {
                    success: false,
                    channel,
                    message_id: None,
                    error: Some("SMS sending not implemented".to_string()),
                })
            }
            CommChannel::InApp => {
                // In-app notifications would be stored differently
                Ok(SendMessageResult {
                    success: true,
                    channel,
                    message_id: Some(format!("in-app-{}", Uuid::new_v4())),
                    error: None,
                })
            }
        }
    }

    async fn process_scheduled_messages(&self, limit: i32) -> Result<SendStats, AppError> {
        let pending = self.repo.get_pending_messages(limit).await?;
        let mut sent = 0i64;
        let mut failed = 0i64;

        for message in pending {
            // Mark as sending
            self.repo.mark_message_sending(message.id).await?;

            // Check if user can still receive
            let can_receive = self.can_receive(
                &message.user_id,
                message.channel,
                message.category,
                &message.recipient_address,
            ).await?;

            if !can_receive.can_send {
                self.repo.mark_message_failed(
                    message.id,
                    "blocked",
                    &can_receive.reason.unwrap_or_else(|| "blocked".to_string()),
                ).await?;
                failed += 1;
                continue;
            }

            // Check suppression
            if self.is_suppressed(message.channel, &message.recipient_address).await? {
                self.repo.mark_message_failed(message.id, "suppressed", "Recipient is suppressed").await?;
                failed += 1;
                continue;
            }

            // Send the message
            let result = match message.channel {
                CommChannel::Email => {
                    self.email_provider
                        .send_email(
                            &message.recipient_address,
                            message.subject.as_deref().unwrap_or(""),
                            &message.body,
                            message.html_body.as_deref(),
                        )
                        .await
                        .map(|r| SendMessageResult {
                            success: r.success,
                            channel: message.channel,
                            message_id: r.message_id,
                            error: r.error,
                        })
                }
                CommChannel::Push => {
                    // Get device info
                    let devices = self.repo.get_user_devices(&message.user_id).await?;

                    // Try to find the specific device or use first active one
                    let device = devices.iter().find(|d| d.active).map(|d| (&d.token, &d.platform));

                    if let Some((token, platform)) = device {
                        self.push_provider
                            .send_push(
                                token,
                                platform,
                                message.title.as_deref().unwrap_or(""),
                                &message.body,
                                None,
                            )
                            .await
                            .map(|r| SendMessageResult {
                                success: r.success,
                                channel: message.channel,
                                message_id: r.message_id,
                                error: r.error,
                            })
                    } else {
                        Ok(SendMessageResult {
                            success: false,
                            channel: message.channel,
                            message_id: None,
                            error: Some("No active device found".to_string()),
                        })
                    }
                }
                CommChannel::Sms | CommChannel::InApp => {
                    Ok(SendMessageResult {
                        success: false,
                        channel: message.channel,
                        message_id: None,
                        error: Some("Channel not implemented".to_string()),
                    })
                }
            };

            match result {
                Ok(r) if r.success => {
                    self.repo.mark_message_sent(message.id, r.message_id.as_deref()).await?;

                    // Increment template usage if applicable
                    if let Some(template_id) = message.template_id {
                        let _ = self.repo.increment_template_usage(template_id).await;
                    }

                    sent += 1;
                }
                Ok(r) => {
                    self.repo.mark_message_failed(
                        message.id,
                        "send_failed",
                        &r.error.unwrap_or_else(|| "Unknown error".to_string()),
                    ).await?;
                    failed += 1;
                }
                Err(e) => {
                    self.repo.mark_message_failed(message.id, "error", &e.to_string()).await?;
                    failed += 1;
                }
            }
        }

        Ok(SendStats {
            scheduled: (limit as i64) - sent - failed,
            sent,
            failed,
            cancelled: 0,
        })
    }

    async fn cancel_message(&self, id: Uuid) -> Result<ScheduledMessage, AppError> {
        self.repo.cancel_message(id).await
    }

    async fn get_user_messages(&self, user_id: &str, limit: i32) -> Result<Vec<ScheduledMessage>, AppError> {
        self.repo.get_user_messages(user_id, limit).await
    }

    async fn register_device(&self, input: RegisterDeviceInput) -> Result<DeviceToken, AppError> {
        self.repo.register_device(input).await
    }

    async fn get_user_devices(&self, user_id: &str) -> Result<Vec<DeviceToken>, AppError> {
        self.repo.get_user_devices(user_id).await
    }

    async fn invalidate_device(&self, token_hash: &str, reason: &str) -> Result<(), AppError> {
        self.repo.invalidate_device(token_hash, reason).await
    }

    async fn get_send_stats(&self) -> Result<SendStats, AppError> {
        self.repo.get_send_stats().await
    }
}

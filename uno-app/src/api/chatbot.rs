//! Chatbot server functions

use leptos::prelude::*;

/// Send a chat message and get a bot response
#[server(SendChatMessage, "/api")]
pub async fn send_chat_message(message: String) -> Result<String, ServerFnError> {
    use crate::server::services::chatbot_service::ChatbotService;

    let service = ChatbotService::new();
    let response = service.get_response(&message);
    
    Ok(response)
}

//! Contains generic message types that can be used with multiple LLM models.
//!
//! These types are used to represent messages that can be sent to and received
//! from LLM models. They are designed to be generic and reusable across
//! different models and applications. This module also provides conversion
//! traits to convert these generic message types into model-specific message
//! types for use with specific LLM models.

use async_openai::types::{
    ChatCompletionRequestAssistantMessageArgs, ChatCompletionRequestMessage,
    ChatCompletionRequestUserMessageArgs,
};

use crate::models::claude_3::{ClaudeMessage, ClaudeMessageContent};

pub enum ChatMessage {
    User(UserChatMessage),
    Assistant(AssistantChatMessage),
}

#[allow(dead_code)]
enum ChatMessageContent {
    Text(String),
    Image(String),
}

pub struct UserChatMessage {
    content: ChatMessageContent,
}

pub struct AssistantChatMessage {
    content: ChatMessageContent,
}

struct ChatMessageVec {
    messages: Vec<ChatMessage>,
}

impl From<ChatMessage> for ChatCompletionRequestMessage {
    fn from(message: ChatMessage) -> ChatCompletionRequestMessage {
        match message {
            ChatMessage::User(user_message) => user_message.into(),
            ChatMessage::Assistant(assistant_message) => assistant_message.into(),
        }
    }
}

impl From<ChatMessage> for ClaudeMessage {
    fn from(message: ChatMessage) -> ClaudeMessage {
        match message {
            ChatMessage::User(user_message) => user_message.into(),
            ChatMessage::Assistant(assistant_message) => assistant_message.into(),
        }
    }
}

impl From<UserChatMessage> for ChatCompletionRequestMessage {
    fn from(message: UserChatMessage) -> ChatCompletionRequestMessage {
        match message.content {
            ChatMessageContent::Text(text) => ChatCompletionRequestUserMessageArgs::default()
                .content(text)
                .build()
                .unwrap()
                .into(),
            ChatMessageContent::Image(_image) => todo!("Image support"),
        }
    }
}

impl From<UserChatMessage> for ClaudeMessage {
    fn from(message: UserChatMessage) -> ClaudeMessage {
        match message.content {
            ChatMessageContent::Text(text) => ClaudeMessage {
                role: Some("user".to_string()),
                content: vec![ClaudeMessageContent {
                    content_type: "text".to_string(),
                    text: Some(text),
                    source: None,
                }],
            },
            ChatMessageContent::Image(_image) => todo!("Image support"),
        }
    }
}

impl From<AssistantChatMessage> for ChatCompletionRequestMessage {
    fn from(message: AssistantChatMessage) -> ChatCompletionRequestMessage {
        match message.content {
            ChatMessageContent::Text(text) => ChatCompletionRequestAssistantMessageArgs::default()
                .content(text)
                .build()
                .unwrap()
                .into(),
            ChatMessageContent::Image(_image) => todo!("Image support"),
        }
    }
}

impl From<AssistantChatMessage> for ClaudeMessage {
    fn from(message: AssistantChatMessage) -> ClaudeMessage {
        match message.content {
            ChatMessageContent::Text(text) => ClaudeMessage {
                role: Some("assistant".to_string()),
                content: vec![ClaudeMessageContent {
                    content_type: "text".to_string(),
                    text: Some(text),
                    source: None,
                }],
            },
            ChatMessageContent::Image(_image) => todo!("Image support"),
        }
    }
}

impl From<Vec<ChatMessage>> for ChatMessageVec {
    fn from(messages: Vec<ChatMessage>) -> ChatMessageVec {
        ChatMessageVec { messages }
    }
}

impl From<ChatMessageVec> for Vec<ChatCompletionRequestMessage> {
    fn from(messages: ChatMessageVec) -> Vec<ChatCompletionRequestMessage> {
        messages.messages.into_iter().map(|m| m.into()).collect()
    }
}

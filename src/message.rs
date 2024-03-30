use async_openai::types::{
    ChatCompletionRequestAssistantMessageArgs, ChatCompletionRequestMessage,
    ChatCompletionRequestUserMessageArgs,
};

pub enum MessageType {
    User,
    Assistant,
}

/// A message to be sent to an LLM
pub struct Message<T>
where
    T: Into<String>,
{
    text: T,
    message_type: MessageType,
}

impl<T> Message<T>
where
    T: Into<String>,
{
    pub fn new(text: T, message_type: MessageType) -> Self {
        Message { text, message_type }
    }
}

impl<T> From<Message<T>> for String
where
    T: Into<String>,
{
    fn from(message: Message<T>) -> String {
        message.text.into()
    }
}

impl<T> From<Message<T>> for ChatCompletionRequestMessage
where
    T: Into<String>,
{
    fn from(message: Message<T>) -> ChatCompletionRequestMessage {
        let content = message.text.into();
        match message.message_type {
            MessageType::User => ChatCompletionRequestUserMessageArgs::default()
                .content(content)
                .build()
                .unwrap()
                .into(),
            MessageType::Assistant => ChatCompletionRequestAssistantMessageArgs::default()
                .content(content)
                .build()
                .unwrap()
                .into(),
        }
    }
}

/// A vector of messages to be sent to an LLM
pub struct MessageVec<T>
where
    T: Into<String>,
{
    messages: Vec<Message<T>>,
}

impl<T> From<Vec<Message<T>>> for MessageVec<T>
where
    T: Into<String>,
{
    fn from(messages: Vec<Message<T>>) -> Self {
        MessageVec { messages }
    }
}

impl<T> From<MessageVec<T>> for Vec<ChatCompletionRequestMessage>
where
    T: Into<String>,
{
    fn from(messages: MessageVec<T>) -> Vec<ChatCompletionRequestMessage> {
        messages.messages.into_iter().map(|m| m.into()).collect()
    }
}

#![allow(dead_code)]

use anyhow::{anyhow, Context, Result};
use async_trait::async_trait;
use std::ops::BitOr;

use async_openai::types::{
    ChatCompletionRequestAssistantMessageArgs, ChatCompletionRequestMessage,
    ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
    CreateChatCompletionRequestArgs,
};

#[async_trait]
pub trait Link {
    async fn run(&self, input: &str) -> Result<String>;
}

pub struct Chain {
    links: Vec<Box<dyn Link>>,
}

impl Chain {
    pub fn new() -> Self {
        Chain { links: Vec::new() }
    }

    fn add_link<L: Link + 'static>(mut self, link: L) -> Self {
        self.links.push(Box::new(link));
        self
    }

    pub async fn run(self, input: String) -> Result<String> {
        let mut result = input;
        for link in self.links {
            result = link.run(&result).await?;
        }

        Ok(result)
    }
}

impl Default for Chain {
    fn default() -> Self {
        Chain::new()
    }
}

impl<L: Link + 'static> BitOr<L> for Chain {
    type Output = Chain;

    fn bitor(self, link: L) -> Chain {
        self.add_link(link)
    }
}

pub enum MessageType {
    System,
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
            MessageType::System => ChatCompletionRequestSystemMessageArgs::default()
                .content(content)
                .build()
                .unwrap()
                .into(),
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
///
/// Example:
/// ```rust
/// use async_openai::types::ChatCompletionRequestMessage;
/// use anchor_chain::MessageVec;
/// use anchor_chain::Message;
/// use anchor_chain::MessageType;
/// let messages = vec![
///    Message::new("You are a helpful assistant", MessageType::System),
///    Message::new("Hello", MessageType::User),
///    Message::new("How can I help you?", MessageType::Assistant),
///    Message::new("How far is the the Sun from Earth?", MessageType::User),
///    Message::new("About 3.50", MessageType::Assistant),
/// ];
/// let messages = MessageVec::from(messages);
/// let messages: Vec<ChatCompletionRequestMessage> = messages.into();
/// ```
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

pub struct Gpt3_5Turbo {
    system_prompt: String,
    client: async_openai::Client<async_openai::config::OpenAIConfig>,
}

impl Gpt3_5Turbo {
    pub async fn new(system_prompt: String) -> Result<Self> {
        let config = async_openai::config::OpenAIConfig::new();
        let client = async_openai::Client::with_config(config);
        Ok(Gpt3_5Turbo {
            system_prompt,
            client,
        })
    }

    pub async fn new_with_key(system_prompt: String, api_key: String) -> Result<Self> {
        let config = async_openai::config::OpenAIConfig::new().with_api_key(api_key);
        let client = async_openai::Client::with_config(config);
        Ok(Gpt3_5Turbo {
            system_prompt,
            client,
        })
    }
}

#[async_trait]
impl Link for Gpt3_5Turbo {
    async fn run(&self, input: &str) -> Result<String> {
        let system_prompt = ChatCompletionRequestSystemMessageArgs::default()
            .content(self.system_prompt.clone())
            .build()?
            .into();

        let input = ChatCompletionRequestUserMessageArgs::default()
            .content(input)
            .build()?
            .into();

        let request = CreateChatCompletionRequestArgs::default()
            .max_tokens(512u16)
            .model("gpt-3.5-turbo")
            .messages([system_prompt, input])
            .build()?;

        let response = self.client.chat().create(request).await?;
        if response.choices.is_empty() {
            return Err(anyhow!("No choices in response"));
        }

        let content = response.choices[0]
            .message
            .clone()
            .content
            .context("No content in response")?;

        Ok(content)
    }
}

pub struct Prompt {
    text: String,
}

impl Prompt {
    pub fn new(text: &str) -> Self {
        Prompt {
            text: text.to_string(),
        }
    }
}

#[async_trait]
impl Link for Prompt {
    async fn run(&self, input: &str) -> Result<String> {
        // TODO: Take hashmap for parameterized input
        let result = self.text.replace("{input}", input);
        println!("Prompt: {}", result);
        Ok(result)
    }
}

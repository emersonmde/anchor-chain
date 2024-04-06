//! Module for handling dynamic prompts in processing chains.
//!
//! This module provides the `Prompt` struct, a processor for handling
//! and displaying text prompts.

use std::{collections::HashMap, marker::PhantomData};

use crate::node::Node;
use anyhow::Result;
use async_trait::async_trait;
use tera::{Context, Tera};

trait IntoContext {
    fn into_context(self) -> Context;
}

impl IntoContext for Context {
    fn into_context(self) -> Context {
        self
    }
}

impl IntoContext for String {
    fn into_context(self) -> Context {
        let mut context = Context::new();
        context.insert("input", &self);
        context
    }
}

/// A processor for handling text prompts within a processing chain.
///
/// `Prompt` is primarily used for manipulating or logging text data as it flows through
/// a processing chain.
#[derive(Debug)]
pub struct Prompt<'a> {
    /// The prompt text that will be combined with the input text.
    text: String,
    tera: Tera,
    _marker: PhantomData<&'a str>,
}

impl<'a> Prompt<'a> {
    /// Creates a new `Prompt` processor with the specified text.
    ///
    /// # Parameters
    /// - `text`: The static or template text to be associated with this prompt processor.
    ///
    /// # Examples
    /// ```
    /// use anchor_chain::prompt::Prompt;
    ///
    /// let prompt = Prompt::new("Hello, World!");
    /// ```
    pub fn new(text: &str) -> Self {
        let mut tera = Tera::default();
        tera.add_raw_template("prompt", text)
            .expect("Error creating template");
        Prompt {
            text: text.to_string(),
            tera,
            _marker: PhantomData,
        }
    }
}

#[async_trait]
impl<'a> Node for Prompt<'a> {
    type Input = HashMap<&'a str, &'a str>;
    type Output = String;

    async fn process(&self, input: Self::Input) -> Result<Self::Output> {
        let context = Context::from_serialize(input)?;
        Ok(self.tera.render("prompt", &context)?.to_string())
    }
}

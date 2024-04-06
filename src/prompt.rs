//! Module for handling dynamic prompts in processing chains.
//!
//! This module provides the `Prompt` struct, a processor for handling
//! and displaying text prompts. The `Prompt` struct uses Tera templating
//! to allow for dynamic input substitution in the prompt text. Tera is a
//! template engine that allows for dynamic templating using variables with
//! a similar syntax to Jinja2. For more information on Tera, see the
//! [Tera documentation](https://keats.github.io/tera/docs/#templates).

use std::{collections::HashMap, marker::PhantomData};

use crate::node::Node;
use anyhow::Result;
use async_trait::async_trait;
use tera::{Context, Tera};

/// A processor for handling text prompts within a processing chain.
///
/// The `Prompt` struct is a processor for handling text prompts within a
/// processing chain using Tera templating.
#[derive(Debug)]
pub struct Prompt<'a> {
    /// The Tera template used to process the prompt text.
    tera: Tera,
    _marker: PhantomData<&'a str>,
}

impl<'a> Prompt<'a> {
    /// Creates a new `Prompt` processor with the specified template.
    ///
    /// Templates need to be specified using the Tera syntax which is based on
    /// Jinja2. For more information on Tera, see the
    /// [Tera Templates documentation](https://keats.github.io/tera/docs/#templates).
    ///
    /// # Parameters
    /// - `template`: The raw Tera template for the prompt.
    ///
    /// # Examples
    /// ```rust
    /// use anchor_chain::prompt::Prompt;
    ///
    /// let prompt = Prompt::new("Create a {{ language }} program that prints 'Hello, World!'");
    /// ```
    pub fn new(template: &str) -> Self {
        let mut tera = Tera::default();
        tera.add_raw_template("prompt", template)
            .expect("Error creating template");
        Prompt {
            tera,
            _marker: PhantomData,
        }
    }
}

/// Implements the `Node` trait for the `Prompt` struct.
#[async_trait]
impl<'a> Node for Prompt<'a> {
    /// Input HashMap that will be converted to the tera::Context.
    type Input = HashMap<&'a str, &'a str>;
    /// Output string from the rendered template.
    type Output = String;

    /// Processes the input HashMap and returns the rendered template.
    async fn process(&self, input: Self::Input) -> Result<Self::Output> {
        let context = Context::from_serialize(input)?;
        Ok(self.tera.render("prompt", &context)?.to_string())
    }
}

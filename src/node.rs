//! Module providing foundational structures for building chains.
//!
//! This module defines a `Node` trait for asynchronous operations and
//! constructs (`Link` and `End`) to create chains of these operations.

use std::marker::PhantomData;

use anyhow::Result;
use async_trait::async_trait;

/// Represents an node that can process an input to produce an output.
///
/// The `Node` trait defines a generic processing operation with a
/// specified input and output type. Implementors of this trait can be
/// composed together to form a processing chain.
#[async_trait]
pub trait Node {
    /// The input type for the node.
    type Input;
    /// The output type for the node.
    type Output;

    /// Asynchronously processes the given input, returning the output.
    ///
    /// # Parameters
    /// - `input`: The input value to be processed.
    ///
    /// # Returns
    /// A `Result` containing the processed output value or an error if processing fails.
    async fn process(&self, input: Self::Input) -> Result<Self::Output>;
}

pub struct PassthroughNode<T> {
    _marker: PhantomData<T>,
}

impl<T> PassthroughNode<T> {
    pub fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

impl<T> Default for PassthroughNode<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl<T> Node for PassthroughNode<T>
where
    T: Send + Sync,
{
    type Input = T;
    type Output = T;

    async fn process(&self, input: Self::Input) -> Result<Self::Output> {
        Ok(input)
    }
}

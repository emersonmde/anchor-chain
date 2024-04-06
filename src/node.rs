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
pub trait Node: std::fmt::Debug {
    /// The input type for the node.
    type Input;
    /// The output type for the node.
    type Output;

    /// Asynchronously processes the given input, returning the output. When
    /// chained together the output type of one node must match the input of
    /// the next node in the chain.
    async fn process(&self, input: Self::Input) -> Result<Self::Output>;
}

/// A no-op node that passes input through unchanged.
#[derive(Debug)]
pub struct PassthroughNode<T> {
    _marker: PhantomData<T>,
}

impl<T> PassthroughNode<T> {
    /// Creates a new `PassthroughNode`.
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
    T: Send + Sync + std::fmt::Debug,
{
    /// The input type for the PassthroughNode.
    type Input = T;
    /// The output type for the PassthroughNode.
    type Output = T;

    /// Returns the input unchanged.
    async fn process(&self, input: Self::Input) -> Result<Self::Output> {
        Ok(input)
    }
}

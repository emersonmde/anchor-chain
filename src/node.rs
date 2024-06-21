//! Module providing foundational structures for building chains.
//!
//! This module defines a `Node` trait for asynchronous operations and
//! constructs (`Link` and `End`) to create chains of these operations.

use std::fmt;
use std::marker::PhantomData;

use anchor_chain_macros::Stateless;
use async_trait::async_trait;
#[cfg(feature = "tracing")]
use tracing::instrument;

use crate::error::AnchorChainError;
use crate::state_manager::StateManager;

/// Represents a node that can process an input to produce an output.
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
    async fn process(&self, input: Self::Input) -> Result<Self::Output, AnchorChainError>;
}

pub trait Stateless: Node {}

#[async_trait]
pub trait Stateful<M>: Node {
    async fn set_state(&mut self, state: StateManager<M>);
}

/// A no-op node that passes input through unchanged.
#[derive(Debug, Stateless)]
pub struct NoOpNode<T: fmt::Debug> {
    _marker: PhantomData<T>,
}

impl<T: fmt::Debug> NoOpNode<T> {
    /// Creates a new `NoOpNode`.
    pub fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

impl<T: fmt::Debug> Default for NoOpNode<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl<T> Node for NoOpNode<T>
where
    T: Send + Sync + fmt::Debug,
{
    /// The input type for the NoOpNode.
    type Input = T;
    /// The output type for the NoOpNode.
    type Output = T;

    /// Returns the input unchanged.
    #[cfg_attr(feature = "tracing", instrument(skip(self)))]
    async fn process(&self, input: Self::Input) -> Result<Self::Output, AnchorChainError> {
        Ok(input)
    }
}

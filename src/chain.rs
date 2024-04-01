//! Provides structures for creating and executing chains.
//!
//! This module defines `Chain` and `ChainBuilder` structures for building chains of
//! operations where each operation is represented by a `Node`. These chains
//! facilitate asynchronous processing of data from an initial input to a final output.

use crate::node::{Link, Node};
use anyhow::Result;
use std::marker::PhantomData;

/// Represents a chain of nodes that can asynchronously process data.
///
/// `Chain` is constructed from a sequence of `Node` instances, each taking an input
/// and producing an output. The output of one node serves as the input to the next,
/// allowing for a flexible and composable approach to complex asynchronous processing tasks.
pub struct Chain<I, O, L> {
    link: L,
    _input: PhantomData<I>,
    _output: PhantomData<O>,
}

impl<I, O, L> Chain<I, O, L>
where
    L: Node<Input = I, Output = O> + Send + Sync,
{
    /// Creates a new `Chain` from the provided initial link.
    ///
    /// # Parameters
    /// - `link`: The starting link of the chain.
    pub fn new(link: L) -> Self {
        Chain {
            link,
            _input: PhantomData,
            _output: PhantomData,
        }
    }

    /// Asynchronously processes the provided input through the chain of nodes.
    ///
    /// # Parameters
    /// - `input`: The initial input to the chain.
    ///
    /// # Returns
    /// A `Result` containing the final output after processing or an error if processing fails.
    pub async fn process(&self, input: I) -> Result<O> {
        self.link.process(input).await
    }
}

/// A builder for constructing a `Chain` of nodes.
///
/// `ChainBuilder` allows for incremental construction of a processing chain, adding
/// node one at a time. This approach facilitates clear and concise assembly
/// of complex processing logic.
pub struct ChainBuilder<I, L> {
    link: L,
    _input: PhantomData<I>,
}

impl<I, L> ChainBuilder<I, L>
where
    L: Node<Input = I> + Send + Sync,
    I: Send,
{
    /// Initializes a new `ChainBuilder` with the provided starting link.
    ///
    /// # Parameters
    /// - `link`: The first node or link to start building the chain.
    pub fn new(link: L) -> Self {
        ChainBuilder {
            link,
            _input: PhantomData,
        }
    }

    /// Adds a new node to the chain, linking it to the previous node.
    ///
    /// # Parameters
    /// - `next`: The node to add to the chain.
    ///
    /// # Returns
    /// A new `ChainBuilder` instance representing the current state of the chain,
    /// with the new node added.
    pub fn link<N>(self, next: N) -> ChainBuilder<I, Link<L, N>>
    where
        N: Node<Input = L::Output> + Send + Sync,
        L::Output: Send,
        Link<L, N>: Node<Input = I>,
    {
        ChainBuilder {
            link: Link {
                node: self.link,
                next,
            },
            _input: PhantomData,
        }
    }

    /// Finalizes the construction of the chain, returning a `Chain` instance
    /// ready for processing.
    ///
    /// # Returns
    /// A `Chain` instance constructed from the nodes added to the builder.
    pub fn build(self) -> Chain<I, L::Output, L>
    where
        L: Node,
    {
        Chain {
            link: self.link,
            _input: PhantomData,
            _output: PhantomData,
        }
    }
}

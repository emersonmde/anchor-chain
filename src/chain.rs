//! Provides structures for creating and executing chains.
//!
//! This module defines `Chain` and `ChainBuilder` structures for building chains of
//! operations where each operation is represented by a `Node`. These chains
//! facilitate asynchronous processing of data from an initial input to a final output.

use crate::{link::Link, node::Node};
use anyhow::Result;
use std::marker::PhantomData;

/// Represents a chain of nodes that can asynchronously process data.
///
/// `Chain` is constructed from a sequence of `Node` instances, each taking an input
/// and producing an output. The output of one node serves as the input to the next,
/// allowing for a flexible and composable approach to complex asynchronous processing tasks.
#[derive(Debug)]
pub struct Chain<I, O, L> {
    link: L,
    _input: PhantomData<I>,
    _output: PhantomData<O>,
}

impl<I, O, L> Chain<I, O, L>
where
    L: Node<Input = I, Output = O> + Send + Sync + std::fmt::Debug,
    I: std::fmt::Debug,
    O: std::fmt::Debug,
{
    /// Creates a new `Chain` from the provided initial link.
    ///
    /// `Link` serves as a container for chaining two `Node` instances together,
    /// where the output of the first node is fed as the input to the next. These
    /// links can be nested to create a chain of nodes.
    pub fn new(link: L) -> Self {
        Chain {
            link,
            _input: PhantomData,
            _output: PhantomData,
        }
    }

    /// Asynchronously processes the provided input through the chain of nodes.
    ///
    /// The input is processed by each node in the chain, with the output of one node
    /// serving as the input to the next. The final output of the chain is returned.
    /// If any node in the chain returns an error, the processing is halted and
    /// the error is returned.
    pub async fn process(&self, input: I) -> Result<O> {
        self.link.process(input).await
    }
}

/// A builder for constructing a `Chain` of nodes.
///
/// `ChainBuilder` allows for incremental construction of a processing chain, adding
/// node one at a time. This approach facilitates clear and concise assembly
/// of complex processing logic.
pub struct ChainBuilder {}

impl ChainBuilder {
    /// Creates a new `ChainBuilder` instance.
    pub fn new() -> Self {
        ChainBuilder {}
    }

    /// Adds the first node to the chain.
    pub fn link<I, N>(self, node: N) -> LinkedChainBuilder<I, N>
    where
        N: Node<Input = I> + Send + Sync + std::fmt::Debug,
        I: Send,
    {
        LinkedChainBuilder {
            link: node,
            _input: PhantomData,
        }
    }
}

impl Default for ChainBuilder {
    fn default() -> Self {
        ChainBuilder::new()
    }
}

/// A builder for constructing a `Chain` of nodes using Link.
///
/// `LinkedChainBuilder` takes an initial node and allows for incremental
/// construction of a processing chain, adding nodes one at a time. New nodes
/// are linked to the previous nodes using nested `Link` instances.
pub struct LinkedChainBuilder<I, L> {
    link: L,
    _input: PhantomData<I>,
}

impl<I, L> LinkedChainBuilder<I, L>
where
    L: Node<Input = I> + Send + Sync + std::fmt::Debug,
    I: Send,
{
    /// Adds a new node to the chain, linking it to the previous node.
    pub fn link<N>(self, next: N) -> LinkedChainBuilder<I, Link<L, N>>
    where
        N: Node<Input = L::Output> + Send + Sync + std::fmt::Debug,
        L::Output: Send,
        Link<L, N>: Node<Input = I>,
    {
        LinkedChainBuilder {
            link: Link {
                node: self.link,
                next,
            },
            _input: PhantomData,
        }
    }

    /// Finalizes the construction of the chain, returning a `Chain` instance
    /// ready for processing.
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

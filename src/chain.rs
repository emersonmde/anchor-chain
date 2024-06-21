//! Provides structures for creating and executing chains.
//!
//! This module defines `Chain` and `ChainBuilder` structures for building chains of
//! operations where each operation is represented by a `Node`. These chains
//! facilitate asynchronous processing of data from an initial input to a final output.

use async_trait::async_trait;
use std::fmt;
use std::marker::PhantomData;

use crate::error::AnchorChainError;
use crate::link::StatefulLink;
use crate::node::{Stateful, Stateless};
use crate::state_manager::StateManager;
use crate::{link::Link, node::Node};

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
    L: Node<Input = I, Output = O> + Send + Sync + fmt::Debug,
    I: fmt::Debug,
    O: fmt::Debug,
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
    pub async fn process(&self, input: I) -> Result<O, AnchorChainError> {
        self.link.process(input).await
    }
}

#[async_trait]
impl<I, O, L> Node for Chain<I, O, L>
where
    L: Node<Input = I, Output = O> + Send + Sync + fmt::Debug,
    I: fmt::Debug + Send + Sync,
    O: fmt::Debug + Send + Sync,
{
    type Input = I;
    type Output = O;

    async fn process(&self, input: Self::Input) -> Result<Self::Output, AnchorChainError> {
        self.process(input).await
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
        N: Node<Input = I> + Stateless + Send + Sync + fmt::Debug,
        I: Send,
    {
        LinkedChainBuilder {
            link: node,
            _input: PhantomData,
        }
    }

    pub fn link_with_state<I, N, M>(self, node: N) -> StatefulLinkedChainBuilder<I, N, M>
    where
        N: Node<Input = I> + Stateful<M> + Send + Sync + fmt::Debug,
        I: Send,
        M: Clone,
    {
        StatefulLinkedChainBuilder {
            link: node,
            state: StateManager::new(),
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
    L: Node<Input = I> + Send + Sync + fmt::Debug,
    I: Send,
{
    /// Adds a new node to the chain, linking it to the previous node.
    pub fn link<N>(self, next: N) -> LinkedChainBuilder<I, Link<L, N>>
    where
        N: Node<Input = L::Output> + Stateless + Send + Sync + fmt::Debug,
        L::Output: Send,
        Link<L, N>: Node<Input = I>,
    {
        LinkedChainBuilder {
            link: Link::new(self.link, next),
            _input: PhantomData,
        }
    }

    pub fn link_with_state<N, M>(
        self,
        next: N,
    ) -> StatefulLinkedChainBuilder<I, StatefulLink<L, N, M>, M>
    where
        N: Node<Input = L::Output> + Stateful<M> + Send + Sync + fmt::Debug,
        L::Output: Send,
        Link<L, N>: Node<Input = I>,
        M: Clone + Sync + Send + fmt::Debug,
    {
        let state = StateManager::new();
        StatefulLinkedChainBuilder {
            link: StatefulLink::new(self.link, next, state.clone()),
            state,
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

pub struct StatefulLinkedChainBuilder<I, L, M> {
    link: L,
    state: StateManager<M>,
    _input: PhantomData<I>,
}

impl<I, L, M> StatefulLinkedChainBuilder<I, L, M>
where
    L: Node<Input = I> + Send + Sync + fmt::Debug,
    I: Send,
    M: Clone + Send + Sync + fmt::Debug,
{
    /// Adds a new node to the chain, linking it to the previous node.
    pub fn link<N>(self, next: N) -> StatefulLinkedChainBuilder<I, Link<L, N>, M>
    where
        N: Node<Input = L::Output> + Stateless + Send + Sync + fmt::Debug,
        L::Output: Send,
        Link<L, N>: Node<Input = I>,
    {
        StatefulLinkedChainBuilder {
            link: Link::new(self.link, next),
            state: self.state,
            _input: PhantomData,
        }
    }

    pub fn link_with_state<N>(
        self,
        next: N,
    ) -> StatefulLinkedChainBuilder<I, StatefulLink<L, N, M>, M>
    where
        N: Node<Input = L::Output> + Stateful<M> + Send + Sync + fmt::Debug,
        L::Output: Send,
        StatefulLink<L, N, String>: Node<Input = I>,
    {
        StatefulLinkedChainBuilder {
            link: StatefulLink::new(self.link, next, self.state.clone()),
            state: self.state,
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

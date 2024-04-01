//! Provides structures for creating and executing asynchronous processing chains.
//!
//! This module defines `Chain` and `ChainBuilder` structures for building chains of
//! operations where each operation is represented by a `Processor`. These chains
//! facilitate asynchronous processing of data from an initial input to a final output.

use crate::link::{Link, Processor};
use anyhow::Result;
use std::marker::PhantomData;

/// Represents a chain of processors that can asynchronously process data.
///
/// `Chain` is constructed from a sequence of `Processor` instances, each taking an input
/// and producing an output. The output of one processor serves as the input to the next,
/// allowing for a flexible and composable approach to complex asynchronous processing tasks.
pub struct Chain<I, O, L> {
    link: L,
    _input: PhantomData<I>,
    _output: PhantomData<O>,
}

impl<I, O, L> Chain<I, O, L>
where
    L: Processor<Input = I, Output = O> + Send + Sync,
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

    /// Asynchronously processes the provided input through the chain of processors.
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

/// A builder for constructing a `Chain` of processors.
///
/// `ChainBuilder` allows for incremental construction of a processing chain, adding
/// processors one at a time. This approach facilitates clear and concise assembly
/// of complex processing logic.
pub struct ChainBuilder<I, L> {
    link: L,
    _input: PhantomData<I>,
}

impl<I, L> ChainBuilder<I, L>
where
    L: Processor<Input = I> + Send + Sync,
    I: Send,
{
    /// Initializes a new `ChainBuilder` with the provided starting link.
    ///
    /// # Parameters
    /// - `link`: The first processor or link to start building the chain.
    pub fn new(link: L) -> Self {
        ChainBuilder {
            link,
            _input: PhantomData,
        }
    }

    /// Adds a new processor to the chain, linking it to the previous processor.
    ///
    /// # Parameters
    /// - `next`: The processor to add to the chain.
    ///
    /// # Returns
    /// A new `ChainBuilder` instance representing the current state of the chain,
    /// with the new processor added.
    pub fn link<N>(self, next: N) -> ChainBuilder<I, Link<L, N>>
    where
        N: Processor<Input = L::Output> + Send + Sync,
        L::Output: Send,
        Link<L, N>: Processor<Input = I>,
    {
        ChainBuilder {
            link: Link {
                processor: self.link,
                next,
            },
            _input: PhantomData,
        }
    }

    /// Finalizes the construction of the chain, returning a `Chain` instance
    /// ready for processing.
    ///
    /// # Returns
    /// A `Chain` instance constructed from the processors added to the builder.
    pub fn build(self) -> Chain<I, L::Output, L>
    where
        L: Processor,
    {
        Chain {
            link: self.link,
            _input: PhantomData,
            _output: PhantomData,
        }
    }
}

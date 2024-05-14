//! A simple input logging node.
//!
//! This node logs the input to the console and passes it through unchanged.
use crate::error::AnchorChainError;
use crate::node::Node;
#[cfg(feature = "tracing")]
use tracing::instrument;

/// A simple input logging node
#[derive(Debug)]
pub struct Logger<T> {
    prefix: String,
    _marker: std::marker::PhantomData<T>,
}

impl<T> Logger<T> {
    /// Create a new Logger node with the given prefix.
    ///
    /// The prefix is prepended to the input in the format `prefix: input`.
    pub fn new(prefix: &str) -> Self {
        Self {
            prefix: prefix.to_string(),
            _marker: std::marker::PhantomData,
        }
    }
}

#[async_trait::async_trait]
impl<T> Node for Logger<T>
where
    T: std::fmt::Debug + Send + Sync,
{
    type Input = T;
    type Output = T;

    /// Log the input and pass it through unchanged.
    #[cfg_attr(feature = "tracing", instrument)]
    async fn process(&self, input: Self::Input) -> Result<Self::Output, AnchorChainError> {
        println!("{}: {:?}", self.prefix, input);
        Ok(input)
    }
}

impl<T> Default for Logger<T> {
    fn default() -> Self {
        Self {
            prefix: "Input".to_string(),
            _marker: std::marker::PhantomData,
        }
    }
}

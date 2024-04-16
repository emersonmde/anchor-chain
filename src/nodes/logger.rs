use crate::error::AnchorChainError;
use crate::node::Node;

#[derive(Debug)]
pub struct Logger<T> {
    prefix: String,
    _marker: std::marker::PhantomData<T>,
}

impl<T> Logger<T> {
    pub fn new(prefix: String) -> Self {
        Self {
            prefix,
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

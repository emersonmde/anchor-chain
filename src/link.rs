use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait Processor {
    type Input;
    type Output;
    async fn process(&self, input: Self::Input) -> Result<Self::Output>;
}

pub struct Link<P, N> {
    pub processor: P,
    pub next: N,
}

pub struct End;

#[async_trait]
impl Processor for End {
    type Input = String;
    type Output = String;
    async fn process(&self, input: Self::Input) -> Result<Self::Output> {
        Ok(input)
    }
}

#[async_trait]
impl<P, N> Processor for Link<P, N>
where
    P: Processor + Send + Sync,
    N: Processor<Input = P::Output> + Send + Sync,
    P::Output: Send + 'static,
    P::Input: Send,
    N::Output: Send,
{
    type Input = P::Input;
    type Output = <N as Processor>::Output;
    async fn process(&self, input: Self::Input) -> Result<Self::Output> {
        let output = self.processor.process(input).await?;
        self.next.process(output).await
    }
}

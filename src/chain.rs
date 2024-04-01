use crate::link::{Link, Processor};

pub struct Chain<I, O, L> {
    link: L,
    _input: std::marker::PhantomData<I>,
    _output: std::marker::PhantomData<O>,
}

impl<I, O, L> Chain<I, O, L>
where
    L: Processor<Input = I, Output = O> + Send + Sync,
{
    pub fn new(link: L) -> Self {
        Chain {
            link,
            _input: std::marker::PhantomData,
            _output: std::marker::PhantomData,
        }
    }

    pub async fn process(&self, input: I) -> anyhow::Result<O> {
        self.link.process(input).await
    }
}

pub struct ChainBuilder<I, L> {
    link: L,
    _input: std::marker::PhantomData<I>,
}

impl<I, L> ChainBuilder<I, L>
where
    L: Processor<Input = I> + Send + Sync,
    I: Send,
{
    pub fn new(link: L) -> Self {
        ChainBuilder {
            link,
            _input: std::marker::PhantomData,
        }
    }

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
            _input: std::marker::PhantomData,
        }
    }

    pub fn build(self) -> Chain<I, L::Output, L>
    where
        L: Processor,
    {
        Chain {
            link: self.link,
            _input: std::marker::PhantomData,
            _output: std::marker::PhantomData,
        }
    }
}

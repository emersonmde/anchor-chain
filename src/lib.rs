#![allow(dead_code)]

pub mod message;
pub mod models;
pub mod prompt;

use anyhow::Result;
use async_trait::async_trait;
use std::ops::BitOr;

#[async_trait]
pub trait Link {
    async fn run(&self, input: &str) -> Result<String>;
}

pub struct Chain {
    links: Vec<Box<dyn Link>>,
}

impl Chain {
    pub fn new() -> Self {
        Chain { links: Vec::new() }
    }

    fn add_link<L: Link + 'static>(mut self, link: L) -> Self {
        self.links.push(Box::new(link));
        self
    }

    pub async fn run(self, input: String) -> Result<String> {
        let mut result = input;
        for link in self.links {
            result = link.run(&result).await?;
        }

        Ok(result)
    }
}

impl Default for Chain {
    fn default() -> Self {
        Chain::new()
    }
}

impl<L: Link + 'static> BitOr<L> for Chain {
    type Output = Chain;

    fn bitor(self, link: L) -> Chain {
        self.add_link(link)
    }
}

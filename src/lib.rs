#![allow(dead_code)]

use std::ops::BitOr;

trait Link {
    fn run(&self, input: String) -> String;
}

struct Chain {
    links: Vec<Box<dyn Link>>,
}

impl Chain {
    fn new() -> Self {
        Chain { links: Vec::new() }
    }

    fn add_link<L: Link + 'static>(mut self, link: L) -> Self {
        self.links.push(Box::new(link));
        self
    }

    fn run(self, input: String) -> String {
        self.links.into_iter().fold(input, |acc, link| link.run(acc))
    }
}

impl<L: Link + 'static> BitOr<L> for Chain {
    type Output = Chain;

    fn bitor(self, link: L) -> Chain {
        self.add_link(link)
    }
}

enum LanguageModel {
    Gpt3_5Turbo { api_key: String },
    BedrockClaude2_1 { aws_profile: String },
}

impl LanguageModel {
    fn new_gpt3_5_turbo(api_key: String) -> Self {
        LanguageModel::Gpt3_5Turbo { api_key }
    }

    fn new_bedrock_claude2_1(aws_profile: String) -> Self {
        LanguageModel::BedrockClaude2_1 { aws_profile }
    }
}

impl Link for LanguageModel {
    fn run(&self, input: String) -> String {
        match self {
            LanguageModel::Gpt3_5Turbo { api_key } => {
                println!("LanguageModel {} {}", input, api_key);
                format!("LanguageModel {} {}", input, api_key)
            }
            LanguageModel::BedrockClaude2_1 { aws_profile } => {
                println!("LanguageModel {} {}", input, aws_profile);
                format!("LanguageModel {} {}", input, aws_profile)
            }
        }
    }
}

struct Prompt {
    text: String,
}

impl Prompt {
    fn new(text: String) -> Self {
        Prompt { text }
    }
}

impl Link for Prompt {
    fn run(&self, input: String) -> String {
        println!("Prompt {} {}", self.text, input);
        format!("Prompt {} {}", self.text, input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let chain = Chain::new()
            | Prompt::new("Hello".to_string())
            | LanguageModel::new_gpt3_5_turbo("api_key".to_string());

        chain.run("Test".to_string());
    }
}

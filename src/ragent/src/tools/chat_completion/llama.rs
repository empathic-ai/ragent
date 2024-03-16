use crate::prelude::*;
use std::*;
use futures::Stream;
use futures_util::{StreamExt, stream};
use anyhow::*;

pub struct Llama {
}

/* 
impl Agent for Llama {
    fn new_message(&mut self, role: Role, text: String) {
 
    }

    async fn get_response(&self, prompt: String, token: CancellationToken) -> Result<impl Stream<Item = Result<LLMResponse>>> {
        let items = vec![Ok(LLMResponse::new("".to_string()))];
        let mut stream = stream::iter(items);
        Ok(stream)
    }

    fn new(config: AgentConfig) -> Self {
        Llama {
        }
    }

    fn get_config(&mut self) -> AgentConfig {
        todo!()
    }

    async fn send_event(&mut self, task: UserEvent) {
        todo!()
    }
}
*/
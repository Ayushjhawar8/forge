use model::{CallToolRequest, CallToolResponse};

mod fs;
mod model;
mod think;

#[async_trait::async_trait]
pub trait Tool {
    fn name(&self) -> &'static str;
    async fn tools_call(&self, input: CallToolRequest) -> Result<CallToolResponse, String>;
    fn tools_list(&self) -> Vec<&'static str>;
}

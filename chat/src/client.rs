use libchat::context::{Context, ConversationIdOwned, Introduction};
use libchat::errors::ChatError;
use libchat::types::AddressedEnvelope;
struct Client {
    ctx: Context,
}

impl Client {
    pub fn new_with_name(name: impl Into<String>) -> Self {
        Client {
            ctx: Context::new_with_name(name),
        }
    }

    pub fn create_intro_bundle(&mut self) -> Result<Vec<u8>, ChatError> {
        self.ctx.create_intro_bundle()
    }

    pub fn create_private_convo(
        &mut self,
        remote_bundle: &Introduction,
        content: &[u8],
    ) -> (ConversationIdOwned, Vec<AddressedEnvelope>) {
        self.ctx.create_private_convo(remote_bundle, content)
    }
}

use crux_macros::Capability;
use serde::{Deserialize, Serialize};

use crux_core::capability::{CapabilityContext, Operation};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct SignerRequest {
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum SignerResponse {
    Signature(Vec<u8>),
}

impl Operation for SignerRequest {
    type Output = SignerResponse;
}

#[derive(Capability)]
pub struct Signer<Ev> {
    context: CapabilityContext<SignerRequest, Ev>,
}

impl<Ev> Signer<Ev>
where
    Ev: 'static,
{
    pub fn new(context: CapabilityContext<SignerRequest, Ev>) -> Self {
        Self { context }
    }

    pub fn sign_message<F>(&self, message: &str, make_event: F)
    where
        F: Fn(Vec<u8>) -> Ev + Clone + Send + 'static,
    {
        self.context.spawn({
            let context = self.context.clone();
            let message = message.to_string();

            async move {
                let response = context.request_from_shell(SignerRequest { message }).await;

                match response {
                    SignerResponse::Signature(data) => {
                        context.update_app(make_event(data));
                    }
                }
            }
        });
    }
}

use std::sync::Arc;

use tera::Tera;
use tonic::transport::Channel;

use util_pb::blog_service_client::BlogServiceClient;

pub struct SharedState {
    inner_state: Arc<InnerState>,
}

impl Clone for SharedState {
    fn clone(&self) -> Self {
        Self {
            inner_state: Arc::clone(&self.inner_state),
        }
    }
}

struct InnerState {
    tera: Tera,
    client: BlogServiceClient<Channel>,
}

impl SharedState {
    pub fn new(tera: Tera, client: BlogServiceClient<Channel>) -> Self {
        Self {
            inner_state: Arc::new(InnerState { tera, client }),
        }
    }

    pub fn client(&self) -> BlogServiceClient<Channel> {
        self.inner_state.client.clone()
    }

    pub fn tera(&self) -> &Tera {
        &self.inner_state.tera
    }
}

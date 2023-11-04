use async_trait::async_trait;
use ipc_client::client::{error::Error, message::JsonValue, shared_object::SharedObject};
use tokio::sync::mpsc::UnboundedSender;

use crate::{interface::Interface, task_manager::TaskMessage};

pub struct EmailerObject<I>
where
    I: Interface + Send + Sync + 'static,
{
    _interface: I,
    _tx: UnboundedSender<TaskMessage>,
}

impl<I> EmailerObject<I>
where
    I: Interface + Send + Sync + 'static,
{
    pub fn new(interface: I, tx: UnboundedSender<TaskMessage>) -> Self {
        Self {
            _interface: interface,
            _tx: tx,
        }
    }
}

#[async_trait]
impl<I> SharedObject for EmailerObject<I>
where
    I: Interface + Send + Sync + 'static + Clone,
{
    async fn remote_call(
        &self,
        method: &str,
        param: Option<JsonValue>,
    ) -> Result<JsonValue, Error> {
        log::trace!("Method: {} Param: {:?}", method, param);
        // TODO: Implement the shared object
        Ok(JsonValue::Bool(true))
    }
}

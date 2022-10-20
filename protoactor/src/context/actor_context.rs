use crate::message::{Message, MessageEnvelope};

pub struct ActorContext<M>
where
    M: Message + Send + 'static,
    M::Result: Send,
{
    message: MessageEnvelope<M>,
}

mod actor_context;

pub use actor_context::*;

use crate::message::MessageHeader;

pub trait SenderContext {
    type Message;

    /// [MessageHeader] of the context.
    fn get_headers(&self) -> MessageHeader;
    fn get_message(&self) -> Self::Message;
}

pub trait ReceiverContext {
    fn receive();
}

// pub trait Context: SenderContext + ReceiverContext {}

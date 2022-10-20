mod message_envelope;
mod protos;

pub use message_envelope::*;
#[doc(inline)]
pub use protos::*;

use std::sync::Arc;

// pub trait IsMessage {}

pub trait Message {
    type Result;
}

// impl<T> IsMessage for T where T: Message {}

impl<M> Message for Arc<M>
where
    M: Message,
{
    type Result = M::Result;
}

impl<M> Message for Box<M>
where
    M: Message,
{
    type Result = M::Result;
}

pub trait MessageResponse {}

pub enum SystemMessage {
    Started,
    Stop,
}

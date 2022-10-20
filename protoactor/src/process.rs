mod actor_process;
mod registry;

pub use self::actor_process::*;
pub use self::registry::*;
use crate::message::{Message, Pid, SystemMessage};
use crate::system::ActorSystem;
use std::sync::Arc;

//todo: `fn send_user_message(&self, pid: &Pid, msg: impl Message);`

pub trait Process {
    fn system(&self) -> Arc<ActorSystem>;

    fn send_system_message(&self, pid: &Pid, msg: SystemMessage);

    fn stop(&self, pid: &Pid) {
        self.send_system_message(pid, SystemMessage::Stop)
    }
}

pub trait ReceiverProcess {
    fn send_user_message<M>(&self, pid: &Pid, msg: M)
    where
        M: Message + Sync + 'static,
        M::Result: Sync + 'static;
}

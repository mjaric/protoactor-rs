use crate::message::{Message, Pid, SystemMessage};
use crate::process::{Process, ReceiverProcess};
use crate::system::ActorSystem;
use std::sync::Arc;

/// A [Process] that holds reference to Actor Mailbox
pub struct ActorProcess {
    system: Arc<ActorSystem>,
}

impl Process for ActorProcess {
    #[inline]
    fn system(&self) -> Arc<ActorSystem> {
        self.system.clone()
    }

    fn send_system_message(&self, _pid: &Pid, _msg: SystemMessage) {
        todo!()
    }
}

impl ReceiverProcess for ActorProcess {
    fn send_user_message<M>(&self, _pid: &Pid, _msg: M)
    where
        M: Message + Sync + 'static,
        M::Result: Sync + 'static,
    {
        todo!()
    }
}

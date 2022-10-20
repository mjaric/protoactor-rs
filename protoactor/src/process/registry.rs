use crate::message::Pid;
use std::sync::Arc;

use crate::process::Process;
use std::sync::atomic::AtomicU64;

/// Manages all processes in the actor system (actors, futures, event stream, etc.).
pub struct Registry {
    sequence_id: AtomicU64,
    host_resolvers: Vec<Box<dyn Fn(&Pid) -> Arc<dyn Process>>>,
}

impl Registry {}

#[cfg(test)]
mod tests {}

use crate::process::Registry;
use config::ActorSystemConfig;

pub mod config;

const NO_HOST: &str = "nohost";
const CLIENT: &str = "$client";

pub struct ActorSystem {
    host: String,
    port: i16,
    config: ActorSystemConfig,
    registry: Registry,
}

impl ActorSystem {}

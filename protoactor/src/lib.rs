// #[macro_use]
// extern crate log;

pub mod actor;
pub mod context;
pub mod diagnostics;
pub mod message;
pub mod process;
pub mod system;

#[cfg(test)]
mod tests {
    // use log::LevelFilter;

    // fn setup() {
    //     let _ = env_logger::builder().filter_level(LevelFilter::Trace).is_test(true).try_init();
    // }
}

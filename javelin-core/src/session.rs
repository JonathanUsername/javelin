mod instance;
pub mod manager;
mod transport;

type Event = &'static str;
type AppName = String;
type StreamKey = String;

pub use self::{
    manager::Manager,
    transport::{trigger_channel, Handle, ManagerHandle, ManagerMessage, Message, Watcher},
};

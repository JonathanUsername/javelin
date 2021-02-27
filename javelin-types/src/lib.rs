pub mod data;
pub mod models;
pub mod transport;

pub type Error = Box<dyn std::error::Error>;

pub use self::{
    data::{Metadata, Timestamp},
    transport::{Packet, PacketType},
};

// foreign re-exports
pub use async_trait::async_trait;

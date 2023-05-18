#[allow(dead_code)] // We allow this, since extra constants are not too harmful.
pub(crate) mod constant;

pub(crate) mod dp_event_loop;
pub(crate) mod fragment_assembler;
pub(crate) mod message_receiver;
pub(crate) mod reader;
pub(crate) mod rtps_reader_proxy;
pub(crate) mod rtps_writer_proxy;
pub mod writer;

pub mod message;
pub use message::{Message, MessageBuilder};

pub mod submessage;
pub use submessage::{Submessage, SubmessageBody};

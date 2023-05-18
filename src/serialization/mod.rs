pub mod builtin_data_deserializer;
pub mod builtin_data_serializer;
pub mod cdr_deserializer;
pub mod cdr_serializer;
pub mod error;
pub mod pl_cdr_deserializer;
pub mod pl_cdr_serializer;
pub mod visitors;

pub mod message;
pub mod submessage;

// crate exports
pub use message::*;
pub use submessage::*;
// public exports
pub use cdr_serializer::{to_writer_endian, CDRSerializerAdapter, CdrSerializer};
pub use cdr_deserializer::{deserialize_from_cdr, CDRDeserializerAdapter, CdrDeserializer};
pub use byteorder::{BigEndian, LittleEndian};

pub use crate::dds::traits::serde_adapters::{no_key, with_key};

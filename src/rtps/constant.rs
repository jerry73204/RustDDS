use crate::structure::guid::EntityId;

// EntityIds for built-in readers with secured communication
// See the definition of “Builtin Secure Endpoints” in the Security spec
pub const SECURE_BUILTIN_READER_ENTITY_IDS: &[EntityId] = &[
  EntityId::SPDP_RELIABLE_BUILTIN_PARTICIPANT_SECURE_READER,
  EntityId::SEDP_BUILTIN_PUBLICATIONS_SECURE_READER,
  EntityId::SEDP_BUILTIN_SUBSCRIPTIONS_SECURE_READER,
  EntityId::P2P_BUILTIN_PARTICIPANT_MESSAGE_SECURE_READER,
  EntityId::P2P_BUILTIN_PARTICIPANT_VOLATILE_SECURE_READER,
];

// EntityIds for built-in writers with secured communication
pub const SECURE_BUILTIN_WRITER_ENTITY_IDS: &[EntityId] = &[
  EntityId::SPDP_RELIABLE_BUILTIN_PARTICIPANT_SECURE_WRITER,
  EntityId::SEDP_BUILTIN_PUBLICATIONS_SECURE_WRITER,
  EntityId::SEDP_BUILTIN_SUBSCRIPTIONS_SECURE_WRITER,
  EntityId::P2P_BUILTIN_PARTICIPANT_MESSAGE_SECURE_WRITER,
  EntityId::P2P_BUILTIN_PARTICIPANT_VOLATILE_SECURE_WRITER,
];

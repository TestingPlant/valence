use uuid::Uuid;

use crate::{Decode, Encode, Packet};

#[derive(Copy, Clone, Debug, Encode, Decode, Packet)]
/// If uuid is None, all resource packs are removed. Else, only the resource
/// pack with the given uuid is removed.
pub struct ResourcePackPopS2c(pub Option<Uuid>);
use crate::{Decode, Encode, Packet};

#[derive(Copy, Clone, Debug, Encode, Decode, Packet)]
pub struct ClientTickEndC2s;
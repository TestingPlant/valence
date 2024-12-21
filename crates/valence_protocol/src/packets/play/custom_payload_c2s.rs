use valence_ident::Ident;

use crate::{Bounded, DecodeBytes, Encode, Packet, RawBytes};

pub const MAX_PAYLOAD_SIZE: usize = 32767;

#[derive(Clone, Debug, Encode, DecodeBytes, Packet)]
pub struct CustomPayloadC2s {
    pub channel: Ident,
    pub data: Bounded<RawBytes, MAX_PAYLOAD_SIZE>,
}

use valence_ident::Ident;

use crate::{Bounded, DecodeBytes, Encode, Packet, RawBytes};

const MAX_PAYLOAD_SIZE: usize = 0x100000;

#[derive(Clone, Debug, Encode, DecodeBytes, Packet)]
pub struct CustomPayloadS2c {
    pub channel: Ident,
    pub data: Bounded<RawBytes, MAX_PAYLOAD_SIZE>,
}

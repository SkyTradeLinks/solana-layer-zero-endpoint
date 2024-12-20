use crate::*;

#[event]
pub struct MsgSent {
    pub guid: [u8; 32],
    pub dst_eid: u32,
    pub from: Pubkey,
}

#[event]
pub struct MsgReceived {
    pub guid: [u8; 32],
    pub src_eid: u32,
    pub to: Pubkey,
}

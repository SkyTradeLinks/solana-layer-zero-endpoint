use crate::*;

#[account]
#[derive(InitSpace)]
pub struct MsgStore {
    pub endpoint_program: Pubkey,
    pub bump: u8,

    // configurable
    pub admin: Pubkey,
    pub default_fee_bps: u16,
    pub paused: bool,
    pub pauser: Option<Pubkey>,
    pub unpauser: Option<Pubkey>,
}

#[account]
#[derive(InitSpace)]
pub struct LzReceiveTypesAccounts {
    pub msg_store: Pubkey,
}

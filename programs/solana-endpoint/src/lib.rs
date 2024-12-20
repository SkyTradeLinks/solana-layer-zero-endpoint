use anchor_lang::prelude::*;

// use oapp::endpoint::{
//     self,
//     cpi::accounts::{Clear, ClearCompose, Quote, RegisterOApp, Send, SendCompose, SetDelegate},
//     instructions::{
//         ClearComposeParams, ClearParams, QuoteParams, RegisterOAppParams, SendComposeParams,
//         SendParams, SetDelegateParams,
//     },
//     ConstructCPIContext, MessagingFee, MessagingReceipt, COMPOSED_MESSAGE_HASH_SEED, ENDPOINT_SEED,
//     NONCE_SEED, OAPP_SEED, PAYLOAD_HASH_SEED,
// };

pub mod constants;
pub mod errors;
pub mod events;
pub mod instructions;
pub mod state;

use constants::*;
use errors::*;
use events::*;
use instructions::*;
use state::*;

declare_id!("4UNXLwaPCWP8KgqbxtCH28EWRWJAd84VEVGRqga8FuJq");

#[program]
pub mod solana_endpoint {
    use super::*;

    pub fn initialize(mut ctx: Context<Initialize>, params: InitializeParams) -> Result<()> {
        Initialize::apply(&mut ctx, &params)
    }
}

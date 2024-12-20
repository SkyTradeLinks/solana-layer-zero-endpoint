use crate::*;

use oapp::endpoint::{instructions::SendParams as EndpointSendParams, MessagingReceipt};

#[event_cpi]
#[derive(Accounts)]
#[instruction(params: SendParams)]
pub struct Send<'info> {
    pub signer: Signer<'info>,
    #[account(
        mut,
        seeds = [
            PEER_SEED,
            msg_store.key().as_ref(),
            &params.dst_eid.to_be_bytes()
        ],
        bump = peer.bump
    )]
    pub peer: Account<'info, PeerConfig>,
    #[account(
        mut,
        seeds = [MSG_SEED, b"random"],
        bump = msg_store.bump
    )]
    pub msg_store: Account<'info, MsgStore>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
struct MsgInfo<'a> {
    eth_address: &'a str,
}

impl Send<'_> {
    pub fn apply(ctx: &mut Context<Send>, params: &SendParams) -> Result<MessagingReceipt> {
        // just trigger mint on the other side
        let message = MsgInfo {
            eth_address: &params.eth_wallet,
        };

        let encoded_msg = message.try_to_vec()?;

        let msg_receipt = oapp::endpoint_cpi::send(
            ctx.accounts.msg_store.endpoint_program,
            ctx.accounts.msg_store.key(),
            ctx.remaining_accounts,
            &[MSG_SEED, b"random", &[ctx.accounts.msg_store.bump]],
            EndpointSendParams {
                dst_eid: params.dst_eid,
                receiver: ctx.accounts.peer.peer_address,
                message: encoded_msg.clone(),
                options: ctx
                    .accounts
                    .peer
                    .enforced_options
                    .combine_options(&Some(encoded_msg), &params.options)?,
                native_fee: params.native_fee,
                lz_token_fee: params.lz_token_fee,
            },
        )?;

        emit_cpi!(MsgSent {
            guid: msg_receipt.guid,
            dst_eid: params.dst_eid,
            from: ctx.accounts.msg_store.key(),
        });

        Ok(msg_receipt)
    }
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct SendParams {
    pub dst_eid: u32,
    pub to: [u8; 32],
    pub options: Vec<u8>,
    pub eth_wallet: String,
    pub native_fee: u64,
    pub lz_token_fee: u64,
}

use crate::*;
use oapp::endpoint::{instructions::RegisterOAppParams, ID as ENDPOINT_ID};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init,
        payer = payer,
        space = 8 + MsgStore::INIT_SPACE,
        seeds = [MSG_SEED, b"random"],
        bump
    )]
    pub msg_store: Account<'info, MsgStore>,

    #[account(
        init,
        payer = payer,
        space = 8 + LzReceiveTypesAccounts::INIT_SPACE,
        seeds = [LZ_RECEIVE_TYPES_SEED, msg_store.key().as_ref()],
        bump
    )]
    pub lz_receive_types_accounts: Account<'info, LzReceiveTypesAccounts>,

    pub system_program: Program<'info, System>,
}

impl Initialize<'_> {
    pub fn apply(ctx: &mut Context<Initialize>, params: &InitializeParams) -> Result<()> {
        // Initialize the msg_store
        ctx.accounts.msg_store.endpoint_program =
            if let Some(endpoint_program) = params.endpoint_program {
                endpoint_program
            } else {
                ENDPOINT_ID
            };
        ctx.accounts.msg_store.bump = ctx.bumps.msg_store;
        ctx.accounts.msg_store.admin = params.admin;
        ctx.accounts.msg_store.default_fee_bps = 0;
        ctx.accounts.msg_store.paused = false;
        ctx.accounts.msg_store.pauser = None;
        ctx.accounts.msg_store.unpauser = None;

        // Initialize the lz_receive_types_accounts
        ctx.accounts.lz_receive_types_accounts.msg_store = ctx.accounts.msg_store.key();

        // Register the oapp
        oapp::endpoint_cpi::register_oapp(
            ctx.accounts.msg_store.endpoint_program,
            ctx.accounts.msg_store.key(),
            ctx.remaining_accounts,
            &[MSG_SEED, b"random", &[ctx.bumps.msg_store]],
            RegisterOAppParams {
                delegate: params.admin,
            },
        )
    }
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct InitializeParams {
    pub admin: Pubkey,
    pub endpoint_program: Option<Pubkey>,
}

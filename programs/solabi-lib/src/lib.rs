use anchor_lang::prelude::*;
use solabi::*;
use std::mem::size_of;
declare_id!("J7FFJHrRfgG43HCHZqNJnF7y8GQa5uxu7g4FCTs1jsUu");

#[program]
pub mod solabi_lib {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {

        let from:Address = address!("0x0E9E31E3e3D56D7396A0eCc9233a824AbC45988E");
        let amount:U256 = 34.as_u256();
        msg!("send {} from {}",amount,from);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize <'info> {
    #[account(init,
              payer = signer,
              space=size_of::<Message>() + 8,
              seeds = [],
              bump)]
    pub message: Account<'info, Message>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
    }
#[account]
pub struct Message {
    from: Address,
    amount:U256
}
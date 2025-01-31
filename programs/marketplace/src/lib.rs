use anchor_lang::prelude::*;

declare_id!("Fi1J1rh4vTfiqWDLyfjW18r3teyfeTWwdLWf8JUNyf9x");

#[program]
pub mod marketplace {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

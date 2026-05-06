use anchor_lang::prelude::*;

mod error;
mod seeds;
mod state;
mod instructions;
mod competition_systems;

use instructions::*;
use competition_systems::*;

declare_id!("63yvyYYUHSZyHEKnz4YerXBvZ5VomBwZtLF1XLmSWfbR");

#[program]
pub mod competition_constructor_program {
    use super::*;

    pub fn program_config_init(
        ctx: Context<ProgramConfigInit>,
        args: ProgramConfigInitArgs,
    ) -> Result<()> {
        ProgramConfigInit::
            program_config_init(ctx, args)
    }

    pub fn program_config_authority_update(
        ctx: Context<ProgramConfigUpdate>,
        args: ProgramConfigUpdateArgs,
    ) -> Result<()> {
        ProgramConfigUpdate::
            program_config_authority_update(ctx, args)
    }

    pub fn program_config_creator_key_update(
        ctx: Context<ProgramConfigUpdate>,
        args: ProgramConfigUpdateArgs,
    ) -> Result<()> {
        ProgramConfigUpdate::
            program_config_creator_key_update(ctx, args)
    }

    pub fn program_config_treasury_update(
        ctx: Context<ProgramConfigUpdate>,
        args: ProgramConfigUpdateArgs,
    ) -> Result<()> {
        ProgramConfigUpdate::
            program_config_treasury_update(ctx, args)
    }

    pub fn constructor_create(
        ctx: Context<ConstructorCreate>,
        args: ConstructorCreateArgs,
    ) -> Result<()> {
        ConstructorCreate::
            constructor_create(ctx, args)
    }

    pub fn constructor_authority_update(
        ctx: Context<ConstructorAuthorityUpdate>,
        args: ConstructorAuthorityUpdateArgs,
    ) -> Result<()> {
        ConstructorAuthorityUpdate::
            constructor_authority_update(ctx, args)
    }

    pub fn constructor_creator_key_update(
        ctx: Context<ConstructorUpdate>,
        args: ConstructorUpdateArgs,
        ) -> Result<()> {
        ConstructorUpdate::
            constructor_creator_key_update(ctx, args)
    }

    pub fn constructor_transaction_fee_update(
        ctx: Context<ConstructorUpdate>,
        args: ConstructorTransactionFeeUpdateArgs,
    ) -> Result<()> {
        ConstructorUpdate::
            constructor_transaction_fee_update(ctx, args)
    }
}
use std::borrow::BorrowMut;

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    pubkey::Pubkey,
};

use crate::{instruction::CampaignInstruction, state::CampaignState};

pub struct Processor;

impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let campaign_pda_seed = b"campaign funding seed";
        let unpacked_campaign_instruction =
            CampaignInstruction::unpack_instruction_data(instruction_data)?;

        Ok(match unpacked_campaign_instruction {
            CampaignInstruction::CreateCampaign {
                campaign_amount,
                campaign_description,
            } => {
                let mutable_iter_accounts = &mut accounts.iter();
                // Accounts
                let campaign_account = next_account_info(mutable_iter_accounts)?;

                let campaign_creator_account = next_account_info(mutable_iter_accounts)?;

                let mut campaign_account_state =
                    CampaignState::try_from_slice(&campaign_account.data.borrow())?;

                campaign_account_state.campaign_description = campaign_description;
                campaign_account_state.campaign_amount = campaign_amount;
                campaign_account_state.is_campaign_funded = false;
                campaign_account_state.campaign_owner = *campaign_creator_account.key;

                campaign_account_state.serialize(&mut campaign_account.data.borrow_mut())?;
            }

            CampaignInstruction::CheckFundsForCampaign => {}

            CampaignInstruction::FundCampaign => {}

            CampaignInstruction::WithdrawFundsFromCampaign => {}
        })
    }
}

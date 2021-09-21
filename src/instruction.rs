use std::convert::TryInto;

use solana_program::program_error::ProgramError;

pub enum CampaignInstruction {
    // [writable] campaign account
    // [signer] campaign creator account
    /// Creates a new campaign
    CreateCampaign {
        campaign_amount: u64,
        campaign_description: String,
    },
    /// Funds a created campaign
    FundCampaign,
    /// Check how much funds are remaining to close the request
    CheckFundsForCampaign,
    /// Withdraw all funds from the campaign
    WithdrawFundsFromCampaign,
}

impl CampaignInstruction {
    pub fn unpack_campaign_amount(rest_data: &[u8]) -> Result<u64, ProgramError> {
        let campaign_amount = rest_data
            .get(..8)
            .and_then(|byte_slice| byte_slice.try_into().ok())
            .map(u64::from_le_bytes)
            .ok_or(ProgramError::InvalidInstructionData);

        campaign_amount
    }

    pub fn unpack_campaign_description(rest_data: &[u8]) -> Result<String, ProgramError> {
        let campaign_description_vector = rest_data[9..].to_vec();
        let campaign_description = match String::from_utf8(campaign_description_vector) {
            Ok(desc) => return Ok(desc),
            Err(err) => return Err(ProgramError::InvalidInstructionData),
        };
    }

    pub fn unpack_instruction_data(instruction_data: &[u8]) -> Result<Self, ProgramError> {
        let (campaign_instruction_tag, rest_data) = instruction_data
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;

        Ok(match campaign_instruction_tag {
            0 => CampaignInstruction::CreateCampaign {
                campaign_amount: Self::unpack_campaign_amount(rest_data)?,
                campaign_description: Self::unpack_campaign_description(rest_data)?,
            },
            1 => CampaignInstruction::FundCampaign,
            2 => CampaignInstruction::CheckFundsForCampaign,
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }
}

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct CampaignState {
    pub campaign_owner: Pubkey,
    pub campaign_amount: u64,
    pub campaign_description: String,
    pub is_campaign_funded: bool,
}

use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct UserProfile {
    pub authority: Pubkey,
    pub diary_count: u8,
    pub last_diary_date: String
}

#[account]
#[derive(Default)]
pub struct DiaryAccount {
    pub authority: Pubkey,
    pub content: String,
    pub title: String,
    pub image: String,
    pub date: String,
    pub uuid: String,
}

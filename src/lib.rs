use anchor_lang::prelude::*;

pub mod constant;
pub mod error;
pub mod states;
use crate::{constant::*, error::*, states::*};

declare_id!("GQdZb18RvtuSqoSJ9tE4GRBwjWRMXoMCRumCNc4ZFLC1");

#[program]
pub mod personal_journaling {
    use super::*;
    // Initialize the user
    // Add a user profile to the blockchain
    pub fn initialize_user(ctx: Context<InitializeUser>) -> Result<()> {
        // Initialize user with the default values
        let user_profile: &mut Account<UserProfile> = &mut ctx.accounts.user_profile;
        user_profile.authority = ctx.accounts.authority.key();
        user_profile.diary_count = 0;
        user_profile.last_diary_date = "".to_string();

        Ok(())
    }

    // Add a diary
    pub fn add_diary(
        ctx: Context<AddDiary>,
        user_date: String,
        _content: String,
        _title: String,
        _image: String,
        _uuid: String,
    ) -> Result<()> {
        // Initialize variables
        let diary_account: &mut Account<DiaryAccount> = &mut ctx.accounts.diary_account;
        let user_profile: &mut Account<UserProfile> = &mut ctx.accounts.user_profile;

        require!(
            !(user_profile.last_diary_date == user_date),
            DiaryError::TodayDiaryIsALreadyExist
        );

        if _title.chars().count() > 100 {
            return Err(DiaryError::TitleTooLong.into());
        }

        if _content.chars().count() > 10000 {
            return Err(DiaryError::ContentTooLong.into());
        }

        // Fill the Diary struct with the proper values
        diary_account.authority = ctx.accounts.authority.key();
        diary_account.title = _title.to_string();
        diary_account.content = _content.to_string();
        diary_account.date = user_date.clone();
        diary_account.uuid = _uuid.to_string();
        diary_account.image = _image.to_string();

        // Set last diary date of user profile
        user_profile.last_diary_date = user_date.clone();

        // Increase total diary count
        // check if i can increment it, increment by 1
        user_profile.diary_count = user_profile.diary_count.checked_add(1).unwrap();

        Ok(())
    }

    // Delete a diary
    pub fn remove_diary(ctx: Context<RemoveDiary>, user_date: String) -> Result<()> {
        // Decrease total diary count of user profile
        let user_profile: &mut Account<UserProfile> = &mut ctx.accounts.user_profile;
        user_profile.diary_count = user_profile.diary_count.checked_sub(1).unwrap();

        // Diary PDA already closed in context
        msg!("Diary entry on date {} deleted", user_date.to_string());

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction()]
pub struct InitializeUser<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        seeds = [USER_TAG, authority.key().as_ref()],
        bump,
        payer = authority,
        space = 8 + std::mem::size_of::<UserProfile>(),
    )]
    pub user_profile: Account<'info, UserProfile>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(user_date: String, content: String, title: String, image: String)]
pub struct AddDiary<'info> {
    #[account(
        mut,
        seeds = [USER_TAG, authority.key.as_ref()],
        bump,
        has_one = authority,
    )]
    pub user_profile: Account<'info, UserProfile>,

    #[account(
        init,
        seeds = [DIARY_TAG, authority.key.as_ref(), date_seed(&user_date)],
        bump,
        payer = authority,
        // When you make diary updatable, change it to fixed values(maybe max)
        space = 8 + 32 + (4 + content.len()) + (4 + title.len()) + (4 + image.len()) + (4 + 12) + (4 + 40),
    )]
    pub diary_account: Account<'info, DiaryAccount>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(user_date: String)]
pub struct RemoveDiary<'info> {
    #[account(
        mut,
        seeds = [USER_TAG, authority.key.as_ref()],
        bump,
        has_one = authority,
    )]
    pub user_profile: Account<'info, UserProfile>,

    #[account(
        mut,
        close = authority,
        // Should i add uuid to diary account seed? I want to make users be able to only one diary for each day?
        seeds = [DIARY_TAG, authority.key.as_ref(), date_seed(&user_date)],
        bump,
        has_one = authority
    )]
    pub diary_account: Account<'info, DiaryAccount>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

fn date_seed(date: &str) -> &[u8] {
    let b = date.as_bytes(); // Convert the input string name to a byte slice b
    if b.len() > 32 {
        &b[0..32] // If the length of b is greater than 32, return the first 32 bytes
    } else {
        b // Otherwise, return b as is
    }
}
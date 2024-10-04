use anchor_lang::prelude::*;

#[error_code]
pub enum DiaryError {
    #[msg("You are not authorized to perform this action")]
    Unauthorized,
    #[msg("Not allowed")]
    NotAllowed,
    #[msg("Math operation overflow")]
    MathOverflow,
    #[msg("You have already wrote a diary for today")]
    TodayDiaryIsALreadyExist,
    #[msg("The provided title should be 100 characters long maximum.")]
    TitleTooLong,
    #[msg("The provided content should be 10000 characters long maximum.")]
    ContentTooLong,
}
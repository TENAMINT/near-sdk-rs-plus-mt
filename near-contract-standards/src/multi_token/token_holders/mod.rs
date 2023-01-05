use crate::multi_token::token::TokenId;
use near_sdk::AccountId;

mod token_holders;

pub use token_holders::*;

pub trait MultiTokenHolders {
    /// Get a list of all token holders (with pagination)
    ///
    /// # Arguments:
    /// * `token_id` - ID of the token
    ///
    /// returns: List of [AccountId]s.
    ///
    fn mt_token_holders(&self, token_id: TokenId) -> Vec<AccountId>;
}

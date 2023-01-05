use crate::multi_token::{core::MultiToken, token::TokenId};
use near_sdk::AccountId;

use super::MultiTokenHolders;

impl MultiTokenHolders for MultiToken {
    fn mt_token_holders(&self, token_id: TokenId) -> Vec<AccountId> {
        if let Some(holders_per_token) = &self.holders_per_token {
            if let Some(holders) = holders_per_token.get(&token_id) {
                return holders.iter().collect();
            }
        }

        vec![]
    }
}

// Partial SPL Token v2.0.x declarations inlined to avoid an external dependency on the spl-token crate
solana_sdk::declare_id!("HMGr16f8Ct1Zeb9TGPypt9rPgzCkmhCQB8Not8vwiPW1");

/*
    spl_token::state::Account {
        mint: Pubkey,
        owner: Pubkey,
        amount: u64,
        delegate: COption<Pubkey>,
        state: AccountState,
        is_native: COption<u64>,
        delegated_amount: u64,
        close_authority: COption<Pubkey>,
    }
*/
pub const SPL_TOKEN_ACCOUNT_MINT_OFFSET: usize = 0;
pub const SPL_TOKEN_ACCOUNT_OWNER_OFFSET: usize = 32;

pub mod state {
    const LEN: usize = 165;
    pub struct Account;
    impl Account {
        pub fn get_packed_len() -> usize {
            LEN
        }
    }
}

pub mod native_mint {
    solana_sdk::declare_id!("So11111111111111111111111111111111111111112");

    /*
        Mint {
            mint_authority: COption::None,
            supply: 0,
            decimals: 9,
            is_initialized: true,
            freeze_authority: COption::None,
        }
    */
    pub const ACCOUNT_DATA: [u8; 82] = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
}

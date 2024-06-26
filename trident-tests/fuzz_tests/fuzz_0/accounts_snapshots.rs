use test_anchor::ID as PROGRAM_ID;
use trident_client::anchor_lang::{self, prelude::*};
use trident_client::fuzzing::FuzzingError;
pub struct InitializeSnapshot<'info> {
    pub system_program: Program<'info, System>,
    pub signer: Signer<'info>,
    pub user: Option<Account<'info, test_anchor::User>>,
}
pub struct UpdateSnapshot<'info> {
    pub signer: Signer<'info>,
    pub user: Account<'info, test_anchor::User>,
}
impl<'info> InitializeSnapshot<'info> {
    pub fn deserialize_option(
        accounts: &'info mut [Option<AccountInfo<'info>>],
    ) -> core::result::Result<Self, FuzzingError> {
        let mut accounts_iter = accounts.iter();
        let system_program: anchor_lang::accounts::program::Program<System> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "system_program".to_string(),
            ))?
            .as_ref()
            .map(anchor_lang::accounts::program::Program::try_from)
            .ok_or(FuzzingError::AccountNotFound("system_program".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("system_program".to_string()))?;
        let signer: Signer<'_> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("signer".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::signer::Signer::try_from)
            .ok_or(FuzzingError::AccountNotFound("signer".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("signer".to_string()))?;
        let user: Option<anchor_lang::accounts::account::Account<test_anchor::User>> =
            accounts_iter
                .next()
                .ok_or(FuzzingError::NotEnoughAccounts("user".to_string()))?
                .as_ref()
                .map(|acc| {
                    if acc.key() != PROGRAM_ID {
                        anchor_lang::accounts::account::Account::try_from(acc)
                            .map_err(|_| FuzzingError::CannotDeserializeAccount("user".to_string()))
                    } else {
                        Err(FuzzingError::OptionalAccountNotProvided("user".to_string()))
                    }
                })
                .transpose()
                .unwrap_or(None);
        Ok(Self {
            system_program,
            signer,
            user,
        })
    }
}
impl<'info> UpdateSnapshot<'info> {
    pub fn deserialize_option(
        accounts: &'info mut [Option<AccountInfo<'info>>],
    ) -> core::result::Result<Self, FuzzingError> {
        let mut accounts_iter = accounts.iter();
        let signer: Signer<'_> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("signer".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::signer::Signer::try_from)
            .ok_or(FuzzingError::AccountNotFound("signer".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("signer".to_string()))?;
        let user: anchor_lang::accounts::account::Account<test_anchor::User> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("user".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::account::Account::try_from)
            .ok_or(FuzzingError::AccountNotFound("user".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("user".to_string()))?;
        Ok(Self { signer, user })
    }
}

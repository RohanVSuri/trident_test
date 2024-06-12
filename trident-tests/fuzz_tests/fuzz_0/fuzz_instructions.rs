pub mod test_anchor_fuzz_instructions {
    use crate::accounts_snapshots::*;
    use trident_client::{fuzzing::*, solana_sdk::native_token::LAMPORTS_PER_SOL}; 
    #[derive(Arbitrary, DisplayIx, FuzzTestExecutor, FuzzDeserialize)]
    pub enum FuzzInstruction {
        Initialize(Initialize),
        Update(Update),
    }
    #[derive(Arbitrary, Debug)]
    pub struct Initialize {
        pub accounts: InitializeAccounts,
        pub data: InitializeData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct InitializeAccounts {
        pub system_program: AccountId,
        pub signer: AccountId,
        pub user: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct InitializeData {}
    #[derive(Arbitrary, Debug)]
    pub struct Update {
        pub accounts: UpdateAccounts,
        pub data: UpdateData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct UpdateAccounts {
        pub signer: AccountId,
        pub user: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct UpdateData {
        pub new_id: u64,
        pub verify: u64,
    }
    impl<'info> IxOps<'info> for Initialize {
        type IxData = test_anchor::instruction::Initialize;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = InitializeSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = test_anchor::instruction::Initialize {};
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let signer = fuzz_accounts.signer.get_or_create_account(
                self.accounts.signer,
                client, 
                5 * LAMPORTS_PER_SOL);

            let user = fuzz_accounts.user.get_or_create_account(
                self.accounts.user, 
                client, 
                5 * LAMPORTS_PER_SOL);

            let signers = vec![signer.clone(), user.clone()];

            let acc_meta = test_anchor::accounts::Initialize {
                system_program: SYSTEM_PROGRAM_ID,
                signer: signer.pubkey(),
                user: user.pubkey(),
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for Update {
        type IxData = test_anchor::instruction::Update;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = UpdateSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = test_anchor::instruction::Update {
                new_id: self.data.new_id,
                verify: self.data.verify,
            };
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let signer = fuzz_accounts.signer.get_or_create_account(
                self.accounts.signer,
                client, 
                5 * LAMPORTS_PER_SOL);

            let user = fuzz_accounts.user.get_or_create_account(
                self.accounts.user, 
                client, 
                5 * LAMPORTS_PER_SOL);

            let signers = vec![signer.clone()];
            let acc_meta = test_anchor::accounts::Update {
                signer: signer.pubkey(),
                user: user.pubkey(),
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }

        fn check(
            &self,
            pre_ix: Self::IxSnapshot,
            post_ix: Self::IxSnapshot,
            ix_data: Self::IxData,
        ) -> Result<(), FuzzingError> {
            let pre_user = pre_ix.user;
            let post_user = post_ix.user;
            println!("{}", ix_data.verify);
            if ix_data.verify % 4 != 0 {
                return Err(FuzzingError::DataMismatch);
            }
            Ok(())
        }
    }
    #[doc = r" Use AccountsStorage<T> where T can be one of:"]
    #[doc = r" Keypair, PdaStore, TokenStore, MintStore, ProgramStore"]
    #[derive(Default)]
    pub struct FuzzAccounts {
        signer: AccountsStorage<Keypair>,
        // system_program: AccountsStorage<todo!()>,
        user: AccountsStorage<Keypair>,
    }
}

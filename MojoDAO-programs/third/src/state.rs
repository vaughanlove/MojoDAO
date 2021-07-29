//! Program state
use {
    borsh::{BorshDeserialize, BorshSchema, BorshSerialize},
    solana_program::{program_pack::IsInitialized, pubkey::Pubkey},
};

/// Struct wrapping data and providing metadata
#[derive(Clone, Debug, BorshSerialize, BorshDeserialize, BorshSchema, PartialEq)]
pub struct RecordData {
    /// Struct version, allows for upgrades to the program
    pub version: u8,

    /// The account allowed to update the data
    pub authority: Pubkey,

    /// The data contained by the account, could be anything serializable
    pub data: Data,
}

/// Struct just for data
#[derive(Clone, Debug, Default, BorshSerialize, BorshDeserialize, BorshSchema, PartialEq)]
pub struct Data {
    /// The data contained by the account, could be anything or serializable
    pub total: u32,
    /// duration of the subscription
    pub duration: u32,
    /// cost of the transaction
    pub cost: u32,
    /// tier of the subscription
    pub tier: u8,
}

impl RecordData {
    /// Version to fill in on new created accounts
    pub const CURRENT_VERSION: u8 = 1;

    /// Start of writable account data, after version and authority
    pub const WRITABLE_START_INDEX: usize = 33;
}

impl IsInitialized for RecordData {
    /// Is initialized
    fn is_initialized(&self) -> bool {
        self.version == Self::CURRENT_VERSION
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use solana_program::program_error::ProgramError;
    use borsh::*;

    /// Version for tests
    pub const TEST_VERSION: u8 = 1;
    /// Pubkey for tests
    pub const TEST_PUBKEY: Pubkey = Pubkey::new_from_array([100; 32]);
    /// total for tests
    pub const TEST_TOTAL: u32 = 0;
    /// duration for tests
    pub const TEST_DURATION: u32 = 0;
    /// tier for tests
    pub const TEST_TIER: u8 = 0;
    /// tier for tests
    pub const TEST_COST: u32 = 45;
    /// Data for tests
    pub const TEST_DATA: Data = Data { total: TEST_TOTAL, duration: TEST_DURATION,  cost: TEST_COST, tier: TEST_TIER};
    /// RecordData for tests
    pub const TEST_RECORD_DATA: RecordData = RecordData {
        version: TEST_VERSION,
        authority: TEST_PUBKEY,
        data: TEST_DATA,
    };

    #[test]
    fn serialize_data() {
        let mut expected = vec![TEST_VERSION];
        expected.extend_from_slice(&TEST_PUBKEY.to_bytes());
        expected.extend_from_slice(&TEST_DATA.total.to_le_bytes());
        expected.extend_from_slice(&TEST_DATA.duration.to_le_bytes());
        expected.extend_from_slice(&TEST_DATA.cost.to_le_bytes());
        expected.extend_from_slice(&[TEST_DATA.tier]);
        assert_eq!(TEST_RECORD_DATA.try_to_vec().unwrap(), expected);
        println!("{:?}",RecordData::try_from_slice(&expected).unwrap());
        assert_eq!(
            RecordData::try_from_slice(&expected).unwrap(),
            TEST_RECORD_DATA
        );
    }
}

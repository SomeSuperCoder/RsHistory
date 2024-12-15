pub const ACCOUNT_STORAGE_SIZE: usize = 20 * 1024 * 1024; // 20 MB
pub type AccountStorage = [u8; ACCOUNT_STORAGE_SIZE];

pub struct Account {
    pubkey: String,
    owner: String,
    storage: AccountStorage,    
    atoms: u128
}

pub struct AccountRefBuilder {
    pubkey: String,
    is_signer: bool,
    is_writable: bool
}

pub struct AccountRef<'a> {
    info: AccountInfo,
    storage: Option<&'a mut AccountStorage>,
    balance: Option<&'a mut u128>
}

pub struct AccountInfo {
    pubkey: String,
    owner: String,
    is_signer: bool
}

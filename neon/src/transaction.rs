use crate::account::AccountRefBuilder;

pub struct Transaction {
    body: Body,
    sigs: Vec<Vec<u8>>
}

pub struct Body {
    prev_blockhash: String,
    instructions: Vec<InstructionBuilder>
}

pub struct InstructionBuilder {
    program_id: String,
    accounts: Vec<AccountRefBuilder>
}

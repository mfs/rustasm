use std::collections::HashMap;

#[derive(PartialEq,Eq,Hash)]
pub enum OperandType {
    Imm8,
    Imm16,
    Imm32,
    Imm64,

    Reg8,
    Reg16,
    Reg32,
    Reg64,

    Mem8,
    Mem16,
    Mem32,
    Mem64,
}

#[derive(PartialEq,Eq,Hash)]
pub enum OpCodeType {
    None,
    Type(OperandType, OperandType),
}

#[derive(PartialEq,Eq,Hash)]
pub struct Mnemonic {
    pub mnemonic: &'static str,
    pub mnemonic_type: OpCodeType
}

struct OpCode {
    pub code: Vec<u8>,
    pub length: usize,
}

pub struct OpCodeTable {
    table: HashMap<Mnemonic, OpCode>
}


impl OpCodeTable {
    pub fn new() -> OpCodeTable {
        OpCodeTable {
            table: HashMap::new(),
        }
    }

    pub fn insert(&mut self, m: &'static str, mt: OpCodeType, code: Vec<u8>, l: usize) {
        self.table.insert(
            Mnemonic { mnemonic: m, mnemonic_type: mt },
            OpCode { code: code, length: l},
        );
    }

    pub fn get(&self, mnemonic: &Mnemonic) -> Option<&OpCode> {
        self.table.get(mnemonic)
    }
}

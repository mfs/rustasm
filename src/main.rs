use std::collections::HashMap;

use OperandType::*;

#[derive(PartialEq,Eq,Hash)]
enum OperandType {
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
enum OpCodeType {
    None,
    Type(OperandType, OperandType),
}

#[derive(PartialEq,Eq,Hash)]
struct Mnemonic {
    mnemonic: &'static str,
    mnemonic_type: OpCodeType
}

struct OpCode {
    code: Vec<u8>,
    length: usize
}

struct Assembler {
    _location_counter: u64,
    op_code_table: HashMap<Mnemonic, OpCode>,
    //symbol_table
}

impl Assembler {
    fn new() -> Assembler {
        let mut a = Assembler {
            _location_counter: 0,
            op_code_table: HashMap::new(),
        };

        a.op_code_table.insert(
            Mnemonic { mnemonic: "syscall", mnemonic_type: OpCodeType::None },
            OpCode {code: vec![0x0f, 0x05], length: 2}
        );

        a.op_code_table.insert(
            Mnemonic { mnemonic: "mov", mnemonic_type: OpCodeType::Type(Reg32, Imm32) },
            OpCode {code: vec![0xb8], length: 5}
        );

        a.op_code_table.insert(
            Mnemonic { mnemonic: "mov", mnemonic_type: OpCodeType::Type(Reg64, Imm64) },
            OpCode {code: vec![0x48, 0xbe], length: 10}
        );

        a
    }

    fn list(&self, mnemonic: Mnemonic) {
        let op = self.op_code_table.get(&mnemonic).unwrap();

        println!("mnemonic: {} | op_code: {:?} | length: {}",
                 mnemonic.mnemonic, op.code, op.length);
    }
}


fn main() {

    let assembler = Assembler::new();

    assembler.list(Mnemonic{ mnemonic: "syscall", mnemonic_type: OpCodeType::None });
    assembler.list(Mnemonic{ mnemonic: "mov", mnemonic_type: OpCodeType::Type(Reg32, Imm32) });
    assembler.list(Mnemonic{ mnemonic: "mov", mnemonic_type: OpCodeType::Type(Reg64, Imm64) });

}

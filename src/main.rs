mod symbol_table;
mod opcode_table;

use std::collections::HashMap;
use symbol_table::SymbolTable;
use opcode_table::{OpCodeTable, OpCodeType, Mnemonic};
use opcode_table::OperandType::*;

struct Assembler {
    _location_counter: u64,
    op_code_table: OpCodeTable, //HashMap<Mnemonic, OpCode>,
    symbol_table: SymbolTable,
}

impl Assembler {
    fn new() -> Assembler {
        let mut a = Assembler {
            _location_counter: 0,
            op_code_table: OpCodeTable::new(),
            symbol_table: SymbolTable::new(),
        };

        a.op_code_table.insert("syscall", OpCodeType::None, vec![0x5], 2);
        a.op_code_table.insert("mov", OpCodeType::Type(Reg32, Imm32), vec![0xb8], 5);
        a.op_code_table.insert("mov", OpCodeType::Type(Reg64, Imm64), vec![0x48, 0xbe], 10);

        a
    }

    fn pass1(&mut self) {
        self.list(Mnemonic{ mnemonic: "syscall", mnemonic_type: OpCodeType::None });
        self.list(Mnemonic{ mnemonic: "mov", mnemonic_type: OpCodeType::Type(Reg32, Imm32) });
        self.list(Mnemonic{ mnemonic: "mov", mnemonic_type: OpCodeType::Type(Reg64, Imm64) });
    }

    fn pass2(&mut self) {

    }

    fn list(&self, mnemonic: Mnemonic) {
        let op = self.op_code_table.get(&mnemonic).unwrap();

        println!("mnemonic: {} | op_code: {:?} | length: {}",
                 mnemonic.mnemonic, op.code, op.length);
    }
}


fn main() {

    let mut assembler = Assembler::new();

    assembler.pass1();
    assembler.pass2();


}

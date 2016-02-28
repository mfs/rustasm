use std::collections::HashMap;

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

enum OpCodeType {
    None,
    Type(OperandType, OperandType),
}

struct OpCode {
    code: Vec<u8>,
    code_type: OpCodeType,
    length: usize
}

struct Assembler {
    _location_counter: u64,
    op_code_table: HashMap<&'static str, OpCode>,
    //symbol_table
}

impl Assembler {
    fn new() -> Assembler {
        let mut a = Assembler {
            _location_counter: 0,
            op_code_table: HashMap::new(),
        };

        a.op_code_table.insert("syscall",
            OpCode {code: vec![0x0f, 0x05], code_type: OpCodeType::None, length: 2});

        a
    }

    fn list(&self, mnemonic: &str) {
        let op = self.op_code_table.get(mnemonic).unwrap();

        println!("mnemonic: {} | op_code: {:?} | length: {}", mnemonic, op.code, op.length);
    }
}


fn main() {

    let assembler = Assembler::new();

    assembler.list("syscall");

}

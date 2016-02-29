use std::collections::HashMap;

struct Symbol {
    value: u64,
    //type
}

pub struct SymbolTable {
    table: HashMap<String, Symbol>
}


impl SymbolTable {
    pub fn new() -> SymbolTable {
        SymbolTable {
            table: HashMap::new(),
        }
    }



}

#[macro_use]
extern crate nom;

use std::str;
use nom::{space, is_alphanumeric, line_ending};

// types of input lines these can all have comments too.
// empty
// label
// instruction operands?
// label instructions operands?

#[derive(Debug)]
struct Line<'a> {
    label: Option<&'a str>,
    instruction: Option<&'a str>,
    operand: Option<&'a str>,
    comment: Option<&'a str>
}

impl<'a> Line<'a> {
    fn new(label: Option<&'a [u8]>,
           instruction: Option<&'a [u8]>,
           operands: Option<&'a [u8]>,
           comment: Option<&'a [u8]>) -> Line<'a> {
        Line {
            label: label.map(|x| str::from_utf8(x).unwrap()),
            instruction: instruction.map(|x| str::from_utf8(x).unwrap()),
            operand: operands.map(|x| str::from_utf8(x).unwrap().trim()),
            comment: comment.map(|x| str::from_utf8(x).unwrap().trim()),
        }
    }

}

named!( line_asm<Line>,
    alt!(line_instruction_operands | line_label | label_instruction_operands)
);

named!( line_instruction_operands<Line>,
    chain!(
        instruction: instruction ~
        space? ~
        operands: operands? ~
        comment: comment ~
        line_ending,
        || { Line::new( None,
                        Some(instruction),
                        operands,
                        Some(comment)) }
    )
);

named!( line_label<Line>,
    chain!(
        label: label ~
        space ~
        comment: comment ~
        line_ending,
        || { Line::new( Some(label),
                        None,
                        None,
                        Some(comment)) }
    )
);

named!( label_instruction_operands<Line>,
    chain!(
        label: label ~
        space? ~
        instruction: instruction ~
        space? ~
        operands: operands? ~
        comment: comment ~
        line_ending,
        || { Line::new( Some(label),
                        Some(instruction),
                        operands,
                        Some(comment)) }
    )
);

// comments start with ';' and go to the end of the line
named!( comment, preceded!( char!( ';' ), take_until!( b"\n" ) ) );

// instruction TODO need to add all instructions
named!( instruction, alt!( tag!( "mov" ) | tag!( "syscall" ) ) );

// labels TODO add optional ':'
named!( label, take_while!( is_alphanumeric ) );

// operands TODO need to handle spaces, strings, etc
named!( operands, take_until_either!( b";\n" ) );

fn main() {

    let lines = vec![
        "start                        ; (1) label only\n",
        "start mov    st1,st0         ; (2) this sets st1 := st0\n",
        "start syscall                ; (3) perform syscall\n",
        "syscall                      ; (4) perform syscall\n",
    ];

    for line in lines {
        let asm = line_asm(line.as_bytes());
        println!("{:#?}", asm);
    }

}


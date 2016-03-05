#[macro_use]
extern crate nom;

use std::str;
use nom::{space, is_alphanumeric, line_ending};

// types of input lines these can all have comments too.
// empty
// label
// instruction operands?
// label instructions operands?

named!( comment, preceded!( char!( ';' ), take_until!( b"\n" ) ) );

named!( instruction, alt!( tag!( "mov" ) | tag!( "syscall" ) ) );

named!( label, take_while!( nom::is_alphanumeric ) );

named!( operands, take_until_either!( b";\n" ) );

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

fn main() {

    let line = b"start mov    st1,st0         ; this sets st1 := st1 + st0\n";

    let b = line_asm(line);

    println!("{:#?}", b);

}


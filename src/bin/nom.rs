#[macro_use]
extern crate nom;

use std::str;
use nom::{space, is_alphanumeric};

named!( comment, preceded!( char!( ';' ), take_until!( b"\n" ) ) );

named!( instruction, alt!( tag!( "mov" ) | tag!( "syscall" ) ) );

named!( label, take_while!( nom::is_alphanumeric ) );

named!( operands, take_until_either!( b";\n" ) );

#[derive(Debug)]
struct Line<'a> {
    label: Option<&'a str>,
    instruction: Option<&'a str>,
    operand: Option<String>,
    comment: Option<String>
}

impl<'a> Line<'a> {
    fn new(label: Option<&'a str>, instruction: Option<&'a str>) -> Line<'a> {
        Line {
            label: label,
            instruction: instruction,
            operand: None,
            comment: None,
        }
    }

}

named!( line_asm<Line>,
    chain!(
        label: label ~
        space? ~
        i: instruction,
        || { Line::new( Some(str::from_utf8(label).unwrap() ),
                        Some(str::from_utf8(i).unwrap())) }
    )
);

fn main() {

    let line = b"start mov    st1,st0         ; this sets st1 := st1 + st0\n";

    let b = line_asm(line);

    println!("{:?}", b);

}


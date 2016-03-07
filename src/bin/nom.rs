#[macro_use]
extern crate nom;

use std::str;
use nom::{space, is_alphanumeric, line_ending};

// types of input lines these can all have comments too.
// empty
// label
// instruction operands?
// label instructions operands?

#[derive(Debug, PartialEq, Eq)]
struct Line<'a> {
    label: Option<&'a str>,
    instruction: Option<&'a str>,
    operand: Option<Operand>,
    comment: Option<&'a str>
}

impl<'a> Line<'a> {
    fn new(label: Option<&'a [u8]>,
           instruction: Option<&'a [u8]>,
           operands: Option<Operand>,
           comment: Option<&'a [u8]>) -> Line<'a> {
        Line {
            label: label.map(|x| str::from_utf8(x).unwrap()),
            instruction: instruction.map(|x| str::from_utf8(x).unwrap()),
            operand: operands, //.map(|x| str::from_utf8(x).unwrap().trim()),
            comment: comment.map(|x| str::from_utf8(x).unwrap().trim()),
        }
    }

}

#[derive(Debug, PartialEq, Eq)]
enum Operand {
    Register(String),
    RegisterPair(String, String),
    // strings, literals, etc
}

enum Register {
   RAX, RBX, RCX, RDX, RBP, RSP, RSI, RDI, R8, R9, R10, R11, R12, R13, R14, R15
}

impl Register {
   fn from_bytes(x: &[u8]) -> Register {
      match x {
         b"rax" => Register::RAX,
         b"rbx" => Register::RBX,
         b"rcx" => Register::RCX,
         b"rdx" => Register::RDX,
         b"rbp" => Register::RBP,
         b"rsp" => Register::RSP,
         b"rsi" => Register::RSI,
         b"rdi" => Register::RDI,
         b"r8"  => Register::R8,
         b"r9"  => Register::R9,
         b"r10" => Register::R10,
         b"r11" => Register::R11,
         b"r12" => Register::R12,
         b"r13" => Register::R13,
         b"r14" => Register::R14,
         b"r15" => Register::R15,
         _      => panic!("Unknown register."),
      }
   }
}

///////////////////////////////////////////////////////////////////////
// top level parser
///////////////////////////////////////////////////////////////////////

named!( line_asm<Line>,
    alt!( line_comment |
          line_instruction_operands |
          line_label |
          line_label_instruction_operands
    )
);

///////////////////////////////////////////////////////////////////////
// per line parsers
///////////////////////////////////////////////////////////////////////

named!( line_comment<Line>,
    chain!(
        space? ~
        comment: comment ~
        line_ending,
        || { Line::new( None,
                        None,
                        None,
                        Some(comment)) }
    )
);

named!( line_instruction_operands<Line>,
    chain!(
        instruction: instruction ~
        space? ~
        operands: operands? ~
        space? ~
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

named!( line_label_instruction_operands<Line>,
    chain!(
        label: label ~
        space? ~
        instruction: instruction ~
        space? ~
        operands: operands? ~
        space? ~
        comment: comment ~
        line_ending,
        || { Line::new( Some(label),
                        Some(instruction),
                        operands,
                        Some(comment)) }
    )
);

///////////////////////////////////////////////////////////////////////
// intermediate parsers
///////////////////////////////////////////////////////////////////////

// operands TODO need to handle spaces, strings, etc
named!( operands<Operand>,
    alt!(operand_register_pair | operand_one_register)
);

named!(operand_one_register<Operand>,
   chain!(register, || Operand::Register("Test".to_string()))
);

named!(operand_register_pair<Operand>,
   chain!(
       r0: register ~
       space? ~
       char!(',') ~
       space? ~
       r1: register,
       || Operand::RegisterPair(String::from_utf8_lossy(r0).into_owned(),
                                String::from_utf8_lossy(r1).into_owned()))
);

///////////////////////////////////////////////////////////////////////
// base parsers
///////////////////////////////////////////////////////////////////////

// comments start with ';' and go to the end of the line
named!( comment, preceded!( char!( ';' ), take_until!( b"\n" ) ) );

// instruction TODO need to add all instructions
named!( instruction, alt!( tag!( "mov" ) | tag!( "syscall" ) ) );

// labels TODO don't allow numeric first char
named!( label, terminated!(take_while!( is_alphanumeric ), opt!(char!(':'))) );

// registers TODO segment, xmss, etc.
named!(register, alt!(
    tag!(b"rax") | tag!(b"rbx") | tag!(b"rcx") | tag!(b"rdx") | tag!(b"rbp") |
    tag!(b"rsp") | tag!(b"rsi") | tag!(b"rdi") | tag!(b"r8") | tag!(b"r9") |
    tag!(b"r10") | tag!(b"r11") | tag!(b"r12") | tag!(b"r13") | tag!(b"r14") |
    tag!(b"r15")
));


fn main() {

    let lines = vec![
        "; comment only line\n",
        "start                        ; (1) label only\n",
        "start mov    st1,st0         ; (2) this sets st1 := st0\n",
        "start: syscall                ; (3) perform syscall\n",
        "mov    st1,st0               ; (4) this sets st1 := st0\n",
        "syscall                      ; (5) perform syscall\n",
    ];

    for line in lines {
        let asm = line_asm(line.as_bytes());
        println!("{:#?}", asm);
    }

}

#[cfg(test)]
mod tests {
    use nom::IResult;
    use super::{Operand, Line, line_asm};
    fn wrap_done<'a>(label: Option<&'a [u8]>,
                 instructions: Option<&'a [u8]>,
                 operand: Option<Operand>,
                 comment: Option<&'a [u8]>) -> IResult<&'a [u8], Line<'a> > {
        IResult::Done(&b""[..], Line::new( label, instructions, operand, comment))
    }

    #[test]
    fn test_comment() {
        assert_eq!(line_asm(b"; a single comment\n"),
                   wrap_done(None, None, None, Some(b"a single comment")));
    }

    #[test]
    fn test_comment_leading_whitespace() {
        assert_eq!(line_asm(b" \t ; a single comment\n"),
                   wrap_done(None, None, None, Some(b"a single comment")));
    }

    #[test]
    fn test_instruction() {
        assert_eq!(line_asm(b"syscall ; single instruction\n"),
                   wrap_done(None, Some(b"syscall"), None, Some(b"single instruction") ));
    }

    #[test]
    fn test_instruction_operand() {
        assert_eq!(line_asm(b"mov rax,rbx ; instruction\n"),
                   wrap_done(None, Some(b"mov"),
                             Some(Operand::RegisterPair("rax".to_string(), "rbx".to_string())),
                             Some(b"instruction") ));
    }

    #[test]
    fn test_label_instruction() {
        assert_eq!(line_asm(b"start: syscall ; instruction\n"),
                   wrap_done(Some(b"start"), Some(b"syscall"), None, Some(b"instruction") ));
    }

    #[test]
    fn test_label_instruction_operand() {
        assert_eq!(line_asm(b"start: mov rax,rbx ; instruction\n"),
                   wrap_done(Some(b"start"), Some(b"mov"),
                             Some(Operand::RegisterPair("rax".to_string(), "rbx".to_string())),
                             Some(b"instruction") ));
    }
}

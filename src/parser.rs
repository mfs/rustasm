
use std::str;
use std::str::FromStr;
use nom::{alpha, is_alphabetic, digit, space, is_alphanumeric, line_ending};


// Line
// |- Directive
// |- Source
// |- Blank
// |- ...

#[derive(Debug, PartialEq, Eq)]
pub enum Line<'a> {
    Source(Source<'a>),
    Directive(Directive),
    Blank,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Source<'a> {
    label: Option<&'a str>,
    instruction: Option<&'a str>,
    operand: Option<Operand>,
    comment: Option<&'a str>
}

impl<'a> Source<'a> {
    fn new(label: Option<&'a [u8]>,
           instruction: Option<&'a [u8]>,
           operands: Option<Operand>,
           comment: Option<&'a [u8]>) -> Source<'a> {
        Source {
            label: label.map(|x| str::from_utf8(x).unwrap()),
            instruction: instruction.map(|x| str::from_utf8(x).unwrap()),
            operand: operands,
            comment: comment.map(|x| str::from_utf8(x).unwrap().trim()),
        }
    }

}

#[derive(Debug, PartialEq, Eq)]
pub enum Directive {
    Section(String),
    Global(String),
    Data(String, Option<String>), // Optional label
}

#[derive(Debug, PartialEq, Eq)]
enum Operand {
    Register(Register),
    RegisterPair(Register, Register),
    RegisterImmediate(Register, u64),
    RegisterSymbol(Register, String),
    // strings, literals, etc
}

#[derive(Debug, PartialEq, Eq)]
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

named!( pub line<Line>,
        alt!(
            chain!(space? ~ line_ending, || { Line::Blank } ) |
            chain!(d: directive, || { Line::Directive(d) } ) |
            chain!(l: source, || { Line::Source(l) } )
        )
);

named!( source<Source>,
    alt!( line_comment |
          line_instruction_operands |
          line_label |
          line_label_instruction_operands
    )
);

named!( directive<Directive>,
    alt!( directive_section |
          directive_global |
          directive_data
    )
);


///////////////////////////////////////////////////////////////////////
// per line parsers
///////////////////////////////////////////////////////////////////////

named!( directive_section<Directive>,
    chain!(
        space? ~
        tag!("section") ~
        space ~
        s: section ~
        space? ~
        line_ending,
        || {  Directive::Section(String::from_utf8_lossy(s).into_owned()) }
    )
);

named!( directive_data<Directive>,
    chain!(
        space? ~
        l: label? ~
        space ~
        tag!("db") ~ // TODO: add dd, dq, etc
        space ~
        data: delimited!(tag!("\""), take_until!("\""), tag!("\"") ) ~
        space? ~
        line_ending,
        || {  Directive::Data(String::from_utf8_lossy(data).into_owned(),
                              l.map( |x| String::from_utf8_lossy(x).into_owned()  )) }
    )
);

named!( directive_global<Directive>,
    chain!(
        space? ~
        tag!("global") ~
        space ~
        l: label ~
        space? ~
        line_ending,
        || {  Directive::Global(String::from_utf8_lossy(l).into_owned()) }
    )
);

named!( line_comment<Source>,
    chain!(
        space? ~
        comment: comment ~
        line_ending,
        || { Source::new( None,
                        None,
                        None,
                        Some(comment)) }
    )
);

named!( line_instruction_operands<Source>,
    chain!(
        space? ~
        instruction: instruction ~
        space? ~
        operands: operands? ~
        space? ~
        comment: comment? ~
        line_ending,
        || { Source::new( None,
                        Some(instruction),
                        operands,
                        comment) }
    )
);

named!( line_label<Source>,
    chain!(
        space? ~
        label: label ~
        space? ~
        comment: comment? ~
        line_ending,
        || { Source::new( Some(label),
                        None,
                        None,
                        comment) }
    )
);

named!( line_label_instruction_operands<Source>,
    chain!(
        space? ~
        label: label ~
        space? ~
        instruction: instruction ~
        space? ~
        operands: operands? ~
        space? ~
        comment: comment ~
        line_ending,
        || { Source::new( Some(label),
                        Some(instruction),
                        operands,
                        Some(comment)) }
    )
);

///////////////////////////////////////////////////////////////////////
// intermediate parsers
///////////////////////////////////////////////////////////////////////

named!( operands<Operand>,
    alt!(operand_register_pair |
         operand_register_immediate |
         operand_register_label |
         operand_register)
);

named!(operand_register<Operand>,
    chain!(r: register, || Operand::Register(Register::from_bytes(r)))
);

named!(operand_register_pair<Operand>,
    chain!(
        r0: register ~
        space? ~
        char!(',') ~
        space? ~
        r1: register,
        || Operand::RegisterPair(Register::from_bytes(r0),
                                 Register::from_bytes(r1)))
);

named!(operand_register_immediate<Operand>,
    chain!(
        r: register ~
        space? ~
        char!(',') ~
        space? ~
        i: integer,
        || Operand::RegisterImmediate(Register::from_bytes(r), i)
    )
);

named!(operand_register_label<Operand>,
    chain!(
        r: register ~
        space? ~
        char!(',') ~
        space? ~
        l: label,
        || Operand::RegisterSymbol(Register::from_bytes(r),
                                   String::from_utf8_lossy(l).into_owned())
    )
);

///////////////////////////////////////////////////////////////////////
// base parsers
///////////////////////////////////////////////////////////////////////

// comments start with ';' and go to the end of the line
named!( comment, preceded!( char!( ';' ), take_until!( b"\n" ) ) );

// instruction TODO need to add all instructions
named!( instruction, alt!( tag!( "mov" ) | tag!( "syscall" ) ) );

// labels TODO still need to allow other chars in take_while
named!(label,
    terminated!(
        recognize!(
            preceded!(
                alt!(alpha | tag!(".") | tag!("_") | tag!("?")),
                take_while!( is_alphanumeric ))),
        opt!(tag!(":"))));

named!(section,
    recognize!(
        preceded!(
            opt!(tag!(".")),
            take_while!(is_alphabetic))));

// integer - base 10
named!( integer<u64>,
    map_res!(
        map_res!(
            digit,
            str::from_utf8
        ),
        FromStr::from_str
    )
);

// registers TODO segment, xmss, etc.
named!(register, alt!(
    tag!(b"rax") | tag!(b"rbx") | tag!(b"rcx") | tag!(b"rdx") | tag!(b"rbp") |
    tag!(b"rsp") | tag!(b"rsi") | tag!(b"rdi") | tag!(b"r8") | tag!(b"r9") |
    tag!(b"r10") | tag!(b"r11") | tag!(b"r12") | tag!(b"r13") | tag!(b"r14") |
    tag!(b"r15")
));


#[cfg(test)]
mod tests {
    use nom::IResult;
    use super::{Register, Operand, Source, source};
    fn wrap_done<'a>(label: Option<&'a [u8]>,
                 instructions: Option<&'a [u8]>,
                 operand: Option<Operand>,
                 comment: Option<&'a [u8]>) -> IResult<&'a [u8], Source<'a> > {
        IResult::Done(&b""[..], Source::new( label, instructions, operand, comment))
    }

    #[test]
    fn test_comment() {
        assert_eq!(source(b"; a single comment\n"),
                   wrap_done(None, None, None, Some(b"a single comment")));
    }

    #[test]
    fn test_comment_leading_whitespace() {
        assert_eq!(source(b" \t ; a single comment\n"),
                   wrap_done(None, None, None, Some(b"a single comment")));
    }

    #[test]
    fn test_instruction() {
        assert_eq!(source(b"syscall ; single instruction\n"),
                   wrap_done(None, Some(b"syscall"), None, Some(b"single instruction") ));
    }

    #[test]
    fn test_instruction_operand() {
        assert_eq!(source(b"mov rax,rbx ; instruction\n"),
                   wrap_done(None, Some(b"mov"),
                             Some(Operand::RegisterPair(Register::RAX, Register::RBX)),
                             Some(b"instruction") ));
    }

    #[test]
    fn test_label_instruction() {
        assert_eq!(source(b"start: syscall ; instruction\n"),
                   wrap_done(Some(b"start"), Some(b"syscall"), None, Some(b"instruction") ));
    }

    #[test]
    fn test_label_instruction_operand() {
        assert_eq!(source(b"start: mov rax,rbx ; instruction\n"),
                   wrap_done(Some(b"start"), Some(b"mov"),
                             Some(Operand::RegisterPair(Register::RAX, Register::RBX)),
                             Some(b"instruction") ));
    }

    #[test]
    fn test_space_instruction() {
        assert_eq!(source(b"    syscall\n"),
                   wrap_done(None, Some(b"syscall"), None, None ));
    }
}

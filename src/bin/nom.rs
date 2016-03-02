#[macro_use]
extern crate nom;

named!( comment, preceded!( char!( ';' ), take_until!( b"\n" ) ) );

named!( instruction, alt!( tag!( "mov" ) | tag!( "syscall" ) ) );

named!( label, take_while!( nom::is_alphanumeric ) );

named!( operands, take_until_either!( b";\n" ) );

fn main() {

    let line = b"start mov    st1,st0         ; this sets st1 := st1 + st0\n";

    let b = label(line);

    println!("{:?}", b);

}


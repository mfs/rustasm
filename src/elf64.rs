type Elf64Addr = u64;
type Elf64Off = u64;
type Elf64Half = u16;
type Elf64Word = u32;
type Elf64Sword = i32;
type Elf64Xword = u64;
type Elf64Sxword = i64;

struct FileHeader {
    ident: [u8; 16],
    etype: Elf64Half,
    machine: Elf64Half,
    version: Elf64Word,
    entry: Elf64Addr,
    phoff: Elf64Off,
    shoff: Elf64Off,
    flags: Elf64Word,
    ehsize: Elf64Half,
    phentsize: Elf64Half,
    phnum: Elf64Half,
    shentsize: Elf64Half,
    shnum: Elf64Half,
    shstrndx: Elf64Half,
}

struct SectionHeader {
    name: Elf64Word,
    stype: Elf64Word,
    flags: Elf64Xword,
    addr: Elf64Addr,
    offset: Elf64Off,
    size: Elf64Xword,
    link: Elf64Word,
    info: Elf64Word,
    addralign: Elf64Xword,
    entsize: Elf64Xword,
}

struct ProgramHeader {
    stype: Elf64Word,
    flags: Elf64Word,
    offset: Elf64Off,
    vaddr: Elf64Addr,
    paddr: Elf64Addr,
    filesz: Elf64Xword,
    memsz: Elf64Xword,
    align: Elf64Xword,
}

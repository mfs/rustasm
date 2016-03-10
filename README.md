## RUSTASM

[![Build Status](https://travis-ci.org/mfs/rustasm.svg?branch=master)](https://travis-ci.org/mfs/rustasm)

I'm currently writing an [Intel 4004 emulator][box] and have started writing an
assembler for it. Unfortunately while I have written a compiler before I have
not written an assembler. I was going to just jump straight into it however have
decided to do some research first. I'm reading [Assemblers and Loaders][asl] by
David Salomon. I should be able to write a simple amd64 assembler after reading
the first two or three chapters. I'll leave out macros for now.

Currently very much a work in progress. Have decided to use [NOM][nom] for
parsing which I'm really enjoying the more I get the hang of it.

[box]: https://github.com/mfs/box
[asl]: http://www.davidsalomon.name/assem.advertis/AssemAd.html
[nom]: https://github.com/Geal/nom

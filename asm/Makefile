all: hello_world

hello_world.o: hello_world.asm
	nasm -f elf64 -l hello_world.lst hello_world.asm

hello_world: hello_world.o
	ld hello_world.o -o hello_world

.PHONY: clean
clean:
	rm -rf hello_world hello_world.lst hello_world.o

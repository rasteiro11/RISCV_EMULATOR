default:
	rm add-addi
	riscv64-linux-gnu-as add-addi.s -o add-addi.o
	riscv64-linux-gnu-gcc-9 -o add-addi add-addi.o -nostdlib -static

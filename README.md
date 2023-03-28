# RISCV_EMULATOR
A risc-v emulator made using rust for a computer architecture class

- Features
  - emulate a risc-v architecture
  - run code in emulator

- How to run
  - Compile to assembly to binary
  ```
   make compile
  ```
  - Compile rust emulator
  ```
    cargo build <program_name>
  ```
  - Run emulator with binary compiled program
  ```
    ./<program_name> add-addi.bin
  ```

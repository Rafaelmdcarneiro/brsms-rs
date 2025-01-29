# brsms-rs

First, I know that the Sega Master System uses the Zilog Z80 processor, so the core of the emulator will be accurately emulating the Z80.
The Z80 has a lot of instructions, like around 500 opcodes when you consider all the variations.
That's going to be a lot of work. The Z80 has registers like A, B, C, D, E, H, L, and the flags register.
Also, there are 16-bit registers like AF, BC, DE, HL, IX, IY, SP, and PC. So I need to model these registers in the emulator.

In Rust, I can create a struct for the CPU state. Each register can be a u8 or u16. For example, A is 8-bit, AF is 16-bit (though the lower 8 bits are the flags).
The program counter (PC) and stack pointer (SP) are 16-bit. Then there are the interrupt vectors and flip-flops for interrupts.
Developing a complete Sega Master System emulator with all Z80 instructions is a great challenge!

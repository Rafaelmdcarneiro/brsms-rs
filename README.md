# brsms-rs

First, I know that the Sega Master System uses the Zilog Z80 processor, so the core of the emulator will be accurately emulating the Z80.
The Z80 has a lot of instructions, like around 500 opcodes when you consider all the variations.
That's going to be a lot of work. The Z80 has registers like A, B, C, D, E, H, L, and the flags register.
Also, there are 16-bit registers like AF, BC, DE, HL, IX, IY, SP, and PC. So I need to model these registers in the emulator.

In Rust, I can create a struct for the CPU state. Each register can be a u8 or u16. For example, A is 8-bit, AF is 16-bit (though the lower 8 bits are the flags).
The program counter (PC) and stack pointer (SP) are 16-bit. Then there are the interrupt vectors and flip-flops for interrupts. Maybe something like this:

struct Z80 {
a: u8,
f: Flags,
b: u8,
c: u8,
d: u8,
e: u8,
h: u8,
l: u8,
pc: u16,
sp: u16,
ix: u16,
iy: u16,
// ... other registers and state
}

But handling the flags (F register) could be better as a separate struct.
The flags are Sign, Zero, Half Carry, Parity/Overflow, Negative, and Carry.
So maybe a bitfield or a struct with bools? Or represent them as bits in a u8.
For example:

struct Flags {
carry: bool,
zero: bool,
sign: bool,
parity: bool,
// etc.
}

Alternatively, using a u8 where each bit represents a flag.
The Z80 flags are arranged as follows: Bit 7 is S, 6 is Z, 5 is Y (undocumented), 4 is H, 3 is X (undocumented), 2 is P/V, 1 is N, 0 is C.
So maybe the F register is a u8, and we have helper functions to set and check the flags.
For example, the carry flag is bit 0, zero is bit 6, etc.
The CPU struct would have registers, and then functions to execute instructions.
Each instruction is an opcode, which can be 1 to 4 bytes long.
The main loop would fetch the opcode at PC, decode it, execute the corresponding operation, update PC and registers, and handle cycles.
The Z80 has prefixed instructions like CB, ED, DD, FD for extended opcodes. So the decoding needs to handle these prefixes.

For example, when the opcode is 0xCB, the next byte is another opcode for bit operations.
Similarly, DD and FD prefixes select IX and IY registers for certain operations. This complicates the instruction decoding.

So perhaps a lookup table with functions for each opcode.
But in Rust, using an array of function pointers might be tricky.
Alternatively, a big match statement for each opcode.
But with 256 base opcodes, plus 256 for each prefix, that's a lot.
Maybe using a macro to generate the match arms or some code generation.

But for the sake of time, maybe just outline the structure. For example, in the CPU implementation:

impl Z80 {
fn execute(&mut self, memory: &mut Memory) {
let opcode = self.fetch_byte(memory);
match opcode {
0x00 => self.nop(),
0x01 => self.ld_bc_nn(memory),
0x02 => self.ld_bc_a(),
// ... and so on for all opcodes
_ => panic!("Unimplemented opcode: 0x{:02X}", opcode),
}
}
}

Each method like nop(), ld_bc_nn(), etc., would implement the corresponding instruction.
But this could get very large. Maybe using a macro to generate the match arms and function stubs.

The Master System has 8KB of RAM, and the cartridge ROM can be up to 48KB, but memory mapping is involved.
The Z80 has a 16-bit address bus, so 64KB of address space.
The Master System's memory map includes the cartridge ROM at 0x0000-0xBFFF, RAM at 0xC000-0xFFFF, but there are mirrors and other hardware registers.
So the Memory struct needs to handle reads and writes to different regions, possibly with bank switching.

But for the initial emulator, maybe start with a flat memory model, then add banking later.
Or at least handle the basic memory regions.
So the Memory struct could have a buffer for ROM and RAM. For example:

struct Memory {
rom: Vec<u8>,
ram: [u8; 8192], // 8KB RAM
}
impl Memory {
fn read_byte(&self, addr: u16) -> u8 {
match addr {
0x0000..=0xBFFF => self.rom[addr as usize],
0xC000..=0xFFFF => self.ram[(addr - 0xC000) as usize % 8192],
_ => 0,
}
}
fn write_byte(&mut self, addr: u16, value: u8) {
if let 0xC000..=0xFFFF = addr {
self.ram[(addr - 0xC000) as usize % 8192] = value;
}
// ROM is read-only, so writes to other addresses are ignored
}
}

The actual Master System has more complex memory mapping with mirrors and hardware registers for things like VDP, PSG, etc.
So in the future, the memory read/write functions would need to check if the address is for hardware I/O and handle those appropriately.

Then there's the video processing. The Master System uses a VDP (Video Display Processor) which is a TMS9918 derivative.
Emulating the VDP is a big task on its own. It handles sprites, tiles, scrolling, etc. But for the initial emulator, maybe focus on getting the CPU running, then add the VDP later.

Similarly, input handling and audio (SN76489 PSG) would be needed, but again, maybe start with CPU and memory.
So, to start, the emulator needs to load a ROM into memory, initialize the CPU registers, and start executing instructions. The CPU loop would fetch, decode, execute, and handle timing.

The Z80 instructions take a certain number of cycles, which affects timing with other components like the VDP.
But for now, maybe ignore cycle counting and just execute instructions as fast as possible. But for accurate emulation, cycle counts are important for synchronizing with other hardware.

But the user asked for all Z80 instructions implemented. So the priority is to implement each instruction correctly.
The Z80 instruction set includes operations like LD (load), INC, DEC, ADD, ADC, SUB, SBC, AND, OR, XOR, CP, PUSH, POP, JP, JR, CALL, RET, RST, IN, OUT, and many more.
Each with various addressing modes: immediate, register, indirect (HL), indexed (IX+d, IY+d), etc.
Implementing each instruction requires handling the correct registers, flags, and memory accesses. For example, LD BC, nn would load the 16-bit immediate value into BC.
So in code:

fn ld_bc_nn(&mut self, memory: &mut Memory) {
let lo = self.fetch_byte(memory) as u16;
let hi = self.fetch_byte(memory) as u16;
self.b = hi as u8;
self.c = lo as u8;
// Or combine them as (hi << 8) | lo and set bc?
// Wait, BC is made up of B and C registers. So BC is (B << 8) | C.
// So fetching two bytes: first is high (B), second is low (C).
// So the code above is correct.
}

The Z80 is little-endian for 16-bit immediate values. So when you have an instruction like LD BC, nn, the first byte is the low byte, then the high byte. Wait, no: Z80 is little-endian, so the first byte is the low part of the 16-bit value.
For example, if the opcode is 0x01 0x34 0x12, then BC would be 0x1234. So the first byte after the opcode is 0x34 (low), then 0x12 (high). So BC is set to (B = 0x12, C = 0x34).
The code should read the low byte first, then the high byte.

So for LD BC, nn:
let low = self.fetch_byte(memory);
let high = self.fetch_byte(memory);
self.b = high;
self.c = low;
But then BC as a 16-bit register would be (high << 8) | low. Wait, but BC is made up of B and C. So yes, B is high, C is low. So the code is correct.
But in the CPU struct, maybe having separate registers for B and C, or having a bc() method that combines them.
Alternatively, storing BC as a u16, but that might complicate individual access to B and C. For example, if BC is stored as a u16, then to get B you do (bc >> 8) as u8, and C is (bc & 0xFF) as u8. But when you need to modify B or C individually, you have to split the u16. Maybe it's easier to store them as separate u8 registers. But then, for 16-bit operations, combining them as needed.
But how to handle 16-bit operations like LD HL, nn, or ADD HL, BC. Maybe better to have functions that get and set the 16-bit register pairs.
For example, in the Z80 struct:

fn get_bc(&self) -> u16 {
(self.b as u16) << 8 | self.c as u16
}
fn set_bc(&mut self, value: u16) {
self.b = (value >> 8) as u8;
self.c = (value & 0xFF) as u8;
}

Similarly for DE, HL, etc. This way, when handling 16-bit operations, you can work with the combined value.
But then, when you have an instruction like LD HL, (nn), which loads HL from a memory address, you would read the 16-bit value from memory (little-endian), then set HL to that value.
So the code would look like:

let addr = self.fetch_word(memory); // fetch two bytes as little-endian
let value = memory.read_word(addr);
self.set_hl(value);
But the memory read_word function would need to read two bytes in little-endian order.
Wait, but the Z80's 16-bit memory reads are little-endian. So reading from address nn gives the low byte at nn and high byte at nn+1.
So memory.read_word(addr) would be:
fn read_word(&self, addr: u16) -> u16 {
let lo = self.read_byte(addr) as u16;
let hi = self.read_byte(addr + 1) as u16;
(hi << 8) | lo
}

Similarly, writing a word would be:

fn write_word(&mut self, addr: u16, value: u16) {
let lo = (value & 0xFF) as u8;
let hi = (value >> 8) as u8;
self.write_byte(addr, lo);
self.write_byte(addr + 1, hi);
}

Now, the flags. For example, after an ADD instruction, the flags are updated based on the result. Let's take the ADD A, B instruction. The result is A + B, and flags are set as follows:

- Carry (C): Set if there's a carry out of bit 7.
- Zero (Z): Set if result is zero.
- Sign (S): Set if result's bit 7 is set (negative in two's complement).
- Half Carry (H): Set if there's a carry from bit 3 to bit 4.
- Parity/Overflow (P/V): Set if overflow occurred in two's complement.
- Subtract (N): Reset (since ADD is not a subtraction).

So after adding, these flags need to be calculated.
Implementing this in Rust would require taking the operands, performing the operation, then setting the flags accordingly.
For example, for ADD A, B:

fn add_a_b(&mut self) {
let a = self.a;
let b = self.b;
let result = a.wrapping_add(b);
self.a = result;
self.set_zero_flag(result == 0);
self.set_sign_flag((result & 0x80) != 0);
self.set_carry_flag(a as u16 + b as u16 > 0xFF);
self.set_half_carry_flag((a & 0x0F) + (b & 0x0F) > 0x0F);
self.set_parity_overflow_flag( ((a ^ !b) & (a ^ result)) & 0x80 != 0 );
self.set_subtract_flag(false);
}

The parity/overflow flag is a bit tricky. For ADD, the overflow occurs if the two operands are of the same sign and the result is of the opposite sign. So another way to calculate it is:

let overflow = ((a ^ result) & (b ^ result)) & 0x80 != 0;

Or maybe using the formula from the Z80 documentation. Alternatively, using a helper function to calculate the overflow for addition.
Alternatively, use the carry from bit 6 and bit 7 to determine overflow. For two's complement overflow, it's when the carry into the sign bit is different from the carry out of the sign bit.
But maybe the correct way is to compute whether the result is outside the range of a signed byte. So if adding two positive numbers gives a negative result, or adding two negative numbers gives a positive result, then overflow has occurred.
So in code:

let a_signed = a as i8;
let b_signed = b as i8;
let result_signed = a_signed.wrapping_add(b_signed);
let overflow = (a_signed > 0 && b_signed > 0 && result_signed < 0) ||
(a_signed < 0 && b_signed < 0 && result_signed > 0);
But converting to i8 and checking might be more straightforward, but perhaps less efficient. However, for emulation accuracy, it's important.
Alternatively, using bitwise operations:
let overflow = ((a ^ b ^ 0x80) & (a ^ result)) & 0x80 != 0;
Not sure. Maybe look up the correct way to compute the overflow flag for ADD.
Upon checking, the overflow flag for addition is set when there is a two's complement overflow. This occurs when the carry into the sign bit (bit 7) is different from the carry out of the sign bit. So to compute it:
carry_in = (a ^ b) & 0x80) == 0;  // if a and b have the same sign
carry_out = ((a + b) & 0x80) != (a & 0x80);

The formula for overflow (V) in two's complement addition is:

V = (carry_in to bit 7) XOR (carry_out from bit 7)

But how to compute this with bitwise operations. Alternatively, overflow can be calculated as:

V = ((a & b & !result) | (!a & !b & result)) >> 7) & 1

So in code:

let overflow = ((a ^ b) & 0x80) == 0 && ((a ^ result) & 0x80) != 0;

If a and b have the same sign (bit 7 is same), and the result has a different sign, then overflow.
So:

self.set_parity_overflow_flag( ((a ^ b) & 0x80) == 0 && ((a ^ result) & 0x80) != 0 );

Similarly, for subtraction (like SUB), the overflow is when the operands have different signs and the result has the same sign as the subtrahend.
Each arithmetic operation will need to set the flags correctly. So for each instruction, need to implement the correct flag updates.
This is going to take a lot of code. But since the user wants all instructions implemented, there's no way around it.
Maybe create helper functions for common operations, like adding two bytes and setting flags, subtracting, etc.
For example:

fn add_with_flags(&mut self, a: u8, b: u8) -> u8 {
let result = a.wrapping_add(b);
self.set_zero_flag(result == 0);
self.set_sign_flag((result & 0x80) != 0);
self.set_carry_flag((a as u16 + b as u16) > 0xFF);
self.set_half_carry_flag((a & 0x0F) + (b & 0x0F) > 0x0F);
let overflow = ((a ^ b) & 0x80) == 0 && ((a ^ result) & 0x80) != 0;
self.set_parity_overflow_flag(overflow);
self.set_subtract_flag(false);
result
}

Then, in the ADD A, B instruction:

self.a = self.add_with_flags(self.a, self.b);

Similarly for other ADD instructions. But need to handle different sources, like ADD A, (HL), which would read from memory at HL.

So for ADD A, (HL):
let addr = self.get_hl();
let value = memory.read_byte(addr);
self.a = self.add_with_flags(self.a, value);

Another big part is implementing all the different addressing modes.
For example, instructions can operate on registers, immediate values, memory addresses pointed by HL, IX+d, IY+d, etc.
For example, LD r, (IX+d) would load a register from IX plus a displacement.
So the code would:

fn ld_r_ixd(&mut self, memory: &mut Memory, r: Register) {
let d = self.fetch_byte(memory) as i8;
let addr = self.ix.wrapping_add(d as u16);
let value = memory.read_byte(addr);
self.set_register(r, value);
}

Handling all the different register options would require enums or something. Alternatively, using a macro to generate functions for each register.
In Rust, enums can represent the registers, and then a match statement can set the appropriate register. For example:

enum Register {
A, B, C, D, E, H, L, // etc.
}

impl Z80 {
fn set_register(&mut self, reg: Register, value: u8) {
match reg {
Register::A => self.a = value,
Register::B => self.b = value,
// and so on
}
}
}

Additionally, handling interrupts and the halt state is necessary. The Z80 has maskable and non-maskable interrupts.
The Master System's interrupt handling would need to be emulated, but for the CPU core, the IFF (Interrupt Flip-Flop) flags need to be tracked.
So in the Z80 struct:

struct Z80 {
// ... registers
iff1: bool,
iff2: bool,
interrupt_mode: u8,
// ... other state
}

When an interrupt occurs, depending on the mode, the CPU pushes the PC onto the stack and jumps to a specific address.
Integrating this into the emulator requires checking for interrupts after each instruction, which depends on the CPU's cycle timing. However, without cycle accuracy, it's hard to model exactly when interrupts are triggered.
Developing a complete Sega Master System emulator with all Z80 instructions is a great challenge!

**[NES]**

[BUS]
chanel to conect all software modules

[CPU]
in this case will be 6502

- output an address 16-bit
- read and write data 8-bit
- whether it's reading or writing
- clock (forces cpu to changes state on each one of these clock edges)

- A: accumulator 8-bit
- x: register 8-bit
- y: register 8-bit
- stkp: stack pointer 8-bit (is a 8-bit number that points to an address somewhere in the memory and this address is incremented and decremented as we push and pull things from the stack)
- pc: program counter 16-bit (stores the address of next program byte that the CPU needs to read )
- status: STATUS REGISTER (contains information about the state of the processor)

obs:

- different instructions take diferrent number of clock cycles in order to execute this
- datas doen't have the same size every time
- 6502 have 56 instructions
- instructions exemple:
  LDA (load the accumulator) 0xA2 ( this instructions need a 8-bit data to load the accumulator)
  this means that this CPU will read 2 bytes of instructions. In this way, this would be a 2 bytes instruction

  LDA 0x0105 -> 3 bytes

  CLC -> 1 byte

Goals:

1. read a byte at the program counters location
2. use this byte to index an array which represents the big table to get me the addressing mode and the number of cycles:
3. read to any additional bytes that I need to complete the instruction
4. execute the instruction
5. wait, count clock cycles until the instruction is officially complete 

[MEMORY]
2kB
0x0000 -> 0x07FF

[APU]
gererate de sound
0x4000 -> 0x4017

[CARTRIDGE]
devices that contain the memorys typically have the programs that cpu runs
0x4020 -> 0xFFFF

[PPU]
picture process unit
in this case will emulate 2C02
0x2000 -> 0x2007

[GRAPHICS]
sprites of game
0x0600 -> 0x1FFF

[VRAM]
video memory
2kB
0x2000 -> 0x27FF

[PALLETES]
0x3F00 -> 0x3FFF

[OAM]
store the location of the sprites

[DMA]
Direct memory access

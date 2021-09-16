# yobemag

GameBoy emulator. WIP for now :)

## General knowledge

### ROM

This is the read-only memory, it's also called "cartridge" and holds the data of the game. Usually
they have the .gb extension in the filename.

#### Header

`0x0100-0x014F`

This area contains information about the cartridge that is inserted, including; type of cartridge,
size of rom, size of ram, a Nintendo logo, and other information.
Also, at 0x0100, there is a NOP instruction, followed by a JUMP to the start of the program,
usually 0x0150. The CPU begins execution at 0x0100, that is why this is included.
If the Nintendo logo bytes are not correct, the GameBoy will not execute the game.

#### ROM bank 0

`0x0150-0x3FFF (16KB ROM bank 00)`

Rom bank 0 is the home bank. It's the fist 16K bank of any ROM image, and is a fixed bank,
which means you can't switch it out with other banks. Since this bank is fixed and cannot be changed out,
it usually contains the majority of the game's engine, or core routines.

#### ROM bank N

`0x4000-0x7FFF (16KB ROM Bank 01~NN, from cartridge, switchable bank via MBC if any)`

This is where alot of your game data will exist, in this 16K space. This area is switchable or banked.
You can switch in 16K chunks of the entire ROM in this area, through the use of the
Memory Bank Controller (MBC) on the cartridge. How the MBC works is you basically "write" to an area in ROM,
and since ROM is by nature read only memory, writing a value to ROM is futile, except in the case of the MBC.
It intercepts the attempted write to ROM and interprets it into a bank switch,
the value you try to write to ROM is generally the bank number you want to change to.
Example:
`LD A,5
LD HL,$2000
LD (HL),A`
The first line loads the value 5 into register A.
The second line loads register pair HL with 0X2000, our destination address.
The third line loads the value in register A into the address indexed by register pair HL.

### RAM

There are many different ram.
Here we can have "high ram" also called "zero page memory", "vide ram" (vram) or working ram 
(simply called "ram" sometimes).

#### VRAM

`0x8000-0x9FFF (8KB Video RAM only bank 0 in Non-CGB mode Switchable bank 0/1 in CGB mode)`

##### Character RAM

`0x8000-0x97FF`

This area is also known as Tile RAM, since it holds tiles.
Each tile is 8x8 pixels of 2-bit color, which makes each tile 16 bytes long.
This area is also divided up into two modes of tiles, signed and unsigned.
In unsigned mode, tiles are numbered from 0-255 at 0x8000-0x9000.
In signed mode, tiles are numbered in two's complement from -127 to 128 at 0X87FF-0X97FF.

##### BG map data 1

`0x9800-0x9BFF`

This 1024-byte long area is what the video processor uses to build the display.
Each byte in this space represnts an 8x8 pixel space on the display. This area is 32x32 tiles large.
The display processor takes each byte and then goes into the Character RAM area and gets the
corresponding tile from that area and draws it to the screen. So, if the first byte in the map area
contained 0x40, the display processor would get tile 0x40 from the Character RAM and put it in the
top-left corner of the virtual screen.

##### BG map data 2

`0x9C00-0x9FFF`

This area is just a second background map area like the previous one.
To specify which map the video processor uses to build the background image, change the apropriate bit in the LCDC I/O register.

#### HRAM

`0xFF80-0xFFFE`
Originally intended to be used as 127 bytes of stack space, this area is better suited for use as a Zero-Page,
or a quick RAM access area, since there is an instruction that accesses the area 0xFF00-0xFFFF quicker than a regular LD instruction.
Most coders nowadays just set the stack to the TOP of internal RAM, since it works the same and frees the high RAM for quick variables and such.

#### External RAM

`0xA000-0xBFFF (8KB External RAM in cartridge, switchable bank if any)`

If present on the cartridge, this area is mapped to the RAM on the cartridge.

#### WRAM

`0xC000-0xCFFF (4KB Work RAM bank 0)`
`0xD000-0xDFFF (4KB Work RAM bank 1~N) Only bank 1 in Non-CGB modem, switchable bank 1~7 in CGB mode`

This RAM in inside the GameBoy. Generally used for most common variables and such in games.

#### Echo RAM

`0xE000-0xFDFF (Mirror of 0xC000-0xDDFF)`

This area echoes internal ram, but is specified by Nintendo as reserved and shouldn't be used at all. To keep with standards and to keep compatibility, don't use this area.

#### OAM

`0xFE00-0xFE9F`

Object Attribute Memory is sprite RAM. This area is 40 sprites X 4 bytes long.
When you want to display an object (sprite) you write 4 corresponding bytes to OAM. These 4 bytes are:
`Byte 1: X Location,
Byte 2: Y Location,
Tile Number (0-255),
Attributes`
The tile number is taken from the Character RAM, just a BG tiles are. The X and Y locations are slightly
offset (8 pixels and 16 pixels), so you can have sprites partially off of the left and top of the LCD.
So if you set the location to 0,0 then the sprite would be off of the screen. To set a sprite to the top-left corner, 
you'd set it's location to 8,16.

### Hardware I/O register

`0xFF00-0xFF7F`

This area contains all the control registers for all the hardware on the GameBoy and is basically a memory-mapped I/O area.

### CPU

Is the heart of every computer, and in this case we have eight 8 bit registers:
A, B, C, D, E, F, H, and L, as well as two 16 bit registers: SP, and PC.
PC -> program counter, tells the CPU the address that the next instruction is to be fetched from in memory, starts from 0x0100.
SP -> stack pointer, address in memory of the top of the stack, 0xFFFE when start.
A -> where almost all data being processed passes through. It is also known as the "accumulator".
B,C -> generally used as counters during repetitive blocks of code such as moving data from one location to another.
D,E -> generally used together as a 16-bit register for holding a destination address in moving data from one address to another.
H,L -> special due to the fact that they are extensively used for indirect addressing as register pair HL.
Indirect Addressing is when instead of specifying an specific address for an operation, you could just use the 16-bit value in HL as an address.

#### Flags

F -> contains flags that rapresents last operation result.
The cpu has an 8 bit register which controls if the last operation resulted in zero, an underflow,
a nibble overflow, or a byte overflow; refered to as the zero flag, the negative flag, the half carry flag, and the full carry flag, respectively.
Usually the name are: `z, n, h, c`.

`F register (8 bit)`

`| 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |`

`| z | n | h | c | 0 | 0 | 0 | 0 |`

#### Istruction set

Here you can find all opcodes:

`https://gbdev.io/gb-opcodes/optables/`

`https://rgbds.gbdev.io/docs/master/gbz80.7`

`https://izik1.github.io/gbops/index.html`

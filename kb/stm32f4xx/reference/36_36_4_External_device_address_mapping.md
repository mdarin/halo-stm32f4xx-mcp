**36.4 External device address mapping**
From the FSMC point of view, the external memory is divided into 4 fixed-size banks of 256 Mbytes each (Refer to Figure 435):

*   Bank 1 used to address up to 4 NOR Flash or PSRAM memory devices. This bank is split into 4 NOR/PSRAM subbanks with 4 dedicated Chip Selects, as follows:
    *   Bank 1 - NOR/PSRAM 1
    *   Bank 1 - NOR/PSRAM 2
    *   Bank 1 - NOR/PSRAM 3
    *   Bank 1 - NOR/PSRAM 4
*   Banks 2 and 3 used to address NAND Flash devices (1 device per bank)
*   Bank 4 used to address a PC Card device

For each bank the type of memory to be used is user-defined in the Configuration register.

**Figure 435. FSMC memory banks**

| Address | Banks | Supported memory type |
|---|---|---|
| 6000 0000h6FFF FFFh | Bank 14 x 64 MB | NOR / PSRAM |
| 7000 0000h7FFF FFFh | Bank 24 x 64 MB | NAND Flash |
| 8000 0000h8FFF FFFh | Bank 34 x 64 MB |  |
| 9000 0000h9FFF FFFh | Bank 44 x 64 MB | PC Card |


ai14719

**36.4.1 NOR/PSRAM address mapping**
HADDR[27:26] bits are used to select one of the four memory banks as shown in Table 216.

ST
RM0090 Rev 18
1547/1749

--- Page 183 ---

Flexible static memory controller (FSMC)
RM0090

Table 216. NOR/PSRAM bank selection
| HADDR[27:26](1) | Selected bank |
|---|---|
| 00 | Bank 1 - NOR/PSRAM 1 |
| 01 | Bank 1 - NOR/PSRAM 2 |
| 10 | Bank 1 - NOR/PSRAM 3 |
| 11 | Bank 1 - NOR/PSRAM 4 |


1. HADDR are internal AHB address lines that are translated to external memory.

HADDR[25:0] contain the external memory address. Since HADDR is a byte address whereas the memory is addressed in words, the address actually issued to the memory varies according to the memory data width, as shown in the following table.

Table 217. External memory address
| Memory width(1) | Data address issued to the memory | Maximum memory capacity (bits) |
|---|---|---|
| 8-bit | HADDR[25:0] | 64 Mbyte x 8 = 512 Mbit |
| 16-bit | HADDR[25:1] >> 1 | 64 Mbyte/2 x 16 = 512 Mbit |


1. In case of a 16-bit external memory width, the FSMC will internally use HADDR[25:1] to generate the address for external memory FSMC_A[24:0].
Whatever the external memory width (16-bit or 8-bit), FSMC_A[0] should be connected to external memory address A[0].

**Wrap support for NOR Flash/PSRAM**

Wrap burst mode for synchronous memories is not supported. The memories must be configured in linear burst mode of undefined length.

**36.4.2 NAND/PC Card address mapping**

In this case, three banks are available, each of them divided into memory spaces as indicated in Table 218.

Table 218. Memory mapping and timing registers
| Start address | End address | FSMC Bank | Memory space | Timing register |
|---|---|---|---|---|
| 0x9C00 0000 | 0x9FFF FFFF | Bank 4 - PC card | I/O | FSMC_PIO4 (0xB0) |
| 0x9800 0000 | 0x9BFF FFFF | Attribute | FSMC_PATT4 (0xAC) |  |
| 0x9000 0000 | 0x93FF FFFF | Common | FSMC_PMEM4 (0xA8) |  |
| 0x8800 0000 | 0x8BFF FFFF | Bank 3 - NAND Flash | Attribute | FSMC_PATT3 (0x8C) |
| 0x8000 0000 | 0x83FF FFFF | Common | FSMC_PMEM3 (0x88) |  |
| 0x7800 0000 | 0x7BFF FFFF | Bank 2- NAND Flash | Attribute | FSMC_PATT2 (0x6C) |
| 0x7000 0000 | 0x73FF FFFF | Common | FSMC_PMEM2 (0x68) |  |


1548/1749
RM0090 Rev 18

--- Page 184 ---

RM0090
Flexible static memory controller (FSMC)

For NAND Flash memory, the common and attribute memory spaces are subdivided into three sections (see in Table 219 below) located in the lower 256 Kbytes:

*   Data section (first 64 Kbytes in the common/attribute memory space)
*   Command section (second 64 Kbytes in the common / attribute memory space)
*   Address section (next 128 Kbytes in the common / attribute memory space)

Table 219. NAND bank selections

| Section name | HADDR[17:16] | Address range |
|---|---|---|
| Address section | 1X | 0x020000-0x03FFFF |
| Command section | 01 | 0x010000-0x01FFFF |
| Data section | 00 | 0x000000-0x0FFFF |


The application software uses the 3 sections to access the NAND Flash memory:

*   **To send a command to NAND Flash memory**: the software must write the command value to any memory location in the command section.
*   **To specify the NAND Flash address that must be read or written**: the software must write the address value to any memory location in the address section. Since an address can be 4 or 5 bytes long (depending on the actual memory size), several consecutive writes to the address section are needed to specify the full address.
*   **To read or write data**: the software reads or writes the data value from or to any memory location in the data section.

Since the NAND Flash memory automatically increments addresses, there is no need to increment the address of the data section to access consecutive memory locations.


### 37.4 External device address mapping

From the FMC point of view, the external memory is divided into 6 fixed-size banks of 256 Mbyte each (see Figure 457):

*   Bank 1 used to address up to 4 NOR Flash memory or PSRAM devices. This bank is split into 4 NOR/PSRAM subbanks with 4 dedicated Chip Selects, as follows:
    *   Bank 1 - NOR/PSRAM 1
    *   Bank 1 - NOR/PSRAM 2
    *   Bank 1 - NOR/PSRAM 3
    *   Bank 1 - NOR/PSRAM 4
*   Banks 2 and 3 used to address NAND Flash memory devices (1 device per bank)
*   Bank 4 used to address a PC Card
*   Bank 5 and 6 used to address SDRAM devices (1 device per bank).

For each bank the type of memory to be used can be configured by the user application through the Configuration register.

1606/1749 RM0090 Rev 18 ST

--- Page 47 ---

RM0090
Flexible memory controller (FMC)

Figure 457. FMC memory banks

Address
Bank
Supported memory type
0x6000 0000
Bank 1
4 x 64 MB
NOR/PSRAM/SRAM
0x6FFF FFFF
0x7000 0000
Bank 2
4 x 64 MB
NAND Flash memory
0x7FFF FFFF
0x8000 0000
Bank 3
4 x 64 MB
0x8FFF FFFF
0x9000 0000
Bank 4
4 x 64 MB
PC Card
0x9FFF FFFF
0xC000 0000
SDRAM Bank 1
4 x 64 MB
SDRAM
0xCFFF FFFF
0xD000 0000
SDRAM Bank 2
4 x 64 MB
0xDFFF FFFF
MS30444V2

37.4.1 NOR/PSRAM address mapping

HADDR[27:26] bits are used to select one of the four memory banks as shown in Table 254.

Table 254. NOR/PSRAM bank selection
| HADDR[27:26](1) | Selected bank |
|---|---|
| 00 | Bank 1 - NOR/PSRAM 1 |
| 01 | Bank 1 - NOR/PSRAM 2 |
| 10 | Bank 1 - NOR/PSRAM 3 |
| 11 | Bank 1 - NOR/PSRAM 4 |


1. HADDR are internal AHB address lines that are translated to external memory.

ST
RM0090 Rev 18
1607/1749

--- Page 48 ---

**Flexible memory controller (FMC)**
RM0090

The HADDR[25:0] bits contain the external memory address. Since HADDR is a byte address whereas the memory is addressed at word level, the address actually issued to the memory varies according to the memory data width, as shown in the following table.

**Table 255. NOR/PSRAM External memory address**

| Memory width(1) | Data address issued to the memory | Maximum memory capacity (bits) |
|---|---|---|
| 8-bit | HADDR[25:0] | 64 Mbyte x 8 = 512 Mbit |
| 16-bit | HADDR[25:1] >> 1 | 64 Mbyte/2 x 16 = 512 Mbit |
| 32-bit | HADDR[25:2] >> 2 | 64 Mbyte/4 x 32 = 512 Mbit |


1. In case of a 16-bit external memory width, the FMC will internally use HADDR[25:1] to generate the address for external memory FMC_A[24:0]. In case of a 32-bit memory width, the FMC will internally use HADDR[25:2] to generate the external address.
Whatever the external memory width, FMC_A[0] should be connected to external memory address A[0].

**Wrap support for NOR Flash/PSRAM**

Wrap burst mode for synchronous memories is not supported. The memories must be configured in linear burst mode of undefined length.

**37.4.2 NAND Flash memory/PC Card address mapping**

In this case, three banks are available, each of them being divided into memory areas as indicated in Table 256.

**Table 256. NAND/PC Card memory mapping and timing registers**

| Start address | End address | FMC bank | Memory space | Timing register |
|---|---|---|---|---|
| 0x9C00 0000 | 0x9FFF FFFF | Bank 4 - PC card | I/O | FMC_PIO4 (0xB0) |
| 0x9800 0000 | 0x9BFF FFFF | Attribute | FMC_PATT4 (0xAC) |  |
| 0x9000 0000 | 0x93FF FFFF | Common | FMC_PMEM4 (0xA8) |  |
| 0x8800 0000 | 0x8BFF FFFF | Bank 3 - NAND Flash | Attribute | FMC_PATT3 (0x8C) |
| 0x8000 0000 | 0x83FF FFFF | Common | FMC_PMEM3 (0x88) |  |
| 0x7800 0000 | 0x7BFF FFFF | Bank 2- NAND Flash | Attribute | FMC_PATT2 (0x6C) |
| 0x7000 0000 | 0x73FF FFFF | Common | FMC_PMEM2 (0x68) |  |


For NAND Flash memory, the common and attribute memory spaces are subdivided into three sections (see in Table 257 below) located in the lower 256 Kbytes:

*   Data section (first 64 Kbytes in the common/attribute memory space)
*   Command section (second 64 Kbytes in the common / attribute memory space)
*   Address section (next 128 Kbytes in the common / attribute memory space)

1608/1749
RM0090 Rev 18
ST

--- Page 49 ---

RM0090
Flexible memory controller (FMC)

Table 257. NAND bank selection

| Section name | HADDR[17:16] | Address range |
|---|---|---|
| Address section | 1X | 0x020000-0x03FFFF |
| Command section | 01 | 0x010000-0x01FFFF |
| Data section | 00 | 0x000000-0x0FFFFF |


The application software uses the 3 sections to access the NAND Flash memory:

*   **To send a command to NAND Flash memory**, the software must write the command value to any memory location in the command section.
*   **To specify the NAND Flash address that must be read or written**, the software must write the address value to any memory location in the address section. Since an address can be 4 or 5 byte long (depending on the actual memory size), several consecutive write operations to the address section are required to specify the full address.
*   **To read or write data**, the software reads or writes the data from/to any memory location in the data section.

Since the NAND Flash memory automatically increments addresses, there is no need to increment the address of the data section to access consecutive memory locations.

### 37.4.3 SDRAM address mapping

The HADDR[28] bit (internal AHB address line 28) is used to select one of the two memory banks as indicated in Table 258.

Table 258. SDRAM bank selection

| HADDR[28] | Selected bank | Control register | Timing register |
|---|---|---|---|
| 0 | SDRAM Bank1 | FMC_SDCR1 | FMC_SDTR1 |
| 1 | SDRAM Bank2 | FMC_SDCR2 | FMC_SDTR2 |


The following table shows SDRAM mapping for an 13-bit row ,a 11-bit column and 4 internal bank configurations.

Table 259. SDRAM address mapping

| Memory width(1) | Internal bank | Row address | Column address(2) | Maximum memory capacity (Mbyte) |
|---|---|---|---|---|
| 8-bit | HADDR[25:24] | HADDR[23:11] | HADDR[10:0] | 64 Mbyte: 4 x 8K x 2K |
| 16-bit | HADDR[26:25] | HADDR[24:12] | HADDR[11:1] | 128 Mbyte: 4 x 8K x 2K x 2 |
| 32-bit | HADDR[27:26] | HADDR[25:13] | HADDR[12:2] | 256 Mbyte: 4 x 8K x 2K x 4 |


RM0090 Rev 18
1609/1749

--- Page 50 ---

**Flexible memory controller (FMC)**
RM0090

1. When interfacing with a 16-bit memory, the FMC internally uses the HADDR[11:1] internal AHB address lines to generate the external address. When interfacing with a 32-bit memory, the FMC internally uses HADDR[12:2] lines to generate the external address. Whatever the memory width, FMC_A[0] has to be connected to the external memory address A[0].
2. The AutoPrecharge is not supported. FMC_A[10] must be connected to the external memory address A[10] but it will be always driven 'low'.

The HADDR[27:0] bits are translated to external SDRAM address depending on the SDRAM controller configuration:
* Data size: 8, 16 or 32 bits
* Row size: 11, 12 or 13 bits
* Column size: 8, 9, 10 or 11 bits
* Number of internal banks: two or four internal banks

Table 260 to Table shows the SDRAM address mapping versus the SDRAM controller configuration.

**Table 260. SDRAM address mapping with 8-bit data bus width**(1)(2)

| Row size configuration | HADDR(AHB Internal Address Lines) |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
| 11-bit row size configuration | Res. | Bank [1:0] | Row[10:0] | Column[7:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Res. | Bank [1:0] | Row[10:0] | Column[8:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Res. | Bank [1:0] | Row[10:0] | Column[9:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Res. | Bank [1:0] | Row[10:0] | Column[10:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 12-bit row size configuration | Res. | Bank [1:0] | Row[11:0] | Column[7:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Res. | Bank [1:0] | Row[11:0] | Column[8:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Res. | Bank [1:0] | Row[11:0] | Column[9:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Res. | Bank [1:0] | Row[11:0] | Column[10:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 13-bit row size configuration | Res. | Bank [1:0] | Row[12:0] | Column[7:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Res. | Bank [1:0] | Row[12:0] | Column[8:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Res. | Bank [1:0] | Row[12:0] | Column[9:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Res. | Bank [1:0] | Row[12:0] | Column[10:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


1. BANK[1:0] are the Bank Address BA[1:0]. When only 2 internal banks are used, BA1 must always be set to '0'.
2. Access to Reserved (Res.) address range generates an AHB error.

1610/1749
RM0090 Rev 18

--- Page 51 ---

RM0090
Flexible memory controller (FMC)

Table 261. SDRAM address mapping with 16-bit data bus width(1)(2)

| Row size Configuration | HADDR(AHB address Lines) |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
| 11-bit row size configuration | Res. | Bank [1:0] | Row[10:0] | Column[7:0] | BM0(3) |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Res. | Bank [1:0] | Row[10:0] | Column[8:0] | BM0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Res. | Bank [1:0] | Row[10:0] | Column[9:0] | BM0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Res. | Bank [1:0] | Row[10:0] | Column[10:0] | BM0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 12-bit row size configuration | Res. | Bank [1:0] | Row[11:0] | Column[7:0] | BM0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Res. | Bank [1:0] | Row[11:0] | Column[8:0] | BM0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Res. | Bank [1:0] | Row[11:0] | Column[9:0] | BM0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Res. | Bank [1:0] | Row[11:0] | Column[10:0] | BM0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 13-bit row size configuration | Res. | Bank [1:0] | Row[12:0] | Column[7:0] | BM0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Res. | Bank [1:0] | Row[12:0] | Column[8:0] | BM0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Res. | Bank [1:0] | Row[12:0] | Column[9:0] | BM0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Res. | Bank [1:0] | Row[12:0] | Column[10:0] | BM0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


1. BANK[1:0] are the Bank Address BA[1:0]. When only 2 internal banks are used, BA1 must always be set to '0'.
2. Access to Reserved space (Res.) generates an AHB error.
3. BM0: is the byte mask for 16-bit access.

Table 262. SDRAM address mapping with 32-bit data bus width(1)(2)

| Row size configuration | HADDR(AHB address Lines) |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
| 11-bit row size configuration | Res. | Bank [1:0] | Row[10:0] | Column[7:0] | BM[1:0](3) |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Res. | Bank [1:0] | Row[10:0] | Column[8:0] | BM[1:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Res. | Bank [1:0] | Row[10:0] | Column[9:0] | BM[1:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Res. | Bank [1:0] | Row[10:0] | Column[10:0] | BM[1:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 12-bit row size configuration | Res. | Bank [1:0] | Row[11:0] | Column[7:0] | BM[1:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Res. | Bank [1:0] | Row[11:0] | Column[8:0] | BM[1:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Res. | Bank [1:0] | Row[11:0] | Column[9:0] | BM[1:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Res. | Bank [1:0] | Row[11:0] | Column[10:0] | BM[1:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


ST
RM0090 Rev 18
1611/1749

--- Page 52 ---

**Flexible memory controller (FMC)**
RM0090

**Table 262. SDRAM address mapping with 32-bit data bus width(1)(2) (continued)**

| Row size configuration | HADDR(AHB address Lines) |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
| 13-bit row size configuration | Res. | Bank [1:0] | Row[12:0] | Column[7:0] | BM[1:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Res. | Bank [1:0] | Row[12:0] | Column[8:0] | BM[1:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Res. | Bank [1:0] | Row[12:0] | Column[9:0] | BM[1:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Bank [1:0] | Row[12:0] | Column[10:0] | BM[1:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


1. BANK[1:0] are the Bank Address BA[1:0]. When only 2 internal banks are used, BA1 must always be set to '0'.
2. Access to Reserved space (Res.) generates an AHB error.
3. BM[1:0]: is the byte mask for 32-bit access.


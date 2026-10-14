> [RM0090 ch.36 §36.6] NAND Flash/PC Card controller


### 36.6.7 PC Card/CompactFlash operations

**Address spaces and memory accesses**

The FSMC supports Compact Flash storage or PC Cards in Memory Mode and I/O Mode (True IDE mode is not supported).

The Compact Flash storage and PC Cards are made of 3 memory spaces:
* Common Memory Space
* Attribute Space
* I/O Memory Space

The nCE2 and nCE1 pins (FSMC_NCE4_2 and FSMC_NCE4_1 respectively) select the card and indicate whether a byte or a word operation is being performed: nCE2 accesses the odd byte on D15-8 and nCE1 accesses the even byte on D7-0 if A0=0 or the odd byte on D7-0 if A0=1. The full word is accessed on D15-0 if both nCE2 and nCE1 are low.

The memory space is selected by asserting low nOE for read accesses or nWE for write accesses, combined with the low assertion of nCE2/nCE1 and nREG.
* If pin nREG=1 during the memory access, the common memory space is selected
* If pin nREG=0 during the memory access, the attribute memory space is selected

The I/O Space is selected by asserting low nIORD for read accesses or nIOWR for write accesses [instead of nOE/nWE for memory Space], combined with nCE2/nCE1. Note that nREG must also be asserted low during accesses to I/O Space.

Three type of accesses are allowed for a 16-bit PC Card:
* Accesses to Common Memory Space for data storage can be either 8-bit accesses at even addresses or 16 bit AHB accesses.

Note that 8-bit accesses at odd addresses are not supported and will not lead to the low assertion of nCE2. A 32-bit AHB request is translated into two 16-bit memory accesses.

* Accesses to Attribute Memory Space where the PC Card stores configuration information are limited to 8-bit AHB accesses at even addresses.

Note that a 16-bit AHB access will be converted into a single 8-bit memory transfer: nCE1 will be asserted low, nCE2 will be asserted high and only the even Byte on D7-D0 will be valid. Instead a 32-bit AHB access will be converted into two 8-bit memory

ST
RM0090 Rev 18
1591/1749

--- Page 32 ---

Flexible static memory controller (FSMC)
RM0090

transfers at even addresses: nCE1 will be asserted low, NCE2 will be asserted high and only the even bytes will be valid.
*   Accesses to I/O Space can be performed either through AHB 8-bit or 16-bit accesses.

Table 251. 16-bit PC-Card signals and access type

| nCE2 | nCE1 | nREG | nOE/nWE | nIORD /nIOWR | A10 | A9 | A7-1 | A0 | Space | Access Type | Allowed/not Allowed |
|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | 0 | 1 | 0 | 1 | X | X-X | X | Common Memory Space | Read/Write byte on D7-D0 | YES |  |
| 0 | 1 | 0 | 1 | X | X-X | X | Read/Write byte on D15-D8 | Not supported |  |  |  |
| 0 | 1 | 0 | 1 | X | X-X | 0 | Read/Write word on D15-D0 | YES |  |  |  |
| X | 0 | 1 | 0 | 1 | X-X | 0 | Attribute Space | Read or Write Configuration Registers | YES |  |  |
| X | 0 | 1 | 0 | X-X | 0 | Read or Write CIS (Card Information Structure) | YES |  |  |  |  |
| 1 | 0 | 1 | X | X-X | 1 | Attribute Space | Invalid Read or Write (odd address) | YES |  |  |  |
| 0 | 1 | 0 | 1 | X | X-X | x | Invalid Read or Write (odd address) | YES |  |  |  |
| 1 | 0 | 1 | 0 | X | X-X | 0 | I/O space | Read Even Byte on D7-0 | YES |  |  |
| 1 | 0 | 1 | 0 | X | X-X | 1 | Read Odd Byte on D7-0 | YES |  |  |  |
| 1 | 0 | 1 | 0 | X | X-X | 0 | Write Even Byte on D7-0 | YES |  |  |  |
| 1 | 0 | 1 | 0 | X | X-X | 1 | Write Odd Byte on D7-0 | YES |  |  |  |
| 0 | 1 | 0 | X | X-X | 0 | Read Word on D15-0 | YES |  |  |  |  |
| 0 | 1 | 0 | X | X-X | 0 | Write word on D15-0 | YES |  |  |  |  |
| 0 | 1 | 0 | 1 | 0 | X | X-X | X | Read Odd Byte on D15-8 | Not supported |  |  |
| 0 | 1 | 0 | 1 | 0 | X | X-X | X | Write Odd Byte on D15-8 | Not supported |  |  |


The FSMC Bank 4 gives access to those 3 memory spaces as described in Section 36.4.2: NAND/PC Card address mapping and Table 218: Memory mapping and timing registers.

**Wait Feature**

The CompactFlash Storage or PC Card may request the FSMC to extend the length of the access phase programmed by MEMWAITx/ATTWAITx/IOWAITx bits, asserting the nWAIT signal after nOE/nWE or nIORD/nIOWR activation if the wait feature is enabled through the PWAITEN bit in the FSMC_PCRx register. In order to detect the nWAIT assertion correctly, the MEMWAITx/ATTWAITx/IOWAITx bits must be programmed as follows:

1592/1749
RM0090 Rev 18
ST

--- Page 33 ---

RM0090
Flexible static memory controller (FSMC)

xxWAITx >= 4 + max_wait_assertion_time/HCLK
Where max_wait_assertion_time is the maximum time taken by NWAIT to go low once
nOE/nWE or nIORD/nIOWR is low.
After the de-assertion of nWAIT, the FSMC extends the WAIT phase for 4 HCLK clock
cycles.


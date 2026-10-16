> [RM0090 ch.36 §36.6] NAND Flash/PC Card controller


### Common memory space timing register 2..4 (FSMC_PMEM2..4)

Address offset: Address: 0xA000 0000 + 0x48 + 0x20 * (x – 1), x = 2..4

Reset value: 0xFCFC FCFC

Each FSMC_PMEMx (x = 2..4) read/write register contains the timing information for PC Card or NAND Flash memory bank x, used for access to the common memory space of the 16-bit PC Card/CompactFlash, or to access the NAND Flash for command, address write access and data read/write access.

| 31 30 29 28 27 26 25 24 | 23 22 21 20 19 18 17 16 | 15 14 13 12 11 10 9 8 | 7 6 5 4 3 2 1 0 |
|---|---|---|---|
| MEMHIZ[7:0] | MEMHOLD[7:0] | MEMWAIT[7:0] | MEMSET[7:0] |
| rw |  |  |  |


ST
RM0090 Rev 18
1595/1749

--- Page 36 ---

**Flexible static memory controller (FSMC)**
RM0090

Bits 31:24 **MEMHIZx[7:0]**: Common memory x databus HiZ time
Defines the number of HCLK clock cycles during which the databus is kept in HiZ after the start of a PC Card/NAND Flash write access to common memory space on socket x. Only valid for write transaction:

0000 0000: 1 HCLK cycle
1111 1110: 255 HCLK cycles
1111 1111: Reserved

Bits 23:16 **MEMHOLDx[7:0]**: Common memory x hold time
For NAND Flash read accesses to the common memory space, these bits define the number of (HCLK+2) clock cycles during which the address is held after the command is deasserted (NWE, NOE).
For NAND Flash write accesses to the common memory space, these bits define the number of HCLK clock cycles during which the data are held after the command is deasserted (NWE, NOE).
0000 0000: Reserved
0000 0001: 1 HCLK cycle for write accesses, 3 HCLK cycles for read accesses
1111 1110: 254 HCLK cycle for write accesses, 256 HCLK cycles for read accesses
1111 1111: Reserved

Bits 15:8 **MEMWAITx[7:0]**: Common memory x wait time
Defines the minimum number of HCLK (+1) clock cycles to assert the command (NWE, NOE), for PC Card/NAND Flash read or write access to common memory space on socket x. The duration for command assertion is extended if the wait signal (NWAIT) is active (low) at the end of the programmed value of HCLK:
0000 0000: Reserved
0000 0001: 2 HCLK cycles (+ wait cycle introduced by deasserting NWAIT)
1111 1110: 255 HCLK cycles (+ wait cycle introduced by deasserting NWAIT)
1111 1111: Reserved.

Bits 7:0 **MEMSETx[7:0]**: Common memory x setup time
Defines the number of HCLK () clock cycles to set up the address before the command assertion (NWE, NOE), for PC Card/NAND Flash read or write access to common memory space on socket x:
0000 0000: 1 HCLK cycle
1111 1110: 255 HCLK cycles
1111 1111: Reserved

**Attribute memory space timing registers 2..4 (FSMC_PATT2..4)**
Address offset: 0xA000 0000 + 0x4C + 0x20 * (x − 1), x = 2..4
Reset value: 0xFCFC FCFC

Each FSMC_PATTx (x = 2..4) read/write register contains the timing information for PC Card/CompactFlash or NAND Flash memory bank x. It is used for 8-bit accesses to the attribute memory space of the PC Card/CompactFlash or to access the NAND Flash for the last address write access if the timing must differ from that of previous accesses (for Ready/Busy management, refer to Section 36.6.5: NAND Flash prewait functionality).

| 31 30 29 28 27 26 25 24 | 23 22 21 20 19 18 17 16 | 15 14 13 12 11 10 9 8 | 7 6 5 4 3 2 1 0 |
|---|---|---|---|
| ATTHIZ[7:0] | ATTHOLD[7:0] | ATTWAIT[7:0] | ATTSET[7:0] |
| rw |  |  |  |


1596/1749
RM0090 Rev 18
STI

--- Page 37 ---

RM0090
Flexible static memory controller (FSMC)

Bits 31:24 **ATTHIZ[7:0]**: Attribute memory x databus HiZ time
Defines the number of HCLK clock cycles during which the databus is kept in HiZ after the start of a PC CARD/NAND Flash write access to attribute memory space on socket x. Only valid for write transaction:
0000 0000: 0 HCLK cycle
1111 1110: 255 HCLK cycles
1111 1111: Reserved.

Bits 23:16 **ATTHOLD[7:0]**: Attribute memory x hold time
For PC Card/NAND Flash read accesses to attribute memory space on socket x, these bits define the number of HCLK clock cycles (HCLK +2) clock cycles during which the address is held after the command is deasserted (NWE, NOE).
For PC Card/NAND Flash write accesses to attribute memory space on socket x, these bits define the number of HCLK clock cycles during which the data are held after the command is deasserted (NWE, NOE).
0000 0000: reserved
0000 0001: 1 HCLK cycle for write access, 3 HCLK cycles for read accesses
1111 1110: 254 HCLK cycle for write access, 256 HCLK cycles for read accesses
1111 1111: Reserved

Bits 15:8 **ATTWAIT[7:0]**: Attribute memory x wait time
Defines the minimum number of HCLK (+1) clock cycles to assert the command (NWE, NOE), for PC Card/NAND Flash read or write access to attribute memory space on socket x. The duration for command assertion is extended if the wait signal (NWAIT) is active (low) at the end of the programmed value of HCLK:
0000 0000: Reserved
0000 0001: 2 HCLK cycles (+ wait cycle introduced by deassertion of NWAIT)
1111 1111: 255 HCLK cycles (+ wait cycle introduced by deasserting NWAIT)
1111 1111: Reserved.

Bits 7:0 **ATTSET[7:0]**: Attribute memory x setup time
Defines the number of HCLK (+1) clock cycles to set up address before the command assertion (NWE, NOE), for PC CARD/NAND Flash read or write access to attribute memory space on socket x:
0000 0000: 1 HCLK cycle
1111 1110: 255 HCLK cycles
1111 1111: Reserved.

**I/O space timing register 4 (FSMC_PIO4)**

Address offset: 0xA000 0000 + 0xB0
Reset value: 0xFCFCFCFC

The FSMC_PIO4 read/write registers contain the timing information used to gain access to the I/O space of the 16-bit PC Card/CompactFlash.

| 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0 |  |  |  |
|---|---|---|---|
| IOHIZ[7:0] | IOHOLD[7:0] | IOWAIT[7:0] | IOSET[7:0] |
| r/w |  |  |  |


ST
RM0090 Rev 18
1597/1749

--- Page 38 ---

**Flexible static memory controller (FSMC)**
RM0090

Bits 31:24 **IOHIZ[7:0]**: I/O x databus HiZ time
Defines the number of HCLK clock cycles during which the databus is kept in HiZ after the start of a PC Card write access to I/O space on socket x. Only valid for write transaction:
0000 0000: 0 HCLK cycle
1111 1111: 255 HCLK cycles (default value after reset)

Bits 23:16 **IOHOLD[7:0]**: I/O x hold time
Defines the number of HCLK clock cycles to hold address (and data for write access) after the command deassertion (NWE, NOE), for PC Card read or write access to I/O space on socket x:
0000 0000: reserved
0000 0001: 1 HCLK cycle
1111 1111: 255 HCLK cycles (default value after reset)

Bits 15:8 **IOWAIT[7:0]**: I/O x wait time
Defines the minimum number of HCLK (+1) clock cycles to assert the command (SMNWE, SMNOE), for PC Card read or write access to I/O space on socket x. The duration for command assertion is extended if the wait signal (NWAIT) is active (low) at the end of the programmed value of HCLK:
0000 0000: reserved, do not use this value
0000 0001: 2 HCLK cycles (+ wait cycle introduced by deassertion of NWAIT)
1111 1111: 256 HCLK cycles (+ wait cycle introduced by the Card deasserting NWAIT) (default value after reset)

Bits 7:0 **IOSET[7:0]**: I/O x setup time
Defines the number of HCLK (+1) clock cycles to set up the address before the command assertion (NWE, NOE), for PC Card read or write access to I/O space on socket x:
0000 0000: 1 HCLK cycle
1111 1111: 256 HCLK cycles (default value after reset)

**ECC result registers 2/3 (FSMC_ECCR2/3)**

Address offset: 0xA000 0000 + 0x54 + 0x20 * (x – 1), x = 2 or 3
Reset value: 0x0000 0000

These registers contain the current error correction code value computed by the ECC computation modules of the FSMC controller (one module per NAND Flash memory bank). When the CPU reads the data from a NAND Flash memory page at the correct address (refer to Section 36.6.6: *Computation of the error correction code (ECC) in NAND Flash memory*), the data read from or written to the NAND Flash are processed automatically by ECC computation module. At the end of X bytes read (according to the ECCPS field in the FSMC_PCRx registers), the CPU must read the computed ECC value from the FSMC_ECCx registers, and then verify whether these computed parity data are the same as the parity value recorded in the spare area, to determine whether a page is valid, and, to correct it if applicable. The FSMC_ECCRx registers should be cleared after being read by setting the ECCEN bit to zero. For computing a new data block, the ECCEN bit must be set to one.

| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| ECCx[31:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| r |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


1598/1749
RM0090 Rev 18
ST

--- Page 39 ---

RM0090
Flexible static memory controller (FSMC)

Bits 31:0 **ECCx[31:0]**: ECC result
This field provides the value computed by the ECC computation logic. *Table 252* hereafter describes the contents of these bit fields.

Table 252. ECC result relevant bits
| ECCPS[2:0] | Page size in bytes | ECC bits |
|---|---|---|
| 000 | 256 | ECC[21:0] |
| 001 | 512 | ECC[23:0] |
| 010 | 1024 | ECC[25:0] |
| 011 | 2048 | ECC[27:0] |
| 100 | 4096 | ECC[29:0] |
| 101 | 8192 | ECC[31:0] |


ST
RM0090 Rev 18
1599/1749

--- Page 40 ---

Flexible static memory controller (FSMC)

RM0090

36.6.9 FSMC register map
The following table summarizes the FSMC registers.

RM0090 Rev 18

1600/1749

Table 253. FSMC register map

| Offset | Register | FSMC_BCR1 | FSMC_BCR2 | FSMC_BCR3 | FSMC_BCR4 | FSMC_BTR1 | FSMC_BTR2 | FSMC_BTR3 | FSMC_BTR4 | FSMC_BWTR1 | FSMC_BWTR2 |
|---|---|---|---|---|---|---|---|---|---|---|---|
| 0000 | 0008 | 0010 | 0018 | 0004 | 000C | 0014 | 001C | 0104 | 010C |  |  |
| 0 | MBKEN | ADDSET[3:0] |  |  |  |  |  |  |  |  |  |
| 1 | MUXEN | ADDHLD[3:0] |  |  |  |  |  |  |  |  |  |
| 2 | MTYP[0:1] |  |  |  |  |  |  |  |  |  |  |
| 3 | MWID[1:0] | FACEN |  |  |  |  |  |  |  |  |  |
| 4 | FACEN |  |  |  |  |  |  |  |  |  |  |
| 5 | Reserved | BURSTEN |  |  |  |  |  |  |  |  |  |
| 6 | Reserved |  |  |  |  |  |  |  |  |  |  |
| 7 | WAITPOL | WRAPMOD |  |  |  |  |  |  |  |  |  |
| 8 | WAITCFG |  |  |  |  |  |  |  |  |  |  |
| 9 | WREN | WAITEN |  |  |  |  |  |  |  |  |  |
| 10 | EXTMOD |  |  |  |  |  |  |  |  |  |  |
| 11 | ASYNCWAIT | CPSIZE[2:0] |  |  |  |  |  |  |  |  |  |
| 12 | CBURSTRW |  |  |  |  |  |  |  |  |  |  |
| 13 | Reserved | CLKDIV[3:0] |  |  |  |  |  |  |  |  |  |
| 14 | Reserved |  |  |  |  |  |  |  |  |  |  |
| 15 | Reserved | DATLAT[3:0] |  |  |  |  |  |  |  |  |  |
| 16 | Reserved |  |  |  |  |  |  |  |  |  |  |
| 17 | ACC MOD [1:0] |  |  |  |  |  |  |  |  |  |  |
| 18 | Res. |  |  |  |  |  |  |  |  |  |  |


--- Page 41 ---

RM0090
Flexible static memory controller (FSMC)

Table 253. FSMC register map (continued)

| Offset | Register |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
| 0114 | FSMC_BWTR3 | Res. | ACCMOD[1:0] | Res. | BUSTURN[3:0] |  |  | DATAST[7:0] |  |  | ADDHLD[3:0] |  |  | ADDSET[3:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 011C | FSMC_BWTR4 | Res. | ACCMOD[1:0] | Res. | BUSTURN[3:0] |  |  | DATAST[7:0] |  |  | ADDHLD[3:0] |  |  | ADDSET[3:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0xA0000060 | FSMC_PCR2 | Reserved | ECCPS[2:0] | TAR[2:0] | TCLR[2:0] | Res. | ECCEN | PWD[1:0] | PTYP | PBKEN | PWAITEN | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0xA0000080 | FSMC_PCR3 | Reserved | ECCPS[2:0] | TAR[2:0] | TCLR[2:0] | Res. | ECCEN | PWD[1:0] | PTYP | PBKEN | PWAITEN | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0xA00000A0 | FSMC_PCR4 | Reserved | ECCPS[2:0] | TAR[2:0] | TCLR[2:0] | Res. | ECCEN | PWD[1:0] | PTYP | PBKEN | PWAITEN | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0xA0000064 | FSMC_SR2 | Reserved | FEMPT | IFEN | ILEN | IREN | IFS | ILS | IRS |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0xA0000064 | FSMC_SR3 | Reserved | FEMPT | IFEN | ILEN | IREN | IFS | ILS | IRS |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0xA0000044 | FSMC_SR4 | Reserved | FEMPT | IFEN | ILEN | IREN | IFS | ILS | IRS |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x0088 | FSMC_PMEM2 | MEMHIZ[7:0] | MEMWAIT[7:0] | MEMHOLD[7:0] | MEMSET[7:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x0088 | FSMC_PMEM3 | MEMHIZ[7:0] | MEMWAIT[7:0] | MEMHOLD[7:0] | MEMSET[7:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x00A8 | FSMC_PMEM4 | MEMHIZ[7:0] | MEMWAIT[7:0] | MEMHOLD[7:0] | MEMSET[7:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x008C | FSMC_PATT2 | ATTIIZ[7:0] | ATTWAIT[7:0] | ATTHOLD[7:0] | ATTSET[7:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x008C | FSMC_PATT3 | ATTIIZ[7:0] | ATTWAIT[7:0] | ATTHOLD[7:0] | ATTSET[7:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x00AC | FSMC_PATT4 | ATTIIZ[7:0] | ATTWAIT[7:0] | ATTHOLD[7:0] | ATTSET[7:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x0080 | FSMC_PIO4 | IOHIZ[7:0] | IOWAIT[7:0] | IOHOLD[7:0] | IOSET[7:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x00740014 | FSMC_ECCR2 | ECC[31:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x00940xA000 | FSMC_ECCR3 | ECC[31:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Refer to Table 1 on page 64 for the register boundary addresses.

RM0090 Rev 18
1601/1749

--- Page 42 ---

Flexible memory controller (FMC)
RM0090


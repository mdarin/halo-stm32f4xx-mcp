> [RM0090 ch.36 §36.6] NAND Flash/PC Card controller


### 36.6.8 NAND Flash/PC Card control registers

The NAND Flash/PC Card control registers have to be accessed by words (32 bits).

**PC Card/NAND Flash control registers 2..4 (FSMC_PCR2..4)**

Address offset: 0xA0000000 + 0x40 + 0x20 * (x – 1), x = 2..4
Reset value: 0x0000 0018

| 31 30 29 28 27 26 25 24 23 22 21 20 | 19 18 17 | 16 15 14 | 13 12 11 | 10 | 9 | 8 | 7 | 6 5 | 4 | 3 | 2 | 1 | 0 |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved | ECCPS[2:0] | TAR[2:0] | TCLR[2:0] | Res. | ECCEN | PWID[1:0] | PTYP | PBKEN | PWAITEN | Reserved |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  | rw |  | rw |  |


Bits 31:20 Reserved, must be kept at reset value.
Bits 19:17 **ECCPS[2:0]**: ECC page size.
Defines the page size for the extended ECC:
000: 256 bytes
001: 512 bytes
010: 1024 bytes
011: 2048 bytes
100: 4096 bytes
101: 8192 bytes

Bits 16:13 **TAR[2:0]**: ALE to RE delay.
Sets time from ALE low to RE low in number of AHB clock cycles (HCLK).
Time is: t_ar = (TAR + SET + 2) × THCLK where THCLK is the HCLK clock period
0000: 1 HCLK cycle (default)
1111: 16 HCLK cycles
*Note: SET is MEMSET or ATTSET according to the addressed space.*

Bits 12:9 **TCLR[2:0]**: CLE to RE delay.
Sets time from CLE low to RE low in number of AHB clock cycles (HCLK).
Time is t_clr = (TCLR + SET + 2) × THCLK where THCLK is the HCLK clock period
0000: 1 HCLK cycle (default)
1111: 16 HCLK cycles
*Note: SET is MEMSET or ATTSET according to the addressed space.*

Bits 8:7 Reserved, must be kept at reset value.
Bit 6 **ECCEN**: ECC computation logic enable bit
0: ECC logic is disabled and reset (default after reset),
1: ECC logic is enabled.

ST
RM0090 Rev 18
1593/1749

--- Page 34 ---

Flexible static memory controller (FSMC)
RM0090

Bits 5:4 **PWID[1:0]**: Databus width.
Defines the external memory device width.
00: 8 bits
01: 16 bits (default after reset). This value is mandatory for PC Cards.
10: reserved, do not use
11: reserved, do not use

Bit 3 **PTYP**: Memory type.
Defines the type of device attached to the corresponding memory bank:
0: PC Card, CompactFlash, CF+ or PCMCIA
1: NAND Flash (default after reset)

Bit 2 **PBKEN**: PC Card/NAND Flash memory bank enable bit.
Enables the memory bank. Accessing a disabled memory bank causes an ERROR on AHB bus
0: Corresponding memory bank is disabled (default after reset)
1: Corresponding memory bank is enabled

Bit 1 **PWAITEN**: Wait feature enable bit.
Enables the Wait feature for the PC Card/NAND Flash memory bank:
0: disabled
1: enabled
Note: For a PC Card, when the wait feature is enabled, the MEMWAITx/ATTWAITx/IOWAITx bits must be programmed to a value as follows:
xxWAITx ≥ 4 + max_wait_assertion_time/HCLK
Where max_wait_assertion_time is the maximum time taken by NWAIT to go low once nOE/nWE or nIORD/nIOWR is low.

Bit 0 Reserved, must be kept at reset value.

**FIFO status and interrupt register 2..4 (FSMC_SR2..4)**

Address offset: 0xA000 0000 + 0x44 + 0x20 * (x-1), x = 2..4
Reset value: 0x0000 0040

This register contains information about FIFO status and interrupt. The FSMC has a FIFO that is used when writing to memories to store up to16 words of data from the AHB. This is used to quickly write to the AHB and free it for transactions to peripherals other than the FSMC, while the FSMC is draining its FIFO into the memory. This register has one of its bits that indicates the status of the FIFO, for ECC purposes.
The ECC is calculated while the data are written to the memory, so in order to read the correct ECC the software must wait until the FIFO is empty.

| Reserved | FEMPT | IFEN | ILEN | IREN | IFS | ILS | IRS |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | r | rw |  |  |  |  |  |


1594/1749
RM0090 Rev 18
ST

--- Page 35 ---

RM0090
Flexible static memory controller (FSMC)

Bits 31:7 Reserved, must be kept at reset value.

Bit 6 **FEMPT**: FIFO empty.
Read-only bit that provides the status of the FIFO
0: FIFO not empty
1: FIFO empty

Bit 5 **IFEN**: Interrupt falling edge detection enable bit
0: Interrupt falling edge detection request disabled
1: Interrupt falling edge detection request enabled

Bit 4 **ILEN**: Interrupt high-level detection enable bit
0: Interrupt high-level detection request disabled
1: Interrupt high-level detection request enabled

Bit 3 **IREN**: Interrupt rising edge detection enable bit
0: Interrupt rising edge detection request disabled
1: Interrupt rising edge detection request enabled

Bit 2 **IFS**: Interrupt falling edge status
The flag is set by hardware and reset by software.
0: No interrupt falling edge occurred
1: Interrupt falling edge occurred

*Note: This bit is set by programming it to 1 by software.*

Bit 1 **ILS**: Interrupt high-level status
The flag is set by hardware and reset by software.
0: No Interrupt high-level occurred
1: Interrupt high-level occurred

Bit 0 **IRS**: Interrupt rising edge status
The flag is set by hardware and reset by software.
0: No interrupt rising edge occurred
1: Interrupt rising edge occurred

*Note: This bit is set by programming it to 1 by software.*


> [RM0090 ch.29 §29.17] SAI registers


## 29.17.2 SAI xConfiguration register 2 (SAI_xCR2) where x is A or B

Address offset: Block A: 0x008
Address offset: Block B: 0x028
Reset value: 0x0000 0000

| 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|
| Reserved |  |  |  |  |  |  |  |
| 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0 |  |  |  |  |  |  |  |
| COMP[1:0] | CPL | MUTECNT[5:0] | MUTE VAL | Mute | TRIS | FFLUS | FTH |
| rw |  |  |  |  |  |  |  |


Bits 31:16 Reserved, always read as 0
Bits 15:14 **COMP[1:0]**: Companding mode. These bits are set and cleared by software.
00: No companding algorithm
01: Reserved.
10: μ-Law algorithm
11: A-Law algorithm
The μ-Law and the A-Law log are a part of the CCITT G.711 recommendation, the type of complement that will be used depends on *Complement* bit.
The data expansion or data compression are determined by the state of bit MODE[0].
The data compression is applied if the audio block is configured as a transmitter.
The data expansion is automatically applied when the audio block is configured as a receiver.
Refer to Section 29.12.3 for more details.
*Note: Companding mode is applicable only when TDM is selected.*

Bit 13 **CPL**: Complement bit. This bit is set and cleared by software.
It defines the type of complement to be used for companding mode
0: 1’s complement representation.
1: 2’s complement representation.
*Note: This bit has effect only when the companding mode is μ-Law algorithm or A-Law algorithm.*

Bits 12:7 **MUTECNT[5:0]**: Mute counter. These bits are set and cleared by software.
These bits are used only in reception mode.
The value set in these bits is compared to the number of consecutive mute frames detected in reception. When the number of mute frames is equal to this value, the flag MUTEDDET will be set and an interrupt will be generated if bit MUTEDTIE is set.
Refer to Section 29.12.1 for more details.

STI
RM0090 Rev 18
953/1749

--- Page 174 ---

**Serial audio interface (SAI)** RM0090

Bit 6 **MUTEVAL**: Mute value. This bit is set and cleared by software. This bit has to be written before enabling the audio block: SAxEN.
0: Bit value 0 is sent during the MUTE mode.
1: Last values are sent during the MUTE mode.
This bit has a meaning only when the audio block is a transmitter and when the number of slots is lower or equal to 2 and if the MUTE bit is set.
If more slots are declared, the bit value sent during the transmission in mute mode will be equal to 0, whatever the value of this MUTEVAL bit.
if the number of slot is lower or equal to 2 and MUTEVAL = 1, the mute value transmitted for each slot will be the ones sent during the previous frame.
Refer to Section 29.12.1 for more details.

Bit 5 **MUTE**: Mute. This bit is set and cleared by software.
0: No Mute mode.
1: Mute mode enabled.
This bit has a meaning only when the audio block is a transmitter. The MUTE value is linked to the MUTEVAL value if the number of slots is lower or equal to 2, or equal to 0 if it is greater than 2.
Refer to Section 29.12.1 for more details.

Bit 4 **TRIS**: Tristate management on data line. This bit is set and cleared by software.
0: SD output line is still driven by the SAI when a slot is inactive.
1: SD output line is released (HI-Z) at the end of the last data bit of the last active slot if the next one is inactive.
This bit has a meaning only if the audio block is configured to be a transmitter.
This bit should be configured when SAI is disabled.
Refer to Section 29.12.4 for more details.

Bit 3 **FFLUSH**: FIFO flush. This bit is set by software. It is always read low.
0: No FIFO flush.
1: FIFO flush.
Writing 1 to the bit triggers the FIFO Flush. All the internal FIFO pointers (read and write) are cleared.
Data still present in the FIFO will be lost in such case (no more transmission or received data lost).
This bit should be configured when SAI is disabled.
Before flushing SAI, DMA stream/interruption must be disabled

Bits 2:0 **FTH**: FIFO threshold. This bit is set and cleared by software.
000: FIFO empty
001: ¼ FIFO
010: ½ FIFO
011: ¾ FIFO
100: FIFO full
101: Reserved
110: Reserved
111: Reserved

954/1749 RM0090 Rev 18 ST

--- Page 175 ---

RM0090
Serial audio interface (SAI)


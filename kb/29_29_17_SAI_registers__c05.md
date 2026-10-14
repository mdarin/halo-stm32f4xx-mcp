> [RM0090 ch.29 §29.17] SAI registers


### 29.17.4 SAI xSlot register (SAI_xSLOTR) where x is A or B

Address offset: Block A: 0x010
Address offset: Block B: 0x030
Reset value: 0x0000 0000

*Note:* This register has no meaning in AC'97 audio protocol

| SLOTEN[15:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reserved | NBSLOT[3:0] | SLOTSZ[1:0] | Res | FBOFF[4:0] |  |  |  |  |  |  |  |  |  |  |  |
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 |
| 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |


Bits 31:16 **SLOTEN[15:0]**: Slot enable. These bits are set and cleared by software.
Each bit of the SLOTEN bits identify a slot position from 0 to 15 (maximum 16 slots)
0: Inactive slot.
1: Active slot.
These bits must be set when the audio block is disabled.
They are ignored in AC'97 mode.

Bits 15:12 Reserved, always read as 0.

Bits 11:8 **NBSLOT[3:0]**: Number of slots in an audio frame. These bits are set and cleared by software.
The value set in these bits register represents the number of slots + 1 in the audio frame (including the number of inactive slots). The maximum number of slots is 16.
The number of slots should be even if bit FSDEF in the SAI_AFRCR register is set.
If the size is greater than the data size, the remaining bits will be forced to 0 if bit TRIS in the SAI_xCR1 register is clear, otherwise they will be forced to 0 if the next slot is active or the SD line will be forced to HI-Z if the next slot is inactive and bit TRIS = 1.
These bits must be set when the audio block is disabled.
They are ignored in AC'97 omode.

Bits 7:6 **SLOTSZ[1:0]**: Slot size
This bits is set and cleared by software.
00: The slot size is equivalent to the data size (specified in DS[3:0] in the SAI_ACR1 register).
01: 16-bit
10: 32-bit
11: Reserved
The slot size must be greater or equal to the data size. If this condition is not respected, the behavior of the SAI will be undetermined.
These bits must be set when the audio block is disabled.
They are ignored in AC'97 mode.

Bit 1 Reserved, always read as 0.

Bits 4:0 **FBOFF[4:0]**: First bit offset
These bits are set and cleared by software.
The value set in these bits represents the position of the first data transfer bit in the slot. It represents an offset value. During this offset phase 0 value are sent on the data line for transmission mode. For reception mode, the received bit are discarded during the offset phase.
These bits must be set when the audio block is disabled.
They are ignored in AC'97 mode.

ST
RM0090 Rev 18
957/1749

--- Page 178 ---

**Serial audio interface (SAI)**
RM0090


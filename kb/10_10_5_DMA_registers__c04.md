> [RM0090 ch.10 §10.5] DMA registers


### 10.5.5 DMA stream x configuration register (DMA_SxCR) (x = 0..7)

This register is used to configure the concerned stream.
Address offset: 0x10 + 0x18 × *stream number*
Reset value: 0x0000 0000

| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved | CHSEL[2:0] | MBURST[1:0] | PBURST[1:0] | Reserved | CT | DBM | PL[1:0] |  |  |  |  |  |  |  |  |
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
| PINCOS | MSIZE[1:0] | PSIZE[1:0] | MINC | PINC | CIRC | DIR[1:0] | PFCTRL | TCIE | HTIE | TEIE | DMEIE | EN |  |  |  |
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bits 31:28 Reserved, must be kept at reset value.
Bits 27:25 **CHSEL[2:0]**: Channel selection
These bits are set and cleared by software.
000: channel 0 selected
001: channel 1 selected
010: channel 2 selected
011: channel 3 selected
100: channel 4 selected
101: channel 5 selected
110: channel 6 selected
111: channel 7 selected
These bits are protected and can be written only if EN is '0'

Bits 24:23 **MBURST**: Memory burst transfer configuration
These bits are set and cleared by software.
00: single transfer
01: INCR4 (incremental burst of 4 beats)
10: INCR8 (incremental burst of 8 beats)
11: INCR16 (incremental burst of 16 beats)
These bits are protected and can be written only if EN is '0'
In direct mode, these bits are forced to 0x0 by hardware as soon as bit EN= '1'.

Bits 22:21 **PBURST[1:0]**: Peripheral burst transfer configuration
These bits are set and cleared by software.
00: single transfer
01: INCR4 (incremental burst of 4 beats)
10: INCR8 (incremental burst of 8 beats)
11: INCR16 (incremental burst of 16 beats)
These bits are protected and can be written only if EN is '0'
In direct mode, these bits are forced to 0x0 by hardware.

Bit 20 Reserved, must be kept at reset value.

328/1749
RM0090 Rev 18

--- Page 134 ---

RM0090
DMA controller (DMA)

Bit 19 **CT**: Current target (only in double buffer mode)
This bits is set and cleared by hardware. It can also be written by software.
0: The current target memory is Memory 0 (addressed by the DMA_SxM0AR pointer)
1: The current target memory is Memory 1 (addressed by the DMA_SxM1AR pointer)
This bit can be written only if EN is '0' to indicate the target memory area of the first transfer.
Once the stream is enabled, this bit operates as a status flag indicating which memory area is the current target.

Bit 18 **DBM**: Double buffer mode
This bits is set and cleared by software.
0: No buffer switching at the end of transfer
1: Memory target switched at the end of the DMA transfer
This bit is protected and can be written only if EN is '0'.

Bits 17:16 **PL[1:0]**: Priority level
These bits are set and cleared by software.
00: Low
01: Medium
10: High
11: Very high
These bits are protected and can be written only if EN is '0'.

Bit 15 **PINCOS**: Peripheral increment offset size
This bit is set and cleared by software
0: The offset size for the peripheral address calculation is linked to the PSIZE
1: The offset size for the peripheral address calculation is fixed to 4 (32-bit alignment).
This bit has no meaning if bit PINC = '0'.
This bit is protected and can be written only if EN is '0'.
This bit is forced low by hardware when the stream is enabled (bit EN = '1') if the direct mode is selected or if PBURST are different from "00".

Bits 14:13 **MSIZE[1:0]**: Memory data size
These bits are set and cleared by software.
00: byte (8-bit)
01: half-word (16-bit)
10: word (32-bit)
11: reserved
These bits are protected and can be written only if EN is '0'.
In direct mode, MSIZE is forced by hardware to the same value as PSIZE as soon as bit EN = '1'.

Bits 12:11 **PSIZE[1:0]**: Peripheral data size
These bits are set and cleared by software.
00: Byte (8-bit)
01: Half-word (16-bit)
10: Word (32-bit)
11: reserved
These bits are protected and can be written only if EN is '0'

Bit 10 **MINC**: Memory increment mode
This bit is set and cleared by software.
0: Memory address pointer is fixed
1: Memory address pointer is incremented after each data transfer (increment is done according to MSIZE)
This bit is protected and can be written only if EN is '0'.

ST
RM0090 Rev 18
329/1749

--- Page 135 ---

**DMA controller (DMA)**

RM0090

**Bit 9 PINC**: Peripheral increment mode
This bit is set and cleared by software.
0: Peripheral address pointer is fixed
1: Peripheral address pointer is incremented after each data transfer (increment is done according to PSIZE)
This bit is protected and can be written only if EN is '0'.

**Bit 8 CIRC**: Circular mode
This bit is set and cleared by software and can be cleared by hardware.
0: Circular mode disabled
1: Circular mode enabled
When the peripheral is the flow controller (bit PFCTRL=1) and the stream is enabled (bit EN=1), then this bit is automatically forced by hardware to 0.
It is automatically forced by hardware to 1 if the DBM bit is set, as soon as the stream is enabled (bit EN ='1').

**Bits 7:6 DIR[1:0]**: Data transfer direction
These bits are set and cleared by software.
00: Peripheral-to-memory
01: Memory-to-peripheral
10: Memory-to-memory
11: reserved
These bits are protected and can be written only if EN is '0'.

**Bit 5 PFCTRL**: Peripheral flow controller
This bit is set and cleared by software.
0: The DMA is the flow controller
1: The peripheral is the flow controller
This bit is protected and can be written only if EN is '0'.
When the memory-to-memory mode is selected (bits DIR[1:0]=10), then this bit is automatically forced to 0 by hardware.

**Bit 4 TCIE**: Transfer complete interrupt enable
This bit is set and cleared by software.
0: TC interrupt disabled
1: TC interrupt enabled

**Bit 3 HTIE**: Half transfer interrupt enable
This bit is set and cleared by software.
0: HT interrupt disabled
1: HT interrupt enabled

**Bit 2 TEIE**: Transfer error interrupt enable
This bit is set and cleared by software.
0: TE interrupt disabled
1: TE interrupt enabled

**Bit 1 DMEIE**: Direct mode error interrupt enable
This bit is set and cleared by software.
0: DME interrupt disabled
1: DME interrupt enabled

330/1749 RM0090 Rev 18 ST

--- Page 136 ---

RM0090
DMA controller (DMA)

Bit 0 **EN**: Stream enable / flag stream ready when read low
This bit is set and cleared by software.
0: Stream disabled
1: Stream enabled
This bit may be cleared by hardware:
- on a DMA end of transfer (stream ready to be configured)
- if a transfer error occurs on the AHB master buses
- when the FIFO threshold on memory AHB port is not compatible with the size of the burst
When this bit is read as 0, the software is allowed to program the Configuration and FIFO bits registers. It is forbidden to write these registers when the EN bit is read as 1.
*Note: Before setting EN bit to '1' to start a new transfer, the event flags corresponding to the stream in DMA_LISR or DMA_HISR register must be cleared.*

> [RM0090 ch.10 §10.5] DMA registers

### 10.5.6 DMA stream x number of data register (DMA_SxNDTR) (x = 0..7)

### 10.5.6 DMA stream x number of data register (DMA_SxNDTR) (x = 0..7)

Address offset: 0x14 + 0x18 × *stream number*
Reset value: 0x0000 0000

| 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 |
|---|
| Reserved |
| 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0 |
| NDT[15:0] |
| rw |


Bits 31:16 Reserved, must be kept at reset value.
Bits 15:0 **NDT[15:0]**: Number of data items to transfer
Number of data items to be transferred (0 up to 65535). This register can be written only when the stream is disabled. When the stream is enabled, this register is read-only, indicating the remaining data items to be transmitted. This register decrements after each DMA transfer.
Once the transfer has completed, this register can either stay at zero (when the stream is in normal mode) or be reloaded automatically with the previously programmed value in the following cases:
- when the stream is configured in Circular mode.
- when the stream is enabled again by setting EN bit to '1'
If the value of this register is zero, no transaction can be served even if the stream is enabled.

STI
RM0090 Rev 18
331/1749

--- Page 137 ---

**DMA controller (DMA)**
RM0090

> [RM0090 ch.10 §10.5] DMA registers

### 10.5.7 DMA stream x peripheral address register (DMA_SxPAR) (x = 0..7)

### 10.5.7 DMA stream x peripheral address register (DMA_SxPAR) (x = 0..7)

Address offset: 0x18 + 0x18 × *stream number*
Reset value: 0x0000 0000

| PAR[31:16] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 |
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| PAR[15:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bits 31:0 **PAR[31:0]**: Peripheral address
Base address of the peripheral data register from/to which the data will be read/written.
These bits are write-protected and can be written only when bit EN = '0' in the DMA_SxCR register.

> [RM0090 ch.10 §10.5] DMA registers

### 10.5.8 DMA stream x memory 0 address register (DMA_SxM0AR) (x = 0..7)

### 10.5.8 DMA stream x memory 0 address register (DMA_SxM0AR) (x = 0..7)

Address offset: 0x1C + 0x18 × *stream number*
Reset value: 0x0000 0000

| MOA[31:16] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 |
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| MOA[15:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bits 31:0 **MOA[31:0]**: Memory 0 address
Base address of Memory area 0 from/to which the data will be read/written.
These bits are write-protected. They can be written only if:
- the stream is disabled (bit EN= '0' in the DMA_SxCR register) or
- the stream is enabled (EN='1' in DMA_SxCR register) and bit CT = '1' in the DMA_SxCR register (in Double buffer mode).

> [RM0090 ch.10 §10.5] DMA registers

### 10.5.9 DMA stream x memory 1 address register (DMA_SxM1AR) (x = 0..7)

### 10.5.9 DMA stream x memory 1 address register (DMA_SxM1AR) (x = 0..7)

Address offset: 0x20 + 0x18 × *stream number*
Reset value: 0x0000 0000

| M1A[31:16] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 |
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| M1A[15:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


332/1749
RM0090 Rev 18

--- Page 138 ---

RM0090
DMA controller (DMA)

Bits 31:0 **M1A[31:0]**: Memory 1 address (used in case of Double buffer mode)
Base address of Memory area 1 from/to which the data will be read/written.
This register is used only for the Double buffer mode.
These bits are write-protected. They can be written only if:
- the stream is disabled (bit EN= '0' in the DMA_SxCR register) or
- the stream is enabled (EN='1' in DMA_SxCR register) and bit CT = '0' in the DMA_SxCR register.


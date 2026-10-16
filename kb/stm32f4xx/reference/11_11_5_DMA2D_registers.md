**11.5 DMA2D registers**

**11.5.1 DMA2D control register (DMA2D_CR)**

Address offset: 0x0000
Reset value: 0x0000 0000

31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16
Reserved MODE
15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0
Reserved CEIE CTCIE CAEIF TWIE TCIE TEIE Reserved ABORT SUSP START
rw rw rw rw rw rw rs rw rs

Bits 31:18 Reserved, must be kept at reset value
Bits 17:16 **MODE**: DMA2D mode
This bit is set and cleared by software. It cannot be modified while a transfer is ongoing.
00: Memory-to-memory (FG fetch only)
01: Memory-to-memory with PFC (FG fetch only with FG PFC active)
10: Memory-to-memory with blending (FG and BG fetch with PFC and blending)
11: Register-to-memory (no FG nor BG, only output stage active)

Bits 15:14 Reserved, must be kept at reset value
Bit 13 **CEIE**: Configuration Error Interrupt Enable
This bit is set and cleared by software.
0: CE interrupt disable
1: CE interrupt enable

Bit 12 **CTCIE**: CLUT transfer complete interrupt enable
This bit is set and cleared by software.
0: CTC interrupt disable
1: CTC interrupt enable

Bit 11 **CAEIE**: CLUT access error interrupt enable
This bit is set and cleared by software.
0: CAE interrupt disable
1: CAE interrupt enable

352/1749 RM0090 Rev 18

--- Page 158 ---

RM0090
Chrom-Art Accelerator™ controller (DMA2D)

Bit 10 **TWIE**: Transfer watermark interrupt enable
This bit is set and cleared by software.
0: TW interrupt disable
1: TW interrupt enable

Bit 9 **TCIE**: Transfer complete interrupt enable
This bit is set and cleared by software.
0: TC interrupt disable
1: TC interrupt enable

Bit 8 **TEIE**: Transfer error interrupt enable
This bit is set and cleared by software.
0: TE interrupt disable
1: TE interrupt enable

Bits 7:3 Reserved, must be kept at reset value

Bit 2 **ABORT**: Abort
This bit can be used to abort the current transfer. This bit is set by software and is automatically reset by hardware when the START bit is reset.
0: No transfer abort requested
1: Transfer abort requested

Bit 1 **SUSP**: Suspend
This bit can be used to suspend the current transfer. This bit is set and reset by software. It is automatically reset by hardware when the START bit is reset.
0: Transfer not suspended
1: Transfer suspended

Bit 0 **START**: Start
This bit can be used to launch the DMA2D according to the parameters loaded in the various configuration registers. This bit is automatically reset by the following events:
- At the end of the transfer
- When the data transfer is aborted by the user application by setting the ABORT bit in DMA2D_CR
- When a data transfer error occurs
- When the data transfer has not started due to a configuration error or another transfer operation already ongoing (automatic CLUT loading).

ST
RM0090 Rev 18
353/1749

--- Page 159 ---

Chrom-Art Accelerator™ controller (DMA2D)
RM0090

### 11.5.2 DMA2D Interrupt Status Register (DMA2D_ISR)

Address offset: 0x0004
Reset value: 0x0000 0000

| 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved |  |  |  |  |  |  |  |  |  |  |  |
| 15 14 13 12 11 10 9 8 7 6 | 5 | 4 | 3 | 2 | 1 | 0 |  |  |  |  |  |
| Reserved | CEIF | CTCIF | CAEIF | TWIF | TCIF | TEIF |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  | r |


Bits 31:6 Reserved, must be kept at reset value

**Bit 5 CEIF**: Configuration error interrupt flag
This bit is set when the START bit of DMA2D_CR, DMA2DFGPFCR or DMA2D_BGPFCR is set and a wrong configuration has been programmed.

**Bit 4 CTCIF**: CLUT transfer complete interrupt flag
This bit is set when the CLUT copy from a system memory area to the internal DMA2D memory is complete.

**Bit 3 CAEIF**: CLUT access error interrupt flag
This bit is set when the CPU accesses the CLUT while the CLUT is being automatically copied from a system memory to the internal DMA2D.

**Bit 2 TWIF**: Transfer watermark interrupt flag
This bit is set when the last pixel of the watermarked line has been transferred.

**Bit 1 TCIF**: Transfer complete interrupt flag
This bit is set when a DMA2D transfer operation is complete (data transfer only).

**Bit 0 TEIF**: Transfer error interrupt flag
This bit is set when an error occurs during a DMA transfer (data transfer or automatic CLUT loading).

354/1749
RM0090 Rev 18

--- Page 160 ---

RM0090
Chrom-Art Accelerator™ controller (DMA2D)

### 11.5.3 DMA2D interrupt flag clear register (DMA2D_IFCR)

Address offset: 0x0008
Reset value: 0x0000 0000

| 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|
| Reserved |  |  |  |  |  |  |  |  |  |  |
| 15 14 13 12 11 10 9 8 7 6 | 5 | 4 | 3 | 2 | 1 | 0 |  |  |  |  |
| Reserved | CCEIF | CCTCIF | CAECIF | CTWIF | CTCIF | CTEIF |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  | rc_w1 |


Bits 31:6 Reserved, must be kept at reset value

**Bit 5 CCEIF**: Clear configuration error interrupt flag
Programming this bit to 1 clears the CEIF flag in the DMA2D_ISR register

**Bit 4 CCTCIF**: Clear CLUT transfer complete interrupt flag
Programming this bit to 1 clears the CTCIF flag in the DMA2D_ISR register

**Bit 3 CAECIF**: Clear CLUT access error interrupt flag
Programming this bit to 1 clears the CAEIF flag in the DMA2D_ISR register

**Bit 2 CTWIF**: Clear transfer watermark interrupt flag
Programming this bit to 1 clears the TWIF flag in the DMA2D_ISR register

**Bit 1 CTCIF**: Clear transfer complete interrupt flag
Programming this bit to 1 clears the TCIF flag in the DMA2D_ISR register

**Bit 0 CTEIF**: Clear Transfer error interrupt flag
Programming this bit to 1 clears the TEIF flag in the DMA2D_ISR register

STI
RM0090 Rev 18
355/1749

--- Page 161 ---

Chrom-Art Accelerator™ controller (DMA2D)
RM0090

### 11.5.4 DMA2D foreground memory address register (DMA2D_FGMAR)

Address offset: 0x000C
Reset value: 0x0000 0000

| 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 |
|---|
| MA[31:16] |
| rw |
| 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0 |
| MA[15:0] |
| rw |


Bits 31:0 **MA[31: 0]**: Memory address
Address of the data used for the foreground image. This register can only be written when data transfers are disabled. Once the data transfer has started, this register is read-only.
The address alignment must match the image format selected e.g. a 32-bit per pixel format must be 32-bit aligned, a 16-bit per pixel format must be 16-bit aligned and a 4-bit per pixel format must be 8-bit aligned.

### 11.5.5 DMA2D foreground offset register (DMA2D_FGOR)

Address offset: 0x0010
Reset value: 0x0000 0000

| 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 |  |
|---|---|
| Reserved |  |
| 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0 |  |
| Reserved | LO[13:0] |
| rw |  |


Bits 31:14 Reserved, must be kept at reset value
Bits 13:0 **LO[13: 0]**: Line offset
Line offset used for the foreground expressed in pixel. This value is used to generate the address. It is added at the end of each line to determine the starting address of the next line.
These bits can only be written when data transfers are disabled. Once a data transfer has started, they become read-only.
If the image format is 4-bit per pixel, the line offset must be even.

356/1749
RM0090 Rev 18
ST

--- Page 162 ---

RM0090
Chrom-Art Accelerator™ controller (DMA2D)

### 11.5.6 DMA2D background memory address register (DMA2D_BGMAR)

Address offset: 0x0014
Reset value: 0x0000 0000

| 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 |
|---|
| MA[31:16] |
| rw |
| 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0 |
| MA[15:0] |
| rw |


Bits 31:0 **MA[31: 0]**: Memory address
Address of the data used for the background image. This register can only be written when data transfers are disabled. Once a data transfer has started, this register is read-only.
The address alignment must match the image format selected e.g. a 32-bit per pixel format must be 32-bit aligned, a 16-bit per pixel format must be 16-bit aligned and a 4-bit per pixel format must be 8-bit aligned.

### 11.5.7 DMA2D background offset register (DMA2D_BGOR)

Address offset: 0x0018
Reset value: 0x0000 0000

| 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 |  |
|---|---|
| Reserved |  |
| 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0 |  |
| Reserved | LO[13:0] |
| rw |  |


Bits 31:14 Reserved, must be kept at reset value
Bits 13:0 **LO[13: 0]**: Line offset
Line offset used for the background image (expressed in pixel). This value is used for the address generation. It is added at the end of each line to determine the starting address of the next line.
These bits can only be written when data transfers are disabled. Once data transfer has started, they become read-only.
If the image format is 4-bit per pixel, the line offset must be even.

STI
RM0090 Rev 18
357/1749

--- Page 163 ---

Chrom-Art Accelerator™ controller (DMA2D)
RM0090

### 11.5.8 DMA2D foreground PFC control register (DMA2D_FGPFCCR)

Address offset: 0x001C
Reset value: 0x0000 0000

| 31 30 29 28 27 26 25 24 | 23 22 21 20 19 18 | 17 16 |  |  |  |  |  |
|---|---|---|---|---|---|---|---|
| ALPHA[7:0] | Reserved | AM[1:0] |  |  |  |  |  |
| rw |  |  |  |  |  |  | rw |
| 15 14 13 12 11 10 9 8 | 7 6 | 5 | 4 | 3 2 1 0 |  |  |  |
| CS[7:0] | Reserved | START | CCM | CM[3:0] |  |  |  |
| rw |  |  | rs | rw |  |  |  |


**Bits 31:24 ALPHA[7: 0]: Alpha value**
These bits define a fixed alpha channel value which can replace the original alpha value or be multiplied by the original alpha value according to the alpha mode selected through the AM[1:0] bits.
These bits can only be written when data transfers are disabled. Once a transfer has started, they become read-only.

**Bits 23:18** Reserved, must be kept at reset value

**Bits 17:16 AM[1: 0]: Alpha mode**
These bits select the alpha channel value to be used for the foreground image. They can only be written data the transfer are disabled. Once the transfer has started, they become read-only.
00: No modification of the foreground image alpha channel value
01: Replace original foreground image alpha channel value by ALPHA[7: 0]
10: Replace original foreground image alpha channel value by ALPHA[7:0] multiplied with original alpha channel value
other configurations are meaningless

**Bits 15:8 CS[7: 0]: CLUT size**
These bits define the size of the CLUT used for the foreground image. Once the CLUT transfer has started, this field is read-only.
The number of CLUT entries is equal to CS[7:0] + 1.

**Bits 7:6** Reserved, must be kept at reset value

358/1749
RM0090 Rev 18
ST

--- Page 164 ---

RM0090
Chrom-Art Accelerator™ controller (DMA2D)

**Bit 5 START**: Start
This bit can be set to start the automatic loading of the CLUT. It is automatically reset:
- at the end of the transfer
- when the transfer is aborted by the user application by setting the ABORT bit in DMA2D_CR
- when a transfer error occurs
- when the transfer has not started due to a configuration error or another transfer operation already ongoing (data transfer or automatic background CLUT transfer).

**Bit 4 CCM**: CLUT color mode
This bit defines the color format of the CLUT. It can only be written when the transfer is disabled. Once the CLUT transfer has started, this bit is read-only.
0: ARGB8888
1: RGB888
others: meaningless

**Bits 3:0 CM[3: 0]**: Color mode
These bits defines the color format of the foreground image. They can only be written when data transfers are disabled. Once the transfer has started, they are read-only.
0000: ARGB8888
0001: RGB888
0010: RGB565
0011: ARGB1555
0100: ARGB4444
0101: L8
0110: AL44
0111: AL88
1000: L4
1001: A8
1010: A4
others: meaningless

ST
RM0090 Rev 18
359/1749

--- Page 165 ---

Chrom-Art Accelerator™ controller (DMA2D)
RM0090

### 11.5.9 DMA2D foreground color register (DMA2D_FGCOLR)

Address offset: 0x0020
Reset value: 0x0000 0000

| 31 30 29 28 27 26 25 24 | 23 22 21 20 19 18 17 16 |
|---|---|
| Reserved | RED[7:0] |
| rw |  |
| 15 14 13 12 11 10 9 8 | 7 6 5 4 3 2 1 0 |
| GREEN[7:0] | BLUE[7:0] |
| rw |  |


Bits 31:24 Reserved, must be kept at reset value

Bits 23:16 **RED[7: 0]**: Red Value
These bits defines the red value for the A4 or A8 mode of the foreground image. They can only be written when data transfers are disabled. Once the transfer has started, they are read-only.

Bits 15:8 **GREEN[7: 0]**: Green Value
These bits defines the green value for the A4 or A8 mode of the foreground image. They can only be written when data transfers are disabled. Once the transfer has started, They are read-only.

Bits 7:0 **BLUE[7: 0]**: Blue Value
These bits defines the blue value for the A4 or A8 mode of the foreground image. They can only be written when data transfers are disabled. Once the transfer has started, They are read-only.

360/1749
RM0090 Rev 18

--- Page 166 ---

RM0090
Chrom-Art Accelerator™ controller (DMA2D)

### 11.5.10 DMA2D background PFC control register (DMA2D_BGPFCR)

Address offset: 0x0024
Reset value: 0x0000 0000

| 31 30 29 28 27 26 25 24 | 23 22 21 20 19 18 | 17 16 |  |  |
|---|---|---|---|---|
| ALPHA[7:0] | Reserved | AM[1:0] |  |  |
| rw | Reserved | rw |  |  |
| 15 14 13 12 11 10 9 8 | 7 6 | 5 | 4 | 3 2 1 0 |
| CS[7:0] | Reserved | START | CCM | CM[3:0] |
| rw | Reserved | rs | rw |  |


**Bits 31:24 ALPHA[7: 0]: Alpha value**
These bits define a fixed alpha channel value which can replace the original alpha value or be multiplied with the original alpha value according to the alpha mode selected with bits AM[1: 0]. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only.

**Bits 23:18 Reserved, must be kept at reset value**

**Bits 17:16 AM[1: 0]: Alpha mode**
These bits define which alpha channel value to be used for the background image.
These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only.
00: No modification of the foreground image alpha channel value
01: Replace original background image alpha channel value by ALPHA[7: 0]
10: Replace original background image alpha channel value by ALPHA[7:0] multiplied with original alpha channel value
others: meaningless

**Bits 15:8 CS[7: 0]: CLUT size**
These bits define the size of the CLUT used for the BG. Once the CLUT transfer has started, this field is read-only.
The number of CLUT entries is equal to CS[7:0] + 1.

**Bits 7:6 Reserved, must be kept at reset value**

ST
RM0090 Rev 18
361/1749

--- Page 167 ---

**Chrom-Art Accelerator™ controller (DMA2D)**
RM0090

**Bit 5 START**: Start
This bit is set to start the automatic loading of the CLUT. This bit is automatically reset:
- at the end of the transfer
- when the transfer is aborted by the user application by setting the ABORT bit in the DMA2D_CR
- when a transfer error occurs
- when the transfer has not started due to a configuration error or another transfer operation already on going (data transfer or automatic foreground CLUT transfer).

**Bit 4 CCM**: CLUT Color mode
These bits define the color format of the CLUT. This register can only be written when the transfer is disabled. Once the CLUT transfer has started, this bit is read-only.
0: ARGB8888
1: RGB888
others: meaningless

**Bits 3:0 CM[3: 0]**: Color mode
These bits define the color format of the foreground image. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only.
0000: ARGB8888
0001: RGB888
0010: RGB565
0011: ARGB1555
0100: ARGB4444
0101: L8
0110: AL44
0111: AL88
1000: L4
1001: A8
1010: A4
others: meaningless

362/1749
RM0090 Rev 18
STI

--- Page 168 ---

RM0090
Chrom-Art Accelerator™ controller (DMA2D)

### 11.5.11 DMA2D background color register (DMA2D_BGCOLR)

Address offset: 0x0028
Reset value: 0x0000 0000

| 31 | 23 |
|---|---|
| Reserved | RED[7:0] |
| rw |  |
| 15 | 7 |
| GREEN[7:0] | BLUE[7:0] |
| rw |  |


Bits 31:24 Reserved, must be kept at reset value
Bits 23:16 **RED[7: 0]**: Red Value
These bits define the red value for the A4 or A8 mode of the background. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only.
Bits 15:8 **GREEN[7: 0]**: Green Value
These bits define the green value for the A4 or A8 mode of the background. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only.
Bits 7:0 **BLUE[7: 0]**: Blue Value
These bits define the blue value for the A4 or A8 mode of the background. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only.

### 11.5.12 DMA2D foreground CLUT memory address register (DMA2D_FGCMAR)

Address offset: 0x002C
Reset value: 0x0000 0000

| MA[31:16] |
|---|
| rw |
| MA[15:0] |
| rw |


Bits 31: 0 **MA[31: 0]**: Memory Address
Address of the data used for the CLUT address dedicated to the foreground image. This register can only be written when no transfer is ongoing. Once the CLUT transfer has started, this register is read-only.
If the foreground CLUT format is 32-bit, the address must be 32-bit aligned.

ST
RM0090 Rev 18
363/1749

--- Page 169 ---

Chrom-Art Accelerator™ controller (DMA2D)
RM0090

### 11.5.13 DMA2D background CLUT memory address register (DMA2D_BGCMAR)

Address offset: 0x0030
Reset value: 0x0000 0000

| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| MA[31:16] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
| MA[15:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bits 31: 0 **MA[31: 0]**: Memory address
Address of the data used for the CLUT address dedicated to the background image. This register can only be written when no transfer is on going. Once the CLUT transfer has started, this register is read-only.
If the background CLUT format is 32-bit, the address must be 32-bit aligned.

### 11.5.14 DMA2D output PFC control register (DMA2D_OPFCCR)

Address offset: 0x0034
Reset value: 0x0000 0000

| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
| Reserved | CM[2:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  |  | rw |  |  |


Bits 31: 3 Reserved, must be kept at reset value
Bits 2: 0 **CM[2: 0]**: Color mode
These bits define the color format of the output image. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only.
000: ARGB8888
001: RGB888
010: RGB565
011: ARGB1555
100: ARGB4444
others: meaningless

364/1749
RM0090 Rev 18
ST

--- Page 170 ---

RM0090
Chrom-Art Accelerator™ controller (DMA2D)

### 11.5.15 DMA2D output color register (DMA2D_OCOLR)

Address offset: 0x0038
Reset value: 0x0000 0000

| 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 |  |  |  |
|---|---|---|---|
| ALPHA[7:0] | RED[7:0] |  |  |
| rw |  |  |  |
| 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0 |  |  |  |
| GREEN[7:0] | BLUE[7:0] |  |  |
| RED[4:0] | GREEN[5:0] | BLUE[4:0] |  |
| A | RED[4:0] | GREEN[4:0] | BLUE[4:0] |
| ALPHA[3:0] | RED[3:0] | GREEN[3:0] | BLUE[3:0] |
| rw |  |  |  |


**Bits 31:24 ALPHA[7: 0]:** Alpha Channel Value
These bits define the alpha channel of the output color. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only.

**Bits 23:16 RED[7: 0]:** Red Value
These bits define the red value of the output image. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only.

**Bits 15:8 GREEN[7: 0]:** Green Value
These bits define the green value of the output image. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only.

**Bits 7:0 BLUE[7: 0]:** Blue Value
These bits define the blue value of the output image. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only.

ST
RM0090 Rev 18
365/1749

--- Page 171 ---

Chrom-Art Accelerator™ controller (DMA2D)
RM0090

### 11.5.16 DMA2D output memory address register (DMA2D_OMAR)

Address offset: 0x003C
Reset value: 0x0000 0000

| 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 |
|---|
| MA[31:16] |
| rw |
| 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0 |
| MA[15:0] |
| rw |


Bits 31: 0 **MA[31: 0]**: Memory Address
Address of the data used for the output FIFO. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only.
The address alignment must match the image format selected e.g. a 32-bit per pixel format must be 32-bit aligned and a 16-bit per pixel format must be 16-bit aligned.

366/1749
RM0090 Rev 18

--- Page 172 ---

RM0090
Chrom-Art Accelerator™ controller (DMA2D)

### 11.5.17 DMA2D output offset register (DMA2D_OOR)

Address offset: 0x0040
Reset value: 0x0000 0000

| 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 |  |
|---|---|
| Reserved |  |
| 15 14 | 13 12 11 10 9 8 7 6 5 4 3 2 1 0 |
| Reserved | LO[13:0] |
| rw |  |


Bits 31:14 Reserved, must be kept at reset value
Bits 13:0 **LO[13: 0]**: Line Offset
Line offset used for the output (expressed in pixels). This value is used for the address generation. It is added at the end of each line to determine the starting address of the next line. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only.

### 11.5.18 DMA2D number of line register (DMA2D_NLR)

Address offset: 0x0044
Reset value: 0x0000 0000

| 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 |  |
|---|---|
| Reserved | PL[13:0] |
| rw |  |
| 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0 |  |
| NL[15:0] |  |
| rw |  |


Bits 31:30 Reserved, must be kept at reset value
Bits 29:16 **PL[13: 0]**: Pixel per lines
Number of pixels per lines of the area to be transferred. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only.
If any of the input image format is 4-bit per pixel, pixel per lines must be even.

Bits 15:0 **NL[15: 0]**: Number of lines
Number of lines of the area to be transferred. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only.

STI
RM0090 Rev 18
367/1749

--- Page 173 ---

Chrom-Art Accelerator™ controller (DMA2D)
RM0090

### 11.5.19 DMA2D line watermark register (DMA2D_LWR)

Address offset: 0x0048
Reset value: 0x0000 0000

| 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 |
|---|
| Reserved |
| 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0 |
| LW[15:0] |
| rw |


Bits 31:16 Reserved, must be kept at reset value
Bits 15:0 **LW[15:0]**: Line watermark
These bits allow to configure the line watermark for interrupt generation.
An interrupt is raised when the last pixel of the watermarked line has been transferred.
These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only.

### 11.5.20 DMA2D AHB master timer configuration register (DMA2D_AMTCR)

Address offset: 0x004C
Reset value: 0x0000 0000

| 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|
| Reserved |  |  |  |  |  |  |  |  |
| 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0 |  |  |  |  |  |  |  |  |
| DT[7:0] | Reserved | EN |  |  |  |  |  |  |
| rw |  |  |  |  |  |  |  | rw |


Bits 31:16 Reserved
Bits 15:8 **DT[7: 0]**: Dead Time
Dead time value in the AHB clock cycle inserted between two consecutive accesses on the AHB master port. These bits represent the minimum guaranteed number of cycles between two consecutive AHB accesses.
Bits 7:1 Reserved
Bit 0 **EN**: Enable
Enables the dead time functionality.

368/1749
RM0090 Rev 18
STI

--- Page 174 ---

11.5.21
**DMA2D register map**
The following table summarizes the DMA2D registers. Refer to *Section 2.3: Memory map*
for the DMA2D register base address.

**Table 60. DMA2D register map and reset values**

| Offset | Register |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
| 0x0000 | DMA2D_CR | Reserved | MODE[1:0] | Reserved | CEIE | CTIE | TCIE | TWIE | CAEIE | TOEIE | TEIE | Reserved | ABORT | SUSP | START |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x0004 | DMA2D_ISR | Reserved | CEIF | CTCIF | TCIF | TWIF | CAEIF | TOEIF | TEIF |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x0008 | DMA2D_IFCR | Reserved | CCEIF | CCTCIF | CTCIF | CTWIF | CAEIF | CTOIF | CTEIF |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x000C | DMA2D_FGMAR | MA[31:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x0010 | DMA2D_FGOR | Reserved | LO[13:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x0014 | DMA2D_BGMAR | MA[31:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x0018 | DMA2D_BGOR | Reserved | LO[13:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x001C | DMA2D_FGPFCCR | ALPHA[7:0] | Reserved | AM[1:0] | CST7:0] | Res | START | CCM | CM[3:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x0020 | DMA2D_FGCOLR | APLHA[7:0] | RED[7:0] | GREEN[7:0] | BLUE[7:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x0024 | DMA2D_BGPFCCR | ALPHA[7:0] | Reserved | AM[1:0] | CST7:0] | Res | START | CCM | CM[3:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x0028 | DMA2D_BGCOLR | APLHA[7:0] | RED[7:0] | GREEN[7:0] | BLUE[7:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x002C | DMA2D_FGCMAR | MA[31:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x0030 | DMA2D_BGCMAR | MA[31:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x0034 | DMA2D_OPFCCR | Reserved | CM[2:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x0038 | DMA2D_OCOLR | APLHA[7:0] | RED[7:0] | GREEN[7:0] | BLUE[7:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x003C | DMA2D_OMAR | MA[31:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


--- Page 175 ---

Chrom-Art Accelerator™ controller (DMA2D)
RM0090

Table 60. DMA2D register map and reset values (continued)

| Offset | Register | Bits |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
| 0x0040 | DMA2D_OOCR | Reserved | LO[13:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x0044 | DMA2D_NLR | Reset value | PL[13:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Res | NL[15:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x0048 | DMA2D_LWR | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x004C | DMA2D_AMTCR | Reset value | DT[7:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reserved | EN |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x0050-0x03FF | - | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x0400-0x07FF | DMA2D_FGCLUT | APHLHA[7:0][255:0] | RED[7:0][255:0] | GREEN[7:0][255:0] | BLUE[7:0][255:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x0800-0x0BFF | DMA2D_BGCLUT | APHLHA[7:0][255:0] | RED[7:0][255:0] | GREEN[7:0][255:0] | BLUE[7:0][255:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


370/1749
RM0090 Rev 18

--- Page 176 ---

RM0090
Interrupts and events


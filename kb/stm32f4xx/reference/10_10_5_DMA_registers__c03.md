> [RM0090 ch.10 §10.5] DMA registers


### 10.5.2 DMA high interrupt status register (DMA_HISR)

Address offset: 0x04
Reset value: 0x0000 0000

| 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved | TCIF7 | HTIF7 | TEIF7 | DMEIF7 | Reserved | FEIF7 | TCIF6 | HTIF6 | TEIF6 | DMEIF6 | Reserved | FEIF6 |
| r |  |  |  |  |  |  |  |  |  |  |  |  |
| 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0 |  |  |  |  |  |  |  |  |  |  |  |  |
| Reserved | TCIF5 | HTIF5 | TEIF5 | DMEIF5 | Reserved | FEIF5 | TCIF4 | HTIF4 | TEIF4 | DMEIF4 | Reserved | FEIF4 |
| r |  |  |  |  |  |  |  |  |  |  |  |  |


Bits 31:28, 15:12 Reserved, must be kept at reset value.

**Bits 27, 21, 11, 5 TCIFx:** Stream x transfer complete interrupt flag (x=7..4)
This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_HIFCR register.
0: No transfer complete event on stream x
1: A transfer complete event occurred on stream x

**Bits 26, 20, 10, 4 HTIFx:** Stream x half transfer interrupt flag (x=7..4)
This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_HIFCR register.
0: No half transfer event on stream x
1: A half transfer event occurred on stream x

**Bits 25, 19, 9, 3 TEIFx:** Stream x transfer error interrupt flag (x=7..4)
This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_HIFCR register.
0: No transfer error on stream x
1: A transfer error occurred on stream x

**Bits 24, 18, 8, 2 DMEIFx:** Stream x direct mode error interrupt flag (x=7..4)
This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_HIFCR register.
0: No Direct mode error on stream x
1: A Direct mode error occurred on stream x

**Bits 23, 17, 7, 1** Reserved, must be kept at reset value.

**Bits 22, 16, 6, 0 FEIFx:** Stream x FIFO error interrupt flag (x=7..4)
This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_HIFCR register.
0: No FIFO error event on stream x
1: A FIFO error event occurred on stream x

326/1749
RM0090 Rev 18

--- Page 132 ---

RM0090
DMA controller (DMA)

> [RM0090 ch.10 §10.5] DMA registers

### 10.5.3 DMA low interrupt flag clear register (DMA_LIFCR)

### 10.5.3 DMA low interrupt flag clear register (DMA_LIFCR)

Address offset: 0x08
Reset value: 0x0000 0000

| Reserved | CTCIF3 | CHTIF3 | CTEIF3 | CDMEIF3 | Reserved | CFEIF3 | CTCIF2 | CHTIF2 | CTEIF2 | CDMEIF2 | Reserved | CFEIF2 |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
|  |  |  |  | w |  |  |  |  | w |  |  | w |  |  |  |
| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 |
| Reserved | CTCIF1 | CHTIF1 | CTEIF1 | CDMEIF1 | Reserved | CFEIF1 | CTCIF0 | CHTIF0 | CTEIF0 | CDMEIF0 | Reserved | CFEIF0 |  |  |  |
|  |  |  |  | w |  |  |  |  | w |  |  | w |  |  |  |
| 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |


Bits 31:28, 15:12 Reserved, must be kept at reset value.
Bits 27, 21, 11, 5 **CTCIFx**: Stream x clear transfer complete interrupt flag (x = 3..0)
Writing 1 to this bit clears the corresponding TCIFx flag in the DMA_LISR register
Bits 26, 20, 10, 4 **CHTIFx**: Stream x clear half transfer interrupt flag (x = 3..0)
Writing 1 to this bit clears the corresponding HTIFx flag in the DMA_LISR register
Bits 25, 19, 9, 3 **CTEIFx**: Stream x clear transfer error interrupt flag (x = 3..0)
Writing 1 to this bit clears the corresponding TEIFx flag in the DMA_LISR register
Bits 24, 18, 8, 2 **CDMEIFx**: Stream x clear direct mode error interrupt flag (x = 3..0)
Writing 1 to this bit clears the corresponding DMEIFx flag in the DMA_LISR register
Bits 23, 17, 7, 1 Reserved, must be kept at reset value.
Bits 22, 16, 6, 0 **CFEIFx**: Stream x clear FIFO error interrupt flag (x = 3..0)
Writing 1 to this bit clears the corresponding CFEIFx flag in the DMA_LISR register

> [RM0090 ch.10 §10.5] DMA registers

### 10.5.4 DMA high interrupt flag clear register (DMA_HIFCR)

### 10.5.4 DMA high interrupt flag clear register (DMA_HIFCR)

Address offset: 0x0C
Reset value: 0x0000 0000

| Reserved | CTCIF7 | CHTIF7 | CTEIF7 | CDMEIF7 | Reserved | CFEIF7 | CTCIF6 | CHTIF6 | CTEIF6 | CDMEIF6 | Reserved | CFEIF6 |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
|  |  |  |  | w |  |  |  |  | w |  |  | w |  |  |  |
| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 |
| Reserved | CTCIF5 | CHTIF5 | CTEIF5 | CDMEIF5 | Reserved | CFEIF5 | CTCIF4 | CHTIF4 | CTEIF4 | CDMEIF4 | Reserved | CFEIF4 |  |  |  |
|  |  |  |  | w |  |  |  |  | w |  |  | w |  |  |  |
| 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |


Bits 31:28, 15:12 Reserved, must be kept at reset value.
Bits 27, 21, 11, 5 **CTCIFx**: Stream x clear transfer complete interrupt flag (x = 7..4)
Writing 1 to this bit clears the corresponding TCIFx flag in the DMA_HISR register
Bits 26, 20, 10, 4 **CHTIFx**: Stream x clear half transfer interrupt flag (x = 7..4)
Writing 1 to this bit clears the corresponding HTIFx flag in the DMA_HISR register
Bits 25, 19, 9, 3 **CTEIFx**: Stream x clear transfer error interrupt flag (x = 7..4)
Writing 1 to this bit clears the corresponding TEIFx flag in the DMA_HISR register

ST
RM0090 Rev 18
327/1749

--- Page 133 ---

**DMA controller (DMA)**
RM0090

Bits 24, 18, 8, 2 **CDMEIFx**: Stream x clear direct mode error interrupt flag (x = 7..4)
Writing 1 to this bit clears the corresponding DMEIFx flag in the DMA_HISR register
Bits 23, 17, 7, 1 Reserved, must be kept at reset value.
Bits 22, 16, 6, 0 **CFEIFx**: Stream x clear FIFO error interrupt flag (x = 7..4)
Writing 1 to this bit clears the corresponding CFEIFx flag in the DMA_HISR register


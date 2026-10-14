> [RM0090 ch.10 §10.5] DMA registers


#### 10.5.1 DMA low interrupt status register (DMA_LISR)

Address offset: 0x00
Reset value: 0x0000 0000

| 31 30 29 28 27 26 25 24 | 23 | 22 21 20 19 18 | 17 | 16 |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved | TCIF3 | HTIF3 | TEIF3 | DMEIF3 | Reserved | FEIF3 | TCIF2 | HTIF2 | TEIF2 | DMEIF2 | Reserved | FEIF2 |
| r | Reserved | r | Reserved | r |  |  |  |  |  |  |  |  |
| 15 14 13 12 11 10 9 8 | 7 | 6 5 4 3 2 | 1 | 0 |  |  |  |  |  |  |  |  |
| Reserved | TCIF1 | HTIF1 | TEIF1 | DMEIF1 | Reserved | FEIF1 | TCIF0 | HTIF0 | TEIF0 | DMEIF0 | Reserved | FEIF0 |
| r | Reserved | r | Reserved | r |  |  |  |  |  |  |  |  |


Bits 31:28, 15:12 **Reserved**, must be kept at reset value.

Bits 27, 21, 11, 5 **TCIFx**: Stream x transfer complete interrupt flag (x = 3..0)
This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_LIFCR register.
0: No transfer complete event on stream x
1: A transfer complete event occurred on stream x

Bits 26, 20, 10, 4 **HTIFx**: Stream x half transfer interrupt flag (x=3..0)
This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_LIFCR register.
0: No half transfer event on stream x
1: A half transfer event occurred on stream x

Bits 25, 19, 9, 3 **TEIFx**: Stream x transfer error interrupt flag (x=3..0)
This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_LIFCR register.
0: No transfer error on stream x
1: A transfer error occurred on stream x

Bits 24, 18, 8, 2 **DMEIFx**: Stream x direct mode error interrupt flag (x=3..0)
This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_LIFCR register.
0: No Direct Mode Error on stream x
1: A Direct Mode Error occurred on stream x

Bits 23, 17, 7, 1 **Reserved**, must be kept at reset value.

Bits 22, 16, 6, 0 **FEIFx**: Stream x FIFO error interrupt flag (x=3..0)
This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_LIFCR register.
0: No FIFO Error event on stream x
1: A FIFO Error event occurred on stream x

ST
RM0090 Rev 18
325/1749

--- Page 131 ---

**DMA controller (DMA)**
RM0090


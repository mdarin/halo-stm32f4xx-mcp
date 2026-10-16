> [RM0090 ch.10 §10.3] DMA functional description


### 10.3.10 Programmable data width, packing/unpacking, endianness

The number of data items to be transferred has to be programmed into DMA_SxNDTR (number of data items to transfer bit, NDT) before enabling the stream (except when the flow controller is the peripheral, PFCTRL bit in DMA_SxCR is set).

When using the internal FIFO, the data widths of the source and destination data are programmable through the PSIZE and MSIZE bits in the DMA_SxCR register (can be 8-, 16- or 32-bit).

When PSIZE and MSIZE are not equal:

* The data width of the number of data items to transfer, configured in the DMA_SxNDTR register is equal to the width of the peripheral bus (configured by the PSIZE bits in the DMA_SxCR register). For instance, in case of peripheral-to-memory, memory-to-peripheral or memory-to-memory transfers and if the PSIZE[1:0] bits are configured for half-word, the number of bytes to be transferred is equal to 2 × NDT.
* The DMA controller only copes with little-endian addressing for both source and destination. This is described in Table 46: Packing/unpacking & endian behavior (bit PINC = MINC = 1).

This packing/unpacking procedure may present a risk of data corruption when the operation is interrupted before the data are completely packed/unpacked. So, to ensure data coherence, the stream may be configured to generate burst transfers: in this case, each group of transfers belonging to a burst are indivisible (refer to Section 10.3.11: Single and burst transfers).

In direct mode (DMDIS = 0 in the DMA_SxFCR register), the packing/unpacking of data is not possible. In this case, it is not allowed to have different source and destination transfer data widths: both are equal and defined by the PSIZE bits in the DMA_SxCR MSIZE bits are don't care).

314/1749
RM0090 Rev 18
ST

--- Page 120 ---

RM0090
DMA controller (DMA)

Table 46. Packing/unpacking & endian behavior (bit PINC = MINC = 1)

| AHB memory port width | AHB peripheral port width | Number of data items to transfer (NDT) | Memory transfer number | Memory port address / byte lane | Peripheral transfer number | Peripheral port address / byte lane |
|---|---|---|---|---|---|---|
| PINCOS = 1 | PINCOS = 0 |  |  |  |  |  |
| 8 | 4 | 1 | 0x0 / B0[7:0] | 1 | 0x0 / B0[7:0] |  |
| 2 | 0x1 / B1[7:0] | 2 | 0x4 / B1[7:0] | 0x1 / B1[7:0] |  |  |
| 3 | 0x2 / B2[7:0] | 3 | 0x8 / B2[7:0] | 0x2 / B2[7:0] |  |  |
| 4 | 0x3 / B3[7:0] | 4 | 0xC / B3[7:0] | 0x3 / B3[7:0] |  |  |
| 8 | 16 | 2 | 1 | 0x0 / B0[15:0] | 2 | 0x0 / B1|B0[15:0] |
| 2 | 0x1 / B1[15:0] |  |  |  |  |  |
| 3 | 0x2 / B2[15:0] | 0x4 / B3|B2[15:0] | 0x2 / B3|B2[15:0] |  |  |  |
| 4 | 0x3 / B3[15:0] |  |  |  |  |  |
| 8 | 32 | 1 | 0x0 / B0[7:0] | 1 | 0x0 / B3|B2|B1|B0[31:0] |  |
| 2 | 0x1 / B1[7:0] |  |  |  |  |  |
| 3 | 0x2 / B2[7:0] |  |  |  |  |  |
| 4 | 0x3 / B3[7:0] |  |  |  |  |  |
| 16 | 8 | 4 | 1 | 0x0 / B1|B0[15:0] | 4 | 0x0 / B0[7:0] |
| 2 | 0x2 / B3|B2[15:0] | 2 | 0x4 / B1[7:0] | 0x1 / B1[7:0] |  |  |
|  |  | 3 | 0x8 / B2[7:0] | 0x2 / B2[7:0] |  |  |
|  |  | 4 | 0xC / B3[7:0] | 0x3 / B3[7:0] |  |  |
| 16 | 2 | 1 | 0x0 / B1|B0[15:0] | 2 | 0x0 / B1|B0[15:0] |  |
| 2 | 0x2 / B1|B0[15:0] |  |  |  |  |  |
|  |  | 2 | 0x4 / B3|B2[15:0] | 0x2 / B3|B2[15:0] |  |  |
| 16 | 32 | 1 | 0x0 / B1|B0[15:0] | 1 | 0x0 / B3|B2|B1|B0[31:0] |  |
| 2 | 0x2 / B3|B2[15:0] |  |  |  |  |  |
| 32 | 8 | 4 | 1 | 0x0 / B3|B2|B1|B0[31:0] | 4 | 0x0 / B0[7:0] |
| 2 |  | 2 | 0x4 / B1[7:0] | 0x1 / B1[7:0] |  |  |
| 3 |  | 3 | 0x8 / B2[7:0] | 0x2 / B2[7:0] |  |  |
| 4 |  | 4 | 0xC / B3[7:0] | 0x3 / B3[7:0] |  |  |
| 32 | 16 | 2 | 1 | 0x0 / B3|B2|B1|B0[31:0] | 2 | 0x0 / B1|B0[15:0] |
| 2 |  |  |  |  |  |  |
|  |  | 2 | 0x4 / B3|B2[15:0] | 0x2 / B3|B2[15:0] |  |  |
| 32 | 1 | 0x0 / B3|B2|B1|B0 [31:0] | 1 | 0x0 / B3|B2|B1|B0 [31:0] |  |  |


Note: Peripheral port may be the source or the destination (it could also be the memory source in the case of memory-to-memory transfer).

PSIZE, MSIZE and NDT[15:0] have to be configured so as to ensure that the last transfer will not be incomplete. This can occur when the data width of the peripheral port (PSIZE bits) is lower than the data width of the memory port (MSIZE bits). This constraint is summarized in Table 47.

STi
RM0090 Rev 18
315/1749

--- Page 121 ---

DMA controller (DMA) RM0090

Table 47. Restriction on NDT versus PSIZE and MSIZE

| PSIZE[1:0] of DMA_SxCR | MSIZE[1:0] of DMA_SxCR | NDT[15:0] of DMA_SxNDTR |
|---|---|---|
| 00 (8-bit) | 01 (16-bit) | must be a multiple of 2 |
| 00 (8-bit) | 10 (32-bit) | must be a multiple of 4 |
| 01 (16-bit) | 10 (32-bit) | must be a multiple of 2 |



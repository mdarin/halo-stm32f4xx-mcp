> [RM0090 ch.10 §10.3] DMA functional description


### 10.3.9 Double buffer mode

This mode is available for all the DMA1 and DMA2 streams.

The Double buffer mode is enabled by setting the DBM bit in the DMA_SxCR register.

A double-buffer stream works as a regular (single buffer) stream with the difference that it has two memory pointers. When the Double buffer mode is enabled, the Circular mode is automatically enabled (CIRC bit in DMA_SxCR is don't care) and at each end of transaction, the memory pointers are swapped.

In this mode, the DMA controller swaps from one memory target to another at each end of transaction. This allows the software to process one memory area while the second memory area is being filled/used by the DMA transfer. The double-buffer stream can work in both directions (the memory can be either the source or the destination) as described in Table 45: Source and destination address registers in Double buffer mode (DBM=1).

**Note:** In Double buffer mode, it is possible to update the base address for the AHB memory port on-the-fly (DMA_SxM0AR or DMA_SxM1AR) when the stream is enabled, by respecting the following conditions:

*   When the CT bit is '0' in the DMA_SxCR register, the DMA_SxM1AR register can be written. Attempting to write to this register while CT = '1' sets an error flag (TEIF) and the stream is automatically disabled.
*   When the CT bit is '1' in the DMA_SxCR register, the DMA_SxM0AR register can be written. Attempting to write to this register while CT = '0', sets an error flag (TEIF) and the stream is automatically disabled.

To avoid any error condition, it is advised to change the base address as soon as the TCIF flag is asserted because, at this point, the targeted memory must have changed from

RM0090 Rev 18
313/1749

--- Page 119 ---

DMA controller (DMA)
RM0090

memory 0 to 1 (or from 1 to 0) depending on the value of CT in the DMA_SxCR register in accordance with one of the two above conditions.
For all the other modes (except the Double buffer mode), the memory address registers are write-protected as soon as the stream is enabled.

Table 45. Source and destination address registers in Double buffer mode (DBM=1)

| Bits DIR[1:0] of the DMA_SxCR register | Direction | Source address | Destination address |
|---|---|---|---|
| 00 | Peripheral-to-memory | DMA_SxPAR | DMA_SxM0AR / DMA_SxM1AR |
| 01 | Memory-to-peripheral | DMA_SxM0AR / DMA_SxM1AR | DMA_SxPAR |
| 10 | Not allowed(1) |  |  |
| 11 | Reserved | - |  |


1. When the Double buffer mode is enabled, the Circular mode is automatically enabled. Since the memory-to-memory mode is not compatible with the Circular mode, when the Double buffer mode is enabled, it is not allowed to configure the memory-to-memory mode.


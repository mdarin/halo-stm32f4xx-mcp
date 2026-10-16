> [RM0090 ch.10 §10.3] DMA functional description


### 10.3.18 Error management

The DMA controller can detect the following errors:

*   **Transfer error:** the transfer error interrupt flag (TEIFx) is set when:
    *   A bus error occurs during a DMA read or a write access
    *   A write access is requested by software on a memory address register in Double buffer mode whereas the stream is enabled and the current target memory is the one impacted by the write into the memory address register (refer to Section 10.3.9: *Double buffer mode*)
*   **FIFO error:** the FIFO error interrupt flag (FEIFx) is set if:
    *   A FIFO underrun condition is detected
    *   A FIFO overrun condition is detected (no detection in memory-to-memory mode because requests and transfers are internally managed by the DMA)
    *   The stream is enabled while the FIFO threshold level is not compatible with the size of the memory burst (refer to Table 48: *FIFO threshold configurations*)
*   **Direct mode error:** the direct mode error interrupt flag (DMEIFx) can only be set in the peripheral-to-memory mode while operating in direct mode and when the MINC bit in the DMA_SxCR register is cleared. This flag is set when a DMA request occurs while the previous data have not yet been fully transferred into the memory (because the memory bus was not granted). In this case, the flag indicates that 2 data items were be transferred successively to the same destination address, which could be an issue if the destination is not able to manage this situation

In direct mode, the FIFO error flag can also be set under the following conditions:

*   In the peripheral-to-memory mode, the FIFO can be saturated (overrun) if the memory bus is not granted for several peripheral requests
*   In the memory-to-peripheral mode, an underrun condition may occur if the memory bus has not been granted before a peripheral request occurs

If the TEIFx or the FEIFx flag is set due to incompatibility between burst size and FIFO threshold level, the faulty stream is automatically disabled through a hardware clear of its EN bit in the corresponding stream configuration register (DMA_SxCR).

ST
RM0090 Rev 18
323/1749

--- Page 129 ---

**DMA controller (DMA)**
RM0090

If the DMEIFx or the FEIFx flag is set due to an overrun or underrun condition, the faulty stream is not automatically disabled and it is up to the software to disable or not the stream by resetting the EN bit in the DMA_SxCR register. This is because there is no data loss when this kind of errors occur.

When the stream's error interrupt flag (TEIF, FEIF, DMEIF) in the DMA_LISR or DMA_HISR register is set, an interrupt is generated if the corresponding interrupt enable bit (TEIE, FEIE, DMIE) in the DMA_SxCR or DMA_SxFCR register is set.

**Note:**
*When a FIFO overrun or underrun condition occurs, the data are not lost because the peripheral request is not acknowledged by the stream until the overrun or underrun condition is cleared. If this acknowledge takes too much time, the peripheral itself may detect an overrun or underrun condition of its internal buffer and data might be lost.*

> [RM0090 ch.10 §10.3] DMA functional description

### 10.4 DMA interrupts

## 10.4 DMA interrupts

For each DMA stream, an interrupt can be produced on the following events:

*   Half-transfer reached
*   Transfer complete
*   Transfer error
*   Fifo error (overrun, underrun or FIFO level error)
*   Direct mode error

Separate interrupt enable control bits are available for flexibility as shown in Table 50.

**Table 50. DMA interrupt requests**

| Interrupt event | Event flag | Enable control bit |
|---|---|---|
| Half-transfer | HTIF | HTIE |
| Transfer complete | TCIF | TCIE |
| Transfer error | TEIF | TEIE |
| FIFO overrun/underrun | FEIF | FEIE |
| Direct mode error | DMEIF | DMEIE |


**Note:**
*Before setting an Enable control bit to '1', the corresponding event flag should be cleared, otherwise an interrupt is immediately generated.*

324/1749
RM0090 Rev 18

--- Page 130 ---

RM0090
DMA controller (DMA)


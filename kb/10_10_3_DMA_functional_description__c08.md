> [RM0090 ch.10 §10.3] DMA functional description


#### FIFO structure

The FIFO is used to temporarily store data coming from the source before transmitting them to the destination.

Each stream has an independent 4-word FIFO and the threshold level is software-configurable between 1/4, 1/2, 3/4 or full.

To enable the use of the FIFO threshold level, the direct mode must be disabled by setting the DMDIS bit in the DMA_SxFCR register.

The structure of the FIFO differs depending on the source and destination data widths, and is described in Figure 39: FIFO structure.

**Figure 39. FIFO structure**

| 4 words |  |  |  |  |
|---|---|---|---|---|
| Empty 1/4 1/2 3/4 Full |  |  |  |  |
| Source: byte |  |  |  |  |
| B15 B14 B13 B12 B11 B10 B9 B8 B7 B6 B5 B4 B3 B2 B1 B0 |  |  |  |  |
| byte lane 3 | B15 | B 11 | B7 | B3 |
| byte lane 2 | B14 | B10 | B6 | B2 |
| byte lane 1 | B13 | B9 | B5 | B1 |
| byte lane 0 | W3 B12 | W2 B8 | W1 B4 | W0 B0 |
| Destination: word |  |  |  |  |
| W3, W2, W1, W0 |  |  |  |  |
| 4 words |  |  |  |  |
| Empty 1/4 1/2 3/4 Full |  |  |  |  |
| Source: byte |  |  |  |  |
| B15 B14 B13 B12 B11 B10 B9 B8 B7 B6 B5 B4 B3 B2 B1 B0 |  |  |  |  |
| byte lane 3 | B15 | B 11 | B7 | B3 |
| byte lane 2 | H7 B14 | H5 B10 | H3 B6 | H1 B2 |
| byte lane 1 | B13 | B9 | B5 | B1 |
| byte lane 0 | H6 B12 | H4 B8 | H2 B4 | H0 B0 |
| Destination: half-word |  |  |  |  |
| H7, H6, H5, H4, H3, H2, H1, H0 |  |  |  |  |
| 4 words |  |  |  |  |
| Empty 1/4 1/2 3/4 Full |  |  |  |  |
| Source: half-word |  |  |  |  |
| H7 H6 H5 H4 H3 H2 H1 H0 |  |  |  |  |
| byte lane 3 | H7 | H5 | H3 | H1 |
| byte lane 2 | H6 | H4 | H2 | H0 |
| byte lane 1 | W3 | W2 | W1 | W0 |
| byte lane 0 |  |  |  |  |
| Destination: word |  |  |  |  |
| W3, W2, W1, W0 |  |  |  |  |
| 4-words |  |  |  |  |
| Empty 1/4 1/2 3/4 Full |  |  |  |  |
| Source: half-word |  |  |  |  |
| H7 H6 H5 H4 H3 H2 H1 H0 |  |  |  |  |
| byte lane 3 | B15 | B 11 | B7 | B3 |
| byte lane 2 | H7 B14 | H5 B10 | H3 B6 | H1 B2 |
| byte lane 1 | B13 | B9 | B5 | B1 |
| byte lane 0 | H6 B12 | H4 B8 | H2 B4 | H0 B0 |
| Destination: byte |  |  |  |  |
| B15 B14 B13 B12 B11 B10 B9 B8 B7 B6 B5 B4 B3 B2 B1 B0 |  |  |  |  |


ai15951

ST
RM0090 Rev 18
317/1749

--- Page 123 ---

DMA controller (DMA)
RM0090

**FIFO threshold and burst configuration**

Caution is required when choosing the FIFO threshold (bits FTH[1:0] of the DMA_SxFCR register) and the size of the memory burst (MBURST[1:0] of the DMA_SxCR register): The content pointed by the FIFO threshold must exactly match to an integer number of memory burst transfers. If this is not in the case, a FIFO error (flag FEIFx of the DMA_HISR or DMA_LISR register) will be generated when the stream is enabled, then the stream will be automatically disabled. The allowed and forbidden configurations are described in the Table 48: FIFO threshold configurations.

Table 48. FIFO threshold configurations

| MSIZE | FIFO level | MBURST = INCR4 | MBURST = INCR8 | MBURST = INCR16 |
|---|---|---|---|---|
| Byte | 1/4 | 1 burst of 4 beats | forbidden |  |
| 1/2 | 2 bursts of 4 beats | 1 burst of 8 beats |  |  |
| 3/4 | 3 bursts of 4 beats |  |  |  |
| Full | 4 bursts of 4 beats | 2 bursts of 8 beats | 1 burst of 16 beats |  |
| Half-word | 1/4 | forbidden |  |  |
| 1/2 | 1 burst of 4 beats |  |  |  |
| 3/4 | forbidden |  |  |  |
| Full | 2 bursts of 4 beats | 1 burst of 8 beats |  |  |
| Word | 1/4 | forbidden |  |  |
| 1/2 |  |  |  |  |
| 3/4 |  |  |  |  |
| Full | 1 burst of 4 beats |  |  |  |


In all cases, the burst size multiplied by the data size must not exceed the FIFO size (data size can be: 1 (byte), 2 (half-word) or 4 (word)).

Incomplete Burst transfer at the end of a DMA transfer may happen if one of the following conditions occurs:

*   For the AHB peripheral port configuration: the total number of data items (set in the DMA_SxNDTR register) is not a multiple of the burst size multiplied by the data size
*   For the AHB memory port configuration: the number of remaining data items in the FIFO to be transferred to the memory is not a multiple of the burst size multiplied by the data size

In such cases, the remaining data to be transferred will be managed in single mode by the DMA, even if a burst transaction was requested during the DMA stream configuration.

**Note:** When burst transfers are requested on the peripheral AHB port and the FIFO is used (DMDIS = 1 in the DMA_SxCR register), it is mandatory to respect the following rule to avoid permanent underrun or overrun conditions, depending on the DMA stream direction: If (PBURST × PSIZE) = FIFO_SIZE (4 words), FIFO_Threshold = 3/4 is forbidden with PSIZE = 1, 2 or 4 and PBURST = 4, 8 or 16. This rule ensures that enough FIFO space at a time will be free to serve the request from the peripheral.

318/1749
RM0090 Rev 18
ST

--- Page 124 ---

RM0090
DMA controller (DMA)

**FIFO flush**

The FIFO can be flushed when the stream is disabled by resetting the EN bit in the DMA_SxCR register and when the stream is configured to manage peripheral-to-memory or memory-to-memory transfers: If some data are still present in the FIFO when the stream is disabled, the DMA controller continues transferring the remaining data to the destination (even though stream is effectively disabled). When this flush is completed, the transfer complete status bit (TCIFx) in the DMA_LISR or DMA_HISR register is set.

The remaining data counter DMA_SxNDTR keeps the value in this case to indicate how many data items are currently available in the destination memory.

Note that during the FIFO flush operation, if the number of remaining data items in the FIFO to be transferred to memory (in bytes) is less than the memory data width (for example 2 bytes in FIFO while MSIZE is configured to word), data will be sent with the data width set in the MSIZE bit in the DMA_SxCR register. This means that memory will be written with an undesired value. The software may read the DMA_SxNDTR register to determine the memory area that contains the good data (start address and last address).

If the number of remaining data items in the FIFO is lower than a burst size (if the MBURST bits in DMA_SxCR register are set to configure the stream to manage burst on the AHB memory port), single transactions will be generated to complete the FIFO flush.

**Direct mode**

By default, the FIFO operates in direct mode (DMDIS bit in the DMA_SxFCR is reset) and the FIFO threshold level is not used. This mode is useful when the system requires an immediate and single transfer to or from the memory after each DMA request.

When the DMA is configured in direct mode (FIFO disabled), to transfer data in memory-to-peripheral mode, the DMA preloads one data from the memory to the internal FIFO to ensure an immediate data transfer as soon as a DMA request is triggered by a peripheral.

To avoid saturating the FIFO, it is recommended to configure the corresponding stream with a high priority.

This mode is restricted to transfers where:

*   The source and destination transfer widths are equal and both defined by the PSIZE[1:0] bits in DMA_SxCR (MSIZE[1:0] bits are don't care)
*   Burst transfers are not possible (PBURST[1:0] and MBURST[1:0] bits in DMA_SxCR are don't care)

Direct mode must not be used when implementing memory-to-memory transfers.

> [RM0090 ch.10 §10.3] DMA functional description

### 10.3.13 DMA transfer completion**

**10.3.13 DMA transfer completion**

Different events can generate an end of transfer by setting the TCIFx bit in the DMA_LISR or DMA_HISR status register:

*   In DMA flow controller mode:
    *   The DMA_SxNDTR counter has reached zero in the memory-to-peripheral mode
    *   The stream is disabled before the end of transfer (by clearing the EN bit in the DMA_SxCR register) and (when transfers are peripheral-to-memory or memory-

ST
RM0090 Rev 18
319/1749

--- Page 125 ---

**DMA controller (DMA)** RM0090

to-memory) all the remaining data have been flushed from the FIFO into the memory

*   In Peripheral flow controller mode:
    *   The last external burst or single request has been generated from the peripheral and (when the DMA is operating in peripheral-to-memory mode) the remaining data have been transferred from the FIFO into the memory
    *   The stream is disabled by software, and (when the DMA is operating in peripheral-to-memory mode) the remaining data have been transferred from the FIFO into the memory

**Note:** *The transfer completion is dependent on the remaining data in FIFO to be transferred into memory only in the case of peripheral-to-memory mode. This condition is not applicable in memory-to-peripheral mode.*

If the stream is configured in noncircular mode, after the end of the transfer (that is when the number of data to be transferred reaches zero), the DMA is stopped (EN bit in DMA_SxCR register is cleared by Hardware) and no DMA request is served unless the software reprograms the stream and re-enables it (by setting the EN bit in the DMA_SxCR register).


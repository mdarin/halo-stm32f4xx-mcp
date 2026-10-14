> [RM0090 ch.10 §10.3] DMA functional description


#### Peripheral-to-memory mode

*Figure 36* describes this mode.

When this mode is enabled (by setting the bit EN in the DMA_SxCR register), each time a peripheral request occurs, the stream initiates a transfer from the source to fill the FIFO.

When the threshold level of the FIFO is reached, the contents of the FIFO are drained and stored into the destination.

The transfer stops once the DMA_SxNDTR register reaches zero, when the peripheral requests the end of transfers (in case of a peripheral flow controller) or when the EN bit in the DMA_SxCR register is cleared by software.

ST
RM0090 Rev 18
309/1749

--- Page 115 ---

**DMA controller (DMA)**
RM0090

In direct mode (when the DMDIS value in the DMA_SxFCR register is '0'), the threshold level of the FIFO is not used: after each single data transfer from the peripheral to the FIFO, the corresponding data are immediately drained and stored into the destination.

The stream has access to the AHB source or destination port only if the arbitration of the corresponding stream is won. This arbitration is performed using the priority defined for each stream using the PL[1:0] bits in the DMA_SxCR register.

**Figure 36. Peripheral-to-memory mode**

DMA controller
DMA_SxM0AR
DMA_SxM1AR(1)
AHB memory port
Memory bus
Memory destination
REQ_STREAMx
Arbiter
FIFO level
FIFO
AHB peripheral port
Peripheral bus
Peripheral source
DMA_SxPAR
Peripheral DMA request
MS47543V1

1. For double-buffer mode.

**Memory-to-peripheral mode**

*Figure 37* describes this mode.

When this mode is enabled (by setting the EN bit in the DMA_SxCR register), the stream immediately initiates transfers from the source to entirely fill the FIFO.

Each time a peripheral request occurs, the contents of the FIFO are drained and stored into the destination. When the level of the FIFO is lower than or equal to the predefined threshold level, the FIFO is fully reloaded with data from the memory.

The transfer stops once the DMA_SxNDTR register reaches zero, when the peripheral requests the end of transfers (in case of a peripheral flow controller) or when the EN bit in the DMA_SxCR register is cleared by software.

In direct mode (when the DMDIS value in the DMA_SxFCR register is '0'), the threshold level of the FIFO is not used. Once the stream is enabled, the DMA preloads the first data to transfer into an internal FIFO. As soon as the peripheral requests a data transfer, the DMA transfers the preloaded value into the configured destination. It then reloads again the

310/1749
RM0090 Rev 18
ST

--- Page 116 ---

RM0090
DMA controller (DMA)

empty internal FIFO with the next data to be transfer. The preloaded data size corresponds to the value of the PSIZE bitfield in the DMA_SxCR register.

The stream has access to the AHB source or destination port only if the arbitration of the corresponding stream is won. This arbitration is performed using the priority defined for each stream using the PL[1:0] bits in the DMA_SxCR register.

**Figure 37. Memory-to-peripheral mode**

DMA controller
DMA_SxM0AR
DMA_SxM1AR(1)
AHB memory port
Memory bus
Memory source
REQ_STREAMx
Arbiter
FIFO level
FIFO
AHB peripheral port
Peripheral bus
Peripheral destination
DMA_SxPAR
Peripheral DMA request

ai15949

1. For double-buffer mode.

**Memory-to-memory mode**

The DMA channels can also work without being triggered by a request from a peripheral. This is the memory-to-memory mode, described in Figure 38.

When the stream is enabled by setting the Enable bit (EN) in the DMA_SxCR register, the stream immediately starts to fill the FIFO up to the threshold level. When the threshold level is reached, the FIFO contents are drained and stored into the destination.

The transfer stops once the DMA_SxNDTR register reaches zero or when the EN bit in the DMA_SxCR register is cleared by software.

The stream has access to the AHB source or destination port only if the arbitration of the corresponding stream is won. This arbitration is performed using the priority defined for each stream using the PL[1:0] bits in the DMA_SxCR register.

Note:
When memory-to-memory mode is used, the Circular and direct modes are not allowed. Only the DMA2 controller is able to perform memory-to-memory transfers.

STi
RM0090 Rev 18
311/1749

--- Page 117 ---

DMA controller (DMA)
RM0090

Figure 38. Memory-to-memory mode

DMA controller
DMA_SxM0AR
DMA_SxM1AR(1)
Stream enable
Arbiter
FIFO level
FIFO
AHB memory port
Memory bus
Memory 2 destination
AHB peripheral port
Peripheral bus
Memory 1 source
DMA_SxPAR

ai15950

1. For double-buffer mode.

> [RM0090 ch.10 §10.3] DMA functional description

### 10.3.7 Pointer incrementation**

**10.3.7 Pointer incrementation**

Peripheral and memory pointers can optionally be automatically post-incremented or kept constant after each transfer depending on the PINC and MINC bits in the DMA_SxCR register.

Disabling the Increment mode is useful when the peripheral source or destination data are accessed through a single register.

If the Increment mode is enabled, the address of the next transfer will be the address of the previous one incremented by 1 (for bytes), 2 (for half-words) or 4 (for words) depending on the data width programmed in the PSIZE or MSIZE bits in the DMA_SxCR register.

In order to optimize the packing operation, it is possible to fix the increment offset size for the peripheral address whatever the size of the data transferred on the AHB peripheral port. The PINCOS bit in the DMA_SxCR register is used to align the increment offset size with the data size on the peripheral AHB port, or on a 32-bit address (the address is then incremented by 4). The PINCOS bit has an impact on the AHB peripheral port only.

If PINCOS bit is set, the address of the next transfer is the address of the previous one incremented by 4 (automatically aligned on a 32-bit address) whatever the PSIZE value. The AHB memory port, however, is not impacted by this operation.

312/1749
RM0090 Rev 18
ST

--- Page 118 ---

RM0090
DMA controller (DMA)

> [RM0090 ch.10 §10.3] DMA functional description

### 10.3.8 Circular mode

### 10.3.8 Circular mode

The Circular mode is available to handle circular buffers and continuous data flows (e.g. ADC scan mode). This feature can be enabled using the CIRC bit in the DMA_SxCR register.

When the circular mode is activated, the number of data items to be transferred is automatically reloaded with the initial value programmed during the stream configuration phase, and the DMA requests continue to be served.

**Note:** In the circular mode, it is mandatory to respect the following rule in case of a burst mode configured for memory:

DMA_SxNDTR = Multiple of ((Mburst beat) × (Msize)/(Psize)), where:

*   (Mburst beat) = 4, 8 or 16 (depending on the MBURST bits in the DMA_SxCR register)
*   ((Msize)/(Psize)) = 1, 2, 4, 1/2 or 1/4 (Msize and Psize represent the MSIZE and PSIZE bits in the DMA_SxCR register. They are byte dependent)
*   DMA_SxNDTR = Number of data items to transfer on the AHB peripheral port

For example: Mburst beat = 8 (INCR8), MSIZE = '00' (byte) and PSIZE = '01' (half-word), in this case: DMA_SxNDTR must be a multiple of (8 × 1/2 = 4).

If this formula is not respected, the DMA behavior and data integrity are not guaranteed. NDTR must also be a multiple of the Peripheral burst size multiplied by the peripheral data size, otherwise this could result in a bad DMA behavior.


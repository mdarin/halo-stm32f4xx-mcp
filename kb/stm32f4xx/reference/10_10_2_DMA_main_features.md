# 10 DMA controller (DMA)

This section applies to the whole STM32F4xx family, unless otherwise specified.

## 10.1 DMA introduction

Direct memory access (DMA) is used in order to provide high-speed data transfer between peripherals and memory and between memory and memory. Data can be quickly moved by DMA without any CPU action. This keeps CPU resources free for other operations.

The DMA controller combines a powerful dual AHB master bus architecture with independent FIFO to optimize the bandwidth of the system, based on a complex bus matrix architecture.

The two DMA controllers have 16 streams in total (8 for each controller), each dedicated to managing memory access requests from one or more peripherals. Each stream can have up to 8 channels (requests) in total. And each has an arbiter for handling the priority between DMA requests.

## 10.2 DMA main features

The main DMA features are:

*   Dual AHB master bus architecture, one dedicated to memory accesses and one dedicated to peripheral accesses
*   AHB slave programming interface supporting only 32-bit accesses
*   8 streams for each DMA controller, up to 8 channels (requests) per stream
*   Four-word depth 32 first-in, first-out memory buffers (FIFOs) per stream, that can be used in FIFO mode or direct mode:
    *   FIFO mode: with threshold level software selectable between 1/4, 1/2 or 3/4 of the FIFO size
    *   Direct mode

Each DMA request immediately initiates a transfer from/to the memory. When it is configured in direct mode (FIFO disabled), to transfer data in memory-to-peripheral mode, the DMA preloads only one data from the memory to the internal

302/1749 RM0090 Rev 18 ST

--- Page 108 ---

RM0090
DMA controller (DMA)

FIFO to ensure an immediate data transfer as soon as a DMA request is triggered by a peripheral.

*   Each stream can be configured by hardware to be:
    *   a regular channel that supports peripheral-to-memory, memory-to-peripheral and memory-to-memory transfers
    *   a double buffer channel that also supports double buffering on the memory side
*   Each of the 8 streams are connected to dedicated hardware DMA channels (requests)
*   Priorities between DMA stream requests are software-programmable (4 levels consisting of very high, high, medium, low) or hardware in case of equality (request 0 has priority over request 1, etc.)
*   Each stream also supports software trigger for memory-to-memory transfers (only available for the DMA2 controller)
*   Each stream request can be selected among up to 8 possible channel requests. This selection is software-configurable and allows several peripherals to initiate DMA requests
*   The number of data items to be transferred can be managed either by the DMA controller or by the peripheral:
    *   DMA flow controller: the number of data items to be transferred is software-programmable from 1 to 65535
    *   Peripheral flow controller: the number of data items to be transferred is unknown and controlled by the source or the destination peripheral that signals the end of the transfer by hardware
*   Independent source and destination transfer width (byte, half-word, word): when the data widths of the source and destination are not equal, the DMA automatically packs/unpacks the necessary transfers to optimize the bandwidth. This feature is only available in FIFO mode
*   Incrementing or non-incrementing addressing for source and destination
*   Supports incremental burst transfers of 4, 8 or 16 beats. The size of the burst is software-configurable, usually equal to half the FIFO size of the peripheral
*   Each stream supports circular buffer management
*   5 event flags (DMA Half Transfer, DMA Transfer complete, DMA Transfer Error, DMA FIFO Error, Direct Mode Error) logically ORed together in a single interrupt request for each stream

ST
RM0090 Rev 18
303/1749

--- Page 109 ---

DMA controller (DMA)
RM0090


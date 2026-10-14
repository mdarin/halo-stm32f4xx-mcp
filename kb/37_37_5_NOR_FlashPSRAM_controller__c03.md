> [RM0090 ch.37 §37.5] NOR Flash/PSRAM controller

### 37.5.2 Supported memories and transactions**

**37.5.2 Supported memories and transactions**

*Table 268* below shows an example of the supported devices, access modes and transactions when the memory data bus is 16-bit wide for NOR Flash memory, PSRAM and SRAM. The transactions not allowed (or not supported) by the FMC are shown in gray in this example.

STI
RM0090 Rev 18
1615/1749

--- Page 56 ---

Flexible memory controller (FMC)
RM0090

Table 268. NOR Flash/PSRAM: Example of supported memories and transactions

| Device | Mode | R/W | AHB data size | Memory data size | Allowed/not allowed | Comments |
|---|---|---|---|---|---|---|
| NOR Flash (muxed I/Os and nonmuxed I/Os) | Asynchronous | R | 8 | 16 | Y |  |
| Asynchronous | W | 8 | 16 | N |  |  |
| Asynchronous | R | 16 | Y |  |  |  |
| Asynchronous | W | 16 | Y |  |  |  |
| Asynchronous | R | 32 | 16 | Y | Split into 2 FMC accesses |  |
| Asynchronous | W | 32 | 16 | Y | Split into 2 FMC accesses |  |
| Asynchronous page | R | - | 16 | N | Mode is not supported |  |
| Synchronous | R | 8 | 16 | N |  |  |
| Synchronous | R | 16 | Y |  |  |  |
| Synchronous | R | 32 | 16 | Y |  |  |
| Synchronous | R | 32 | 16 | Y |  |  |
| PSRAM (multiplexed I/Os and non-multiplexed I/Os) | Asynchronous | R | 8 | 16 | Y |  |
| Asynchronous | W | 8 | 16 | Y | Use of byte lanes NBL[1:0] |  |
| Asynchronous | R | 16 | Y |  |  |  |
| Asynchronous | W | 16 | Y |  |  |  |
| Asynchronous | R | 32 | 16 | Y | Split into 2 FMC accesses |  |
| Asynchronous | W | 32 | 16 | Y | Split into 2 FMC accesses |  |
| Asynchronous page | R | - | 16 | N | Mode is not supported |  |
| Synchronous | R | 8 | 16 | N |  |  |
| Synchronous | R | 16 | Y |  |  |  |
| Synchronous | R | 32 | 16 | Y |  |  |
| Synchronous | W | 8 | 16 | Y | Use of byte lanes NBL[1:0] |  |
| Synchronous | W | 16/32 | 16 | Y |  |  |
| SRAM and ROM | Asynchronous | R | 8 / 16 | 16 | Y |  |
| Asynchronous | W | 8 / 16 | 16 | Y | Use of byte lanes NBL[1:0] |  |
| Asynchronous | R | 32 | 16 | Y | Split into 2 FMC accesses |  |
| Asynchronous | W | 32 | 16 | Y | Split into 2 FMC accesses Use of byte lanes NBL[1:0] |  |


1616/1749
RM0090 Rev 18

--- Page 57 ---

RM0090
Flexible memory controller (FMC)

> [RM0090 ch.37 §37.5] NOR Flash/PSRAM controller

### 37.5.3 General timing rules

### 37.5.3 General timing rules

**Signals synchronization**

*   All controller output signals change on the rising edge of the internal clock (HCLK)
*   In synchronous mode (read or write), all output signals change on the rising edge of HCLK. Whatever the CLKDIV value, all outputs change as follows:
    *   NOEL/NWEL/ NEL/NADVL/ NADVH /NBLL/ Address valid outputs change on the falling edge of FMC_CLK clock.
    *   NOEH/ NWEH / NEH/ NOEH/NBLH/ Address invalid outputs change on the rising edge of FMC_CLK clock.


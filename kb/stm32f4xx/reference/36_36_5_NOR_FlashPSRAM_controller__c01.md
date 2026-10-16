> [RM0090 ch.36 §36.5] NOR Flash/PSRAM controller


### 36.5 NOR Flash/PSRAM controller

The FSMC generates the appropriate signal timings to drive the following types of memories:

*   Asynchronous SRAM and ROM
    *   8-bit
    *   16-bit
    *   32-bit
*   PSRAM (Cellular RAM)
    *   Asynchronous mode
    *   Burst mode for synchronous accesses
    *   Multiplexed or nonmultiplexed
*   NOR Flash
    *   Asynchronous mode
    *   Burst mode for synchronous accesses
    *   Multiplexed or nonmultiplexed

The FSMC outputs a unique chip select signal NE[4:1] per bank. All the other signals (addresses, data and control) are shared.

ST
RM0090 Rev 18
1549/1749

--- Page 185 ---

**Flexible static memory controller (FSMC)**
RM0090

For synchronous accesses, the FSMC issues the clock (CLK) to the selected external device only during the read/write transactions. This clock is a submultiple of the HCLK clock. The size of each bank is fixed and equal to 64 Mbytes.

Each bank is configured by means of dedicated registers (see Section 36.5.6).

The programmable memory parameters include access timings (see Table 220) and support for wait management (for PSRAM and NOR Flash accessed in burst mode).

**Table 220. Programmable NOR/PSRAM access parameters**

| Parameter | Function | Access mode | Unit | Min. | Max. |
|---|---|---|---|---|---|
| Address setup | Duration of the address setup phase | Asynchronous | AHB clock cycle (HCLK) | 0 | 15 |
| Address hold | Duration of the address hold phase | Asynchronous, muxed I/Os | AHB clock cycle (HCLK) | 1 | 15 |
| Data setup | Duration of the data setup phase | Asynchronous | AHB clock cycle (HCLK) | 1 | 256 |
| Bus turn | Duration of the bus turnaround phase | Asynchronous and synchronous read/write | AHB clock cycle (HCLK) | 0 | 15 |
| Clock divide ratio | Number of AHB clock cycles (HCLK) to build one memory clock cycle (CLK) | Synchronous | AHB clock cycle (HCLK) | 2 | 16 |
| Data latency | Number of clock cycles to issue to the memory before the first data of the burst | Synchronous | Memory clock cycle (CLK) | 2 | 17 |



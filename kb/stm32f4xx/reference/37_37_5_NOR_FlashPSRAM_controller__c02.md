> [RM0090 ch.37 §37.5] NOR Flash/PSRAM controller


### 37.5.1 External memory interface signals

Table 264, Table 265 and Table 266 list the signals that are typically used to interface with NOR Flash memory, SRAM and PSRAM.

*Note:* The prefix "N" identifies the signals which are active low.

STi
RM0090 Rev 18
1613/1749

--- Page 54 ---

Flexible memory controller (FMC)
RM0090

**NOR Flash memory, non-multiplexed I/Os**

Table 264. Non-multiplexed I/O NOR Flash memory

| FMC signal name | I/O | Function |
|---|---|---|
| CLK | O | Clock (for synchronous access) |
| A[25:0] | O | Address bus |
| D[31:0] | I/O | Bidirectional data bus |
| NE[x] | O | Chip Select, x = 1..4 |
| NOE | O | Output enable |
| NWE | O | Write enable |
| NL(=NADV) | O | Latch enable (this signal is called address valid, NADV, by some NOR Flash devices) |
| NWAIT | I | NOR Flash wait input signal to the FMC |


The maximum capacity is 512 Mbits (26 address lines).

**NOR Flash memory, 16-bit multiplexed I/Os**

Table 265. 16-bit multiplexed I/O NOR Flash memory

| FMC signal name | I/O | Function |
|---|---|---|
| CLK | O | Clock (for synchronous access) |
| A[25:16] | O | Address bus |
| AD[15:0] | I/O | 16-bit multiplexed, bidirectional address/data bus (the 16-bit address A[15:0] and data D[15:0] are multiplexed on the databus) |
| NE[x] | O | Chip Select, x = 1..4 |
| NOE | O | Output enable |
| NWE | O | Write enable |
| NL(=NADV) | O | Latch enable (this signal is called address valid, NADV, by some NOR Flash devices) |
| NWAIT | I | NOR Flash wait input signal to the FMC |


The maximum capacity is 512 Mbits.

**PSRAM/SRAM, non-multiplexed I/Os**

Table 266. Non-multiplexed I/Os PSRAM/SRAM

| FMC signal name | I/O | Function |
|---|---|---|
| CLK | O | Clock (only for PSRAM synchronous access) |
| A[25:0] | O | Address bus |
| D[31:0] | I/O | Data bidirectional bus |


1614/1749
RM0090 Rev 18

--- Page 55 ---

RM0090
Flexible memory controller (FMC)

Table 266. Non-multiplexed I/Os PSRAM/SRAM (continued)

| FMC signal name | I/O | Function |
|---|---|---|
| NE[x] | O | Chip Select, x = 1..4 (called NCE by PSRAM (Cellular RAM i.e. CRAM)) |
| NOE | O | Output enable |
| NWE | O | Write enable |
| NL(= NADV) | O | Address valid only for PSRAM input (memory signal name: NADV) |
| NWAIT | I | PSRAM wait input signal to the FMC |
| NBL[3] | O | Byte3 Upper byte enable (memory signal name: NUB) |
| NBL[2] | O | Byte2 Lowed byte enable (memory signal name: NLB) |
| NBL[1] | O | Byte1 Upper byte enable (memory signal name: NLB) |
| NBL[0] | O | Byte0 Lower byte enable (memory signal name: NLB) |


The maximum capacity is 512 Mbits.

**PSRAM, 16-bit multiplexed I/Os**

Table 267. 16-Bit multiplexed I/O PSRAM

| FMC signal name | I/O | Function |
|---|---|---|
| CLK | O | Clock (for synchronous access) |
| A[25:16] | O | Address bus |
| AD[15:0] | I/O | 16-bit multiplexed, bidirectional address/data bus (the 16-bit address A[15:0] and data D[15:0] are multiplexed on the databus) |
| NE[x] | O | Chip Select, x = 1..4 (called NCE by PSRAM (Cellular RAM i.e. CRAM)) |
| NOE | O | Output enable |
| NWE | O | Write enable |
| NL(= NADV) | O | Address valid PSRAM input (memory signal name: NADV) |
| NWAIT | I | PSRAM wait input signal to the FMC |
| NBL[1] | O | Upper byte enable (memory signal name: NUB) |
| NBL[0] | O | Lowed byte enable (memory signal name: NLB) |


The maximum capacity is 512 Mbits (26 address lines).


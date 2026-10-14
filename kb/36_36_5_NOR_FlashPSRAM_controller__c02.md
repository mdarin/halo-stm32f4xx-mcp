> [RM0090 ch.36 §36.5] NOR Flash/PSRAM controller

### 36.5.1 External memory interface signals**

**36.5.1 External memory interface signals**

Table 221, Table 222 and Table 223 list the signals that are typically used to interface NOR Flash, SRAM and PSRAM.

*Note:* Prefix "N". specifies the associated signal as active low.

**NOR Flash, nonmultiplexed I/Os**

**Table 221. Nonmultiplexed I/O NOR Flash**

| FSMC signal name | I/O | Function |
|---|---|---|
| CLK | O | Clock (for synchronous access) |
| A[25:0] | O | Address bus |
| D[15:0] | I/O | Bidirectional data bus |
| NE[x] | O | Chip select, x = 1..4 |
| NOE | O | Output enable |
| NWE | O | Write enable |
| NL(=NADV) | O | Latch enable (this signal is called address valid, NADV, by some NOR Flash devices) |
| NWAIT | I | NOR Flash wait input signal to the FSMC |


1550/1749
RM0090 Rev 18

--- Page 186 ---

RM0090
Flexible static memory controller (FSMC)

NOR Flash memories are addressed in 16-bit words. The maximum capacity is 512 Mbit (26 address lines).

**NOR Flash, multiplexed I/Os**

Table 222. Multiplexed I/O NOR Flash

| FSMC signal name | I/O | Function |
|---|---|---|
| CLK | O | Clock (for synchronous access) |
| A[25:16] | O | Address bus |
| AD[15:0] | I/O | 16-bit multiplexed, bidirectional address/data bus |
| NE[x] | O | Chip select, x = 1..4 |
| NOE | O | Output enable |
| NWE | O | Write enable |
| NL(=NADV) | O | Latch enable (this signal is called address valid, NADV, by some NOR Flash devices) |
| NWAIT | I | NOR Flash wait input signal to the FSMC |


NOR-Flash memories are addressed in 16-bit words. The maximum capacity is 512 Mbit (26 address lines).

**PSRAM/SRAM, nonmultiplexed I/Os**

Table 223. Nonmultiplexed I/Os PSRAM/SRAM

| FSMC signal name | I/O | Function |
|---|---|---|
| CLK | O | Clock (only for PSRAM synchronous access) |
| A[25:0] | O | Address bus |
| D[15:0] | I/O | Data bidirectional bus |
| NE[x] | O | Chip select, x = 1..4 (called NCE by PSRAM (Cellular RAM i.e. CRAM)) |
| NOE | O | Output enable |
| NWE | O | Write enable |
| NL(= NADV) | O | Address valid only for PSRAM input (memory signal name: NADV) |
| NWAIT | I | PSRAM wait input signal to the FSMC |
| NBL[1] | O | Upper byte enable (memory signal name: NUB) |
| NBL[0] | O | Lowed byte enable (memory signal name: NLB) |


PSRAM memories are addressed in 16-bit words. The maximum capacity is 512 Mbit (26 address lines).

PSRAM, multiplexed I/Os

ST
RM0090 Rev 18
1551/1749

--- Page 187 ---

**Flexible static memory controller (FSMC)**
RM0090

Table 224. Multiplexed I/O PSRAM

| FSMC signal name | I/O | Function |
|---|---|---|
| CLK | O | Clock (for synchronous access) |
| A[25:16] | O | Address bus |
| AD[15:0] | I/O | 16-bit multiplexed, bidirectional address/data bus |
| NE[x] | O | Chip select, x = 1..4 (called NCE by PSRAM (Cellular RAM i.e. CRAM)) |
| NOE | O | Output enable |
| NWE | O | Write enable |
| NL(= NADV) | O | Address valid PSRAM input (memory signal name: NADV) |
| NWAIT | I | PSRAM wait input signal to the FSMC |
| NBL[1] | O | Upper byte enable (memory signal name: NUB) |
| NBL[0] | O | Lowed byte enable (memory signal name: NLB) |


PSRAM memories are addressed in 16-bit words. The maximum capacity is 512 Mbit (26 address lines).


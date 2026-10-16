> [RM0090 ch.36 §36.5] NOR Flash/PSRAM controller


#### Wait management

For synchronous NOR Flash memories, NWAIT is evaluated after the programmed latency period, (DATLAT+2) CLK clock cycles.

ST
RM0090 Rev 18
1571/1749

--- Page 12 ---

**Flexible static memory controller (FSMC)**
RM0090

If NWAIT is sensed active (low level when WAITPOL = 0, high level when WAITPOL = 1), wait states are inserted until NWAIT is sensed inactive (high level when WAITPOL = 0, low level when WAITPOL = 1).

When NWAIT is inactive, the data is considered valid either immediately (bit WAITCFG = 1) or on the next clock edge (bit WAITCFG = 0).

During wait-state insertion via the NWAIT signal, the controller continues to send clock pulses to the memory, keeping the chip select and output enable signals valid, and does not consider the data valid.

There are two timing configurations for the NOR Flash NWAIT signal in burst mode:

*   Flash memory asserts the NWAIT signal one data cycle before the wait state (default after reset)
*   Flash memory asserts the NWAIT signal during the wait state

These two NOR Flash wait state configurations are supported by the FSMC, individually for each chip select, thanks to the WAITCFG bit in the FSMC_BCRx registers (x = 0..3).

**Figure 451. Wait configurations**

Memory transaction = burst of 4 half words

HCLK
CLK
A[25:16] addr[25:16]
NADV
NWAIT (WAITCFG = 0)
NWAIT (WAITCFG = 1)
inserted wait state
A/D[15:0] aaddr[15:0] data data data data

ai15798b

1572/1749
RM0090 Rev 18

--- Page 13 ---

RM0090
Flexible static memory controller (FSMC)

Figure 452. Synchronous multiplexed read mode - NOR, PSRAM (CRAM)

1. Byte lane outputs BL are not shown; for NOR access, they are held high, and, for PSRAM (CRAM) access, they are held low.
2. NWAIT polarity is set to 0.

Table 242. FSMC_BCRx bit fields

| Bit No. | Bit name | Value to set |
|---|---|---|
| 31-20 | Reserved | 0x000 |
| 19 | CBURSTRW | No effect on synchronous read |
| 18-16 | CPSIZE | As needed (0x1 for CRAM 1.5) |
| 15 | ASCYCWAIT | 0x0 |
| 14 | EXTMOD | 0x0 |
| 13 | WAITEN | Set to 1 if the memory supports this feature, otherwise keep at 0. |
| 12 | WREN | no effect on synchronous read |
| 11 | WAITCFG | to be set according to memory |
| 10 | WRAPMOD | 0x0 |
| 9 | WAITPOL | to be set according to memory |
| 8 | BURSTEN | 0x1 |
| 7 | Reserved | 0x1 |
| 6 | FACCEN | Set according to memory support (NOR Flash memory) |
| 5-4 | MWID | As needed |


ST
RM0090 Rev 18
1573/1749

--- Page 14 ---

Flexible static memory controller (FSMC)
RM0090

Table 242. FSMC_BCRx bit fields (continued)

| Bit No. | Bit name | Value to set |
|---|---|---|
| 3-2 | MTYP[0:1] | 0x1 or 0x2 |
| 1 | MUXEN | As needed |
| 0 | MBKEN | 0x1 |


Table 243. FSMC_BTRx bit fields

| Bit No. | Bit name | Value to set |
|---|---|---|
| 31:30 | Reserved | 0x0 |
| 29:28 | ACCMOD | 0x0 |
| 27-24 | DATLAT | Data latency |
| 23-20 | CLKDIV | 0x0 to get CLK = HCLK (not supported)0x1 to get CLK = 2 × HCLK.. |
| 19-16 | BUSTURN | Time between NEx high to NEx low (BUSTURN HCLK) |
| 15-8 | DATAST | Don't care |
| 7-4 | ADDHLD | Don't care |
| 3-0 | ADDSET[3:0] | Don't care |


1574/1749
RM0090 Rev 18
ST

--- Page 15 ---

RM0090
Flexible static memory controller (FSMC)

Figure 453. Synchronous multiplexed write mode - PSRAM (CRAM)

1. Memory must issue NWAIT signal one cycle in advance, accordingly WAITCFG must be programmed to 0.
2. NWAIT polarity is set to 0.
3. Byte Lane (NBL) outputs are not shown, they are held low while NEx is active.

Table 244. FSMC_BCRx bit fields

| Bit No. | Bit name | Value to set |
|---|---|---|
| 31-20 | Reserved | 0x000 |
| 19 | CBURSTRW | 0x1 |
| 18-16 | CPSIZE | As needed (0x1 for CRAM 1,5) |
| 15 | ASCYWAIT | 0x0 |
| 14 | EXTMOD | 0x0 |
| 13 | WAITEN | Set to 1 if the memory supports this feature, otherwise keep at 0. |
| 12 | WREN | 0x1 |
| 11 | WAITCFG | 0x0 |
| 10 | WRAPMOD | 0x0 |


RM0090 Rev 18
1575/1749

--- Page 16 ---

Flexible static memory controller (FSMC)
RM0090

Table 244. FSMC_BCRx bit fields (continued)

| Bit No. | Bit name | Value to set |
|---|---|---|
| 9 | WAITPOL | to be set according to memory |
| 8 | BURSTEN | no effect on synchronous write |
| 7 | Reserved | 0x1 |
| 6 | FACCEN | Set according to memory support |
| 5-4 | MWID | As needed |
| 3-2 | MTYP[0:1] | 0x1 |
| 1 | MUXEN | As needed |
| 0 | MBKEN | 0x1 |


Table 245. FSMC_BTRx bit fields

| Bit No. | Bit name | Value to set |
|---|---|---|
| 31:30 | Reserved | 0x0 |
| 29:28 | ACCMOD | 0x0 |
| 27-24 | DATLAT | Data latency |
| 23-20 | CLKDIV | 0x0 to get CLK = HCLK (not supported) 0x1 to get CLK = 2 × HCLK |
| 19-16 | BUSTURN | Time between NEx high to NEx low (BUSTURN HCLK) |
| 15-8 | DATAST | Don't care |
| 7-4 | ADDHLD | Don't care |
| 3-0 | ADDSET[3:0] | Don't care |


1576/1749
RM0090 Rev 18
ST

--- Page 17 ---

RM0090
Flexible static memory controller (FSMC)


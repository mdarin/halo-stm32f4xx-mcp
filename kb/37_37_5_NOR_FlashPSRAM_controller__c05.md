> [RM0090 ch.37 §37.5] NOR Flash/PSRAM controller


#### Data latency versus NOR memory latency

The data latency is the number of cycles to wait before sampling the data. The DATLAT value must be consistent with the latency value specified in the NOR Flash configuration

1634/1749
RM0090 Rev 18
ST

--- Page 75 ---

RM0090
Flexible memory controller (FMC)

register. The FMC does not include the clock cycle when NADV is low in the data latency count.

**Caution:** Some NOR Flash memories include the NADV Low cycle in the data latency count, so that the exact relation between the NOR Flash latency and the FMC DATLAT parameter can be either:

*   NOR Flash latency = (DATLAT + 2) CLK clock cycles
*   or NOR Flash latency = (DATLAT + 3) CLK clock cycles

Some recent memories assert NWAIT during the latency phase. In such cases DATLAT can be set to its minimum value. As a result, the FMC samples the data and waits long enough to evaluate if the data are valid. Thus the FMC detects when the memory exits latency and real data are processed.

Other memories do not assert NWAIT during latency. In this case the latency must be set correctly for both the FMC and the memory, otherwise invalid data are mistaken for good data, or valid data are lost in the initial phase of the memory access.

**Single-burst transfer**

When the selected bank is configured in burst mode for synchronous accesses, if for example an AHB single-burst transaction is requested on 16-bit memories, the FMC performs a burst transaction of length 1 (if the AHB transfer is 16 bits), or length 2 (if the AHB transfer is 32 bits) and de-assert the Chip Select signal when the last data is strobed.

Such transfers are not the most efficient in terms of cycles compared to asynchronous read operations. Nevertheless, a random asynchronous access would first require to re-program the memory access mode, which would altogether last longer.

**Cross boundary page for Cellular RAM 1.5**

Cellular RAM 1.5 does not allow burst access to cross the page boundary. The FMC controller allows to split automatically the burst access when the memory page size is reached by configuring the CPSIZE bits in the FMC_BCR1 register following the memory page size.

**Wait management**

For synchronous NOR Flash memories, NWAIT is evaluated after the programmed latency period, which corresponds to (DATLAT+2) CLK clock cycles.

If NWAIT is active (low level when WAITPOL = 0, high level when WAITPOL = 1), wait states are inserted until NWAIT is inactive (high level when WAITPOL = 0, low level when WAITPOL = 1).

When NWAIT is inactive, the data is considered valid either immediately (bit WAITCFG = 1) or on the next clock edge (bit WAITCFG = 0).

During wait-state insertion via the NWAIT signal, the controller continues to send clock pulses to the memory, keeping the Chip Select and output enable signals valid. It does not consider the data as valid.

In burst mode, there are two timing configurations for the NOR Flash NWAIT signal:

*   The Flash memory asserts the NWAIT signal one data cycle before the wait state (default after reset).
*   The Flash memory asserts the NWAIT signal during the wait state

ST
RM0090 Rev 18
1635/1749

--- Page 76 ---

Flexible memory controller (FMC)
RM0090

The FMC supports both NOR Flash wait state configurations, for each Chip Select, thanks to the WAITCFG bit in the FMC_BCRx registers (x = 0..3).

Figure 473. Wait configuration waveforms

Figure 474. Synchronous multiplexed read mode waveforms - NOR, PSRAM (CRAM)

1. Byte lane outputs BL are not shown; for NOR access, they are held high, and, for PSRAM (CRAM) access,

1636/1749
RM0090 Rev 18

--- Page 77 ---

RM0090
Flexible memory controller (FMC)

they are held low.

Table 285. FMC_BCRx bit fields

| Bit No. | Bit name | Value to set |
|---|---|---|
| 31-21 | Reserved | 0x000 |
| 20 | CCLKEN | As needed |
| 19 | CBURSTRW | No effect on synchronous read |
| 18-16 | CPSIZE | As needed (0x1 for CRAM 1.5) |
| 15 | ASYNCWAIT | 0x0 |
| 14 | EXTMOD | 0x0 |
| 13 | WAITEN | to be set to 1 if the memory supports this feature, to be kept at 0 otherwise |
| 12 | WREN | no effect on synchronous read |
| 11 | WAITCFG | to be set according to memory |
| 10 | WRAPMOD | 0x0 |
| 9 | WAITPOL | to be set according to memory |
| 8 | BURSTEN | 0x1 |
| 7 | Reserved | 0x1 |
| 6 | FACCEN | Set according to memory support (NOR Flash memory) |
| 5-4 | MWID | As needed |
| 3-2 | MTYP[1:0] | 0x1 or 0x2 |
| 1 | MUXEN | As needed |
| 0 | MBKEN | 0x1 |


Table 286. FMC_BTRx bit fields

| Bit No. | Bit name | Value to set |
|---|---|---|
| 31:30 | Reserved | 0x0 |
| 29:28 | ACCMOD | 0x0 |
| 27-24 | DATLAT | Data latency |
| 27-24 | DATLAT | Data latency |
| 23-20 | CLKDIV | 0x0 to get CLK = HCLK (Not supported) 0x1 to get CLK = 2 × HCLK .. |
| 19-16 | BUSTURN | Time between NEx high to NEx low (BUSTURN HCLK) |
| 15-8 | DATAST | Don't care |
| 7-4 | ADDHLD | Don't care |
| 3-0 | ADDSET[3:0] | Don't care |


ST
RM0090 Rev 18
1637/1749

--- Page 78 ---

Flexible memory controller (FMC)
RM0090

Figure 475. Synchronous multiplexed write mode waveforms - PSRAM (CRAM)

Memory transaction = burst of 2 half words

HCLK
CLK
A[25:16]
addr[25:16]
NEx
NOE
Hi-Z
NWE
NADV
NWAIT
(WAITCFG = 0)
A/D[15:0]
1 clock
1 clock
Addr[15:0]
(DATLAT + 2)
CLK cycles
data
inserted wait state
data
ai14731f

1. The memory must issue NWAIT signal one cycle in advance, accordingly WAITCFG must be programmed to 0.
2. Byte Lane (NBL) outputs are not shown, they are held low while NEx is active.

Table 287. FMC_BCRx bit fields

| Bit No. | Bit name | Value to set |
|---|---|---|
| 31-20 | Reserved | 0x000 |
| 20 | CCLKEN | As needed |
| 19 | CBURSTRW | 0x1 |
| 18-16 | CPSIZE | As needed (0x1 for CRAM 1.5) |
| 15 | ASYNCWAIT | 0x0 |
| 14 | EXTMOD | 0x0 |
| 13 | WAITEN | to be set to 1 if the memory supports this feature, to be kept at 0 otherwise. |
| 12 | WREN | 0x1 |


1638/1749
RM0090 Rev 18

--- Page 79 ---

RM0090
Flexible memory controller (FMC)

**Table 287. FMC_BCRx bit fields (continued)**

| Bit No. | Bit name | Value to set |
|---|---|---|
| 11 | WAITCFG | 0x0 |
| 10 | WRAPMOD | 0x0 |
| 9 | WAITPOL | to be set according to memory |
| 8 | BURSTEN | no effect on synchronous write |
| 7 | Reserved | 0x1 |
| 6 | FACCEN | Set according to memory support |
| 5-4 | MWID | As needed |
| 3-2 | MTYP[1:0] | 0x1 |
| 1 | MUXEN | As needed |
| 0 | MBKEN | 0x1 |


**Table 288. FMC_BTRx bit fields**

| Bit No. | Bit name | Value to set |
|---|---|---|
| 31-30 | Reserved | 0x0 |
| 29:28 | ACCMOD | 0x0 |
| 27-24 | DATLAT | Data latency |
| 23-20 | CLKDIV | 0x0 to get CLK = HCLK (not supported)0x1 to get CLK = 2 × HCLK |
| 19-16 | BUSTURN | Time between NEx high to NEx low (BUSTURN HCLK) |
| 15-8 | DATAST | Don't care |
| 7-4 | ADDHLD | Don't care |
| 3-0 | ADDSET[3:0] | Don't care |


STI
RM0090 Rev 18
1639/1749

--- Page 80 ---

Flexible memory controller (FMC)
RM0090


> [RM0090 ch.36 §36.5] NOR Flash/PSRAM controller


### 36.5.4 NOR Flash/PSRAM controller asynchronous transactions

**Asynchronous static memories (NOR Flash memory, PSRAM, SRAM)**

*   Signals are synchronized by the internal clock HCLK. This clock is not issued to the memory
*   The FSMC always samples the data before de-asserting the NOE signals. This guarantees that the memory data-hold timing constraint is met (chip enable high to data transition, usually 0 ns min.)
*   If the extended mode is enabled (EXTMOD bit is set in the FSMC_BCRx register), up to four extended modes (A, B, C and D) are available. It is possible to mix A, B, C and D modes for read and write operations. For example, read operation can be performed in mode A and write in mode B.
*   If the extended mode is disabled (EXTMOD bit is reset in the FSMC_BCRx register), the FSMC can operate in Mode1 or Mode2 as follows:
    *   Mode 1 is the default mode when SRAM/PSRAM memory type is selected (MTYP[0:1] = 0x0 or 0x01 in the FSMC_BCRx register)
    *   Mode 2 is the default mode when NOR memory type is selected (MTYP[0:1] = 0x10 in the FSMC_BCRx register).

**Mode 1 - SRAM/PSRAM (CRAM)**

The next figures show the read and write transactions for the supported modes followed by the required configuration of FSMC_BCRx, and FSMC_BTRx/FSMC_BWTRx registers.

**Figure 436. Mode1 read accesses**

ai15557

1. NBL[1:0] are driven low during read access.

1554/1749
RM0090 Rev 18
STI

--- Page 190 ---

RM0090
Flexible static memory controller (FSMC)

**Figure 437. Mode1 write accesses**

ai15558

The one HCLK cycle at the end of the write transaction helps guarantee the address and data hold time after the NWE rising edge. Due to the presence of this one HCLK cycle, the DATAST value must be greater than zero (DATAST > 0).

**Table 226. FSMC_BCRx bit fields**

| Bit number | Bit name | Value to set |
|---|---|---|
| 31-20 | Reserved | 0x000 |
| 19 | CBURSTRW | 0x0 (no effect on asynchronous mode) |
| 18:16 | CPSIZE | 0x0 (no effect on asynchronous mode) |
| 15 | ASYNCWAIT | Set to 1 if the memory supports this feature. Otherwise keep at 0. |
| 14 | EXTMOD | 0x0 |
| 13 | WAITEN | 0x0 (no effect on asynchronous mode) |
| 12 | WREN | As needed |
| 11 | WAITCFG | Don't care |
| 10 | WRAPMOD | 0x0 |
| 9 | WAITPOL | Meaningful only if bit 15 is 1 |
| 8 | BURSTEN | 0x0 |
| 7 | Reserved | 0x1 |
| 6 | FACCEN | Don't care |
| 5-4 | MWID | As needed |
| 3-2 | MTYP[0:1] | As needed, exclude 0x2 (NOR Flash) |


ST
RM0090 Rev 18
1555/1749

--- Page 191 ---

Flexible static memory controller (FSMC)
RM0090

Table 226. FSMC_BCRx bit fields (continued)

| Bit number | Bit name | Value to set |
|---|---|---|
| 1 | MUXE | 0x0 |
| 0 | MBKEN | 0x1 |


Table 227. FSMC_BTRx bit fields

| Bit number | Bit name | Value to set |
|---|---|---|
| 31:30 | Reserved | 0x0 |
| 29-28 | ACCMOD | Don't care |
| 27-24 | DTLAT | Don't care |
| 23-20 | CLKDIV | Don't care |
| 19-16 | BUSTURN | Time between NEx high to NEx low (BUSTURN HCLK) |
| 15-8 | DATAST | Duration of the second access phase (DATAST+1 HCLK cycles for write accesses, DATAST HCLK cycles for read accesses). |
| 7-4 | ADDHLD | Don't care |
| 3-0 | ADDSET[3:0] | Duration of the first access phase (ADDSET HCLK cycles). Minimum value for ADDSET is 0. |


Mode A - SRAM/PSRAM (CRAM) OE toggling

Figure 438. ModeA read accesses

Memory transaction
A[25:0]
NBL[1:0]
NEx
NOE
NWE
High
D[15:0]
data driven by memory
ADDSET HCLK cycles
DATAST HCLK cycles

ai15559

1. NBL[1:0] are driven low during read access.

1556/1749
RM0090 Rev 18

--- Page 192 ---

RM0090
Flexible static memory controller (FSMC)

Figure 439. ModeA write accesses

The differences compared with mode1 are the toggling of NOE and the independent read and write timings.

Table 228. FSMC_BCRx bit fields

| Bit number | Bit name | Value to set |
|---|---|---|
| 31-20 | Reserved | 0x000 |
| 19 | CBURSTRW | 0x0 (no effect on asynchronous mode) |
| 18:16 | CPSIZE | 0x0 (no effect on asynchronous mode) |
| 15 | ASYNCWAIT | Set to 1 if the memory supports this feature. Otherwise keep at 0. |
| 14 | EXTMOD | 0x1 |
| 13 | WAITEN | 0x0 (no effect on asynchronous mode) |
| 12 | WREN | As needed |
| 11 | WAITCFG | Don't care |
| 10 | WRAPMOD | 0x0 |
| 9 | WAITPOL | Meaningful only if bit 15 is 1 |
| 8 | BURSTEN | 0x0 |
| 7 | Reserved | 0x1 |
| 6 | FACCEN | Don't care |
| 5-4 | MWID | As needed |
| 3-2 | MTYP[0:1] | As needed, exclude 0x2 (NOR Flash) |


ST
RM0090 Rev 18
1557/1749

--- Page 193 ---

**Flexible static memory controller (FSMC)**

RM0090

**Table 228. FSMC_BCRx bit fields (continued)**

| Bit number | Bit name | Value to set |
|---|---|---|
| 1 | MUXEN | 0x0 |
| 0 | MBKEN | 0x1 |


**Table 229. FSMC_BTRx bit fields**

| Bit number | Bit name | Value to set |
|---|---|---|
| 31:30 | Reserved | 0x0 |
| 29-28 | ACCMOD | 0x0 |
| 27-24 | DATLAT | Don't care |
| 23-20 | CLKDIV | Don't care |
| 19-16 | BUSTURN | Time between NEx high to NEx low (BUSTURN HCLK) |
| 15-8 | DATAST | Duration of the second access phase (DATAST HCLK cycles) for read accesses. |
| 7-4 | ADDHLD | Don't care |
| 3-0 | ADDSET[3:0] | Duration of the first access phase (ADDSET HCLK cycles) for read accesses. Minimum value for ADDSET is 0. |


**Table 230. FSMC_BWTRx bit fields**

| Bit number | Bit name | Value to set |
|---|---|---|
| 31:30 | Reserved | 0x0 |
| 29-28 | ACCMOD | 0x0 |
| 27-24 | DATLAT | Don't care |
| 23-20 | CLKDIV | Don't care |
| 19-16 | BUSTURN | Time between NEx high to NEx low (BUSTURN HCLK) |
| 15-8 | DATAST | Duration of the second access phase (DATAST+1 HCLK cycles for write accesses, |
| 7-4 | ADDHLD | Don't care |
| 3-0 | ADDSET[3:0] | Duration of the first access phase (ADDSET HCLK cycles) for write accesses. Minimum value for ADDSET is 0. |


1558/1749

RM0090 Rev 18

ST

--- Page 194 ---

RM0090
Flexible static memory controller (FSMC)

**Mode 2/B - NOR Flash**

**Figure 440. Mode2 and mode B read accesses**

Memory transaction
A[25:0]
NADV
NEx
NOE
NWE
High
D[15:0]
ADDSET
HCLK cycles
DATAST
HCLK cycles
data driven
by memory
ai15561

**Figure 441. Mode2 write accesses**

Memory transaction
A[25:0]
NADV
NEx
NOE
NWE
1HCLK
D[15:0]
ADDSET
HCLK cycles
(DATAST + 1)
HCLK cycles
data driven by FSMC
ai15562

STi
RM0090 Rev 18
1559/1749

--- Page 195 ---

Flexible static memory controller (FSMC)
RM0090

Figure 442. Mode B write accesses

ai15563

The differences with mode1 are the toggling of NWE and the independent read and write timings when extended mode is set (Mode B).

Table 231. FSMC_BCRx bit fields

| Bit number | Bit name | Value to set |
|---|---|---|
| 31-20 | Reserved | 0x000 |
| 19 | CBURSTRW | 0x0 (no effect on asynchronous mode) |
| 18:16 | Reserved | 0x0 (no effect on asynchronous mode) |
| 15 | ASYNCWAIT | Set to 1 if the memory supports this feature. Otherwise keep at 0. |
| 14 | EXTMOD | 0x1 for mode B, 0x0 for mode 2 |
| 13 | WAITEN | 0x0 (no effect on asynchronous mode) |
| 12 | WREN | As needed |
| 11 | WAITCFG | Don't care |
| 10 | WRAPMOD | 0x0 |
| 9 | WAITPOL | Meaningful only if bit 15 is 1 |
| 8 | BURSTEN | 0x0 |
| 7 | Reserved | 0x1 |
| 6 | FACCEN | 0x1 |
| 5-4 | MWID | As needed |


1560/1749
RM0090 Rev 18
--- Page 1 ---

RM0090
Flexible static memory controller (FSMC)

**Table 231. FSMC_BCRx bit fields (continued)**

| Bit number | Bit name | Value to set |
|---|---|---|
| 3-2 | MTYP[0:1] | 0x2 (NOR Flash memory) |
| 1 | MUXEN | 0x0 |
| 0 | MBKEN | 0x1 |


**Table 232. FSMC_BTRx bit fields**

| Bit number | Bit name | Value to set |
|---|---|---|
| 31:30 | Reserved | 0x0 |
| 29-28 | ACCMOD | 0x1 |
| 27-24 | DATLAT | Don't care |
| 23-20 | CLKDIV | Don't care |
| 19-16 | BUSTURN | Time between NEx high to NEx low (BUSTURN HCLK) |
| 15-8 | DATAST | Duration of the second access phase (DATAST HCLK cycles) for read accesses. |
| 7-4 | ADDHLD | Don't care |
| 3-0 | ADDSET[3:0] | Duration of the first access phase (ADDSET HCLK cycles) for read accesses. Minimum value for ADDSET is 0. |


**Table 233. FSMC_BWTRx bit fields**

| Bit number | Bit name | Value to set |
|---|---|---|
| 31:30 | Reserved | 0x0 |
| 29-28 | ACCMOD | 0x1 |
| 27-24 | DATLAT | Don't care |
| 23-20 | CLKDIV | Don't care |
| 19-16 | BUSTURN | Time between NEx high to NEx low (BUSTURN HCLK) |
| 15-8 | DATAST | Duration of the second access phase (DATAST+1 HCLK cycles for write accesses, |
| 7-4 | ADDHLD | Don't care |
| 3-0 | ADDSET[3:0] | Duration of the first access phase (ADDSET HCLK cycles) for write accesses. Minimum value for ADDSET is 0. |


*Note:* The FSMC_BWTRx register is valid only if extended mode is set (mode B), otherwise all its content is don't care.

STI
RM0090 Rev 18
1561/1749

--- Page 2 ---

Flexible static memory controller (FSMC)
RM0090

**Mode C - NOR Flash - OE toggling**

**Figure 443. Mode C read accesses**

Memory transaction
A[25:0]
NADV
NEx
NOE
NWE
High
D[15:0]
data driven by memory
ADDSET HCLK cycles
DATAST HCLK cycles
ai15564

**Figure 444. Mode C write accesses**

Memory transaction
A[25:0]
NADV
NEx
NOE
NWE
1HCLK
D[15:0]
data driven by FSMC
ADDSET HCLK cycles
(DATAST + 1) HCLK cycles
ai15565

The differences compared with mode1 are the toggling of NOE and the independent read and write timings.

1562/1749
RM0090 Rev 18

--- Page 3 ---

RM0090
Flexible static memory controller (FSMC)

Table 234. FSMC_BCRx bit fields

| Bit No. | Bit name | Value to set |
|---|---|---|
| 31-20 | Reserved | 0x000 |
| 19 | CBURSTRW | 0x0 (no effect on asynchronous mode) |
| 18:16 | CPSIZE | 0x0 (no effect on asynchronous mode) |
| 15 | ASYNCWAIT | Set to 1 if the memory supports this feature. Otherwise keep at 0. |
| 14 | EXTMOD | 0x1 |
| 13 | WAITEN | 0x0 (no effect on asynchronous mode) |
| 12 | WREN | As needed |
| 11 | WAITCFG | Don't care |
| 10 | WRAPMOD | 0x0 |
| 9 | WAITPOL | Meaningful only if bit 15 is 1 |
| 8 | BURSTEN | 0x0 |
| 7 | Reserved | 0x1 |
| 6 | FACCEN | 0x1 |
| 5-4 | MWID | As needed |
| 3-2 | MTYP[0:1] | 0x2 (NOR Flash memory) |
| 1 | MUXEN | 0x0 |
| 0 | MBKEN | 0x1 |


Table 235. FSMC_BTRx bit fields

| Bit number | Bit name | Value to set |
|---|---|---|
| 31:30 | Reserved | 0x0 |
| 29-28 | ACCMOD | 0x2 |
| 27-24 | DATLAT | 0x0 |
| 23-20 | CLKDIV | 0x0 |
| 19-16 | BUSTURN | Time between NEx high to NEx low (BUSTURN HCLK) |
| 15-8 | DATAST | Duration of the second access phase (DATAST HCLK cycles) for read accesses. |
| 7-4 | ADDHLD | Don't care |
| 3-0 | ADDSET[3:0] | Duration of the first access phase (ADDSET HCLK cycles) for read accesses. Minimum value for ADDSET is 0. |


ST
RM0090 Rev 18
1563/1749

--- Page 4 ---

Flexible static memory controller (FSMC)
RM0090

Table 236. FSMC_BWTRx bit fields

| Bit number | Bit name | Value to set |
|---|---|---|
| 31:30 | Reserved | 0x0 |
| 29-28 | ACCMOD | 0x2 |
| 27-24 | DATLAT | Don't care |
| 23-20 | CLKDIV | Don't care |
| 19-16 | BUSTURN | Time between NEx high to NEx low (BUSTURN HCLK) |
| 15-8 | DATAST | Duration of the second access phase (DATAST+1 HCLK cycles for write accesses, |
| 7-4 | ADDHLD | Don't care |
| 3-0 | ADDSET[3:0] | Duration of the first access phase (ADDSET HCLK cycles) for write accesses. Minimum value for ADDSET is 0. |


Mode D - asynchronous access with extended address

Figure 445. Mode D read accesses

Memory transaction
A[25:0]
NADV
NEx
NOE
NWE
High
D[15:0]
data driven by memory
ADDSET HCLK cycles
ADDHLD HCLK cycles
DATAST HCLK cycles
ai15566

1564/1749
RM0090 Rev 18
ST

--- Page 5 ---

RM0090
Flexible static memory controller (FSMC)

**Figure 446. Mode D write accesses**

Memory transaction
A[25:0]
NADV
NEx
NOE
NWE
D[15:0]
1HCLK
ADDSET
HCLK cycles
ADDHLD
HCLK cycles
(DATAST+ 1)
HCLK cycles
data driven by FSMC
ai15567

The differences with mode1 are the toggling of NOE that goes on toggling after NADV changes and the independent read and write timings.

**Table 237. FSMC_BCRx bit fields**

| Bit No. | Bit name | Value to set |
|---|---|---|
| 31-20 | Reserved | 0x000 |
| 19 | CBURSTRW | 0x0 (no effect on asynchronous mode) |
| 18:16 | CPSIZE | 0x0 (no effect on asynchronous mode) |
| 15 | ASYNCWAIT | Set to 1 if the memory supports this feature. Otherwise keep at 0. |
| 14 | EXTMOD | 0x1 |
| 13 | WAITEN | 0x0 (no effect on asynchronous mode) |
| 12 | WREN | As needed |
| 11 | WAITCFG | Don't care |
| 10 | WRAPMOD | 0x0 |
| 9 | WAITPOL | Meaningful only if bit 15 is 1 |
| 8 | BURSTEN | 0x0 |
| 7 | Reserved | 0x1 |
| 6 | FACCEN | Set according to memory support |
| 5-4 | MWID | As needed |
| 3-2 | MTYP[0:1] | As needed |


ST
RM0090 Rev 18
1565/1749

--- Page 6 ---

Flexible static memory controller (FSMC)
RM0090

Table 237. FSMC_BCRx bit fields (continued)

| Bit No. | Bit name | Value to set |
|---|---|---|
| 1 | MUXEN | 0x0 |
| 0 | MBKEN | 0x1 |


Table 238. FSMC_BTRx bit fields

| Bit No. | Bit name | Value to set |
|---|---|---|
| 31:30 | Reserved | 0x0 |
| 29-28 | ACCMOD | 0x3 |
| 27-24 | DATLAT | Don't care |
| 23-20 | CLKDIV | Don't care |
| 19-16 | BUSTURN | Time between NEx high to NEx low (BUSTURN HCLK) |
| 15-8 | DATAST | Duration of the second access phase (DATAST HCLK cycles) for read accesses. |
| 7-4 | ADDHLD | Duration of the middle phase of the read access (ADDHLD HCLK cycles) |
| 3-0 | ADDSET[3:0] | Duration of the first access phase (ADDSET HCLK cycles) for read accesses. Minimum value for ADDSET is 0. |


Table 239. FSMC_BWTRx bit fields

| Bit No. | Bit name | Value to set |
|---|---|---|
| 31:30 | Reserved | 0x0 |
| 29-28 | ACCMOD | 0x3 |
| 27-24 | DATLAT | 0x0 |
| 23-20 | CLKDIV | 0x0 |
| 19-16 | BUSTURN | Time between NEx high to NEx low (BUSTURN HCLK) |
| 15-8 | DATAST | Duration of the second access phase (DATAST+1 HCLK cycles) for write accesses |
| 7-4 | ADDHLD | Duration of the middle phase of the write access (ADDHLD HCLK cycles) |
| 3-0 | ADDSET[3:0] | Duration of the first access phase (ADDSET+1 HCLK cycles) for write accesses. Minimum value for ADDSET is 0. |


1566/1749
RM0090 Rev 18

--- Page 7 ---

RM0090
Flexible static memory controller (FSMC)

Muxed mode - multiplexed asynchronous access to NOR Flash memory

Figure 447. Multiplexed read accesses

ai15568

Memory transaction
A[25:16]
NADV
NEX
NOE
NWE High
AD[15:0]
Lower address
data driven by memory
ADDSET HCLK cycles
ADDHLD HCLK cycles
DATAST HCLK cycles

Figure 448. Multiplexed write accesses

ai15569

Memory transaction
A[25:16]
NADV
NEX
NOE
NWE
AD[15:0]
Lower address
data driven by FSMC
ADDSET HCLK cycles
ADDHLD HCLK cycles
DATAST + 1) HCLK cycles

The difference with mode D is the drive of the lower address byte(s) on the databus.

RM0090 Rev 18
1567/1749

--- Page 8 ---

Flexible static memory controller (FSMC)
RM0090

Table 240. FSMC_BCRx bit fields

| Bit No. | Bit name | Value to set |
|---|---|---|
| 31-21 | Reserved | 0x000 |
| 19 | CBURSTRW | 0x0 (no effect on asynchronous mode) |
| 18:16 | CPSIZE | 0x0 (no effect on asynchronous mode) |
| 15 | ASYNCWAIT | Set to 1 if the memory supports this feature. Otherwise keep at 0. |
| 14 | EXTMOD | 0x0 |
| 13 | WAITEN | 0x0 (no effect on asynchronous mode) |
| 12 | WREN | As needed |
| 11 | WAITCFG | Don't care |
| 10 | WRAPMOD | 0x0 |
| 9 | WAITPOL | Meaningful only if bit 15 is 1 |
| 8 | BURSTEN | 0x0 |
| 7 | Reserved | 0x1 |
| 6 | FACCEN | 0x1 |
| 5-4 | MWID | As needed |
| 3-2 | MTYP[0:1] | 0x2 (NOR Flash memory) |
| 1 | MUXEN | 0x1 |
| 0 | MBKEN | 0x1 |


Table 241. FSMC_BTRx bit fields

| Bit No. | Bit name | Value to set |
|---|---|---|
| 31:30 | Reserved | 0x0 |
| 29-28 | ACCMOD | 0x0 |
| 27-24 | DATLAT | Don't care |
| 23-20 | CLKDIV | Don't care |
| 19-16 | BUSTURN | Time between NEx high to NEx low (BUSTURN HCLK) |
| 15-8 | DATAST | Duration of the second access phase (DATAST HCLK cycles for read accesses and DATAST+1 HCLK cycles for write accesses). |
| 7-4 | ADDHLD | Duration of the middle phase of the access (ADDHLD HCLK cycles). |
| 3-0 | ADDSET[3:0] | Duration of the first access phase (ADDSET HCLK cycles). Minimum value for ADDSET is 1. |


**WAIT management in asynchronous accesses**

If the asynchronous memory asserts a WAIT signal to indicate that it is not yet ready to accept or to provide data, the ASYNCWAIT bit has to be set in FSMC_BCRx register.

1568/1749
RM0090 Rev 18
ST

--- Page 9 ---

RM0090
Flexible static memory controller (FSMC)

If the WAIT signal is active (high or low depending on the WAITPOL bit), the second access phase (Data setup phase) programmed by the DATAST bits, is extended until WAIT becomes inactive. Unlike the data setup phase, the first access phases (Address setup and Address hold phases), programmed by the ADDSET[3:0] and ADDHLD bits, are not WAIT sensitive and so they are not prolonged.

The data setup phase (DATAST in the FSMC_BTRx register) must be programmed so that WAIT can be detected 4 HCLK cycles before the end of memory transaction. The following cases must be considered:

1. DATAST in FSMC_BTRx register) Memory asserts the WAIT signal aligned to NOE/NWE which toggles:

DATAST ≥ (4 × HCLK) + max_wait_assertion_time

2. Memory asserts the WAIT signal aligned to NEx (or NOE/NWE not toggling):
if

max_wait_assertion_time > address_phase + hold_phase

then

DATAST ≥ (4 × HCLK) + (max_wait_assertion_time – address_phase – hold_phase)

otherwise

DATAST ≥ 4 × HCLK

where max_wait_assertion_time is the maximum time taken by the memory to assert the WAIT signal once NEx/NOE/NWE is low.

*Figure 449* and *Figure 450* show the number of HCLK clock cycles that are added to the memory access after WAIT is released by the asynchronous memory (independently of the above cases).

ST
RM0090 Rev 18
1569/1749

--- Page 10 ---

Flexible static memory controller (FSMC)
RM0090

Figure 449. Asynchronous wait during a read access

Memory transaction
A[25:0]
address phase
data setup phase
NEx
NWAIT
don't care
don't care
NOE
D[15:0]
data driven
by memory
4HCLK
ai18471b

1. NWAIT polarity depends on WAITPOL bit setting in FSMC_BCRx register.

Figure 450. Asynchronous wait during a write access

Memory transaction
A[25:0]
address phase
data setup phase
NEx
NWAIT
don't care
don't care
1HCLK
NWE
D[15:0]
data driven by FSMC
3HCLK
ai15797c

1. NWAIT polarity depends on WAITPOL bit setting in FSMC_BCRx register.

1570/1749
RM0090 Rev 18

--- Page 11 ---

RM0090
Flexible static memory controller (FSMC)

> [RM0090 ch.36 §36.5] NOR Flash/PSRAM controller

### 36.5.5 Synchronous transactions

### 36.5.5 Synchronous transactions

The memory clock, CLK, is a submultiple of HCLK according to the value of parameter CLKDIV.

NOR Flash memories specify a minimum time from NADV assertion to CLK high. To meet this constraint, the FSMC does not issue the clock to the memory during the first internal clock cycle of the synchronous access (before NADV assertion). This guarantees that the rising edge of the memory clock occurs *in the middle* of the NADV low pulse.

> [RM0090 ch.36 §36.5] NOR Flash/PSRAM controller

### Data latency versus NOR Flash latency

#### Data latency versus NOR Flash latency

The data latency is the number of cycles to wait before sampling the data. The DATLAT value must be consistent with the latency value specified in the NOR Flash configuration register. The FSMC does not include the clock cycle when NADV is low in the data latency count.

**Caution:** Some NOR Flash memories include the NADV Low cycle in the data latency count, so the exact relation between the NOR Flash latency and the FMSC DATLAT parameter can be either of:

* NOR Flash latency = (DATLAT + 2) CLK clock cycles
* NOR Flash latency = (DATLAT + 3) CLK clock cycles

Some recent memories assert NWAIT during the latency phase. In such cases DATLAT can be set to its minimum value. As a result, the FSMC samples the data and waits long enough to evaluate if the data are valid. Thus the FSMC detects when the memory exits latency and real data are taken.

Other memories do not assert NWAIT during latency. In this case the latency must be set correctly for both the FSMC and the memory, otherwise invalid data are mistaken for good data, or valid data are lost in the initial phase of the memory access.

> [RM0090 ch.36 §36.5] NOR Flash/PSRAM controller

### Single-burst transfer

#### Single-burst transfer

When the selected bank is configured in burst mode for synchronous accesses, if for example an AHB single-burst transaction is requested on 16-bit memories, the FSMC performs a burst transaction of length 1 (if the AHB transfer is 16-bit), or length 2 (if the AHB transfer is 32-bit) and de-assert the chip select signal when the last data is strobed.

Clearly, such a transfer is not the most efficient in terms of cycles (compared to an asynchronous read). Nevertheless, a random asynchronous access would first require to re-program the memory access mode, which would altogether last longer.

> [RM0090 ch.36 §36.5] NOR Flash/PSRAM controller

### Cross boundary page for Cellular RAM 1.5

#### Cross boundary page for Cellular RAM 1.5

Cellular RAM 1.5 does not allow burst access to cross the page boundary. The FSMC controller allows to split automatically the burst access when the memory page size is reached by configuring the CPSIZE bits in the FSMC_BCR1 register following the memory page size.


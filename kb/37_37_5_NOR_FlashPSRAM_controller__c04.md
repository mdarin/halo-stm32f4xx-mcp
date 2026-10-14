> [RM0090 ch.37 §37.5] NOR Flash/PSRAM controller


### 37.5.4 NOR Flash/PSRAM controller asynchronous transactions

**Asynchronous static memories (NOR Flash, PSRAM, SRAM)**

*   Signals are synchronized by the internal clock HCLK. This clock is not issued to the memory
*   The FMC always samples the data before de-asserting the NOE signal. This guarantees that the memory data hold timing constraint is met (minimum Chip Enable high to data transition is usually 0 ns)
*   If the extended mode is enabled (EXTMOD bit is set in the FMC_BCRx register), up to four extended modes (A, B, C and D) are available. It is possible to mix A, B, C and D modes for read and write operations. For example, read operation can be performed in mode A and write in mode B.
*   If the extended mode is disabled (EXTMOD bit is reset in the FMC_BCRx register), the FMC can operate in Mode1 or Mode2 as follows:
    *   Mode 1 is the default mode when SRAM/PSRAM memory type is selected (MTYP[1:0] = 0x0 or 0x01 in the FMC_BCRx register)
    *   Mode 2 is the default mode when NOR memory type is selected (MTYP[1:0] = 0x10 in the FMC_BCRx register).

ST
RM0090 Rev 18
1617/1749

--- Page 58 ---

Flexible memory controller (FMC)
RM0090

**Mode 1 - SRAM/PSRAM (CRAM)**

The next figures show the read and write transactions for the supported modes followed by the required configuration of FMC\_BCRx, and FMC\_BTRx/FMC\_BWTRx registers.

**Figure 458. Mode1 read access waveforms**

Memory transaction
A[25:0]
NBL[3:0]
NEx
NOE
NWE
High
D[31:0]
data driven by memory
ADDSET HCLK cycles
DATAST HCLK cycles
MS30452V1

**Figure 459. Mode1 write access waveforms**

Memory transaction
A[25:0]
NBL[3:0]
NEx
NOE
NWE
1HCLK
D[31:0]
data driven by FSMC
ADDSET HCLK cycles
(DATAST + 1) HCLK cycles
MS30453V1

1618/1749
RM0090 Rev 18
ST

--- Page 59 ---

RM0090
Flexible memory controller (FMC)

The one HCLK cycle at the end of the write transaction helps guarantee the address and data hold time after the NWE rising edge. Due to the presence of this HCLK cycle, the DATAST value must be greater than zero (DATAST > 0).

Table 269. FMC_BCRx bit fields

| Bit number | Bit name | Value to set |
|---|---|---|
| 31-21 | Reserved | 0x000 |
| 20 | CCKEN | As needed |
| 19 | CBURSTRW | 0x0 (no effect in asynchronous mode) |
| 18:16 | CPSIZE | 0x0 (no effect in asynchronous mode) |
| 15 | ASYNCWAIT | Set to 1 if the memory supports this feature. Otherwise keep at 0. |
| 14 | EXTMOD | 0x0 |
| 13 | WAITEN | 0x0 (no effect in asynchronous mode) |
| 12 | WREN | As needed |
| 11 | WAITCFG | Don't care |
| 10 | WRAPMOD | 0x0 |
| 9 | WAITPOL | Meaningful only if bit 15 is 1 |
| 8 | BURSTEN | 0x0 |
| 7 | Reserved | 0x1 |
| 6 | FACCEN | Don't care |
| 5-4 | MWID | As needed |
| 3-2 | MTYP[1:0] | As needed, exclude 0x2 (NOR Flash memory) |
| 1 | MUXE | 0x0 |
| 0 | MBKEN | 0x1 |


Table 270. FMC_BTRx bit fields

| Bit number | Bit name | Value to set |
|---|---|---|
| 31:30 | Reserved | 0x0 |
| 29-28 | ACCMOD | Don't care |
| 27-24 | DATLAT | Don't care |
| 23-20 | CLKDIV | Don't care |
| 19-16 | BUSTURN | Time between NEx high to NEx low (BUSTURN HCLK) |
| 15-8 | DATAST | Duration of the second access phase (DATAST+1 HCLK cycles for write accesses, DATAST HCLK cycles for read accesses). |


ST
RM0090 Rev 18
1619/1749

--- Page 60 ---

Flexible memory controller (FMC)
RM0090

Table 270. FMC_BTRx bit fields (continued)

| Bit number | Bit name | Value to set |
|---|---|---|
| 7-4 | ADDHLD | Don't care |
| 3-0 | ADDSET | Duration of the first access phase (ADDSET HCLK cycles). Minimum value for ADDSET is 0. |


**Mode A - SRAM/PSRAM (CRAM) OE toggling**

**Figure 460. ModeA read access waveforms**

Memory transaction
A[25:0]
NBL[3:0]
NEx
NOE
NWE
High
D[31:0]
ADDSET HCLK cycles
DATAST HCLK cycles
data driven by memory
MS30454V1

1. NBL[3:0] are driven low during the read access

1620/1749
RM0090 Rev 18
ST

--- Page 61 ---

RM0090
Flexible memory controller (FMC)

Figure 461. ModeA write access waveforms

The differences compared with mode1 are the toggling of NOE and the independent read and write timings.

Table 271. FMC_BCRx bit fields

| Bit number | Bit name | Value to set |
|---|---|---|
| 31-21 | Reserved | 0x000 |
| 20 | CCLKEN | As needed |
| 19 | CBURSTRW | 0x0 (no effect in asynchronous mode) |
| 18:16 | CPSIZE | 0x0 (no effect in asynchronous mode) |
| 15 | ASYNCWAIT | Set to 1 if the memory supports this feature. Otherwise keep at 0. |
| 14 | EXTMOD | 0x1 |
| 13 | WAITEN | 0x0 (no effect in asynchronous mode) |
| 12 | WREN | As needed |
| 11 | WAITCFG | Don't care |
| 10 | WRAPMOD | 0x0 |
| 9 | WAITPOL | Meaningful only if bit 15 is 1 |
| 8 | BURSTEN | 0x0 |
| 7 | Reserved | 0x1 |
| 6 | FACCEN | Don't care |


ST
RM0090 Rev 18
1621/1749

--- Page 62 ---

Flexible memory controller (FMC)
RM0090

Table 271. FMC_BCRx bit fields (continued)

| Bit number | Bit name | Value to set |
|---|---|---|
| 5-4 | MWID | As needed |
| 3-2 | MTYP[1:0] | As needed, exclude 0x2 (NOR Flash memory) |
| 1 | MUXEN | 0x0 |
| 0 | MBKEN | 0x1 |


Table 272. FMC_BTRx bit fields

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


Table 273. FMC_BWTRx bit fields

| Bit number | Bit name | Value to set |
|---|---|---|
| 31:30 | Reserved | 0x0 |
| 29-28 | ACCMOD | 0x0 |
| 27-24 | DATLAT | Don't care |
| 23-20 | CLKDIV | Don't care |
| 19-16 | BUSTURN | Time between NEx high to NEx low (BUSTURN HCLK) |
| 15-8 | DATAST | Duration of the second access phase (DATAST HCLK cycles) for write accesses. |
| 7-4 | ADDHLD | Don't care |
| 3-0 | ADDSET[3:0] | Duration of the first access phase (ADDSET HCLK cycles) for write accesses. Minimum value for ADDSET is 0. |


1622/1749
RM0090 Rev 18
ST

--- Page 63 ---

RM0090
Flexible memory controller (FMC)

**Mode 2/B - NOR Flash**

**Figure 462. Mode2 and mode B read access waveforms**

Memory transaction
A[25:0]
NADV
NEx
NOE
NWE
High
D[31:0]
ADDSET
HCLK cycles
DATAST
HCLK cycles
data driven
by memory
MS30456V1

1. NBL[3:0] are driven low during the read access

**Figure 463. Mode2 write access waveforms**

Memory transaction
A[25:0]
NADV
NEx
NOE
NWE
1HCLK
D[31:0]
ADDSET
HCLK cycles
(DATAST + 1)
HCLK cycles
data driven by FSMC
MS30457V1

ST
RM0090 Rev 18
1623/1749

--- Page 64 ---

Flexible memory controller (FMC)
RM0090

Figure 464. ModeB write access waveforms

The differences with mode1 are the toggling of NWE and the independent read and write timings when extended mode is set (Mode B).

Table 274. FMC_BCRx bit fields

| Bit number | Bit name | Value to set |
|---|---|---|
| 31-21 | Reserved | 0x000 |
| 20 | CCLKEN | As needed |
| 19 | CBURSTRW | 0x0 (no effect in asynchronous mode) |
| 18:16 | CPSIZE | 0x0 (no effect in asynchronous mode) |
| 15 | ASYNCWAIT | Set to 1 if the memory supports this feature. Otherwise keep at 0. |
| 14 | EXTMOD | 0x1 for mode B, 0x0 for mode 2 |
| 13 | WAITEN | 0x0 (no effect in asynchronous mode) |
| 12 | WREN | As needed |
| 11 | WAITCFG | Don't care |
| 10 | WRAPMOD | 0x0 |
| 9 | WAITPOL | Meaningful only if bit 15 is 1 |
| 8 | BURSTEN | 0x0 |
| 7 | Reserved | 0x1 |
| 6 | FACCEN | 0x1 |


1624/1749
RM0090 Rev 18

--- Page 65 ---

RM0090
Flexible memory controller (FMC)

Table 274. FMC_BCRx bit fields (continued)
| Bit number | Bit name | Value to set |
|---|---|---|
| 5-4 | MWID | As needed |
| 3-2 | MTYP[1:0] | 0x2 (NOR Flash memory) |
| 1 | MUXEN | 0x0 |
| 0 | MBKEN | 0x1 |


Table 275. FMC_BTRx bit fields
| Bit number | Bit name | Value to set |
|---|---|---|
| 31-30 | Reserved | 0x0 |
| 29-28 | ACCMOD | 0x1 if extended mode is set |
| 27-24 | DATLAT | Don't care |
| 23-20 | CLKDIV | Don't care |
| 19-16 | BUSTURN | Time between NEx high to NEx low (BUSTURN HCLK) |
| 15-8 | DATAST | Duration of the access second phase (DATAST HCLK cycles) for read accesses. |
| 7-4 | ADDHLD | Don't care |
| 3-0 | ADDSET[3:0] | Duration of the access first phase (ADDSET HCLK cycles) for read accesses. Minimum value for ADDSET is 0. |


Table 276. FMC_BWTRx bit fields
| Bit number | Bit name | Value to set |
|---|---|---|
| 31-30 | Reserved | 0x0 |
| 29-28 | ACCMOD | 0x1 if extended mode is set |
| 27-24 | DATLAT | Don't care |
| 23-20 | CLKDIV | Don't care |
| 19-16 | BUSTURN | Time between NEx high to NEx low (BUSTURN HCLK) |
| 15-8 | DATAST | Duration of the access second phase (DATAST HCLK cycles) for write accesses. |
| 7-4 | ADDHLD | Don't care |
| 3-0 | ADDSET[3:0] | Duration of the access first phase (ADDSET HCLK cycles) for write accesses. Minimum value for ADDSET is 0. |


Note: The FMC_BWTRx register is valid only if the extended mode is set (mode B), otherwise its content is don't care.

ST
RM0090 Rev 18
1625/1749

--- Page 66 ---

Flexible memory controller (FMC)
RM0090

**Mode C - NOR Flash - OE toggling**

**Figure 465. ModeC read access waveforms**

Memory transaction
A[25:0]
NADV
NEx
NOE
NWE
High
D[31:0]
ADDSET
HCLK cycles
data driven
by memory
DATAST
HCLK cycles
MS30459V1

**Figure 466. ModeC write access waveforms**

Memory transaction
A[25:0]
NADV
NEx
NOE
1HCLK
NWE
D[31:0]
ADDSET
HCLK cycles
data driven by FSMC
(DATAST + 1)
HCLK cycles
MS30460V1

The differences compared with mode1 are the toggling of NOE and the independent read and write timings.

1626/1749
RM0090 Rev 18

--- Page 67 ---

RM0090
Flexible memory controller (FMC)

Table 277. FMC_BCRx bit fields

| Bit No. | Bit name | Value to set |
|---|---|---|
| 31-21 | Reserved | 0x000 |
| 20 | CCLKEN | As needed |
| 19 | CBURSTRW | 0x0 (no effect in asynchronous mode) |
| 18:16 | Reserved | 0x0 (no effect in asynchronous mode) |
| 15 | ASYNCWAIT | Set to 1 if the memory supports this feature. Otherwise keep at 0. |
| 14 | EXTMOD | 0x1 |
| 13 | WAITEN | 0x0 (no effect in asynchronous mode) |
| 12 | WREN | As needed |
| 11 | WAITCFG | Don't care |
| 10 | WRAPMOD | 0x0 |
| 9 | WAITPOL | Meaningful only if bit 15 is 1 |
| 8 | BURSTEN | 0x0 |
| 7 | Reserved | 0x1 |
| 6 | FACCEN | 0x1 |
| 5-4 | MWID | As needed |
| 3-2 | MTYP[1:0] | 0x02 (NOR Flash memory) |
| 1 | MUXEN | 0x0 |
| 0 | MBKEN | 0x1 |


Table 278. FMC_BTRx bit fields

| Bit No. | Bit name | Value to set |
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
1627/1749

--- Page 68 ---

Flexible memory controller (FMC)
RM0090

Table 279. FMC_BWTRx bit fields

| Bit No. | Bit name | Value to set |
|---|---|---|
| 31:30 | Reserved | 0x0 |
| 29-28 | ACCMOD | 0x2 |
| 27-24 | DATLAT | Don't care |
| 23-20 | CLKDIV | Don't care |
| 19-16 | BUSTURN | Time between NEx high to NEx low (BUSTURN HCLK) |
| 15-8 | DATAST | Duration of the second access phase (DATAST HCLK cycles) for write accesses. |
| 7-4 | ADDHLD | Don't care |
| 3-0 | ADDSET[3:0] | Duration of the first access phase (ADDSET HCLK cycles) for write accesses. Minimum value for ADDSET is 0. |


Mode D - asynchronous access with extended address

Figure 467. ModeD read access waveforms

Memory transaction
A[25:0]
NADV
NEx
NOE
NWE
High
D[31:0]
data driven by memory
ADDSET HCLK cycles
ADDHLD HCLK cycles
DATAST HCLK cycles
MS30461V1

1628/1749
RM0090 Rev 18
ST

--- Page 69 ---

RM0090
Flexible memory controller (FMC)

**Figure 468. ModeD write access waveforms**

The differences with mode1 are the toggling of NOE that goes on toggling after NADV changes and the independent read and write timings.

**Table 280. FMC_BCRx bit fields**

| Bit No. | Bit name | Value to set |
|---|---|---|
| 31-21 | Reserved | 0x000 |
| 20 | CCLKEN | As needed |
| 19 | CBURSTRW | 0x0 (no effect in asynchronous mode) |
| 18:16 | CPSIZE | 0x0 (no effect in asynchronous mode) |
| 15 | ASYNCWAIT | Set to 1 if the memory supports this feature. Otherwise keep at 0. |
| 14 | EXTMOD | 0x1 |
| 13 | WAITEN | 0x0 (no effect in asynchronous mode) |
| 12 | WREN | As needed |
| 11 | WAITCFG | Don't care |
| 10 | WRAPMOD | 0x0 |
| 9 | WAITPOL | Meaningful only if bit 15 is 1 |
| 8 | BURSTEN | 0x0 |
| 7 | Reserved | 0x1 |
| 6 | FACCEN | Set according to memory support |
| 5-4 | MWID | As needed |


RM0090 Rev 18
1629/1749

--- Page 70 ---

**Flexible memory controller (FMC)**
RM0090

**Table 280. FMC_BCRx bit fields (continued)**

| Bit No. | Bit name | Value to set |
|---|---|---|
| 3-2 | MTYP[1:0] | As needed |
| 1 | MUXEN | 0x0 |
| 0 | MBKEN | 0x1 |


**Table 281. FMC_BTRx bit fields**

| Bit No. | Bit name | Value to set |
|---|---|---|
| 31:30 | Reserved | 0x0 |
| 29-28 | ACCMOD | 0x3 |
| 27-24 | DATLAT | Don't care |
| 23-20 | CLKDIV | Don't care |
| 19-16 | BUSTURN | Time between NEx high to NEx low (BUSTURN HCLK) |
| 15-8 | DATAST | Duration of the second access phase (DATAST HCLK cycles) for read accesses. |
| 7-4 | ADDHLD | Duration of the middle phase of the read access (ADDHLD HCLK cycles) |
| 3-0 | ADDSET[3:0] | Duration of the first access phase (ADDSET HCLK cycles) for read accesses. Minimum value for ADDSET is 1. |


**Table 282. FMC_BWTRx bit fields**

| Bit No. | Bit name | Value to set |
|---|---|---|
| 31:30 | Reserved | 0x0 |
| 29-28 | ACCMOD | 0x3 |
| 27-24 | DATLAT | Don't care |
| 23-20 | CLKDIV | Don't care |
| 19-16 | BUSTURN | Time between NEx high to NEx low (BUSTURN HCLK) |
| 15-8 | DATAST | Duration of the second access phase (DATAST + 1 HCLK cycles) for write accesses. |
| 7-4 | ADDHLD | Duration of the middle phase of the write access (ADDHLD HCLK cycles) |
| 3-0 | ADDSET[3:0] | Duration of the first access phase (ADDSET HCLK cycles) for write accesses. Minimum value for ADDSET is 1. |


1630/1749
RM0090 Rev 18
ST

--- Page 71 ---

RM0090
Flexible memory controller (FMC)

**Muxed mode - multiplexed asynchronous access to NOR Flash memory**

**Figure 469. Muxed read access waveforms**

Memory transaction
A[25:16]
NADV
NEx
NOE
NWE
High
AD[15:0]
Lower address
data driven by memory
ADDSET HCLK cycles
ADDHLD HCLK cycles
DATAST HCLK cycles
ai15568

**Figure 470. Muxed write access waveforms**

Memory transaction
A[25:16]
NADV
NEx
NOE
NWE
1HCLK
AD[15:0]
Lower address
data driven by FSMC
ADDSET HCLK cycles
ADDHLD HCLK cycles
(DATAST + 1) HCLK cycles
ai15569

The difference with mode D is the drive of the lower address byte(s) on the data bus.

ST
RM0090 Rev 18
1631/1749

--- Page 72 ---

Flexible memory controller (FMC)
RM0090

Table 283. FMC_BCRx bit fields

| Bit No. | Bit name | Value to set |
|---|---|---|
| 31-21 | Reserved | 0x000 |
| 20 | CCLKEN | As needed |
| 19 | CBURSTRW | 0x0 (no effect in asynchronous mode) |
| 18:16 | CPSIZE | 0x0 (no effect in asynchronous mode) |
| 15 | ASYNCWAIT | Set to 1 if the memory supports this feature. Otherwise keep at 0. |
| 14 | EXTMOD | 0x0 |
| 13 | WAITEN | 0x0 (no effect in asynchronous mode) |
| 12 | WREN | As needed |
| 11 | WAITCFG | Don't care |
| 10 | WRAPMOD | 0x0 |
| 9 | WAITPOL | Meaningful only if bit 15 is 1 |
| 8 | BURSTEN | 0x0 |
| 7 | Reserved | 0x1 |
| 6 | FACCEN | 0x1 |
| 5-4 | MWID | As needed |
| 3-2 | MTYP[1:0] | 0x2 (NOR Flash memory) |
| 1 | MUXEN | 0x1 |
| 0 | MBKEN | 0x1 |


Table 284. FMC_BTRx bit fields

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

If the asynchronous memory asserts the WAIT signal to indicate that it is not yet ready to accept or to provide data, the ASYNCWAIT bit has to be set in FMC_BCRx register.

1632/1749
RM0090 Rev 18
ST

--- Page 73 ---

RM0090
Flexible memory controller (FMC)

If the WAIT signal is active (high or low depending on the WAITPOL bit), the second access phase (Data setup phase), programmed by the DATAST bits, is extended until WAIT becomes inactive. Unlike the data setup phase, the first access phases (Address setup and Address hold phases), programmed by the ADDSET[3:0] and ADDHLD bits, are not WAIT sensitive and so they are not prolonged.

The data setup phase must be programmed so that WAIT can be detected 4 HCLK cycles before the end of the memory transaction. The following cases must be considered:

1. The memory asserts the WAIT signal aligned to NOE/NWE which toggles:
   DATAST ≥ (4 × HCLK) + max_wait_assertion_time

2. The memory asserts the WAIT signal aligned to NEx (or NOE/NWE not toggling):
   if
   max_wait_assertion_time > address_phase + hold_phase
   then:
   DATAST ≥ (4 × HCLK) + (max_wait_assertion_time − address_phase − hold_phase)
   otherwise
   DATAST ≥ 4 × HCLK

where max_wait_assertion_time is the maximum time taken by the memory to assert the WAIT signal once NEx/NOE/NWE is low.

Figure 471 and Figure 472 show the number of HCLK clock cycles that are added to the memory access phase after WAIT is released by the asynchronous memory (independently of the above cases).

**Figure 471. Asynchronous wait during a read access waveforms**

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
data driven by memory
4HCLK

1. NWAIT polarity depends on WAITPOL bit setting in FMC_BCRx register.

MS30463V2

ST
RM0090 Rev 18
1633/1749

--- Page 74 ---

Flexible memory controller (FMC)
RM0090

Figure 472. Asynchronous wait during a write access waveforms

[Image: Figure 472 showing timing waveforms for A[25:0], NEx, NWAIT, NWE, and D[15:0] during a memory transaction. The transaction is divided into an address phase and a data setup phase. The NWAIT signal has a "don't care" region. The data driven by FSMC is shown on D[15:0].]

1. NWAIT polarity depends on WAITPOL bit setting in FMC_BCRx register.

> [RM0090 ch.37 §37.5] NOR Flash/PSRAM controller

### 37.5.5 Synchronous transactions

### 37.5.5 Synchronous transactions

The memory clock, FMC_CLK, is a submultiple of HCLK. It depends on the value of CLKDIV and the MWID/ AHB data size, following the formula given below:

FMC_CLK divider ratio = max(CLKDIV + 1,MWID(AHB data size))

If MWID is 16 or 8 bits, the FMC_CLK divider ratio is always defined by the programmed CLKDIV value.

If MWID is 32 bits, the FMC_CLK divider ratio depends also on AHB data size.

Example:
* If CLKDIV=1, MWID=32 bits, AHB data size=8 bits, FMC_CLK=HCLK/4.
* If CLKDIV=1, MWID=16 bits, AHB data size=8 bits, FMC_CLK=HCLK/2.

NOR Flash memories specify a minimum time from NADV assertion to CLK high. To meet this constraint, the FMC does not issue the clock to the memory during the first internal clock cycle of the synchronous access (before NADV assertion). This guarantees that the rising edge of the memory clock occurs in the middle of the NADV low pulse.


> [RM0090 ch.36 §36.5] NOR Flash/PSRAM controller


### 36.5.6 NOR/PSRAM control registers

The NOR/PSRAM control registers have to be accessed by words (32 bits).

**SRAM/NOR-Flash chip-select control registers 1..4 (FSMC_BCR1..4)**

Address offset: 0xA000 0000 + 8 * (x - 1), x = 1..4
Reset value: 0x0000 30DB for Bank1 and 0x0000 30D2 for Bank 2 to 4

This register contains the control information of each memory bank, used for SRAMs, PSRAM and NOR Flash memories.

| 31 30 29 28 27 26 25 24 23 22 21 20 | 19 | 18 17 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 6 | 5 | 4 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved | CBURSTRW | CPSIZE[2:0] | ASYNCWAIT | EXTMOD | WAITEN | WREN | WAITCFG | WRAPMOD | WAITPOL | BURSTEN | Reserved | FACCEN | MWID[1:0] | MTYP[1:0] | MUXEN | MBKEN |
|  |  |  |  |  |  |  |  |  |  |  |  | rw |  |  |  | rw |


Bits 31: 20 Reserved, must be kept at reset value.

**Bit 19 CBURSTRW:** Write burst enable.
For Cellular RAM (PSRAM) memories, this bit enables the synchronous burst protocol during write operations. The enable bit for synchronous read accesses is the BURSTEN bit in the FSMC_BCRx register.
0: Write operations are always performed in asynchronous mode
1: Write operations are performed in synchronous mode.

**Bits 18: 16 CPSIZE[2:0]:** CRAM page size.
These are used for Cellular RAM 1.5 which does not allow burst access to cross the address boundaries between pages. When these bits are configured, the FSMC controller splits automatically the burst access when the memory page size is reached (refer to memory datasheet for page size).
000: No burst split when crossing page boundary (default after reset)
001: 128 bytes
010: 256 bytes
011: 512 bytes
100: 1024 bytes
Others: reserved.

**Bit 15 ASYNCWAIT:** Wait signal during asynchronous transfers
This bit enables/disables the FSMC to use the wait signal even during an asynchronous protocol.
0: NWAIT signal is not taken into account when running an asynchronous protocol (default after reset)
1: NWAIT signal is taken into account when running an asynchronous protocol

ST
RM0090 Rev 18
1577/1749

--- Page 18 ---

**Flexible static memory controller (FSMC)**
RM0090

**Bit 14 EXTMOD:** Extended mode enable.
This bit enables the FSMC to program the write timings for non-multiplexed asynchronous accesses inside the FSMC_BWTR register, thus resulting in different timings for read and write operations.
0: values inside FSMC_BWTR register are not taken into account (default after reset)
1: values inside FSMC_BWTR register are taken into account
*Note: When the extended mode is disabled, the FSMC can operate in Mode1 or Mode2 as follows:*
- Mode 1 is the default mode when the SRAM/PSRAM memory type is selected (MTYP [0:1]=0x0 or 0x01)
- Mode 2 is the default mode when the NOR memory type is selected (MTYP [0:1]= 0x10).

**Bit 13 WAITEN:** Wait enable bit.
This bit enables/disables wait-state insertion via the NWAIT signal when accessing the Flash memory in synchronous mode.
0: NWAIT signal is disabled (its level not taken into account, no wait state inserted after the programmed Flash latency period)
1: NWAIT signal is enabled (its level is taken into account after the programmed Flash latency period to insert wait states if asserted) (default after reset)

**Bit 12 WREN:** Write enable bit.
This bit indicates whether write operations are enabled/disabled in the bank by the FSMC:
0: Write operations are disabled in the bank by the FSMC, an AHB error is reported,
1: Write operations are enabled for the bank by the FSMC (default after reset).

**Bit 11 WAITCFG:** Wait timing configuration.
The NWAIT signal indicates whether the data from the memory are valid or if a wait state must be inserted when accessing the Flash memory in synchronous mode. This configuration bit determines if NWAIT is asserted by the memory one clock cycle before the wait state or during the wait state:
0: NWAIT signal is active one data cycle before wait state (default after reset),
1: NWAIT signal is active during wait state (not used for PRAM).

**Bit 10 WRAPMOD:** Wrapped burst mode support.
Defines whether the controller will or not split an AHB burst wrap access into two linear accesses. Valid only when accessing memories in burst mode
0: Direct wrapped burst is not enabled (default after reset),
1: Direct wrapped burst is enabled.
*Note: This bit has no effect as the CPU and DMA cannot generate wrapping burst transfers.*

**Bit 9 WAITPOL:** Wait signal polarity bit.
Defines the polarity of the wait signal from memory. Valid only when accessing the memory in burst mode:
0: NWAIT active low (default after reset),
1: NWAIT active high.

**Bit 8 BURSTEN:** Burst enable bit.
This bit enables/disables synchronous accesses during read operations. It is valid only for synchronous memories operating in burst mode:
0: Burst mode disabled (default after reset). Read accesses are performed in asynchronous mode.
1: Burst mode enable. Read accesses are performed in synchronous mode.

1578/1749
RM0090 Rev 18
ST

--- Page 19 ---

RM0090
Flexible static memory controller (FSMC)

Bit 7 Reserved, must be kept at reset value.

Bit 6 **FACCEN**: Flash access enable
Enables NOR Flash memory access operations.
0: Corresponding NOR Flash memory access is disabled
1: Corresponding NOR Flash memory access is enabled (default after reset)

Bits 5:4 **MWID[1:0]**: Memory databus width.
Defines the external memory device width, valid for all type of memories.
00: 8 bits,
01: 16 bits (default after reset),
10: reserved, do not use,
11: reserved, do not use.

Bits 3:2 **MTYP[1:0]**: Memory type.
Defines the type of external memory attached to the corresponding memory bank:
00: SRAM (default after reset for Bank 2...4)
01: PSRAM (CRAM)
10: NOR Flash/OneNAND Flash (default after reset for Bank 1)
11: reserved

Bit 1 **MUXEN**: Address/data multiplexing enable bit.
When this bit is set, the address and data values are multiplexed on the databus, valid only with NOR and PSRAM memories:
0: Address/Data nonmultiplexed
1: Address/Data multiplexed on databus (default after reset)

Bit 0 **MBKEN**: Memory bank enable bit.
Enables the memory bank. After reset Bank1 is enabled, all others are disabled.
Accessing a disabled bank causes an ERROR on AHB bus.
0: Corresponding memory bank is disabled
1: Corresponding memory bank is enabled

ST
RM0090 Rev 18
1579/1749

--- Page 20 ---

**Flexible static memory controller (FSMC)**
RM0090

**SRAM/NOR-Flash chip-select timing registers 1..4 (FSMC_BTR1..4)**

Address offset: 0xA000 0000 + 0x04 + 8 * (x − 1), x = 1..4
Reset value: 0x0FFF FFFF

FSMC_BTRx bits are written by software to add a delay at the end of a read /write transaction. This delay allows matching the minimum time between consecutive transactions (tEHEL from NEx high to FSMC_NEx low) and the maximum time required by the memory to free the data bus after a read access (tEHQZ).

This register contains the control information of each memory bank, used for SRAMs, PSRAM and NOR Flash memories. If the EXTMOD bit is set in the FSMC_BCRx register, then this register is partitioned for write and read access, that is, 2 registers are available: one to configure read accesses (this register) and one to configure write accesses (FSMC_BWTRx registers).

| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved | ACCMOD[1:0] | DATLAT[3:0] | CLKDIV[3:0] | BUSTURN[3:0] | DATAST[7:0] | ADDHLD[3:0] | ADDSET[3:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


**Bits 31:30** Reserved, must be kept at reset value.

**Bits 29:28 ACCMOD[1:0]: Access mode**
Specifies the asynchronous access modes as shown in the timing diagrams. These bits are taken into account only when the EXTMOD bit in the FSMC_BCRx register is 1.
00: access mode A
01: access mode B
10: access mode C
11: access mode D

**Bits 27:24 DATLAT[3:0]: Data latency for synchronous memory** (see note below bit description table)
For synchronous accesses with read/write burst mode enabled (BURSTEN / CBURSTRW bits set), this field defines the number of memory clock cycles (+2) to issue to the memory before reading/writing the first data. This timing parameter is not expressed in HCLK periods, but in FSMC_CLK periods. For asynchronous accesses, this value is don't care.
0000: Data latency of 2 CLK clock cycles for first burst access
1111: Data latency of 17 CLK clock cycles for first burst access (default value after reset)

**Bits 23:20 CLKDIV[3:0]: Clock divide ratio** (for FSMC_CLK signal)
Defines the period of FSMC_CLK clock output signal, expressed in number of HCLK cycles:
0000: Reserved
0001: FSMC_CLK period = 2 × HCLK periods
0010: FSMC_CLK period = 3 × HCLK periods
1111: FSMC_CLK period = 16 × HCLK periods (default value after reset)
In asynchronous NOR Flash, SRAM or PSRAM accesses, this value is don't care.

1580/1749
RM0090 Rev 18

--- Page 21 ---

RM0090
Flexible static memory controller (FSMC)

Bits 19:16 **BUSTURN[3:0]**: Bus turnaround phase duration

These bits are written by software to add a delay at the end of a write-to-read (and read-to write) transaction. The programmed bus turnaround delay is inserted between an asynchronous read (muxed or D mode) or a write transaction and any other asynchronous/synchronous read or write to/from a static bank (for a read operation, the bank can be the same or a different one; for a write operation, the bank can be different except in muxed or D mode).

In some cases, the bus turnaround delay is fixed, whatever the programmed BUSTURN values:

- No bus turnaround delay is inserted between two consecutive asynchronous write transfers to the same static memory bank except in muxed and D mode.
- A bus turnaround delay of 1 FSMC clock cycle is inserted between:
  - Two consecutive asynchronous read transfers to the same static memory bank except for muxed and D modes.
  - An asynchronous read to an asynchronous or synchronous write to any static bank or dynamic bank except for muxed and D modes.
  - An asynchronous (modes 1, 2, A, B or C) read and a read operation from another static bank.
- A bus turnaround delay of 2 FSMC clock cycles is inserted between:
  - Two consecutive synchronous write accesses (in burst or single mode) to the same bank
  - A synchronous write (burst or single) access and an asynchronous write or read transfer to or from static memory bank (the bank can be the same or different in case of a read operation).
  - Two consecutive synchronous read accesses (in burst or single mode) followed by a any synchronous/asynchronous read or write from/to another static memory bank.
- A bus turnaround delay of 3 FSMC clock cycles is inserted between:
  - Two consecutive synchronous write operations (in burst or single mode) to different static banks.
  - A synchronous write access (in burst or single mode) and a synchronous read access from the same or to a different bank.

0000: BUSTURN phase duration = 0 HCLK clock cycle added
...
1111: BUSTURN phase duration = 15 × HCLK clock cycles (default value after reset)

ST
RM0090 Rev 18
1581/1749

--- Page 22 ---

**Flexible static memory controller (FSMC)**
RM0090

**Bits 15:8 DATAST[7:0]: Data-phase duration**
These bits are written by software to define the duration of the data phase (refer to Figure 436 to Figure 448), used in asynchronous accesses:
0000 0000: Reserved
0000 0001: DATAST phase duration = 1 × HCLK clock cycles
0000 0010: DATAST phase duration = 2 × HCLK clock cycles
...
1111 1111: DATAST phase duration = 255 × HCLK clock cycles (default value after reset)
For each memory type and access mode data-phase duration, refer to the respective figure (Figure 436 to Figure 448).
Example: Mode1, write access, DATAST=1: Data-phase duration= DATAST+1 = 2 HCLK clock cycles.
*Note: In synchronous accesses, this value is don't care.*

**Bits 7:4 ADDHLD[3:0]: Address-hold phase duration**
These bits are written by software to define the duration of the *address hold* phase (refer to Figure 445 to Figure 448), used in mode D and multiplexed accesses:
0000: Reserved
0001: ADDHLD phase duration = 1 × HCLK clock cycle
0010: ADDHLD phase duration = 2 × HCLK clock cycle
...
1111: ADDHLD phase duration = 15 × HCLK clock cycles (default value after reset)
For each access mode address-hold phase duration, refer to the respective figure (Figure 445 to Figure 448).
*Note: In synchronous accesses, this value is not used, the address hold phase is always 1 memory clock period duration.*

**Bits 3:0 ADDSET[3:0]: Address setup phase duration**
These bits are written by software to define the duration of the *address setup* phase (refer to Figure 436 to Figure 448), used in SRAMs, ROMs and asynchronous NOR Flash and PSRAM accesses:
0000: ADDSET phase duration = 0 × HCLK clock cycle
...
1111: ADDSET phase duration = 15 × HCLK clock cycles (default value after reset)
For each access mode address setup phase duration, refer to the respective figure (refer to Figure 436 to Figure 448).
*Note: In synchronous NOR Flash and PSRAM accesses, this value is don't care.*

**Note:**
PSRAMs (CRAMs) have a variable latency due to internal refresh. Therefore these memories issue the NWAIT signal during the whole latency phase to prolong the latency as needed.
With PSRAMs (CRAMs) the DATLAT field must be set to 0, so that the FSMC exits its latency phase soon and starts sampling NWAIT from memory, then starts to read or write when the memory is ready.
This method can be used also with the latest generation of synchronous Flash memories that issue the NWAIT signal, unlike older Flash memories (check the datasheet of the specific Flash memory being used).

1582/1749
RM0090 Rev 18
ST

--- Page 23 ---

RM0090
Flexible static memory controller (FSMC)

**SRAM/NOR-Flash write timing registers 1..4 (FSMC_BWTR1..4)**

Address offset: 0xA000 0000 + 0x104 + 8 * (x − 1), x = 1...4
Reset value: 0x0FFF FFFF

This register contains the control information of each memory bank, used for SRAMs, PSRAMs and NOR Flash memories. This register is active for write asynchronous access only when the EXTMOD bit is set in the FSMC_BCRx register.

| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Res. | ACCMOD[2:0] | Reserved | BUSTURN[3:0] | DATAST[7:0] | ADDHLD[3:0] | ADDSET[3:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


**Bits 31:30** Reserved, must be kept at reset value.

**Bits 29:28 ACCMOD[2:0]:** Access mode.
Specifies the asynchronous access modes as shown in the next timing diagrams. These bits are taken into account only when the EXTMOD bit in the FSMC_BCRx register is 1.
00: access mode A
01: access mode B
10: access mode C
11: access mode D

**Bits 27:20** Reserved, must be kept at reset value.

**Bits 19:16 BUSTURN[3:0]:** Bus turnaround phase duration
The programmed bus turnaround delay is inserted between a an asynchronous write transfer and any other asynchronous/synchronous read or write transfer to/from a static bank (for a read operation, the bank can be the same or a different one; for a write operation, the bank can be different except in r muxed or D mode).
In some cases, the bus turnaround delay is fixed, whatever the programmed BUSTURN values:
- No bus turnaround delay is inserted between two consecutive asynchronous write transfers to the same static memory bank except in muxed and D mode.
- A bus turnaround delay of 2 FSMC clock cycles is inserted between:
  - Two consecutive synchronous write accesses (in burst or single mode) to the same bank.
  - A synchronous write transfer (in burst or single mode) and an asynchronous write or read transfer to/from static a memory bank.
- A bus turnaround delay of 3 FSMC clock cycles is inserted between:
  - Two consecutive synchronous write accesses (in burst or single mode) to different static banks.
  - A synchronous write transfer (in burst or single mode) and a synchronous read from the same or from a different bank.
0000: BUSTURN phase duration = 0 HCLK clock cycle added
...
1111: BUSTURN phase duration = 15 HCLK clock cycles added (default value after reset)

ST
RM0090 Rev 18
1583/1749

--- Page 24 ---

**Flexible static memory controller (FSMC)** RM0090

Bits 15:8 **DATAST[7:0]**: Data-phase duration.
These bits are written by software to define the duration of the data phase (refer to Figure 436 to Figure 448), used in asynchronous SRAM, PSRAM and NOR Flash memory accesses:
0000 0000: Reserved
0000 0001: DATAST phase duration = 1 × HCLK clock cycles
0000 0010: DATAST phase duration = 2 × HCLK clock cycles
...
1111 1111: DATAST phase duration = 255 × HCLK clock cycles (default value after reset)
*Note: In synchronous accesses, this value is don't care.*

Bits 7:4 **ADDHLD[3:0]**: Address-hold phase duration.
These bits are written by software to define the duration of the *address hold* phase (refer to Figure 445 to Figure 448), used in asynchronous multiplexed accesses:
0000: Reserved
0001: ADDHLD phase duration = 1 × HCLK clock cycle
0010: ADDHLD phase duration = 2 × HCLK clock cycle
...
1111: ADDHLD phase duration = 15 × HCLK clock cycles (default value after reset)
*Note: In synchronous NOR Flash accesses, this value is not used, the address hold phase is always 1 Flash clock period duration.*

Bits 3:0 **ADDSET[3:0]**: Address setup phase duration.
These bits are written by software to define the duration of the *address setup* phase in HCLK cycles (refer to Figure 445 to Figure 448), used in asynchronous accessed:
0000: ADDSET phase duration = 0 × HCLK clock cycle
...
1111: ADDSET phase duration = 15 × HCLK clock cycles (default value after reset)
*Note: In synchronous NOR Flash and PSRAM accesses, this value is don't care.*


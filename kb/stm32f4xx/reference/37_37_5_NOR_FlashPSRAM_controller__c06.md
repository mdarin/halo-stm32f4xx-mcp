> [RM0090 ch.37 §37.5] NOR Flash/PSRAM controller


### 37.5.6 NOR/PSRAM controller registers

**SRAM/NOR-Flash chip-select control registers 1..4 (FMC_BCR1..4)**

Address offset: 8 * (x − 1), x = 1..4
Reset value: 0x0000 30DB for Bank1 and 0x0000 30D2 for Bank 2 to 4

This register contains the control information of each memory bank, used for SRAMs, PSRAM and NOR Flash memories.

| 31 30 29 28 27 26 25 24 23 22 21 | 20 | 19 | 18 17 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 6 | 5 | 4 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved | CCLKEN | CBURSTRW | CPSIZE[2:0] | ASYCOWAIT | EXTMOD | WAITEN | WREN | WAITCFG | WRAPMOD | WAITPOL | BURSTEN | Reserved | FACCEN | MWID[1:0] | MTYP[1:0] | MUXEN | MBKEN |
| rw |  |  |  | rw |  |  | rw |  |  |  |  |  |  |  |  |  |  |


Bits 31: 21 Reserved, must be kept at reset value

**Bit 20 CCLKEN:** Continuous Clock Enable.
This bit enables the FMC_CLK clock output to external memory devices.
0: The FMC_CLK is only generated during the synchronous memory access (read/write transaction). The FMC_CLK clock ratio is specified by the programmed CLKDIV value in the FMC_BCRx register (default after reset) .
1: The FMC_CLK is generated continuously during asynchronous and synchronous access. The FMC_CLK clock is activated when the CCLKEN is set.

*Note: The CCLKEN bit of the FMC_BCR2..4 registers is don't care. It is only enabled through the FMC_BCR1 register. Bank 1 must be configured in synchronous mode to generate the FMC_CLK continuous clock.*

*Note: If CCLKEN bit is set, the FMC_CLK clock ratio is specified by CLKDIV value in the FMC_BTR1 register. CLKDIV in FMC_BWTR1 is don't care.*

*Note: If the synchronous mode is used and CCLKEN bit is set, the synchronous memories connected to other banks than Bank 1 are clocked by the same clock (the CLKDIV value in the FMC_BTR2..4 and FMC_BWTR2..4 registers for other banks has no effect.)*

**Bit 19 CBURSTRW:** Write burst enable.
For PSRAM (CRAM) operating in burst mode, the bit enables synchronous accesses during write operations. The enable bit for synchronous read accesses is the BURSTEN bit in the FMC_BCRx register.
0: Write operations are always performed in asynchronous mode
1: Write operations are performed in synchronous mode.

**Bits 18:16 CPSIZE[2:0]:** CRAM page size.
These are used for Cellular RAM 1.5 which does not allow burst access to cross the address boundaries between pages. When these bits are configured, the FMC controller splits automatically the burst access when the memory page size is reached (refer to memory datasheet for page size).
000: No burst split when crossing page boundary (default after reset)
001: 128 bytes
010: 256 bytes
011: 512 bytes
100: 1024 bytes
Others: reserved

1640/1749
RM0090 Rev 18
STI

--- Page 81 ---

RM0090
Flexible memory controller (FMC)

**Bit 15 ASYNCWAIT**: Wait signal during asynchronous transfers
This bit enables/disables the FMC to use the wait signal even during an asynchronous protocol.
0: NWAIT signal is not taken in to account when running an asynchronous protocol (default after reset)
1: NWAIT signal is taken in to account when running an asynchronous protocol

**Bit 14 EXTMOD**: Extended mode enable.
This bit enables the FMC to program the write timings for non-multiplexed asynchronous accesses inside the FMC\_BWTR register, thus resulting in different timings for read and write operations.
0: values inside FMC\_BWTR register are not taken into account (default after reset)
1: values inside FMC\_BWTR register are taken into account
*Note: When the extended mode is disabled, the FMC can operate in Mode1 or Mode2 as follows:*
- Mode 1 is the default mode when the SRAM/PSRAM memory type is selected (MTYP[1:0] =0x0 or 0x01)
- Mode 2 is the default mode when the NOR memory type is selected (MTYP[1:0] = 0x10).

**Bit 13 WAITEN**: Wait enable bit.
This bit enables/disables wait-state insertion via the NWAIT signal when accessing the memory in synchronous mode.
0: NWAIT signal is disabled (its level not taken into account, no wait state inserted after the programmed Flash latency period)
1: NWAIT signal is enabled (its level is taken into account after the programmed latency period to insert wait states if asserted) (default after reset)

**Bit 12 WREN**: Write enable bit.
This bit indicates whether write operations are enabled/disabled in the bank by the FMC:
0: Write operations are disabled in the bank by the FMC, an AHB error is reported,
1: Write operations are enabled for the bank by the FMC (default after reset).

**Bit 11 WAITCFG**: Wait timing configuration.
The NWAIT signal indicates whether the data from the memory are valid or if a wait state must be inserted when accessing the memory in synchronous mode. This configuration bit determines if NWAIT is asserted by the memory one clock cycle before the wait state or during the wait state:
0: NWAIT signal is active one data cycle before wait state (default after reset),
1: NWAIT signal is active during wait state (not used for PSRAM).

**Bit 10 WRAPMOD**: Wrapped burst mode support.
Defines whether the controller will or not split an AHB burst wrap access into two linear accesses. Valid only when accessing memories in burst mode
0: Direct wrapped burst is not enabled (default after reset),
1: Direct wrapped burst is enabled.
*Note: This bit has no effect as the CPU and DMA cannot generate wrapping burst transfers.*

**Bit 9 WAITPOL**: Wait signal polarity bit.
Defines the polarity of the wait signal from memory used for either in synchronous or asynchronous mode:
0: NWAIT active low (default after reset),
1: NWAIT active high.

**Bit 8 BURSTEN**: Burst enable bit.
This bit enables/disables synchronous accesses during read operations. It is valid only for synchronous memories operating in burst mode:
0: Burst mode disabled (default after reset). Read accesses are performed in asynchronous mode.
1: Burst mode enable. Read accesses are performed in synchronous mode.

**Bit 7** Reserved, must be kept at reset value

ST
RM0090 Rev 18
1641/1749

--- Page 82 ---

**Flexible memory controller (FMC)**
RM0090

Bit 6 **FACCEN**: Flash access enable
Enables NOR Flash memory access operations.
0: Corresponding NOR Flash memory access is disabled
1: Corresponding NOR Flash memory access is enabled (default after reset)

Bits 5:4 **MWID[1:0]**: Memory data bus width.
Defines the external memory device width, valid for all type of memories.
00: 8 bits,
01: 16 bits (default after reset),
10: 32 bits,
11: reserved, do not use.

Bits 3:2 **MTYP[1:0]**: Memory type.
Defines the type of external memory attached to the corresponding memory bank:
00: SRAM (default after reset for Bank 2...4)
01: PSRAM (CRAM)
10: NOR Flash/OneNAND Flash (default after reset for Bank 1)
11: reserved

Bit 1 **MUXEN**: Address/data multiplexing enable bit.
When this bit is set, the address and data values are multiplexed on the data bus, valid only with NOR and PSRAM memories:
0: Address/Data nonmultiplexed
1: Address/Data multiplexed on databus (default after reset)

Bit 0 **MBKEN**: Memory bank enable bit.
Enables the memory bank. After reset Bank1 is enabled, all others are disabled. Accessing a disabled bank causes an ERROR on AHB bus.
0: Corresponding memory bank is disabled
1: Corresponding memory bank is enabled

**SRAM/NOR-Flash chip-select timing registers 1..4 (FMC_BTR1..4)**

Address offset: 0x04 + 8 * (x - 1), x = 1..4
Reset value: 0x0FFF FFFF
Reset value: 0x0FFF FFFF

FMC_BTRx bits are written by software to add a delay at the end of a read /write transaction. This delay allows matching the minimum time between consecutive transactions (t<sub>EHEL</sub> from NEx high to FMC_NEx low) and the maximum time required by the memory to free the data bus after a read access (t<sub>EHQZ</sub>).

This register contains the control information of each memory bank, used for SRAMs, PSRAM and NOR Flash memories. If the EXTMOD bit is set in the FMC_BCRx register, then this register is partitioned for write and read access, that is, 2 registers are available: one to configure read accesses (this register) and one to configure write accesses (FMC_BWTRx registers).

1642/1749
RM0090 Rev 18
ST

--- Page 83 ---

**RM0090**
**Flexible memory controller (FMC)**

| 31 30 | 29 28 | 27 | 26 25 24 | 23 | 22 21 20 | 19 | 18 17 16 | 15 14 13 12 11 10 | 9 | 8 7 6 | 5 | 4 3 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved | ACCMOD[1:0] | DTLAT[3:0] | CLKDIV[3:0] | BUSTURN[3:0] | DATASET[7:0] | ADDHLD[3:0] | ADDSET[3:0] |  |  |  |  |  |  |  |
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


**Bits 31:30** Reserved, must be kept at reset value

**Bits 29:28 ACCMOD[1:0]: Access mode**
Specifies the asynchronous access modes as shown in the timing diagrams. These bits are taken into account only when the EXTMOD bit in the FMC_BCRx register is 1.
00: access mode A
01: access mode B
10: access mode C
11: access mode D

**Bits 27:24 DTLAT[3:0]: Data latency for synchronous memory** (see note below bit description table)
For synchronous accesses with read/write burst mode enabled (BURSTEN / CBURSTRW bits set), this field defines the number of memory clock cycles (+2) to issue to the memory before reading/writing the first data. This timing parameter is not expressed in HCLK periods, but in FMC_CLK periods. For asynchronous accesses, this value is don't care.
0000: Data latency of 2 CLK clock cycles for first burst access
1111: Data latency of 17 CLK clock cycles for first burst access (default value after reset)

**Bits 23:20 CLKDIV[3:0]: Clock divide ratio** (for FMC_CLK signal)
Defines the period of FMC_CLK clock output signal, expressed in number of HCLK cycles:
0000: Reserved
0001: FMC_CLK period = 2 × HCLK periods
0010: FMC_CLK period = 3 × HCLK periods
1111: FMC_CLK period = 16 × HCLK periods (default value after reset)
In asynchronous NOR Flash, SRAM or PSRAM accesses, this value is don't care.
*Note: Refer to section 37.5.5: Synchronous transactions for FMC_CLK divider ratio formula)*

ST
RM0090 Rev 18
1643/1749

--- Page 84 ---

**Flexible memory controller (FMC)**
RM0090

Bits 19:16 **BUSTURN[3:0]**: Bus turnaround phase duration

These bits are written by software to add a delay at the end of a write-to-read (and read-to-write) transaction. This delay allows to match the minimum time between consecutive transactions (tEH from NEx high to NEx low) and the maximum time needed by the memory to free the data bus after a read access (tEHQZ). The programmed bus turnaround delay is inserted between an asynchronous read (muxed or mode D) or write transaction and any other asynchronous /synchronous read or write to or from a static bank. The bank can be the same or different in case of read, in case of write the bank can be different except for muxed or mode D.

In some cases, whatever the programmed Bustrun values, the bus turnaround delay is fixed as follows:

*   The bus turnaround delay is not inserted between two consecutive asynchronous write transfers to the same static memory bank except for modes muxed and D.
*   There is a bus turnaround delay of 1 FMC clock cycle between:
    *   Two consecutive asynchronous read transfers to the same static memory bank except for modes muxed and D.
    *   An asynchronous read to an asynchronous or synchronous write to any static bank or dynamic bank except for modes muxed and D.
    *   An asynchronous (modes 1, 2, A, B or C) read and a read from another static bank.
*   There is a bus turnaround delay of 2 FMC clock cycle between:
    *   Two consecutive synchronous writes (burst or single) to the same bank.
    *   A synchronous write (burst or single) access and an asynchronous write or read transfer to or from static memory bank (the bank can be the same or different for the case of read.
    *   Two consecutive synchronous reads (burst or single) followed by any synchronous/asynchronous read or write from/to another static memory bank.
*   There is a bus turnaround delay of 3 FMC clock cycle between:
    *   Two consecutive synchronous writes (burst or single) to different static bank.
    *   A synchronous write (burst or single) access and a synchronous read from the same or a different bank.

0000: BUSTURN phase duration = 0 HCLK clock cycle added
...
1111: BUSTURN phase duration = 15 x HCLK clock cycles added (default value after reset)

1644/1749
RM0090 Rev 18

--- Page 85 ---

**RM0090**
**Flexible memory controller (FMC)**

**Bits 15:8 DATAST[7:0]: Data-phase duration**
These bits are written by software to define the duration of the data phase (refer to *Figure 458* to *Figure 470*), used in asynchronous accesses:
0000 0000: Reserved
0000 0001: DATAST phase duration = 1 × HCLK clock cycles
0000 0010: DATAST phase duration = 2 × HCLK clock cycles
...
1111 1111: DATAST phase duration = 255 × HCLK clock cycles (default value after reset)
For each memory type and access mode data-phase duration, please refer to the respective figure (*Figure 458* to *Figure 470*).
Example: Mode1, write access, DATAST=1: Data-phase duration= DATAST+1 = 2 HCLK clock cycles.
*Note: In synchronous accesses, this value is don't care.*

**Bits 7:4 ADDHLD[3:0]: Address-hold phase duration**
These bits are written by software to define the duration of the *address hold phase* (refer to *Figure 467* to *Figure 470*), used in mode D or multiplexed accesses:
0000: Reserved
0001: ADDHLD phase duration =1 × HCLK clock cycle
0010: ADDHLD phase duration = 2 × HCLK clock cycle
...
1111: ADDHLD phase duration = 15 × HCLK clock cycles (default value after reset)
For each access mode address-hold duration, please refer to the respective figure (*Figure 467* to *Figure 470*).
*Note: In synchronous accesses, this value is not used, the address hold phase is always 1 memory clock period duration.*

**Bits 3:0 ADDSET[3:0]: Address setup phase duration**
These bits are written by software to define the duration of the *address setup phase* (refer to *Figure 458* to *Figure 470*), used in SRAMs, ROMs and asynchronous NOR Flash and PSRAM accesses:
0000: ADDSET phase duration = 0 × HCLK clock cycle
...
1111: ADDSET phase duration = 15 × HCLK clock cycles (default value after reset)
For each access mode address setup phase duration, please refer to the respective figure (refer to *Figure 458* to *Figure 470*).
*Note: In synchronous accesses, this value is don't care.*
In Muxed mode or Mode D, the minimum value for ADDSET is 1.

**Note:**
PSRAMs (CRAMs) have a variable latency due to internal refresh. Therefore these memories issue the NWAIT signal during the whole latency phase to prolong the latency as needed.
With PSRAMs (CRAMs) the filled DATLAT must be set to 0, so that the FMC exits its latency phase soon and starts sampling NWAIT from memory, then starts to read or write when the memory is ready.
This method can be used also with the latest generation of synchronous Flash memories that issue the NWAIT signal, unlike older Flash memories (check the datasheet of the specific Flash memory being used).

ST
RM0090 Rev 18
1645/1749

--- Page 86 ---

**Flexible memory controller (FMC)**
RM0090

**SRAM/NOR-Flash write timing registers 1..4 (FMC_BWTR1..4)**

Address offset: 0x104 + 8 * (x - 1), x = 1...4

Reset value: 0x0FFF FFFF

This register contains the control information of each memory bank. It is used for SRAMs, PSRAMs and NOR Flash memories. When the EXTMOD bit is set in the FMC_BCRx register, then this register is active for write access.

| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved | ACCMOD[1:0] | Reserved | BUSTURN[3:0] | DATAST[7:0] | ADDHLD[3:0] | ADDSET[3:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


**Bits 31:30** Reserved, must be kept at reset value

**Bits 29:28** **ACCMOD[1:0]:** Access mode.
Specifies the asynchronous access modes as shown in the next timing diagrams. These bits are taken into account only when the EXTMOD bit in the FMC_BCRx register is 1.
00: access mode A
01: access mode B
10: access mode C
11: access mode D

**Bits 27:20** Reserved, must be kept at reset value

**Bits 19:16** **BUSTURN[3:0]:** Bus turnaround phase duration
The programmed bus turnaround delay is inserted between an asynchronous write transfer and any other asynchronous /synchronous read or write transfer to or from a static bank. The bank can be the same or different in case of read, in case of write the bank can be different expect for muxed or mode D.

In some cases, whatever the programmed Bustrun values, the bus turnaround delay is fixed as follows:
* The bus turnaround delay is not inserted between two consecutive asynchronous write transfers to the same static memory bank except for modes muxed and D.
* There is a bus turnaround delay of 2 FMC clock cycle between:
  –Two consecutive synchronous writes (burst or single) to the same bank.
  –A synchronous write (burst or single) transfer and an asynchronous write or read transfer to or from static memory bank.
* There is a bus turnaround delay of 3 FMC clock cycle between:
  –Two consecutive synchronous writes (burst or single) to different static bank.
  –A synchronous write (burst or single) transfer and a synchronous read from the same or a different bank.

0000: BUSTURN phase duration = 0 HCLK clock cycle added
...
1111: BUSTURN phase duration = 15 HCLK clock cycles added (default value after reset)

1646/1749 RM0090 Rev 18

--- Page 87 ---

RM0090
Flexible memory controller (FMC)

Bits 15:8 **DATAST[3:0]**: Data-phase duration.
These bits are written by software to define the duration of the data phase (refer to Figure 458 to Figure 470), used in asynchronous SRAM, PSRAM and NOR Flash memory accesses:
0000 0000: Reserved
0000 0001: DATAST phase duration = 1 × HCLK clock cycles
0000 0010: DATAST phase duration = 2 × HCLK clock cycles
...
1111 1111: DATAST phase duration = 255 × HCLK clock cycles (default value after reset)

Bits 7:4 **ADDHLD[3:0]**: Address-hold phase duration.
These bits are written by software to define the duration of the *address hold* phase (refer to Figure 467 to Figure 470), used in asynchronous multiplexed accesses:
0000: Reserved
0001: ADDHLD phase duration = 1 × HCLK clock cycle
0010: ADDHLD phase duration = 2 × HCLK clock cycle
...
1111: ADDHLD phase duration = 15 × HCLK clock cycles (default value after reset)
*Note: In synchronous NOR Flash accesses, this value is not used, the address hold phase is always 1 Flash clock period duration.*

Bits 3:0 **ADDSET[3:0]**: Address setup phase duration.
These bits are written by software to define the duration of the *address setup* phase in HCLK cycles (refer to Figure 467 to Figure 470), used in asynchronous accesses:
0000: ADDSET phase duration = 0 × HCLK clock cycle
...
1111: ADDSET phase duration = 15 × HCLK clock cycles (default value after reset)
*Note: In synchronous NOR Flash and PSRAM accesses, this value is not used, the address setup phase is always 1 Flash clock period duration. In muxed mode, the minimum ADDSET value is 1.*


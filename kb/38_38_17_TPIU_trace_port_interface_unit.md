### 38.17 TPIU (trace port interface unit)

#### 38.17.1 Introduction

The TPIU acts as a bridge between the on-chip trace data from the ITM and the ETM.
The output data stream encapsulates the trace source ID, that is then captured by a *trace port analyzer* (TPA).
The core embeds a simple TPIU, especially designed for low-cost debug (consisting of a special version of the CoreSight TPIU).

**Figure 488. TPIU block diagram**

CLK domain
TRACECLKIN domain
TPIU
ETM
ITM
Asynchronous FIFO
Asynchronous FIFO
TPIU formatter
Trace out (serializer)
TRACECLKIN
TRACECK
TRACEDATA [3:0]
TRACESWO
External PPB bus
ai17114

ST
RM0090 Rev 18
1707/1749

--- Page 148 ---

Debug support (DBG)
RM0090

### 38.17.2 TRACE pin assignment

*   Asynchronous mode
    The asynchronous mode requires 1 extra pin and is available on all packages. It is only available if using Serial Wire mode (not in JTAG mode).

Table 310. Asynchronous TRACE pin assignment

| TPUI pin name | Trace synchronous mode | STM32F4xx pin assignment |  |
|---|---|---|---|
| Type | Description |  |  |
| TRACESWO | O | TRACE Async Data Output | PB3 |


*   Synchronous mode
    The synchronous mode requires from 2 to 6 extra pins depending on the data trace size and is only available in the larger packages. In addition it is available in JTAG mode and in Serial Wire mode and provides better bandwidth output capabilities than asynchronous trace.

Table 311. Synchronous TRACE pin assignment

| TPUI pin name | Trace synchronous mode | STM32F4xxpin assignment |  |
|---|---|---|---|
| Type | Description |  |  |
| TRACECK | O | TRACE Clock | PE2 |
| TRACED[3:0] | O | TRACE Sync Data Outputs Can be 1, 2 or 4. | PE[6:3] |


### TPUI TRACE pin assignment

By default, these pins are NOT assigned. They can be assigned by setting the TRACE_IOEN and TRACE_MODE bits in the **MCU Debug component configuration register**. This configuration has to be done by the debugger host.

In addition, the number of pins to assign depends on the trace configuration (asynchronous or synchronous).

*   **Asynchronous mode**: 1 extra pin is needed
*   **Synchronous mode**: from 2 to 5 extra pins are needed depending on the size of the data trace port register (1, 2 or 4):
    *   TRACECK
    *   TRACED(0) if port size is configured to 1, 2 or 4
    *   TRACED(1) if port size is configured to 2 or 4
    *   TRACED(2) if port size is configured to 4
    *   TRACED(3) if port size is configured to 4

To assign the TRACE pin, the debugger host must program the bits TRACE_IOEN and TRACE_MODE[1:0] of the Debug MCU configuration Register (DBGMCU_CR). By default the TRACE pins are not assigned.

This register is mapped on the external PPB and is reset by the PORESET (and not by the SYSTEM reset). It can be written by the debugger under SYSTEM reset.

1708/1749
RM0090 Rev 18
ST

--- Page 149 ---

RM0090
Debug support (DBG)

Table 312. Flexible TRACE pin assignment

| DBGMCU_CR register | Pins assigned for: | TRACE IO pin assigned |  |  |  |  |  |
|---|---|---|---|---|---|---|---|
| TRACE_IOEN | TRACE_MODE [1:0] | PB3 /JTDO/TRACESWO | PE2/TRACECK | PE3 /TRACED[0] | PE4 /TRACED[1] | PE5 /TRACED[2] | PE6 /TRACED[3] |
| 0 | XX | No Trace (default state) | Released (1) | - |  |  |  |
| 1 | 00 | Asynchronous Trace | TRACESWO | - | Released (usable as GPIO) |  |  |
| 1 | 01 | Synchronous Trace 1 bit | Released (1) | TRACECK | TRACED[0] | - |  |
| 1 | 10 | Synchronous Trace 2 bit | TRACECK | TRACED[0] | TRACED[1] | - |  |
| 1 | 11 | Synchronous Trace 4 bit | TRACECK | TRACED[0] | TRACED[1] | TRACED[2] | TRACED[3] |


1. When Serial Wire mode is used, it is released. But when JTAG is used, it is assigned to JTDO.

**Note:** By default, the TRACECLKIN input clock of the TPIU is tied to GND. It is assigned to HCLK two clock cycles after the bit TRACE_IOEN has been set.

The debugger must then program the Trace Mode by writing the PROTOCOL[1:0] bits in the SPP_R (Selected Pin Protocol) register of the TPIU.
* PROTOCOL=00: Trace Port Mode (synchronous)
* PROTOCOL=01 or 10: Serial Wire (Manchester or NRZ) Mode (asynchronous mode). Default state is 01

It then also configures the TRACE port size by writing the bits [3:0] in the CPSPS_R (Current Sync Port Size Register) of the TPIU:
* 0x1 for 1 pin (default state)
* 0x2 for 2 pins
* 0x8 for 4 pins

### 38.17.3 TPUI formatter

The formatter protocol outputs data in 16-byte frames:
* seven bytes of data
* eight bytes of mixed-use bytes consisting of:
    * 1 bit (LSB) to indicate it is a DATA byte ('0') or an ID byte ('1').
    * 7 bits (MSB) which can be data or change of source ID trace.
* one byte of auxiliary bits where each bit corresponds to one of the eight mixed-use bytes:
    * if the corresponding byte was a data, this bit gives bit0 of the data.
    * if the corresponding byte was an ID change, this bit indicates when that ID change takes effect.

ST
RM0090 Rev 18
1709/1749

--- Page 150 ---

Debug support (DBG) RM0090

Note: Refer to the Arm® CoreSight Architecture Specification v1.0 (Arm® IHI 0029B) for further information

### 38.17.4 TPUI frame synchronization packets

The TPUI can generate two types of synchronization packets:

*   The Frame Synchronization packet (or Full Word Synchronization packet)
    It consists of the word: 0x7F_FF_FF_FF (LSB emitted first). This sequence can not occur at any other time provided that the ID source code 0x7F has not been used.
    It is output periodically **between** frames.
    In continuous mode, the TPA must discard all these frames once a synchronization frame has been found.
*   The Half-Word Synchronization packet
    It consists of the half word: 0x7F_FF (LSB emitted first).
    It is output periodically **between or within** frames.
    These packets are only generated in continuous mode and enable the TPA to detect that the TRACE port is in IDLE mode (no TRACE to be captured). When detected by the TPA, it must be discarded.

### 38.17.5 Transmission of the synchronization frame packet

There is no Synchronization Counter register implemented in the TPIU of the core. Consequently, the synchronization trigger can only be generated by the **DWT**. Refer to the registers DWT Control Register (bits SYNCTAP[11:10]) and the DWT Current PC Sampler Cycle Count Register.

The TPUI Frame synchronization packet (0x7F_FF_FF_FF) is emitted:

*   after each TPIU reset release. This reset is synchronously released with the rising edge of the TRACECLKIN clock. This means that this packet is transmitted when the TRACE_IOEN bit in the DBGMCU_CFG register is set. In this case, the word 0x7F_FF_FF_FF is not followed by any formatted packet.
*   at each DWT trigger (assuming DWT has been previously configured). Two cases occur:
    *   If the bit SYNENA of the ITM is reset, only the word 0x7F_FF_FF_FF is emitted without any formatted stream which follows.
    *   If the bit SYNENA of the ITM is set, then the ITM synchronization packets will follow (0x80_00_00_00_00_00), formatted by the TPUI (trace source ID added).

### 38.17.6 Synchronous mode

The trace data output size can be configured to 4, 2 or 1 pin: TRACED(3:0)
The output clock is output to the debugger (TRACECK)
Here, TRACECLKIN is driven internally and is connected to HCLK only when TRACE is used.

Note: In this synchronous mode, it is not required to provide a stable clock frequency.
The TRACE I/Os (including TRACECK) are driven by the rising edge of TRACLKIN (equal to HCLK). Consequently, the output frequency of TRACECK is equal to HCLK/2.

1710/1749 RM0090 Rev 18 ST

--- Page 151 ---

RM0090
Debug support (DBG)

### 38.17.7 Asynchronous mode

This is a low cost alternative to output the trace using only 1 pin: this is the asynchronous output pin TRACESWO. Obviously there is a limited bandwidth.

TRACESWO is multiplexed with JTDO when using the SW-DP pin. This way, this functionality is available in all STM32F4xx packages.

This asynchronous mode requires a constant frequency for TRACECLKIN. For the standard UART (NRZ) capture mechanism, 5% accuracy is needed. The Manchester encoded version is tolerant up to 10%.

### 38.17.8 TRACECLKIN connection inside the STM32F4xx

In the STM32F4xx, this TRACECLKIN input is internally connected to HCLK. This means that when in asynchronous trace mode, the application is restricted to use to time frames where the CPU frequency is stable.

**Note:**
**Important:** when using asynchronous trace: it is important to be aware that:
The default clock of the STM32F4xx MCUs is the internal RC oscillator. Its frequency under reset is different from the one after reset release. This is because the RC calibration is the default one under system reset and is updated at each system reset release.

Consequently, the trace port analyzer (TPA) should not enable the trace (with the TRACE_IOEN bit) under system reset, because a Synchronization Frame Packet will be issued with a different bit time than trace packets which will be transmitted after reset release.

### 38.17.9 TPIU registers

The TPIU APB registers can be read and written only if the bit TRCENA of the Debug Exception and Monitor Control Register (DEMCR) is set. Otherwise, the registers are read as zero (the output of this bit enables the PCLK of the TPIU).

**Table 313. Important TPIU registers**

| Address | Register | Description |
|---|---|---|
| 0xE0040004 | Current port size | Allows the trace port size to be selected:Bit 0: Port size = 1Bit 1: Port size = 2Bit 2: Port size = 3, not supportedBit 3: Port Size = 4Only 1 bit must be set. By default, the port size is one bit. (0x00000001) |
| 0xE00400F0 | Selected pin protocol | Allows the Trace Port Protocol to be selected:Bit1:0=00: Sync Trace Port Mode01: Serial Wire Output - manchester (default value)10: Serial Wire Output - NRZ11: reserved |


ST
RM0090 Rev 18
1711/1749

--- Page 152 ---

Debug support (DBG)
RM0090

Table 313. Important TPIU registers (continued)

| Address | Register | Description |
|---|---|---|
| 0xE0040304 | Formatter and flush control | Bits 31-9 = always '0'Bit 8 = TrigIn = always '1' to indicate that triggers are indicatedBits 7-4 = always 0Bits 3-2 = always 0Bit 1 = EnFCont. In Sync Trace mode (Select_Pin_Protocol register bit1:0=00), this bit is forced to '1': the formatter is automatically enabled in continuous mode. In asynchronous mode (Select_Pin_Protocol register bit1:0 <> 00), this bit can be written to activate or not the formatter.Bit 0 = always 0The resulting default value is 0x102Note: In synchronous mode, because the TRACECTL pin is not mapped outside the chip, the formatter is always enabled in continuous mode -this way the formatter inserts some control packets to identify the source of the trace packets). |
| Not used in Cortex®-M4 with FPU, always read as 0x00000008 |  |  |
| 0xE0040300 | Formatter and flush status |  |


### 38.17.10 Example of configuration

* Set the bit TRCENA in the Debug Exception and Monitor Control Register (DEMCR)
* Write the TPIU Current Port Size Register to the desired value (default is 0x1 for a 1-bit port size)
* Write TPIU Formatter and Flush Control Register to 0x102 (default value)
* Write the TPIU Select Pin Protocol to select the sync or async mode. Example: 0x2 for async NRZ mode (UART like)
* Write the DBGMCU control register to 0x20 (bit IO_TRACEN) to assign TRACE I/Os for async mode. A TPIU Sync packet is emitted at this time (FF_FF_FF_7F)
* Configure the ITM and write the ITM Stimulus register to output a value

1712/1749
RM0090 Rev 18

--- Page 153 ---

RM0090
Debug support (DBG)


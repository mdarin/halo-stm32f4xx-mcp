### 38.14 ITM (instrumentation trace macrocell)

#### 38.14.1 General description

The ITM is an application-driven trace source that supports *printf* style debugging to trace Operating System (OS) and application events, and emits diagnostic system information. The ITM emits trace information as packets which can be generated as:

* **Software trace.** Software can write directly to the ITM stimulus registers to emit packets.
* **Hardware trace.** The DWT generates these packets, and the ITM emits them.
* **Time stamping.** Timestamps are emitted relative to packets. The ITM contains a 21-bit counter to generate the timestamp. The Cortex®-M4 with FPU clock or the bit clock rate of the *Serial Wire Viewer* (SWV) output clocks the counter.

The packets emitted by the ITM are output to the TPIU (Trace Port Interface Unit). The formatter of the TPIU adds some extra packets (refer to TPIU) and then output the complete packets sequence to the debugger host.

The bit TRCEN of the Debug Exception and Monitor Control Register must be enabled before programming or using the ITM.

#### 38.14.2 Time stamp packets, synchronization and overflow packets

Time stamp packets encode time stamp information, generic control and synchronization. It uses a 21-bit timestamp counter (with possible prescalers) which is reset at each time stamp packet emission. This counter can be either clocked by the CPU clock or the SWV clock.

A synchronization packet consists of 6 bytes equal to 0x80_00_00_00_00_00 which is emitted to the TPIU as 00 00 00 00 00 80 (LSB emitted first).

A synchronization packet is a timestamp packet control. It is emitted at each DWT trigger.

ST
RM0090 Rev 18
1699/1749

--- Page 140 ---

Debug support (DBG)
RM0090

For this, the DWT must be configured to trigger the ITM: the bit CYCCNTENA (bit0) of the DWT Control Register must be set. In addition, the bit2 (SYNCENA) of the ITM Trace Control Register must be set.

**Note:** If the SYNENA bit is not set, the DWT generates Synchronization triggers to the TPIU which will send only TPIU synchronization packets and not ITM synchronization packets.

An overflow packet consists is a special timestamp packets which indicates that data has been written but the FIFO was full.

**Table 308. Main ITM registers**

| Address | Register | Details |
|---|---|---|
| @E0000FB0 | ITM lock access | Write 0xC5ACCE55 to unlock Write Access to the other ITM registers |
| @E0000E80 | ITM trace control | Bits 31-24 = Always 0 |
| Bits 23 = Busy |  |  |
| Bits 22-16 = 7-bits ATB ID which identifies the source of the trace data. |  |  |
| Bits 15-10 = Always 0 |  |  |
| Bits 9:8 = TSPrescale = Time Stamp Prescaler |  |  |
| Bits 7-5 = Reserved |  |  |
| Bit 4 = SWOENA = Enable SWV behavior (to clock the timestamp counter by the SWV clock). |  |  |
| Bit 3 = DWTENA: Enable the DWT Stimulus |  |  |
| Bit 2 = SYNCENA: this bit must be to 1 to enable the DWT to generate synchronization triggers so that the TPIU can then emit the synchronization packets. |  |  |
| Bit 1 = TSENA (Timestamp Enable) |  |  |
| Bit 0 = ITMENА: Global Enable Bit of the ITM |  |  |
| @E0000E40 | ITM trace privilege | Bit 3: mask to enable tracing ports31:24 |
| Bit 2: mask to enable tracing ports23:16 |  |  |
| Bit 1: mask to enable tracing ports15:8 |  |  |
| Bit 0: mask to enable tracing ports7:0 |  |  |
| @E0000E00 | ITM trace enable | Each bit enables the corresponding Stimulus port to generate trace. |
| @E0000000-E000007C | Stimulus port registers 0-31 | Write the 32-bits data on the selected Stimulus Port (32 available) to be traced out. |


1700/1749
RM0090 Rev 18
STI

--- Page 141 ---

RM0090
Debug support (DBG)

**Example of configuration**

To output a simple value to the TPIU:

*   Configure the TPIU and assign TRACE I/Os by configuring the DBGMCU_CR (refer to Section 38.17.2 and Section 38.16.3)
*   Write 0xC5ACCE55 to the ITM Lock Access Register to unlock the write access to the ITM registers
*   Write 0x00010005 to the ITM Trace Control Register to enable the ITM with Sync enabled and an ATB ID different from 0x00
*   Write 0x1 to the ITM Trace Enable Register to enable the Stimulus Port 0
*   Write 0x1 to the ITM Trace Privilege Register to unmask stimulus ports 7:0
*   Write the value to output in the Stimulus Port Register 0: this can be done by software (using a printf function)


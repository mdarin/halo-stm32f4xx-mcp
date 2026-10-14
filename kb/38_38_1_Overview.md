# 38 Debug support (DBG)

This section applies to the whole STM32F4xx family, unless otherwise specified.

## 38.1 Overview

The STM32F4xx are built around a Cortex®-M4 with FPU core which contains hardware extensions for advanced debugging features. The debug extensions allow the core to be stopped either on a given instruction fetch (breakpoint) or data access (watchpoint). When stopped, the core’s internal state and the system’s external state may be examined. Once examination is complete, the core and the system may be restored and program execution resumed.

The debug features are used by the debugger host when connecting to and debugging the STM32F4xx MCUs.

Two interfaces for debug are available:

*   Serial wire
*   JTAG debug port

**Figure 485. Block diagram of STM32 MCU and Cortex®-M4 with FPU-level debug support**

STM32F4xx debug support
Cortex-M4 debug support
JTMS/SWDIO
JTDI
JTDO/TRACESWO
NJTRST
JTCK/SWCLK
Cortex-M4 core
Bus matrix
Data
DCode interface
System interface
SWJ-DP
AHB-AP
Internal private peripheral bus (PPB)
Bridge
External private peripheral bus (PPB)
TPIU
Trace port
TRACESWO
TRACECK
TRACED[3:0]
NVIC
DWT
FPB
ITM
DBGMCU
MS19908V3

**Note:** The debug features embedded in the Cortex®-M4 with FPU core are a subset of the Arm® CoreSight Design Kit.

ST
RM0090 Rev 18
1683/1749

--- Page 124 ---

**Debug support (DBG)**
RM0090

The Arm® Cortex®-M4 with FPU core provides integrated on-chip debug support. It is comprised of:

*   SWJ-DP: Serial wire / JTAG debug port
*   AHP-AP: AHB access port
*   ITM: Instrumentation trace macrocell
*   FPB: Flash patch breakpoint
*   DWT: Data watchpoint trigger
*   TPIU: Trace port unit interface (available on larger packages, where the corresponding pins are mapped)
*   ETM: Embedded Trace Macrocell (available on larger packages, where the corresponding pins are mapped)

It also includes debug features dedicated to the STM32F4xx:

*   Flexible debug pinout assignment
*   MCU debug box (support for low-power modes, control over peripheral clocks, etc.)

**Note:**
*For further information on debug functionality supported by the Arm® Cortex®-M4 with FPU core, refer to the Cortex®-M4 with FPU -r0p1 Technical Reference Manual and to the CoreSight Design Kit-r0p1 TRM (see Section 38.2).*

### 38.2 Reference Arm® documentation

*   Cortex®-M4 with FPU r0p1 Technical Reference Manual (TRM)
    (see Related documents on page 1)
*   Arm® Debug Interface V5
*   Arm® CoreSight Design Kit revision r0p1 Technical Reference Manual

### 38.3 SWJ debug port (serial wire and JTAG)

The core of the STM32F4xx integrates the Serial Wire / JTAG Debug Port (SWJ-DP). It is an Arm® standard CoreSight debug port that combines a JTAG-DP (5-pin) interface and a SW-DP (2-pin) interface.

*   The JTAG Debug Port (JTAG-DP) provides a 5-pin standard JTAG interface to the AHP-AP port.
*   The Serial Wire Debug Port (SW-DP) provides a 2-pin (clock + data) interface to the AHP-AP port.

In the SWJ-DP, the two JTAG pins of the SW-DP are multiplexed with some of the five JTAG pins of the JTAG-DP.

1684/1749
RM0090 Rev 18
STI

--- Page 125 ---

RM0090
Debug support (DBG)

Figure 486. SWJ debug port

ai17139

Figure 486 shows that the asynchronous TRACE output (TRACESWO) is multiplexed with TDO. This means that the asynchronous trace can only be used with SW-DP, not JTAG-DP.

### 38.3.1 Mechanism to select the JTAG-DP or the SW-DP

By default, the JTAG-Debug Port is active.

If the debugger host wants to switch to the SW-DP, it must provide a dedicated JTAG sequence on TMS/TCK (respectively mapped to SWDIO and SWCLK) which disables the JTAG-DP and enables the SW-DP. This way it is possible to activate the SWDP using only the SWCLK and SWDIO pins.

This sequence is:

1. Send more than 50 TCK cycles with TMS (SWDIO) =1
2. Send the 16-bit sequence on TMS (SWDIO) = 0111100111100111 (MSB transmitted first)
3. Send more than 50 TCK cycles with TMS (SWDIO) =1


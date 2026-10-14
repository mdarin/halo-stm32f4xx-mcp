### 38.10 Core debug

Core debug is accessed through the core debug registers. Debug access to these registers is by means of the *Advanced High-performance Bus (AHB-AP)* port. The processor can access these registers directly over the internal *Private Peripheral Bus (PPB)*.

It consists of 4 registers:

**Table 307. Core debug registers**

| Register | Description |
|---|---|
| DHCSR | The 32-bit Debug Halting Control and Status RegisterThis provides status information about the state of the processor enable core debug halt and step the processor |
| DCRSR | The 17-bit Debug Core Register Selector Register:This selects the processor register to transfer data to or from. |
| DCRDR | The 32-bit Debug Core Register Data Register:This holds data for reading and writing registers to and from the processor selected by the DCRSR (Selector) register. |
| DEMCR | The 32-bit Debug Exception and Monitor Control Register:This provides Vector Catching and Debug Monitor Control. This register contains a bit named *TRCENA* which enable the use of a TRACE. |


**Note:** **Important:** these registers are not reset by a system reset. They are only reset by a power-on reset.

Refer to the Cortex®-M4 with FPU r0p1 TRM for further details.

To Halt on reset, it is necessary to:
* enable the bit0 (VC_CORRESET) of the Debug and Exception Monitor Control Register
* enable the bit0 (C_DEBUGEN) of the Debug Halting Control and Status Register.

ST
RM0090 Rev 18
1697/1749

--- Page 138 ---

Debug support (DBG) RM0090

### 38.11 Capability of the debugger host to connect under system reset

The reset system of the STM32F4xx MCU comprises the following reset sources:

*   POR (power-on reset) which asserts a RESET at each power-up.
*   Internal watchdog reset
*   Software reset
*   External reset

The Cortex®-M4 with FPU differentiates the reset of the debug part (generally PORRESETn) and the other one (SYSRESETn)

This way, it is possible for the debugger to connect under System Reset, programming the Core Debug Registers to halt the core when fetching the reset vector. Then the host can release the system reset and the core will immediately halt without having executed any instructions. In addition, it is possible to program any debug features under System Reset.

**Note:** *It is highly recommended for the debugger host to connect (set a breakpoint in the reset vector) under system reset.*

### 38.12 FPB (Flash patch breakpoint)

The FPB unit:

*   implements hardware breakpoints
*   patches code and data from code space to system space. This feature gives the possibility to correct software bugs located in the Code Memory Space.

The use of a Software Patch or a Hardware Breakpoint is exclusive.

The FPB consists of:

*   2 literal comparators for matching against literal loads from Code Space and remapping to a corresponding area in the System Space.
*   6 instruction comparators for matching against instruction fetches from Code Space. They can be used either to remap to a corresponding area in the System Space or to generate a Breakpoint Instruction to the core.

1698/1749 RM0090 Rev 18 ST

--- Page 139 ---

RM0090
Debug support (DBG)

### 38.13 DWT (data watchpoint trigger)

The DWT unit consists of four comparators. They are configurable as:

* a hardware watchpoint or
* a trigger to an ETM or
* a PC sampler or
* a data address sampler

The DWT also provides some means to give some profiling informations. For this, some counters are accessible to give the number of:

* Clock cycle
* Folded instructions
* Load store unit (LSU) operations
* Sleep cycles
* CPI (clock per instructions)
* Interrupt overhead


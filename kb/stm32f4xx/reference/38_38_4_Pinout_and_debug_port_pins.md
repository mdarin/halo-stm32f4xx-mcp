### 38.4 Pinout and debug port pins

The STM32F4xx MCUs are available in various packages with different numbers of available pins. As a result, some functionality (ETM) related to pin availability may differ between packages.

ST
RM0090 Rev 18
1685/1749

--- Page 126 ---

Debug support (DBG)
RM0090

### 38.4.1 SWJ debug port pins

Five pins are used as outputs from the STM32F4xx for the SWJ-DP as *alternate functions* of general-purpose I/Os. These pins are available on all packages.

**Table 298. SWJ debug port pins**

| SWJ-DP pin name | JTAG debug port | SW debug port | Pin assignment |  |  |
|---|---|---|---|---|---|
| Type | Description | Type | Debug assignment |  |  |
| JTMS/SWDIO | I | JTAG Test Mode Selection | IO | Serial Wire Data Input/Output | PA13 |
| JTCK/SWCLK | I | JTAG Test Clock | I | Serial Wire Clock | PA14 |
| JTDI | I | JTAG Test Data Input | - | PA15 |  |
| JTDO/TRACESWO | O | JTAG Test Data Output | - | TRACESWO if async trace is enabled | PB3 |
| NJTRST | I | JTAG Test nReset | - | PB4 |  |


### 38.4.2 Flexible SWJ-DP pin assignment

After RESET (SYSRESETn or PORESETn), all five pins used for the SWJ-DP are assigned as dedicated pins immediately usable by the debugger host (note that the trace outputs are not assigned except if explicitly programmed by the debugger host).

However, the STM32F4xx MCUs offers the possibility of disabling some or all of the SWJ-DP ports and so, of releasing the associated pins for general-purpose IO (GPIO) usage. For more details on how to disable SWJ-DP port pins, please refer to *Section 8.3.2: I/O pin multiplexer and mapping*.

**Table 299. Flexible SWJ-DP pin assignment**

| Available debug ports | SWJ IO pin assigned |  |  |  |
|---|---|---|---|---|
| PA13 / JTMS / SWDIO | PA14 / JTCK / SWCLK | PA15 / JTDI | PB3 / JTDO | PB4 / NJTRST |
| Full SWJ (JTAG-DP + SW-DP) - Reset State | X |  |  |  |
| Full SWJ (JTAG-DP + SW-DP) but without NJTRST | X |  |  |  |
| JTAG-DP Disabled and SW-DP Enabled | X |  |  |  |
| JTAG-DP Disabled and SW-DP Disabled |  |  | Released |  |


**Note:**
When the APB bridge write buffer is full, it takes one extra APB cycle when writing the GPIO_AFR register. This is because the deactivation of the JTAGSW pins is done in two cycles to guarantee a clean level on the nTRST and TCK input signals of the core.
* Cycle 1: the JTAGSW input signals to the core are tied to 1 or 0 (to 1 for nTRST, TDI and TMS, to 0 for TCK)
* Cycle 2: the GPIO controller takes the control signals of the SWJTAG IO pins (like controls of direction, pull-up/down, Schmitt trigger activation, etc.).

1686/1749
RM0090 Rev 18

--- Page 127 ---

RM0090
Debug support (DBG)

### 38.4.3 Internal pull-up and pull-down on JTAG pins

It is necessary to ensure that the JTAG input pins are not floating since they are directly connected to flip-flops to control the debug mode features. Special care must be taken with the SWCLK/TCK pin which is directly connected to the clock of some of these flip-flops.

To avoid any uncontrolled IO levels, the device embeds internal pull-ups and pull-downs on the JTAG input pins:

*   NJTRST: Internal pull-up
*   JTDI: Internal pull-up
*   JTMS/SWDIO: Internal pull-up
*   TCK/SWCLK: Internal pull-down

Once a JTAG IO is released by the user software, the GPIO controller takes control again. The reset states of the GPIO control registers put the I/Os in the equivalent state:

*   NJTRST: AF input pull-up
*   JTDI: AF input pull-up
*   JTMS/SWDIO: AF input pull-up
*   JTCK/SWCLK: AF input pull-down
*   JTDO: AF output floating

The software can then use these I/Os as standard GPIOs.

**Note:** *The JTAG IEEE standard recommends to add pull-ups on TDI, TMS and nTRST but there is no special recommendation for TCK. However, for JTCK, the device needs an integrated pull-down.*

*Having embedded pull-ups and pull-downs removes the need to add external resistors.*

RM0090 Rev 18
1687/1749

--- Page 128 ---

Debug support (DBG) RM0090

### 38.4.4 Using serial wire and releasing the unused debug pins as GPIOs

To use the serial wire DP to release some GPIOs, the user software must change the GPIO (PA15, PB3 and PB4) configuration mode in the GPIO_MODER register. This releases PA15, PB3 and PB4 which now become available as GPIOs.

When debugging, the host performs the following actions:

*   Under system reset, all SWJ pins are assigned (JTAG-DP + SW-DP).
*   Under system reset, the debugger host sends the JTAG sequence to switch from the JTAG-DP to the SW-DP.
*   Still under system reset, the debugger sets a breakpoint on vector reset.
*   The system reset is released and the Core halts.
*   All the debug communications from this point are done using the SW-DP. The other JTAG pins can then be reassigned as GPIOs by the user software.

**Note:** For user software designs, note that:

To release the debug pins, remember that they will be first configured either in input-pull-up (nTRST, TMS, TDI) or pull-down (TCK) or output tristate (TDO) for a certain duration after reset until the instant when the user software releases the pins.

When debug pins (JTAG or SW or TRACE) are mapped, changing the corresponding IO pin configuration in the IOPORT controller has no effect.

### 38.5 STM32F4xx JTAG TAP connection

The STM32F4xx MCUs integrate two serially connected JTAG TAPs, the boundary scan TAP (IR is 5-bit wide) and the Cortex®-M4 with FPU TAP (IR is 4-bit wide).

To access the TAP of the Cortex®-M4 with FPU for debug purposes:

1.  First, it is necessary to shift the BYPASS instruction of the boundary scan TAP.
2.  Then, for each IR shift, the scan chain contains 9 bits (=5+4) and the unused TAP instruction must be shifted in using the BYPASS instruction.
3.  For each data shift, the unused TAP, which is in BYPASS mode, adds 1 extra data bit in the data scan chain.

**Note:** **Important:** Once Serial-Wire is selected using the dedicated Arm® JTAG sequence, the boundary scan TAP is automatically disabled (JTMS forced high).

1688/1749 RM0090 Rev 18 ST

--- Page 129 ---

RM0090
Debug support (DBG)

Figure 487. JTAG TAP connections

STM32F4xxx
NJTRST
JTMS
SW-DP
Selected
JTDI
JTDO
TMS nTRST
TDI
TDO
Boundary scan
TAP
IR is 5-bit wide
TMS nTRST
TDI
TDO
Cortex-M4 TAP
IR is 4-bit wide
MS19929V2

ST
RM0090 Rev 18
1689/1749

--- Page 130 ---

Debug support (DBG) RM0090


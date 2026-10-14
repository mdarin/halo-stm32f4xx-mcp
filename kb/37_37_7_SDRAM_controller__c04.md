> [RM0090 ch.37 §37.7] SDRAM controller


### 37.7.4 Low power modes

Two low power modes are available:

*   **Self-refresh mode**
    The auto-refresh cycles are performed by the SDRAM device itself to retain data without external clocking.
*   **Power-down mode**
    The auto-refresh cycles are performed by the SDRAM controller.

**Self-refresh mode**

This mode is selected by setting the MODE bits to '101' and by configuring the Target Bank bits (CTB1 and/or CTB2) in the FMC_SDCMR register.

The SDRAM clock stops running after a TRAS delay and the internal refresh timer stops counting only if one of the following conditions is met:

*   A Self-refresh command is issued to both devices
*   One of the devices is not activated (SDRAM bank is not initialized).

Before entering Self-Refresh mode, the SDRAM controller automatically issues a PALL command.

If the Write data FIFO is not empty, all data are sent to the memory before activating the Self-refresh mode and the BUSY status flag remains set.

In Self-refresh mode, all SDRAM device inputs become don't care except for SDCKE which remains low.

The SDRAM device must remain in Self-refresh mode for a minimum period of time of TRAS and can remain in Self-refresh mode for an indefinite period beyond that. To guarantee this minimum period, the BUSY status flag remains high after the Self-refresh activation during a TRAS delay.

As soon as an SDRAM device is selected, the SDRAM controller generates a sequence of commands to exit from Self-refresh mode. After the memory access, the selected device remains in Normal mode.

To exit from Self-refresh, the MODE bits must be set to '000' (Normal mode) and the Target Bank bits (CTB1 and/or CTB2) must be configured in the FMC_SDCMR register.

ST
RM0090 Rev 18
1671/1749

--- Page 112 ---

Flexible memory controller (FMC)
RM0090

Figure 483. Self-refresh mode

SDCLK
SDCKE
COMMAND
DOM/
DOML/DOMU
A0-A9
A11, A12
A10
Data[31:0] Hi-Z

T0
T1
T2
Tn+1
T0+1
T0+2

tRAS(min)

PRECHARGE
NOP
AUTO
REFRESH
NOP
or COMMAND
INHERIT
AUTO
REFRESH

ALL
BANKS

tRP
tXSR

Precharge all
active banks
Enter Self-refresh mode
CLK stable prior to existing
Self-refresh mode
Exit Self-refresh mode
(restart refresh timebase)

MS30450V1

**Power-down mode**

This mode is selected by setting the MODE bits to '110' and by configuring the Target Bank
bits (CTB1 and/or CTB2) in the FMC_SDCMR register.

1672/1749
RM0090 Rev 18

--- Page 113 ---

RM0090
Flexible memory controller (FMC)

Figure 484. Power-down mode

SDCLK
SDCKE
COMMAND
NOP
All banks idle
Enter Power-down
Input buffers gated off
NOP
Exit Power-down
ACTIVE
tRCD
tRAS
tRC
MS30451V1

If the Write data FIFO is not empty, all data are sent to the memory before activating the Power-down mode.

As soon as an SDRAM device is selected, the SDRAM controller exits from the Power-down mode. After the memory access, the selected SDRAM device remains in Normal mode.

During Power-down mode, all SDRAM device input and output buffers are deactivated except for the SDCKE which remains low.

The SDRAM device cannot remain in Power-down mode longer than the refresh period and cannot perform the Auto-refresh cycles by itself. Therefore, the SDRAM controller carries out the refresh operation by executing the operations below:

1. Exit from Power-down mode and drive the SDCKE high
2. Generate the PALL command only if a row was active during Power-down mode
3. Generate the auto-refresh command
4. Drive SDCKE low again to return to Power-down mode.

To exit from Power-down mode, the MODE bits must be set to '000' (Normal mode) and the Target Bank bits (CTB1 and/or CTB2) must be configured in the FMC_SDCMR register.

ST
RM0090 Rev 18
1673/1749

--- Page 114 ---

Flexible memory controller (FMC)
RM0090


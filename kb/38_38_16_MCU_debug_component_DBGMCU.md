### 38.16 MCU debug component (DBGMCU)

The MCU debug component helps the debugger provide support for:

* Low-power modes
* Clock control for timers, watchdog, I2C and bxCAN during a breakpoint
* Control of the trace pins assignment

### 38.16.1 Debug support for low-power modes

To enter low-power mode, the instruction WFI or WFE must be executed.

The MCU implements several low-power modes which can either deactivate the CPU clock or reduce the power of the CPU.

1702/1749
RM0090 Rev 18
STI

--- Page 143 ---

RM0090
Debug support (DBG)

The core does not allow FCLK or HCLK to be turned off during a debug session. As these are required for the debugger connection, during a debug, they must remain active. The MCU integrates special means to allow the user to debug software in low-power modes.

For this, the debugger host must first set some debug configuration registers to change the low-power mode behavior:

*   In Sleep mode, DBG_SLEEP bit of DBGMCU_CR register must be previously set by the debugger. This will feed HCLK with the same clock that is provided to FCLK (system clock previously configured by the software).
*   In Stop mode, the bit DBG_STOP must be previously set by the debugger. This will enable the internal RC oscillator clock to feed FCLK and HCLK in STOP mode.

### 38.16.2 Debug support for timers, watchdog, bxCAN and I²C

During a breakpoint, it is necessary to choose how the counter of timers and watchdog should behave:

*   They can continue to count inside a breakpoint. This is usually required when a PWM is controlling a motor, for example.
*   They can stop to count inside a breakpoint. This is required for watchdog purposes.

For the bxCAN, the user can choose to block the update of the receive register during a breakpoint.

For the I²C, the user can choose to block the SMBUS timeout during a breakpoint.

For timers having complementary outputs, when the counter is stopped (DBG_TIMx_STOP = 1), the outputs are disabled (as if the MOE bit was reset) for safety purposes.

### 38.16.3 Debug MCU configuration register

This register allows the configuration of the MCU under DEBUG. This concerns:

*   Low-power mode support
*   Timer and watchdog counter support
*   bxCAN communication support
*   Trace pin assignment

This DBGMCU_CR is mapped on the External PPB bus at address 0xE0042004

It is asynchronously reset by the PORESET (and not the system reset). It can be written by the debugger under system reset.

If the debugger host does not support these features, it is still possible for the user software to write to these registers.

**DBGMCU_CR register**

Address: 0xE004 2004

Only 32-bit access supported

POR Reset: 0x0000 0000 (not reset by system reset)

STI
RM0090 Rev 18
1703/1749

--- Page 144 ---

**Debug support (DBG)**

RM0090

| 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved |  |  |  |  |  |  |  |  |  |  |  |
| 15 14 13 12 11 10 9 8 | 7 | 6 | 5 4 3 | 2 | 1 | 0 |  |  |  |  |  |
| Reserved | TRACE_MODE [1:0] | TRACE_IOEN | Reserved | DBG_STANDBY | DBG_STOP | DBG_SLEEP |  |  |  |  |  |
|  |  |  |  |  |  |  |  | rw |  |  | rw |


Bits 31:8 Reserved, must be kept at reset value.

Bits 7:5 **TRACE_MODE[1:0]** and **TRACE_IOEN**: Trace pin assignment control
- With TRACE_IOEN=0:
  - TRACE_MODE=xx: TRACE pins not assigned (default state)
- With TRACE_IOEN=1:
  - TRACE_MODE=00: TRACE pin assignment for Asynchronous Mode
  - TRACE_MODE=01: TRACE pin assignment for Synchronous Mode with a TRACEDATA size of 1
  - TRACE_MODE=10: TRACE pin assignment for Synchronous Mode with a TRACEDATA size of 2
  - TRACE_MODE=11: TRACE pin assignment for Synchronous Mode with a TRACEDATA size of 4

Bits 4:3 Reserved, must be kept at reset value.

Bit 2 **DBG_STANDBY**: Debug Standby mode
0: (FCLK=Off, HCLK=Off) The whole digital part is unpowered.
From software point of view, exiting from Standby is identical than fetching reset vector (except a few status bit indicated that the MCU is resuming from Standby)
1: (FCLK=On, HCLK=On) In this case, the digital part is not unpowered and FCLK and HCLK are provided by the internal RC oscillator which remains active. In addition, the MCU generate a system reset during Standby mode so that exiting from Standby is identical than fetching from reset

Bit 1 **DBG_STOP**: Debug Stop mode
0: (FCLK=Off, HCLK=Off) In STOP mode, the clock controller disables all clocks (including HCLK and FCLK). When exiting from STOP mode, the clock configuration is identical to the one after RESET (CPU clocked by the 8 MHz internal RC oscillator (HSI)). Consequently, the software must reprogram the clock controller to enable the PLL, the Xtal, etc.
1: (FCLK=On, HCLK=On) In this case, when entering STOP mode, FCLK and HCLK are provided by the internal RC oscillator which remains active in STOP mode. When exiting STOP mode, the software must reprogram the clock controller to enable the PLL, the Xtal, etc. (in the same way it would do in case of DBG_STOP=0)

Bit 0 **DBG_SLEEP**: Debug Sleep mode
0: (FCLK=On, HCLK=Off) In Sleep mode, FCLK is clocked by the system clock as previously configured by the software while HCLK is disabled.
In Sleep mode, the clock controller configuration is not reset and remains in the previously programmed state. Consequently, when exiting from Sleep mode, the software does not need to reconfigure the clock controller.
1: (FCLK=On, HCLK=On) In this case, when entering Sleep mode, HCLK is fed by the same clock that is provided to FCLK (system clock as previously configured by the software).

1704/1749 RM0090 Rev 18

--- Page 145 ---

RM0090
Debug support (DBG)

### 38.16.4 Debug MCU APB1 freeze register (DBGMCU_APB1_FZ)

The DBGMCU_APB1_FZ register is used to configure the MCU under Debug. It concerns APB1 peripherals. It is mapped on the external PPB bus at address 0xE004 2008.

The register is asynchronously reset by the POR (and not the system reset). It can be written by the debugger under system reset.

Address : 0xE004 2008

Only 32-bits access are supported.

Power-on reset (POR): 0x0000 0000 (not reset by system reset)

| 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved | DBG_CAN2_STOP | DBG_CAN1_STOP | Reserved | DBG_I2C3_SMBUS_TIMEOUT | DBG_I2C2_SMBUS_TIMEOUT | DBG_I2C1_SMBUS_TIMEOUT | Reserved |  |  |  |  |  |  |
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reserved | DBG_IWDG_STOP | DBG_WWDG_STOP | DBG_RTC_STOP | Reserved | DBG_TIM14_STOP | DBG_TIM13_STOP | DBG_TIM12_STOP | DBG_TIM7_STOP | DBG_TIM6_STOP | DBG_TIM5_STOP | DBG_TIM4_STOP | DBG_TIM3_STOP | DBG_TIM2_STOP |
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bits 31:27 Reserved, must be kept at reset value.

**Bit 26 DBG_CAN2_STOP**: Debug CAN2 stopped when Core is halted
0: Same behavior as in normal mode
1: The CAN2 receive registers are frozen

**Bit 25 DBG_CAN1_STOP**: Debug CAN2 stopped when Core is halted
0: Same behavior as in normal mode
1: The CAN2 receive registers are frozen

**Bit 24** Reserved, must be kept at reset value.

**Bit 23 DBG_I2C3_SMBUS_TIMEOUT**: SMBUS timeout mode stopped when Core is halted
0: Same behavior as in normal mode
1: The SMBUS timeout is frozen

**Bit 22 DBG_I2C2_SMBUS_TIMEOUT**: SMBUS timeout mode stopped when Core is halted
0: Same behavior as in normal mode
1: The SMBUS timeout is frozen

**Bit 21 DBG_I2C1_SMBUS_TIMEOUT**: SMBUS timeout mode stopped when Core is halted
0: Same behavior as in normal mode
1: The SMBUS timeout is frozen

ST
RM0090 Rev 18
1705/1749

--- Page 146 ---

Debug support (DBG)
RM0090

Bit 20:13 Reserved, must be kept at reset value.
Bit 12 **DBG_IWDG_STOP**: Debug independent watchdog stopped when core is halted
0: The independent watchdog counter clock continues even if the core is halted
1: The independent watchdog counter clock is stopped when the core is halted
Bit 11 **DBG_WWDG_STOP**: Debug Window Watchdog stopped when Core is halted
0: The window watchdog counter clock continues even if the core is halted
1: The window watchdog counter clock is stopped when the core is halted
Bit 10 **DBG_RTC_STOP**: RTC stopped when Core is halted
0: The RTC counter clock continues even if the core is halted
1: The RTC counter clock is stopped when the core is halted
Bit 9 Reserved, must be kept at reset value.
Bits 8:0 **DBG_TIMx_STOP**: TIMx counter stopped when core is halted (x=2..7, 12..14)
0: The clock of the involved timer counter is fed even if the core is halted
1: The clock of the involved timer counter is stopped and the outputs are disabled when the core is halted

### 38.16.5 Debug MCU APB2 Freeze register (DBGMCU_APB2_FZ)

The DBGMCU_APB2_FZ register is used to configure the MCU under Debug. It concerns APB2 peripherals.

This register is mapped on the external PPB bus at address 0xE004 200C

It is asynchronously reset by the POR (and not the system reset). It can be written by the debugger under system reset.

Address: 0xE004 200C

Only 32-bit access is supported.

POR: 0x0000 0000 (not reset by system reset)

| 31 30 29 28 27 26 25 24 23 22 21 20 19 | 18 | 17 | 16 |
|---|---|---|---|
| Reserved | DBG_TIM11_STOPrw | DBG_TIM10_STOPrw | DBG_TIM9_STOPrw |
| 15 14 13 12 11 10 9 8 7 6 5 4 3 2 | 1 | 0 |  |
| Reserved | DBG_TIM8_STOPrw | DBG_TIM1_STOPrw |  |


Bits 31:19 Reserved, must be kept at reset value.
Bits 18:16 **DBG_TIMx_STOP**: TIMx counter stopped when core is halted (x=9..11)
0: The clock of the involved timer counter is fed even if the core is halted
1: The clock of the involved timer counter is stopped and the outputs are disabled when the core is halted

1706/1749
RM0090 Rev 18
ST

--- Page 147 ---

RM0090
Debug support (DBG)

Bits 15: Reserved, must be kept at reset value.
Bit 1 **DBG_TIM8_STOP**: TIM8 counter stopped when core is halted
0: The clock of the involved timer counter is fed even if the core is halted
1: The clock of the involved timer counter is stopped and the outputs are disabled when the core is halted
Bit 0 **DBG_TIM1_STOP**: TIM1 counter stopped when core is halted
0: The clock of the involved timer counter is fed even if the core is halted
1: The clock of the involved timer counter is stopped and the outputs are disabled when the core is halted


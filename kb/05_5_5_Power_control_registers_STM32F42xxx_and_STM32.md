**5.5 Power control registers (STM32F42xxx and STM32F43xxx)**

**5.5.1 PWR power control register (PWR_CR)**

**for STM32F42xxx and STM32F43xxx**

Address offset: 0x00

Reset value: 0x0000 C000 (reset by wakeup from Standby mode)

31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16

| Reserved | UDEN[1:0] | ODSWE N | ODEN |
|---|---|---|---|
| rw |  |  |  |


15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0

| VOS[1:0] | ADCDC1 | Res. | MRUDS | LPUDS | FPDS | DBP | PLS[2:0] | PVDE | CSBF | CWUF | PDDS | LPDS |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| rw | rc w1 _ | rw |  |  |  |  |  |  |  |  |  |  |


Bits 31:20 Reserved, must be kept at reset value.

Bits 19:18 UDEN[1:0] : Under-drive enable in stop mode

These bits are set by software. They allow to achieve a lower power consumption in Stop mode but with a longer wakeup time.

When set, the digital area has less leakage consumption when the device enters Stop mode. 00: Under-drive disable

01: Reserved

10: Reserved

11:Under-drive enable

Bit 17 ODSWEN : Over-drive switching enabled.

This bit is set by software. It is cleared automatically by hardware after exiting from Stop mode or when the ODEN bit is reset. When set, It is used to switch to Over-drive mode. To set or reset the ODSWEN bit, the HSI or HSE must be selected as system clock. The ODSWEN bit must only be set when the ODRDY flag is set to switch to Over-drive mode.

0: Over-drive switching disabled

1: Over-drive switching enable d

Note: On any over-drive switch (enabled or disabled), the system clock will be stalled during

the internal voltage set up.

Bit 16 ODEN : Over-drive enable

This bit is set by software. It is cleared automatically by hardware after exiting from Stop mode. It is used to enabled the Over-drive mode in order to reach a higher frequency. To set or reset the ODEN bit, the HSI or HSE must be selected as system clock. When the ODEN bit is set, the application must first wait for the Over-drive ready flag (ODRDY) to be set before setting the ODSWEN bit.

0: Over-drive disabled

1: Over-drive enabled



--- Page 145 ---

**RM0090 Power controller (PWR)**

Bits 15:14 VOS[1:0] : Regulator voltage scaling output selection

These bits control the main internal voltage regulator output voltage to achieve a trade-off between performance and power consumption when the device does not operate at the maximum frequency (refer to the STM32F42xx and STM32F43xx datasheets for more details).

These bits can be modified only when the PLL is OFF. The new value programmed is active only when the PLL is ON. When the PLL is OFF, the voltage scale 3 is automatically selected.

00: Reserved (Scale 3 mode selected)

01: Scale 3 mode

10: Scale 2 mode

11: Scale 1 mode (reset value)

Bit 13 ADCDC1 :

0: No effect.

1: Refer to AN4073 for details on how to use this bit.

Note: This bit can only be set when operating at supply voltage range 2.7 to 3.6V and when

the Prefetch is OFF.

Bit 12 Reserved, must be kept at reset value.

Bit 11 MRUDS : Main regulator in deepsleep under-drive mode

This bit is set and cleared by software.

0: Main regulator ON when the device is in Stop mode

1: Main Regulator in under-drive mode and Flash memory in power-down when the device is in Stop under-drive mode.

Bit 10 LPUDS : Low-power regulator in deepsleep under-drive mode

This bit is set and cleared by software.

0: Low-power regulator ON if LPDS bit is set when the device is in Stop mode

1: Low-power regulator in under-drive mode if LPDS bit is set and Flash memory in power- down when the device is in Stop under-drive mode.

Bit 9 FPDS : Flash power-down in Stop mode

When set, the Flash memory enters power-down mode when the device enters Stop mode. This allows to achieve a lower consumption in stop mode but a longer restart time.

0: Flash memory not in power-down when the device is in Stop mode

1: Flash memory in power-down when the device is in Stop mode

Bit 8 DBP : Disable backup domain write protection

In reset state, the RCC_BDCR register, the RTC registers (including the backup registers), and the BRE bit of the PWR_CSR register, are protected against parasitic write access. This bit must be set to enable write access to these registers.

0: Access to RTC and RTC Backup registers and backup SRAM disabled

1: Access to RTC and RTC Backup registers and backup SRAM enabled

RM0090 Rev 18 145/1749

149

--- Page 146 ---

**Power controller (PWR)**

**RM0090**

Bits 7:5 PLS[2:0]: PVD level selection

These bits are written by software to select the voltage threshold detected by the Power

Voltage Detector

000: 2.0 V

001: 2.1 V

010: 2.3 V

011: 2.5 V

100: 2.6 V

101: 2.7 V

110: 2.8 V

111: 2.9 V

Note:

Refer to the electrical characteristics of the datasheet for more details.

Bit 4 PVDE: Power voltage detector enable This bit is set and cleared by software.

0: PVD disabled

1: PVD enabled

Bit 3 CSBF : Clear standby flag

This bit is always read as 0.

0: No effect

1: Clear the SBF Standby Flag (write).

Bit 2 CWUF: Clear wakeup flag

This bit is always read as 0.

0: No effect

1: Clear the WUF Wakeup Flag

**after 2 System clock cycles**

Bit 1 PDDS : Power-down deepsleep

This bit is set and cleared by software. It works together with the LPDS bit.

0: Enter Stop mode when the CPU enters deepsleep. The regulator status depends on the

LPDS bit.

1: Enter Standby mode when the CPU enters deepsleep. Bit 0 LPDS: Low-power deepsleep

This bit is set and cleared by software. It works together with the PDDS bit.

0:Main voltage regulator ON during Stop mode 1: Low-power voltage regulator ON during Stop mode

146/1749

RM0090 Rev 18

--- Page 147 ---

**RM0090 Power controller (PWR)**

**5.5.2 PWR power control/status register (PWR_CSR)**

**for STM32F42xxx and STM32F43xxx**

Address offset: 0x04

Reset value: 0x0000 0000 (not reset by wakeup from Standby mode)

Additional APB cycles are needed to read this register versus a standard APB read.

31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16

| Reserved | UDRDY[1:0] | ODSWRDY | ODRDY |
|---|---|---|---|
| rc w1 _ | r |  |  |


15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0

| Res | VOS RDY | Reserved | BRE | EWUP | Reserved. | BRR | PVDO | SBF | WUF |
|---|---|---|---|---|---|---|---|---|---|
| r | rw | r |  |  |  |  |  |  |  |


Bits 31:20 Reserved, must be kept at reset value.

Bits 19:18 UDRDY[1:0] : Under-drive ready flag

These bits are set by hardware when MCU entered stop Under-drive mode and exited. When the under-drive mode is enabled, these bits are not set as long as the MCU has not entered stop mode yet. They are cleared by programming them to 1.

00: Under-drive is disabled

01: Reserved

10: Reserved

11:Under-drive mode is activated in Stop mode.

Bit 17 ODSWRDY : Over-drive mode switching ready

0: Over-drive mode is not active.

1: Over-drive mode is active on digital area on 1.2 V domain

Bit 16 ODRDY : Over-drive mode ready

0: Over-drive mode not ready.

1: Over-drive mode ready

Bit 14 VOSRDY : Regulator voltage scaling output selection ready bit

0: Not ready

1: Ready

Bits 13:10 Reserved, must be kept at reset value.

Bit 9 BRE : Backup regulator enable

When set, the Backup regulator (used to maintain backup SRAM content in Standby and V BAT modes) is enabled. If BRE is reset, the backup regulator is switched off. The backup SRAM can still be used but its content will be lost in the Standby and V BAT modes. Once set, the application must wait that the Backup Regulator Ready flag (BRR) is set to indicate that the data written into the RAM will be maintained in the Standby and V BAT modes.

0: Backup regulator disabled

1: Backup regulator enabled

Note: This bit is not reset when the device wakes up from Standby mode, by a system reset,

or by a power reset.



149

--- Page 148 ---

**Power controller (PWR) RM0090**

Bit 8 EWUP : Enable WKUP pin

This bit is set and cleared by software.

0: WKUP pin is used for general purpose I/O. An event on the WKUP pin does not wakeup the device from Standby mode.

1: WKUP pin is used for wakeup from Standby mode and forced in input pull down configuration (rising edge on WKUP pin wakes-up the system from Standby mode). Note: This bit is reset by a system reset.

Bits 7:4 Reserved, must be kept at reset value.

Bit 3 BRR : Backup regulator ready

Set by hardware to indicate that the Backup Regulator is ready.

0: Backup Regulator not ready

1: Backup Regulator ready

Note: This bit is not reset when the device wakes up from Standby mode or by a system reset

or power reset.

Bit 2 PVDO: PVD output

This bit is set and cleared by hardware. It is valid only if PVD is enabled by the PVDE bit. 0: V DD is higher than the PVD threshold selected with the PLS[2:0] bits.

1: V DD is lower than the PVD threshold selected with the PLS[2:0] bits.

Note: The PVD is stopped by Standby mode. For this reason, this bit is equal to 0 after

Standby or reset until the PVDE bit is set.

Bit 1 SBF: Standby flag

This bit is set by hardware and cleared only by a POR/PDR (power-on reset/power-down reset) or by setting the CSBF bit in the PWR power control register (PWR_CR) for STM32F405xx/07xx and STM32F415xx/17xx

0: Device has not been in Standby mode

1: Device has been in Standby mode

Bit 0 WUF: Wakeup flag

This bit is set by hardware and cleared either by a system reset or by setting the CWUF bit in the PWR_CR register.

0: No wakeup event occurred

1: A wakeup event was received from the WKUP pin or from the RTC alarm (Alarm A or Alarm B), RTC Tamper event, RTC TimeStamp event or RTC Wakeup).

Note: An additional wakeup event is detected if the WKUP pin is enabled (by setting the

EWUP bit) when the WKUP pin level is already high.

148/1749 RM0090 Rev 18

--- Page 149 ---

**RM0090**

**Power controller (PWR)**


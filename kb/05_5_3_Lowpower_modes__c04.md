> [RM0090 ch.5 §5.3] Low-power modes**

### 5.3.5 Stop mode (STM32F42xxx and STM32F43xxx)**

**5.3.5 Stop mode (STM32F42xxx and STM32F43xxx)**

The Stop mode is based on the Cortex ® -M4 with FPU deepsleep mode combined with peripheral clock gating. The voltage regulator can be configured either in normal or low- power mode. In Stop mode, all clocks in the 1.2 V domain are stopped, the PLLs, the HSI and the HSE RC oscillators are disabled. Internal SRAM and register contents are preserved.

In Stop mode, the power consumption can be further reduced by using additional settings in the PWR_CR register. However this will induce an additional startup delay when waking up from Stop mode (see Table 28 ).



149

--- Page 134 ---

**Power controller (PWR) RM0090**

**Table 28. Stop operating modes (STM32F42xxx and STM32F43xxx)**

| Voltage Regulator Mode | UDEN[1:0] bits | MRUDS bit | LPUDS bit | LPDS bit | FPDS bit | Wakeup latency |  |
|---|---|---|---|---|---|---|---|
| Normal mode | STOP MR (Main Regulator) | - | 0 | - | 0 | HSI RC startup time |  |
| STOP MR- FPD | - | 0 | - | 0 | 1 | HSI RC startup time + Flash wakeup time from power- down mode |  |
| STOP LP | - | 0 | 1 | 0 | HSI RC startup time + regulator wakeup time from LP mode |  |  |
| STOP LP-FPD | - | 0 | 1 | HSI RC startup time + Flash wakeup time from power- down mode + regulator wakeup time from LP mode |  |  |  |
| Under- drive Mode | STOP UMR- FPD | 3 | 1 | - | 0 | - | HSI RC startup time + Flash wakeup time from power- down mode + Main regulator wakeup time from under-drive mode + Core logic to nominal mode |
| STOP ULP-FPD | 3 | - | 1 | - | HSI RC startup time + Flash wakeup time from power- down mode + regulator wakeup time from LP under-drive mode + Core logic to nominal mode |  |  |


**Entering Stop mode (STM32F42xxx and STM32F43xxx)**

The Stop mode is entered according to Section : Entering low-power mode , when the SLEEPDEEP bit in the Cortex ® -M4 with FPU System Control register is set.

Refer to Table 29 for details on how to enter the Stop mode.

When the microcontroller enters in Stop mode, the voltage scale 3 is automatically selected. To further reduce power consumption in Stop mode, the internal voltage regulator can be put in low voltage mode. This is configured by the LPDS, MRUDS, LPUDS and UDEN bits of the PWR power control register (PWR_CR) for STM32F405xx/07xx and STM32F415xx/17xx .

If Flash memory programming is ongoing, the Stop mode entry is delayed until the memory access is finished.

If an access to the APB domain is ongoing, The Stop mode entry is delayed until the APB access is finished.

If the Over-drive mode was enabled before entering Stop mode, it is automatically disabled during when the Stop mode is activated.



--- Page 135 ---

**RM0090 Power controller (PWR)**

In Stop mode, the following features can be selected by programming individual control bits:

• Independent watchdog (IWDG): the IWDG is started by writing to its Key register or by

hardware option. Once started it cannot be stopped except by a Reset. See Section 21.3 in Section 21: Independent watchdog (IWDG) .

• Real-time clock (RTC): this is configured by the RTCEN bit in the Section 7.3.20: RCC

Backup domain control register (RCC_BDCR)

• Internal RC oscillator (LSI RC): this is configured by the LSION bit in the

Section 7.3.21: RCC clock control & status register (RCC_CSR) .

• External 32.768 kHz oscillator (LSE OSC): this is configured by the LSEON bit in the

RCC Backup domain control register (RCC_BDCR) .

The ADC or DAC can also consume power during the Stop mode, unless they are disabled before entering it. To disable them, the ADON bit in the ADC_CR2 register and the ENx bit in the DAC_CR register must both be written to 0.

Note: Before entering Stop mode, it is recommended to enable the clock security system (CSS) feature to prevent external oscillator (HSE) failure from impacting the internal MCU behavior.

**Exiting Stop mode (STM32F42xxx and STM32F43xxx)**

The Stop mode is exited according to Section : Exiting low-power mode .

Refer to Table 29 for more details on how to exit Stop mode.

When exiting Stop mode by issuing an interrupt or a wakeup event, the HSI RC oscillator is selected as system clock.

If the Under-drive mode was enabled, it is automatically disabled after exiting Stop mode. When the voltage regulator operates in low voltage mode, an additional startup delay is incurred when waking up from Stop mode. By keeping the internal regulator ON during Stop mode, the consumption is higher although the startup time is reduced.

When the voltage regulator operates in Under-drive mode, an additional startup delay is induced when waking up from Stop mode.



149

--- Page 136 ---

**Power controller (PWR) RM0090**

**Table 29. Stop mode entry and exit (STM32F42xxx and STM32F43xxx)**

| Stop mode | Description |
|---|---|
| Mode entry | WFI (Wait for Interrupt) or WFE (Wait for Event) while: – No interrupt or event is pending, – SLEEPDEEP bit is set in Cortex®-M4 with FPU System Control register, – PDDS bit is cleared in Power Control register (PWR CR), _ – Select the voltage regulator mode by configuring LPDS, MRUDS, LPUDS and UDEN bits in PWR CR (see Table 28: Stop operating modes _ (STM32F42xxx and STM32F43xxx)). |
| On Return from ISR while: – No interrupt is pending, – SLEEPDEEP bit is set in Cortex®-M4 with FPU System Control register, and – SLEEPONEXIT = 1, and – PDDS is cleared in PWR CR1. _ |  |
| Note: To enter Stop mode, all EXTI Line pending bits (in Pending register (EXTI PR)), all peripheral interrupts pending bits, the RTC Alarm _ (Alarm A and Alarm B), RTC wakeup, RTC tamper, and RTC time stamp flags, must be reset. Otherwise, the Stop mode entry procedure is ignored and program execution continues. |  |
| Mode exit | If WFI or Return from ISR was used for entry: All EXTI lines configured in Interrupt mode (the corresponding EXTI Interrupt vector must be enabled in the NVIC). The interrupt source can be external interrupts or peripherals with wakeup capability. Refer to Table 61: Vector table for STM32F405xx/07xx and STM32F415xx/17xx on page 372. If WFE was used for entry and SEVONPEND = 0: All EXTI Lines configured in event mode. Refer to Section 12.2.3: Wakeup event management on page 380 If WFE was used for entry and SEVONPEND = 1: – Any EXTI lines configured in Interrupt mode (even if the corresponding EXTI Interrupt vector is disabled in the NVIC). The interrupt source can be external interrupts or peripherals with wakeup capability. Refer to Table 61: Vector table for STM32F405xx/07xx and STM32F415xx/17xx on page 372 and Table 62: Vector table for STM32F42xxx and STM32F43xxx. – Wakeup event: refer to Section 12.2.3: Wakeup event management on page 380. |
| Wakeup latency | Refer to Table 28: Stop operating modes (STM32F42xxx and STM32F43xxx) |



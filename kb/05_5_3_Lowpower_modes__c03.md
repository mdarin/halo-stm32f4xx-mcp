> [RM0090 ch.5 §5.3] Low-power modes**

### 5.3.4 Stop mode (STM32F405xx/07xx and STM32F415xx/17xx)**

**5.3.4 Stop mode (STM32F405xx/07xx and STM32F415xx/17xx)**

The Stop mode is based on the Cortex ® -M4 with FPU deepsleep mode combined with peripheral clock gating. The voltage regulator can be configured either in normal or low- power mode. In Stop mode, all clocks in the 1.2 V domain are stopped, the PLLs, the HSI and the HSE RC oscillators are disabled. Internal SRAM and register contents are preserved.

By setting the FPDS bit in the PWR_CR register, the Flash memory also enters power-down mode when the device enters Stop mode. When the Flash memory is in power-down mode, an additional startup delay is incurred when waking up from Stop mode (see Table 26: Stop operating modes  (STM32F405xx/07xx and STM32F415xx/17xx) and Section 5.4.1: PWR power control register (PWR_CR) for STM32F405xx/07xx and STM32F415xx/17xx ).



--- Page 131 ---

**RM0090 Power controller (PWR)**

**Table 26. Stop operating modes**

**(STM32F405xx/07xx and STM32F415xx/17xx)**

| Stop mode | LPDS bit | FPDS bit | Wake-up latency |
|---|---|---|---|
| STOP MR (Main regulator) | 0 | HSI RC startup time |  |
| STOP MR-FPD | 0 | 1 | HSI RC startup time + Flash wakeup time from Power Down mode |
| STOP LP | 1 | 0 | HSI RC startup time + regulator wakeup time from LP mode |
| STOP LP-FPD | 1 | HSI RC startup time + Flash wakeup time from Power Down mode + regulator wakeup time from LP mode |  |


**Entering Stop mode (for STM32F405xx/07xx and STM32F415xx/17xx)**

The Stop mode is entered according to Section : Entering low-power mode , when the SLEEPDEEP bit in the Cortex ® -M4 with FPU System Control register is set.

Refer to Table 27 for details on how to enter the Stop mode.

To further reduce power consumption in Stop mode, the internal voltage regulator can be put in low-power mode. This is configured by the LPDS bit of the PWR power control register (PWR_CR) for STM32F405xx/07xx and STM32F415xx/17xx and PWR power control register (PWR_CR) for STM32F42xxx and STM32F43xxx .

If Flash memory programming is ongoing, the Stop mode entry is delayed until the memory access is finished.

If an access to the APB domain is ongoing, The Stop mode entry is delayed until the APB access is finished.

In Stop mode, the following features can be selected by programming individual control bits:

• Independent watchdog (IWDG): the IWDG is started by writing to its Key register or by

hardware option. Once started it cannot be stopped except by a Reset. See Section 21.3 in Section 21: Independent watchdog (IWDG) .

• Real-time clock (RTC): this is configured by the RTCEN bit in the Section 7.3.20: RCC

Backup domain control register (RCC_BDCR)

• Internal RC oscillator (LSI RC): this is configured by the LSION bit in the

Section 7.3.21: RCC clock control & status register (RCC_CSR) .

• External 32.768 kHz oscillator (LSE OSC): this is configured by the LSEON bit in the

Section 7.3.20: RCC Backup domain control register (RCC_BDCR) .

The ADC or DAC can also consume power during the Stop mode, unless they are disabled before entering it. To disable them, the ADON bit in the ADC_CR2 register and the ENx bit in the DAC_CR register must both be written to 0.



149

--- Page 132 ---

**Power controller (PWR) RM0090**

Note: If the application needs to disable the external clock before entering Stop mode, the HSEON bit must first be disabled and the system clock switched to HSI.

Otherwise, if the HSEON bit is kept enabled while the external clock (external oscillator) can be removed before entering stop mode, the clock security system (CSS) feature must be enabled to detect any external oscillator failure and avoid a malfunction behavior when entering stop mode.

**Exiting Stop mode (for STM32F405xx/07xx and STM32F415xx/17xx)**

The Stop mode is exited according to Section : Exiting low-power mode .

Refer to Table 27 for more details on how to exit Stop mode.

When exiting Stop mode by issuing an interrupt or a wakeup event, the HSI RC oscillator is selected as system clock.

When the voltage regulator operates in low-power mode, an additional startup delay is incurred when waking up from Stop mode. By keeping the internal regulator ON during Stop mode, the consumption is higher although the startup time is reduced.

**Table 27. Stop mode entry and exit (for STM32F405xx/07xx and STM32F415xx/17xx)**

| Stop mode | Description |
|---|---|
| Mode entry | WFI (Wait for Interrupt) or WFE (Wait for Event) while: – No interrupt (for WFI) or event (for WFE) is pending, – SLEEPDEEP bit is set in Cortex®-M4 with FPU System Control register, – PDDS bit is cleared in Power Control register (PWR CR), _ – Select the voltage regulator mode by configuring LPDS bit in PWR CR. _ |
| On Return from ISR: – No interrupt is pending, – SLEEPDEEP bit is set in Cortex®-M4 with FPU System Control register, – SLEEPONEXIT = 1, – PDDS bit is cleared in Power Control register (PWR CR). _ |  |
| Note: To enter Stop mode, all EXTI Line pending bits (in Pending register (EXTI PR)), all peripheral interrupts pending bits, the RTC Alarm _ (Alarm A and Alarm B), RTC wakeup, RTC tamper, and RTC time stamp flags, must be reset. Otherwise, the Stop mode entry procedure is ignored and program execution continues. |  |




--- Page 133 ---

**RM0090 Power controller (PWR)**

**Table 27. Stop mode entry and exit (for STM32F405xx/07xx and STM32F415xx/17xx)**

| Stop mode | Description |
|---|---|
| Mode exit | If WFI or Return from ISR was used for entry: Any EXTI lines configured in Interrupt mode (the corresponding EXTI Interrupt vector must be enabled in the NVIC). The interrupt source can be external interrupts or peripherals with wakeup capability. Refer to Table 61: Vector table for STM32F405xx/07xx and STM32F415xx/17xx on page 372 and Table 62: Vector table for STM32F42xxx and STM32F43xxx. If WFE was used for entry and SEVONPEND = 0 Any EXTI lines configured in event mode. Refer to Section 12.2.3: Wakeup event management on page 380. If WFE was used for entry and SEVONPEND = 1: – Any EXTI lines configured in Interrupt mode (even if the corresponding EXTI Interrupt vector is disabled in the NVIC). The interrupt source can be an external interrupt or a peripheral with wakeup capability. Refer to Table 61: Vector table for STM32F405xx/07xx and STM32F415xx/17xx on page 372 and Table 62: Vector table for STM32F42xxx and STM32F43xxx. – Wakeup event: refer to Section 12.2.3: Wakeup event management on page 380. |
| Wakeup latency | Table 26: Stop operating modes (STM32F405xx/07xx and STM32F415xx/17xx) |



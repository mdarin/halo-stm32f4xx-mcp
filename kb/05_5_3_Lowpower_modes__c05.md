> [RM0090 ch.5 §5.3] Low-power modes**

### 5.3.6 Standby mode**

**5.3.6 Standby mode**

The Standby mode allows to achieve the lowest power consumption. It is based on the Cortex ® -M4 with FPU deepsleep mode, with the voltage regulator disabled. The 1.2 V domain is consequently powered off. The PLLs, the HSI oscillator and the HSE oscillator are also switched off. SRAM and register contents are lost except for registers in the backup domain (RTC registers, RTC backup register and backup SRAM), and Standby circuitry (see Figure 9 ).



--- Page 137 ---

**RM0090 Power controller (PWR)**

**Entering Standby mode**

The Standby mode is entered according to Section : Entering low-power mode , when the SLEEPDEEP bit in the Cortex ® -M4 with FPU System Control register is set.

Refer to Table 30 for more details on how to enter Standby mode.

In Standby mode, the following features can be selected by programming individual control bits:

• Independent watchdog (IWDG): the IWDG is started by writing to its Key register or by

hardware option. Once started it cannot be stopped except by a reset. See Section 21.3 in Section 21: Independent watchdog (IWDG) .

• Real-time clock (RTC): this is configured by the RTCEN bit in the backup domain

control register (RCC_BDCR)

• Internal RC oscillator (LSI RC): this is configured by the LSION bit in the Control/status

register (RCC_CSR).

• External 32.768 kHz oscillator (LSE OSC): this is configured by the LSEON bit in the

backup domain control register (RCC_BDCR)

**Exiting Standby mode**

The Standby mode is exited according to Section : Exiting low-power mode . The SBF status flag in PWR_CR (see Section 5.4.2: PWR power control/status register (PWR_CSR)  for STM32F405xx/07xx and STM32F415xx/17xx ) indicates that the MCU was in Standby mode. All registers are reset after wakeup from Standby except for PWR_CR.

Refer to Table 30 for more details on how to exit Standby mode.

**Table 30. Standby mode entry and exit**

| Standby mode | Description |
|---|---|
| Mode entry | WFI (Wait for Interrupt) or WFE (Wait for Event) while: – SLEEPDEEP is set in Cortex®-M4 with FPU System Control register, – PDDS bit is set in Power Control register (PWR CR), _ – No interrupt (for WFI) or event (for WFE) is pending, – WUF bit is cleared in Power Control register (PWR CR), _ – the RTC flag corresponding to the chosen wakeup source (RTC Alarm A, RTC Alarm B, RTC wakeup, Tamper or Timestamp flags) is cleared |
| On return from ISR while: – SLEEPDEEP bit is set in Cortex®-M4 with FPU System Control register, and – SLEEPONEXIT = 1, and – PDDS bit is set in Power Control register (PWR CR), and _ – No interrupt is pending, – WUF bit is cleared in Power Control/Status register (PWR SR), _ – The RTC flag corresponding to the chosen wakeup source (RTC Alarm A, RTC Alarm B, RTC wakeup, Tamper or Timestamp flags) is cleared. |  |
| Mode exit | WKUP pin rising edge, RTC alarm (Alarm A and Alarm B), RTC wakeup, tamper event, time stamp event, external reset in NRST pin, IWDG reset. |
| Wakeup latency | Reset phase. |




149

--- Page 138 ---

**Power controller (PWR) RM0090**

**I/O states in Standby mode**

In Standby mode, all I/O pins are high impedance except for:

• Reset pad (still available)

• RTC_AF1 pin (PC13) if configured for tamper, time stamp, RTC Alarm out, or RTC

clock calibration out

• WKUP pin (PA0), if enabled

**Debug mode**

By default, the debug connection is lost if the application puts the MCU in Stop or Standby mode while the debug features are used. This is due to the fact that the Cortex ® -M4 with FPU core is no longer clocked.

However, by setting some configuration bits in the DBGMCU_CR register, the software can be debugged even when using the low-power modes extensively. For more details, refer to Section 38.16.1: Debug support for low-power modes .


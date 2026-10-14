> [RM0090 ch.5 §5.3] Low-power modes**

### 5.3.3 Sleep mode**

**5.3.3 Sleep mode**

**Entering Sleep mode**

The Sleep mode is entered according to Section : Entering low-power mode , when the SLEEPDEEP bit in the Cortex ® -M4 with FPU System Control register is cleared.

Refer to Table 24 and Table 25 for details on how to enter Sleep mode.

Note: All interrupt pending bits must be cleared before the sleep mode entry.

**Exiting Sleep mode**

The Sleep mode is exited according to Section : Exiting low-power mode .

Refer to Table 24 and Table 25 for more details on how to exit Sleep mode.

**Table 24. Sleep-now entry and exit**

| Sleep-now mode | Description |
|---|---|
| Mode entry | WFI (Wait for Interrupt) or WFE (Wait for Event) while: – SLEEPDEEP = 0, and – No interrupt (for WFI) or event (for WFE) is pending. Refer to the Cortex®-M4 with FPU System Control register. |
| On Return from ISR while: – SLEEPDEEP = 0 and – SLEEPONEXIT = 1, – No interrupt is pending. Refer to the Cortex®-M4 with FPU System Control register. |  |




149

--- Page 130 ---

**Power controller (PWR) RM0090**

**Table 24. Sleep-now entry and exit (continued)**

| Sleep-now mode | Description |
|---|---|
| Mode exit | If WFI or Return from ISR was used for entry: Interrupt: Refer to Table 61: Vector table for STM32F405xx/07xx and STM32F415xx/17xx and Table 62: Vector table for STM32F42xxx and STM32F43xxx If WFE was used for entry and SEVONPEND = 0 Wakeup event: Refer to Section 12.2.3: Wakeup event management f WFE was used for entry and SEVONPEND = 1 Interrupt even when disabled in NVIC: refer to Table 61: Vector table for STM32F405xx/07xx and STM32F415xx/17xx and Table 62: Vector table for STM32F42xxx and STM32F43xxx or Wakeup event (see Section 12.2.3: Wakeup event management). |
| Wakeup latency | None |


**Table 25. Sleep-on-exit entry and exit**

| Sleep-on-exit | Description |
|---|---|
| Mode entry | WFI (Wait for Interrupt) or WFE (Wait for Event) while: – SLEEPDEEP = 0, and – No interrupt (for WFI) or event (for WFE) is pending. Refer to the Cortex®-M4 with FPU System Control register. |
| On Return from ISR while: – SLEEPDEEP = 0, and – SLEEPONEXIT = 1, and – No interrupt is pending. Refer to the Cortex®-M4 with FPU System Control register. |  |
| Mode exit | Interrupt: refer to Table 61: Vector table for STM32F405xx/07xx and STM32F415xx/17xx and Table 62: Vector table for STM32F42xxx and STM32F43xxx |
| Wakeup latency | None |



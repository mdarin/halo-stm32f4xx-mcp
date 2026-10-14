> [RM0090 ch.5 §5.3] Low-power modes**

### 5.3 Low-power modes**

**5.3 Low-power modes**

By default, the microcontroller is in Run mode after a system or a power-on reset. In Run

| the CPU are ava le when | is clock ilable to waiting |
|---|---|
| st comp p sourc evices fe | romise b es. ature th |


| the pro en the C vent. It i | gram co PU does s up to th | de is ex not nee e user t | ecuted. d to be o select | Several kept run the mod | low-pow ning, for e that gi |
|---|---|---|---|---|---|
| er cons odes: | umption, | short st | artup tim | e and a | vailable |


• Sleep mode (Cortex ®

-M4 with FPU core stopped, peripherals kept running)

• Stop mode (all clocks are stopped)

• Standby mode (1.2 V domain powered off)



--- Page 127 ---

**RM0090 Power controller (PWR)**

In addition, the power consumption in Run mode can be reduce by one of the following means:

• Slowing down the system clocks

• Gating the clocks to the APBx and AHBx peripherals when they are unused.

**Entering low-power mode**

Low-power modes are entered by the MCU by executing the WFI (Wait For Interrupt), or WFE (Wait for Event) instructions, or when the SLEEPONEXIT bit in the Cortex ® -M4 with FPU System Control register is set on Return from ISR.

Entering Low-power mode through WFI or WFE will be executed only if no interrupt is pending or no event is pending.

**Exiting low-power mode**

The MCU exits from Sleep and Stop modes low-power mode depending on the way the low- power mode was entered:

• If the WFI instruction or Return from ISR was used to enter the low-power mode, any peripheral interrupt acknowledged by the NVIC can wake up the device.

• If the WFE instruction is used to enter the low-power mode, the MCU exits the low- power mode as soon as an event occurs. The wakeup event can be generated either by:

– NVIC IRQ interrupt:

When SEVONPEND = 0 in the Cortex ® -M4 with FPU System Control register: by enabling an interrupt in the peripheral control register and in the NVIC. When the MCU resumes from WFE, the peripheral interrupt pending bit and the NVIC peripheral IRQ channel pending bit (in the NVIC interrupt clear pending register) have to be cleared. Only NVIC interrupts with sufficient priority will wakeup and interrupt the MCU.

When SEVONPEND = 1 in the Cortex ® -M4 with FPU System Control register: by enabling an interrupt in the peripheral control register and optionally in the NVIC. When the MCU resumes from WFE, the peripheral interrupt pending bit and when enabled the NVIC peripheral IRQ channel pending bit (in the NVIC interrupt clear pending register) have to be cleared. All NVIC interrupts will wakeup the MCU, even the disabled ones.Only enabled NVIC interrupts with sufficient priority will wakeup and interrupt the MCU.

– Event

This is done by configuring a EXTI line in event mode. When the CPU resumes from WFE, it is not necessary to clear the EXTI peripheral interrupt pending bit or the NVIC IRQ channel pending bit as the pending bits corresponding to the event line is not set. It may be necessary to clear the interrupt flag in the peripheral. The MCU exits from Standby low-power mode through an external reset (NRST pin), an IWDG reset, a rising edge on one of the enabled WKUPx pins or a RTC event occurs (see Figure 237: RTC block diagram ).

After waking up from Standby mode, program execution restarts in the same way as after a Reset (boot pin sampling, option bytes loading, reset vector is fetched, etc.).



149

--- Page 128 ---

**Power controller (PWR) RM0090**

Only enabled NVIC interrupts with sufficient priority will wakeup and interrupt the MCU.

**Table 23. Low-power mode summary**

| Mode name | Entry | Wakeup | Effect on 1.2 V domain clocks | Effect on V DD domain clocks | Voltage regulator |
|---|---|---|---|---|---|
| Sleep (Sleep now or Sleep-on- exit) | WFI or Return from ISR | Any interrupt | CPU CLK OFF no effect on other clocks or analog clock sources | None | ON |
| WFE | Wakeup event |  |  |  |  |
| Stop | PDDS and LPDS bits + SLEEPDEEP bit + WFI, Return from ISR or WFE | Any EXTI line (configured in the EXTI registers, internal and external lines) | All 1.2 V domain clocks OFF | HSI and HSE oscillator s OFF | ON or in low- power mode (depends on PWR power control register (PWR CR) _ for STM32F405xx/07x x and STM32F415xx/17x x and PWR power control register (PWR CR) for _ STM32F405xx/07x x and STM32F415xx/17x xPWR power control register (PWR CR) for _ STM32F42xxx and STM32F43xxx |
| Standby | PDDS bit + SLEEPDEEP bit + WFI, Return from ISR or WFE | WKUP pin rising edge, RTC alarm (Alarm A or Alarm B), RTC Wakeup event, RTC tamper events, RTC time stamp event, external reset in NRST pin, IWDG reset | OFF |  |  |


> [RM0090 ch.5 §5.3] Low-power modes**

### 5.3.1 Slowing down system clocks**

**5.3.1 Slowing down system clocks**

In Run mode the speed of the system clocks (SYSCLK, HCLK, PCLK1, PCLK2) can be reduced by programming the prescaler registers. These prescalers can also be used to slow down peripherals before entering Sleep mode.

For more details refer to Section 7.3.3: RCC clock configuration register (RCC_CFGR) .

> [RM0090 ch.5 §5.3] Low-power modes**

### 5.3.2 Peripheral clock gating**

**5.3.2 Peripheral clock gating**

In Run mode, the HCLKx and PCLKx for individual peripherals and memories can be stopped at any time to reduce power consumption.

To further reduce power consumption in Sleep mode the peripheral clocks can be disabled prior to executing the WFI or WFE instructions.



--- Page 129 ---

**RM0090 Power controller (PWR)**

Peripheral clock gating is controlled by the AHB1 peripheral clock enable register (RCC_AHB1ENR), AHB2 peripheral clock enable register (RCC_AHB2ENR), AHB3 peripheral clock enable register (RCC_AHB3ENR) (see Section 7.3.10: RCC AHB1 peripheral clock enable register (RCC_AHB1ENR) , Section 7.3.11: RCC AHB2 peripheral clock enable register (RCC_AHB2ENR) , Section 7.3.12: RCC AHB3 peripheral clock enable register (RCC_AHB3ENR) for STM32F405xx/07xx and STM32F415xx/17xx, and Section 6.3.10: RCC AHB1 peripheral clock register (RCC_AHB1ENR) , Section 6.3.11: RCC AHB2 peripheral clock enable register (RCC_AHB2ENR) , and Section 6.3.12: RCC AHB3 peripheral clock enable register (RCC_AHB3ENR) for STM32F42xxx and STM32F43xxx).

Disabling the peripherals clocks in Sleep mode can be performed automatically by resetting the corresponding bit in RCC_AHBxLPENR and RCC_APBxLPENR registers.


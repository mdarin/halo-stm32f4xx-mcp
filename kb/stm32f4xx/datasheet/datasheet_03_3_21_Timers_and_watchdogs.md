### 3.21 Timers and watchdogs

The STM32F405xx and STM32F407xx devices include two advanced-control timers, eight general-purpose timers, two basic timers and two watchdog timers.

All timer counters can be frozen in debug mode.

Table 4 compares the features of the advanced-control, general-purpose and basic timers.

**Table 4. Timer feature comparison**

| Timer type | Timer | Counter resolution | Counter type | Prescaler factor | DMA request generation | Capture/compare channels | Complementary output | Max interface clock (MHz) | Max timer clock (MHz) |
|---|---|---|---|---|---|---|---|---|---|
| Advanced-control | TIM1, TIM8 | 16-bit | Up, Down, Up/down | Any integer between 1 and 65536 | Yes | 4 | Yes | 84 | 168 |


STI
DS8626 Rev 12
33/206

--- Page 34 ---

Functional overview
STM32F405xx, STM32F407xx

Table 4. Timer feature comparison (continued)

| Timer type | Timer | Counter resolution | Counter type | Prescaler factor | DMA request generation | Capture/compare channels | Complementary output | Max interface clock (MHz) | Max timer clock (MHz) |
|---|---|---|---|---|---|---|---|---|---|
| General purpose | TIM2, TIM5 | 32-bit | Up, Down, Up/down | Any integer between 1 and 65536 | Yes | 4 | No | 42 | 84 |
| TIM3, TIM4 | 16-bit | Up, Down, Up/down | Any integer between 1 and 65536 | Yes | 4 | No | 42 | 84 |  |
| TIM9 | 16-bit | Up | Any integer between 1 and 65536 | No | 2 | No | 84 | 168 |  |
| TIM10, TIM11 | 16-bit | Up | Any integer between 1 and 65536 | No | 1 | No | 84 | 168 |  |
| TIM12 | 16-bit | Up | Any integer between 1 and 65536 | No | 2 | No | 42 | 84 |  |
| TIM13, TIM14 | 16-bit | Up | Any integer between 1 and 65536 | No | 1 | No | 42 | 84 |  |
| Basic | TIM6, TIM7 | 16-bit | Up | Any integer between 1 and 65536 | Yes | 0 | No | 42 | 84 |


### 3.21.1 Advanced-control timers (TIM1, TIM8)

The advanced-control timers (TIM1, TIM8) can be seen as three-phase PWM generators multiplexed on 6 channels. They have complementary PWM outputs with programmable inserted dead times. They can also be considered as complete general-purpose timers. Their 4 independent channels can be used for:

*   Input capture
*   Output compare
*   PWM generation (edge- or center-aligned modes)
*   One-pulse mode output

If configured as standard 16-bit timers, they have the same features as the general-purpose TIMx timers. If configured as 16-bit PWM generators, they have full modulation capability (0-100%).

The advanced-control timer can work together with the TIMx timers via the Timer Link feature for synchronization or event chaining.

TIM1 and TIM8 support independent DMA request generation.

34/206
DS8626 Rev 12
ST

--- Page 35 ---

**STM32F405xx, STM32F407xx**
**Functional overview**

### 3.21.2 General-purpose timers (TIMx)

There are ten synchronizable general-purpose timers embedded in the STM32F40xxx devices (see Table 4 for differences).

*   **TIM2, TIM3, TIM4, TIM5**
    The STM32F40xxx include 4 full-featured general-purpose timers: TIM2, TIM5, TIM3, and TIM4. The TIM2 and TIM5 timers are based on a 32-bit auto-reload up/downcounter and a 16-bit prescaler. The TIM3 and TIM4 timers are based on a 16-bit auto-reload up/downcounter and a 16-bit prescaler. They all feature 4 independent channels for input capture/output compare, PWM or one-pulse mode output. This gives up to 16 input capture/output compare/PWMs on the largest packages.

    The TIM2, TIM3, TIM4, TIM5 general-purpose timers can work together, or with the other general-purpose timers and the advanced-control timers TIM1 and TIM8 via the Timer Link feature for synchronization or event chaining.

    Any of these general-purpose timers can be used to generate PWM outputs.

    TIM2, TIM3, TIM4, TIM5 all have independent DMA request generation. They are capable of handling quadrature (incremental) encoder signals and the digital outputs from 1 to 4 hall-effect sensors.

*   **TIM9, TIM10, TIM11, TIM12, TIM13, and TIM14**
    These timers are based on a 16-bit auto-reload upcounter and a 16-bit prescaler. TIM10, TIM11, TIM13, and TIM14 feature one independent channel, whereas TIM9 and TIM12 have two independent channels for input capture/output compare, PWM or one-pulse mode output. They can be synchronized with the TIM2, TIM3, TIM4, TIM5 full-featured general-purpose timers. They can also be used as simple time bases.

### 3.21.3 Basic timers TIM6 and TIM7

These timers are mainly used for DAC trigger and waveform generation. They can also be used as a generic 16-bit time base.

TIM6 and TIM7 support independent DMA request generation.

**Independent watchdog**

The independent watchdog is based on a 12-bit downcounter and 8-bit prescaler. It is clocked from an independent 32 kHz internal RC and as it operates independently from the main clock, it can operate in Stop and Standby modes. It can be used either as a watchdog to reset the device when a problem occurs, or as a free-running timer for application timeout management. It is hardware- or software-configurable through the option bytes.

**Window watchdog**

The window watchdog is based on a 7-bit downcounter that can be set as free-running. It can be used as a watchdog to reset the device when a problem occurs. It is clocked from the main clock. It has an early warning interrupt capability and the counter can be frozen in debug mode.

ST
DS8626 Rev 12
35/206

--- Page 36 ---

Functional overview
STM32F405xx, STM32F407xx

**SysTick timer**

This timer is dedicated to real-time operating systems, but could also be used as a standard downcounter. It features:

*   A 24-bit downcounter
*   Autoreload capability
*   Maskable system interrupt generation when the counter reaches 0
*   Programmable clock source.

### 3.22 Inter-integrated circuit interface (I²C)

Up to three I²C bus interfaces can operate in multimaster and slave modes. They can support the Standard-mode (up to 100 kHz) and Fast-mode (up to 400 kHz). They support the 7/10-bit addressing mode and the 7-bit dual addressing mode (as slave). A hardware CRC generation/verification is embedded.

They can be served by DMA and they support SMBus 2.0/PMBus.


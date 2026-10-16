## 7.2 Clocks

Three different clock sources can be used to drive the system clock (SYSCLK):
* HSI oscillator clock
* HSE oscillator clock
* Main PLL (PLL) clock

The devices have the two following secondary clock sources:
* 32 kHz low-speed internal RC (LSI RC) which drives the independent watchdog and, optionally, the RTC used for Auto-wakeup from the Stop/Standby mode.
* 32.768 kHz low-speed external crystal (LSE crystal) which optionally drives the RTC clock (RTCCLK)

Each clock source can be switched on or off independently when it is not used, to optimize power consumption.

ST
RM0090 Rev 18
215/1749

--- Page 21 ---

**Reset and clock control for STM32F405xx/07xx and STM32F415xx/17xx(RCC)** RM0090

**Figure 21. Clock tree**

LSI RC 32 kHz
LSI
Watchdog enable
IWDGCLK
to independent watchdog

OSC32_IN
OSC32_OUT
LSE OSC 32.768 kHz
LSE
RTCSEL[1:0]
RTC enable
RTCCLK
to RTC

MCO2
/1 to 5
SYSCLK
LSE
HSE_RTC

MCO1
/1 to 5

/2 to 31

16 MHz HS I RC
HSI
SW
HSE
PLLCLK
AHB PRESC /1,2...512
SYSCLK 168 MHz max

Peripheral clock enable
Ethernet PTP clock

168 MHz max.
Clock Enable
/8
HCLK to AHB bus, core, memory and DMA to Cortex System timer
FCLK Cortex free-running clock

OSC_OUT
OSC_IN
4-26 MHz HSE OSC
HSE

APBx PRESC /1,2,4,8,16
Peripheral clock enable
APBx peripheral clocks
Peripheral clock enable
APBx timer clocks
if {APBx presc = 1x1 else x2}

PLL
VCO
/P
xN
/Q
/R
/M
PLL48CK
Peripheral clock enable
48 MHz clocks

I2S_CKIN
PLL I2S
VCO
/P
xN
/Q
/R
PLL I2SCLK
I2SSRC
Ext. clock
Peripheral clock enable
I2S clocks

PHY Ethernet 25 to 50 MHz
ETH_MII_TX_CLK_MII
Peripheral clock enable
MACTXCLK
/2,20
MII_RMII_SEL in SYSCFG_PMC
to Ethernet MAC
ETH_MII_RX_CLK_MII
Peripheral clock enable
MACRXCLK
Peripheral clock enable
MACRMIIK

USB2.0 PHY 24 to 60 MHz
OTG_HS_ULPI_CK
Peripheral clock enable
USBHS ULPI clock

ai16088d

1. For full details about the internal and external clock source characteristics, refer to the Electrical characteristics section in the device datasheet.

216/1749 RM0090 Rev 18 ST

--- Page 22 ---

RM0090
**Reset and clock control for STM32F405xx/07xx and STM32F415xx/17xx(RCC)**

The clock controller provides a high degree of flexibility to the application in the choice of the external crystal or the oscillator to run the core and peripherals at the highest frequency and, guarantee the appropriate frequency for peripherals that need a specific clock like Ethernet, USB OTG FS and HS, I2S and SDIO.

Several prescalers are used to configure the AHB frequency, the high-speed APB (APB2) and the low-speed APB (APB1) domains. The maximum frequency of the AHB domain is 168 MHz. The maximum allowed frequency of the high-speed APB2 domain is 84 MHz. The maximum allowed frequency of the low-speed APB1 domain is 42 MHz

All peripheral clocks are derived from the system clock (SYSCLK) except for:

*   The USB OTG FS clock (48 MHz), the random analog generator (RNG) clock (≤48 MHz) and the SDIO clock (≤48 MHz) which are coming from a specific output of PLL (PLL48CLK)
*   The I2S clock
    To achieve high-quality audio performance, the I2S clock can be derived either from a specific PLL (PLL2S) or from an external clock mapped on the I2S_CKIN pin. For more information about I2S clock frequency and precision, refer to Section 28.4.4: Clock generator.
*   The USB OTG HS (60 MHz) clock which is provided from the external PHY
*   The Ethernet MAC clocks (TX, RX and RMII) which are provided from the external PHY. For further information on the Ethernet configuration, please refer to Section 33.4.4: MII/RMII selection in the Ethernet peripheral description. When the Ethernet is used, the AHB clock frequency must be at least 25 MHz.

The RCC feeds the external clock of the Cortex System Timer (SysTick) with the AHB clock (HCLK) divided by 8. The SysTick can work either with this clock or with the Cortex clock (HCLK), configurable in the SysTick control and status register.

The timer clock frequencies are automatically set by hardware. There are two cases:

1.  If the APB prescaler is 1, the timer clock frequencies are set to the same frequency as that of the APB domain to which the timers are connected.
2.  Otherwise, they are set to twice (×2) the frequency of the APB domain to which the timers are connected.

FCLK acts as Cortex®-M4 with FPU free-running clock. For more details, refer to the Cortex®-M4 with FPU technical reference manual.

### 7.2.1 HSE clock

The high speed external clock signal (HSE) can be generated from two possible clock sources:

*   HSE external crystal/ceramic resonator
*   HSE external user clock

The resonator and the load capacitors have to be placed as close as possible to the oscillator pins in order to minimize output distortion and startup stabilization time. The loading capacitance values must be adjusted according to the selected oscillator.

ST
RM0090 Rev 18
217/1749

--- Page 23 ---

Reset and clock control for STM32F405xx/07xx and STM32F415xx/17xx(RCC) RM0090

Figure 22. HSE/ LSE clock sources

|  | Hardware configuration |
|---|---|
| External clock | OSC_OUT (HI-Z) External source |
| Crystal/ceramic resonators | OSC_IN OSC_OUT CL1 Load capacitors CL2 |


**External source (HSE bypass)**

In this mode, an external clock source must be provided. You select this mode by setting the HSEBYP and HSEON bits in the *RCC clock control register (RCC_CR)*. The external clock signal (square, sinus or triangle) with ~50% duty cycle has to drive the OSC_IN pin while the OSC_OUT pin should be left HI-Z. See Figure 22.

**External crystal/ceramic resonator (HSE crystal)**

The HSE has the advantage of producing a very accurate rate on the main clock.

The associated hardware configuration is shown in Figure 22. Refer to the electrical characteristics section of the datasheet for more details.

The HSERDY flag in the *RCC clock control register (RCC_CR)* indicates if the high-speed external oscillator is stable or not. At startup, the clock is not released until this bit is set by hardware. An interrupt can be generated if enabled in the *RCC clock interrupt register (RCC_CIR)*.

The HSE Crystal can be switched on and off using the HSEON bit in the *RCC clock control register (RCC_CR)*.

**7.2.2 HSI clock**

The HSI clock signal is generated from an internal 16 MHz RC oscillator and can be used directly as a system clock, or used as PLL input.

The HSI RC oscillator has the advantage of providing a clock source at low cost (no external components). It also has a faster startup time than the HSE crystal oscillator however, even with calibration the frequency is less accurate than an external crystal oscillator or ceramic resonator.

218/1749 RM0090 Rev 18 ST

--- Page 24 ---

RM0090 Reset and clock control for STM32F405xx/07xx and STM32F415xx/17xx(RCC)

**Calibration**

RC oscillator frequencies can vary from one chip to another due to manufacturing process variations, this is why each device is factory calibrated by ST for 1% accuracy at $T_A$= 25 °C.

After reset, the factory calibration value is loaded in the HSICAL[7:0] bits in the **RCC clock control register (RCC_CR)**.

If the application is subject to voltage or temperature variations this may affect the RC oscillator speed. You can trim the HSI frequency in the application using the HSITRIM[4:0] bits in the **RCC clock control register (RCC_CR)**.

The HSIRDY flag in the **RCC clock control register (RCC_CR)** indicates if the HSI RC is stable or not. At startup, the HSI RC output clock is not released until this bit is set by hardware.

The HSI RC can be switched on and off using the HSION bit in the **RCC clock control register (RCC_CR)**.

The HSI signal can also be used as a backup source (Auxiliary clock) if the HSE crystal oscillator fails. Refer to Section 7.2.7: Clock security system (CSS) on page 220.

### 7.2.3 PLL configuration

The STM32F4xx devices feature two PLLs:

*   A main PLL (PLL) clocked by the HSE or HSI oscillator and featuring two different output clocks:
    *   The first output is used to generate the high speed system clock (up to 168 MHz)
    *   The second output is used to generate the clock for the USB OTG FS (48 MHz), the random analog generator (≤48 MHz) and the SDIO (≤48 MHz).
*   A dedicated PLL (PLL2S) used to generate an accurate clock to achieve high-quality audio performance on the I2S interface.

Since the main-PLL configuration parameters cannot be changed once PLL is enabled, it is recommended to configure PLL before enabling it (selection of the HSI or HSE oscillator as PLL clock source, and configuration of division factors M, N, P, and Q).

The PLL2S uses the same input clock as PLL (PLLM[5:0] and PLLSRC bits are common to both PLLs). However, the PLL2S has dedicated enable/disable and division factors (N and R) configuration bits. Once the PLL2S is enabled, the configuration parameters cannot be changed.

The two PLLs are disabled by hardware when entering Stop and Standby modes, or when an HSE failure occurs when HSE or PLL (clocked by HSE) are used as system clock. **RCC PLL configuration register (RCC_PLLCFGR)** and **RCC clock configuration register (RCC_CFGR)** can be used to configure PLL and PLL2S, respectively.

### 7.2.4 LSE clock

The LSE clock is generated from a 32.768 kHz low-speed external crystal or ceramic resonator. It has the advantage providing a low-power but highly accurate clock source to the real-time clock peripheral (RTC) for clock/calendar or other timing functions.

The LSE oscillator is switched on and off using the LSEON bit in **RCC Backup domain control register (RCC_BDCR)**.

ST
RM0090 Rev 18 219/1749

--- Page 25 ---

**Reset and clock control for STM32F405xx/07xx and STM32F415xx/17xx(RCC)** RM0090

The LSERDY flag in the *RCC Backup domain control register (RCC_BDCR)* indicates if the LSE crystal is stable or not. At startup, the LSE crystal output clock signal is not released until this bit is set by hardware. An interrupt can be generated if enabled in the *RCC clock interrupt register (RCC_CIR)*.

**External source (LSE bypass)**

In this mode, an external clock source must be provided. It must have a frequency up to 1 MHz. You select this mode by setting the LSEBYP and LSEON bits in the *RCC Backup domain control register (RCC_BDCR)*. The external clock signal (square, sinus or triangle) with ~50% duty cycle has to drive the OSC32_IN pin while the OSC32_OUT pin should be left HI-Z. See Figure 22.

**7.2.5 LSI clock**

The LSI RC acts as an low-power clock source that can be kept running in Stop and Standby mode for the independent watchdog (IWDG) and Auto-wakeup unit (AWU). The clock frequency is around 32 kHz. For more details, refer to the electrical characteristics section of the datasheets.

The LSI RC can be switched on and off using the LSION bit in the *RCC clock control & status register (RCC_CSR)*.

The LSIRDY flag in the *RCC clock control & status register (RCC_CSR)* indicates if the low-speed internal oscillator is stable or not. At startup, the clock is not released until this bit is set by hardware. An interrupt can be generated if enabled in the *RCC clock interrupt register (RCC_CIR)*.

**7.2.6 System clock (SYSCLK) selection**

After a system reset, the HSI oscillator is selected as the system clock. When a clock source is used directly or through PLL as the system clock, it is not possible to stop it.

A switch from one clock source to another occurs only if the target clock source is ready (clock stable after startup delay or PLL locked). If a clock source that is not yet ready is selected, the switch occurs when the clock source is ready. Status bits in the *RCC clock control register (RCC_CR)* indicate which clock(s) is (are) ready and which clock is currently used as the system clock.

**7.2.7 Clock security system (CSS)**

The clock security system can be activated by software. In this case, the clock detector is enabled after the HSE oscillator startup delay, and disabled when this oscillator is stopped.

If a failure is detected on the HSE clock, this oscillator is automatically disabled, a clock failure event is sent to the break inputs of advanced-control timers TIM1 and TIM8, and an interrupt is generated to inform the software about the failure (clock security system interrupt CSSI), allowing the MCU to perform rescue operations. The CSSI is linked to the Cortex®-M4 with FPU NMI (non-maskable interrupt) exception vector.

**Note:** When the CSS is enabled, if the HSE clock happens to fail, the CSS generates an interrupt, which causes the automatic generation of an NMI. The NMI is executed indefinitely unless the CSS interrupt pending bit is cleared. As a consequence, the application has to clear the CSS interrupt in the NMI ISR by setting the CSSC bit in the Clock interrupt register (RCC_CIR).

220/1749 RM0090 Rev 18

--- Page 26 ---

RM0090
Reset and clock control for STM32F405xx/07xx and STM32F415xx/17xx(RCC)

If the HSE oscillator is used directly or indirectly as the system clock (indirectly meaning that it is directly used as PLL input clock, and that PLL clock is the system clock) and a failure is detected, then the system clock switches to the HSI oscillator and the HSE oscillator is disabled.

If the HSE oscillator clock was the clock source of PLL used as the system clock when the failure occurred, PLL is also disabled. In this case, if the PLLI2S was enabled, it is also disabled when the HSE fails.

### 7.2.8 RTC/AWU clock

Once the RTCCLK clock source has been selected, the only possible way of modifying the selection is to reset the power domain.

The RTCCLK clock source can be either the HSE 1 MHz (HSE divided by a programmable prescaler), the LSE or the LSI clock. This is selected by programming the RTCSEL[1:0] bits in the **RCC Backup domain control register (RCC_BDCR)** and the RTCPRE[4:0] bits in **RCC clock configuration register (RCC_CFGR)**. This selection cannot be modified without resetting the Backup domain.

If the LSE is selected as the RTC clock, the RTC operates normally if the backup or the system supply disappears. If the LSI is selected as the AWU clock, the AWU state is not guaranteed if the system supply disappears. If the HSE oscillator divided by a value between 2 and 31 is used as the RTC clock, the RTC state is not guaranteed if the backup or the system supply disappears.

The LSE clock is in the Backup domain, whereas the HSE and LSI clocks are not. As a consequence:

*   If LSE is selected as the RTC clock:
    *   The RTC continues to work even if the VDD supply is switched off, provided the VBAT supply is maintained.
*   If LSI is selected as the Auto-wakeup unit (AWU) clock:
    *   The AWU state is not guaranteed if the VDD supply is powered off. Refer to Section 7.2.5: LSI clock on page 220 for more details on LSI calibration.
*   If the HSE clock is used as the RTC clock:
    *   The RTC state is not guaranteed if the VDD supply is powered off or if the internal voltage regulator is powered off (removing power from the 1.2 V domain).

**Note:**
To read the RTC calendar register when the APB1 clock frequency is less than seven times the RTC clock frequency ($f_{APB1} < 7f_{RTCCLK}$), the software must read the calendar time and date registers twice. The data are correct if the second read access to RTC_TR gives the same result than the first one. Otherwise a third read access must be performed.

### 7.2.9 Watchdog clock

If the independent watchdog (IWDG) is started by either hardware option or software access, the LSI oscillator is forced ON and cannot be disabled. After the LSI oscillator temporization, the clock is provided to the IWDG.

ST
RM0090 Rev 18
221/1749

--- Page 27 ---

**Reset and clock control for STM32F405xx/07xx and STM32F415xx/17xx(RCC)** RM0090

### 7.2.10 Clock-out capability

Two microcontroller clock output (MCO) pins are available:

*   **MCO1**
    You can output four different clock sources onto the MCO1 pin (PA8) using the configurable prescaler (from 1 to 5):
    *   HSI clock
    *   LSE clock
    *   HSE clock
    *   PLL clock
    The desired clock source is selected using the MCO1PRE[2:0] and MCO1[1:0] bits in the *RCC clock configuration register (RCC_CFGR)*.

*   **MCO2**
    You can output four different clock sources onto the MCO2 pin (PC9) using the configurable prescaler (from 1 to 5):
    *   HSE clock
    *   PLL clock
    *   System clock (SYSCLK)
    *   PLLI2S clock
    The desired clock source is selected using the MCO2PRE[2:0] and MCO2 bits in the *RCC clock configuration register (RCC_CFGR)*.

For the different MCO pins, the corresponding GPIO port has to be programmed in alternate function mode.

The selected clock to output onto MCO must not exceed 100 MHz (the maximum I/O speed).

### 7.2.11 Internal/external clock measurement using TIM5/TIM11

It is possible to indirectly measure the frequencies of all on-board clock source generators by means of the input capture of TIM5 channel4 and TIM11 channel1 as shown in Figure 23 and Figure 24.

**Internal/external clock measurement using TIM5 channel4**

TIM5 has an input multiplexer which allows choosing whether the input capture is triggered by the I/O or by an internal clock. This selection is performed through the TI4_RMP [1:0] bits in the TIM5_OR register.

The primary purpose of having the LSE connected to the channel4 input capture is to be able to precisely measure the HSI (this requires to have the HSI used as the system clock source). The number of HSI clock counts between consecutive edges of the LSE signal provides a measurement of the internal clock period. Taking advantage of the high precision of LSE crystals (typically a few tens of ppm) we can determine the internal clock frequency with the same resolution, and trim the source to compensate for manufacturing-process and/or temperature- and voltage-related frequency deviations.

The HSI oscillator has dedicated, user-accessible calibration bits for this purpose.

222/1749 RM0090 Rev 18 ST

--- Page 28 ---

RM0090
**Reset and clock control for STM32F405xx/07xx and STM32F415xx/17xx(RCC)**

The basic concept consists in providing a relative measurement (e.g. HSI/LSE ratio): the precision is therefore tightly linked to the ratio between the two clock sources. The greater the ratio, the better the measurement.

It is also possible to measure the LSI frequency: this is useful for applications that do not have a crystal. The ultralow-power LSI oscillator has a large manufacturing process deviation: by measuring it versus the HSI clock source, it is possible to determine its frequency with the precision of the HSI. The measured value can be used to have more accurate RTC time base timeouts (when LSI is used as the RTC clock source) and/or an IWDG timeout with an acceptable accuracy.

Use the following procedure to measure the LSI frequency:

1. Enable the TIM5 timer and configure channel4 in Input capture mode.
2. Set the TI4_RMP bits in the TIM5_OR register to 0x01 to connect the LSI clock internally to TIM5 channel4 input capture for calibration purposes.
3. Measure the LSI clock frequency using the TIM5 capture/compare 4 event or interrupt.
4. Use the measured LSI frequency to update the prescaler of the RTC depending on the desired time base and/or to compute the IWDG timeout.

**Figure 23. Frequency measurement with TIM5 in Input capture mode**

TIM5
TI4
TI4_RMP[1:0]
GPIO
RTC_WakeUp_IT
LSE
LSI
ai17741V2

**Internal/external clock measurement using TIM11 channel1**

TIM11 has an input multiplexer which allows choosing whether the input capture is triggered by the I/O or by an internal clock. This selection is performed through TI1_RMP [1:0] bits in the TIM11_OR register. The HSE_RTC clock (HSE divided by a programmable prescaler) is connected to channel 1 input capture to have a rough indication of the external crystal frequency. This requires that the HSI is the system clock source. This can be useful for instance to ensure compliance with the IEC 60730/IEC 61335 standards which require to be able to determine harmonic or subharmonic frequencies (–50/+100% deviations).

**Figure 24. Frequency measurement with TIM11 in Input capture mode**

TIM11
TI1
TI1_RMP[1:0]
GPIO
HSE_RTC(1 MHz)
ai18433

ST
RM0090 Rev 18
223/1749

--- Page 29 ---

**Reset and clock control for STM32F405xx/07xx and STM32F415xx/17xx(RCC)** RM0090


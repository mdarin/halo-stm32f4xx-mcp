**6.2 Clocks**

Three different clock sources can be used to drive the system clock (SYSCLK):

• HSI oscillator clock

• HSE oscillator clock

• Main PLL (PLL) clock

The devices have the two following secondary clock sources:

• 32 kHz low-speed internal RC (LSI RC) which drives the independent watchdog and,

optionally, the RTC used for Auto-wakeup from the Stop/Standby mode.

• 32.768 kHz low-speed external crystal (LSE crystal) which optionally drives the RTC

clock (RTCCLK)



212

--- Page 152 ---

**Reset and clock control for   STM32F42xxx and STM32F43xxx (RCC)**

**RM0090**

Each clock source can be switched on or off independently when it is not used, to optimize

power consumption.

**Figure 16. Clock tree**

Watchdog

IWDGCLK To Independent

LSI RC LSI enable

watchdog

32 kHz

RTCSEL[1:0]

RTC

RTCCLK to RTC

OSC32_IN LSE OSC LSE enable

32.768 kHz

OSC32_OUT

SYSCLK

MCO2 /1 to 5

LSE HSE_RTC

| VCO /P /Q xN PLL /R |  |
|---|---|
| /P |  |
| /Q | P |
| /R |  |
| VCO xN | /P |
| /Q |  |


Peripheral Ethernet clock enable PTP clock 180 MHz max. HCLK

MCO1 /2 to 31

enable Clock to AHB bus, core, memory and DMA /8 to Cortex System timer

16 MHz HSI

HSI RC

SW

HSI

AHB FCLK Cortex PRESC free-running clock /1,2,..512



SYSCLK

Peripheral clock

OSC_OUT HSE 180 MHz

APBx enable APBx peripheral clocks PRESC

max

OSC_IN

/1,2,4,8,16

Peripheral clock

if (APBx presc = enable APBx timer clocks 1x1 else x2

/M

Peripheral

clock enable 48 MHz

PLL48CK

clocks

I2SSRC

Peripheral

PLLI2SCLK

clock enable I2S clocks

PLLI2S /R Ext. clock

I2S_CKIN

Peripheral SAI1_A clock clock enable

PLLSAICLK DIV

Peripheral SAI1_B clock clock enable

VCO /P

| xN PLLSAI | /Q |
|---|---|
| /R |  |


PLLSAICLK

DIV

Peripheral LCD-TFT clock enable clock

PLLLCDCLK

DIV

ETH_MII_TX_CLK_MII Peripheral



MACTXCLK

clock enable

/2,20

MII_RMII_SEL in SYSCFG_PMC to Ethernet MAC

Peripheral

ETH_MII_RX_CLK_MI clock enable

MACRXCLK

I Peripheral

clock enable

MACRMIICLK

Peripheral

clock enable USBHS

USB2.0 PHY OTG_HS_ULPI_CK

ULPI clock

24 to 60 MHz

MS30434V2

1. For full details about the internal and external clock source characteristics, refer to the Electrical characteristics section in the device datasheet.

2. When TIMPRE bit of the RCC_DCKCFGR register is reset, if APBx prescaler is 1, then TIMxCLK = PCLKx, otherwise TIMxCLK = 2x PCLKx.

3. When TIMPRE bit in the RCC_DCKCFGR register is set, if APBx prescaler is 1,2 or 4, then TIMxCLK = HCLK, otherwise TIMxCLK = 4x PCLKx.



--- Page 153 ---

**RM0090 Reset and clock control for   STM32F42xxx and STM32F43xxx (RCC)**

The clock controller provides a high degree of flexibility to the application in the choice of the external crystal or the oscillator to run the core and peripherals at the highest frequency and, guarantee the appropriate frequency for peripherals that need a specific clock like Ethernet, USB OTG FS and HS, I2S, SAI, LTDC, and SDIO.

Several prescalers are used to configure the AHB frequency, the high-speed APB (APB2) and the low-speed APB (APB1) domains. The maximum frequency of the AHB domain is 180 MHz. The maximum allowed frequency of the high-speed APB2 domain is 90 MHz. The maximum allowed frequency of the low-speed APB1 domain is 45 MHz

All peripheral clocks are derived from the system clock (SYSCLK) except for:

• The USB OTG FS clock (48 MHz), the random analog generator (RNG) clock

( ≤ 48 MHz) and the SDIO clock ( ≤ 48 MHz) which are coming from a specific output of PLL (PLL48CLK)

• I2S clock

To achieve high-quality audio performance, the I2S clock can be derived either from a specific PLL (PLLI2S) or from an external clock mapped on the I2S_CKIN pin. For more information about I2S clock frequency and precision, refer to Section 28.4.4: Clock generator .

• SAI1 clock

The SAI1 clock is generated from a specific PLL (PLLSAI or PLLI2S) or from an external clock mapped on the I2S_CKIN pin.

The PLLSAI can be used as clock source for SAI1 peripheral in case the PLLI2S is programmed to achieve another audio sampling frequency (49.152 MHz or 11.2896 MHz), and the application requires both frequencies at the same time.

• LTDC clock

The LTDC clock is generated from a specific PLL (PLLSAI).

• The USB OTG HS (60 MHz) clock which is provided from the external PHY

• The Ethernet MAC clocks (TX, RX and RMII) which are provided from the external

PHY. For further information on the Ethernet configuration, please refer to Section 33.4.4: MII/RMII selection in the Ethernet peripheral description. When the Ethernet is used, the AHB clock frequency must be at least 25 MHz.

The RCC feeds the external clock of the Cortex System Timer (SysTick) with the AHB clock (HCLK) divided by 8. The SysTick can work either with this clock or with the Cortex clock (HCLK), configurable in the SysTick control and status register.

The timer clock frequencies for STM32F42xxx and STM32F43xxx are automatically set by hardware. There are two cases depending on the value of TIMPRE bit in RCC_CFGR register:

• If TIMPRE bit in RCC_DKCFGR register is reset:

If the APB prescaler is configured to a division factor of 1, the timer clock frequencies (TIMxCLK) are set to PCLKx. Otherwise, the timer clock frequencies are twice the frequency of the APB domain to which the timers are connected: TIMxCLK = 2xPCLKx.

• If TIMPRE bit in RCC_DKCFGR register is set:

If the APB prescaler is configured to a division factor of 1, 2 or 4, the timer clock frequencies (TIMxCLK) are set to HCLK. Otherwise, the timer clock frequencies is four times the frequency of the APB domain to which the timers are connected: TIMxCLK = 4xPCLKx.



212

--- Page 154 ---

**Reset and clock control for   STM32F42xxx and STM32F43xxx (RCC) RM0090**

FCLK acts as Cortex ® -M4 with FPU free-running clock. For more details, refer to the Cortex ® -M4 with FPU technical reference manual.

**6.2.1 HSE clock**

The high speed external clock signal (HSE) can be generated from two possible clock sources:

• HSE external crystal/ceramic resonator

• HSE external user clock

The resonator and the load capacitors have to be placed as close as possible to the oscillator pins in order to minimize output distortion and startup stabilization time. The loading capacitance values must be adjusted according to the selected oscillator.

**Figure 17. HSE/ LSE clock sources**

|  | Hardware configuration |
|---|---|
| External clock | OSC OUT _ (HI-Z) External source |
| Crystal/ceramic resonators | OSC IN OSC OUT _ _ C C L1 L2 Load capacitors |


**External source (HSE bypass)**

In this mode, an external clock source must be provided. You select this mode by setting the HSEBYP and HSEON bits in the RCC clock control register (RCC_CR) . The external clock signal (square, sinus or triangle) with ~50% duty cycle has to drive the OSC_IN pin while the OSC_OUT pin should be left HI-Z. See Figure 17 .

**External crystal/ceramic resonator (HSE crystal)**

The HSE has the advantage of producing a very accurate rate on the main clock. The associated hardware configuration is shown in Figure 17 . Refer to the electrical characteristics section of the datasheet for more details.

The HSERDY flag in the RCC clock control register (RCC_CR) indicates if the high-speed external oscillator is stable or not. At startup, the clock is not released until this bit is set by hardware. An interrupt can be generated if enabled in the RCC clock interrupt register (RCC_CIR) .



--- Page 155 ---

**RM0090 Reset and clock control for   STM32F42xxx and STM32F43xxx (RCC)**

The HSE Crystal can be switched on and off using the HSEON bit in the RCC clock control register (RCC_CR) .

**6.2.2 HSI clock**

The HSI clock signal is generated from an internal 16 MHz RC oscillator and can be used directly as a system clock, or used as PLL input.

The HSI RC oscillator has the advantage of providing a clock source at low cost (no external components). It also has a faster startup time than the HSE crystal oscillator however, even with calibration the frequency is less accurate than an external crystal oscillator or ceramic resonator.

**Calibration**

RC oscillator frequencies can vary from one chip to another due to manufacturing process variations, this is why each device is factory calibrated by ST for 1% accuracy at T A = 25 °C. After reset, the factory calibration value is loaded in the HSICAL[7:0] bits in the RCC clock control register (RCC_CR) .

If the application is subject to voltage or temperature variations this may affect the RC oscillator speed. You can trim the HSI frequency in the application using the HSITRIM[4:0] bits in the RCC clock control register (RCC_CR) .

The HSIRDY flag in the RCC clock control register (RCC_CR) indicates if the HSI RC is stable or not. At startup, the HSI RC output clock is not released until this bit is set by hardware.

The HSI RC can be switched on and off using the HSION bit in the RCC clock control register (RCC_CR) .

The HSI signal can also be used as a backup source (Auxiliary clock) if the HSE crystal oscillator fails. Refer to Section 6.2.7: Clock security system (CSS) on page 157 .

**6.2.3 PLL configuration**

The STM32F4xx devices feature three PLLs:

• A main PLL (PLL) clocked by the HSE or HSI oscillator and featuring two different

output clocks:

– The first output is used to generate the high speed system clock (up to 180 MHz)

– The second output is used to generate the clock for the USB OTG FS (48 MHz),

the random analog generator ( ≤ 48 MHz) and the SDIO ( ≤ 48 MHz).

• Two dedicated PLLs (PLLI2S and PLLSAI) used to generate an accurate clock to

achieve high-quality audio performance on the I2S and SAI1 interfaces. PLLSAI is also used for the LCD-TFT clock.

Since the main-PLL configuration parameters cannot be changed once PLL is enabled, it is recommended to configure PLL before enabling it (selection of the HSI or HSE oscillator as PLL clock source, and configuration of division factors M, N, P, and Q).

The PLLI2S and PLLSAI use the same input clock as PLL (PLLM[5:0] and PLLSRC bits are common to both PLLs). However, the PLLI2S and PLLSAI have dedicated enable/disable and division factors (N and R) configuration bits. Once the PLLI2S and PLLSAI are enabled, the configuration parameters cannot be changed.



212

--- Page 156 ---

**Reset and clock control for   STM32F42xxx and STM32F43xxx (RCC) RM0090**

The three PLLs are disabled by hardware when entering Stop and Standby modes, or when an HSE failure occurs when HSE or PLL (clocked by HSE) are used as system clock. RCC PLL configuration register (RCC_PLLCFGR) , RCC clock configuration register (RCC_CFGR) , and RCC Dedicated Clock Configuration Register (RCC_DCKCFGR) can be used to configure PLL, PLLI2S, and PLLSAI.

**6.2.4 LSE clock**

The LSE clock is generated from a 32.768 kHz low-speed external crystal or ceramic resonator. It has the advantage providing a low-power but highly accurate clock source to the real-time clock peripheral (RTC) for clock/calendar or other timing functions. The LSE oscillator is switched on and off using the LSEON bit in RCC Backup domain control register (RCC_BDCR) .

The LSERDY flag in the RCC Backup domain control register (RCC_BDCR) indicates if the LSE crystal is stable or not. At startup, the LSE crystal output clock signal is not released until this bit is set by hardware. An interrupt can be generated if enabled in the RCC clock interrupt register (RCC_CIR) .

**External source (LSE bypass)**

In this mode, an external clock source must be provided. It must have a frequency up to 1 MHz. You select this mode by setting the LSEBYP and LSEON bits in the RCC Backup domain control register (RCC_BDCR) . The external clock signal (square, sinus or triangle) with ~50% duty cycle has to drive the OSC32_IN pin while the OSC32_OUT pin should be left HI-Z. See Figure 17 .

**6.2.5 LSI clock**

The LSI RC acts as an low-power clock source that can be kept running in Stop and Standby mode for the independent watchdog (IWDG) and Auto-wakeup unit (AWU). The clock frequency is around 32 kHz. For more details, refer to the electrical characteristics section of the datasheets.

The LSI RC can be switched on and off using the LSION bit in the RCC clock control & status register (RCC_CSR) .

The LSIRDY flag in the RCC clock control & status register (RCC_CSR) indicates if the low- speed internal oscillator is stable or not. At startup, the clock is not released until this bit is set by hardware. An interrupt can be generated if enabled in the RCC clock interrupt register (RCC_CIR) .

**6.2.6 System clock (SYSCLK) selection**

After a system reset, the HSI oscillator is selected as the system clock. When a clock source is used directly or through PLL as the system clock, it is not possible to stop it.

A switch from one clock source to another occurs only if the target clock source is ready (clock stable after startup delay or PLL locked). If a clock source that is not yet ready is selected, the switch occurs when the clock source is ready. Status bits in the RCC clock control register (RCC_CR) indicate which clock(s) is (are) ready and which clock is currently used as the system clock.



--- Page 157 ---

**RM0090 Reset and clock control for   STM32F42xxx and STM32F43xxx (RCC)**

**6.2.7 Clock security system (CSS)**

The clock security system can be activated by software. In this case, the clock detector is enabled after the HSE oscillator startup delay, and disabled when this oscillator is stopped. If a failure is detected on the HSE clock, this oscillator is automatically disabled, a clock failure event is sent to the break inputs of advanced-control timers TIM1 and TIM8, and an interrupt is generated to inform the software about the failure (clock security system interrupt CSSI), allowing the MCU to perform rescue operations. The CSSI is linked to the Cortex ® -M4 with FPU NMI (non-maskable interrupt) exception vector.

Note: When the CSS is enabled, if the HSE clock happens to fail, the CSS generates an interrupt, which causes the automatic generation of an NMI. The NMI is executed indefinitely unless the CSS interrupt pending bit is cleared. As a consequence, the application has to clear the CSS interrupt in the NMI ISR by setting the CSSC bit in the Clock interrupt register (RCC_CIR).

If the HSE oscillator is used directly or indirectly as the system clock (indirectly meaning that it is directly used as PLL input clock, and that PLL clock is the system clock) and a failure is detected, then the system clock switches to the HSI oscillator and the HSE oscillator is disabled.

If the HSE oscillator clock was the clock source of PLL used as the system clock when the failure occurred, PLL is also disabled. In this case, if the PLLI2S was enabled, it is also disabled when the HSE fails.

**6.2.8 RTC/AWU clock**

Once the RTCCLK clock source has been selected, the only possible way of modifying the selection is to reset the power domain.

The RTCCLK clock source can be either the HSE 1 MHz (HSE divided by a programmable prescaler), the LSE or the LSI clock. This is selected by programming the RTCSEL[1:0] bits in the RCC Backup domain control register (RCC_BDCR) and the RTCPRE[4:0] bits in RCC clock configuration register (RCC_CFGR) . This selection cannot be modified without resetting the Backup domain.

If the LSE is selected as the RTC clock, the RTC operates normally if the backup or the system supply disappears. If the LSI is selected as the AWU clock, the AWU state is not guaranteed if the system supply disappears. If the HSE oscillator divided by a value between 2 and 31 is used as the RTC clock, the RTC state is not guaranteed if the backup or the system supply disappears.

The LSE clock is in the Backup domain, whereas the HSE and LSI clocks are not. As a consequence:

• If LSE is selected as the RTC clock:

– The RTC continues to work even if the V DD supply is switched off, provided the

V BAT supply is maintained.

• If LSI is selected as the Auto-wakeup unit (AWU) clock:

– The AWU state is not guaranteed if the V DD supply is powered off. Refer to

Section 6.2.5: LSI clock on page 156 for more details on LSI calibration.

• If the HSE clock is used as the RTC clock:

– The RTC state is not guaranteed if the V DD supply is powered off or if the internal

voltage regulator is powered off (removing power from the 1.2 V domain).



212

--- Page 158 ---

**Reset and clock control for   STM32F42xxx and STM32F43xxx (RCC) RM0090**

Note: To read the RTC calendar register when the APB1 clock frequency is less than seven times the RTC clock frequency (f APB1 < 7xf RTCLCK ), the software must read the calendar time and date registers twice. The data are correct if the second read access to RTC_TR gives the same result than the first one. Otherwise a third read access must be performed.

**6.2.9 Watchdog clock**

If the independent watchdog (IWDG) is started by either hardware option or software access, the LSI oscillator is forced ON and cannot be disabled. After the LSI oscillator temporization, the clock is provided to the IWDG.

**6.2.10 Clock-out capability**

Two microcontroller clock output (MCO) pins are available:

• MCO1

You can output four different clock sources onto the MCO1 pin (PA8) using the configurable prescaler (from 1 to 5):

– HSI clock

– LSE clock

– HSE clock

– PLL clock

The desired clock source is selected using the MCO1PRE[2:0] and MCO1[1:0] bits in the RCC clock configuration register (RCC_CFGR) .

• MCO2

You can output four different clock sources onto the MCO2 pin (PC9) using the configurable prescaler (from 1 to 5):

– HSE clock

– PLL clock

– System clock (SYSCLK)

– PLLI2S clock

The desired clock source is selected using the MCO2PRE[2:0] and MCO2 bits in the RCC clock configuration register (RCC_CFGR) .

For the different MCO pins, the corresponding GPIO port has to be programmed in alternate function mode.

The selected clock to output onto MCO must not exceed 100 MHz (the maximum I/O speed).

**6.2.11 Internal/external clock measurement using TIM5/TIM11**

It is possible to indirectly measure the frequencies of all on-board clock source generators by means of the input capture of TIM5 channel4 and TIM11 channel1 as shown in Figure 18 and Figure 19 .

**Internal/external clock measurement using TIM5 channel4**

TIM5 has an input multiplexer which allows choosing whether the input capture is triggered by the I/O or by an internal clock. This selection is performed through the TI4_RMP [1:0] bits in the TIM5_OR register.



--- Page 159 ---

**RM0090 Reset and clock control for   STM32F42xxx and STM32F43xxx (RCC)**

The primary purpose of having the LSE connected to the channel4 input capture is to be able to precisely measure the HSI (this requires to have the HSI used as the system clock source). The number of HSI clock counts between consecutive edges of the LSE signal provides a measurement of the internal clock period. Taking advantage of the high precision of LSE crystals (typically a few tens of ppm) we can determine the internal clock frequency with the same resolution, and trim the source to compensate for manufacturing-process and/or temperature- and voltage-related frequency deviations.

The HSI oscillator has dedicated, user-accessible calibration bits for this purpose.

The basic concept consists in providing a relative measurement (e.g. HSI/LSE ratio): the precision is therefore tightly linked to the ratio between the two clock sources. The greater the ratio, the better the measurement.

It is also possible to measure the LSI frequency: this is useful for applications that do not have a crystal. The ultralow-power LSI oscillator has a large manufacturing process deviation: by measuring it versus the HSI clock source, it is possible to determine its frequency with the precision of the HSI. The measured value can be used to have more accurate RTC time base timeouts (when LSI is used as the RTC clock source) and/or an IWDG timeout with an acceptable accuracy.

Use the following procedure to measure the LSI frequency:

1. Enable the TIM5 timer and configure channel4 in Input capture mode.

2. This bit is set the TI4_RMP bits in the TIM5_OR register to 0x01 to connect the LSI

clock internally to TIM5 channel4 input capture for calibration purposes.

3. Measure the LSI clock frequency using the TIM5 capture/compare 4 event or interrupt.

4. Use the measured LSI frequency to update the prescaler of the RTC depending on the

desired time base and/or to compute the IWDG timeout.

**Figure 18. Frequency measurement with TIM5 in Input capture mode**

TIM5

TI4_RMP[1:0]

GPIO

RTC_WakeUp_IT TI4

LSE

LSI

ai17741V2

**Internal/external clock measurement using TIM11 channel1**

TIM11 has an input multiplexer which allows choosing whether the input capture is triggered by the I/O or by an internal clock. This selection is performed through TI1_RMP [1:0] bits in the TIM11_OR register. The HSE_RTC clock (HSE divided by a programmable prescaler) is connected to channel 1 input capture to have a rough indication of the external crystal frequency. This requires that the HSI is the system clock source. This can be useful for instance to ensure compliance with the IEC 60730/IEC 61335 standards which require to be able to determine harmonic or subharmonic frequencies (–50/+100% deviations).



212

--- Page 160 ---

**Reset and clock control for   STM32F42xxx and STM32F43xxx (RCC)**

**RM0090**

**Figure 19. Frequency measurement with TIM11 in Input capture mode**

TIM11

TI1_RMP[1:0]

GPIO

TI1

HSE_RTC(1 MHz)

ai18433



--- Page 161 ---

**RM0090**

**Reset and clock control for   STM32F42xxx and STM32F43xxx (RCC)**


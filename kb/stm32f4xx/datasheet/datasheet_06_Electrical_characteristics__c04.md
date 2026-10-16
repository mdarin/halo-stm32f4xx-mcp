> [DATASHEET ch.6 §6] Electrical characteristics

### 6.3.6 Supply current characteristics

### 6.3.6 Supply current characteristics

The current consumption is a function of several parameters and factors such as the operating voltage, ambient temperature, I/O pin loading, device software configuration, operating frequencies, I/O pin switching rate, program location in memory and executed binary code.

The current consumption is measured as described in Figure 22: Current consumption measurement scheme.

All Run mode current consumption measurements given in this section are performed using a CoreMark-compliant code.

**Typical and maximum current consumption**

The MCU is placed under the following conditions:

*   At startup, all I/O pins are configured as analog inputs by firmware.
*   All peripherals are disabled except if it is explicitly mentioned.
*   The flash memory access time is adjusted to f<sub>HCLK</sub> frequency (0 wait state from 0 to 30 MHz, 1 wait state from 30 to 60 MHz, 2 wait states from 60 to 90 MHz, 3 wait states from 90 to 120 MHz, 4 wait states from 120 to 150 MHz, and 5 wait states from 150 to 168 MHz).
*   When the peripherals are enabled HCLK is the system clock, f<sub>PCLK1</sub> = f<sub>HCLK</sub>/4, and f<sub>PCLK2</sub> = f<sub>HCLK</sub>/2, except is explicitly mentioned.
*   The maximum values are obtained for V<sub>DD</sub> = 3.6 V and maximum ambient temperature (T<sub>A</sub>), and the typical values for T<sub>A</sub>= 25 °C and V<sub>DD</sub> = 3.3 V unless otherwise specified.

86/206
DS8626 Rev 12

--- Page 87 ---

STM32F405xx, STM32F407xx
Electrical characteristics

Table 20. Typical and maximum current consumption in Run mode, code with data processing running from flash memory (ART accelerator enabled) or RAM (1)

| Symbol | Parameter | Conditions | fHCLK | Typ | Max(2) | Unit |  |
|---|---|---|---|---|---|---|---|
| TA = 25 °C | TA = 85 °C | TA = 105 °C |  |  |  |  |  |
| IDD | Supply current in Run mode | External clock(3), all peripherals enabled(4)(5) | 168 MHz | 87 | 102 | 109 | mA |
| 144 MHz | 67 | 80 | 86 |  |  |  |  |
| 120 MHz | 56 | 69 | 75 |  |  |  |  |
| 90 MHz | 44 | 56 | 62 |  |  |  |  |
| 60 MHz | 30 | 42 | 49 |  |  |  |  |
| 30 MHz | 16 | 28 | 35 |  |  |  |  |
| 25 MHz | 12 | 24 | 31 |  |  |  |  |
| 16 MHz(6) | 9 | 20 | 28 |  |  |  |  |
| 8 MHz | 5 | 17 | 24 |  |  |  |  |
| 4 MHz | 3 | 15 | 22 |  |  |  |  |
| 2 MHz | 2 | 14 | 21 |  |  |  |  |
| External clock(3), all peripherals disabled(4)(5) | 168 MHz | 40 | 54 | 61 |  |  |  |
| 144 MHz | 31 | 43 | 50 |  |  |  |  |
| 120 MHz | 26 | 38 | 45 |  |  |  |  |
| 90 MHz | 20 | 32 | 39 |  |  |  |  |
| 60 MHz | 14 | 26 | 33 |  |  |  |  |
| 30 MHz | 8 | 20 | 27 |  |  |  |  |
| 25 MHz | 6 | 18 | 25 |  |  |  |  |
| 16 MHz(6) | 5 | 16 | 24 |  |  |  |  |
| 8 MHz | 3 | 15 | 22 |  |  |  |  |
| 4 MHz | 2 | 14 | 21 |  |  |  |  |
| 2 MHz | 2 | 14 | 21 |  |  |  |  |


1. Code and data processing running from SRAM1 using boot pins.
2. Evaluated by characterization, tested in production at V<sub>DD</sub> max and f<sub>HCLK</sub> max with peripherals enabled.
3. External clock is 4 MHz and PLL is on when f<sub>HCLK</sub> > 25 MHz.
4. When the ADC is ON (ADON bit set in the ADC_CR2 register), add an additional power consumption of 1.6 mA per ADC for the analog part.
5. When analog peripheral blocks such as ADCs, DACs, HSE, LSE, HSI, or LSI are ON, an additional power consumption should be considered.
6. In this case HCLK = system clock/2.

ST
DS8626 Rev 12
87/206

--- Page 88 ---

**Electrical characteristics**
**STM32F405xx, STM32F407xx**

**Table 21. Typical and maximum current consumption in Run mode, code with data processing running from flash memory (ART accelerator disabled)**

| Symbol | Parameter | Conditions | fHCLK | Typ | Max(1) | Unit |  |
|---|---|---|---|---|---|---|---|
| TA = 25 °C | TA = 85 °C | TA = 105 °C |  |  |  |  |  |
| IDD | Supply current in Run mode | External clock(2), all peripherals enabled(3)(4) | 168 MHz | 93 | 109 | 117 | mA |
| 144 MHz | 76 | 89 | 96 |  |  |  |  |
| 120 MHz | 67 | 79 | 86 |  |  |  |  |
| 90 MHz | 53 | 65 | 73 |  |  |  |  |
| 60 MHz | 37 | 49 | 56 |  |  |  |  |
| 30 MHz | 20 | 32 | 39 |  |  |  |  |
| 25 MHz | 16 | 27 | 35 |  |  |  |  |
| 16 MHz | 11 | 23 | 30 |  |  |  |  |
| 8 MHz | 6 | 18 | 25 |  |  |  |  |
| 4 MHz | 4 | 16 | 23 |  |  |  |  |
| 2 MHz | 3 | 15 | 22 |  |  |  |  |
| External clock(2), all peripherals disabled(3)(4) | 168 MHz | 46 | 61 | 69 |  |  |  |
| 144 MHz | 40 | 52 | 60 |  |  |  |  |
| 120 MHz | 37 | 48 | 56 |  |  |  |  |
| 90 MHz | 30 | 42 | 50 |  |  |  |  |
| 60 MHz | 22 | 33 | 41 |  |  |  |  |
| 30 MHz | 12 | 24 | 31 |  |  |  |  |
| 25 MHz | 10 | 21 | 29 |  |  |  |  |
| 16 MHz | 7 | 19 | 26 |  |  |  |  |
| 8 MHz | 4 | 16 | 23 |  |  |  |  |
| 4 MHz | 3 | 15 | 22 |  |  |  |  |
| 2 MHz | 2 | 14 | 21 |  |  |  |  |


1. Evaluated by characterization, tested in production at V<sub>DD</sub> max and f<sub>HCLK</sub> max with peripherals enabled.
2. External clock is 4 MHz and PLL is on when f<sub>HCLK</sub> > 25 MHz.
3. When analog peripheral blocks such as (ADCs, DACs, HSE, LSE, HSI,LSI) are on, an additional power consumption should be considered.
4. When the ADC is ON (ADON bit set in the ADC_CR2 register), add an additional power consumption of 1.6 mA per ADC for the analog part.

88/206
DS8626 Rev 12

--- Page 89 ---

STM32F405xx, STM32F407xx
Electrical characteristics

Figure 24. Typical current consumption versus temperature, Run mode, code with data processing running from flash (ART accelerator ON) or RAM, and peripherals OFF

[Graph: Idd run(mA) vs CPU Frequency (MHz) showing current consumption at -45°C, 0°C, 25°C, 55°C, 85°C, and 105°C]
MS19974V1

Figure 25. Typical current consumption versus temperature, Run mode, code with data processing running from flash (ART accelerator ON) or RAM, and peripherals ON

[Graph: Idd run(mA) vs CPU Frequency (MHz) showing current consumption at -45°C, 0°C, 25°C, 55°C, 85°C, and 105°C]
MS19975V1

ST
DS8626 Rev 12
89/206

--- Page 90 ---

**Electrical characteristics**
**STM32F405xx, STM32F407xx**

**Figure 26. Typical current consumption versus temperature, Run mode, code with data processing running from flash (ART accelerator OFF) or RAM, and peripherals OFF**

[Graph: Idd run(mA) vs CPU Frequency (MHz)]
- Legend:
  - -45°C
  - 0°C
  - 25°C
  - 55°C
  - 85°C
  - 105°C

MS19976V1

**Figure 27. Typical current consumption versus temperature, Run mode, code with data processing running from flash (ART accelerator OFF) or RAM, and peripherals ON**

[Graph: Idd run(mA) vs CPU Frequency (MHz)]
- Legend:
  - -45°C
  - 0°C
  - 25°C
  - 55°C
  - 85°C
  - 105°C

MS19977V1

90/206
DS8626 Rev 12

--- Page 91 ---

STM32F405xx, STM32F407xx
Electrical characteristics

Table 22. Typical and maximum current consumption in Sleep mode

| Symbol | Parameter | Conditions | fHCLK | Typ | Max(1) | Unit |  |
|---|---|---|---|---|---|---|---|
| TA = 25 °C | TA = 85 °C | TA = 105 °C |  |  |  |  |  |
| IDD | Supply current in Sleep mode | External clock(2), all peripherals enabled(3) | 168 MHz | 59 | 77 | 84 | mA |
| 144 MHz | 46 | 61 | 67 |  |  |  |  |
| 120 MHz | 38 | 53 | 60 |  |  |  |  |
| 90 MHz | 30 | 44 | 51 |  |  |  |  |
| 60 MHz | 20 | 34 | 41 |  |  |  |  |
| 30 MHz | 11 | 24 | 31 |  |  |  |  |
| 25 MHz | 8 | 21 | 28 |  |  |  |  |
| 16 MHz | 6 | 18 | 25 |  |  |  |  |
| 8 MHz | 3 | 16 | 23 |  |  |  |  |
| 4 MHz | 2 | 15 | 22 |  |  |  |  |
| 2 MHz | 2 | 14 | 21 |  |  |  |  |
| External clock(2), all peripherals disabled | 168 MHz | 12 | 27 | 35 |  |  |  |
| 144 MHz | 9 | 22 | 29 |  |  |  |  |
| 120 MHz | 8 | 20 | 28 |  |  |  |  |
| 90 MHz | 7 | 19 | 26 |  |  |  |  |
| 60 MHz | 5 | 17 | 24 |  |  |  |  |
| 30 MHz | 3 | 16 | 23 |  |  |  |  |
| 25 MHz | 2 | 15 | 22 |  |  |  |  |
| 16 MHz | 2 | 14 | 21 |  |  |  |  |
| 8 MHz | 1 | 14 | 21 |  |  |  |  |
| 4 MHz | 1 | 13 | 21 |  |  |  |  |
| 2 MHz | 1 | 13 | 21 |  |  |  |  |


1. Evaluated by characterization, tested in production at V<sub>DD</sub> max and f<sub>HCLK</sub> max with peripherals enabled.
2. External clock is 4 MHz and PLL is on when f<sub>HCLK</sub> > 25 MHz.
3. Add an additional power consumption of 1.6 mA per ADC for the analog part. In applications, this consumption occurs only while the ADC is ON (ADON bit is set in the ADC_CR2 register).

ST
DS8626 Rev 12
91/206

--- Page 92 ---

**Electrical characteristics**
STM32F405xx, STM32F407xx

**Table 23. Typical and maximum current consumptions in Stop mode**

| Symbol | Parameter | Conditions | Typ | Max | Unit |  |  |
|---|---|---|---|---|---|---|---|
| TA = 25 °C | TA = 85 °C | TA = 105 °C |  |  |  |  |  |
| IDD_STOP | Supply current in Stop mode with main regulator in Run mode | Flash in Stop mode, low-speed and high-speed internal RC oscillators and high-speed oscillator OFF (no independent watchdog) | 0.45 | 1.5 | 11.00 | 20.00 | mA |
| Flash in Deep power-down mode, low-speed and high-speed internal RC oscillators and high-speed oscillator OFF (no independent watchdog) | 0.40 | 1.5 | 11.00 | 20.00 |  |  |  |
| Supply current in Stop mode with main regulator in Low-power mode | Flash in Stop mode, low-speed and high-speed internal RC oscillators and high-speed oscillator OFF (no independent watchdog) | 0.31 | 1.1 | 8.00 | 15.00 |  |  |
| Flash in Deep power-down mode, low-speed and high-speed internal RC oscillators and high-speed oscillator OFF (no independent watchdog) | 0.28 | 1.1 | 8.00 | 15.00 |  |  |  |


**Table 24. Typical and maximum current consumptions in Standby mode**

| Symbol | Parameter | Conditions | Typ | Max(1) | Unit |  |  |  |
|---|---|---|---|---|---|---|---|---|
| TA = 25 °C | TA = 85 °C |  |  |  |  |  |  |  |
| VDD = 1.8 V | VDD= 2.4 V | VDD = 3.3 V | VDD = 3.6 V |  |  |  |  |  |
| IDD_STBY | Supply current in Standby mode | Backup SRAM ON, low-speed oscillator and RTC ON | 3.0 | 3.4 | 4.0 | 20 | 36 | µA |
| Backup SRAM OFF, low-speed oscillator and RTC ON | 2.4 | 2.7 | 3.3 | 16 | 32 |  |  |  |
| Backup SRAM ON, RTC OFF | 2.4 | 2.6 | 3.0 | 12.5 | 24.8 |  |  |  |
| Backup SRAM OFF, RTC OFF | 1.7 | 1.9 | 2.2 | 9.8 | 19.2 |  |  |  |


1. Evaluated by characterization - not tested in production.

92/206
DS8626 Rev 12

--- Page 93 ---

STM32F2405xx, STM32F2407xx
Electrical characteristics

Table 25. Typical and maximum current consumptions in VBAT mode

| Symbol | Parameter | Conditions | Unit |
|---|---|---|---|
| Typ | Max(1) |  |  |
| IDD_VBAT | Backup domain supply current | TA = 25 °C | µA |
| TA = 85 °C |  |  |  |
| TA = 105 °C |  |  |  |
| VBAT = 3.6 V |  |  |  |
|  |  | VBAT = 3.3 V |  |
| VBAT = 2.4 V |  |  |  |
| VBAT = 1.8 V |  |  |  |
| Backup SRAM ON, low-speed oscillator and RTC ON |  |  |  |
|  |  | Backup SRAM ON, low-speed oscillator and RTC ON |  |
| Backup SRAM OFF, low-speed oscillator and RTC ON |  |  |  |
| Backup SRAM ON, RTC OFF |  |  |  |
| Backup SRAM OFF, RTC OFF |  |  |  |


1. Evaluated by characterization - not tested in production.

Figure 28. Typical VBAT current consumption (LSE and RTC ON/backup RAM OFF)

--- Page 94 ---

Electrical characteristics
STM32F405xx, STM32F407xx

Figure 29. Typical V_BAT current consumption (LSE and RTC ON/backup RAM ON)

IVBAT in (µA)
6
5
4
3
2
1
0
0 10 20 30 40 50 60 70 80 90 100
Temperature in (°C)

1.65V
1.8V
2V
2.4V
2.7V
3V
3.3V
3.6V

MS19991V1

**Additional current consumption**

The MCU is placed under the following conditions:
* All I/O pins are configured in analog mode.
* The flash memory access time is adjusted to f_HCLK frequency.
* The voltage scaling is adjusted to f_HCLK frequency as follows:
    * Scale 2 for f_HCLK ≤ 144 MHz
    * Scale 1 for 144 MHz < f_HCLK ≤ 168 MHz.
* The system clock is HCLK, f_PCLK1 = f_HCLK/4, and f_PCLK2 = f_HCLK/2.
* The HSE crystal clock frequency is 25 MHz.
* T_A = 25 °C.

94/206
DS8626 Rev 12

--- Page 95 ---

STM32F405xx, STM32F407xx
Electrical characteristics

Table 26. Typical current consumption in Run mode, code with data processing running from flash memory, regulator ON (ART accelerator enabled except prefetch), VDD = 1.8 V(1)

| Symbol | Parameter | Conditions | fHCLK (MHz) | Typ. at TA = 25 °C | Unit |
|---|---|---|---|---|---|
| IDD | Supply current in Run mode | All peripheral disabled | 160 | 36.2 | mA |
| 144 | 29.3 |  |  |  |  |
| 120 | 24.7 |  |  |  |  |
| 90 | 19.3 |  |  |  |  |
| 60 | 13.4 |  |  |  |  |
| 30 | 7.7 |  |  |  |  |
| 25 | 6.0 |  |  |  |  |


1. When peripherals are enabled, the power consumption corresponding to the analog part of the peripherals (such as ADC or DAC) is not included.

**I/O system current consumption**

The current consumption of the I/O system has two components: static and dynamic.

I/O static current consumption

All the I/Os used as input with pull-up or pull-down generate current consumption when the pin is externally held to the opposite level. The value of this current consumption can be simply computed by using the pull-up/pull-down resistors values given in Table 48: I/O static characteristics.

For the output pins, any internal or external pull-up or pull-down and external load must also be considered to estimate the current consumption.

Additional I/O current consumption is due to I/Os configured as inputs if an intermediate voltage level is externally applied. This current consumption is caused by the input Schmitt trigger circuits used to discriminate the input value. Unless this specific configuration is required by the application, this supply current consumption can be avoided by configuring these I/Os in analog mode. This is notably the case of ADC input pins which should be configured as analog inputs.

**Caution:** Any floating input pin can also settle to an intermediate voltage level or switch inadvertently, as a result of external electromagnetic noise. To avoid current consumption related to floating pins, they must either be configured in analog mode, or forced internally to a definite digital value. This can be done either by using pull-up/down resistors or by configuring the pins in output mode.

I/O dynamic current consumption

In addition to the internal peripheral current consumption measured previously (see Table 28: Peripheral current consumption), the I/Os used by an application also contribute to the current consumption. When an I/O pin switches, it uses the current from the MCU supply voltage to supply the I/O pin circuitry and to charge/discharge the capacitive load internal and external connected to the pin:

I_SW = V_DD × f_SW × C

STI
DS8626 Rev 12
95/206

--- Page 96 ---

**Electrical characteristics** STM32F405xx, STM32F407xx

where
I_SW is the current sunk by a switching I/O to charge/discharge the capacitive load
V_DD is the MCU supply voltage
f_SW is the I/O switching frequency
C is the total capacitance seen by the I/O pin: C = C_INT+ C_EXT
The test pin is configured in push-pull output mode and is toggled by software at a fixed frequency.

96/206 DS8626 Rev 12 ST

--- Page 97 ---

STM32F405xx, STM32F407xx
Electrical characteristics

Table 27. Switching output I/O current consumption

| Symbol | Parameter | Conditions(1) | I/O toggling frequency (fSW) | Typ | Unit |
|---|---|---|---|---|---|
| IDDIO | I/O switching current | VDD = 3.3 V(2)C = CINT | 2 MHz | 0.02 | mA |
| 8 MHz | 0.14 |  |  |  |  |
| 25 MHz | 0.51 |  |  |  |  |
| 50 MHz | 0.86 |  |  |  |  |
| 60 MHz | 1.30 |  |  |  |  |
| VDD = 3.3 VCEXT = 0 pFC = CINT + CEXT + CS | 2 MHz | 0.10 |  |  |  |
| 8 MHz | 0.38 |  |  |  |  |
| 25 MHz | 1.18 |  |  |  |  |
| 50 MHz | 2.47 |  |  |  |  |
| 60 MHz | 2.86 |  |  |  |  |
| VDD = 3.3 VCEXT = 10 pFC = CINT + CEXT + CS | 2 MHz | 0.17 |  |  |  |
| 8 MHz | 0.66 |  |  |  |  |
| 25 MHz | 1.70 |  |  |  |  |
| 50 MHz | 2.65 |  |  |  |  |
| 60 MHz | 3.48 |  |  |  |  |
| VDD = 3.3 VCEXT = 22 pFC = CINT + CEXT + CS | 2 MHz | 0.23 |  |  |  |
| 8 MHz | 0.95 |  |  |  |  |
| 25 MHz | 3.20 |  |  |  |  |
| 50 MHz | 4.69 |  |  |  |  |
| 60 MHz | 8.06 |  |  |  |  |
| VDD = 3.3 VCEXT = 33 pFC = CINT + CEXT + CS | 2 MHz | 0.30 |  |  |  |
| 8 MHz | 1.22 |  |  |  |  |
| 25 MHz | 3.90 |  |  |  |  |
| 50 MHz | 8.82 |  |  |  |  |
| 60 MHz | —(3) |  |  |  |  |


1. C<sub>S</sub> is the PCB board capacitance including the pad pin. C<sub>S</sub> = 7 pF (estimated value).
2. This test is performed by cutting the LQFP package pin (pad removal).
3. At 60 MHz, C maximum load is specified 30 pF.

ST
DS8626 Rev 12
97/206

--- Page 98 ---

Electrical characteristics
STM32F405xx, STM32F407xx

**On-chip peripheral current consumption**

The current consumption of the on-chip peripherals is given in Table 28. The MCU is placed under the following conditions:

*   At startup, all I/O pins are configured as analog pins by firmware.
*   All peripherals are disabled unless otherwise mentioned
*   The code is running from flash memory and the flash memory access time is equal to 5 wait states at 168 MHz.
*   The code is running from flash memory and the flash memory access time is equal to 4 wait states at 144 MHz, and the power scale mode is set to 2.
*   The ART accelerator is ON.
*   The given value is calculated by measuring the difference of current consumption
    *   with all peripherals clocked off
    *   with one peripheral clocked on (with only the clock applied)
*   When the peripherals are enabled: HCLK is the system clock, fPCLK1 = fHCLK/4, and fPCLK2 = fHCLK/2.
*   The typical values are obtained for VDD = 3.3 V and TA = 25 °C, unless otherwise specified.

Table 28. Peripheral current consumption

| Peripheral | IDD(Typ)(1) | Unit |  |  |
|---|---|---|---|---|
| Scale1 (up t 168 MHz) | Scale2 (up to 144 MHz) |  |  |  |
| AHB1 (up to 168 MHz) | GPIOA | 2.70 | 2.40 | µA/MHz |
| GPIOB | 2.50 | 2.22 |  |  |
| GPIOC | 2.54 | 2.28 |  |  |
| GPIOD | 2.55 | 2.28 |  |  |
| GPIOE | 2.68 | 2.40 |  |  |
| GPIOF | 2.53 | 2.28 |  |  |
| GPIOG | 2.51 | 2.22 |  |  |
| GPIOH | 2.51 | 2.22 |  |  |
| GPIOI | 2.50 | 2.22 |  |  |
| OTG_HS+ULPI | 28.33 | 25.38 |  |  |
| CRC | 0.41 | 0.40 |  |  |
| BKPSRAM | 0.63 | 0.58 |  |  |
| DMA1 | 37.44 | 33.58 |  |  |
| DMA2 | 37.69 | 33.93 |  |  |
| ETH_MACETH_MAC_TXETH_MAC_RXETH_MAC_PTP | 20.43 | 18.39 |  |  |


98/206
DS8626 Rev 12

--- Page 99 ---

STM32F405xx, STM32F407xx
Electrical characteristics

Table 28. Peripheral current consumption (continued)

| Peripheral | IDD(Typ)(1) | Unit |  |  |
|---|---|---|---|---|
| Scale1 (up to 168 MHz) | Scale2 (up to 144 MHz) |  |  |  |
| AHB2 (up to 168 MHz) | OTG_FS | 26.45 | 26.67 | µA/MHz |
| DCMI | 5.87 | 5.35 | µA/MHz |  |
| AHB3 (up to 168 MHz) | RNG | 1.50 | 1.67 | µA/MHz |
|  | FSMC | 12.46 | 11.31 | µA/MHz |
| Bus matrix(2) | 13.10 | 11.81 | µA/MHz |  |
| APB1 (up to 42 MHz) | TIM2 | 16.71 | 16.50 | µA/MHz |
| TIM3 | 12.33 | 11.94 |  |  |
| TIM4 | 13.45 | 12.92 |  |  |
| TIM5 | 17.14 | 16.58 |  |  |
| TIM6 | 2.43 | 3.06 |  |  |
| TIM7 | 2.43 | 2.22 |  |  |
| TIM12 | 6.62 | 6.83 |  |  |
| TIM13 | 5.05 | 5.47 |  |  |
| TIM14 | 5.26 | 5.61 |  |  |
| PWR | 1.00 | 0.56 |  |  |
| USART2 | 2.69 | 2.78 |  |  |
| USART3 | 2.74 | 2.78 |  |  |
| UART4 | 3.24 | 3.33 |  |  |
| UART5 | 2.69 | 2.78 |  |  |
| I2C1 | 2.67 | 2.50 |  |  |
| I2C2 | 2.83 | 2.78 |  |  |
| I2C3 | 2.81 | 2.78 |  |  |
| SPI2 | 2.43 | 2.22 |  |  |
| SPI3 | 2.43 | 2.22 |  |  |
| I2S2(3) | 2.43 | 2.22 |  |  |
| I2S3(3) | 2.26 | 2.22 |  |  |
| CAN1 | 5.12 | 5.56 |  |  |
| CAN2 | 4.81 | 5.28 |  |  |
| DAC(4) | 1.67 |  |  |  |
| WWDG | 1.00 | 0.83 |  |  |


DS8626 Rev 12
99/206

--- Page 100 ---

Electrical characteristics
STM32F405xx, STM32F407xx

Table 28. Peripheral current consumption (continued)

| Peripheral | IDD(Typ)(1) | Unit |  |  |
|---|---|---|---|---|
| Scale1(up t 168 MHz) | Scale2(up to 144 MHz) |  |  |  |
| APB2(up to 84 MHz) | SDIO | 7.08 | 7.92 | µA/MHz |
| TIM1 | 16.79 | 15.51 |  |  |
| TIM8 | 17.88 | 16.53 |  |  |
| TIM9 | 7.64 | 7.28 |  |  |
| TIM10 | 4.89 | 4.82 |  |  |
| TIM11 | 5.19 | 4.82 |  |  |
| ADC1(5) | 4.67 | 4.58 |  |  |
| ADC2(5) | 4.67 | 4.58 |  |  |
| ADC3(5) | 4.43 | 4.44 |  |  |
| SPI1 | 1.32 | 1.39 |  |  |
| USART1 | 3.51 | 3.72 |  |  |
| USART6 | 3.55 | 3.75 |  |  |
| SYSCFG | 0.74 | 0.56 |  |  |


1. When the I/O compensation cell is ON, I<sub>DD</sub> typical value increases by 0.22 mA.
2. The BusMatrix is automatically active when at least one master is ON.
3. To enable an I2S peripheral, first set the I2SMOD bit and then the I2SE bit in the SPI_I2SCFGR register.
4. When the DAC is ON and EN1/2 bits are set in DAC_CR register, add an additional power consumption of 0.8 mA per DAC channel for the analog part.
5. When the ADC is ON (ADON bit set in the ADC_CR2 register), add an additional power consumption of 1.6 mA per ADC for the analog part.


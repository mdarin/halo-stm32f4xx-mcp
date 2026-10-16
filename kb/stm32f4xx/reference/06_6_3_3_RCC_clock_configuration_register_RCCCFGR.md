**6.3.3 RCC clock configuration register (RCC_CFGR)**

Address offset: 0x08

Reset value: 0x0000 0000

Access: 0 ≤ wait state ≤ 2, word, half-word and byte access

1 or 2 wait states inserted only if the access occurs during a clock source switch.

31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16

| MCO2 | MCO2 PRE[2:0] | MCO1 PRE[2:0] | I2SSC R | MCO1 | RTCPRE[4:0] |
|---|---|---|---|---|---|
| rw |  | rw |  | rw |  |


15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0

| PPRE2[2:0] | PPRE1[2:0] | Reserved | HPRE[3:0] | SWS1 | SWS0 | SW1 | SW0 |
|---|---|---|---|---|---|---|---|
| rw | r | rw |  |  |  |  |  |


Bits 31:30 MCO2[1:0]: Microcontroller clock output 2

Set and cleared by software. Clock source selection may generate glitches on MCO2. It is highly recommended to configure these bits only after reset before enabling the external oscillators and the PLLs.

00: System clock (SYSCLK) selected

01: PLLI2S clock selected

10: HSE oscillator clock selected

11: PLL clock selected

Bits 27:29 MCO2PRE: MCO2 prescaler

Set and cleared by software to configure the prescaler of the MCO2. Modification of this prescaler may generate glitches on MCO2. It is highly recommended to change this prescaler only after reset before enabling the external oscillators and the PLLs.

0xx: no division

100: division by 2

101: division by 3

110: division by 4

111: division by 5

Bits 24:26 MCO1PRE: MCO1 prescaler

Set and cleared by software to configure the prescaler of the MCO1. Modification of this prescaler may generate glitches on MCO1. It is highly recommended to change this prescaler only after reset before enabling the external oscillators and the PLL.

0xx: no division

100: division by 2

101: division by 3

110: division by 4

111: division by 5

Bit 23 I2SSRC : I2S clock selection

Set and cleared by software. This bit allows to select the I2S clock source between the PLLI2S clock and the external clock. It is highly recommended to change this bit only after reset and before enabling the I2S module.

0: PLLI2S clock used as I2S clock source

1: External clock mapped on the I2S_CKIN pin used as I2S clock source



212

--- Page 166 ---

**Reset and clock control for   STM32F42xxx and STM32F43xxx (RCC) RM0090**

Bits 22:21 MCO1: Microcontroller clock output 1

Set and cleared by software. Clock source selection may generate glitches on MCO1. It is highly recommended to configure these bits only after reset before enabling the external oscillators and PLL.

00: HSI clock selected

01: LSE oscillator selected

10: HSE oscillator clock selected

11: PLL clock selected

Bits 20:16 RTCPRE: HSE division factor for RTC clock

Set and cleared by software to divide the HSE clock input clock to generate a 1 MHz clock for RTC.

**Caution: The software has to set these bits correctly to ensure that the clock supplied to the**

RTC is 1 MHz. These bits must be configured if needed before selecting the RTC clock source.

00000: no clock

00001: no clock

00010: HSE/2

00011: HSE/3

00100: HSE/4

...

11110: HSE/30

11111: HSE/31

Bits 15:13 PPRE2: APB high-speed prescaler (APB2)

Set and cleared by software to control APB high-speed clock division factor.

**Caution: The software has to set these bits correctly not to exceed 90 MHz on this domain.**

The clocks are divided with the new prescaler factor from 1 to 16 AHB cycles after PPRE2 write.

0xx: AHB clock not divided

100: AHB clock divided by 2

101: AHB clock divided by 4

110: AHB clock divided by 8

111: AHB clock divided by 16

Bits 12:10 PPRE1: APB Low speed prescaler (APB1)

Set and cleared by software to control APB low-speed clock division factor.

**Caution: The software has to set these bits correctly not to exceed 45 MHz on this domain.**

The clocks are divided with the new prescaler factor from 1 to 16 AHB cycles after PPRE1 write.

0xx: AHB clock not divided

100: AHB clock divided by 2

101: AHB clock divided by 4

110: AHB clock divided by 8

111: AHB clock divided by 16

Bits 9:8 Reserved, must be kept at reset value.

166/1749 RM0090 Rev 18

--- Page 167 ---

**RM0090**

**Reset and clock control for   STM32F42xxx and STM32F43xxx (RCC)**

Bits 7:4 HPRE: AHB prescaler

Set and cleared by software to control AHB clock division factor. Caution:

The clocks are divided with the new prescaler factor from 1 to 16 AHB cycles after

HPRE write.

**Caution:**

The AHB clock frequency must be at least 25 MHz when the Ethernet is used.

0xxx: system clock not divided

1000: system clock divided by 2

1001: system clock divided by 4

1010: system clock divided by 8

1011: system clock divided by 16

1100: system clock divided by 64

1101: system clock divided by 128

1110: system clock divided by 256

1111: system clock divided by 512

Bits 3:2 SWS: System clock switch status

Set and cleared by hardware to indicate which clock source is used as the system clock.

00: HSI oscillator used as the system clock 01: HSE oscillator used as the system clock 10: PLL used as the system clock

11: not applicable

Bits 1:0 SW: System clock switch

Set and cleared by software to select the system clock source.

Set by hardware to force the HSI selection when leaving the Stop or Standby mode or in case of failure of the HSE oscillator used directly or indirectly as the system clock.

00: HSI oscillator selected as system clock 01: HSE oscillator selected as system clock 10: PLL selected as system clock

11: not allowed


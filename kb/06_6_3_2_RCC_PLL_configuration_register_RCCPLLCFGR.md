**6.3.2 RCC PLL configuration register (RCC_PLLCFGR)**

Address offset: 0x04

Reset value: 0x2400 3010

Access: no wait state, word, half-word and byte access.

This register is used to configure the PLL clock outputs according to the formulas:

• f (VCO clock) = f (PLL clock input)

× (PLLN / PLLM)

• f (PLL general clock output) = f (VCO clock)

/ PLLP

• f (USB OTG FS, SDIO, RNG clock output)

= f (VCO clock) / PLLQ

31 30 29 28 27 26 25 24

23 22 21 20 19 18 17 16

| Reserved | PLLQ3 | PLLQ2 | PLLQ1 | PLLQ0 | Reserv ed | PLLSR C | Reserved | PLLP1 | PLLP0 |
|---|---|---|---|---|---|---|---|---|---|
| rw |  |  |  |  |  |  |  |  |  |


15 14 13 12 11 10 9 8

7 6 5 4 3 2 1 0

| Reserv ed | PLLN | PLLM5 | PLLM4 | PLLM3 | PLLM2 | PLLM1 | PLLM0 |
|---|---|---|---|---|---|---|---|
| rw |  |  |  |  |  |  |  |


Bits 31:28 Reserved, must be kept at reset value.

Bits 27:24 PLLQ:

Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator

clocks

Set and cleared by software to control the frequency of USB OTG FS clock, the random number generator clock and the SDIO clock. These bits should be written only if PLL is

disabled.

**Caution:**

The USB OTG FS requires a 48 MHz clock to work correctly. The SDIO and the random number generator need a frequency lower than or equal to 48 MHz to work

correctly.

≤ PLLQ ≤ 15

USB OTG FS clock frequency = VCO frequency / PLLQ with 2 0000: PLLQ = 0, wrong configuration

0001: PLLQ = 1, wrong configuration

0010: PLLQ = 2

0011: PLLQ = 3

0100: PLLQ = 4

...

1111: PLLQ = 15

Bit 23 Reserved, must be kept at reset value.

Bit 22 PLLSRC:

Main PLL(PLL) and audio PLL (PLLI2S) entry clock source

Set and cleared by software to select PLL and PLLI2S clock source. This bit can be written

only when PLL and PLLI2S are disabled.

0: HSI clock selected as PLL and PLLI2S clock entry 1: HSE oscillator clock selected as PLL and PLLI2S clock entry Bits 21:18 Reserved, must be kept at reset value.



212

--- Page 164 ---

**Reset and clock control for   STM32F42xxx and STM32F43xxx (RCC) RM0090**

Bits 17:16 PLLP: Main PLL (PLL) division factor for main system clock

Set and cleared by software to control the frequency of the general PLL output clock. These bits can be written only if PLL is disabled.

Caution: The software has to set these bits correctly not to exceed 180 MHz on this domain. PLL output clock frequency = VCO frequency / PLLP with PLLP = 2, 4, 6, or 8 00: PLLP = 2

01: PLLP = 4

10: PLLP = 6

11: PLLP = 8

Bits 14:6 PLLN: Main PLL (PLL) multiplication factor for VCO

Set and cleared by software to control the multiplication factor of the VCO. These bits can be written only when PLL is disabled. Only half-word and word accesses are allowed to write these bits.

**Caution: The software has to set these bits correctly to ensure that the VCO output**

frequency is between 100 and 432 MHz.

VCO output frequency = VCO input frequency × PLLN with 50 ≤ PLLN ≤ 432 000000000: PLLN = 0, wrong configuration

000000001: PLLN = 1, wrong configuration

...

000110010: PLLN = 50

...

001100011: PLLN = 99

001100100: PLLN = 100

...

110110000: PLLN = 432

110110001: PLLN = 433, wrong configuration

...

111111111: PLLN = 511, wrong configuration

Note: Multiplication factors ranging from 50 and 99 are possible for VCO input frequency

higher than 1 MHz. However care must be taken that the minimum VCO output frequency respects the value specified above.

Bits 5:0 PLLM: Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock Set and cleared by software to divide the PLL and PLLI2S input clock before the VCO. These bits can be written only when the PLL and PLLI2S are disabled.

**Caution: The software has to set these bits correctly to ensure that the VCO input frequency**

ranges from 1 to 2 MHz. It is recommended to select a frequency of 2 MHz to limit PLL jitter.

VCO input frequency = PLL input clock frequency / PLLM with 2 ≤ PLLM ≤ 63 000000: PLLM = 0, wrong configuration

000001: PLLM = 1, wrong configuration

000010: PLLM = 2

000011: PLLM = 3

000100: PLLM = 4

...

111110: PLLM = 62

111111: PLLM = 63

164/1749 RM0090 Rev 18

--- Page 165 ---

**RM0090 Reset and clock control for   STM32F42xxx and STM32F43xxx (RCC)**


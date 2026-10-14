### 6.3.23 RCC PLLI2S configuration register (RCC_PLLI2SCFGR)

Address offset: 0x84
Reset value: 0x2400 3000
Access: no wait state, word, half-word and byte access.
This register is used to configure the PLLI2S clock outputs according to the formulas:
f(VCO clock) = f(PLLI2S clock input) × (PLLI2SN / PLLM)
f(PLL I2S clock output) = f(VCO clock) / PLLI2SR

| 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|
| Reserved | PLLI2S R2 | PLLI2S R1 | PLLI2S R0 | PLLI2SQ | Reserved |  |  |  |  |  |
| rw |  |  |  |  |  |  |  |  |  |  |
| 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0 |  |  |  |  |  |  |  |  |  |  |
| Reserved | PLLI2SN 8 | PLLI2SN 7 | PLLI2SN 6 | PLLI2SN 5 | PLLI2SN 4 | PLLI2SN 3 | PLLI2SN 2 | PLLI2SN 1 | PLLI2SN 0 | Reserved |
| rw |  |  |  |  |  |  |  |  |  |  |


ST
RM0090 Rev 18
203/1749

--- Page 9 ---

**Reset and clock control for STM32F42xxx and STM32F43xxx (RCC)** RM0090

Bit 31 Reserved, must be kept at reset value.

Bits 30:28 **PLL2SR**: PLLI2S division factor for I2S clocks

These bits are set and cleared by software to control the I2S clock frequency. These bits should be written only if the PLLI2S is disabled. The factor must be chosen in accordance with the prescaler values inside the I2S peripherals, to reach 0.3% error when using standard crystals and 0% error with audio crystals. For more information about I2S clock frequency and precision, refer to *Section 28.4.4: Clock generator* in the I2S chapter.

**Caution**: The I2Ss requires a frequency lower than or equal to 192 MHz to work correctly.
I2S clock frequency = VCO frequency / PLLR with 2 ≤ PLLR ≤ 7
000: PLLR = 0, wrong configuration
001: PLLR = 1, wrong configuration
010: PLLR = 2
...
111: PLLR = 7

Bits 27:24 **PLL2SQ**: PLLI2S division factor for SAI1 clock

These bits are set and cleared by software to control the SAI1 clock frequency.
They should be written when the PLLI2S is disabled.
SAI1 clock frequency = VCO frequency / PLLI2SQ with 2 <= PLLI2SQ <= 15
0000: PLLI2SQ = 0, wrong configuration
0001: PLLI2SQ = 1, wrong configuration
0010: PLLI2SQ = 2
0011: PLLI2SQ = 3
0100: PLLI2SQ = 4
0101: PLLI2SQ = 5
...
1111: PLLI2SQ = 15

204/1749 RM0090 Rev 18

--- Page 10 ---

**RM0090** **Reset and clock control for STM32F42xxx and STM32F43xxx (RCC)**

Bits 23:15 Reserved, must be kept at reset value.

Bits 14:6 **PLL2SN**: PLLI2S multiplication factor for VCO

These bits are set and cleared by software to control the multiplication factor of the VCO.
These bits can be written only when PLLI2S is disabled. Only half-word and word accesses are allowed to write these bits.

**Caution**: The software has to set these bits correctly to ensure that the VCO output frequency is between 100 and 432 MHz.
VCO output frequency = VCO input frequency × PLLI2SN with 50 ≤PLL2SN ≤432
00000000: PLLI2SN = 0, wrong configuration
00000001: PLLI2SN = 1, wrong configuration
...
000110010: PLLI2SN = 50
...
001100011: PLLI2SN = 99
001100100: PLLI2SN = 100
001100101: PLLI2SN = 101
001100110: PLLI2SN = 102
...
110110000: PLLI2SN = 432
110110001: PLLI2SN = 433, wrong configuration
...
11111111: PLLI2SN = 511, wrong configuration

*Note: Multiplication factors ranging from 50 and 99 are possible for VCO input frequency higher than 1 MHz. However care must be taken that the minimum VCO output frequency respects the value specified above.*

Bits 5:0 Reserved, must be kept at reset value.

STI
RM0090 Rev 18
205/1749

--- Page 11 ---

**Reset and clock control for STM32F42xxx and STM32F43xxx (RCC)** RM0090


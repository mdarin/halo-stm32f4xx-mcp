**6.3.24 RCC PLL configuration register (RCC_PLLSAICFGR)**

Address offset: 0x88
Reset value: 0x2400 3000
Access: no wait state, word, half-word and byte access.

This register is used to configure the PLLSAI clock outputs according to the formulas:

*   f(VCO clock) = f(PLLSAI clock input) × (PLLSAIN / PLLM)
*   f(PLLSAI1 clock output) = f(VCO clock) / PLLSAIQ
*   f(PLL LCD clock output) = f(VCO clock) / PLLSAIR

| 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|
| Reserved | PLLSAIR | PLLSAIQ | Reserved |  |  |  |  |
| rw |  |  |  |  |  |  |  |
| 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0 |  |  |  |  |  |  |  |
| Reserved | PLLSAIN | Reserved |  |  |  |  |  |
| rw |  |  |  |  |  |  |  |


Bit 31 Reserved, must be kept at reset value.

**Bits 30:28 PLLSAIR:** PLLSAI division factor for LCD clock
Set and reset by software to control the LCD clock frequency.
These bits should be written when the PLLSAI is disabled.
LCD clock frequency = VCO frequency / PLLSAIR with 2 ≤PLLSAIR ≤7
000: PLLSAIR = 0, wrong configuration
001: PLLSAIR = 1, wrong configuration
010: PLLSAIR = 2
...
111: PLLSAIR = 7

**Bits 27:24 PLLSAIQ:** PLLSAI division factor for SAI1 clock
Set and reset by software to control the frequency of SAI1 clock.
These bits should be written when the PLLSAI is disabled.
SAI1 clock frequency = VCO frequency / PLLSAIQ with 2 ≤PLLSAIQ ≤5
0000: PLLSAIQ = 0, wrong configuration
0001: PLLSAIQ = 1, wrong configuration
...
0010: PLLSAIQ = 2
0011: PLLSAIQ = 3
0100: PLLSAIQ = 4
0101: PLLSAIQ = 5
...
1111: PLLSAIQ = 15

206/1749 RM0090 Rev 18 ST

--- Page 12 ---

RM0090
Reset and clock control for STM32F42xxx and STM32F43xxx (RCC)

Bits 23:15 Reserved, must be kept at reset value.

Bits 14:6 **PLLSAIN**: PLLSAI division factor for VCO

These bits are set and cleared by software to control the multiplication factor of the VCO.
These bits can be written only when PLLSAI is disabled. Only half-word and word accesses are allowed to write these bits.

**Caution**: The software has to set these bits correctly to ensure that the VCO output frequency is between 100 and 432 MHz.

VCO output frequency = VCO input frequency × PLLSAIN with 50 ≤PLLSAIN ≤432
00000000: PLLSAIN = 0, wrong configuration
00000001: PLLSAIN = 1, wrong configuration
...
000110010: PLLSAIN = 50
...
001100011: PLLSAIN = 99
001100100: PLLSAIN = 100
001100101: PLLSAIN = 101
001100110: PLLSAIN = 102
...
110110000: PLLSAIN = 432
110110001: PLLSAIN = 433, wrong configuration
...
11111111: PLLSAIN = 511, wrong configuration

*Note: Multiplication factors ranging from 50 and 99 are possible for VCO input frequency higher than 1 MHz. However care must be taken that the minimum VCO output frequency respects the value specified above.*

Bits 5:0 Reserved, must be kept at reset value


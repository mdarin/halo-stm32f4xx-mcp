### 7.3.23 RCC PLLI2S configuration register (RCC_PLLI2SCFGR)

Address offset: 0x84
Reset value: 0x2000 3000
Access: no wait state, word, half-word and byte access.

This register is used to configure the PLLI2S clock outputs according to the formulas:
* f(VCO clock) = f(PLLI2S clock input) × (PLLI2SN / PLLM)
* f(PLL I2S clock output) = f(VCO clock) / PLLI2SR

| Reserved | PLLI2S R2 | PLLI2S R1 | PLLI2S R0 | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reserved | PLLI2SN 8 | PLLI2SN 7 | PLLI2SN 6 | PLLI2SN 5 | PLLI2SN 4 | PLLI2SN 3 | PLLI2SN 2 | PLLI2SN 1 | PLLI2SN 0 | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bit 31 Reserved, must be kept at reset value.
Bits 30:28 **PLLI2SR**: PLLI2S division factor for I2S clocks
Set and cleared by software to control the I2S clock frequency. These bits should be written only if the PLLI2S is disabled. The factor must be chosen in accordance with the prescaler values inside the I2S peripherals, to reach 0.3% error when using standard crystals and 0% error with audio crystals. For more information about I2S clock frequency and precision, refer to *Section 28.4.4: Clock generator* in the I2S chapter.

**Caution:** The I2Ss requires a frequency lower than or equal to 192 MHz to work correctly.
I2S clock frequency = VCO frequency / PLLR with 2 ≤PLLR ≤7
000: PLLR = 0, wrong configuration
001: PLLR = 1, wrong configuration
010: PLLR = 2
...
111: PLLR = 7

ST
RM0090 Rev 18
263/1749

--- Page 69 ---

**Reset and clock control for STM32F405xx/07xx and STM32F415xx/17xx(RCC)** RM0090

Bits 27:15 Reserved, must be kept at reset value.

Bits 14:6 **PLLI2SN**: PLLI2S multiplication factor for VCO

These bits are set and cleared by software to control the multiplication factor of the VCO.
These bits can be written only when PLLI2S is disabled. Only half-word and word accesses are allowed to write these bits.

**Caution:** The software has to set these bits correctly to ensure that the VCO output frequency is between 100 and 432 MHz.
VCO output frequency = VCO input frequency × PLLI2SN with 50 ≤PLLI2SN ≤432
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
111111111: PLLI2SN = 511, wrong configuration

*Note: Multiplication factors ranging from 50 and 99 are possible for VCO input frequency higher than 1 MHz. However care must be taken that the minimum VCO output frequency respects the value specified above.*

Bits 5:0 Reserved, must be kept at reset value.

264/1749 RM0090 Rev 18

--- Page 70 ---

RM0090
Reset and clock control for STM32F405xx/07xx and STM32F415xx/17xx (RCC)

7.3.24 RCC register map
Table 34 gives the register map and reset values.

| Addr. offset | Register name | Table 34. RCC register map and reset values |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 0x000 | 0x004 | 0x008 | 0x0C | 0x10 | 0x14 | 0x18 | 0x1C | 0x20 | 0x24 | 0x28 | 0x2C | 0x30 | 0x34 | 0x38 | 0x3C | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27 | 28 | 29 | 30 | 31 |
| 0x000 | RCC_CR | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x004 | RCC_PLLCFGR | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x008 | RCC_CFGR | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x0C | RCC_CIR | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x10 | RCC_AHB1RSTR | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x14 | RCC_AHB2RSTR | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x18 | RCC_AHB3RSTR | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x1C | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x20 | RCC_APB1RSTR | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x24 | RCC_APB2RSTR | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x28 | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x2C | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x30 | RCC_AHB1ENR | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x34 | RCC_AHB2ENR | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x38 | RCC_AHB3ENR | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x3C | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


RM0090 Rev 18
265/1749

--- Page 71 ---

RM0090
Reset and clock control for STM32F405xx/07xx and STM32F415xx/17xx(RCC)

Table 34. RCC register map and reset values (continued)

| Addr. offset | Register name | Reset values |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
| 0x40 | RCC_APB1ENR | Reserved | OTGHSLPILPEN | OTGSLPEN | RNGPEN | GPIOHPEN | GPIOGPEN | GPIOFPEN | GPIOEPEN | GPIOGPEN | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x44 | RCC_APB2ENR | Reserved | OTGHSLPILPEN | OTGSLPEN | DACLPEN | PWRLPEN | ETHMACPTPLPEN | ETHMACRXLPEN | ETHMACPTXL PEN | ETHMACLPEN | CAN1LPEN | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x48 | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x4C | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x50 | RCC_AHB1LPENR | Reserved | OTGHSLPILPEN | OTGSLPEN | DACLPEN | PWRLPEN | ETHMACPTPLPEN | ETHMACRXLPEN | ETHMACPTXL PEN | ETHMACLPEN | CAN1LPEN | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x54 | RCC_AHB2LPENR | Reserved | OTGHSLPILPEN | OTGSLPEN | DACLPEN | PWRLPEN | ETHMACPTPLPEN | ETHMACRXLPEN | ETHMACPTXL PEN | ETHMACLPEN | CAN1LPEN | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x58 | RCC_AHB3LPENR | Reserved | OTGHSLPILPEN | OTGSLPEN | DACLPEN | PWRLPEN | ETHMACPTPLPEN | ETHMACRXLPEN | ETHMACPTXL PEN | ETHMACLPEN | CAN1LPEN | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x5C | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x60 | RCC_APB1LPENR | Reserved | OTGHSLPILPEN | OTGSLPEN | DACLPEN | PWRLPEN | ETHMACPTPLPEN | ETHMACRXLPEN | ETHMACPTXL PEN | ETHMACLPEN | CAN1LPEN | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x64 | RCC_APB2LPENR | Reserved | OTGHSLPILPEN | OTGSLPEN | DACLPEN | PWRLPEN | ETHMACPTPLPEN | ETHMACRXLPEN | ETHMACPTXL PEN | ETHMACLPEN | CAN1LPEN | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x68 | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x6C | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x70 | RCC_BDCR | Reserved | OTGHSLPILPEN | OTGSLPEN | DACLPEN | PWRLPEN | ETHMACPTPLPEN | ETHMACRXLPEN | ETHMACPTXL PEN | ETHMACLPEN | CAN1LPEN | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x74 | RCC_CSR | Reserved | OTGHSLPILPEN | OTGSLPEN | DACLPEN | PWRLPEN | ETHMACPTPLPEN | ETHMACRXLPEN | ETHMACPTXL PEN | ETHMACLPEN | CAN1LPEN | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x78 | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x7C | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x80 | RCC_SSCGR | Reserved | OTGHSLPILPEN | OTGSLPEN | DACLPEN | PWRLPEN | ETHMACPTPLPEN | ETHMACRXLPEN | ETHMACPTXL PEN | ETHMACLPEN | CAN1LPEN | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x84 | RCC_PLLI2SCFGR | Reserved | OTGHSLPILPEN | OTGSLPEN | DACLPEN | PWRLPEN | ETHMACPTPLPEN | ETHMACRXLPEN | ETHMACPTXL PEN | ETHMACLPEN | CAN1LPEN | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Refer to Section 2.3: Memory map for the register boundary addresses.
266/1749
RM0090 Rev 18

--- Page 72 ---

RM0090
General-purpose I/Os (GPIO)


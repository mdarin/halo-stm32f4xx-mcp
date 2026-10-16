### 6.3.19 RCC APB2 peripheral clock enabled in low power mode register (RCC_APB2LPENR)

Address offset: 0x64
Reset value: 0x0x0477 7F33
Access: no wait state, word, half-word and byte access.

| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved |  |  |  |  | LTDC LPEN | Reserved |  |  | SAI1 LPEN | SPI6 LPEN | SPI5 LPEN | Reserved | TIM11 LPEN | TIM10 LPEN | TIM9 LPEN |  |  |  |
|  |  |  |  |  |  |  |  |  |  | rw |  |  |  |  |  | rw |  | rw |
| 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |  |  |  |
| Reserved | SYSC FG LPEN | SPI4 LPEN | SPI1 LPEN | SDIO LPEN | ADC3 LPEN | ADC2 LPEN | ADC1 LPEN | Reserved | USART 6 LPEN | USART 1 LPEN | Reserved | TIM8 LPEN | TIM1 LPEN |  |  |  |  |  |
|  |  |  |  |  |  | rw |  | rw |  | rw |  |  |  |  |  |  |  |  |


Bits 31:27 Reserved, must be kept at reset value.

**Bit 26 LTDC LPEN:** LTDC clock enable during Sleep mode
This bit is set and cleared by software.
0: LTDC clock disabled during Sleep mode
1: LTDC clock enabled during Sleep mode

Bits 25:23 Reserved, must be kept at reset value.

**Bit 22 SAI1 LPEN:** SAI1 clock enable during Sleep mode
This bit is set and cleared by software.
0: SAI1 clock disabled during Sleep mode
1: SAI1 clock enabled during Sleep mode

**Bit 21 SPI6 LPEN:** SPI6 clock enable during Sleep mode
This bit is set and cleared by software.
0: SPI6 clock disabled during Sleep mode
1: SPI6 clock enabled during Sleep mode

**Bit 20 SPI5 LPEN:** SPI5 clock enable during Sleep mode
This bit is set and cleared by software.
0: SPI5 clock disabled during Sleep mode
1: SPI5 clock enabled during Sleep mode

Bit 19 Reserved, must be kept at reset value.

**Bit 18 TIM11 LPEN:** TIM11 clock enable during Sleep mode
This bit is set and cleared by software.
0: TIM11 clock disabled during Sleep mode
1: TIM11 clock enabled during Sleep mode

**Bit 17 TIM10 LPEN:** TIM10 clock enable during Sleep mode
This bit is set and cleared by software.
0: TIM10 clock disabled during Sleep mode
1: TIM10 clock enabled during Sleep mode

ST
RM0090 Rev 18
197/1749

--- Page 3 ---

**Reset and clock control for STM32F42xxx and STM32F43xxx (RCC)**

RM0090

Bit 16 **TIM9LPEN**: TIM9 clock enable during sleep mode
This bit is set and cleared by software.
0: TIM9 clock disabled during Sleep mode
1: TIM9 clock enabled during Sleep mode

Bit 15 Reserved, must be kept at reset value.

Bit 14 **SYSCFGLPEN**: System configuration controller clock enable during Sleep mode
This bit is set and cleared by software.
0: System configuration controller clock disabled during Sleep mode
1: System configuration controller clock enabled during Sleep mode

Bit 13 **SPI4LPEN**: SPI4 clock enable during Sleep mode
This bit is set and cleared by software.
0: SPI4 clock disabled during Sleep mode
1: SPI4 clock enabled during Sleep mode

Bit 12 **SPI1LPEN**: SPI1 clock enable during Sleep mode
This bit is set and cleared by software.
0: SPI1 clock disabled during Sleep mode
1: SPI1 clock enabled during Sleep mode

Bit 11 **SDIOLPEN**: SDIO clock enable during Sleep mode
This bit is set and cleared by software.
0: SDIO module clock disabled during Sleep mode
1: SDIO module clock enabled during Sleep mode

Bit 10 **ADC3LPEN**: ADC 3 clock enable during Sleep mode
This bit is set and cleared by software.
0: ADC 3 clock disabled during Sleep mode
1: ADC 3 clock disabled during Sleep mode

Bit 9 **ADC2LPEN**: ADC2 clock enable during Sleep mode
This bit is set and cleared by software.
0: ADC2 clock disabled during Sleep mode
1: ADC2 clock disabled during Sleep mode

Bit 8 **ADC1LPEN**: ADC1 clock enable during Sleep mode
This bit is set and cleared by software.
0: ADC1 clock disabled during Sleep mode
1: ADC1 clock disabled during Sleep mode

Bits 7:6 Reserved, must be kept at reset value.

Bit 5 **USART6LPEN**: USART6 clock enable during Sleep mode
This bit is set and cleared by software.
0: USART6 clock disabled during Sleep mode
1: USART6 clock enabled during Sleep mode

Bit 4 **USART1LPEN**: USART1 clock enable during Sleep mode
This bit is set and cleared by software.
0: USART1 clock disabled during Sleep mode
1: USART1 clock enabled during Sleep mode

198/1749
RM0090 Rev 18

--- Page 4 ---

RM0090
Reset and clock control for STM32F42xxx and STM32F43xxx (RCC)

Bits 3:2 Reserved, must be kept at reset value.

Bit 1 **TIM8LPEN**: TIM8 clock enable during Sleep mode
This bit is set and cleared by software.
0: TIM8 clock disabled during Sleep mode
1: TIM8 clock enabled during Sleep mode

Bit 0 **TIM1LPEN**: TIM1 clock enable during Sleep mode
This bit is set and cleared by software.
0: TIM1 clock disabled during Sleep mode
1: TIM1 clock enabled during Sleep mode


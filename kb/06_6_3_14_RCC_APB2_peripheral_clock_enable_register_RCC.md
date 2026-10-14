**6.3.14 RCC APB2 peripheral clock enable register (RCC_APB2ENR)**

Address offset: 0x44

Reset value: 0x0000 0000

Access: no wait state, word, half-word and byte access.

31 30 29 28 27 26 25 24

23 22 21 20 19 18 17 16

| Reserved | LTDC EN | Reserved | SAI1EN | SPI6EN | SPI5EN | Res. | TIM11 EN | TIM10 EN | TIM9 EN |
|---|---|---|---|---|---|---|---|---|---|
| rw |  |  |  |  |  |  |  |  |  |


15 14 13 12 11 10 9 8

7 6 5 4 3 2 1 0

| Reser- ved | SYSCF G EN | SPI4E N | SPI1 EN | SDIO EN | ADC3 EN | ADC2 EN | ADC1 EN | Reserved | USART 6 EN | USART 1 EN | Reserved | TIM8 EN | TIM1 EN |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bits 31:27 Reserved, must be kept at reset value.

Bit 26 LTDCEN: LTDC clock enable

This bit is set and cleared by software.

0: LTDC clock disabled

1: LTDC clock enabled

Bits 27: 23 Reserved, must be kept at reset value.

Bit 22 SAI1EN: SAI1 clock enable

This bit is set and cleared by software.

0: SAI1 clock disabled

1: SAI1 clock enabled

Bit 21 SPI6EN: SPI6 clock enable

This bit is set and cleared by software.

0: SPI6 clock disabled

1: SPI6 clock enabled

Bit 20 SPI5EN: SPI5 clock enable

This bit is set and cleared by software.

0: SPI5 clock disabled

1: SPI5 clock enabled

Bit 18 TIM11EN: TIM11 clock enable

This bit is set and cleared by software.

0: TIM11 clock disabled

1: TIM11 clock enabled

Bit 17 TIM10EN: TIM10 clock enable

This bit is set and cleared by software.

0: TIM10 clock disabled

1: TIM10 clock enabled

Bit 16 TIM9EN: TIM9 clock enable

This bit is set and cleared by software.

0: TIM9 clock disabled

1: TIM9 clock enabled

Bit 15 Reserved, must be kept at reset value.



212

--- Page 188 ---

**Reset and clock control for   STM32F42xxx and STM32F43xxx (RCC) RM0090**

Bit 14 SYSCFGEN: System configuration controller clock enable

This bit is set and cleared by software.

0: System configuration controller clock disabled

1: System configuration controller clock enabled

Bit 13 SPI4EN: SPI4 clock enable

This bit is set and cleared by software.

0: SPI4 clock disabled

1: SPI4 clock enabled

Bit 12 SPI1EN: SPI1 clock enable

This bit is set and cleared by software.

0: SPI1 clock disabled

1: SPI1 clock enabled

Bit 11 SDIOEN: SDIO clock enable

This bit is set and cleared by software.

0: SDIO module clock disabled

1: SDIO module clock enabled

Bit 10 ADC3EN: ADC3 clock enable

This bit is set and cleared by software.

0: ADC3 clock disabled

1: ADC3 clock enabled

Bit 9 ADC2EN: ADC2 clock enable

This bit is set and cleared by software.

0: ADC2 clock disabled

1: ADC2 clock enabled

Bit 8 ADC1EN: ADC1 clock enable

This bit is set and cleared by software.

0: ADC1 clock disabled

1: ADC1 clock enabled

Bits 7:6 Reserved, must be kept at reset value.

Bit 5 USART6EN: USART6 clock enable

This bit is set and cleared by software.

0: USART6 clock disabled

1: USART6 clock enabled

Bit 4 USART1EN: USART1 clock enable

This bit is set and cleared by software.

0: USART1 clock disabled

1: USART1 clock enabled

Bits 3:2 Reserved, must be kept at reset value.

Bit 1 TIM8EN: TIM8 clock enable

This bit is set and cleared by software.

0: TIM8 clock disabled

1: TIM8 clock enabled

Bit 0 TIM1EN: TIM1 clock enable

This bit is set and cleared by software.

0: TIM1 clock disabled

1: TIM1 clock enabled

188/1749 RM0090 Rev 18

--- Page 189 ---

**RM0090 Reset and clock control for   STM32F42xxx and STM32F43xxx (RCC)**


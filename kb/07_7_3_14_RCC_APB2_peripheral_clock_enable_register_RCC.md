### 7.3.14 RCC APB2 peripheral clock enable register (RCC_APB2ENR)

Address offset: 0x44
Reset value: 0x0000 0000
Access: no wait state, word, half-word and byte access.

| 31 | 15 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved | SYSCFG ENrw | Reserved | SPI1 ENrw | SDIO ENrw | ADC3 ENrw | ADC2 ENrw | ADC1 ENrw | Reserved | USART 6 ENrw | USART 1 ENrw | Reserved | TIM8 ENrw | TIM1 ENrw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
| Reserved | SYSCFG ENrw | Reserved | SPI1 ENrw | SDIO ENrw | ADC3 ENrw | ADC2 ENrw | ADC1 ENrw | Reserved | USART 6 ENrw | USART 1 ENrw | Reserved | TIM8 ENrw | TIM1 ENrw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |


Bits 31:19 Reserved, must be kept at reset value.

Bit 18 **TIM11EN**: TIM11 clock enable
Set and cleared by software.
0: TIM11 clock disabled
1: TIM11 clock enabled

Bit 17 **TIM10EN**: TIM10 clock enable
Set and cleared by software.
0: TIM10 clock disabled
1: TIM10 clock enabled

Bit 16 **TIM9EN**: TIM9 clock enable
Set and cleared by software.
0: TIM9 clock disabled
1: TIM9 clock enabled

Bit 15 Reserved, must be kept at reset value.

248/1749 RM0090 Rev 18

--- Page 54 ---

**RM0090** **Reset and clock control for STM32F405xx/07xx and STM32F415xx/17xx(RCC)**

Bit 14 **SYSCFGEN**: System configuration controller clock enable
Set and cleared by software.
0: System configuration controller clock disabled
1: System configuration controller clock enabled

Bit 13 Reserved, must be kept at reset value.

Bit 12 **SPI1EN**: SPI1 clock enable
Set and cleared by software.
0: SPI1 clock disabled
1: SPI1 clock enabled

Bit 11 **SDIOEN**: SDIO clock enable
Set and cleared by software.
0: SDIO module clock disabled
1: SDIO module clock enabled

Bit 10 **ADC3EN**: ADC3 clock enable
Set and cleared by software.
0: ADC3 clock disabled
1: ADC3 clock disabled

Bit 9 **ADC2EN**: ADC2 clock enable
Set and cleared by software.
0: ADC2 clock disabled
1: ADC2 clock disabled

Bit 8 **ADC1EN**: ADC1 clock enable
Set and cleared by software.
0: ADC1 clock disabled
1: ADC1 clock disabled

Bits 7:6 Reserved, must be kept at reset value.

Bit 5 **USART6EN**: USART6 clock enable
Set and cleared by software.
0: USART6 clock disabled
1: USART6 clock enabled

Bit 4 **USART1EN**: USART1 clock enable
Set and cleared by software.
0: USART1 clock disabled
1: USART1 clock enabled

Bits 3:2 Reserved, must be kept at reset value.

Bit 1 **TIM8EN**: TIM8 clock enable
Set and cleared by software.
0: TIM8 clock disabled
1: TIM8 clock enabled

Bit 0 **TIM1EN**: TIM1 clock enable
Set and cleared by software.
0: TIM1 clock disabled
1: TIM1 clock enabled

ST
RM0090 Rev 18
249/1749

--- Page 55 ---

Reset and clock control for STM32F405xx/07xx and STM32F415xx/17xx(RCC) RM0090


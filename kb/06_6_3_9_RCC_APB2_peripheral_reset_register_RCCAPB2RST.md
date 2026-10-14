**6.3.9 RCC APB2 peripheral reset register (RCC_APB2RSTR)**

Address offset: 0x24

Reset value: 0x0000 0000

Access: no wait state, word, half-word and byte access.

31 30 29 28 27 26 25 24

23 22 21 20 19 18 17 16

| Reserved | LTD CRST | Reserved | SAI1 RST | SPI6 RST | SPI5 RST | Res. | TIM11 RST | TIM10 RST | TIM9 RST |
|---|---|---|---|---|---|---|---|---|---|
| rw |  |  |  |  |  |  |  |  |  |


15 14 13 12 11 10 9 8

7 6 5 4 3 2 1 0

| Reser- ved | SYSCF G RST | SPI4 RST | SPI1 RST | SDIO RST | Reserved | ADC RST | Reserved | USART 6 RST | USART 1 RST | Reserved | TIM8 RST | TIM1 RST |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| rw |  |  |  |  |  |  |  |  |  |  |  |  |


Bits 31:27 Reserved, must be kept at reset value.

Bit 26 LTDCRST : LTDC reset

This bit is set and reset by software.

0: does not reset LCD-TFT

1: resets LCD-TFT

Bits 27:23 Reserved, must be kept at reset value.

Bit 22 SAI1RST : SAI1 reset

This bit is set and reset by software.

0: does not reset SAI1

1: resets SAI1

Bit 21 SPI6RST : SPI6 reset

This bit is set and cleared by software.

0: does not reset SPI6

1: resets SPI6

Bit 20 SPI5RST : SPI5 reset

This bit is set and cleared by software.

0: does not reset SPI5

1: resets SPI5

Bit 19 Reserved, must be kept at reset value.

Bit 18 TIM11RST: TIM11 reset

This bit is set and cleared by software.

0: does not reset TIM11

1: resets TIM14

Bit 17 TIM10RST: TIM10 reset

This bit is set and cleared by software.

0: does not reset TIM10

1: resets TIM10

Bit 16 TIM9RST: TIM9 reset

This bit is set and cleared by software.

0: does not reset TIM9

1: resets TIM9



--- Page 179 ---

**RM0090**

**Reset and clock control for   STM32F42xxx and STM32F43xxx (RCC)**

Bit 15 Reserved, must be kept at reset value.

Bit 14 SYSCFGRST: System configuration controller reset This bit is set and cleared by software.

0: does not reset the System configuration controller 1: resets the System configuration controller Bit 13 SPI4RST : SPI4 reset

This bit is set and cleared by software.

0: does not reset SPI4

1: resets SPI4

Bit 12 SPI1RST: SPI1 reset

This bit is set and cleared by software.

0: does not reset SPI1

1: resets SPI1

Bit 11 SDIORST: SDIO reset

This bit is set and cleared by software.

0: does not reset the SDIO module 1: resets the SDIO module

Bits 10:9 Reserved, must be kept at reset value.

Bit 8 ADCRST: ADC interface reset (common to all ADCs) This bit is set and cleared by software.

0: does not reset the ADC interface 1: resets the ADC interface

Bits 7:6 Reserved, must be kept at reset value.

Bit 5 USART6RST: USART6 reset

This bit is set and cleared by software.

0: does not reset USART6

1: resets USART6

Bit 4 USART1RST: USART1 reset

This bit is set and cleared by software.

0: does not reset USART1

1: resets USART1

Bits 3:2 Reserved, must be kept at reset value.

Bit 1 TIM8RST: TIM8 reset

This bit is set and cleared by software.

0: does not reset TIM8

1: resets TIM8

Bit 0 TIM1RST: TIM1 reset

This bit is set and cleared by software.

0: does not reset TIM1

1: resets TIM1

RM0090 Rev 18 179/1749

212

--- Page 180 ---

**Reset and clock control for   STM32F42xxx and STM32F43xxx (RCC)**

**RM0090**


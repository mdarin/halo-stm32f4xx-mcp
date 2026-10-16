**6.3.13 RCC APB1 peripheral clock enable register (RCC_APB1ENR)**

Address offset: 0x40

Reset value: 0x0000 0000

Access: no wait state, word, half-word and byte access.

31 30 29 28 27 26 25 24

23 22 21 20 19 18 17 16

| UART8 EN | UART7 EN | DAC EN | PWR EN | Reser- ved | CAN2 EN | CAN1 EN | Reser- ved | I2C3 EN | I2C2 EN | I2C1 EN | UART5 EN | UART4 EN | USART 3 EN | USART 2 EN | Reser- ved |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


15 14 13 12 11 10 9 8

7 6 5 4 3 2 1 0

| SPI3 EN | SPI2 EN | Reserved | WWDG EN | Reserved | TIM14 EN | TIM13 EN | TIM12 EN | TIM7 EN | TIM6 EN | TIM5 EN | TIM4 EN | TIM3 EN | TIM2 EN |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |


RM0090 Rev 18 183/1749

212

--- Page 184 ---

**Reset and clock control for   STM32F42xxx and STM32F43xxx (RCC) RM0090**

Bit 31 UART8EN : UART8 clock enable

This bit is set and cleared by software.

0: UART8 clock disabled

1: UART8 clock enabled

Bit 30 UART7EN : UART7 clock enable

This bit is set and cleared by software.

0: UART7 clock disabled

1: UART7 clock enabled

Bit 29 DACEN: DAC interface clock enable

This bit is set and cleared by software.

0: DAC interface clock disabled

1: DAC interface clock enable

Bit 28 PWREN: Power interface clock enable

This bit is set and cleared by software.

0: Power interface clock disabled

1: Power interface clock enable

Bit 27 Reserved, must be kept at reset value.

Bit 26 CAN2EN: CAN 2 clock enable

This bit is set and cleared by software.

0: CAN 2 clock disabled

1: CAN 2 clock enabled

Bit 25 CAN1EN: CAN 1 clock enable

This bit is set and cleared by software.

0: CAN 1 clock disabled

1: CAN 1 clock enabled

Bit 24 Reserved, must be kept at reset value.

Bit 23 I2C3EN: I2C3 clock enable

This bit is set and cleared by software.

0: I2C3 clock disabled

1: I2C3 clock enabled

Bit 22 I2C2EN: I2C2 clock enable

This bit is set and cleared by software.

0: I2C2 clock disabled

1: I2C2 clock enabled

Bit 21 I2C1EN: I2C1 clock enable

This bit is set and cleared by software.

0: I2C1 clock disabled

1: I2C1 clock enabled

Bit 20 UART5EN: UART5 clock enable

This bit is set and cleared by software.

0: UART5 clock disabled

1: UART5 clock enabled

Bit 19 UART4EN: UART4 clock enable

This bit is set and cleared by software.

0: UART4 clock disabled

1: UART4 clock enabled

184/1749 RM0090 Rev 18

--- Page 185 ---

**RM0090**

**Reset and clock control for   STM32F42xxx and STM32F43xxx (RCC)**

Bit 18 USART3EN: USART3 clock enable This bit is set and cleared by software. 0: USART3 clock disabled 1: USART3 clock enabled Bit 17 USART2EN: USART2 clock enable This bit is set and cleared by software. 0: USART2 clock disabled 1: USART2 clock enabled Bit 16 Reserved, must be kept at reset value. Bit 15 SPI3EN: SPI3 clock enable This bit is set and cleared by software. 0: SPI3 clock disabled 1: SPI3 clock enabled Bit 14 SPI2EN: SPI2 clock enable This bit is set and cleared by software. 0: SPI2 clock disabled 1: SPI2 clock enabled Bits 13:12 Reserved, must be kept at reset value. Bit 11 WWDGEN: Window watchdog clock enable This bit is set and cleared by software. 0: Window watchdog clock disabled 1: Window watchdog clock enabled Bit 10:9 Reserved, must be kept at reset value. Bit 8 TIM14EN: TIM14 clock enable This bit is set and cleared by software. 0: TIM14 clock disabled 1: TIM14 clock enabled Bit 7 TIM13EN: TIM13 clock enable This bit is set and cleared by software. 0: TIM13 clock disabled 1: TIM13 clock enabled Bit 6 TIM12EN: TIM12 clock enable This bit is set and cleared by software. 0: TIM12 clock disabled 1: TIM12 clock enabled Bit 5 TIM7EN: TIM7 clock enable This bit is set and cleared by software. 0: TIM7 clock disabled 1: TIM7 clock enabled Bit 4 TIM6EN: TIM6 clock enable This bit is set and cleared by software. 0: TIM6 clock disabled 1: TIM6 clock enabled

RM0090 Rev 18 185/1749

212

--- Page 186 ---

**Reset and clock control for   STM32F42xxx and STM32F43xxx (RCC) RM0090**

Bit 3 TIM5EN: TIM5 clock enable

This bit is set and cleared by software.

0: TIM5 clock disabled

1: TIM5 clock enabled

Bit 2 TIM4EN: TIM4 clock enable

This bit is set and cleared by software.

0: TIM4 clock disabled

1: TIM4 clock enabled

Bit 1 TIM3EN: TIM3 clock enable

This bit is set and cleared by software.

0: TIM3 clock disabled

1: TIM3 clock enabled

Bit 0 TIM2EN: TIM2 clock enable

This bit is set and cleared by software.

0: TIM2 clock disabled

1: TIM2 clock enabled

186/1749 RM0090 Rev 18

--- Page 187 ---

**RM0090**

**Reset and clock control for   STM32F42xxx and STM32F43xxx (RCC)**


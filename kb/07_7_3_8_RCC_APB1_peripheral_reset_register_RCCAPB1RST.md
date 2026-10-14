### 7.3.8 RCC APB1 peripheral reset register (RCC_APB1RSTR)

Address offset: 0x20
Reset value: 0x0000 0000
Access: no wait state, word, half-word and byte access.

| 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved | DACRST | PWR RST | Reserved | CAN2 RST | CAN1 RST | Reserved | I2C3 RST | I2C2 RST | I2C1 RST | UART5 RST | UART4 RST | UART3 RST | UART2 RST | Reserved |
| rw |  | rw |  | rw |  |  |  |  |  |  |  |  |  |  |
| 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| SPI3 RST | SPI2 RST | Reserved | WWDG RST | Reserved | TIM14 RST | TIM13 RST | TIM12 RST | TIM7 RST | TIM6 RST | TIM5 RST | TIM4 RST | TIM3 RST | TIM2 RST |  |
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bits 31:30 Reserved, must be kept at reset value.
Bit 29 **DACRST**: DAC reset
Set and cleared by software.
0: does not reset the DAC interface
1: resets the DAC interface
Bit 28 **PWRRST**: Power interface reset
Set and cleared by software.
0: does not reset the power interface
1: resets the power interface
Bit 27 Reserved, must be kept at reset value.

ST
RM0090 Rev 18
237/1749

--- Page 43 ---

**Reset and clock control for STM32F405xx/07xx and STM32F415xx/17xx(RCC)** RM0090

Bit 26 **CAN2RST**: CAN2 reset
Set and cleared by software.
0: does not reset CAN2
1: resets CAN2

Bit 25 **CAN1RST**: CAN1 reset
Set and cleared by software.
0: does not reset CAN1
1: resets CAN1

Bit 24 Reserved, must be kept at reset value.

Bit 23 **I2C3RST**: I2C3 reset
Set and cleared by software.
0: does not reset I2C3
1: resets I2C3

Bit 22 **I2C2RST**: I2C2 reset
Set and cleared by software.
0: does not reset I2C2
1: resets I2C2

Bit 21 **I2C1RST**: I2C1 reset
Set and cleared by software.
0: does not reset I2C1
1: resets I2C1

Bit 20 **UART5RST**: UART5 reset
Set and cleared by software.
0: does not reset UART5
1: resets UART5

Bit 19 **UART4RST**: USART4 reset
Set and cleared by software.
0: does not reset UART4
1: resets UART4

Bit 18 **USART3RST**: USART3 reset
Set and cleared by software.
0: does not reset USART3
1: resets USART3

Bit 17 **USART2RST**: USART2 reset
Set and cleared by software.
0: does not reset USART2
1: resets USART2

Bit 16 Reserved, must be kept at reset value.

Bit 15 **SPI3RST**: SPI3 reset
Set and cleared by software.
0: does not reset SPI3
1: resets SPI3

Bit 14 **SPI2RST**: SPI2 reset
Set and cleared by software.
0: does not reset SPI2
1: resets SPI2

238/1749 RM0090 Rev 18

--- Page 44 ---

RM0090 Reset and clock control for STM32F405xx/07xx and STM32F415xx/17xx(RCC)

Bits 13:12 Reserved, must be kept at reset value.

Bit 11 **WWDGRST**: Window watchdog reset
Set and cleared by software.
0: does not reset the window watchdog
1: resets the window watchdog

Bits 10:9 Reserved, must be kept at reset value.

Bit 8 **TIM14RST**: TIM14 reset
Set and cleared by software.
0: does not reset TIM14
1: resets TIM14

Bit 7 **TIM13RST**: TIM13 reset
Set and cleared by software.
0: does not reset TIM13
1: resets TIM13

Bit 6 **TIM12RST**: TIM12 reset
Set and cleared by software.
0: does not reset TIM12
1: resets TIM12

Bit 5 **TIM7RST**: TIM7 reset
Set and cleared by software.
0: does not reset TIM7
1: resets TIM7

Bit 4 **TIM6RST**: TIM6 reset
Set and cleared by software.
0: does not reset TIM6
1: resets TIM6

Bit 3 **TIM5RST**: TIM5 reset
Set and cleared by software.
0: does not reset TIM5
1: resets TIM5

Bit 2 **TIM4RST**: TIM4 reset
Set and cleared by software.
0: does not reset TIM4
1: resets TIM4

Bit 1 **TIM3RST**: TIM3 reset
Set and cleared by software.
0: does not reset TIM3
1: resets TIM3

Bit 0 **TIM2RST**: TIM2 reset
Set and cleared by software.
0: does not reset TIM2
1: resets TIM2

ST
RM0090 Rev 18
239/1749

--- Page 45 ---

Reset and clock control for STM32F405xx/07xx and STM32F415xx/17xx(RCC) RM0090

### 7.3.9 RCC APB2 peripheral reset register (RCC_APB2RSTR)

Address offset: 0x24
Reset value: 0x0000 0000
Access: no wait state, word, half-word and byte access.

| 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved | TIM11 RSTrw | TIM10 RSTrw | TIM9 RSTrw |  |  |  |  |  |  |  |  |  |
| 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0 |  |  |  |  |  |  |  |  |  |  |  |  |
| Reserved | SYSCFG RSTrw | Reserved | SPI1 RSTrw | SDIO RSTrw | Reserved | ADC RSTrw | Reserved | USART6 RSTrw | USART1 RSTrw | Reserved | TIM8 RSTrw | TIM1 RSTrw |


Bits 31:19 Reserved, must be kept at reset value.

**Bit 18 TIM11RST: TIM11 reset**
Set and cleared by software.
0: does not reset TIM11
1: resets TIM14

**Bit 17 TIM10RST: TIM10 reset**
Set and cleared by software.
0: does not reset TIM10
1: resets TIM10

**Bit 16 TIM9RST: TIM9 reset**
Set and cleared by software.
0: does not reset TIM9
1: resets TIM9

Bit 15 Reserved, must be kept at reset value.

**Bit 14 SYSCFGRST: System configuration controller reset**
Set and cleared by software.
0: does not reset the System configuration controller
1: resets the System configuration controller

Bit 13 Reserved, must be kept at reset value.

**Bit 12 SPI1RST: SPI1 reset**
Set and cleared by software.
0: does not reset SPI1
1: resets SPI1

**Bit 11 SDIORST: SDIO reset**
Set and cleared by software.
0: does not reset the SDIO module
1: resets the SDIO module

Bits 10:9 Reserved, must be kept at reset value.

240/1749 RM0090 Rev 18

--- Page 46 ---

RM0090 Reset and clock control for STM32F405xx/07xx and STM32F415xx/17xx(RCC)

Bit 8 **ADCRST**: ADC interface reset (common to all ADCs)
Set and cleared by software.
0: does not reset the ADC interface
1: resets the ADC interface

Bits 7:6 Reserved, must be kept at reset value.

Bit 5 **USART6RST**: USART6 reset
Set and cleared by software.
0: does not reset USART6
1: resets USART6

Bit 4 **USART1RST**: USART1 reset
Set and cleared by software.
0: does not reset USART1
1: resets USART1

Bits 3:2 Reserved, must be kept at reset value.

Bit 1 **TIM8RST**: TIM8 reset
Set and cleared by software.
0: does not reset TIM8
1: resets TIM8

Bit 0 **TIM1RST**: TIM1 reset
Set and cleared by software.
0: does not reset TIM1
1: resets TIM1

ST
RM0090 Rev 18
241/1749

--- Page 47 ---

Reset and clock control for STM32F405xx/07xx and STM32F415xx/17xx(RCC) RM0090


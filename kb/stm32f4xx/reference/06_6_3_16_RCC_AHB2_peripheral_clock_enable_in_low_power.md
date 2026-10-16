**6.3.16 RCC AHB2 peripheral clock enable in low power mode register (RCC_AHB2LPENR)**

Address offset: 0x54

Reset value: 0x0000 00F1

Access: no wait state, word, half-word and byte access.

31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16

Reserved

15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0

| Reserved | OTGFS LPEN | RNG LPEN | HASH LPEN | CRYP LPEN | Reserved | DCMI LPEN |
|---|---|---|---|---|---|---|
| rw |  |  |  |  |  |  |


Bits 31:8 Reserved, must be kept at reset value.

Bit 7 OTGFSLPEN: USB OTG FS clock enable during Sleep mode

This bit is set and cleared by software.

0: USB OTG FS clock disabled during Sleep mode

1: USB OTG FS clock enabled during Sleep mode

Bit 6 RNGLPEN: Random number generator clock enable during Sleep mode This bit is set and cleared by software.

0: Random number generator clock disabled during Sleep mode

1: Random number generator clock enabled during Sleep mode

Bit 5 HASHLPEN: Hash modules clock enable during Sleep mode

This bit is set and cleared by software.

0: Hash modules clock disabled during Sleep mode

1: Hash modules clock enabled during Sleep mode

Bit 4 CRYPLPEN: Cryptography modules clock enable during Sleep mode

This bit is set and cleared by software.

0: cryptography modules clock disabled during Sleep mode

1: cryptography modules clock enabled during Sleep mode

Bits 3:1 Reserved, must be kept at reset value.

Bit 0 DCMILPEN: Camera interface enable during Sleep mode

This bit is set and cleared by software.

0: Camera interface clock disabled during Sleep mode

1: Camera interface clock enabled during Sleep mode



--- Page 193 ---

**RM0090**

**Reset and clock control for   STM32F42xxx and STM32F43xxx (RCC)**

**6.3.17**

**RCC AHB3 peripheral clock enable in low power mode register**

**(RCC_AHB3LPENR)**

Address offset: 0x58

Reset value: 0x0000 0001

Access: no wait state, word, half-word and byte access.

31 30 29 28 27 26 25 24

23 22 21 20 19 18 17 16 Reserved

15 14 13 12 11 10 9 8

7 6 5 4 3 2 1 0

| Reserved | FMC LPEN |
|---|---|
| rw |  |


Bits 31:1Reserved, must be kept at reset value.

**FMCLPEN:**

Flexible memory controller module clock enable during Sleep mode

Bit 0 This bit is set and cleared by software.

0: FMC module clock disabled during Sleep mode 1: FMC module clock enabled during Sleep mode

**6.3.18**

**RCC APB1 peripheral clock enable in low power mode register**

**(RCC_APB1LPENR)**

Address offset: 0x60

Reset value: 0xF6FE C9FF

Access: no wait state, word, half-word and byte access.

31 30 29 28 27 26 25 24

23 22 21 20 19 18 17 16

| UART8 LPEN | UART7 LPEN | DAC LPEN | PWR LPEN | RESER VED | CAN2 LPEN | CAN1 LPEN | Reser- ved | I2C3 LPEN | I2C2 LPEN | I2C1 LPEN | UART5 LPEN | UART4 LPEN | USART 3 LPEN | USART 2 LPEN | Reser- ved |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| rw |  | rw |  |  |  |  |  |  |  |  |  |  |  |  |  |


15 14 13 12 11 10 9 8

7 6 5 4 3 2 1 0

| SPI3 LPEN | SPI2 LPEN | Reserved | WWDG LPEN | Reserved | TIM14 LPEN | TIM13 LPEN | TIM12 LPEN | TIM7 LPEN | TIM6 LPEN | TIM5 LPEN | TIM4 LPEN | TIM3 LPEN | TIM2 LPEN |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |




212

--- Page 194 ---

**Reset and clock control for   STM32F42xxx and STM32F43xxx (RCC) RM0090**

Bit 31 UART8LPEN: UART8 clock enable during Sleep mode

This bit is set and cleared by software.

0: UART8 clock disabled during Sleep mode

1: UART8 clock enabled during Sleep mode

Bit 30 UART7LPEN: UART7 clock enable during Sleep mode

This bit is set and cleared by software.

0: UART7 clock disabled during Sleep mode

1: UART7 clock enabled during Sleep mode

Bit 29 DACLPEN: DAC interface clock enable during Sleep mode

This bit is set and cleared by software.

0: DAC interface clock disabled during Sleep mode

1: DAC interface clock enabled during Sleep mode

Bit 28 PWRLPEN: Power interface clock enable during Sleep mode

This bit is set and cleared by software.

0: Power interface clock disabled during Sleep mode

1: Power interface clock enabled during Sleep mode

Bit 27 Reserved, must be kept at reset value.

Bit 26 CAN2LPEN: CAN 2 clock enable during Sleep mode

This bit is set and cleared by software.

0: CAN 2 clock disabled during sleep mode

1: CAN 2 clock enabled during sleep mode

Bit 25 CAN1LPEN: CAN 1 clock enable during Sleep mode

This bit is set and cleared by software.

0: CAN 1 clock disabled during Sleep mode

1: CAN 1 clock enabled during Sleep mode

Bit 24 Reserved, must be kept at reset value.

Bit 23 I2C3LPEN: I2C3 clock enable during Sleep mode

This bit is set and cleared by software.

0: I2C3 clock disabled during Sleep mode

1: I2C3 clock enabled during Sleep mode

Bit 22 I2C2LPEN: I2C2 clock enable during Sleep mode

This bit is set and cleared by software.

0: I2C2 clock disabled during Sleep mode

1: I2C2 clock enabled during Sleep mode

Bit 21 I2C1LPEN: I2C1 clock enable during Sleep mode

This bit is set and cleared by software.

0: I2C1 clock disabled during Sleep mode

1: I2C1 clock enabled during Sleep mode

Bit 20 UART5LPEN: UART5 clock enable during Sleep mode

This bit is set and cleared by software.

0: UART5 clock disabled during Sleep mode

1: UART5 clock enabled during Sleep mode

Bit 19 UART4LPEN: UART4 clock enable during Sleep mode

This bit is set and cleared by software.

0: UART4 clock disabled during Sleep mode

1: UART4 clock enabled during Sleep mode

194/1749 RM0090 Rev 18

--- Page 195 ---

**RM0090**

**Reset and clock control for   STM32F42xxx and STM32F43xxx (RCC)**

Bit 18 USART3LPEN:

USART3 clock enable during Sleep mode

This bit is set and cleared by software.

0: USART3 clock disabled during Sleep mode 1: USART3 clock enabled during Sleep mode Bit 17 USART2LPEN:

USART2 clock enable during Sleep mode

This bit is set and cleared by software.

0: USART2 clock disabled during Sleep mode 1: USART2 clock enabled during Sleep mode Bit 16 Reserved, must be kept at reset value.

Bit 15 SPI3LPEN: SPI3 clock enable during Sleep mode This bit is set and cleared by software.

0: SPI3 clock disabled during Sleep mode 1: SPI3 clock enabled during Sleep mode Bit 14 SPI2LPEN: SPI2 clock enable during Sleep mode This bit is set and cleared by software.

0: SPI2 clock disabled during Sleep mode 1: SPI2 clock enabled during Sleep mode Bits 13:12 Reserved, must be kept at reset value.

Bit 11 WWDGLPEN:

Window watchdog clock enable during Sleep mode

This bit is set and cleared by software.

0: Window watchdog clock disabled during sleep mode 1: Window watchdog clock enabled during sleep mode Bits 10:9 Reserved, must be kept at reset value.

Bit 8 TIM14LPEN: TIM14 clock enable during Sleep mode This bit is set and cleared by software.

0: TIM14 clock disabled during Sleep mode 1: TIM14 clock enabled during Sleep mode Bit 7 TIM13LPEN: TIM13 clock enable during Sleep mode This bit is set and cleared by software.

0: TIM13 clock disabled during Sleep mode 1: TIM13 clock enabled during Sleep mode Bit 6 TIM12LPEN: TIM12 clock enable during Sleep mode This bit is set and cleared by software.

0: TIM12 clock disabled during Sleep mode 1: TIM12 clock enabled during Sleep mode Bit 5 TIM7LPEN: TIM7 clock enable during Sleep mode This bit is set and cleared by software.

0: TIM7 clock disabled during Sleep mode 1: TIM7 clock enabled during Sleep mode Bit 4 TIM6LPEN: TIM6 clock enable during Sleep mode This bit is set and cleared by software.

0: TIM6 clock disabled during Sleep mode 1: TIM6 clock enabled during Sleep mode

RM0090 Rev 18 195/1749

212
--- Page 1 ---

**Reset and clock control for STM32F42xxx and STM32F43xxx (RCC)** RM0090

Bit 3 **TIM5LPEN**: TIM5 clock enable during Sleep mode
This bit is set and cleared by software.
0: TIM5 clock disabled during Sleep mode
1: TIM5 clock enabled during Sleep mode

Bit 2 **TIM4LPEN**: TIM4 clock enable during Sleep mode
This bit is set and cleared by software.
0: TIM4 clock disabled during Sleep mode
1: TIM4 clock enabled during Sleep mode

Bit 1 **TIM3LPEN**: TIM3 clock enable during Sleep mode
This bit is set and cleared by software.
0: TIM3 clock disabled during Sleep mode
1: TIM3 clock enabled during Sleep mode

Bit 0 **TIM2LPEN**: TIM2 clock enable during Sleep mode
This bit is set and cleared by software.
0: TIM2 clock disabled during Sleep mode
1: TIM2 clock enabled during Sleep mode

196/1749 RM0090 Rev 18 ST

--- Page 2 ---

RM0090
Reset and clock control for STM32F42xxx and STM32F43xxx (RCC)


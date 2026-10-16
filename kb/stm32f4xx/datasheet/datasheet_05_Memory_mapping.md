# 5 Memory mapping

The memory map is shown in Figure 18.

Figure 18. STM32F40xxx memory map

| Address Range | Description | Address Range | Description |
|---|---|---|---|
| 0xE000 0000 - 0xE00F FFFF | CORTEX-M4 internal peripherals | 0xE010 0000 - 0xFFFF FFFF | Reserved |
| 0xA000 0000 - 0xDFFF FFFF | Reserved | 0xA000 0000 - 0xA000 FFFF | AHB3 |
| 0x6000 0000 - 0x5FFF FFFF | Reserved | 0x5006 0C00 - 0x5FFF FFFF | Reserved |
| 0x5000 0000 - 0x4FFF FFFF | Reserved | 0x5006 0000 - 0x5006 0BFF | AHB2 |
| 0x4001 5800 - 0x4001 FFFF | Reserved | 0x4007 0000 - 0x4007 FFFF | Reserved |
| 0x4002 0000 - 0x4001 57FF | Reserved | 0x4001 0000 - 0x4001 7FFF | AHB1 |
| 0x4000 7800 - 0x4000 FFFF | Reserved | 0x4000 0000 - 0x4000 77FF | Reserved |
| 0x1FFF C008 - 0x1FFF FFFF | Reserved | 0x1FFF C000 - 0x1FFF C007 | Option Bytes |
| 0x1FFF 7A10 - 0x1FFF 7FFF | Reserved | 0x1FFF 0000 - 0x1FFF 7A0F | System memory + OTP |
| 0x1000 0000 - 0x1000 FFFF | CCM data RAM (64 KB data SRAM) | 0x1001 0000 - 0x1FFF FFFF | Reserved |
| 0x0810 0000 - 0x080F FFFF | Reserved | 0x0800 0000 - 0x080F FFFF | Flash |
| 0x0010 0000 - 0x000F FFFF | Reserved | 0x0000 0000 - 0x000F FFFF | Aliased to Flash, system memory or SRAM depending on the BOOT pins |
| 0x2002 0000 - 0x2001 BFFF | SRAM (112 KB aliased by bit-banding) | 0x2002 0000 - 0x2001 FFFF | SRAM (16 KB aliased by bit-banding) |
| 0x2000 0000 - 0x1FFF FFFF | 512-Mbyte block 0 Code | 0x2000 0000 - 0x2001 FFFF | 512-Mbyte block 1 SRAM |
| 0x4000 0000 - 0x3FFF FFFF | 512-Mbyte block 2 Peripherals | 0x4000 0000 - 0x3FFF FFFF | APB2 |
| 0x6000 0000 - 0x5FFF FFFF | 512-Mbyte block 3 FSMC bank1 & bank2 | 0x6000 0000 - 0x5FFF FFFF | APB1 |
| 0x8000 0000 - 0x7FFF FFFF | 512-Mbyte block 4 FSMC bank 3 & bank4 | 0x8000 0000 - 0x7FFF FFFF | Reserved |
| 0xA000 0000 - 0x9FFF FFFF | 512-Mbyte block 5 FSMC registers | 0xA000 0000 - 0x9FFF FFFF | Reserved |
| 0xC000 0000 - 0xBFFF FFFF | 512-Mbyte block 6 Not used | 0xC000 0000 - 0xBFFF FFFF | Reserved |
| 0xE000 0000 - 0xDFFF FFFF | 512-Mbyte block 7 Cortex-M4's internal peripherals | 0xE000 0000 - 0xDFFF FFFF | Reserved |
| 0xFFFF FFFF - 0xFFFF FFFF | Reserved | 0xFFFF FFFF - 0xFFFF FFFF | Reserved |


DS8626 Rev 12
73/206

--- Page 74 ---

Memory mapping
STM32F405xx, STM32F407xx

Table 10. Register boundary addresses

| Bus | Boundary address | Peripheral |
|---|---|---|
|  | 0xE00F FFFF - 0xFFFF FFFF | Reserved |
| Cortex-M4 | 0xE000 0000 - 0xE00F FFFF | Cortex-M4 internal peripherals |
|  | 0xA000 1000 - 0xDFFF FFFF | Reserved |
| AHB3 | 0xA000 0000 - 0xA000 0FFF | FSMC control register |
| 0x9000 0000 - 0x9FFF FFFF | FSMC bank 4 |  |
| 0x8000 0000 - 0x8FFF FFFF | FSMC bank 3 |  |
| 0x7000 0000 - 0x7FFF FFFF | FSMC bank 2 |  |
| 0x6000 0000 - 0x6FFF FFFF | FSMC bank 1 |  |
|  | 0x5006 0C00- 0x5FFF FFFF | Reserved |
| AHB2 | 0x5006 0800 - 0x5006 0BFF | RNG |
| 0x5005 0400 - 0x5006 07FF | Reserved |  |
| 0x5005 0000 - 0x5005 03FF | DCMI |  |
|  | 0x5004 0000- 0x5004 FFFF | Reserved |
|  | 0x5000 0000 - 0x5003 FFFF | USB OTG FS |
|  | 0x4008 0000- 0x4FFF FFFF | Reserved |


74/206
DS8626 Rev 12
ST

--- Page 75 ---

**STM32F405xx, STM32F407xx**

**Memory mapping**

Table 10. Register boundary addresses (continued)

| Bus | Boundary address | Peripheral |
|---|---|---|
| AHB1 | 0x4004 0000 - 0x4007 FFFF | USB OTG HS |
| 0x4002 9400 - 0x4003 FFFF | Reserved |  |
| 0x4002 9000 - 0x4002 93FF | ETHERNET MAC |  |
| 0x4002 8C00 - 0x4002 8BFF |  |  |
| 0x4002 8800 - 0x4002 8BFF |  |  |
| 0x4002 8400 - 0x4002 87FF |  |  |
| 0x4002 8000 - 0x4002 83FF |  |  |
| 0x4002 6800 - 0x4002 7FFF | Reserved |  |
| 0x4002 6400 - 0x4002 67FF | DMA2 |  |
| 0x4002 6000 - 0x4002 63FF | DMA1 |  |
| 0x4002 5000 - 0x4002 5FFF | Reserved |  |
| 0x4002 4000 - 0x4002 4FFF | BKPSRAM |  |
| 0x4002 3C00 - 0x4002 3FFF | Flash interface register |  |
| 0x4002 3800 - 0x4002 3BFF | RCC |  |
| 0x4002 3400 - 0x4002 37FF | Reserved |  |
| 0x4002 3000 - 0x4002 33FF | CRC |  |
| 0x4002 2400 - 0x4002 2FFF | Reserved |  |
| 0x4002 2000 - 0x4002 23FF | GPIOD |  |
| 0x4002 1C00 - 0x4002 1FFF | GPIOD |  |
| 0x4002 1800 - 0x4002 1BFF | GPIOD |  |
| 0x4002 1400 - 0x4002 17FF | GPIOD |  |
| 0x4002 1000 - 0x4002 13FF | GPIOD |  |
| 0x4002 0C00 - 0x4002 0FFF | GPIOD |  |
| 0x4002 0800 - 0x4002 0BFF | GPIOD |  |
| 0x4002 0400 - 0x4002 07FF | GPIOD |  |
| 0x4002 0000 - 0x4002 03FF | GPIOD |  |
| 0x4001 5800- 0x4001 FFFF | Reserved |  |


ST
DS8626 Rev 12
75/206

--- Page 76 ---

**Memory mapping**
**STM32F405xx, STM32F407xx**

Table 10. Register boundary addresses (continued)

| Bus | Boundary address | Peripheral |
|---|---|---|
| APB2 | 0x4001 4C00 - 0x4001 57FF | Reserved |
| 0x4001 4800 - 0x4001 4BFF | TIM11 |  |
| 0x4001 4400 - 0x4001 47FF | TIM10 |  |
| 0x4001 4000 - 0x4001 43FF | TIM9 |  |
| 0x4001 3C00 - 0x4001 3FFF | EXTI |  |
| 0x4001 3800 - 0x4001 3BFF | SYSCFG |  |
| 0x4001 3400 - 0x4001 37FF | Reserved |  |
| 0x4001 3000 - 0x4001 33FF | SPI1 |  |
| 0x4001 2C00 - 0x4001 2FFF | SDIO |  |
| 0x4001 2400 - 0x4001 2BFF | Reserved |  |
| 0x4001 2000 - 0x4001 23FF | ADC1 - ADC2 - ADC3 |  |
| 0x4001 1800 - 0x4001 1FFF | Reserved |  |
| 0x4001 1400 - 0x4001 17FF | USART6 |  |
| 0x4001 1000 - 0x4001 13FF | USART1 |  |
| 0x4001 0800 - 0x4001 0FFF | Reserved |  |
| 0x4001 0400 - 0x4001 07FF | TIM8 |  |
| 0x4001 0000 - 0x4001 03FF | TIM1 |  |
|  | 0x4000 7800- 0x4000 FFFF | Reserved |


76/206
DS8626 Rev 12

--- Page 77 ---

STM32F405xx, STM32F407xx
Memory mapping

Table 10. Register boundary addresses (continued)

| Bus | Boundary address | Peripheral |
|---|---|---|
| APB1 | 0x4000 7800 - 0x4000 7FFF | Reserved |
| 0x4000 7400 - 0x4000 77FF | DAC |  |
| 0x4000 7000 - 0x4000 73FF | PWR |  |
| 0x4000 6C00 - 0x4000 6FFF | Reserved |  |
| 0x4000 6800 - 0x4000 6BFF | CAN2 |  |
| 0x4000 6400 - 0x4000 67FF | CAN1 |  |
| 0x4000 6000 - 0x4000 63FF | Reserved |  |
| 0x4000 5C00 - 0x4000 5FFF | I2C3 |  |
| 0x4000 5800 - 0x4000 5BFF | I2C2 |  |
| 0x4000 5400 - 0x4000 57FF | I2C1 |  |
| 0x4000 5000 - 0x4000 53FF | UART5 |  |
| 0x4000 4C00 - 0x4000 4FFF | UART4 |  |
| 0x4000 4800 - 0x4000 4BFF | USART3 |  |
| 0x4000 4400 - 0x4000 47FF | USART2 |  |
| 0x4000 4000 - 0x4000 43FF | I2S3ext |  |
| 0x4000 3C00 - 0x4000 3FFF | SPI3 / I2S3 |  |
| 0x4000 3800 - 0x4000 3BFF | SPI2 / I2S2 |  |
| 0x4000 3400 - 0x4000 37FF | I2S2ext |  |
| 0x4000 3000 - 0x4000 33FF | IWDG |  |
| 0x4000 2C00 - 0x4000 2FFF | WWDG |  |
| 0x4000 2800 - 0x4000 2BFF | RTC & BKP Registers |  |
| 0x4000 2400 - 0x4000 27FF | Reserved |  |
| 0x4000 2000 - 0x4000 23FF | TIM14 |  |
| 0x4000 1C00 - 0x4000 1FFF | TIM13 |  |
| 0x4000 1800 - 0x4000 1BFF | TIM12 |  |
| 0x4000 1400 - 0x4000 17FF | TIM7 |  |
| 0x4000 1000 - 0x4000 13FF | TIM6 |  |
| 0x4000 0C00 - 0x4000 0FFF | TIM5 |  |
| 0x4000 0800 - 0x4000 0BFF | TIM4 |  |
| 0x4000 0400 - 0x4000 07FF | TIM3 |  |
| 0x4000 0000 - 0x4000 03FF | TIM2 |  |


ST
DS8626 Rev 12
77/206

--- Page 78 ---

Electrical characteristics
STM32F405xx, STM32F407xx


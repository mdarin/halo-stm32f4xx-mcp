# 12 Interrupts and events

This Section applies to the whole STM32F4xx family, unless otherwise specified.

## 12.1 Nested vectored interrupt controller (NVIC)

### 12.1.1 NVIC features

The nested vector interrupt controller NVIC includes the following features:

*   82 maskable interrupt channels for STM32F405xx/07xx and STM32F415xx/17xx, and up to 91 maskable interrupt channels for STM32F42xxx and STM32F43xxx (not including the 16 interrupt lines of Cortex®-M4 with FPU)
*   16 programmable priority levels (4 bits of interrupt priority are used)
*   low-latency exception and interrupt handling
*   power management control
*   implementation of system control registers

The NVIC and the processor core interface are closely coupled, which enables low latency interrupt processing and efficient processing of late arriving interrupts.

All interrupts including the core exceptions are managed by the NVIC. For more information on exceptions and NVIC programming, refer to programming manual PM0214.

### 12.1.2 SysTick calibration value register

The SysTick calibration value is fixed to 18750, which gives a reference time base of 1 ms with the SysTick clock set to 18.75 MHz (HCLK/8, with HCLK set to 150 MHz).

### 12.1.3 Interrupt and exception vectors

See Table 62 and Table 62, for the vector table for the STM32F405xx/07xx and STM32F415xx/17xx and STM32F42xxx and STM32F43xxx devices.

## 12.2 External interrupt/event controller (EXTI)

The external interrupt/event controller consists of up to 23 edge detectors for generating event/interrupt requests. Each input line can be independently configured to select the type (interrupt or event) and the corresponding trigger event (rising or falling or both). Each line can also masked independently. A pending register maintains the status line of the interrupt requests.

ST
RM0090 Rev 18
371/1749

--- Page 177 ---

Interrupts and events
RM0090

Table 61. Vector table for STM32F405xx/07xx and STM32F415xx/17xx

| Position | Priority | Type of priority | Acronym | Description | Address |
|---|---|---|---|---|---|
|  | - | Reserved | 0x0000 0000 |  |  |
|  | -3 | fixed | Reset | 0x0000 0004 |  |
|  | -2 | fixed | NMI | Non maskable interrupt. The RCC Clock Security System (CSS) is linked to the NMI vector. | 0x0000 0008 |
|  | -1 | fixed | HardFault | All class of fault | 0x0000 000C |
| 0 |  | settable | MemManage | Memory management | 0x0000 0010 |
| 1 |  | settable | BusFault | Pre-fetch fault, memory access fault | 0x0000 0014 |
| 2 |  | settable | UsageFault | Undefined instruction or illegal state | 0x0000 0018 |
|  | - | Reserved | 0x0000 001C - 0x0000 002B |  |  |
|  | 3 | settable | SVCall | System service call via SWI instruction | 0x0000 002C |
| 4 |  | settable | Debug Monitor | 0x0000 0030 |  |
|  | - | Reserved | 0x0000 0034 |  |  |
| 5 |  | settable | PendSV | Pendable request for system service | 0x0000 0038 |
| 6 |  | settable | SysTick | System tick timer | 0x0000 003C |
| 0 | 7 | settable | WWDG | Window Watchdog interrupt | 0x0000 0040 |
| 1 | 8 | settable | PVD | PVD through EXTI line detection interrupt | 0x0000 0044 |
| 2 | 9 | settable | TAMP_STAMP | Tamper and TimeStamp interrupts through the EXTI line | 0x0000 0048 |
| 3 | 10 | settable | RTC_WKUP | RTC Wakeup interrupt through the EXTI line | 0x0000 004C |
| 4 | 11 | settable | FLASH | Flash global interrupt | 0x0000 0050 |
| 5 | 12 | settable | RCC | RCC global interrupt | 0x0000 0054 |
| 6 | 13 | settable | EXTI0 | EXTI Line0 interrupt | 0x0000 0058 |
| 7 | 14 | settable | EXTI1 | EXTI Line1 interrupt | 0x0000 005C |
| 8 | 15 | settable | EXTI2 | EXTI Line2 interrupt | 0x0000 0060 |
| 9 | 16 | settable | EXTI3 | EXTI Line3 interrupt | 0x0000 0064 |
| 10 | 17 | settable | EXTI4 | EXTI Line4 interrupt | 0x0000 0068 |
| 11 | 18 | settable | DMA1_Stream0 | DMA1 Stream0 global interrupt | 0x0000 006C |


372/1749
RM0090 Rev 18

--- Page 178 ---

RM0090
Interrupts and events

Table 61. Vector table for STM32F405xx/07xx and STM32F415xx/17xx (continued)

| Position | Priority | Type of priority | Acronym | Description | Address |
|---|---|---|---|---|---|
| 12 | 19 | settable | DMA1_Stream1 | DMA1 Stream1 global interrupt | 0x0000 0070 |
| 13 | 20 | settable | DMA1_Stream2 | DMA1 Stream2 global interrupt | 0x0000 0074 |
| 14 | 21 | settable | DMA1_Stream3 | DMA1 Stream3 global interrupt | 0x0000 0078 |
| 15 | 22 | settable | DMA1_Stream4 | DMA1 Stream4 global interrupt | 0x0000 007C |
| 16 | 23 | settable | DMA1_Stream5 | DMA1 Stream5 global interrupt | 0x0000 0080 |
| 17 | 24 | settable | DMA1_Stream6 | DMA1 Stream6 global interrupt | 0x0000 0084 |
| 18 | 25 | settable | ADC | ADC1, ADC2 and ADC3 global interrupts | 0x0000 0088 |
| 19 | 26 | settable | CAN1_TX | CAN1 TX interrupts | 0x0000 008C |
| 20 | 27 | settable | CAN1_RX0 | CAN1 RX0 interrupts | 0x0000 0090 |
| 21 | 28 | settable | CAN1_RX1 | CAN1 RX1 interrupt | 0x0000 0094 |
| 22 | 29 | settable | CAN1_SCE | CAN1 SCE interrupt | 0x0000 0098 |
| 23 | 30 | settable | EXTI9_5 | EXTI Line[9:5] interrupts | 0x0000 009C |
| 24 | 31 | settable | TIM1_BRK_TIM9 | TIM1 Break interrupt and TIM9 global interrupt | 0x0000 00A0 |
| 25 | 32 | settable | TIM1_UP_TIM10 | TIM1 Update interrupt and TIM10 global interrupt | 0x0000 00A4 |
| 26 | 33 | settable | TIM1_TRG_COM_TIM11 | TIM1 Trigger and Commutation interrupts and TIM11 global interrupt | 0x0000 00A8 |
| 27 | 34 | settable | TIM1_CC | TIM1 Capture Compare interrupt | 0x0000 00AC |
| 28 | 35 | settable | TIM2 | TIM2 global interrupt | 0x0000 00B0 |
| 29 | 36 | settable | TIM3 | TIM3 global interrupt | 0x0000 00B4 |
| 30 | 37 | settable | TIM4 | TIM4 global interrupt | 0x0000 00B8 |
| 31 | 38 | settable | I2C1_EV | I²C1 event interrupt | 0x0000 00BC |
| 32 | 39 | settable | I2C1_ER | I²C1 error interrupt | 0x0000 00C0 |
| 33 | 40 | settable | I2C2_EV | I²C2 event interrupt | 0x0000 00C4 |
| 34 | 41 | settable | I2C2_ER | I²C2 error interrupt | 0x0000 00C8 |
| 35 | 42 | settable | SPI1 | SPI1 global interrupt | 0x0000 00CC |
| 36 | 43 | settable | SPI2 | SPI2 global interrupt | 0x0000 00D0 |
| 37 | 44 | settable | USART1 | USART1 global interrupt | 0x0000 00D4 |
| 38 | 45 | settable | USART2 | USART2 global interrupt | 0x0000 00D8 |
| 39 | 46 | settable | USART3 | USART3 global interrupt | 0x0000 00DC |


RM0090 Rev 18
373/1749

--- Page 179 ---

Interrupts and events
RM0090

Table 61. Vector table for STM32F405xx/07xx and STM32F415xx/17xx (continued)

| Position | Priority | Type of priority | Acronym | Description | Address |
|---|---|---|---|---|---|
| 40 | 47 | settable | EXTI15_10 | EXTI Line[15:10] interrupts | 0x0000 00E0 |
| 41 | 48 | settable | RTC_Alarm | RTC Alarms (A and B) through EXTI line interrupt | 0x0000 00E4 |
| 42 | 49 | settable | OTG_FS_WKUP | USB On-The-Go FS Wakeup through EXTI line interrupt | 0x0000 00E8 |
| 43 | 50 | settable | TIM8_BRK_TIM12 | TIM8 Break interrupt and TIM12 global interrupt | 0x0000 00EC |
| 44 | 51 | settable | TIM8_UP_TIM13 | TIM8 Update interrupt and TIM13 global interrupt | 0x0000 00F0 |
| 45 | 52 | settable | TIM8_TRG_COM_TIM14 | TIM8 Trigger and Commutation interrupts and TIM14 global interrupt | 0x0000 00F4 |
| 46 | 53 | settable | TIM8_CC | TIM8 Capture Compare interrupt | 0x0000 00F8 |
| 47 | 54 | settable | DMA1_Stream7 | DMA1 Stream7 global interrupt | 0x0000 00FC |
| 48 | 55 | settable | FSMC | FSMC global interrupt | 0x0000 0100 |
| 49 | 56 | settable | SDIO | SDIO global interrupt | 0x0000 0104 |
| 50 | 57 | settable | TIM5 | TIM5 global interrupt | 0x0000 0108 |
| 51 | 58 | settable | SPI3 | SPI3 global interrupt | 0x0000 010C |
| 52 | 59 | settable | UART4 | UART4 global interrupt | 0x0000 0110 |
| 53 | 60 | settable | UART5 | UART5 global interrupt | 0x0000 0114 |
| 54 | 61 | settable | TIM6_DAC | TIM6 global interrupt, DAC1 and DAC2 underrun error interrupts | 0x0000 0118 |
| 55 | 62 | settable | TIM7 | TIM7 global interrupt | 0x0000 011C |
| 56 | 63 | settable | DMA2_Stream0 | DMA2 Stream0 global interrupt | 0x0000 0120 |
| 57 | 64 | settable | DMA2_Stream1 | DMA2 Stream1 global interrupt | 0x0000 0124 |
| 58 | 65 | settable | DMA2_Stream2 | DMA2 Stream2 global interrupt | 0x0000 0128 |
| 59 | 66 | settable | DMA2_Stream3 | DMA2 Stream3 global interrupt | 0x0000 012C |
| 60 | 67 | settable | DMA2_Stream4 | DMA2 Stream4 global interrupt | 0x0000 0130 |
| 61 | 68 | settable | ETH | Ethernet global interrupt | 0x0000 0134 |
| 62 | 69 | settable | ETH_WKUP | Ethernet Wakeup through EXTI line interrupt | 0x0000 0138 |
| 63 | 70 | settable | CAN2_TX | CAN2 TX interrupts | 0x0000 013C |
| 64 | 71 | settable | CAN2_RX0 | CAN2 RX0 interrupts | 0x0000 0140 |


374/1749
RM0090 Rev 18

--- Page 180 ---

RM0090
Interrupts and events

Table 61. Vector table for STM32F405xx/07xx and STM32F415xx/17xx (continued)

| Position | Priority | Type of priority | Acronym | Description | Address |
|---|---|---|---|---|---|
| 65 | 72 | settable | CAN2_RX1 | CAN2 RX1 interrupt | 0x0000 0144 |
| 66 | 73 | settable | CAN2_SCE | CAN2 SCE interrupt | 0x0000 0148 |
| 67 | 74 | settable | OTG_FS | USB On The Go FS global interrupt | 0x0000 014C |
| 68 | 75 | settable | DMA2_Stream5 | DMA2 Stream5 global interrupt | 0x0000 0150 |
| 69 | 76 | settable | DMA2_Stream6 | DMA2 Stream6 global interrupt | 0x0000 0154 |
| 70 | 77 | settable | DMA2_Stream7 | DMA2 Stream7 global interrupt | 0x0000 0158 |
| 71 | 78 | settable | USART6 | USART6 global interrupt | 0x0000 015C |
| 72 | 79 | settable | I2C3_EV | I²C3 event interrupt | 0x0000 0160 |
| 73 | 80 | settable | I2C3_ER | I²C3 error interrupt | 0x0000 0164 |
| 74 | 81 | settable | OTG_HS_EP1_OUT | USB On The Go HS End Point 1 Out global interrupt | 0x0000 0168 |
| 75 | 82 | settable | OTG_HS_EP1_IN | USB On The Go HS End Point 1 In global interrupt | 0x0000 016C |
| 76 | 83 | settable | OTG_HS_WKUP | USB On The Go HS Wakeup through EXTI interrupt | 0x0000 0170 |
| 77 | 84 | settable | OTG_HS | USB On The Go HS global interrupt | 0x0000 0174 |
| 78 | 85 | settable | DCMI | DCMI global interrupt | 0x0000 0178 |
| 79 | 86 | settable | CRYP | CRYP crypto global interrupt | 0x0000 017C |
| 80 | 87 | settable | HASH_RNG | Hash and Rng global interrupt | 0x0000 0180 |
| 81 | 88 | settable | FPU | FPU global interrupt | 0x0000 0184 |


Table 62. Vector table for STM32F42xxx and STM32F43xxx

| Position | Priority | Type of priority | Acronym | Description | Address |
|---|---|---|---|---|---|
| - | Reserved | 0x0000 0000 |  |  |  |
| -3 | fixed | Reset | 0x0000 0004 |  |  |
| -2 | fixed | NMI | Non maskable interrupt, Clock Security System | 0x0000 0008 |  |


ST
RM0090 Rev 18
375/1749

--- Page 181 ---

**Interrupts and events**

RM0090

376/1749
RM0090 Rev 18

Table 62. Vector table for STM32F42xxx and STM32F43xxx (continued)

| Position | Priority | Type of priority | Acronym | Description | Address |
|---|---|---|---|---|---|
| -1 | fixed | HardFault | All class of fault | 0x0000 000C |  |
| 0 | settable | MemManage | Memory management | 0x0000 0010 |  |
| 1 | settable | BusFault | Pre-fetch fault, memory access fault | 0x0000 0014 |  |
| 2 | settable | UsageFault | Undefined instruction or illegal state | 0x0000 0018 |  |
| - | Reserved | 0x0000 001C - 0x0000 002B |  |  |  |
| 3 | settable | SVCall | System Service call via SWI instruction | 0x0000 002C |  |
| 4 | settable | Debug Monitor | 0x0000 0030 |  |  |
| - | Reserved | 0x0000 0034 |  |  |  |
| 5 | settable | PendSV | Pendable request for system service | 0x0000 0038 |  |
| 6 | settable | Systick | System tick timer | 0x0000 003C |  |
| 0 | settable | WWDG | Window Watchdog interrupt | 0x0000 0040 |  |
| 1 | settable | PVD | PVD through EXTI line detection interrupt | 0x0000 0044 |  |
| 2 | settable | TAMP_STAMP | Tamper and Timestamp interrupts through the EXTI line | 0x0000 0048 |  |
| 3 | settable | RTC_WKUP | RTC Wakeup interrupt through the EXTI line | 0x0000 004C |  |
| 4 | settable | FLASH | Flash global interrupt | 0x0000 0050 |  |
| 5 | settable | RCC | RCC global interrupt | 0x0000 0054 |  |
| 6 | settable | EXTI0 | EXTI Line0 interrupt | 0x0000 0058 |  |
| 7 | settable | EXTI1 | EXTI Line1 interrupt | 0x0000 005C |  |
| 8 | settable | EXTI2 | EXTI Line2 interrupt | 0x0000 0060 |  |
| 9 | settable | EXTI3 | EXTI Line3 interrupt | 0x0000 0064 |  |
| 10 | settable | EXTI4 | EXTI Line4 interrupt | 0x0000 0068 |  |
| 11 | settable | DMA1_Stream0 | DMA1 Stream0 global interrupt | 0x0000 006C |  |
| 12 | settable | DMA1_Stream1 | DMA1 Stream1 global interrupt | 0x0000 0070 |  |
| 13 | settable | DMA1_Stream2 | DMA1 Stream2 global interrupt | 0x0000 0074 |  |
| 14 | settable | DMA1_Stream3 | DMA1 Stream3 global interrupt | 0x0000 0078 |  |
| 15 | settable | DMA1_Stream4 | DMA1 Stream4 global interrupt | 0x0000 007C |  |
| 16 | settable | DMA1_Stream5 | DMA1 Stream5 global interrupt | 0x0000 0080 |  |
| 17 | settable | DMA1_Stream6 | DMA1 Stream6 global interrupt | 0x0000 0084 |  |


--- Page 182 ---

RM0090
Interrupts and events

Table 62. Vector table for STM32F42xxx and STM32F43xxx (continued)

| Position | Priority | Type of priority | Acronym | Description | Address |
|---|---|---|---|---|---|
| 18 | 25 | settable | ADC | ADC1, ADC2 and ADC3 global interrupts | 0x0000 0088 |
| 19 | 26 | settable | CAN1_TX | CAN1 TX interrupts | 0x0000 008C |
| 20 | 27 | settable | CAN1_RX0 | CAN1 RX0 interrupts | 0x0000 0090 |
| 21 | 28 | settable | CAN1_RX1 | CAN1 RX1 interrupt | 0x0000 0094 |
| 22 | 29 | settable | CAN1_SCE | CAN1 SCE interrupt | 0x0000 0098 |
| 23 | 30 | settable | EXTI9_5 | EXTI Line[9:5] interrupts | 0x0000 009C |
| 24 | 31 | settable | TIM1_BRK_TIM9 | TIM1 Break interrupt and TIM9 global interrupt | 0x0000 00A0 |
| 25 | 32 | settable | TIM1_UP_TIM10 | TIM1 Update interrupt and TIM10 global interrupt | 0x0000 00A4 |
| 26 | 33 | settable | TIM1_TRG_COM_TIM11 | TIM1 Trigger and Commutation interrupts and TIM11 global interrupt | 0x0000 00A8 |
| 27 | 34 | settable | TIM1_CC | TIM1 Capture Compare interrupt | 0x0000 00AC |
| 28 | 35 | settable | TIM2 | TIM2 global interrupt | 0x0000 00B0 |
| 29 | 36 | settable | TIM3 | TIM3 global interrupt | 0x0000 00B4 |
| 30 | 37 | settable | TIM4 | TIM4 global interrupt | 0x0000 00B8 |
| 31 | 38 | settable | I2C1_EV | I²C1 event interrupt | 0x0000 00BC |
| 32 | 39 | settable | I2C1_ER | I²C1 error interrupt | 0x0000 00C0 |
| 33 | 40 | settable | I2C2_EV | I²C2 event interrupt | 0x0000 00C4 |
| 34 | 41 | settable | I2C2_ER | I²C2 error interrupt | 0x0000 00C8 |
| 35 | 42 | settable | SPI1 | SPI1 global interrupt | 0x0000 00CC |
| 36 | 43 | settable | SPI2 | SPI2 global interrupt | 0x0000 00D0 |
| 37 | 44 | settable | USART1 | USART1 global interrupt | 0x0000 00D4 |
| 38 | 45 | settable | USART2 | USART2 global interrupt | 0x0000 00D8 |
| 39 | 46 | settable | USART3 | USART3 global interrupt | 0x0000 00DC |
| 40 | 47 | settable | EXTI15_10 | EXTI Line[15:10] interrupts | 0x0000 00E0 |
| 41 | 48 | settable | RTC_Alarm | RTC Alarms (A and B) through EXTI line interrupt | 0x0000 00E4 |
| 42 | 49 | settable | OTG_FS_WKUP | USB On-The-Go FS Wakeup through EXTI line interrupt | 0x0000 00E8 |
| 43 | 50 | settable | TIM8_BRK_TIM12 | TIM8 Break interrupt and TIM12 global interrupt | 0x0000 00EC |


ST
RM0090 Rev 18
377/1749

--- Page 183 ---

Interrupts and events
RM0090

Table 62. Vector table for STM32F42xxx and STM32F43xxx (continued)

| Position | Priority | Type of priority | Acronym | Description | Address |
|---|---|---|---|---|---|
| 44 | 51 | settable | TIM8_UP_TIM13 | TIM8 Update interrupt and TIM13 global interrupt | 0x0000 00F0 |
| 45 | 52 | settable | TIM8_TRG_COM_TIM14 | TIM8 Trigger and Commutation interrupts and TIM14 global interrupt | 0x0000 00F4 |
| 46 | 53 | settable | TIM8_CC | TIM8 Capture Compare interrupt | 0x0000 00F8 |
| 47 | 54 | settable | DMA1_Stream7 | DMA1 Stream7 global interrupt | 0x0000 00FC |
| 48 | 55 | settable | FSMC | FSMC global interrupt | 0x0000 0100 |
| 49 | 56 | settable | SDIO | SDIO global interrupt | 0x0000 0104 |
| 50 | 57 | settable | TIM5 | TIM5 global interrupt | 0x0000 0108 |
| 51 | 58 | settable | SPI3 | SPI3 global interrupt | 0x0000 010C |
| 52 | 59 | settable | UART4 | UART4 global interrupt | 0x0000 0110 |
| 53 | 60 | settable | UART5 | UART5 global interrupt | 0x0000 0114 |
| 54 | 61 | settable | TIM6_DAC | TIM6 global interrupt, DAC1 and DAC2 underrun error interrupts | 0x0000 0118 |
| 55 | 62 | settable | TIM7 | TIM7 global interrupt | 0x0000 011C |
| 56 | 63 | settable | DMA2_Stream0 | DMA2 Stream0 global interrupt | 0x0000 0120 |
| 57 | 64 | settable | DMA2_Stream1 | DMA2 Stream1 global interrupt | 0x0000 0124 |
| 58 | 65 | settable | DMA2_Stream2 | DMA2 Stream2 global interrupt | 0x0000 0128 |
| 59 | 66 | settable | DMA2_Stream3 | DMA2 Stream3 global interrupt | 0x0000 012C |
| 60 | 67 | settable | DMA2_Stream4 | DMA2 Stream4 global interrupt | 0x0000 0130 |
| 61 | 68 | settable | ETH | Ethernet global interrupt | 0x0000 0134 |
| 62 | 69 | settable | ETH_WKUP | Ethernet Wakeup through EXTI line interrupt | 0x0000 0138 |
| 63 | 70 | settable | CAN2_TX | CAN2 TX interrupts | 0x0000 013C |
| 64 | 71 | settable | CAN2_RX0 | CAN2 RX0 interrupts | 0x0000 0140 |
| 65 | 72 | settable | CAN2_RX1 | CAN2 RX1 interrupt | 0x0000 0144 |
| 66 | 73 | settable | CAN2_SCE | CAN2 SCE interrupt | 0x0000 0148 |
| 67 | 74 | settable | OTG_FS | USB On The Go FS global interrupt | 0x0000 014C |
| 68 | 75 | settable | DMA2_Stream5 | DMA2 Stream5 global interrupt | 0x0000 0150 |
| 69 | 76 | settable | DMA2_Stream6 | DMA2 Stream6 global interrupt | 0x0000 0154 |
| 70 | 77 | settable | DMA2_Stream7 | DMA2 Stream7 global interrupt | 0x0000 0158 |
| 71 | 78 | settable | USART6 | USART6 global interrupt | 0x0000 015C |


378/1749
RM0090 Rev 18

--- Page 184 ---

RM0090
Interrupts and events

**Table 62. Vector table for STM32F42xxx and STM32F43xxx (continued)**

| Position | Priority | Type of priority | Acronym | Description | Address |
|---|---|---|---|---|---|
| 72 | 79 | settable | I2C3_EV | I²C3 event interrupt | 0x0000 0160 |
| 73 | 80 | settable | I2C3_ER | I²C3 error interrupt | 0x0000 0164 |
| 74 | 81 | settable | OTG_HS_EP1_OUT | USB On The Go HS End Point 1 Out global interrupt | 0x0000 0168 |
| 75 | 82 | settable | OTG_HS_EP1_IN | USB On The Go HS End Point 1 In global interrupt | 0x0000 016C |
| 76 | 83 | settable | OTG_HS_WKUP | USB On The Go HS Wakeup through EXTI interrupt | 0x0000 0170 |
| 77 | 84 | settable | OTG_HS | USB On The Go HS global interrupt | 0x0000 0174 |
| 78 | 85 | settable | DCMI | DCMI global interrupt | 0x0000 0178 |
| 79 | 86 | settable | CRYP | CRYP crypto global interrupt | 0x0000 017C |
| 80 | 87 | settable | HASH_RNG | Hash and Rng global interrupt | 0x0000 0180 |
| 81 | 88 | Settable | FPU | FPU global interrupt | 0x0000 0184 |
| 82 | 89 | settable | UART7 | UART 7 global interrupt | 0x0000 0188 |
| 83 | 90 | settable | UART8 | UART 8 global interrupt | 0x0000 018C |
| 84 | 91 | settable | SPI4 | SPI 4 global interrupt | 0x0000 0190 |
| 85 | 92 | settable | SPI5 | SPI 5 global interrupt | 0x0000 0194 |
| 86 | 93 | settable | SPI6 | SPI 6 global interrupt | 0x0000 0198 |
| 87 | 94 | settable | SAI1 | SAI1 global interrupt | 0x0000 019C |
| 88 | 95 | settable | LCD-TFT | LTDC global interrupt | 0x0000 01A0 |
| 89 | 96 | settable | LCD-TFT | LTDC global Error interrupt | 0x0000 01A4 |
| 90 | 97 | settable | DMA2D | DMA2D global interrupt | 0x0000 01A8 |


### 12.2.1 EXTI main features

The main features of the EXTI controller are the following:

*   independent trigger and mask on each interrupt/event line
*   dedicated status bit for each interrupt line
*   generation of up to 23 software event/interrupt requests
*   detection of external signals with a pulse width lower than the APB2 clock period. Refer to the electrical characteristics section of the STM32F4xx datasheets for details on this parameter.

RM0090 Rev 18
379/1749

--- Page 185 ---

Interrupts and events
RM0090

## 12.2.2 EXTI block diagram

*Figure 41* shows the block diagram.

**Figure 41. External interrupt/event controller block diagram**

AMBA APB bus
PCLK2 -> Peripheral interface
23
Pending request register
23
Interrupt mask register
23
Software interrupt event register
23
Rising trigger selection register
23
Falling trigger selection register
To NVIC interrupt controller
23
23
23
23
23
Pulse generator
23
23
Event mask register
Edge detect circuit
Input line
MS32662V1

## 12.2.3 Wakeup event management

The STM32F4xx are able to handle external or internal events in order to wake up the core (WFE). The wakeup event can be generated either by:

* enabling an interrupt in the peripheral control register but not in the NVIC, and enabling the SEVONPEND bit in the Cortex®-M4 with FPU System Control register. When the MCU resumes from WFE, the peripheral interrupt pending bit and the peripheral NVIC IRQ channel pending bit (in the NVIC interrupt clear pending register) have to be cleared.
* or configuring an external or internal EXTI line in event mode. When the CPU resumes from WFE, it is not necessary to clear the peripheral interrupt pending bit or the NVIC IRQ channel pending bit as the pending bit corresponding to the event line is not set.

To use an external line as a wakeup event, refer to *Section 12.2.4: Functional description*.

## 12.2.4 Functional description

To generate the interrupt, the interrupt line should be configured and enabled. This is done by programming the two trigger registers with the desired edge detection and by enabling the interrupt request by writing a '1' to the corresponding bit in the interrupt mask register. When the selected edge occurs on the external interrupt line, an interrupt request is

380/1749
RM0090 Rev 18
ST

--- Page 186 ---

RM0090
Interrupts and events

generated. The pending bit corresponding to the interrupt line is also set. This request is reset by writing a '1' in the pending register.

To generate the event, the event line should be configured and enabled. This is done by programming the two trigger registers with the desired edge detection and by enabling the event request by writing a '1' to the corresponding bit in the event mask register. When the selected edge occurs on the event line, an event pulse is generated. The pending bit corresponding to the event line is not set.

An interrupt/event request can also be generated by software by writing a '1' in the software interrupt/event register.

**Hardware interrupt selection**

To configure the 23 lines as interrupt sources, use the following procedure:

*   Configure the mask bits of the 23 interrupt lines (EXTI_IMR)
*   Configure the Trigger selection bits of the interrupt lines (EXTI_RTSR and EXTI_FTSR)
*   Configure the enable and mask bits that control the NVIC IRQ channel mapped to the external interrupt controller (EXTI) so that an interrupt coming from one of the 23 lines can be correctly acknowledged.

**Hardware event selection**

To configure the 23 lines as event sources, use the following procedure:

*   Configure the mask bits of the 23 event lines (EXTI_EMR)
*   Configure the Trigger selection bits of the event lines (EXTI_RTSR and EXTI_FTSR)

**Software interrupt/event selection**

The 23 lines can be configured as software interrupt/event lines. The following is the procedure to generate a software interrupt.

*   Configure the mask bits of the 23 interrupt/event lines (EXTI_IMR, EXTI_EMR)
*   Set the required bit in the software interrupt register (EXTI_SWIER)

STI
RM0090 Rev 18
381/1749

--- Page 187 ---

Interrupts and events
RM0090

### 12.2.5 External interrupt/event line mapping

Up to 140 GPIOs (STM32F405xx/07xx and STM32F415xx/17xx), 168 GPIOs (STM32F42xxx and STM32F43xxx) are connected to the 16 external interrupt/event lines in the following manner:

**Figure 42. External interrupt/event GPIO mapping (STM32F405xx/07xx and STM32F415xx/17xx)**

EXTI0[3:0] bits in the SYSCFG_EXTICR1 register
PA0
PB0
PC0
PD0
PE0
PF0
PG0
PH0
PI0
EXTI0

EXTI1[3:0] bits in the SYSCFG_EXTICR1 register
PA1
PB1
PC1
PD1
PE1
PF1
PG1
PH1
PI1
EXTI1

...

EXTI15[3:0] bits in the SYSCFG_EXTICR4 register
PA15
PB15
PC15
PD15
PE15
PF15
PG15
PH15
EXTI15

ai15897

382/1749
RM0090 Rev 18

--- Page 188 ---

RM0090
Interrupts and events

Figure 43. External interrupt/event GPIO mapping (STM32F42xxx and STM32F43xxx)

EXTI0[3:0] bits in the SYSCFG_EXTICR1 register
PA0
PB0
PC0
PD0
PE0
PF0
PG0
PH0
PI0
PJ0
PK0
EXTI0

EXTI1[3:0] bits in the SYSCFG_EXTICR1 register
PA1
PB1
PC1
PD1
PE1
PF1
PG1
PH1
PI1
PJ1
PK1
EXTI1

EXTI15[3:0] bits in the SYSCFG_EXTICR4 register
PA15
PB15
PC15
PD15
PE15
PF15
PG15
PH15
PJ15
EXTI15

MS30440V1

The seven other EXTI lines are connected as follows:
* EXTI line 16 is connected to the PVD output
* EXTI line 17 is connected to the RTC Alarm event
* EXTI line 18 is connected to the USB OTG FS Wakeup event
* EXTI line 19 is connected to the Ethernet Wakeup event
* EXTI line 20 is connected to the USB OTG HS (configured in FS) Wakeup event
* EXTI line 21 is connected to the RTC Tamper and TimeStamp events
* EXTI line 22 is connected to the RTC Wakeup event

ST
RM0090 Rev 18
383/1749

--- Page 189 ---

Interrupts and events
RM0090


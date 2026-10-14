> [RM0090 ch.10 §10.3] DMA functional description


### 10.3.3 Channel selection

Each stream is associated with a DMA request that can be selected out of 8 possible channel requests. The selection is controlled by the CHSEL[2:0] bits in the DMA_SxCR register.

**Figure 35. Channel selection**

REQ_STRx_CH7
REQ_STRx_CH6
REQ_STRx_CH5
REQ_STRx_CH4
REQ_STRx_CH3
REQ_STRx_CH2
REQ_STRx_CH1
REQ_STRx_CH0

REQ_STREAMx

31 27 25 0
DMA_SxCR CHSEL[2:0]

ai15947b

The 8 requests from the peripherals (TIM, ADC, SPI, I2C, etc.) are independently connected to each channel and their connection depends on the product implementation.

See the following table(s) for examples of DMA request mappings.

**Table 42. DMA1 request mapping**

| Peripheral requests | Stream 0 | Stream 1 | Stream 2 | Stream 3 | Stream 4 | Stream 5 | Stream 6 | Stream 7 |
|---|---|---|---|---|---|---|---|---|
| Channel 0 | SPI3_RX | - | SPI3_RX | SPI2_RX | SPI2_TX | SPI3_TX | - | SPI3_TX |
| Channel 1 | I2C1_RX | - | TIM7_UP | - | TIM7_UP | I2C1_RX | I2C1_TX |  |
| Channel 2 | TIM4_CH1 | - | I2S3_EXT_RX | TIM4_CH2 | I2S2_EXT_TX | I2S3_EXT_TX | TIM4_UP | TIM4_CH3 |
| Channel 3 | I2S3_EXT_RX | TIM2_UP TIM2_CH3 | I2C3_RX | I2S2_EXT_RX | I2C3_TX | TIM2_CH1 | TIM2_CH2 TIM2_CH4 | TIM2_UP TIM2_CH4 |
| Channel 4 | UART5_RX | USART3_RX | UART4_RX | USART_TX | UART4_TX | USART2_RX | USART2_TX | UART5_TX |
| Channel 5 | UART8_TX(1) | UART7_TX(1) | TIM3_CH4 TIM3_UP | UART7_RX(1) | TIM3_CH1 TIM3_TRIG | TIM3_CH2 | UART8_RX(1) | TIM3_CH3 |


ST
RM0090 Rev 18
307/1749

--- Page 113 ---

DMA controller (DMA)
RM0090

Table 42. DMA1 request mapping (continued)

| Peripheral requests | Stream 0 | Stream 1 | Stream 2 | Stream 3 | Stream 4 | Stream 5 | Stream 6 | Stream 7 |
|---|---|---|---|---|---|---|---|---|
| Channel 6 | TIM5_CH3TIM5_UP | TIM5_CH4TIM5_TRIG | TIM5_CH1 | TIM5_CH4TIM5_TRIG | TIM5_CH2 | - | TIM5_UP | - |
| Channel 7 | - | TIM6_UP | I2C2_RX | USART3_TX | DAC1 | DAC2 | I2C2_TX |  |


1. These requests are available on STM32F42xxx and STM32F43xxx only.

Table 43. DMA2 request mapping

| Peripheral requests | Stream 0 | Stream 1 | Stream 2 | Stream 3 | Stream 4 | Stream 5 | Stream 6 | Stream 7 |
|---|---|---|---|---|---|---|---|---|
| Channel 0 | ADC1 | SAI1_A(1) | TIM8_CH1TIM8_CH2TIM8_CH3 | SAI1_A(1) | ADC1 | SAI1_B(1) | TIM1_CH1TIM1_CH2TIM1_CH3 | - |
| Channel 1 | - | DCMI | ADC2 | SAI1_B(1) | SPI6_TX(1) | SPI6_RX(1) | DCMI |  |
| Channel 2 | ADC3 | - | SPI5_RX(1) | SPI5_TX(1) | CRYP_OUT | CRYP_IN | HASH_IN |  |
| Channel 3 | SPI1_RX | - | SPI1_RX | SPI1_TX | - | SPI1_TX | - |  |
| Channel 4 | SPI4_RX(1) | SPI4_TX(1) | USART1_RX | SDIO | - | USART1_RX | SDIO | USART1_TX |
| Channel 5 | - | USART6_RX | SPI4_RX(1) | SPI4_TX(1) | - | USART6_TX |  |  |
| Channel 6 | TIM1_TRIG | TIM1_CH1 | TIM1_CH2 | TIM1_CH1 | TIM1_CH4TIM1_TRIGTIM1_COM | TIM1_UP | TIM1_CH3 | - |
| Channel 7 | - | TIM8_UP | TIM8_CH1 | TIM8_CH2 | TIM8_CH3 | SPI5_RX(1) | SPI5_TX(1) | TIM8_CH4TIM8_TRIGTIM8_COM |


1. These requests are available on STM32F42xxx and STM32F43xxx.

> [RM0090 ch.10 §10.3] DMA functional description

### 10.3.4 Arbiter

### 10.3.4 Arbiter

An arbiter manages the 8 DMA stream requests based on their priority for each of the two AHB master ports (memory and peripheral ports) and launches the peripheral/memory access sequences.

Priorities are managed in two stages:

* Software: each stream priority can be configured in the DMA_SxCR register. There are four levels:
    - Very high priority
    - High priority
    - Medium priority
    - Low priority
* Hardware: If two requests have the same software priority level, the stream with the lower number takes priority over the stream with the higher number. For example, Stream 2 takes priority over Stream 4.

308/1749
RM0090 Rev 18
ST

--- Page 114 ---

RM0090
DMA controller (DMA)

> [RM0090 ch.10 §10.3] DMA functional description

### 10.3.5 DMA streams

### 10.3.5 DMA streams

Each of the 8 DMA controller streams provides a unidirectional transfer link between a source and a destination.

Each stream can be configured to perform:

*   Regular type transactions: memory-to-peripherals, peripherals-to-memory or memory-to-memory transfers
*   Double-buffer type transactions: double buffer transfers using two memory pointers for the memory (while the DMA is reading/writing from/to a buffer, the application can write/read to/from the other buffer).

The amount of data to be transferred (up to 65535) is programmable and related to the source width of the peripheral that requests the DMA transfer connected to the peripheral AHB port. The register that contains the amount of data items to be transferred is decremented after each transaction.

> [RM0090 ch.10 §10.3] DMA functional description

### 10.3.6 Source, destination and transfer modes

### 10.3.6 Source, destination and transfer modes

Both source and destination transfers can address peripherals and memories in the entire 4 GB area, at addresses comprised between 0x0000 0000 and 0xFFFF FFFF.

The direction is configured using the DIR[1:0] bits in the DMA_SxCR register and offers three possibilities: memory-to-peripheral, peripheral-to-memory or memory-to-memory transfers. Table 44 describes the corresponding source and destination addresses.

Table 44. Source and destination address

| Bits DIR[1:0] of the DMA_SxCR register | Direction | Source address | Destination address |
|---|---|---|---|
| 00 | Peripheral-to-memory | DMA_SxPAR | DMA_SxM0AR |
| 01 | Memory-to-peripheral | DMA_SxM0AR | DMA_SxPAR |
| 10 | Memory-to-memory | DMA_SxPAR | DMA_SxM0AR |
| 11 | reserved | - |  |


When the data width (programmed in the PSIZE or MSIZE bits in the DMA_SxCR register) is a half-word or a word, respectively, the peripheral or memory address written into the DMA_SxPAR or DMA_M0AR/M1AR registers has to be aligned on a word or half-word address boundary, respectively.


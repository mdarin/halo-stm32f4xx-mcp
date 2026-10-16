### 3.23 Universal synchronous/asynchronous receiver transmitters (USART)

The STM32F405xx and STM32F407xx embed four universal synchronous/asynchronous receiver transmitters (USART1, USART2, USART3 and USART6) and two universal asynchronous receiver transmitters (UART4 and UART5).

These six interfaces provide asynchronous communication, IrDA SIR ENDEC support, multiprocessor communication mode, single-wire half-duplex communication mode and have LIN Master/Slave capability. The USART1 and USART6 interfaces are able to communicate at speeds of up to 10.5 Mbit/s. The other available interfaces communicate at up to 5.25 Mbit/s.

USART1, USART2, USART3 and USART6 also provide hardware management of the CTS and RTS signals, Smart Card mode (ISO 7816 compliant) and SPI-like communication capability. All interfaces can be served by the DMA controller.

36/206
DS8626 Rev 12
ST

--- Page 37 ---

STM32F405xx, STM32F407xx
Functional overview

Table 5. USART feature comparison

| USART name | Standard features | Modem (RTS/CTS) | LIN | SPI master | irDA | Smartcard (ISO 7816) | Max. baud rate in Mbit/s (oversampling by 16) | Max. baud rate in Mbit/s (oversampling by 8) | APB mapping |
|---|---|---|---|---|---|---|---|---|---|
| USART1 | X | 5.25 | 10.5 | APB2 (max. 84 MHz) |  |  |  |  |  |
| USART2 | X | 2.62 | 5.25 | APB1 (max. 42 MHz) |  |  |  |  |  |
| USART3 | X | 2.62 | 5.25 | APB1 (max. 42 MHz) |  |  |  |  |  |
| UART4 | X | - | X | - | X | - | 2.62 | 5.25 | APB1 (max. 42 MHz) |
| UART5 | X | - | X | - | X | - | 2.62 | 5.25 | APB1 (max. 42 MHz) |
| USART6 | X | 5.25 | 10.5 | APB2 (max. 84 MHz) |  |  |  |  |  |


### 3.24 Serial peripheral interface (SPI)

The STM32F40xxx feature up to three SPIs in slave and master modes in full-duplex and simplex communication modes. SPI1 can communicate at up to 42 Mbits/s, SPI2 and SPI3 can communicate at up to 21 Mbit/s. The 3-bit prescaler gives 8 master mode frequencies and the frame is configurable to 8 bits or 16 bits. The hardware CRC generation/verification supports basic SD Card/MMC modes. All SPIs can be served by the DMA controller.

The SPI interface can be configured to operate in TI mode for communications in master mode and slave mode.

### 3.25 Inter-integrated sound (I²S)

Two standard I²S interfaces (multiplexed with SPI2 and SPI3) are available. They can be operated in master or slave mode, in full duplex and half-duplex communication modes, and can be configured to operate with a 16-/32-bit resolution as an input or output channel. Audio sampling frequencies from 8 kHz up to 192 kHz are supported. When either or both of the I²S interfaces is/are configured in master mode, the master clock can be output to the external DAC/CODEC at 256 times the sampling frequency.

All I²Sx can be served by the DMA controller.

ST
DS8626 Rev 12
37/206

--- Page 38 ---

Functional overview
STM32F405xx, STM32F407xx

### 3.26 Audio PLL (PLLI2S)

The devices feature an additional dedicated PLL for audio I²S application. It allows to achieve error-free I²S sampling clock accuracy without compromising on the CPU performance, while using USB peripherals.

The PLLI2S configuration can be modified to manage an I²S sample rate change without disabling the main PLL (PLL) used for CPU, USB and Ethernet interfaces.

The audio PLL can be programmed with very low error to obtain sampling rates ranging from 8 KHz to 192 KHz.

In addition to the audio PLL, a master clock input pin can be used to synchronize the I²S flow with an external PLL (or Codec output).

### 3.27 Secure digital input/output interface (SDIO)

An SD/SDIO/MMC host interface is available, that supports MultiMediaCard System Specification Version 4.2 in three different databus modes: 1-bit (default), 4-bit and 8-bit.

The interface allows data transfer at up to 48 MHz, and is compliant with the SD Memory Card Specification Version 2.0.

The SDIO Card Specification Version 2.0 is also supported with two different databus modes: 1-bit (default) and 4-bit.

The current version supports only one SD/SDIO/MMC4.2 card at any one time and a stack of MMC4.1 or previous.

In addition to SD/SDIO/MMC, this interface is fully compliant with the CE-ATA digital protocol Rev1.1.


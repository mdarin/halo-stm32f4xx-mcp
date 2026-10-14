**6.3.15 RCC AHB1 peripheral clock enable in low power mode register (RCC_AHB1LPENR)**

Address offset: 0x50

Reset value: 0x7EEF 97FF

Access: no wait state, word, half-word and byte access.

31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16

| Res. | OTGHS ULPILPE N | OTGH S LPEN | ETHPT P LPEN | ETHRX LPEN | ETHTX LPEN | ETHMA C LPEN | Res. | DMA2D LPEN | DMA2 LPEN | DMA1 LPEN | Res. | SRAM3 LPEN | BKPSRA M LPEN | SRAM 2 LPEN | SRAM 1 LPEN |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0

| FLITF LPEN | Reserved | CRC LPEN | Res. | GPIOK LPEN | GPIOIJ LPEN | GPIOI LPEN | GPIOH LPEN | GPIOG G LPEN | GPIO F LPEN | GPIOE LPEN | GPIOD LPEN | GPIOC LPEN | GPIOB LPEN | GPIOA LPEN |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bit 31 Reserved, must be kept at reset value.

Bit 30 OTGHSULPILPEN: USB OTG HS ULPI clock enable during Sleep mode

This bit is set and cleared by software. It must be cleared when the OTG_HS is used in FS mode.

0: USB OTG HS ULPI clock disabled during Sleep mode

1: USB OTG HS ULPI clock enabled during Sleep mode

Bit 29 OTGHSLPEN: USB OTG HS clock enable during Sleep mode

This bit is set and cleared by software.

0: USB OTG HS clock disabled during Sleep mode

1: USB OTG HS clock enabled during Sleep mode

Bit 28 ETHMACPTPLPEN: Ethernet PTP clock enable during Sleep mode

This bit is set and cleared by software.

0: Ethernet PTP clock disabled during Sleep mode

1: Ethernet PTP clock enabled during Sleep mode

Bit 27 ETHMACRXLPEN: Ethernet reception clock enable during Sleep mode

This bit is set and cleared by software.

0: Ethernet reception clock disabled during Sleep mode

1: Ethernet reception clock enabled during Sleep mode

Bit 26 ETHMACTXLPEN: Ethernet transmission clock enable during Sleep mode

This bit is set and cleared by software.

0: Ethernet transmission clock disabled during sleep mode

1: Ethernet transmission clock enabled during sleep mode

Bit 25 ETHMACLPEN: Ethernet MAC clock enable during Sleep mode

This bit is set and cleared by software.

0: Ethernet MAC clock disabled during Sleep mode

1: Ethernet MAC clock enabled during Sleep mode

Bit 24 Reserved, must be kept at reset value.



212

--- Page 190 ---

**Reset and clock control for   STM32F42xxx and STM32F43xxx (RCC) RM0090**

Bit 23 DMA2DLPEN: DMA2D clock enable during Sleep mode

This bit is set and cleared by software.

0: DMA2D clock disabled during Sleep mode

1: DMA2D clock enabled during Sleep mode

Bit 22 DMA2LPEN: DMA2 clock enable during Sleep mode

This bit is set and cleared by software.

0: DMA2 clock disabled during Sleep mode

1: DMA2 clock enabled during Sleep mode

Bit 21 DMA1LPEN: DMA1 clock enable during Sleep mode

This bit is set and cleared by software.

0: DMA1 clock disabled during Sleep mode

1: DMA1 clock enabled during Sleep mode

Bit 20 Reserved, must be kept at reset value.

Bit 19 SRAM3LPEN: SRAM3 interface clock enable during Sleep mode

This bit is set and cleared by software.

0: SRAM3 interface clock disabled during Sleep mode

1: SRAM3 interface clock enabled during Sleep mode

Bit 18 BKPSRAMLPEN: Backup SRAM interface clock enable during Sleep mode This bit is set and cleared by software.

0: Backup SRAM interface clock disabled during Sleep mode

1: Backup SRAM interface clock enabled during Sleep mode

Bit 17 SRAM2LPEN: SRAM2 interface clock enable during Sleep mode

This bit is set and cleared by software.

0: SRAM2 interface clock disabled during Sleep mode

1: SRAM2 interface clock enabled during Sleep mode

Bit 16 SRAM1LPEN: SRAM1 interface clock enable during Sleep mode

This bit is set and cleared by software.

0: SRAM1 interface clock disabled during Sleep mode

1: SRAM1 interface clock enabled during Sleep mode

Bit 15 FLITFLPEN: Flash interface clock enable during Sleep mode

This bit is set and cleared by software.

0: Flash interface clock disabled during Sleep mode

1: Flash interface clock enabled during Sleep mode

Bits 14:13 Reserved, must be kept at reset value.

Bit 12 CRCLPEN: CRC clock enable during Sleep mode

This bit is set and cleared by software.

0: CRC clock disabled during Sleep mode

1: CRC clock enabled during Sleep mode

Bit 11 Reserved, must be kept at reset value.

Bit 10 GPIOKLPEN: IO port K clock enable during Sleep mode

This bit is set and cleared by software.

0: IO port K clock disabled during Sleep mode

1: IO port K clock enabled during Sleep mode

190/1749 RM0090 Rev 18

--- Page 191 ---

**RM0090 Reset and clock control for   STM32F42xxx and STM32F43xxx (RCC)**

Bit 9 GPIOJLPEN: IO port J clock enable during Sleep mode

This bit is set and cleared by software.

0: IO port J clock disabled during Sleep mode

1: IO port J clock enabled during Sleep mode

Bit 8 GPIOILPEN: IO port I clock enable during Sleep mode

This bit is set and cleared by software.

0: IO port I clock disabled during Sleep mode

1: IO port I clock enabled during Sleep mode

Bit 7 GPIOHLPEN: IO port H clock enable during Sleep mode

This bit is set and cleared by software.

0: IO port H clock disabled during Sleep mode

1: IO port H clock enabled during Sleep mode

Bits 6 GPIOGLPEN: IO port G clock enable during Sleep mode

This bit is set and cleared by software.

0: IO port G clock disabled during Sleep mode

1: IO port G clock enabled during Sleep mode

Bit 5 GPIOFLPEN: IO port F clock enable during Sleep mode

This bit is set and cleared by software.

0: IO port F clock disabled during Sleep mode

1: IO port F clock enabled during Sleep mode

Bit 4 GPIOELPEN: IO port E clock enable during Sleep mode

Set and cleared by software.

0: IO port E clock disabled during Sleep mode

1: IO port E clock enabled during Sleep mode

Bit 3 GPIODLPEN: IO port D clock enable during Sleep mode

This bit is set and cleared by software.

0: IO port D clock disabled during Sleep mode

1: IO port D clock enabled during Sleep mode

Bit 2 GPIOCLPEN: IO port C clock enable during Sleep mode

This bit is set and cleared by software.

0: IO port C clock disabled during Sleep mode

1: IO port C clock enabled during Sleep mode

Bit 1 GPIOBLPEN: IO port B clock enable during Sleep mode

This bit is set and cleared by software.

0: IO port B clock disabled during Sleep mode

1: IO port B clock enabled during Sleep mode

Bit 0 GPIOALPEN: IO port A clock enable during sleep mode

This bit is set and cleared by software.

0: IO port A clock disabled during Sleep mode

1: IO port A clock enabled during Sleep mode

RM0090 Rev 18 191/1749

212

--- Page 192 ---

**Reset and clock control for   STM32F42xxx and STM32F43xxx (RCC) RM0090**


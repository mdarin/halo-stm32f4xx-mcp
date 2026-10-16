**6.3.10 RCC AHB1 peripheral clock register (RCC_AHB1ENR)**

Address offset: 0x30

Reset value: 0x0010 0000

Access: no wait state, word, half-word and byte access.

31 30 29 28 27 26 25 24

23 22 21 20 19 18 17 16

| Reser- ved | OTGH S ULPIE N | OTGH SEN | ETHM ACPTP EN | ETHM ACRXE N | ETHM ACTXE N | ETHMA CEN | Res. | DMA2D EN | DMA2E N | DMA1E N | CCMDAT ARAMEN | Res. | BKPSR AMEN | Reserved |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| rw |  |  | rw |  |  |  |  |  |  |  |  |  |  |  |


15 14 13 12 11 10 9 8

7 6 5 4 3 2 1 0

| Reserved | CRCE N | Res. | GPIOK EN | GPIOJ EN | GPIOIE N | GPIOH EN | GPIOG EN | GPIOFE N | GPIOEEN | GPIOD EN | GPIOC EN | GPIO BEN | GPIO AEN |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bit 31 Reserved, must be kept at reset value.

Bit 30 OTGHSULPIEN: USB OTG HSULPI clock enable

This bit is set and cleared by software. It must be cleared when the OTG_HS is used in FS

mode.

0: USB OTG HS ULPI clock disabled

1: USB OTG HS ULPI clock enabled

Bit 29 OTGHSEN: USB OTG HS clock enable

This bit is set and cleared by software.

0: USB OTG HS clock disabled

1: USB OTG HS clock enabled

Bit 28 ETHMACPTPEN: Ethernet PTP clock enable

This bit is set and cleared by software.

0: Ethernet PTP clock disabled

1: Ethernet PTP clock enabled

Bit 27 ETHMACRXEN: Ethernet Reception clock enable This bit is set and cleared by software.

0: Ethernet Reception clock disabled

1: Ethernet Reception clock enabled

Bit 26 ETHMACTXEN: Ethernet Transmission clock enable This bit is set and cleared by software.

0: Ethernet Transmission clock disabled

1: Ethernet Transmission clock enabled

Bit 25 ETHMACEN: Ethernet MAC clock enable

This bit is set and cleared by software.

0: Ethernet MAC clock disabled

1: Ethernet MAC clock enabled

Bit 24 Reserved, must be kept at reset value.

Bit 23 DMA2DEN: DMA2D clock enable

This bit is set and cleared by software.

0: DMA2D clock disabled

1: DMA2D clock enabled



--- Page 181 ---

**RM0090**

**Reset and clock control for   STM32F42xxx and STM32F43xxx (RCC)**

Bit 22 DMA2EN: DMA2 clock enable

This bit is set and cleared by software. 0: DMA2 clock disabled

1: DMA2 clock enabled

Bit 21 DMA1EN: DMA1 clock enable

This bit is set and cleared by software. 0: DMA1 clock disabled

1: DMA1 clock enabled

Bit 20 CCMDATARAMEN : CCM data RAM clock enable This bit is set and cleared by software. 0: CCM data RAM clock disabled 1: CCM data RAM clock enabled Bit 19 Reserved, must be kept at reset value.

Bit 18 BKPSRAMEN: Backup SRAM interface clock enable This bit is set and cleared by software. 0: Backup SRAM interface clock disabled 1: Backup SRAM interface clock enabled Bits 17:13 Reserved, must be kept at reset value.

Bit 12 CRCEN: CRC clock enable

This bit is set and cleared by software. 0: CRC clock disabled

1: CRC clock enabled

Bit 11 Reserved, must be kept at reset value.

Bit 10 GPIOKEN: IO port K clock enable This bit is set and cleared by software. 0: IO port K clock disabled

1: IO port K clock enabled

Bit 9 GPIOJEN: IO port J clock enable This bit is set and cleared by software. 0: IO port J clock disabled

1: IO port J clock enabled

Bit 8 GPIOIEN: IO port I clock enable This bit is set and cleared by software. 0: IO port I clock disabled

1: IO port I clock enabled

Bit 7 GPIOHEN: IO port H clock enable This bit is set and cleared by software. 0: IO port H clock disabled

1: IO port H clock enabled

Bit 6 GPIOGEN: IO port G clock enable This bit is set and cleared by software. 0: IO port G clock disabled

1: IO port G clock enabled

RM0090 Rev 18 181/1749

212

--- Page 182 ---

**Reset and clock control for   STM32F42xxx and STM32F43xxx (RCC)**

**RM0090**

Bit 5 GPIOFEN: IO port F clock enable

This bit is set and cleared by software.

0: IO port F clock disabled

1: IO port F clock enabled

Bit 4 GPIOEEN: IO port E clock enable

This bit is set and cleared by software.

0: IO port E clock disabled

1: IO port E clock enabled

Bit 3 GPIODEN: IO port D clock enable

This bit is set and cleared by software.

0: IO port D clock disabled

1: IO port D clock enabled

Bit 2 GPIOCEN: IO port C clock enable

This bit is set and cleared by software.

0: IO port C clock disabled

1: IO port C clock enabled

Bit 1 GPIOBEN: IO port B clock enable

This bit is set and cleared by software.

0: IO port B clock disabled

1: IO port B clock enabled

Bit 0 GPIOAEN: IO port A clock enable

This bit is set and cleared by software.

0: IO port A clock disabled

1: IO port A clock enabled

**6.3.11**

**RCC AHB2 peripheral clock enable register (RCC_AHB2ENR)**

Address offset: 0x34

Reset value: 0x0000 0000

Access: no wait state, word, half-word and byte access.

31 30 29 28 27 26 25 24

23 22 21 20 19 18 17 16 Reserved

15 14 13 12 11 10 9 8

7 6 5 4 3 2 1 0

| Reserved | OTGFS EN | RNG EN | HASH EN | CRYP EN | Reserved | DCMI EN |
|---|---|---|---|---|---|---|
| rw |  |  |  |  |  |  |


Bits 31:8 Reserved, must be kept at reset value.

Bit 7 OTGFSEN: USB OTG FS clock enable This bit is set and cleared by software.

0: USB OTG FS clock disabled

1: USB OTG FS clock enabled

Bit 6 RNGEN: Random number generator clock enable This bit is set and cleared by software.

0: Random number generator clock disabled 1: Random number generator clock enabled

182/1749

RM0090 Rev 18

--- Page 183 ---

**RM0090**

**Reset and clock control for   STM32F42xxx and STM32F43xxx (RCC)**

Bit 5 HASHEN: Hash modules clock enable

This bit is set and cleared by software.

0: Hash modules clock disabled

1: Hash modules clock enabled

Bit 4 CRYPEN: Cryptographic modules clock enable

This bit is set and cleared by software.

0: cryptographic module clock disabled

1: cryptographic module clock enabled

Bits 3:1 Reserved, must be kept at reset value.

Bit 0 DCMIEN: Camera interface enable

This bit is set and cleared by software.

0: Camera interface clock disabled

1: Camera interface clock enabled

**6.3.12**

**RCC AHB3 peripheral clock enable register (RCC_AHB3ENR)**

Address offset: 0x38

Reset value: 0x0000 0000

Access: no wait state, word, half-word and byte access.

31 30 29 28 27 26 25 24

23 22 21 20 19 18 17 16 Reserved

15 14 13 12 11 10 9 8

7 6 5 4 3 2 1 0

| Reserved | FMCEN |
|---|---|
| rw |  |


Bits 31:1 Reserved, must be kept at reset value.

Bit 0 FMCEN: Flexible memory controller module clock enable

This bit is set and cleared by software.

0: FMC module clock disabled

1: FMC module clock enabled


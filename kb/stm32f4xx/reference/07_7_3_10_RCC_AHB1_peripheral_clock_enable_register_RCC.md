### 7.3.10 RCC AHB1 peripheral clock enable register (RCC_AHB1ENR)

Address offset: 0x30
Reset value: 0x0010 0000
Access: no wait state, word, half-word and byte access.

| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved | OTGHSULPIEN | OTGHSEN | ETHMACPTPEN | ETHMACRXEN | ETHMACTXEN | ETHMACEN | Reserved | DMA2EN | DMA1EN | CCMDATARAMEN | Res. | BKPSRAMEN | Reserved |  |  |
|  |  | rw |  |  | rw |  |  | rw |  |  |  |  |  |  |  |
| 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
| Reserved | CRCE N | Reserved | GPIOIEN | GPIOHEN | GPIOGEN | GPIOFEN | GPIOEEN | GPIO DEN | GPIOCEN | GPIOBEN | GPIOAEN |  |  |  |  |
|  |  | rw |  |  | rw |  |  |  |  |  |  |  |  |  |  |


Bit 31 Reserved, must be kept at reset value.

Bit 30 **OTGHSULPIEN**: USB OTG HSULPI clock enable
Set and cleared by software. This bit must be cleared when the OTG_HS is used in FS mode.
0: USB OTG HS ULPI clock disabled
1: USB OTG HS ULPI clock enabled

Bit 29 **OTGHSEN**: USB OTG HS clock enable
Set and cleared by software.
0: USB OTG HS clock disabled
1: USB OTG HS clock enabled

Bit 28 **ETHMACPTPEN**: Ethernet PTP clock enable
Set and cleared by software.
0: Ethernet PTP clock disabled
1: Ethernet PTP clock enabled

Bit 27 **ETHMACRXEN**: Ethernet Reception clock enable
Set and cleared by software.
0: Ethernet Reception clock disabled
1: Ethernet Reception clock enabled

Bit 26 **ETHMACTXEN**: Ethernet Transmission clock enable
Set and cleared by software.
0: Ethernet Transmission clock disabled
1: Ethernet Transmission clock enabled

Bit 25 **ETHMACEN**: Ethernet MAC clock enable
Set and cleared by software.
0: Ethernet MAC clock disabled
1: Ethernet MAC clock enabled

Bits 24:23 Reserved, must be kept at reset value.

Bit 22 **DMA2EN**: DMA2 clock enable
Set and cleared by software.
0: DMA2 clock disabled
1: DMA2 clock enabled

242/1749 RM0090 Rev 18

--- Page 48 ---

RM0090 Reset and clock control for STM32F405xx/07xx and STM32F415xx/17xx(RCC)

Bit 21 **DMA1EN**: DMA1 clock enable
Set and cleared by software.
0: DMA1 clock disabled
1: DMA1 clock enabled

Bit 20 **CCMDATARAMEN**: CCM data RAM clock enable
Set and cleared by software.
0: CCM data RAM clock disabled
1: CCM data RAM clock enabled

Bit 19 Reserved, must be kept at reset value.

Bit 18 **BKPSRAMEN**: Backup SRAM interface clock enable
Set and cleared by software.
0: Backup SRAM interface clock disabled
1: Backup SRAM interface clock enabled

Bits 17:13 Reserved, must be kept at reset value.

Bit 12 **CRCEN**: CRC clock enable
Set and cleared by software.
0: CRC clock disabled
1: CRC clock enabled

Bits 11:9 Reserved, must be kept at reset value.

Bit 8 **GPIOIEN**: IO port I clock enable
Set and cleared by software.
0: IO port I clock disabled
1: IO port I clock enabled

Bit 7 **GPIOHEN**: IO port H clock enable
Set and cleared by software.
0: IO port H clock disabled
1: IO port H clock enabled

Bit 6 **GPIOGEN**: IO port G clock enable
Set and cleared by software.
0: IO port G clock disabled
1: IO port G clock enabled

Bit 5 **GPIOFEN**: IO port F clock enable
Set and cleared by software.
0: IO port F clock disabled
1: IO port F clock enabled

Bit 4 **GPIOEEN**: IO port E clock enable
Set and cleared by software.
0: IO port E clock disabled
1: IO port E clock enabled

Bit 3 **GPIODEN**: IO port D clock enable
Set and cleared by software.
0: IO port D clock disabled
1: IO port D clock enabled

ST
RM0090 Rev 18
243/1749

--- Page 49 ---

**Reset and clock control for STM32F405xx/07xx and STM32F415xx/17xx(RCC)** RM0090

Bit 2 **GPIOCEN**: IO port C clock enable
Set and cleared by software.
0: IO port C clock disabled
1: IO port C clock enabled

Bit 1 **GPIOBEN**: IO port B clock enable
Set and cleared by software.
0: IO port B clock disabled
1: IO port B clock enabled

Bit 0 **GPIOAEN**: IO port A clock enable
Set and cleared by software.
0: IO port A clock disabled
1: IO port A clock enabled

### 7.3.11 RCC AHB2 peripheral clock enable register (RCC_AHB2ENR)

Address offset: 0x34
Reset value: 0x0000 0000
Access: no wait state, word, half-word and byte access.

| 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 |  |  |  |  |  |  |
|---|---|---|---|---|---|---|
| Reserved |  |  |  |  |  |  |
| 15 14 13 12 11 10 9 8 | 7 | 6 | 5 | 4 | 3 2 1 | 0 |
| Reserved | OTGFS ENrw | RNG ENrw | HASH ENrw | CRYP ENrw | Reserved | DCMI ENrw |


Bits 31:8 Reserved, must be kept at reset value.

Bit 7 **OTGFSSEN**: USB OTG FS clock enable
Set and cleared by software.
0: USB OTG FS clock disabled
1: USB OTG FS clock enabled

Bit 6 **RNGEN**: Random number generator clock enable
Set and cleared by software.
0: Random number generator clock disabled
1: Random number generator clock enabled

Bit 5 **HASHEN**: Hash modules clock enable
Set and cleared by software.
0: Hash modules clock disabled
1: Hash modules clock enabled

244/1749 RM0090 Rev 18 ST

--- Page 50 ---

RM0090
**Reset and clock control for STM32F405xx/07xx and STM32F415xx/17xx(RCC)**

Bit 4 **CRYPEN**: Cryptographic modules clock enable
Set and cleared by software.
0: cryptographic module clock disabled
1: cryptographic module clock enabled

Bits 3:1 Reserved, must be kept at reset value.

Bit 0 **DCMIEN**: Camera interface enable
Set and cleared by software.
0: Camera interface clock disabled
1: Camera interface clock enabled

### 7.3.12 RCC AHB3 peripheral clock enable register (RCC_AHB3ENR)

Address offset: 0x38
Reset value: 0x0000 0000
Access: no wait state, word, half-word and byte access.

| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


| 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved | FSMCENrw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bits 31:1 Reserved, must be kept at reset value.

Bit 0 **FSMCEN**: Flexible static memory controller module clock enable
Set and cleared by software.
0: FSMC module clock disabled
1: FSMC module clock enabled


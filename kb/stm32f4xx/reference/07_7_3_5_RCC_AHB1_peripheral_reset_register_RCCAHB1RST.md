### 7.3.5 RCC AHB1 peripheral reset register (RCC_AHB1RSTR)

Address offset: 0x10
Reset value: 0x0000 0000
Access: no wait state, word, half-word and byte access.

| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved | OTGH S RST | Reserved | ETHMAC RST | Reserved | DMA2 RST | DMA1 RST | Reserved |  |  |  |  |  |  |  |  |
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
| Reserved | CRCR ST | Reserved | GPIOI RST | GPIOH RST | GPIOGG RST | GPIOF RST | GPIOE RST | GPIO D RST | GPIOC RST | GPIOB RST | GPIOA RST |  |  |  |  |
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bits 31:30 Reserved, must be kept at reset value.
Bit 29 **OTGH SRST**: USB OTG HS module reset
Set and cleared by software.
0: does not reset the USB OTG HS module
1: resets the USB OTG HS module

Bits 28:26 Reserved, must be kept at reset value.
Bit 25 **ETHMACRST**: Ethernet MAC reset
Set and cleared by software.
0: does not reset Ethernet MAC
1: resets Ethernet MAC

Bits 24:23 Reserved, must be kept at reset value.

ST
RM0090 Rev 18
233/1749

--- Page 39 ---

**Reset and clock control for STM32F405xx/07xx and STM32F415xx/17xx(RCC)** RM0090

Bit 22 **DMA2RST**: DMA2 reset
Set and cleared by software.
0: does not reset DMA2
1: resets DMA2

Bit 21 **DMA1RST**: DMA1 reset
Set and cleared by software.
0: does not reset DMA1
1: resets DMA1

Bits 20:13 Reserved, must be kept at reset value.

Bit 12 **CRCRST**: CRC reset
Set and cleared by software.
0: does not reset CRC
1: resets CRC

Bits 11:9 Reserved, must be kept at reset value.

Bit 8 **GPIOIRST**: IO port I reset
Set and cleared by software.
0: does not reset IO port I
1: resets IO port I

Bit 7 **GPIOHRST**: IO port H reset
Set and cleared by software.
0: does not reset IO port H
1: resets IO port H

Bits 6 **GPIOGRST**: IO port G reset
Set and cleared by software.
0: does not reset IO port G
1: resets IO port G

Bit 5 **GPIOFRST**: IO port F reset
Set and cleared by software.
0: does not reset IO port F
1: resets IO port F

Bit 4 **GPIOERST**: IO port E reset
Set and cleared by software.
0: does not reset IO port E
1: resets IO port E

Bit 3 **GPIOIRST**: IO port D reset
Set and cleared by software.
0: does not reset IO port D
1: resets IO port D

234/1749 RM0090 Rev 18

--- Page 40 ---

RM0090 Reset and clock control for STM32F405xx/07xx and STM32F415xx/17xx(RCC)

Bit 2 **GPIOCRST**: IO port C reset
Set and cleared by software.
0: does not reset IO port C
1: resets IO port C

Bit 1 **GPIOBRST**: IO port B reset
Set and cleared by software.
0: does not reset IO port B
1:resets IO port B

Bit 0 **GPIOARST**: IO port A reset
Set and cleared by software.
0: does not reset IO port A
1: resets IO port A

ST
RM0090 Rev 18
235/1749

--- Page 41 ---

Reset and clock control for STM32F405xx/07xx and STM32F415xx/17xx(RCC) RM0090

### 7.3.6 RCC AHB2 peripheral reset register (RCC_AHB2RSTR)

Address offset: 0x14
Reset value: 0x0000 0000
Access: no wait state, word, half-word and byte access

| 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 |  |  |  |  |  |  |
|---|---|---|---|---|---|---|
| Reserved |  |  |  |  |  |  |
| 15 14 13 12 11 10 9 8 | OTGFS RST | RNG RST | HASH RST | CRYP RST | 3 2 1 | DCMI RST |
| Reserved | rw | Reserved | rw |  |  |  |


Bits 31:8 Reserved, must be kept at reset value.

**Bit 7 OTGFSRST**: USB OTG FS module reset
Set and cleared by software.
0: does not reset the USB OTG FS module
1: resets the USB OTG FS module

**Bit 6 RNGRST**: Random number generator module reset
Set and cleared by software.
0: does not reset the random number generator module
1: resets the random number generator module

**Bit 5 HASHRST**: Hash module reset
Set and cleared by software.
0: does not reset the HASH module
1: resets the HASH module

**Bit 4 CRYPRST**: Cryptographic module reset
Set and cleared by software.
0: does not reset the cryptographic module
1: resets the cryptographic module

Bits 3:1 Reserved, must be kept at reset value.

**Bit 0 DCMIRST**: Camera interface reset
Set and cleared by software.
0: does not reset the Camera interface
1: resets the Camera interface

236/1749 RM0090 Rev 18 STI

--- Page 42 ---

RM0090 Reset and clock control for STM32F405xx/07xx and STM32F415xx/17xx(RCC)

### 7.3.7 RCC AHB3 peripheral reset register (RCC_AHB3RSTR)

Address offset: 0x18
Reset value: 0x0000 0000
Access: no wait state, word, half-word and byte access.

| 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reserved | FSMCRST |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | rw |


Bits 31:1 Reserved, must be kept at reset value.
Bit 0 **FSMCRST**: Flexible static memory controller module reset
Set and cleared by software.
0: does not reset the FSMC module
1: resets the FSMC module


**6.3.5 RCC AHB1 peripheral reset register (RCC_AHB1RSTR)**

Address offset: 0x10

Reset value: 0x0000 0000

Access: no wait state, word, half-word and byte access.

31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16

| Reserved | OTGH S RST | Reserved | ETHMAC RST | Res. | DMA2D RST | DMA2 RST | DMA1 RST | Reserved |
|---|---|---|---|---|---|---|---|---|
| rw |  |  |  |  |  |  |  |  |


15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0

| Reserved | CRCR ST | Res. | GPIOK RST | GPIOJ RST | GPIOI RST | GPIOH RST | GPIOGG RST | GPIOF RST | GPIOE RST | GPIOD RST | GPIOC RST | GPIOB RST | GPIOA RST |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bits 31:30 Reserved, must be kept at reset value.

Bit 29 OTGHSRST: USB OTG HS module reset

This bit is set and cleared by software.

0: does not reset the USB OTG HS module

1: resets the USB OTG HS module

Bits 28:26 Reserved, must be kept at reset value.

170/1749 RM0090 Rev 18

--- Page 171 ---

**RM0090**

**Reset and clock control for   STM32F42xxx and STM32F43xxx (RCC)**

Bit 25 ETHMACRST: Ethernet MAC reset This bit is set and cleared by software. 0: does not reset Ethernet MAC 1: resets Ethernet MAC Bit 24 Reserved, must be kept at reset value. Bit 23 DMA2DRST : DMA2D reset This bit is set and reset by software. 0: does not reset DMA2D 1: resets DMA2D Bit 22 DMA2RST: DMA2 reset This bit is set and cleared by software. 0: does not reset DMA2 1: resets DMA2

Bit 21 DMA1RST: DMA2 reset This bit is set and cleared by software. 0: does not reset DMA2 1: resets DMA2

Bits 20:13 Reserved, must be kept at reset value. Bit 12 CRCRST: CRC reset This bit is set and cleared by software. 0: does not reset CRC 1: resets CRC

Bit 11 Reserved, must be kept at reset value. Bit 10 GPIOKRST : IO port K reset This bit is set and cleared by software. 0: does not reset IO port K 1: resets IO port K Bit 9 GPIOJRST : IO port J reset This bit is set and cleared by software. 0: does not reset IO port J 1: resets IO port J Bit 8 GPIOIRST: IO port I reset This bit is set and cleared by software. 0: does not reset IO port I 1: resets IO port I Bit 7 GPIOHRST: IO port H reset This bit is set and cleared by software. 0: does not reset IO port H 1: resets IO port H Bit 6 GPIOGRST: IO port G reset This bit is set and cleared by software. 0: does not reset IO port G 1: resets IO port G

RM0090 Rev 18 171/1749

212

--- Page 172 ---

**Reset and clock control for   STM32F42xxx and STM32F43xxx (RCC) RM0090**

Bit 5 GPIOFRST: IO port F reset

This bit is set and cleared by software.

0: does not reset IO port F

1: resets IO port F

Bit 4 GPIOERST: IO port E reset

This bit is set and cleared by software.

0: does not reset IO port E

1: resets IO port E

Bit 3 GPIODRST: IO port D reset

This bit is set and cleared by software.

0: does not reset IO port D

1: resets IO port D

Bit 2 GPIOCRST: IO port C reset

This bit is set and cleared by software.

0: does not reset IO port C

1: resets IO port C

Bit 1 GPIOBRST: IO port B reset

This bit is set and cleared by software.

0: does not reset IO port B

1:resets IO port B

Bit 0 GPIOARST: IO port A reset

This bit is set and cleared by software.

0: does not reset IO port A

1: resets IO port A

172/1749 RM0090 Rev 18

--- Page 173 ---

**RM0090**

**Reset and clock control for   STM32F42xxx and STM32F43xxx (RCC)**

**6.3.6 RCC AHB2 peripheral reset register (RCC_AHB2RSTR)**

Address offset: 0x14

Reset value: 0x0000 0000

Access: no wait state, word, half-word and byte access

31 30 29 28 27 26 25 24

23 22 21 20 19 18 17 16 Reserved

15 14 13 12 11 10 9 8

7 6 5 4 3 2 1 0

| Reserved | OTGFS RST | RNG RST | HASH RST | CRYP RST | Reserved | DCMI RST |
|---|---|---|---|---|---|---|
| rw |  |  |  |  |  |  |


Bits 31:8 Reserved, must be kept at reset value.

Bit 7 OTGFSRST: USB OTG FS module reset

Set and cleared by software.

0: does not reset the USB OTG FS module

1: resets the USB OTG FS module

Bit 6 RNGRST: Random number generator module reset Set and cleared by software.

0: does not reset the random number generator module 1: resets the random number generator module Bit 5 HASHRST: Hash module reset

Set and cleared by software.

0: does not reset the HASH module

1: resets the HASH module

Bit 4 CRYPRST: Cryptographic module reset

Set and cleared by software.

0: does not reset the cryptographic module

1: resets the cryptographic module

Bits 3:1 Reserved, must be kept at reset value.

Bit 0 DCMIRST: Camera interface reset

Set and cleared by software.

0: does not reset the Camera interface

1: resets the Camera interface



212

--- Page 174 ---

**Reset and clock control for   STM32F42xxx and STM32F43xxx (RCC)**

**RM0090**

**6.3.7 RCC AHB3 peripheral reset register (RCC_AHB3RSTR)**

Address offset: 0x18

Reset value: 0x0000 0000

Access: no wait state, word, half-word and byte access.

31 30 29 28 27 26 25 24

23 22 21 20 19 18 17 16

Reserved

15 14 13 12 11 10 9 8

7 6 5 4 3 2 1 0

| Reserved | FMCRST |
|---|---|
| rw |  |


Bits 31:1 Reserved, must be kept at reset value.

Bit 0 FMCRST: Flexible memory controller module reset

Set and cleared by software.

0: does not reset the FMC module

1: resets the FMC module


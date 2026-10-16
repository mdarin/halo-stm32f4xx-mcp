## 28.5 SPI and I²S registers

The peripheral registers have to be accessed by half-words (16 bits) or words (32 bits).

### 28.5.1 SPI control register 1 (SPI_CR1) (not used in I²S mode)

Address offset: 0x00
Reset value: 0x0000

| 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| BIDI MODE | BIDI OE | CRC EN | CRC NEXT | DFF | RX ONLY | SSM | SSI | LSB FIRST | SPE | BR [2:0] | MSTR | CPOL | CPHA |  |  |
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


**Bit 15 BIDIMODE**: Bidirectional data mode enable
0: 2-line unidirectional data mode selected
1: 1-line bidirectional data mode selected
*Note: This bit is not used in I²S mode*

**Bit 14 BIDIOE**: Output enable in bidirectional mode
This bit combined with the BIDImode bit selects the direction of transfer in bidirectional mode
0: Output disabled (receive-only mode)
1: Output enabled (transmit-only mode)
*Note: This bit is not used in I²S mode.*
In master mode, the MOSI pin is used while the MISO pin is used in slave mode.

**Bit 13 CRCEN**: Hardware CRC calculation enable
0: CRC calculation disabled
1: CRC calculation enabled
*Note: This bit should be written only when SPI is disabled (SPE = '0') for correct operation. It is not used in I²S mode.*

**Bit 12 CRCNEXT**: CRC transfer next
0: Data phase (no CRC phase)
1: Next transfer is CRC (CRC phase)
*Note: When the SPI is configured in full duplex or transmitter only modes, CRCNEXT must be written as soon as the last data is written to the SPI_DR register. When the SPI is configured in receiver only mode, CRCNEXT must be set after the second last data reception. This bit should be kept cleared when the transfers are managed by DMA. It is not used in I²S mode.*

**Bit 11 DFF**: Data frame format
0: 8-bit data frame format is selected for transmission/reception
1: 16-bit data frame format is selected for transmission/reception
*Note: This bit should be written only when SPI is disabled (SPE = '0') for correct operation. It is not used in I²S mode.*

916/1749
RM0090 Rev 18
ST

--- Page 137 ---

RM0090
Serial peripheral interface (SPI)

**Bit 10 RXONLY**: Receive only
This bit combined with the BIDImode bit selects the direction of transfer in 2-line unidirectional mode. This bit is also useful in a multislave system in which this particular slave is not accessed, the output from the accessed slave is not corrupted.
0: Full duplex (Transmit and receive)
1: Output disabled (Receive-only mode)
*Note: This bit is not used in I²S mode*

**Bit 9 SSM**: Software slave management
When the SSM bit is set, the NSS pin input is replaced with the value from the SSI bit.
0: Software slave management disabled
1: Software slave management enabled
*Note: This bit is not used in I²S mode and SPI TI mode*

**Bit 8 SSI**: Internal slave select
This bit has an effect only when the SSM bit is set. The value of this bit is forced onto the NSS pin and the IO value of the NSS pin is ignored.
*Note: This bit is not used in I²S mode and SPI TI mode*

**Bit 7 LSBFIRST**: Frame format
0: MSB transmitted first
1: LSB transmitted first
*Note: This bit should not be changed when communication is ongoing.*
*It is not used in I²S mode and SPI TI mode*

**Bit 6 SPE**: SPI enable
0: Peripheral disabled
1: Peripheral enabled
*Note: This bit is not used in I²S mode.*
*When disabling the SPI, follow the procedure described in Section 28.3.8.*

**Bits 5:3 BR[2:0]**: Baud rate control
000: fPCLK/2
001: fPCLK/4
010: fPCLK/8
011: fPCLK/16
100: fPCLK/32
101: fPCLK/64
110: fPCLK/128
111: fPCLK/256
*Note: These bits should not be changed when communication is ongoing.*
*They are not used in I²S mode.*

**Bit 2 MSTR**: Master selection
0: Slave configuration
1: Master configuration
*Note: This bit should not be changed when communication is ongoing.*
*It is not used in I²S mode.*

ST
RM0090 Rev 18
917/1749

--- Page 138 ---

**Serial peripheral interface (SPI)**
RM0090

Bit1 **CPOL**: Clock polarity
0: CK to 0 when idle
1: CK to 1 when idle
*Note: This bit should not be changed when communication is ongoing. It is not used in I²S mode and SPI TI mode.*

Bit 0 **CPHA**: Clock phase
0: The first clock transition is the first data capture edge
1: The second clock transition is the first data capture edge
*Note: This bit should not be changed when communication is ongoing. It is not used in I²S mode and SPI TI mode.*

### 28.5.2 SPI control register 2 (SPI_CR2)

Address offset: 0x04
Reset value: 0x0000

| 15 14 13 12 11 10 9 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |  |  |
|---|---|---|---|---|---|---|---|---|---|---|
| Reserved | TXEIE | RXNEIE | ERRIE | FRF | Res. | SSOE | TXDMAEN | RXDMAEN |  |  |
|  |  |  |  |  |  |  |  | rw |  | rw |


Bits 15:8 Reserved, must be kept at reset value.

Bit 7 **TXEIE**: Tx buffer empty interrupt enable
0: TXE interrupt masked
1: TXE interrupt not masked. Used to generate an interrupt request when the TXE flag is set.

Bit 6 **RXNEIE**: RX buffer not empty interrupt enable
0: RXNE interrupt masked
1: RXNE interrupt not masked. Used to generate an interrupt request when the RXNE flag is set.

Bit 5 **ERRIE**: Error interrupt enable
This bit controls the generation of an interrupt when an error condition occurs )(CRCERR, OVR, MODF in SPI mode, FRE in TI mode and UDR, OVR, and FRE in I²S mode).
0: Error interrupt is masked
1: Error interrupt is enabled

Bit 4 **FRF**: Frame format
0: SPI Motorola mode
1 SPI TI mode
*Note: This bit is not used in I²S mode.*

Bit 3 Reserved. Forced to 0 by hardware.

Bit 2 **SSOE**: SS output enable
0: SS output is disabled in master mode and the cell can work in multimaster configuration
1: SS output is enabled in master mode and when the cell is enabled. The cell cannot work in a multimaster environment.
*Note: This bit is not used in I²S mode and SPI TI mode.*

918/1749
RM0090 Rev 18
STI

--- Page 139 ---

RM0090
Serial peripheral interface (SPI)

Bit 1 **TXDMAEN**: Tx buffer DMA enable
When this bit is set, the DMA request is made whenever the TXE flag is set.
0: Tx buffer DMA disabled
1: Tx buffer DMA enabled

Bit 0 **RXDMAEN**: Rx buffer DMA enable
When this bit is set, the DMA request is made whenever the RXNE flag is set.
0: Rx buffer DMA disabled
1: Rx buffer DMA enabled

### 28.5.3 SPI status register (SPI_SR)

Address offset: 0x08
Reset value: 0x0002

| 15 14 13 12 11 10 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|
| Reserved | FRE | BSY | OVR | MODF | CRC ERR | UDR | CHSIDE | TXE | RXNE |
| r | rc_w0 | r |  |  |  |  |  |  |  |


Bits 15:9 Reserved. Forced to 0 by hardware.

Bit 8 **FRE**: Frame format error
0: No frame format error
1: A frame format error occurred
This flag is set by hardware and cleared by software when the SPIx_SR register is read.
Note: This flag is used when the SPI operates in TI slave mode or I2S slave mode (refer to Section 28.3.10).

Bit 7 **BSY**: Busy flag
0: SPI (or I2S) not busy
1: SPI (or I2S) is busy in communication or Tx buffer is not empty
This flag is set and cleared by hardware.
Note: BSY flag must be used with caution: refer to Section 28.3.7 and Section 28.3.8.

Bit 6 **OVR**: Overrun flag
0: No overrun occurred
1: Overrun occurred
This flag is set by hardware and reset by a software sequence. Refer to Section 28.4.8: Error flags for the software sequence.

Bit 5 **MODF**: Mode fault
0: No mode fault occurred
1: Mode fault occurred
This flag is set by hardware and reset by a software sequence. Refer to Section 28.4.8: Error flags for the software sequence.
Note: This bit is not used in I²S mode

Bit 4 **CRCErr**: CRC error flag
0: CRC value received matches the SPI_RXCRCR value
1: CRC value received does not match the SPI_RXCRCR value
This flag is set by hardware and cleared by software writing 0.
Note: This bit is not used in I²S mode.

ST
RM0090 Rev 18
919/1749

--- Page 140 ---

**Serial peripheral interface (SPI)**
RM0090

Bit 3 **UDR**: Underrun flag
0: No underrun occurred
1: Underrun occurred
This flag is set by hardware and reset by a software sequence. Refer to Section 28.4.8: *Error flags* for the software sequence.
Note: This bit is not used in SPI mode.

Bit 2 **CHSIDE**: Channel side
0: Channel Left has to be transmitted or has been received
1: Channel Right has to be transmitted or has been received
Note: This bit is not used for SPI mode and is meaningless in PCM mode.

Bit 1 **TXE**: Transmit buffer empty
0: Tx buffer not empty
1: Tx buffer empty

Bit 0 **RXNE**: Receive buffer not empty
0: Rx buffer empty
1: Rx buffer not empty

### 28.5.4 SPI data register (SPI_DR)

Address offset: 0x0C
Reset value: 0x0000

| 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| DR[15:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bits 15:0 **DR[15:0]**: Data register
Data received or to be transmitted.
The data register is split into 2 buffers - one for writing (Transmit Buffer) and another one for reading (Receive buffer). A write to the data register will write into the Tx buffer and a read from the data register will return the value held in the Rx buffer.
Note: These notes apply to SPI mode:
Depending on the data frame format selection bit (DFF in SPI_CR1 register), the data sent or received is either 8-bit or 16-bit. This selection has to be made before enabling the SPI to ensure correct operation.
For an 8-bit data frame, the buffers are 8-bit and only the LSB of the register (SPI_DR[7:0]) is used for transmission/reception. When in reception mode, the MSB of the register (SPI_DR[15:8]) is forced to 0.
For a 16-bit data frame, the buffers are 16-bit and the entire register, SPI_DR[15:0] is used for transmission/reception.

920/1749
RM0090 Rev 18
ST

--- Page 141 ---

RM0090
Serial peripheral interface (SPI)

### 28.5.5 SPI CRC polynomial register (SPI_CRCPR) (not used in I²S mode)

Address offset: 0x10
Reset value: 0x0007

| 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| CRCPOLY[15:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bits 15:0 **CRCPOLY[15:0]**: CRC polynomial register
This register contains the polynomial for the CRC calculation.
The CRC polynomial (0007h) is the reset value of this register. Another polynomial can be configured as required.
*Note: These bits are not used for the I²S mode.*

### 28.5.6 SPI RX CRC register (SPI_RXCRCR) (not used in I²S mode)

Address offset: 0x14
Reset value: 0x0000

| 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| RXCRC[15:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| r |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bits 15:0 **RXCRC[15:0]**: Rx CRC register
When CRC calculation is enabled, the RxCRC[15:0] bits contain the computed CRC value of the subsequently received bytes. This register is reset when the CRCEN bit in SPI_CR1 register is written to 1. The CRC is calculated serially using the polynomial programmed in the SPI_CRCPR register.
Only the 8 LSB bits are considered when the data frame format is set to be 8-bit data (DFF bit of SPI_CR1 is cleared). CRC calculation is done based on any CRC8 standard.
The entire 16-bits of this register are considered when a 16-bit data frame format is selected (DFF bit of the SPI_CR1 register is set). CRC calculation is done based on any CRC16 standard.
*Note: A read to this register when the BSY Flag is set could return an incorrect value. These bits are not used for I²S mode.*

ST
RM0090 Rev 18
921/1749

--- Page 142 ---

**Serial peripheral interface (SPI)**
RM0090

### 28.5.7 SPI TX CRC register (SPI_TXCRCR) (not used in I²S mode)

Address offset: 0x18
Reset value: 0x0000

| 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| TXCRC[15:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| r |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bits 15:0 **TXCRC[15:0]**: Tx CRC register

When CRC calculation is enabled, the TxCRC[7:0] bits contain the computed CRC value of the subsequently transmitted bytes. This register is reset when the CRCEN bit of SPI_CR1 is written to 1. The CRC is calculated serially using the polynomial programmed in the SPI_CRCPR register.

Only the 8 LSB bits are considered when the data frame format is set to be 8-bit data (DFF bit of SPI_CR1 is cleared). CRC calculation is done based on any CRC8 standard.

The entire 16-bits of this register are considered when a 16-bit data frame format is selected (DFF bit of the SPI_CR1 register is set). CRC calculation is done based on any CRC16 standard.

*Note: A read to this register when the BSY flag is set could return an incorrect value. These bits are not used for I²S mode.*

### 28.5.8 SPI_I²S configuration register (SPI_I2SCFGR)

Address offset: 0x1C
Reset value: 0x0000

| 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved | I2SMOD | I2SE | I2SCFG | PCMSYNC | Res. | I2SSTD | CKPOL | DATLEN | CHLEN |  |  |  |  |  |  |
| rw | rW | rw |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bits 15:12 Reserved, must be kept at reset value.

Bit 11 **I2SMOD**: I2S mode selection
0: SPI mode is selected
1: I2S mode is selected
*Note: This bit should be configured when the SPI or I²S is disabled*

Bit 10 **I2SE**: I2S Enable
0: I²S peripheral is disabled
1: I²S peripheral is enabled
*Note: This bit is not used in SPI mode.*

Bits 9:8 **I2SCFG**: I2S configuration mode
00: Slave - transmit
01: Slave - receive
10: Master - transmit
11: Master - receive
*Note: This bit should be configured when the I²S is disabled. It is not used in SPI mode.*

922/1749
RM0090 Rev 18
STI

--- Page 143 ---

RM0090
Serial peripheral interface (SPI)

Bit 7 **PCMSYNC**: PCM frame synchronization
0: Short frame synchronization
1: Long frame synchronization
*Note: This bit has a meaning only if I2SSTD = 11 (PCM standard is used)*
*It is not used in SPI mode.*

Bit 6 Reserved: forced at 0 by hardware

Bits 5:4 **I2SSTD**: I2S standard selection
00: I²S Philips standard.
01: MSB justified standard (left justified)
10: LSB justified standard (right justified)
11: PCM standard
For more details on I²S standards, refer to Section 28.4.3: Supported audio protocols. Not used in SPI mode.
*Note: For correct operation, these bits should be configured when the I²S is disabled.*

Bit 3 **CKPOL**: Steady state clock polarity
0: I²S clock steady state is low level
1: I²S clock steady state is high level
*Note: For correct operation, this bit should be configured when the I²S is disabled.*
*This bit is not used in SPI mode*

Bits 2:1 **DATLEN**: Data length to be transferred
00: 16-bit data length
01: 24-bit data length
10: 32-bit data length
11: Not allowed
*Note: For correct operation, these bits should be configured when the I²S is disabled.*
*This bit is not used in SPI mode.*

Bit 0 **CHLEN**: Channel length (number of bits per audio channel)
0: 16-bit wide
1: 32-bit wide
The bit write operation has a meaning only if DATLEN = 00 otherwise the channel length is fixed to 32-bit by hardware whatever the value filled in. Not used in SPI mode.
*Note: For correct operation, this bit should be configured when the I²S is disabled.*

### 28.5.9 SPI_I2S prescaler register (SPI_I2SPR)

Address offset: 0x20
Reset value: 0000 0010 (0x0002)

| 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0 |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|
| Reserved | MCKOE | ODD | I2SDIV |  |  |  |  |  |
|  |  |  |  |  |  |  |  | rw |


STI
RM0090 Rev 18
923/1749

--- Page 144 ---

**Serial peripheral interface (SPI)** RM0090

Bits 15:10 Reserved, must be kept at reset value.

Bit 9 **MCKOE**: Master clock output enable
0: Master clock output is disabled
1: Master clock output is enabled
*Note: This bit should be configured when the I²S is disabled. It is used only when the I²S is in master mode.*
*This bit is not used in SPI mode.*

Bit 8 **ODD**: Odd factor for the prescaler
0: real divider value is = I2SDIV *2
1: real divider value is = (I2SDIV * 2)+1
Refer to Section 28.4.4: *Clock generator*. Not used in SPI mode.
*Note: This bit should be configured when the I²S is disabled. It is used only when the I²S is in master mode.*

Bits 7:0 **I2SDIV**: I2S Linear prescaler
I2SDIV [7:0] = 0 or I2SDIV [7:0] = 1 are forbidden values.
Refer to Section 28.4.4: *Clock generator*. Not used in SPI mode.
*Note: These bits should be configured when the I²S is disabled. It is used only when the I²S is in master mode.*

924/1749 RM0090 Rev 18

--- Page 145 ---

RM0090
Serial peripheral interface (SPI)

28.5.10
**SPI register map**

The table provides shows the SPI register map and reset values.

Table 129. SPI register map and reset values

Refer to Section 2.3: Memory map for the register boundary addresses.

| Offset | Register | 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 0x00 | SPL_CR1 | Reserved | BIDIMODE | BIDIOE | CRXEN | CRXNEXT | DFF | RXONLY | SSM | LSBFRST | SPE | BR [2:0] | MSTR | CPOL | CPHA |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x04 | SPL_CR2 | Reserved | TXEIE | RXNEIE | ERIE | FRF | SSOE | TXDMAEN | RXDMAEN |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x08 | SPL_SR | Reserved | FRE | BSY | OVR | MODF | CRCEERR | UDR | CHSIDE | TXE | RXNE |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x0C | SPL_DR | Reserved | DR [15:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x10 | SPL_CRCPR | Reserved | CRCPOLY [15:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x14 | SPL_RXCRCR | Reserved | RxCRC [15:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x18 | SPL_TXCRCR | Reserved | TxCRC [15:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x1C | SPL_I2SCFGR | Reserved | I2SMOD | I2SE | I2SCFG | PCMSYNC | Reserved | I2SSTD | CKPOL | DATLEN | CHLEN |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x20 | SPL_I2SPR | Reserved | MCKOE | ODD | I2SDIV |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 0 | 0 0 0 0 0 0 0 1 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


RM0090 Rev 18
925/1749

--- Page 146 ---

Serial audio interface (SAI)
RM0090


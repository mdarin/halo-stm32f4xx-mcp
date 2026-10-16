## 28.2 SPI and I²S main features

### 28.2.1 SPI features

* Full-duplex synchronous transfers on three lines
* Simplex synchronous transfers on two lines with or without a bidirectional data line
* 8- or 16-bit transfer frame format selection
* Master or slave operation
* Multimaster mode capability
* 8 master mode baud rate prescalers (fPCLK/2 max.)
* Slave mode frequency (fPCLK/2 max)
* Faster communication for both master and slave
* NSS management by hardware or software for both master and slave: dynamic change of master/slave operations
* Programmable clock polarity and phase
* Programmable data order with MSB-first or LSB-first shifting
* Dedicated transmission and reception flags with interrupt capability
* SPI bus busy status flag
* SPI TI mode
* Hardware CRC feature for reliable communication:
    * CRC value can be transmitted as last byte in Tx mode
    * Automatic CRC error checking for last received byte
* Master mode fault, overrun and CRC error flags with interrupt capability
* 1-byte transmission and reception buffer with DMA capability: Tx and Rx requests

874/1749
RM0090 Rev 18
ST

--- Page 95 ---

RM0090
Serial peripheral interface (SPI)

### 28.2.2 I²S features

*   Full duplex communication
*   Half-duplex communication (only transmitter or receiver)
*   Master or slave operations
*   8-bit programmable linear prescaler to reach accurate audio sample frequencies (from 8 kHz to 192 kHz)
*   Data format may be 16-bit, 24-bit or 32-bit
*   Packet frame is fixed to 16-bit (16-bit data frame) or 32-bit (16-bit, 24-bit, 32-bit data frame) by audio channel
*   Programmable clock polarity (steady state)
*   Underrun flag in slave transmission mode, overrun flag in reception mode (master and slave), and Frame Error flag in reception and transmission mode (slave only)
*   16-bit register for transmission and reception with one data register for both channel sides
*   Supported I²S protocols:
    *   I²S Philips standard
    *   MSB-justified standard (left-justified)
    *   LSB-justified standard (right-justified)
    *   PCM standard (with short and long frame synchronization on 16-bit channel frame or 16-bit data frame extended to 32-bit channel frame)
*   Data direction is always MSB first
*   DMA capability for transmission and reception (16-bit wide)
*   Master clock may be output to drive an external audio component. Ratio is fixed at 256 × Fₛ (where Fₛ is the audio sampling frequency)
*   Both I²S (I2S2 and I2S3) have a dedicated PLL (PLL I2S) to generate an even more accurate clock.
*   I²S (I2S2 and I2S3) clock can be derived from an external clock mapped on the I2S_CKIN pin.

ST
RM0090 Rev 18
875/1749

--- Page 96 ---

Serial peripheral interface (SPI)
RM0090


> [RM0090 ch.28 §28.3] SPI functional description


#### Overrun condition

An overrun condition occurs when the master device has sent data bytes and the slave device has not cleared the RXNE bit resulting from the previous data byte transmitted. When an overrun condition occurs:

*   the OVR bit is set and an interrupt is generated if the ERRIE bit is set.

In this case, the receiver buffer contents will not be updated with the newly received data from the master device. A read from the SPI_DR register returns this byte. All other subsequently transmitted bytes are lost.

Clearing the OVR bit is done by a read from the SPI_DR register followed by a read access to the SPI_SR register.

ST
RM0090 Rev 18
897/1749

--- Page 118 ---

Serial peripheral interface (SPI)
RM0090

**CRC error**
This flag is used to verify the validity of the value received when the CRCEN bit in the SPI_CR1 register is set. The CRCERR flag in the SPI_SR register is set if the value received in the shift register does not match the receiver SPI_RXCRCR value.

**TI mode frame format error**
A TI mode frame format error is detected when an NSS pulse occurs during an ongoing communication when the SPI is acting in slave mode and configured to conform to the TI mode protocol. When this error occurs, the FRE flag is set in the SPI_SR register. The SPI is not disabled when an error occurs, the NSS pulse is ignored, and the SPI waits for the next NSS pulse before starting a new transfer. The data may be corrupted since the error detection may result in the lost of two data bytes.

The FRE flag is cleared when SPI_SR register is read. If the bit ERRIE is set, an interrupt is generated on the NSS error detection. In this case, the SPI should be disabled because data consistency is no more guaranteed and communications should be reinitiated by the master when the slave SPI is re-enabled.

Figure 261. TI mode frame format error detection
NSS output
SCK output
MOSI input
MISO output
TIFRFE
ai18438

> [RM0090 ch.28 §28.3] SPI functional description

### 28.3.11 SPI interrupts**

**28.3.11 SPI interrupts**

Table 126. SPI interrupt requests
| Interrupt event | Event flag | Enable Control bit |
|---|---|---|
| Transmit buffer empty flag | TXE | TXEIE |
| Receive buffer not empty flag | RXNE | RXNEIE |
| Master Mode fault event | MODF | ERRIE |
| Overrun error | OVR |  |
| CRC error flag | CRCERR |  |
| TI frame format error | FRE | ERRIE |


898/1749
RM0090 Rev 18
ST

--- Page 119 ---

RM0090
Serial peripheral interface (SPI)


> [RM0090 ch.28 §28.4] I²S functional description

### 28.4.8 Error flags**

**28.4.8 Error flags**
There are three error flags for the I²S cell.

**Underrun flag (UDR)**
In slave transmission mode this flag is set when the first clock for data transmission appears while the software has not yet loaded any value into SPI_DR. It is available when the I2SMOD bit in SPI_I2SCFGR is set. An interrupt may be generated if the ERRIE bit in SPI_CR2 is set.
The UDR bit is cleared by a read operation on the SPI_SR register.

**Overrun flag (OVR)**
This flag is set when data are received and the previous data have not yet been read from SPI_DR. As a result, the incoming data are lost. An interrupt may be generated if the ERRIE bit is set in SPI_CR2.

In this case, the receive buffer contents are not updated with the newly received data from the transmitter device. A read operation to the SPI_DR register returns the previous correctly received data. All other subsequently transmitted half-words are lost.

Clearing the OVR bit is done by a read operation on the SPI_DR register followed by a read access to the SPI_SR register.

**Frame error flag (FRE)**
This flag can be set by hardware only if the I2S is configured in Slave mode. It is set if the external master is changing the WS line at a moment when the slave is not expected this

914/1749
RM0090 Rev 18
ST

--- Page 135 ---

RM0090
Serial peripheral interface (SPI)

change. If the synchronization is lost, to recover from this state and resynchronize the external master device with the I2S slave device, follow the steps below:
1. Disable the I2S
2. Re-enable it when the correct level is detected on the WS line (WS line is high in I2S mode, or low for MSB- or LSB-justified or PCM modes).

Desynchronization between the master and slave device may be due to noisy environment on the SCK communication clock or on the WS frame synchronization line. An error interrupt can be generated if the ERRIE bit is set. The desynchronization flag (FRE) is cleared by software when the status register is read.

> [RM0090 ch.28 §28.4] I²S functional description

### 28.4.9 I²S interrupts

### 28.4.9 I²S interrupts

*Table 128* provides the list of I²S interrupts.

**Table 128. I²S interrupt requests**

| Interrupt event | Event flag | Enable Control bit |
|---|---|---|
| Transmit buffer empty flag | TXE | TXEIE |
| Receive buffer not empty flag | RXNE | RXNEIE |
| Overrun error | OVR | ERRIE |
| Underrun error | UDR |  |
| Frame error flag | FRE | ERRIE |


> [RM0090 ch.28 §28.4] I²S functional description

### 28.4.10 DMA features

### 28.4.10 DMA features

DMA is working in exactly the same way as for the SPI mode. There is no difference on the I²S. Only the CRC feature is not available in I²S mode since there is no data transfer protection system.

ST
RM0090 Rev 18
915/1749

--- Page 136 ---

Serial peripheral interface (SPI)
RM0090


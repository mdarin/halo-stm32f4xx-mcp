> [RM0090 ch.28 §28.3] SPI functional description


### 28.3.9 SPI communication using DMA (direct memory addressing)

To operate at its maximum speed, the SPI needs to be fed with the data for transmission and the data received on the Rx buffer should be read to avoid overrun. To facilitate the transfers, the SPI features a DMA capability implementing a simple request/acknowledge protocol.

A DMA access is requested when the enable bit in the SPI_CR2 register is enabled. Separate requests must be issued to the Tx and Rx buffers (see Figure 259 and Figure 260):

* In transmission, a DMA request is issued each time TXE is set to 1. The DMA then writes to the SPI_DR register (this clears the TXE flag).
* In reception, a DMA request is issued each time RXNE is set to 1. The DMA then reads the SPI_DR register (this clears the RXNE flag).

When the SPI is used only to transmit data, it is possible to enable only the SPI Tx DMA channel. In this case, the OVR flag is set because the data received are not read.

When the SPI is used only to receive data, it is possible to enable only the SPI Rx DMA channel.

In transmission mode, when the DMA has written all the data to be transmitted (flag TCIF is set in the DMA_ISR register), the BSY flag can be monitored to ensure that the SPI communication is complete. This is required to avoid corrupting the last transmission before disabling the SPI or entering the Stop mode. The software must first wait until TXE=1 and then until BSY=0.

**Note:** During discontinuous communications, there is a 2 APB clock period delay between the write operation to SPI_DR and the BSY bit setting. As a consequence, it is mandatory to wait first until TXE=1 and then until BSY=0 after writing the last data.

ST
RM0090 Rev 18
895/1749

--- Page 116 ---

**Serial peripheral interface (SPI)**
RM0090

**Figure 259. Transmission using DMA**

Example with CPOL=1, CPHA=1

SCK
MISO/MOSI (out)
DATA 1 = 0xF1
DATA 2 = 0xF2
DATA 3 = 0xF3
b0 b1 b2 b3 b4 b5 b6 b7 b0 b1 b2 b3 b4 b5 b6 b7 b0 b1 b2 b3 b4 b5 b6 b7

TXE flag
set by hardware
cleared by DMA write
set by hardware
clear by DMA write
set by hardware

BSY flag
set by hardware
reset by hardware

DMA request
ignored by the DMA because DMA transfer is complete

Tx buffer (write to SPI_DR)
0xF1
0xF2
0xF3

DMA writes to SPI_DR

DMA TCIF flag (DMA transfer complete)
set by hardware
clear by software

software configures the DMA SPI Tx channel to send 3 data items and enables the SPI
DMA writes DATA1 into SPI_DR
DMA writes DATA2 into SPI_DR
DMA writes DATA3 into SPI_DR
DMA transfer is complete (TCIF=1 in DMA_ISR)
software waits until TXE=1
software waits until BSY=0

ai17349

**Figure 260. Reception using DMA**

Example with CPOL=1, CPHA=1

SCK
MISO/MOSI (in)
DATA 1 = 0xA1
DATA 2 = 0xA2
DATA 3 = 0xA3
b0 b1 b2 b3 b4 b5 b6 b7 b0 b1 b2 b3 b4 b5 b6 b7 b0 b1 b2 b3 b4 b5 b6 b7

RXNE flag
set by hardware
clear by DMA read

DMA request

Rx buffer (read from SPI_DR)
0xA1
0xA2
0xA3

DMA read from SPI_DR

flag DMA TCIF (DMA transfer complete)
set by hardware
clear by software

software configures the DMA SPI Rx channel to receive 3 data items and enables the SPI
DMA reads DATA1 from SPI_DR
DMA reads DATA2 from SPI_DR
DMA reads DATA3 from SPI_DR
The DMA transfer is complete (TCIF=1 in DMA_ISR)

ai17350

896/1749
RM0090 Rev 18

--- Page 117 ---

RM0090
Serial peripheral interface (SPI)

> [RM0090 ch.28 §28.3] SPI functional description

### DMA capability with CRC

### DMA capability with CRC

When SPI communication is enabled with CRC communication and DMA mode, the transmission and reception of the CRC at the end of communication are automatic that is without using the bit CRCNEXT. After the CRC reception, the CRC must be read in the SPI_DR register to clear the RXNE flag.

At the end of data and CRC transfers, the CRCERR flag in SPI_SR is set if corruption occurs during the transfer.

> [RM0090 ch.28 §28.3] SPI functional description

### 28.3.10 Error flags

### 28.3.10 Error flags

> [RM0090 ch.28 §28.3] SPI functional description

### Master mode fault (MODF)

#### Master mode fault (MODF)

Master mode fault occurs when the master device has its NSS pin pulled low (in NSS hardware mode) or SSI bit low (in NSS software mode), this automatically sets the MODF bit. Master mode fault affects the SPI peripheral in the following ways:

*   The MODF bit is set and an SPI interrupt is generated if the ERRIE bit is set.
*   The SPE bit is cleared. This blocks all output from the device and disables the SPI interface.
*   The MSTR bit is cleared, thus forcing the device into slave mode.

Use the following software sequence to clear the MODF bit:
1. Make a read or write access to the SPI_SR register while the MODF bit is set.
2. Then write to the SPI_CR1 register.

To avoid any multiple slave conflicts in a system comprising several MCUs, the NSS pin must be pulled high during the MODF bit clearing sequence. The SPE and MSTR bits can be restored to their original state after this clearing sequence.

As a security, hardware does not allow the setting of the SPE and MSTR bits while the MODF bit is set.

In a slave device the MODF bit cannot be set. However, in a multimaster configuration, the device can be in slave mode with this MODF bit set. In this case, the MODF bit indicates that there might have been a multimaster conflict for system control. An interrupt routine can be used to recover cleanly from this state by performing a reset or returning to a default state.


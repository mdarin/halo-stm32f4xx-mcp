> [RM0090 ch.28 §28.3] SPI functional description


### 28.3.1 General description

The block diagram of the SPI is shown in Figure 246.

**Figure 246. SPI block diagram**

Address and data bus
Read
Rx buffer
MOSI
MISO
Shift register
LSB first
Tx buffer
Write
SCK
NSS
Baud rate generator
Master control logic
SPI_CR2
TXE IE
RXNE IE
ERR IE
0
0
SSOE
TXDMAEN
RXDMAEN
SPI_SR
BSY
OVR
MODF
CRCERR
0
0
TXE
RXNE
Communication control
BR[2:0]
0
1
SPI_CR1
LSB FIRST
SPE
BR2
BR1
BR0
MSTR
CPOL
CPHA
BIDI MODE
BIDI OE
CRCEN
CRC Next
DFF
RX ONLY
SSM
SSI
MS51604V1

Usually, the SPI is connected to external devices through four pins:

*   MISO: Master In / Slave Out data. This pin can be used to transmit data in slave mode and receive data in master mode.
*   MOSI: Master Out / Slave In data. This pin can be used to transmit data in master mode and receive data in slave mode.
*   SCK: Serial Clock output for SPI masters and input for SPI slaves.
*   NSS: Slave select. This is an optional pin to select a slave device. This pin acts as a ‘chip select’ to let the SPI master communicate with slaves individually and to avoid contention on the data lines. Slave NSS inputs can be driven by standard IO ports on the master device. The NSS pin may also be used as an output if enabled (SSOE bit) and driven low if the SPI is in master configuration. In this manner, all NSS pins from devices connected to the Master NSS pin see a low level and become slaves when they are configured in NSS hardware mode. When configured in master mode with NSS configured as an input (MSTR=1 and SSOE=0) and if NSS is pulled low, the SPI enters the master mode fault state: the MSTR bit is automatically cleared and the device is configured in slave mode (refer to Section 28.3.10).

A basic example of interconnections between a single master and a single slave is illustrated in Figure 247.

876/1749
RM0090 Rev 18
ST

--- Page 97 ---

RM0090
Serial peripheral interface (SPI)

**Figure 247. Single master/ single slave application**

1. Here, the NSS pin is configured as an input.

The MOSI pins are connected together and the MISO pins are connected together. In this way data is transferred serially between master and slave (most significant bit first).

The communication is always initiated by the master. When the master device transmits data to a slave device via the MOSI pin, the slave device responds via the MISO pin. This implies full-duplex communication with both data out and data in synchronized with the same clock signal (which is provided by the master device via the SCK pin).

**Slave select (NSS) pin management**

Hardware or software slave select management can be set using the SSM bit in the SPI_CR1 register.

*   Software NSS management (SSM = 1)
    The slave select information is driven internally by the value of the SSI bit in the SPI_CR1 register. The external NSS pin remains free for other application uses.
*   Hardware NSS management (SSM = 0)
    Two configurations are possible depending on the NSS output configuration (SSOE bit in register SPI_CR2).
    *   NSS output enabled (SSM = 0, SSOE = 1)
        This configuration is used only when the device operates in master mode. The NSS signal is driven low when the master starts the communication and is kept low until the SPI is disabled.
    *   NSS output disabled (SSM = 0, SSOE = 0)
        This configuration allows multimaster capability for devices operating in master mode. For devices set as slave, the NSS pin acts as a classical NSS input: the slave is selected when NSS is low and deselected when NSS high.

ST
RM0090 Rev 18
877/1749

--- Page 98 ---

**Serial peripheral interface (SPI)**
RM0090

**Clock phase and clock polarity**

Four possible timing relationships may be chosen by software, using the CPOL and CPHA bits in the SPI_CR1 register. The CPOL (clock polarity) bit controls the steady state value of the clock when no data is being transferred. This bit affects both master and slave modes. If CPOL is reset, the SCK pin has a low-level idle state. If CPOL is set, the SCK pin has a high-level idle state.

If the CPHA (clock phase) bit is set, the second edge on the SCK pin (falling edge if the CPOL bit is reset, rising edge if the CPOL bit is set) is the MSBit capture strobe. Data are latched on the occurrence of the second clock transition. If the CPHA bit is reset, the first edge on the SCK pin (falling edge if CPOL bit is set, rising edge if CPOL bit is reset) is the MSBit capture strobe. Data are latched on the occurrence of the first clock transition.

The combination of the CPOL (clock polarity) and CPHA (clock phase) bits selects the data capture clock edge.

*Figure 248*, shows an SPI transfer with the four combinations of the CPHA and CPOL bits. The diagram may be interpreted as a master or slave timing diagram where the SCK pin, the MISO pin, the MOSI pin are directly connected between the master and the slave device.

*Note:*
*Prior to changing the CPOL/CPHA bits the SPI must be disabled by resetting the SPE bit.*
*Master and slave must be programmed with the same timing mode.*
*The idle state of SCK must correspond to the polarity selected in the SPI_CR1 register (by pulling up SCK if CPOL=1 or pulling down SCK if CPOL=0).*
*The Data Frame Format (8- or 16-bit) is selected through the DFF bit in SPI_CR1 register, and determines the data length during transmission/reception.*

878/1749
RM0090 Rev 18
ST

--- Page 99 ---

RM0090
Serial peripheral interface (SPI)

**Figure 248. Data clock timing diagram**

CPHA =1
CPOL = 1
CPOL = 0
MOSI
MSBit
LSBit
MISO
MSBit
LSBit
NSS
(to slave)
Capture strobe

CPHA =0
CPOL = 1
CPOL = 0
MOSI
MSBit
LSBit
MISO
MSBit
LSBit
NSS
(to slave)
Capture strobe

ai17154d

1. These timings are shown with the LSBFIRST bit reset in the SPI_CR1 register.

**Data frame format**

Data can be shifted out either MSB-first or LSB-first depending on the value of the LSBFIRST bit in the SPI_CR1 Register.

Each data frame is 8 or 16 bits long depending on the size of the data programmed using the DFF bit in the SPI_CR1 register. The selected data frame format is applicable for transmission and/or reception.

STI
RM0090 Rev 18
879/1749

--- Page 100 ---

**Serial peripheral interface (SPI)**
RM0090


> [RM0090 ch.28 §28.4] I²S functional description


### I²S Philips standard

For this standard, the WS signal is used to indicate which channel is being transmitted. It is activated one CK clock cycle before the first bit (MSB) is available.

ST
RM0090 Rev 18
901/1749

--- Page 122 ---

Serial peripheral interface (SPI)
RM0090

**Figure 264. I²S Philips protocol waveforms (16/32-bit full accuracy, CPOL = 0)**

CK
WS
SD
transmission
reception
Can be 16-bit or 32-bit
MSB
LSB
MSB
Channel left
Channel right
MS19591V1

Data are latched on the falling edge of CK (for the transmitter) and are read on the rising edge (for the receiver). The WS signal is also latched on the falling edge of CK.

**Figure 265. I²S Philips standard waveforms (24-bit frame with CPOL = 0)**

CK
WS
SD
Transmission
Reception
24-bit data
8-bit remaining 0 forced
MSB
LSB
Channel left 32-bit
Channel right
MS19592V1

This mode needs two write or read operations to/from the SPI_DR.

*   In transmission mode:
    if 0x8EAA33 has to be sent (24-bit):

**Figure 266. Transmitting 0x8EAA33**

First write to Data register
0x8EAA

Second write to Data register
0x33XX
Only the 8 MSB are sent to compare the 24 bits
8 LSBs have no meaning and can be anything
MS19593V1

*   In reception mode:
    if data 0x8EAA33 is received:

902/1749
RM0090 Rev 18
ST

--- Page 123 ---

RM0090
Serial peripheral interface (SPI)

**Figure 267. Receiving 0x8EAA33**

First read to Data register
0x8EAA

Second read to Data register
0x33XX
Only the 8 MSB are sent to compare the 24 bits
8 LSBs have no meaning and can be anything

MS19594V1

**Figure 268. I²S Philips standard (16-bit extended to 32-bit packet frame with CPOL = 0)**

CK
WS
SD
Transmission
Reception
16-bit data
16-bit remaining 0 forced
MSB
LSB
Channel left 32-bit
Channel right

MS19599V1

When 16-bit data frame extended to 32-bit channel frame is selected during the I²S configuration phase, only one access to SPI_DR is required. The 16 remaining bits are forced by hardware to 0x0000 to extend the data to 32-bit format.

If the data to transmit or the received data are 0x76A3 (0x76A30000 extended to 32-bit), the operation shown in Figure 269 is required.

**Figure 269. Example**

Only one access to SPIx_DR
0x76A3

MS19595V1

For transmission, each time an MSB is written to SPI_DR, the TXE flag is set and its interrupt, if allowed, is generated to load SPI_DR with the new value to send. This takes place even if 0x0000 have not yet been sent because it is done by hardware.

For reception, the RXNE flag is set and its interrupt, if allowed, is generated when the first 16 MSB half-word is received.

In this way, more time is provided between two write or read operations, which prevents underrun or overrun conditions (depending on the direction of the data transfer).

**MSB justified standard**

For this standard, the WS signal is generated at the same time as the first data bit, which is the MSBit.

ST
RM0090 Rev 18
903/1749

--- Page 124 ---

**Serial peripheral interface (SPI)**
RM0090

**Figure 270. MSB justified 16-bit or 32-bit full-accuracy length with CPOL = 0**

[Image: Timing diagram showing CK, WS, and SD signals. Labels include "Transmission", "Reception", "16- or 32 bit data", "MSB", "LSB", "Channel left", "Channel right".]
MS30100 V1

Data are latched on the falling edge of CK (for transmitter) and are read on the rising edge (for the receiver).

**Figure 271. MSB justified 24-bit frame length with CPOL = 0**

[Image: Timing diagram showing CK, WS, and SD signals. Labels include "Transmission", "Reception", "24 bit data", "MSB", "LSB", "8-bit remaining 0 forced", "Channel left 32-bit", "Channel right".]
MS30101V1

**Figure 272. MSB justified 16-bit extended to 32-bit packet frame with CPOL = 0**

[Image: Timing diagram showing CK, WS, and SD signals. Labels include "Transmission", "Reception", "16-bit data", "MSB", "LSB", "16-bit remaining 0 forced", "Channel left 32-bit", "Channel right".]
MS30102V1

**LSB justified standard**

This standard is similar to the MSB justified standard (no difference for the 16-bit and 32-bit full-accuracy frame formats).

904/1749
RM0090 Rev 18

--- Page 125 ---

RM0090
Serial peripheral interface (SPI)

Figure 273. LSB justified 16-bit or 32-bit full-accuracy with CPOL = 0

CK
WS
SD
Transmission
Reception
16- or 32-bit data
MSB
LSB
MSB
Channel left
Channel right
MS30103V1

Figure 274. LSB justified 24-bit frame length with CPOL = 0

CK
WS
SD
8-bit data
'0' forced
Transmission
Reception
24-bit remaining
MSB
LSB
Channel left 32-bit
Channel right
MS30104V1

*   In transmission mode:
    If data 0x3478AE have to be transmitted, two write operations to the SPI_DR register are required from software or by DMA. The operations are shown below.

Figure 275. Operations required to transmit 0x3478AE

First write to Data register conditioned by TXE=1
0xXX34
Only the 8 LSB of the half-word are significant.
A field of 0x00 is forced instead of the 8 MSBs.

Second write to Data register conditioned by TXE=1
0x78AE

MS19596V1

*   In reception mode:
    If data 0x3478AE are received, two successive read operations from SPI_DR are required on each RXNE event.

ST
RM0090 Rev 18
905/1749

--- Page 126 ---

**Serial peripheral interface (SPI)**
RM0090

**Figure 276. Operations required to receive 0x3478AE**

First read from Data register conditioned by RXNE=1
0xXX34

Second read from Data register conditioned by RXNE=1
0x78AE

Only the 8 LSB of the half-word are significant.
A field of 0x00 is forced instead of the 8 MSBs.

MS19597V1

**Figure 277. LSB justified 16-bit extended to 32-bit packet frame with CPOL = 0**

CK
WS
SD
Transmission
Reception
16-bit data 0 forced
16-bit remaining
MSB
LSB
Channel left 32-bit
Channel right

MS30105V1

When 16-bit data frame extended to 32-bit channel frame is selected during the I²S configuration phase, Only one access to SPI_DR is required. The 16 remaining bits are forced by hardware to 0x0000 to extend the data to 32-bit format. In this case it corresponds to the half-word MSB.

If the data to transmit or the received data are 0x76A3 (0x0000 76A3 extended to 32-bit), the operation shown in Figure 278 is required.

**Figure 278. Example of LSB justified 16-bit extended to 32-bit packet frame**

Only one access to the SPIx-DR register

0x76A3

MS19598V1

In transmission mode, when TXE is asserted, the application has to write the data to be transmitted (in this case 0x76A3). The 0x000 field is transmitted first (extension on 32-bit). TXE is asserted again as soon as the effective data (0x76A3) is sent on SD.

In reception mode, RXNE is asserted as soon as the significant half-word is received (and not the 0x0000 field).

In this way, more time is provided between two write or read operations to prevent underrun or overrun conditions.

906/1749
RM0090 Rev 18

--- Page 127 ---

RM0090
Serial peripheral interface (SPI)

**PCM standard**

For the PCM standard, there is no need to use channel-side information. The two PCM modes (short and long frame) are available and configurable using the PCMSYNC bit in SPI_I2SCFGR.

Figure 279. PCM standard waveforms (16-bit)

MS30106V1

For long frame synchronization, the WS signal assertion time is fixed 13 bits in master mode.
For short frame synchronization, the WS synchronization signal is only one cycle long.

Figure 280. PCM standard waveforms (16-bit extended to 32-bit packet frame)

MS30107V1

**Note:** For both modes (master and slave) and for both synchronizations (short and long), the number of bits between two consecutive pieces of data (and so two synchronization signals) needs to be specified (DATLEN and CHLEN bits in the SPI_I2SCFGR register) even in slave mode.


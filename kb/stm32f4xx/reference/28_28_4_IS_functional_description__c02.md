> [RM0090 ch.28 §28.4] I²S functional description


### 28.4.1 I²S general description

The block diagram of the I²S is shown in Figure 262.

**Figure 262. I²S block diagram**

Address and data bus
Tx buffer
16-bit
BSY OVR MODF CRCERR UDR CHSIDE TxE RxNE FRE
MOSI/SD
MISO/
I2S2ext_SD/
I2S3ext_SD(1)
Shift register
16-bit
LSB first
Rx buffer
Communication control
NSS/WS
I2SCFG [1:0] I2SSTD [1:0] CKPOL DATLEN [1:0] CHLEN
— — — — — I2SMOD I2SE
Master control logic
Bidi mode Bidi OE CRC EN CRC Next DFF Rx only SSM SSI
SPI baud rate generator
LSB First SPE BR2 BR1 BR0 MSTR CPOL CPHA
CK
I2S clock generator
I2S_CK
I2SMOD
MCK
MCKOE ODD I2SDIV[7:0]
I2SxCLK
MS19909V1

1. I2S2ext_SD and I2S3ext_SD are the extended SD pins that control the I²S full duplex mode.
The SPI could function as an audio I²S interface when the I²S capability is enabled (by setting the I2SMOD bit in the SPI_I2SCFGR register). This interface uses almost the same pins, flags and interrupts as the SPI.

ST
RM0090 Rev 18
899/1749

--- Page 120 ---

**Serial peripheral interface (SPI)**
RM0090

The I²S shares three common pins with the SPI:

*   SD: Serial Data (mapped on the MOSI pin) to transmit or receive the two time-multiplexed data channels (in half-duplex mode only).
*   WS: Word Select (mapped on the NSS pin) is the data control signal output in master mode and input in slave mode.
*   CK: Serial Clock (mapped on the SCK pin) is the serial clock output in master mode and serial clock input in slave mode.
*   I2S2ext_SD and I2S3ext_SD: additional pins (mapped on the MISO pin) to control the I²S full duplex mode.

An additional pin could be used when a master clock output is needed for some external audio devices:

*   MCK: Master Clock (mapped separately) is used, when the I²S is configured in master mode (and when the MCKOE bit in the SPI_I2SPR register is set), to output this additional clock generated at a preconfigured frequency rate equal to 256 × F_S, where F_S is the audio sampling frequency.

The I²S uses its own clock generator to produce the communication clock when it is set in master mode. This clock generator is also the source of the master clock output. Two additional registers are available in I²S mode. One is linked to the clock generator configuration SPI_I2SPR and the other one is a generic I²S configuration register SPI_I2SCFGR (audio standard, slave/master mode, data format, packet frame, clock polarity, etc.).

The SPI_CR1 register and all CRC registers are not used in the I²S mode. Likewise, the SSOE bit in the SPI_CR2 register and the MODF and CRCERR bits in the SPI_SR are not used.

The I²S uses the same SPI register for data transfer (SPI_DR) in 16-bit wide mode.

> [RM0090 ch.28 §28.4] I²S functional description

### 28.4.2 I2S full duplex**

**28.4.2 I2S full duplex**

To support I2S full duplex mode, two extra I²S instances called extended I2Ss (I2S2_ext, I2S3_ext) are available in addition to I2S2 and I2S3 (see Figure 263). The first I2S full-duplex interface is consequently based on I2S2 and I2S2_ext, and the second one on I2S3 and I2S3_ext.

*Note:* I2S2_ext an I2S3_ext are used only in full-duplex mode.

**Figure 263. I2S full duplex block diagram**

I2Sx_SCK
I2S_WS
SPI/I2Sx
SPIx_MOSI/I2Sx_SD(in/out)
I2Sx_ext
I2Sx_extSD(in/out)
MS19910V1

1. Where x can be 2 or 3.

900/1749
RM0090 Rev 18
ST

--- Page 121 ---

RM0090
Serial peripheral interface (SPI)

I2Sx can operate in master mode. As a result:
* Only I2Sx can output SCK and WS in half duplex mode
* Only I2Sx can deliver SCK and WS to I2S2_ext and I2S3_ext in full duplex mode.

The extended I2Ss (I2Sx_ext) can be used only in full duplex mode. The I2Sx_ext operate always in slave mode.

Both I2Sx and I2Sx_ext can be configured as transmitters or receivers.

> [RM0090 ch.28 §28.4] I²S functional description

### 28.4.3 Supported audio protocols

### 28.4.3 Supported audio protocols

The four-line bus has to handle only audio data generally time-multiplexed on two channels: the right channel and the left channel. However there is only one 16-bit register for the transmission and the reception. So, it is up to the software to write into the data register the adequate value corresponding to the considered channel side, or to read the data from the data register and to identify the corresponding channel by checking the CHSIDE bit in the SPI_SR register. Channel Left is always sent first followed by the channel right (CHSIDE has no meaning for the PCM protocol).

Four data and packet frames are available. Data may be sent with a format of:
* 16-bit data packed in 16-bit frame
* 16-bit data packed in 32-bit frame
* 24-bit data packed in 32-bit frame
* 32-bit data packed in 32-bit frame

When using 16-bit data extended on 32-bit packet, the first 16 bits (MSB) are the significant bits, the 16-bit LSB is forced to 0 without any need for software action or DMA request (only one read/write operation).

The 24-bit and 32-bit data frames need two CPU read or write operations to/from the SPI_DR or two DMA operations if the DMA is preferred for the application. For 24-bit data frame specifically, the 8 nonsignificant bits are extended to 32 bits with 0-bits (by hardware).

For all data formats and communication standards, the most significant bit is always sent first (MSB first).

The I²S interface supports four audio standards, configurable using the I2SSTD[1:0] and PCMSYNC bits in the SPI_I2SCFGR register.


## 28 Serial peripheral interface (SPI)

This section applies to the whole STM32F4xx family, unless otherwise specified.

### 28.1 SPI introduction

The SPI interface provides two main functions, supporting either the SPI protocol or the I²S audio protocol. By default, it is the SPI function that is selected. It is possible to switch the interface from SPI to I²S by software.

The serial peripheral interface (SPI) allows half/ full-duplex, synchronous, serial communication with external devices. The interface can be configured as the master and in this case it provides the communication clock (SCK) to the external slave device. The interface is also capable of operating in multimaster configuration.

It may be used for a variety of purposes, including simplex synchronous transfers on two lines with a possible bidirectional data line or reliable communication using CRC checking.

The I²S is also a synchronous serial communication interface. It can address four different audio standards including the I²S Philips standard, the MSB- and LSB-justified standards, and the PCM standard. It can operate as a slave or a master device in full-duplex mode (using 4 pins) or in half-duplex mode (using 3 pins). Master clock can be provided by the interface to an external slave component when the I²S is configured as the communication master.

**Warning:** Since some SPI1 and SPI3/I2S3 pins may be mapped onto some pins used by the JTAG interface (SPI1_NSS onto JTDI, SPI3_NSS/I2S3_WS onto JTDI and SPI3_SCK/I2S3_CK onto JTDO), you may either:
– map SPI/I2S onto other pins
– disable the JTAG and use the SWD interface prior to configuring the pins listed as SPI I/Os (when debugging the application) or
– disable both JTAG/SWD interfaces (for standalone applications).
For more information on the configuration of the JTAG/SWD interface pins, please refer to **Section 8.3.2: I/O pin multiplexer and mapping**.

ST
RM0090 Rev 18
873/1749

--- Page 94 ---

Serial peripheral interface (SPI)
RM0090


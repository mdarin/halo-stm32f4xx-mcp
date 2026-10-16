> [RM0090 ch.28 §28.3] SPI functional description


### 28.3.5 Data transmission and reception procedures

**Rx and Tx buffers**

In reception, data are received and then stored into an internal Rx buffer while In transmission, data are first stored into an internal Tx buffer before being transmitted.

A read access of the SPI_DR register returns the Rx buffered value whereas a write access to the SPI_DR stores the written data into the Tx buffer.

**Start sequence in master mode**

*   In full-duplex (BIDIMODE=0 and RXONLY=0)
    *   The sequence begins when data are written into the SPI_DR register (Tx buffer).
    *   The data are then parallel loaded from the Tx buffer into the 8-bit shift register during the first bit transmission and then shifted out serially to the MOSI pin.
    *   At the same time, the received data on the MISO pin is shifted in serially to the 8-bit shift register and then parallel loaded into the SPI_DR register (Rx buffer).
*   In unidirectional receive-only mode (BIDIMODE=0 and RXONLY=1)
    *   The sequence begins as soon as SPE=1
    *   Only the receiver is activated and the received data on the MISO pin are shifted in serially to the 8-bit shift register and then parallel loaded into the SPI_DR register (Rx buffer).
*   In bidirectional mode, when transmitting (BIDIMODE=1 and BIDIOE=1)
    *   The sequence begins when data are written into the SPI_DR register (Tx buffer).
    *   The data are then parallel loaded from the Tx buffer into the 8-bit shift register during the first bit transmission and then shifted out serially to the MOSI pin.
    *   No data are received.
*   In bidirectional mode, when receiving (BIDIMODE=1 and BIDIOE=0)
    *   The sequence begins as soon as SPE=1 and BIDIOE=0.
    *   The received data on the MOSI pin are shifted in serially to the 8-bit shift register and then parallel loaded into the SPI_DR register (Rx buffer).
    *   The transmitter is not activated and no data are shifted out serially to the MOSI pin.

**Start sequence in slave mode**

*   In full-duplex mode (BIDIMODE=0 and RXONLY=0)
    *   The sequence begins when the slave device receives the clock signal and the first bit of the data on its MOSI pin. The 7 remaining bits are loaded into the shift register.
    *   At the same time, the data are parallel loaded from the Tx buffer into the 8-bit shift register during the first bit transmission, and then shifted out serially to the MISO

RM0090 Rev 18
885/1749

--- Page 106 ---

**Serial peripheral interface (SPI)** RM0090

pin. The software must have written the data to be sent before the SPI master device initiates the transfer.

*   In unidirectional receive-only mode (BIDIMODE=0 and RXONLY=1)
    *   The sequence begins when the slave device receives the clock signal and the first bit of the data on its MOSI pin. The 7 remaining bits are loaded into the shift register.
    *   The transmitter is not activated and no data are shifted out serially to the MISO pin.
*   In bidirectional mode, when transmitting (BIDIMODE=1 and BIDIOE=1)
    *   The sequence begins when the slave device receives the clock signal and the first bit in the Tx buffer is transmitted on the MISO pin.
    *   The data are then parallel loaded from the Tx buffer into the 8-bit shift register during the first bit transmission and then shifted out serially to the MISO pin. The software must have written the data to be sent before the SPI master device initiates the transfer.
    *   No data are received.
*   In bidirectional mode, when receiving (BIDIMODE=1 and BIDIOE=0)
    *   The sequence begins when the slave device receives the clock signal and the first bit of the data on its MISO pin.
    *   The received data on the MISO pin are shifted in serially to the 8-bit shift register and then parallel loaded into the SPI_DR register (Rx buffer).
    *   The transmitter is not activated and no data are shifted out serially to the MISO pin.

**Handling data transmission and reception**

The TXE flag (Tx buffer empty) is set when the data are transferred from the Tx buffer to the shift register. It indicates that the internal Tx buffer is ready to be loaded with the next data. An interrupt can be generated if the TXEIE bit in the SPI_CR2 register is set. Clearing the TXE bit is performed by writing to the SPI_DR register.

*Note:* The software must ensure that the TXE flag is set to 1 before attempting to write to the Tx buffer. Otherwise, it overwrites the data previously written to the Tx buffer.

The RXNE flag (Rx buffer not empty) is set on the last sampling clock edge, when the data are transferred from the shift register to the Rx buffer. It indicates that data are ready to be read from the SPI_DR register. An interrupt can be generated if the RXNEIE bit in the SPI_CR2 register is set. Clearing the RXNE bit is performed by reading the SPI_DR register.

For some configurations, the BSY flag can be used during the last data transfer to wait until the completion of the transfer.

Full-duplex transmit and receive procedure in master or slave mode (BIDIMODE=0 and RXONLY=0)

The software has to follow this procedure to transmit and receive data (see Figure 253 and Figure 254):

886/1749 RM0090 Rev 18 ST

--- Page 107 ---

RM0090
Serial peripheral interface (SPI)

1. Enable the SPI by setting the SPE bit to 1.
2. Write the first data item to be transmitted into the SPI_DR register (this clears the TXE flag).
3. Wait until TXE=1 and write the second data item to be transmitted. Then wait until RXNE=1 and read the SPI_DR to get the first received data item (this clears the RXNE bit). Repeat this operation for each data item to be transmitted/received until the n–1 received data.
4. Wait until RXNE=1 and read the last received data.
5. Wait until TXE=1 and then wait until BSY=0 before disabling the SPI.

This procedure can also be implemented using dedicated interrupt subroutines launched at each rising edges of the RXNE or TXE flag.

Figure 253. TXE/RXNE/BSY behavior in Master / full-duplex mode (BIDIMODE=0 and RXONLY=0) in case of continuous transfers

Example in Master mode with CPOL=1, CPHA=1

SCK
MISO/MOSI (out)
DATA1 = 0xF1
DATA2 = 0xF2
DATA3 = 0xF3
b0 b1 b2 b3 b4 b5 b6 b7 b0 b1 b2 b3 b4 b5 b6 b7 b0 b1 b2 b3 b4 b5 b6 b7

TXE flag
set by hardware cleared by software
set by hardware cleared by software
set by hardware

Tx buffer (write SPI_DR)
0xF1
0xF2
0xF3

BSY flag
set by hardware
reset by hardware

MISO/MOSI (in)
DATA1 = 0xA1
DATA 2 = 0xA2
DATA 3 = 0xA3
b0 b1 b2 b3 b4 b5 b6 b7 b0 b1 b2 b3 b4 b5 b6 b7 b0 b1 b2 b3 b4 b5 b6 b7

RXNE flag
set by hardware cleared by software

Rx buffer (read SPI_DR)
0xA1
0xA2
0xA3

software writes 0xF1 into SPI_DR
software waits until TXE=1 and writes 0xF2 into SPI_DR
software waits until RXNE=1 and reads 0xA1 from SPI_DR
software waits until TXE=1 and writes 0xF3 into SPI_DR
software waits until RXNE=1 and reads 0xA2 from SPI_DR
software waits until RXNE=1 and reads 0xA3 from SPI_DR

ai17343

ST
RM0090 Rev 18
887/1749

--- Page 108 ---

**Serial peripheral interface (SPI)**
RM0090

**Figure 254. TXE/RXNE/BSY behavior in Slave / full-duplex mode (BIDIMODE=0, RXONLY=0) in case of continuous transfers**

Example in Slave mode with CPOL=1, CPHA=1

SCK
MISO/MOSI (out)
DATA 1 = 0xF1
DATA 2 = 0xF2
DATA 3 = 0xF3
b0 b1 b2 b3 b4 b5 b6 b7 b0 b1 b2 b3 b4 b5 b6 b7 b0 b1 b2 b3 b4 b5 b6 b7
TXE flag
set by hardware cleared by software
set by hardware cleared by software
set by hardware
Tx buffer (write to SPI_DR)
0xF1
0xF2
0xF3
BSY flag
set by cleared by software
reset by hardware
MISO/MOSI (in)
DATA 1 = 0xA1
DATA 2 = 0xA2
DATA 3 = 0xA3
b0 b1 b2 b3 b4 b5 b6 b7 b0 b1 b2 b3 b4 b5 b6 b7 b0 b1 b2 b3 b4 b5 b6 b7
RXNE flag
set by hardware
cleared by software
Rx buffer (read from SPI_DR)
0xA1
0xA2
0xA3

software writes 0xF1 into SPI_DR
software waits until TXE=1 and writes 0xF2 into SPI_DR
software waits until RXNE=1 and reads 0xA1 from SPI_DR
software waits until TXE=1 and writes 0xF3 into SPI_DR
software waits until RXNE=1 and reads 0xA2 from SPI_DR
software waits until RXNE=1 and reads 0xA3 from SPI_DR

ai17344

**Transmit-only procedure (BIDIMODE=0 RXONLY=0)**

In this mode, the procedure can be reduced as described below and the BSY bit can be used to wait until the completion of the transmission (see Figure 255 and Figure 256).

1. Enable the SPI by setting the SPE bit to 1.
2. Write the first data item to send into the SPI_DR register (this clears the TXE bit).
3. Wait until TXE=1 and write the next data item to be transmitted. Repeat this step for each data item to be transmitted.
4. After writing the last data item into the SPI_DR register, wait until TXE=1, then wait until BSY=0, this indicates that the transmission of the last data is complete.

This procedure can be also implemented using dedicated interrupt subroutines launched at each rising edge of the TXE flag.

**Note:**
During discontinuous communications, there is a 2 APB clock period delay between the write operation to SPI_DR and the BSY bit setting. As a consequence, in transmit-only mode, it is mandatory to wait first until TXE is set and then until BSY is cleared after writing the last data.

After transmitting two data items in transmit-only mode, the OVR flag is set in the SPI_SR register since the received data are never read.

888/1749
RM0090 Rev 18

--- Page 109 ---

RM0090
Serial peripheral interface (SPI)

**Figure 255. TXE/BSY behavior in Master transmit-only mode (BIDIMODE=0 and RXONLY=0) in case of continuous transfers**

Example in Master mode with CPOL=1, CPHA=1

SCK
MISO/MOSI (out)
DATA 1 = 0xF1
DATA 2 = 0xF2
DATA 3 = 0xF3
b0 b1 b2 b3 b4 b5 b6 b7 b0 b1 b2 b3 b4 b5 b6 b7 b0 b1 b2 b3 b4 b5 b6 b7
TXE flag
set by hardware cleared by software
set by hardware cleared by software
set by hardware
Tx buffer (write to SPI_DR)
0xF1
0xF2
0xF3
BSY flag
set by hardware
reset by hardware
software writes 0xF1 into SPI_DR
software waits until TXE=1 and writes 0xF2 into SPI_DR
software waits until TXE=1 and writes 0xF3 into SPI_DR
software waits until TXE=1
software waits until BSY=0
ai17345

**Figure 256. TXE/BSY in Slave transmit-only mode (BIDIMODE=0 and RXONLY=0) in case of continuous transfers**

Example in slave mode with CPOL=1, CPHA=1

SCK
MISO/MOSI (out)
DATA 1 = 0xF1
DATA 2 = 0xF2
DATA 3 = 0xF3
b0 b1 b2 b3 b4 b5 b6 b7 b0 b1 b2 b3 b4 b5 b6 b7 b0 b1 b2 b3 b4 b5 b6 b7
TXE flag
set by hardware cleared by software
set by hardware cleared by software
set by hardware
Tx buffer (write to SPI_DR)
0xF1
0xF2
0xF3
BSY flag
set by hardware
reset by hardware
software writes 0xF1 into SPI_DR
software waits until TXE=1 and writes 0xF2 into SPI_DR
software waits until TXE=1 and writes 0xF3 into SPI_DR
software waits until TXE=1
software waits until BSY=0
ai17346

**Bidirectional transmit procedure (BIDIMODE=1 and BIDIOE=1)**

In this mode, the procedure is similar to the procedure in Transmit-only mode except that the BIDIMODE and BIDIOE bits both have to be set in the SPI_CR2 register before enabling the SPI.

**Unidirectional receive-only procedure (BIDIMODE=0 and RXONLY=1)**

In this mode, the procedure can be reduced as described below (see Figure 257):

STI
RM0090 Rev 18
889/1749

--- Page 110 ---

**Serial peripheral interface (SPI)**
RM0090

1. Set the RXONLY bit in the SPI_CR1 register.
2. Enable the SPI by setting the SPE bit to 1:
    a) In master mode, this immediately activates the generation of the SCK clock, and data are serially received until the SPI is disabled (SPE=0).
    b) In slave mode, data are received when the SPI master device drives NSS low and generates the SCK clock.
3. Wait until RXNE=1 and read the SPI_DR register to get the received data (this clears the RXNE bit). Repeat this operation for each data item to be received.

This procedure can also be implemented using dedicated interrupt subroutines launched at each rising edge of the RXNE flag.

*Note:* If it is required to disable the SPI after the last transfer, follow the recommendation described in Section 28.3.8.

**Figure 257. RXNE behavior in receive-only mode (BIDIRMODE=0 and RXONLY=1) in case of continuous transfers**

Example with CPOL=1, CPHA=1, RXONLY=1

SCK
MISO/MOSI (in)
DATA 1 = 0xA1
DATA 2 = 0xA2
DATA 3 = 0xA3
b0 b1 b2 b3 b4 b5 b6 b7 b0 b1 b2 b3 b4 b5 b6 b7 b0 b1 b2 b3 b4 b5 b6 b7
RXNE flag
set by hardware
cleared by software
Rx buffer (read from SPI_DR)
0xA1
0xA2
0xA3
software waits until RXNE=1 and reads 0xA1 from SPI_DR
software waits until RXNE=1 and reads 0xA2 from SPI_DR
software waits until RXNE=1 and reads 0xA3 from SPI_DR
ai17347

**Bidirectional receive procedure (BIDIMODE=1 and BIDIOE=0)**

In this mode, the procedure is similar to the Receive-only mode procedure except that the BIDIMODE bit has to be set and the BIDIOE bit cleared in the SPI_CR2 register before enabling the SPI.

**Continuous and discontinuous transfers**

When transmitting data in master mode, if the software is fast enough to detect each rising edge of TXE (or TXE interrupt) and to immediately write to the SPI_DR register before the ongoing data transfer is complete, the communication is said to be continuous. In this case, there is no discontinuity in the generation of the SPI clock between each data item and the BSY bit is never cleared between each data transfer.

On the contrary, if the software is not fast enough, this can lead to some discontinuities in the communication. In this case, the BSY bit is cleared between each data transmission (see Figure 258).

In Master receive-only mode (RXONLY=1), the communication is always continuous and the BSY flag is always read at 1.

890/1749
RM0090 Rev 18
ST

--- Page 111 ---

RM0090
Serial peripheral interface (SPI)

In slave mode, the continuity of the communication is decided by the SPI master device. In any case, even if the communication is continuous, the BSY flag goes low between each transfer for a minimum duration of one SPI clock cycle (see Figure 256).

Figure 258. TXE/BSY behavior when transmitting (BIDIRMODE=0 and RXONLY=0) in case of discontinuous transfers

Example with CPOL=1, CPHA=1

SCK
MOSI (out)
TXE flag
Tx buffer
(write to SPI_DR)
BSY flag

DATA 1 = 0xF1
b0 b1 b2 b3 b4 b5 b6 b7

DATA 2 = 0xF2
b0 b1 b2 b3 b4 b5 b6 b7

DATA 3 = 0xF3
b0 b1 b2 b3 b4 b5 b6 b7

0xF1
0xF2
0xF3

software writes 0xF1 into SPI_DR
software waits until TXE=1 but is late to write 0xF2 into SPI_DR
software waits until TXE=1 but is late to write 0xF3 into SPI_DR
software waits until TXE=1
software waits until BSY=0

ai17348

28.3.6 CRC calculation

A CRC calculator has been implemented for communication reliability. Separate CRC calculators are implemented for transmitted data and received data. The CRC is calculated using a programmable polynomial serially on each bit. It is calculated on the sampling clock edge defined by the CPHA and CPOL bits in the SPI_CR1 register.

Note: This SPI offers two kinds of CRC calculation standard which depend directly on the data frame format selected for the transmission and/or reception: 8-bit data (CR8) and 16-bit data (CRC16).

CRC calculation is enabled by setting the CRCEN bit in the SPI_CR1 register. This action resets the CRC registers (SPI_RXCRCR and SPI_TXCRCR). In full duplex or transmitter only mode, when the transfers are managed by the software (CPU mode), it is necessary to write the bit CRCNEXT immediately after the last data to be transferred is written to the SPI_DR. At the end of this last data transfer, the SPI_TXCRCR value is transmitted.

In receive only mode and when the transfers are managed by software (CPU mode), it is necessary to write the CRCNEXT bit after the second last data has been received. The CRC is received just after the last data reception and the CRC check is then performed.

At the end of data and CRC transfers, the CRCERR flag in the SPI_SR register is set if corruption occurs during the transfer.

If data are present in the TX buffer, the CRC value is transmitted only after the transmission of the data byte. During CRC transmission, the CRC calculator is switched off and the register value remains unchanged.

SPI communication using the CRC is possible through the following procedure:

ST
RM0090 Rev 18
891/1749

--- Page 112 ---

**Serial peripheral interface (SPI)** RM0090

1. Program the CPOL, CPHA, LSBFirst, BR, SSM, SSI and MSTR values.
2. Program the polynomial in the SPI_CRCPR register.
3. Enable the CRC calculation by setting the CRCEN bit in the SPI_CR1 register. This also clears the SPI_RXCRCR and SPI_TXCRCR registers.
4. Enable the SPI by setting the SPE bit in the SPI_CR1 register.
5. Start the communication and sustain the communication until all but one byte or half-word have been transmitted or received.
    - In full duplex or transmitter-only mode, when the transfers are managed by software, when writing the last byte or half word to the Tx buffer, set the CRCNEXT bit in the SPI_CR1 register to indicate that the CRC will be transmitted after the transmission of the last byte.
    - In receiver only mode, set the bit CRCNEXT just after the reception of the second to last data to prepare the SPI to enter in CRC Phase at the end of the reception of the last data. CRC calculation is frozen during the CRC transfer.
6. After the transfer of the last byte or half word, the SPI enters the CRC transfer and check phase. In full duplex mode or receiver-only mode, the received CRC is compared to the SPI_RXCRCR value. If the value does not match, the CRCErr flag in SPI_SR is set and an interrupt can be generated when the ERRIE bit in the SPI_CR2 register is set.

**Note:** When the SPI is in slave mode, be careful to enable CRC calculation only when the clock is stable, that is, when the clock is in the steady state. If not, a wrong CRC calculation may be done. In fact, the CRC is sensitive to the SCK slave input clock as soon as CRCEN is set, and this, whatever the value of the SPE bit.

With high bitrate frequencies, be careful when transmitting the CRC. As the number of used CPU cycles has to be as low as possible in the CRC transfer phase, it is forbidden to call software functions in the CRC transmission sequence to avoid errors in the last data and CRC reception. In fact, CRCNEXT bit has to be written before the end of the transmission/reception of the last data.

For high bit rate frequencies, it is advised to use the DMA mode to avoid the degradation of the SPI speed performance due to CPU accesses impacting the SPI bandwidth.

When the devices are configured as slaves and the NSS hardware mode is used, the NSS pin needs to be kept low between the data phase and the CRC phase.

When the SPI is configured in slave mode with the CRC feature enabled, CRC calculation takes place even if a high level is applied on the NSS pin. This may happen for example in case of a multislave environment where the communication master addresses slaves alternately.

Between a slave deselection (high level on NSS) and a new slave selection (low level on NSS), the CRC value should be cleared on both master and slave sides in order to resynchronize the master and slave for their respective CRC calculation.

To clear the CRC, follow the procedure below:
1. Disable SPI (SPE = 0)
2. Clear the CRCEN bit
3. Set the CRCEN bit
4. Enable the SPI (SPE = 1)

892/1749 RM0090 Rev 18 ST

--- Page 113 ---

RM0090
Serial peripheral interface (SPI)


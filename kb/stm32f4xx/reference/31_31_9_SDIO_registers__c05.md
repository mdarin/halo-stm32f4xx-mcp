> [RM0090 ch.31 §31.9] SDIO registers**


### 31.9.11 SDIO status register (SDIO_STA)

Address offset: 0x34
Reset value: 0x0000 0000

The SDIO_STA register is a read-only register. It contains two types of flag:

*   Static flags (bits [23:22,10:0]): these bits remain asserted until they are cleared by writing to the SDIO Interrupt Clear register (see SDIO_ICR)
*   Dynamic flags (bits [21:11]): these bits change state depending on the state of the underlying logic (for example, FIFO full and empty flags are asserted and deasserted as data while written to the FIFO)

| Reserved | CEATAEND | SDIOIT | RXDAVL | TXDAVL | RXFIFOE | TXFIFOE | RXFIFOF | TXFIFOF | RXFIFOHF | TXFIFOHE | RXACT | TXACT | CMDACT | DBCKEND | STBITERR | DATAEND | CMDSENT | CMDREND | RXOVERR | TXUNERR | DTIMEOUT | CTIMEOUT | DCRCFAIL | CCRCFAIL |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| r |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bits 31:24 Reserved, must be kept at reset value

Bit 23 **CEATAEND**: CE-ATA command completion signal received for CMD61
Bit 22 **SDIOIT**: SDIO interrupt received
Bit 21 **RXDAVL**: Data available in receive FIFO
Bit 20 **TXDAVL**: Data available in transmit FIFO
Bit 19 **RXFIFOE**: Receive FIFO empty
Bit 18 **TXFIFOE**: Transmit FIFO empty
When HW Flow Control is enabled, TXFIFOE signals becomes activated when the FIFO contains 2 words.
Bit 17 **RXFIFOF**: Receive FIFO full
When HW Flow Control is enabled, RXFIFOF signals becomes activated 2 words before the FIFO is full.
Bit 16 **TXFIFOF**: Transmit FIFO full
Bit 15 **RXFIFOHF**: Receive FIFO half full: there are at least 8 words in the FIFO
Bit 14 **TXFIFOHE**: Transmit FIFO half empty: at least 8 words can be written into the FIFO
Bit 13 **RXACT**: Data receive in progress
Bit 12 **TXACT**: Data transmit in progress
Bit 11 **CMDACT**: Command transfer in progress
Bit 10 **DBCKEND**: Data block sent/received (CRC check passed)
Bit 9 **STBITERR**: Start bit not detected on all data signals in wide bus mode
Bit 8 **DATAEND**: Data end (data counter, SDIDCOUNT, is zero)
Bit 7 **CMDSENT**: Command sent (no response required)
Bit 6 **CMDREND**: Command response received (CRC check passed)
Bit 5 **RXOVERR**: Received FIFO overrun error

1068/1749
RM0090 Rev 18

--- Page 94 ---

RM0090
Secure digital input/output interface (SDIO)

Bit 4 **TXUNDERR**: Transmit FIFO underrun error
Bit 3 **DTIMEOUT**: Data timeout
Bit 2 **CTIMEOUT**: Command response timeout
The Command TimeOut period has a fixed value of 64 SDIO\_CK clock periods.
Bit 1 **DCRCFAIL**: Data block sent/received (CRC check failed)
Bit 0 **CCRCFAIL**: Command response received (CRC check failed)


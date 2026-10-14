> [RM0090 ch.28 §28.3] SPI functional description


### 28.3.7 Status flags

Four status flags are provided for the application to completely monitor the state of the SPI bus.

**Tx buffer empty flag (TXE)**

When it is set, this flag indicates that the Tx buffer is empty and the next data to be transmitted can be loaded into the buffer. The TXE flag is cleared when writing to the SPI_DR register.

**Rx buffer not empty (RXNE)**

When set, this flag indicates that there are valid received data in the Rx buffer. It is cleared when SPI_DR is read.

**BUSY flag**

This BSY flag is set and cleared by hardware (writing to this flag has no effect). The BSY flag indicates the state of the communication layer of the SPI.

When BSY is set, it indicates that the SPI is busy communicating. There is one exception in master mode / bidirectional receive mode (MSTR=1 and BDM=1 and BDOE=0) where the BSY flag is kept low during reception.

The BSY flag is useful to detect the end of a transfer if the software wants to disable the SPI and enter Halt mode (or disable the peripheral clock). This avoids corrupting the last transfer. For this, the procedure described below must be strictly respected.

The BSY flag is also useful to avoid write collisions in a multimaster system.

The BSY flag is set when a transfer starts, with the exception of master mode / bidirectional receive mode (MSTR=1 and BDM=1 and BDOE=0).

It is cleared:
* when a transfer is finished (except in master mode if the communication is continuous)
* when the SPI is disabled
* when a master mode fault occurs (MODF=1)

When communication is not continuous, the BSY flag is low between each communication.

When communication is continuous:
* in master mode, the BSY flag is kept high during all the transfers
* in slave mode, the BSY flag goes low for one SPI clock cycle between each transfer

**Note:** Do not use the BSY flag to handle each data transmission or reception. It is better to use the TXE and RXNE flags instead.

ST
RM0090 Rev 18
893/1749

--- Page 114 ---

Serial peripheral interface (SPI)
RM0090


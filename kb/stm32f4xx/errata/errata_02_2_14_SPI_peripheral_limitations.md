**2.14 SPI peripheral limitations**

**2.14.1 Wrong CRC calculation when the polynomial is even.**

**Description:**

When the CRC is enabled, the CRC calculation will be wrong if the polynomial is even.

**Work-around:**

Use odd polynomial.



37

--- Page 34 ---

**STM32F40x and STM32F41x silicon limitations STM32F40x and STM32F41x**

**2.14.2 BSY bit may stay high at the end of a data transfer in slave mode**

**Description**

BSY flag may sporadically remain high at the end of a data transfer in slave mode. The issue appears when an accidental synchronization happens between internal CPU clock and external SCK clock provided by master.

This is related to the end of data transfer detection while the SPI is enabled in slave mode. As a consequence, the end of data transaction is not recognized before entering the low power mode or for a change of the SPI configuration (e.g. direction of the bidirectional mode). The BSY flag is then not a reliable way to handle the end of a data frame transmission when it is used to signal the end of this transaction with master.

**Workaround**

• When in a SPI receiving mode, the end of a transaction with master can be detected by

the corresponding RXNE event when this flag is set after the last bit of that transaction is sampled and received data stored.

• When the following sequence is used the condition of the synchronization issue is

prevented and the BSY bit works correctly. The BSY flag then can be used to recognize end of any transmission transaction (including the case of bidirectional mode when RXNE is not raised):

– write last data to data register

– poll TXE till it becomes high to ensure the data transfer has started

– disable SPI by clearing SPE while the last data transfer is still on going

– poll the BSY bit till it becomes low

Note: This workaround can be used only when CPU has enough performance to disable SPI after TXE event is detected while the data frame transfer is still ongoing. It is impossible to achieve it when ratio between CPU and SPI clock is low and data frame is short especially. In this specific case timeout can be measured from TXE, while calculating fixed number of CPU clock periods corresponding to the time necessary to complete the data frame transaction.

**2.14.3 LSB bit of data or CRC calculation can be corrupted for the received data in master mode depending on the timing of the feedback communication clock respect to the APB clock (SPI or I2S)**

**Description**

When the SPI or I2S is configured in master mode (in full duplex or receiver mode, reading back the data register or CRC enabled), the data received may have the last transacted bit corrupted if delay of internal feedback derived from SCK pin is comparable with APB clock period. The last bit value is strobed too late into the shift register, while its content has been already either copied into the data register or compared with CRC pattern calculated internally. In case of data corruption, the bit position in the data register contains the value of the last bit received during the previous data transfer and CRCERR flag is asserted in spite of all the data being received correctly.

The main factors which can contribute negatively to the delay are decreased Vdd level, extreme temperature, high SPI bus capacity load and low SCK IO output speed. SPI communication speed has no impact.



--- Page 35 ---

**STM32F40x and STM32F41x STM32F40x and STM32F41x silicon limitations**

**Workaround**

• Decrease the APB clock.

• Set the IO pad configuration for the SCK pin to be faster.

• Ignore the lastly transacted bit value



37

--- Page 36 ---

**Revision history**

**STM32F40x and STM32F41x**


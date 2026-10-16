### 33.4 Ethernet functional description: SMI, MII and RMII

The Ethernet peripheral consists of a MAC 802.3 (media access control) with a dedicated DMA controller. It supports both default media-independent interface (MII) and reduced media-independent interface (RMII) through one selection bit (refer to SYSCFG_PMC register).

The DMA controller interfaces with the Core and memories through the AHB Master and Slave interfaces. The AHB Master Interface controls data transfers while the AHB Slave interface accesses Control and Status Registers (CSR) space.

The Transmit FIFO (Tx FIFO) buffers data read from system memory by the DMA before transmission by the MAC Core. Similarly, the Receive FIFO (Rx FIFO) stores the Ethernet frames received from the line until they are transferred to system memory by the DMA.

The Ethernet peripheral also includes an SMI to communicate with external PHY. A set of configuration registers permit the user to select the desired mode and features for the MAC and the DMA controller.

*Note:* The AHB clock frequency must be at least 25 MHz when the Ethernet is used.

**Figure 350. ETH block diagram**

ai15620c

1. For AHB connections refer to Figure 1: System architecture for STM32F405xx/07xx and STM32F415xx/17xx devices and Figure 2: System architecture for STM32F42xxx and STM32F43xxx devices.

### 33.4.1 Station management interface: SMI

The station management interface (SMI) allows the application to access any PHY registers through a 2-wire clock and data lines. The interface supports accessing up to 32 PHYs.

The application can select one of the 32 PHYs and one of the 32 registers within any PHY and send control data or receive status information. Only one register in one PHY can be addressed at any given time.

Both the MDC clock line and the MDIO data line are implemented as alternate function I/O in the microcontroller:

* MDC: a periodic clock that provides the timing reference for the data transfer at the maximum frequency of 2.5 MHz. The minimum high and low times for MDC must be

1126/1749
RM0090 Rev 18
ST

--- Page 152 ---

RM0090
Ethernet (ETH): media access control (MAC) with DMA controller

160 ns each, and the minimum period for MDC must be 400 ns. In idle state the SMI management interface drives the MDC clock signal low.
* MDIO: data input/output bitstream to transfer status information to/from the PHY device synchronously with the MDC clock signal

**Figure 351. SMI interface signals**

ai15621b

**SMI frame format**

The frame structure related to a read or write operation is shown in Table 13, the order of bit transmission must be from left to right.

**Table 186. Management frame format**

|  | Management frame fields |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|
| Preamble (32 bits) | Start | Operation | PADDR | RADDR | TA | Data (16 bits) | Idle |  |
| Read | 1... 1 | 01 | 10 | ppppp | rrrrr | Z0 | dddddddddddddd | Z |
| Write | 1... 1 | 01 | ppppp | rrrrr | 10 | dddddddddddddd | Z |  |


The management frame consists of eight fields:
* **Preamble**: each transaction (read or write) can be initiated with the preamble field that corresponds to 32 contiguous logic one bits on the MDIO line with 32 corresponding cycles on MDC. This field is used to establish synchronization with the PHY device.
* **Start**: the start of frame is defined by a <01> pattern to verify transitions on the line from the default logic one state to zero and back to one.
* **Operation**: defines the type of transaction (read or write) in progress.
* **PADDR**: the PHY address is 5 bits, allowing 32 unique PHY addresses. The MSB bit of the address is the first transmitted and received.
* **RADDR**: the register address is 5 bits, allowing 32 individual registers to be addressed within the selected PHY device. The MSB bit of the address is the first transmitted and received.
* **TA**: the turn-around field defines a 2-bit pattern between the RADDR and DATA fields to avoid contention during a read transaction. For a read transaction the MAC controller drives high-impedance on the MDIO line for the 2 bits of TA. The PHY device must drive a high-impedance state on the first bit of TA, a zero bit on the second one.

ST
RM0090 Rev 18
1127/1749

--- Page 153 ---

**Ethernet (ETH): media access control (MAC) with DMA controller** RM0090

For a write transaction, the MAC controller drives a <10> pattern during the TA field.
The PHY device must drive a high-impedance state for the 2 bits of TA.

*   **Data**: the data field is 16-bit. The first bit transmitted and received must be bit 15 of the ETH_MIID register.
*   **Idle**: the MDIO line is driven in high-impedance state. All three-state drivers must be disabled and the PHY’s pull-up resistor keeps the line at logic one.

**SMI write operation**

When the application sets the MII Write and Busy bits (in *Ethernet MAC MII address register (ETH_MACMIAR)*), the SMI initiates a write operation into the PHY registers by transferring the PHY address, the register address in PHY, and the write data (in *Ethernet MAC MII data register (ETH_MACMIIDR)*). The application should not change the MII Address register contents or the MII Data register while the transaction is ongoing. Write operations to the MII Address register or the MII Data Register during this period are ignored (the Busy bit is high), and the transaction is completed without any error. After the Write operation has completed, the SMI indicates this by resetting the Busy bit.

*Figure 352* shows the frame format for the write operation.

**Figure 352. MDIO timing and frame structure - Write cycle**

ai15626

**SMI read operation**

When the user sets the MII Busy bit in the Ethernet MAC MII address register (ETH_MACMIAR) with the MII Write bit at 0, the SMI initiates a read operation in the PHY registers by transferring the PHY address and the register address in PHY. The application should not change the MII Address register contents or the MII Data register while the transaction is ongoing. Write operations to the MII Address register or MII Data Register during this period are ignored (the Busy bit is high) and the transaction is completed without any error. After the read operation has completed, the SMI resets the Busy bit and then updates the MII Data register with the data read from the PHY.

*Figure 353* shows the frame format for the read operation.

1128/1749 RM0090 Rev 18 ST

--- Page 154 ---

RM0090
Ethernet (ETH): media access control (MAC) with DMA controller

Figure 353. MDIO timing and frame structure - Read cycle

ai15627

SMI clock selection

The MAC initiates the Management Write/Read operation. The SMI clock is a divided clock whose source is the application clock (AHB clock). The divide factor depends on the clock range setting in the MII Address register.

Table 187 shows how to set the clock ranges.

Table 187. Clock range
| Selection | HCLK clock | MDC clock |
|---|---|---|
| 000 | 60-100 MHz | AHB clock / 42 |
| 001 | 100-150 MHz | AHB clock / 62 |
| 010 | 20-35 MHz | AHB clock / 16 |
| 011 | 35-60 MHz | AHB clock / 26 |
| 100 | 150-180 MHz | AHB clock / 102 |
| 101, 110, 111 | Reserved | - |


33.4.2 Media-independent interface: MII

The media-independent interface (MII) defines the interconnection between the MAC sublayer and the PHY for data transfer at 10 Mbit/s and 100 Mbit/s.

Figure 354. Media independent interface signals

ai15622c

STI
RM0090 Rev 18
1129/1749

--- Page 155 ---

**Ethernet (ETH): media access control (MAC) with DMA controller** RM0090

*   **MII_TX_CLK**: continuous clock that provides the timing reference for the TX data transfer. The nominal frequency is: 2.5 MHz at 10 Mbit/s speed; 25 MHz at 100 Mbit/s speed.
*   **MII_RX_CLK**: continuous clock that provides the timing reference for the RX data transfer. The nominal frequency is: 2.5 MHz at 10 Mbit/s speed; 25 MHz at 100 Mbit/s speed.
*   **MII_TX_EN**: transmission enable indicates that the MAC is presenting nibbles on the MII for transmission. It must be asserted synchronously (MII_TX_CLK) with the first nibble of the preamble and must remain asserted while all nibbles to be transmitted are presented to the MII.
*   **MII_TXD[3:0]**: transmit data is a bundle of 4 data signals driven synchronously by the MAC sublayer and qualified (valid data) on the assertion of the MII_TX_EN signal. MII_TXD[0] is the least significant bit, MII_TXD[3] is the most significant bit. While MII_TX_EN is deasserted the transmit data must have no effect upon the PHY.
*   **MII_CRS**: carrier sense is asserted by the PHY when either the transmit or receive medium is non idle. It shall be deasserted by the PHY when both the transmit and receive media are idle. The PHY must ensure that the MII_CS signal remains asserted throughout the duration of a collision condition. This signal is not required to transition synchronously with respect to the TX and RX clocks. In full duplex mode the state of this signal is don't care for the MAC sublayer.
*   **MII_COL**: collision detection must be asserted by the PHY upon detection of a collision on the medium and must remain asserted while the collision condition persists. This signal is not required to transition synchronously with respect to the TX and RX clocks. In full duplex mode the state of this signal is don't care for the MAC sublayer.
*   **MII_RXD[3:0]**: reception data is a bundle of 4 data signals driven synchronously by the PHY and qualified (valid data) on the assertion of the MII_RX_DV signal. MII_RXD[0] is the least significant bit, MII_RXD[3] is the most significant bit. While MII_RX_EN is deasserted and MII_RX_ER is asserted, a specific MII_RXD[3:0] value is used to transfer specific information from the PHY (see Table 189).
*   **MII_RX_DV**: receive data valid indicates that the PHY is presenting recovered and decoded nibbles on the MII for reception. It must be asserted synchronously (MII_RX_CLK) with the first recovered nibble of the frame and must remain asserted through the final recovered nibble. It must be deasserted prior to the first clock cycle that follows the final nibble. In order to receive the frame correctly, the MII_RX_DV signal must encompass the frame, starting no later than the SFD field.
*   **MII_RX_ER**: receive error must be asserted for one or more clock periods (MII_RX_CLK) to indicate to the MAC sublayer that an error was detected somewhere in the frame. This error condition must be qualified by MII_RX_DV assertion as described in Table 189.

Table 188. TX interface signal encoding

| MII_TX_EN | MII_TXD[3:0] | Description |
|---|---|---|
| 0 | 0000 through 1111 | Normal inter-frame |
| 1 | 0000 through 1111 | Normal data transmission |


1130/1749 RM0090 Rev 18 ST

--- Page 156 ---

RM0090
Ethernet (ETH): media access control (MAC) with DMA controller

Table 189. RX interface signal encoding

| MII_RX_DV | MII_RX_ERR | MII_RXD[3:0] | Description |
|---|---|---|---|
| 0 | 0000 through 1111 | Normal inter-frame |  |
| 0 | 1 | 0000 | Normal inter-frame |
| 0 | 1 | 0001 through 1101 | Reserved |
| 0 | 1 | 1110 | False carrier indication |
| 0 | 1 | 1111 | Reserved |
| 1 | 0 | 0000 through 1111 | Normal data reception |
| 1 | 0000 through 1111 | Data reception with errors |  |


**MII clock sources**

To generate both TX_CLK and RX_CLK clock signals, the external PHY must be clocked with an external 25 MHz as shown in Figure 355. Instead of using an external 25 MHz quartz to provide this clock, the STM32F4xxmicrocontroller can output this signal on its MCO pin. In this case, the PLL multiplier has to be configured so as to get the desired frequency on the MCO pin, from the 25 MHz external quartz.

Figure 355. MII clock sources

25 MHz
STM32 MCU
HSE
MCO
802.3 MAC
TX_CLK
RX_CLK
External PHY
25 MHz
For 10/100 Mbit/s
ai15623b

**33.4.3 Reduced media-independent interface: RMII**

The reduced media-independent interface (RMII) specification reduces the pin count between the microcontroller Ethernet peripheral and the external Ethernet in 10/100 Mbit/s. According to the IEEE 802.3u standard, an MII contains 16 pins for data and control. The RMII specification is dedicated to reduce the pin count to 7 pins (a 62.5% decrease in pin count).

ST
RM0090 Rev 18
1131/1749

--- Page 157 ---

**Ethernet (ETH): media access control (MAC) with DMA controller** RM0090

The RMII is instantiated between the MAC and the PHY. This helps translation of the MAC's MII into the RMII. The RMII block has the following characteristics:

*   It supports 10-Mbit/s and 100-Mbit/s operating rates
*   The clock reference must be doubled to 50 MHz
*   The same clock reference must be sourced externally to both MAC and external Ethernet PHY
*   It provides independent 2-bit wide (dibit) transmit and receive data paths

**Figure 356. Reduced media-independent interface signals**

STM32 MCU
802.3 MAC
TXD[1:0]
TX_EN
RXD[1:0]
CRSDV
MDC
MDIO
REF CLK
Clock source
External PHY
ai15624b

**RMII clock sources**

Either clock the PHY from an external 50 MHz clock or use a PHY with an embedded PLL to generate the 50 MHz frequency.

**Figure 357. RMII clock sources**

STM32 MCU
802.3 MAC
25 MHz
PLL
External PHY
REF_CLK
50 MHz
For 10/100 Mbit/s
MS19930V1

**33.4.4 MII/RMII selection**

The mode, MII or RMII, is selected using the configuration bit 23, MII_RMII_SEL, in the SYSCFG_PMC register. The application has to set the MII/RMII mode while the Ethernet controller is under reset or before enabling the clocks.

1132/1749 RM0090 Rev 18 STI

--- Page 158 ---

RM0090
Ethernet (ETH): media access control (MAC) with DMA controller

### MII/RMII internal clock scheme

The clock scheme required to support both the MII and RMII, as well as 10 and 100 Mbit/s operations is described in Figure 358.

**Figure 358. Clock scheme**

1. The MII/RMII selection is controlled through bit 23, MII_RMII_SEL, in the SYSCFG_PMC register.
To save a pin, the two input clock signals, RMII_REF_CK and MII_RX_CK, are multiplexed on the same GPIO pin.


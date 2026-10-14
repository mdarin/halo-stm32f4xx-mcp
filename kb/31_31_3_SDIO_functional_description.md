### 31.3 SDIO functional description

The SDIO consists of two parts:
*   The SDIO adapter block provides all functions specific to the MMC/SD/SD I/O card such as the clock generation unit, command and data transfer.
*   The APB2 interface accesses the SDIO adapter registers, and generates interrupt and DMA request signals.

**Figure 326. SDIO block diagram**

ai15898b

By default SDIO_D0 is used for data transfer. After initialization, the host can change the databus width.

If a MultiMediaCard is connected to the bus, SDIO_D0, SDIO_D[3:0] or SDIO_D[7:0] can be used for data transfer. MMC V3.31 or previous, supports only 1 bit of data so only SDIO_D0 can be used.

If an SD or SD I/O card is connected to the bus, data transfer can be configured by the host to use SDIO_D0 or SDIO_D[3:0]. All data lines are operating in push-pull mode.

**SDIO_CMD** has two operational modes:
*   Open-drain for initialization (only for MMCV3.31 or previous)
*   Push-pull for command transfer (SD/SD I/O card MMC4.2 use push-pull drivers also for initialization)

**SDIO_CK** is the clock to the card: one bit is transferred on both command and data lines with each clock cycle.

The SDIO uses two clock signals:
*   SDIO adapter clock SDIOCLK up to 50 MHz (48 MHz when in use with USB)
*   APB2 bus clock (PCLK2)

PCLK2 and SDIO_CK clock frequencies must respect the following condition:

$$Frequency(PCLK2) \ge 3 / 8 \times Frequency(SDIO\_CK)$$

The signals shown in Table 150 are used on the MultiMediaCard/SD/SD I/O card bus.

1022/1749
RM0090 Rev 18
ST

--- Page 48 ---

RM0090
Secure digital input/output interface (SDIO)

Table 150. SDIO I/O definitions

| Pin | Direction | Description |
|---|---|---|
| SDIO_CK | Output | MultiMediaCard/SD/SDIO card clock. This pin is the clock from host to card. |
| SDIO_CMD | Bidirectional | MultiMediaCard/SD/SDIO card command. This pin is the bidirectional command/response signal. |
| SDIO_D[7:0] | Bidirectional | MultiMediaCard/SD/SDIO card data. These pins are the bidirectional databus. |


### 31.3.1 SDIO adapter

*Figure 327* shows a simplified block diagram of an SDIO adapter.

**Figure 327. SDIO adapter**

SDMMC adapter
Control unit
Command path
Data path
Adapter registers
FIFO
To APB2 interface
PCLK2
SDMMC_CK
SDMMC_CMD
SDMMC_D[7:0]
SDMMCCLK
Card bus
ai15899b

The SDIO adapter is a multimedia/secure digital memory card bus master that provides an interface to a multimedia card stack or to a secure digital memory card. It consists of five subunits:

* Adapter register block
* Control unit
* Command path
* Data path
* Data FIFO

**Note:** The adapter registers and FIFO use the APB2 bus clock domain (PCLK2). The control unit, command path and data path use the SDIO adapter clock domain (SDIOCLK).

**Adapter register block**

The adapter register block contains all system registers. This block also generates the signals that clear the static flags in the multimedia card. The clear signals are generated when 1 is written into the corresponding bit location in the SDIO Clear register.

ST
RM0090 Rev 18
1023/1749

--- Page 49 ---

Secure digital input/output interface (SDIO)
RM0090

**Control unit**

The control unit contains the power management functions and the clock divider for the memory card clock.

There are three power phases:
*   power-off
*   power-up
*   power-on

**Figure 328. Control unit**

Control unit
Power management
Adapter registers
Clock management
SDMMC_CK
To command and data path
ai14804b

The control unit is illustrated in Figure 328. It consists of a power management subunit and a clock management subunit.

The power management subunit disables the card bus output signals during the power-off and power-up phases.

The clock management subunit generates and controls the SDIO\_CK signal. The SDIO\_CK output can use either the clock divide or the clock bypass mode. The clock output is inactive:
*   after reset
*   during the power-off or power-up phases
*   if the power saving mode is enabled and the card bus is in the Idle state (eight clock periods after both the command and data path subunits enter the Idle phase)

1024/1749
RM0090 Rev 18
ST

--- Page 50 ---

RM0090
Secure digital input/output interface (SDIO)

**Command path**
The command path unit sends commands to and receives responses from the cards.

**Figure 329. SDIO adapter command path**

To control unit
Status flag
Control logic
Command timer
Adapter registers
CMD
Argument
SDMMC_CMDin
Shift register
CRC
SDMMC_CMDout
To APB2 interface
CMD
Response
registers
ai15900b

*   Command path state machine (CPSM)
    *   When the command register is written to and the enable bit is set, command transfer starts. When the command has been sent, the command path state machine (CPSM) sets the status flags and enters the Idle state if a response is not required. If a response is required, it waits for the response (see Figure 330 on page 1026). When the response is received, the received CRC code and the internally generated code are compared, and the appropriate status flags are set.

ST
RM0090 Rev 18
1025/1749

--- Page 51 ---

**Secure digital input/output interface (SDIO)**
RM0090

**Figure 330. Command path state machine (CPSM)**

ai14806b

When the Wait state is entered, the command timer starts running. If the timeout is reached before the CPSM moves to the Receive state, the timeout flag is set and the Idle state is entered.

**Note:** *The command timeout has a fixed value of 64 SDIO\_CK clock periods.*

If the interrupt bit is set in the command register, the timer is disabled and the CPSM waits for an interrupt request from one of the cards. If a pending bit is set in the command register, the CPSM enters the Pend state, and waits for a CmdPend signal from the data path subunit. When CmdPend is detected, the CPSM moves to the Send state. This enables the data counter to trigger the stop command transmission.

**Note:** *The CPSM remains in the Idle state for at least eight SDIO\_CK periods to meet the $N_{CC}$ and $N_{RC}$ timing constraints. $N_{CC}$ is the minimum delay between two host commands, and $N_{RC}$ is the minimum delay between the host command and the card response.*

1026/1749
RM0090 Rev 18
ST

--- Page 52 ---

RM0090
Secure digital input/output interface (SDIO)

**Figure 331. SDIO command transfer**

ai14807b

*   **Command format**
    *   Command: a command is a token that starts an operation. Command are sent from the host either to a single card (addressed command) or to all connected cards (broadcast command are available for MMC V3.31 or previous). Commands are transferred serially on the CMD line. All commands have a fixed length of 48 bits. The general format for a command token for MultiMediaCards, SD-Memory cards and SDIO-Cards is shown in Table 151. CE-ATA commands are an extension of MMC commands V4.2, and so have the same format.
    The command path operates in a half-duplex mode, so that commands and responses can either be sent or received. If the CPSM is not in the Send state, the SDIO_CMD output is in the Hi-Z state, as shown in Figure 331 on page 1027. Data on SDIO_CMD are synchronous with the rising edge of SDIO_CK. Table shows the command format.

**Table 151. Command format**

| Bit position | Width | Value | Description |
|---|---|---|---|
| 47 | 1 | 0 | Start bit |
| 46 | 1 | Transmission bit |  |
| [45:40] | 6 | - | Command index |
| [39:8] | 32 | - | Argument |
| [7:1] | 7 | - | CRC7 |
| 0 | 1 | End bit |  |


*   Response: a response is a token that is sent from an addressed card (or synchronously from all connected cards for MMC V3.31 or previous), to the host as an answer to a previously received command. Responses are transferred serially on the CMD line.

The SDIO supports two response types. Both use CRC error checking:
*   48 bit short response
*   136 bit long response

**Note:** If the response does not contain a CRC (CMD1 response), the device driver must ignore the CRC failed status.

ST
RM0090 Rev 18
1027/1749

--- Page 53 ---

Secure digital input/output interface (SDIO)
RM0090

Table 152. Short response format
| Bit position | Width | Value | Description |
|---|---|---|---|
| 47 | 1 | 0 | Start bit |
| 46 | 1 | 0 | Transmission bit |
| [45:40] | 6 | - | Command index |
| [39:8] | 32 | - | Argument |
| [7:1] | 7 | - | CRC7(or 1111111) |
| 0 | 1 | End bit |  |


Table 153. Long response format
| Bit position | Width | Value | Description |
|---|---|---|---|
| 135 | 1 | 0 | Start bit |
| 134 | 1 | 0 | Transmission bit |
| [133:128] | 6 | 111111 | Reserved |
| [127:1] | 127 | - | CID or CSD (including internal CRC7) |
| 0 | 1 | End bit |  |


The command register contains the command index (six bits sent to a card) and the command type. These determine whether the command requires a response, and whether the response is 48 or 136 bits long (see Section 31.9.4 on page 1062). The command path implements the status flags shown in Table 154:

Table 154. Command path status flags
| Flag | Description |
|---|---|
| CMDREND | Set if response CRC is OK. |
| CCRCFAIL | Set if response CRC fails. |
| CMDSENT | Set when command (that does not require response) is sent |
| CTIMEOUT | Response timeout. |
| CMDACT | Command transfer in progress. |


The CRC generator calculates the CRC checksum for all bits before the CRC code. This includes the start bit, transmitter bit, command index, and command argument (or card status). The CRC checksum is calculated for the first 120 bits of CID or CSD for the long response format. Note that the start bit, transmitter bit and the six reserved bits are not used in the CRC calculation.

The CRC checksum is a 7-bit value:
CRC[6:0] = Remainder [(M(x) * x^7) / G(x)]
G(x) = x^7 + x^3 + 1
M(x) = (start bit) * x^39 + ... + (last bit before CRC) * x^0, or
M(x) = (start bit) * x^119 + ... + (last bit before CRC) * x^0

1028/1749
RM0090 Rev 18

--- Page 54 ---

RM0090
Secure digital input/output interface (SDIO)

**Data path**

The data path subunit transfers data to and from cards. *Figure 332* shows a block diagram of the data path.

**Figure 332. Data path**

Data path
To control unit
Status flag
Control logic
Data timer
Data FIFO
Transmit
Receive
Shift register
CRC
SDMMC_Din[7:0]
SDMMC_Dout[7:0]
ai14808b

The card databus width can be programmed using the clock control register. If the 4-bit wide bus mode is enabled, data is transferred at four bits per clock cycle over all four data signals (SDIO_D[3:0]). If the 8-bit wide bus mode is enabled, data is transferred at eight bits per clock cycle over all eight data signals (SDIO_D[7:0]). If the wide bus mode is not enabled, only one bit per clock cycle is transferred over SDIO_D0.

Depending on the transfer direction (send or receive), the data path state machine (DPSM) moves to the *Wait_S* or *Wait_R* state when it is enabled:

*   Send: the DPSM moves to the *Wait_S* state. If there is data in the transmit FIFO, the DPSM moves to the Send state, and the data path subunit starts sending data to a card.
*   Receive: the DPSM moves to the *Wait_R* state and waits for a start bit. When it receives a start bit, the DPSM moves to the Receive state, and the data path subunit starts receiving data from a card.

Data path state machine (DPSM)

The DPSM operates at SDIO_CK frequency. Data on the card bus signals is synchronous to the rising edge of SDIO_CK. The DPSM has six states, as shown in *Figure 333: Data path state machine (DPSM)*.

ST
RM0090 Rev 18
1029/1749

--- Page 55 ---

**Secure digital input/output interface (SDIO)**

**RM0090**

**Figure 333. Data path state machine (DPSM)**

*   **Idle**: the data path is inactive, and the SDIO_D[7:0] outputs are in Hi-Z. When the data control register is written and the enable bit is set, the DPSM loads the data counter with a new value and, depending on the data direction bit, moves to either the Wait_S or the Wait_R state.
*   **Wait_R**: if the data counter equals zero, the DPSM moves to the Idle state when the receive FIFO is empty. If the data counter is not zero, the DPSM waits for a start bit on SDIO_D. The DPSM moves to the Receive state if it receives a start bit before a timeout, and loads the data block counter. If it reaches a timeout before it detects a start bit, or a start bit error occurs, it moves to the Idle state and sets the timeout status flag.
*   **Receive**: serial data received from a card is packed in bytes and written to the data FIFO. Depending on the transfer mode bit in the data control register, the data transfer mode can be either block or stream:
    *   In block mode, when the data block counter reaches zero, the DPSM waits until it receives the CRC code. If the received code matches the internally generated

1030/1749
RM0090 Rev 18

--- Page 56 ---

RM0090
Secure digital input/output interface (SDIO)

CRC code, the DPSM moves to the Wait_R state. If not, the CRC fail status flag is set and the DPSM moves to the Idle state.

- In stream mode, the DPSM receives data while the data counter is not zero. When the counter is zero, the remaining data in the shift register is written to the data FIFO, and the DPSM moves to the Wait_R state.

If a FIFO overrun error occurs, the DPSM sets the FIFO error flag and moves to the Idle state:

- Wait_S: the DPSM moves to the Idle state if the data counter is zero. If not, it waits until the data FIFO empty flag is deasserted, and moves to the Send state.

Note: The DPSM remains in the Wait_S state for at least two clock periods to meet the $N_{WR}$ timing requirements, where $N_{WR}$ is the number of clock cycles between the reception of the card response and the start of the data transfer from the host.

- Send: the DPSM starts sending data to a card. Depending on the transfer mode bit in the data control register, the data transfer mode can be either block or stream:
  - In block mode, when the data block counter reaches zero, the DPSM sends an internally generated CRC code and end bit, and moves to the Busy state.
  - In stream mode, the DPSM sends data to a card while the enable bit is high and the data counter is not zero. It then moves to the Idle state.

If a FIFO underrun error occurs, the DPSM sets the FIFO error flag and moves to the Idle state.

- Busy: the DPSM waits for the CRC status flag:
  - If it does not receive a positive CRC status, it moves to the Idle state and sets the CRC fail status flag.
  - If it receives a positive CRC status, it moves to the Wait_S state if SDIO_D0 is not low (the card is not busy).

If a timeout occurs while the DPSM is in the Busy state, it sets the data timeout flag and moves to the Idle state.

The data timer is enabled when the DPSM is in the Wait_R or Busy state, and generates the data timeout error:
  - When transmitting data, the timeout occurs if the DPSM stays in the Busy state for longer than the programmed timeout period
  - When receiving data, the timeout occurs if the end of the data is not true, and if the DPSM stays in the Wait_R state for longer than the programmed timeout period.

- Data: data can be transferred from the card to the host or vice versa. Data is transferred via the data lines. They are stored in a FIFO of 32 words, each word is 32 bits wide.

Table 155. Data token format

| Description | Start bit | Data | CRC16 | End bit |
|---|---|---|---|---|
| Block Data | 0 | - | yes | 1 |
| Stream Data | 0 | - | no | 1 |


ST
RM0090 Rev 18
1031/1749

--- Page 57 ---

**Secure digital input/output interface (SDIO)**
RM0090

**Data FIFO**

The data FIFO (first-in-first-out) subunit is a data buffer with a transmit and receive unit.

The FIFO contains a 32-bit wide, 32-word deep data buffer, and transmit and receive logic. Because the data FIFO operates in the APB2 clock domain (PCLK2), all signals from the subunits in the SDIO clock domain (SDIOCLK) are resynchronized.

Depending on the TXACT and RXACT flags, the FIFO can be disabled, transmit enabled, or receive enabled. TXACT and RXACT are driven by the data path subunit and are mutually exclusive:

- The transmit FIFO refers to the transmit logic and data buffer when TXACT is asserted
- The receive FIFO refers to the receive logic and data buffer when RXACT is asserted

- Transmit FIFO:
Data can be written to the transmit FIFO through the APB2 interface when the SDIO is enabled for transmission.

The transmit FIFO is accessible via 32 sequential addresses. The transmit FIFO contains a data output register that holds the data word pointed to by the read pointer. When the data path subunit has loaded its shift register, it increments the read pointer and drives new data out.

If the transmit FIFO is disabled, all status flags are deasserted. The data path subunit asserts TXACT when it transmits data.

Table 156. Transmit FIFO status flags

| Flag | Description |
|---|---|
| TXFIFOF | Set to high when all 32 transmit FIFO words contain valid data. |
| TXFIFOE | Set to high when the transmit FIFO does not contain valid data. |
| TXFIFOHE | Set to high when 8 or more transmit FIFO words are empty. This flag can be used as a DMA request. |
| TXDAVL | Set to high when the transmit FIFO contains valid data. This flag is the inverse of the TXFIFOE flag. |
| TXUNDERR | Set to high when an underrun error occurs. This flag is cleared by writing to the SDIO Clear register. |


- Receive FIFO
When the data path subunit receives a word of data, it drives the data on the write databus. The write pointer is incremented after the write operation completes. On the read side, the contents of the FIFO word pointed to by the current value of the read pointer is driven onto the read databus. If the receive FIFO is disabled, all status flags are deasserted, and the read and write pointers are reset. The data path subunit asserts RXACT when it receives data. Table 157 lists the receive FIFO status flags. The receive FIFO is accessible via 32 sequential addresses.

1032/1749
RM0090 Rev 18

--- Page 58 ---

RM0090
Secure digital input/output interface (SDIO)

**Table 157. Receive FIFO status flags**

| Flag | Description |
|---|---|
| RXFIFOF | Set to high when all 32 receive FIFO words contain valid data |
| RXFIFOE | Set to high when the receive FIFO does not contain valid data. |
| RXFIFOHF | Set to high when 8 or more receive FIFO words contain valid data. This flag can be used as a DMA request. |
| RXDAVL | Set to high when the receive FIFO is not empty. This flag is the inverse of the RXFIFOE flag. |
| RXOVERRR | Set to high when an overrun error occurs. This flag is cleared by writing to the SDIO Clear register. |


**31.3.2 SDIO APB2 interface**

The APB2 interface generates the interrupt and DMA requests, and accesses the SDIO adapter registers and the data FIFO. It consists of a data path, register decoder, and interrupt/DMA logic.

**SDIO interrupts**

The interrupt logic generates an interrupt request signal that is asserted when at least one of the selected status flags is high. A mask register is provided to allow selection of the conditions that will generate an interrupt. A status flag generates the interrupt request if a corresponding mask flag is set.

**SDIO/DMA interface - procedure for data transfers between the SDIO and memory**

In the example shown, the transfer is from the SDIO host controller to an MMC (512 bytes using CMD24 (WRITE_BLOCK)). The SDIO FIFO is filled by data stored in a memory using the DMA controller.

1. Do the card identification process
2. Increase the SDIO\_CK frequency
3. Select the card by sending CMD7
4. Configure the DMA2 as follows:
    a) Enable DMA2 controller and clear any pending interrupts.
    b) Program the DMA2\_Stream3 or DMA2\_Stream6 Channel4 source address register with the memory location's base address and DMA2\_Stream3 or

STI
RM0090 Rev 18
1033/1749

--- Page 59 ---

Secure digital input/output interface (SDIO)
RM0090

DMA2_Stream6 Channel4 destination address register with the SDIO_FIFO register address.
c) Program DMA2_Stream3 or DMA2_Stream6 Channel4 control register (memory increment, not peripheral increment, peripheral and source width is word size).
d) Program DMA2_Stream3 or DMA2_Stream6 Channel4 to select the peripheral as flow controller (set PFCTRL bit in DMA_S3CR or DMA_S6CR configuration register).
e) Configure the incremental burst transfer to 4 beats (at least from peripheral side) in DMA2_Stream3 or DMA2_Stream6 Channel4.
f) Enable DMA2_Stream3 or DMA2_Stream6 Channel4
5. Send CMD24 (WRITE_BLOCK) as follows:
a) Program the SDIO data length register (SDIO data timer register should be already programmed before the card identification process).
b) Program the SDIO argument register with the address location of the card where data is to be transferred.
c) Program the SDIO command register: CmdIndex with 24 (WRITE_BLOCK); WaitResp with '1' (SDIO card host waits for a response); CPSMEN with '1' (SDIO card host enabled to send a command). Other fields are at their reset value.
d) Wait for SDIO_STA[6] = CMDREND interrupt, then program the SDIO data control register: DTEN with '1' (SDIO card host enabled to send data); DTDIR with '0' (from controller to card); DTMODE with '0' (block data transfer); DMAEN with '1' (DMA enabled); DBLOCKSIZE with 0x9 (512 bytes). Other fields are don't care.
e) Wait for SDIO_STA[10] = DBCKEND.
6. Check that no channels are still enabled by polling the DMA Enabled Channel Status register.


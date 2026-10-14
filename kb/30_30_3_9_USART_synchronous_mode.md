### 30.3.9 USART synchronous mode

The synchronous mode is selected by writing the CLKEN bit in the USART_CR2 register to 1. In synchronous mode, the following bits must be kept cleared:

* LINEN bit in the USART_CR2 register,
* SCEN, HDSEL and IREN bits in the USART_CR3 register.

The USART allows the user to control a bidirectional synchronous serial communications in master mode. The CK pin is the output of the USART transmitter clock. No clock pulses are sent to the CK pin during start bit and stop bit. Depending on the state of the LBCL bit in the USART_CR2 register clock pulses will or will not be generated during the last valid data bit (address mark). The CPOL bit in the USART_CR2 register allows the user to select the clock polarity, and the CPHA bit in the USART_CR2 register allows the user to select the phase of the external clock (see Figure 307, Figure 308 & Figure 309).

During the Idle state, preamble and send break, the external CK clock is not activated.

In synchronous mode the USART transmitter works exactly like in asynchronous mode. But as CK is synchronized with TX (according to CPOL and CPHA), the data on TX is synchronous.

In this mode the USART receiver works in a different manner compared to the asynchronous mode. If RE=1, the data is sampled on CK (rising or falling edge, depending on CPOL and CPHA), without any oversampling. A setup and a hold time must be respected (which depends on the baud rate: 1/16 bit time).

Note: The CK pin works in conjunction with the TX pin. Thus, the clock is provided only if the transmitter is enabled (TE=1) and a data is being transmitted (the data register USART_DR

994/1749 RM0090 Rev 18 ST

--- Page 20 ---

RM0090 Universal synchronous asynchronous receiver transmitter (USART)

has been written). This means that it is not possible to receive a synchronous data without transmitting data.

The LBCL, CPOL and CPHA bits have to be selected when both the transmitter and the receiver are disabled (TE=RE=0) to ensure that the clock pulses function correctly. These bits should not be changed while the transmitter or the receiver is enabled.

It is advised that TE and RE are set in the same instruction in order to minimize the setup and the hold time of the receiver.

The USART supports master mode only: it cannot receive or send data related to an input clock (CK is always an output).

Figure 307. USART example of synchronous transmission

USART
RX
TX
CK
Data out
Data in
Synchronous device
(e.g. slave SPI)
Clock
MSv31158V2

Figure 308. USART data clock timing diagram (M=0)

Idle or preceding transmission
Start
M=0 (8 data bits)
Idle or next Stop transmission

Clock (CPOL=0, CPHA=0)
Clock (CPOL=0, CPHA=1)
Clock (CPOL=1, CPHA=0)
Clock (CPOL=1, CPHA=1)

Data on TX (from master)
Start LSB 0 1 2 3 4 5 6 7 MSB Stop

Data on RX (from slave)
LSB 0 1 2 3 4 5 6 7 MSB

Capture strobe

*LBCL bit controls last data pulse
MSv31159V1

STI
RM0090 Rev 18
995/1749

--- Page 21 ---

**Universal synchronous asynchronous receiver transmitter (USART)**
RM0090

**Figure 309. USART data clock timing diagram (M=1)**

Idle or preceding transmission
Start transmission
M=1 (9 data bits)
Stop
Idle or next transmission

Clock (CPOL=0, CPHA=0)
Clock (CPOL=0, CPHA=1)
Clock (CPOL=1, CPHA=0)
Clock (CPOL=1, CPHA=1)

Data on TX (from master)
Start LSB 0 1 2 3 4 5 6 7 8 MSB Stop

Data on RX (from slave)
LSB 0 1 2 3 4 5 6 7 8 MSB

Capture strobe

*LBCL bit controls last data pulse

MSv31160V1

**Figure 310. RX data setup/hold time**

CK
(capture strobe on CK rising edge in this example)

Data on RX (from slave)
Valid DATA bit

tSETUP tHOLD

tSETUP=tHOLD 1/16 bit time

MSv31161V2

**Note:** The function of CK is different in Smartcard mode. Refer to the Smartcard mode chapter for more details.

### 30.3.10 Single-wire half-duplex communication

The single-wire half-duplex mode is selected by setting the HDSEL bit in the USART_CR3 register. In this mode, the following bits must be kept cleared:

* LINEN and CLKEN bits in the USART_CR2 register,
* SCEN and IREN bits in the USART_CR3 register.

The USART can be configured to follow a single-wire half-duplex protocol where the TX and RX lines are internally connected. The selection between half- and full-duplex communication is made with a control bit ‘HALF DUPLEX SEL’ (HDSEL in USART_CR3).

996/1749
RM0090 Rev 18

--- Page 22 ---

RM0090 Universal synchronous asynchronous receiver transmitter (USART)

As soon as HDSEL is written to 1:
* the TX and RX lines are internally connected
* the RX pin is no longer used
* the TX pin is always released when no data is transmitted. Thus, it acts as a standard I/O in idle or in reception. It means that the I/O must be configured so that TX is configured as floating input (or output high open-drain) when not driven by the USART.

Apart from this, the communications are similar to what is done in normal USART mode. The conflicts on the line must be managed by the software (by the use of a centralized arbiter, for instance). In particular, the transmission is never blocked by hardware and continue to occur as soon as a data is written in the data register while the TE bit is set.


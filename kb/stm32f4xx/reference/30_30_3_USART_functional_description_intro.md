### 30.3 USART functional description

The interface is externally connected to another device by three pins (see Figure 296). Any USART bidirectional communication requires a minimum of two pins: Receive Data In (RX) and Transmit Data Out (TX):

RX: Receive Data Input is the serial data input. Oversampling techniques are used for data recovery by discriminating between valid incoming data and noise.

TX: Transmit Data Output. When the transmitter is disabled, the output pin returns to its I/O port configuration. When the transmitter is enabled and nothing is to be transmitted, the TX pin is at high level. In single-wire and smartcard modes, this I/O is used to transmit and receive the data (at USART level, data are then received on SW_RX).

966/1749 RM0090 Rev 18 ST

--- Page 187 ---

RM0090
Universal synchronous asynchronous receiver transmitter (USART)

Through these pins, serial data is transmitted and received in normal USART mode as frames comprising:

*   An Idle Line prior to transmission or reception
*   A start bit
*   A data word (8 or 9 bits) least significant bit first
*   0.5, 1, 1.5, 2 Stop bits indicating that the frame is complete
*   This interface uses a fractional baud rate generator - with a 12-bit mantissa and 4-bit fraction
*   A status register (USART_SR)
*   Data Register (USART_DR)
*   A baud rate register (USART_BRR) - 12-bit mantissa and 4-bit fraction.
*   A Guardtime Register (USART_GTPR) in case of Smartcard mode.

Refer to Section 30.6: USART registers on page 1007 for the definitions of each bit.

The following pin is required to interface in synchronous mode:

*   **CK**: Transmitter clock output. This pin outputs the transmitter data clock for synchronous transmission corresponding to SPI master mode (no clock pulses on start bit and stop bit, and a software option to send a clock pulse on the last data bit). In parallel data can be received synchronously on RX. This can be used to control peripherals that have shift registers (e.g. LCD drivers). The clock phase and polarity are software programmable. In smartcard mode, CK can provide the clock to the smartcard.

The following pins are required in Hardware flow control mode:

*   **CTS**: Clear To Send blocks the data transmission at the end of the current transfer when high
*   **RTS**: Request to send indicates that the USART is ready to receive a data (when low).

ST
RM0090 Rev 18
967/1749

--- Page 188 ---

Universal synchronous asynchronous receiver transmitter (USART)
RM0090

Figure 296. USART block diagram

PWRDATA
Write
(CPU or DMA)
Transmit data register (TDR)
Transmit Shift Register
Read
(CPU or DMA)
Receive data register (RDR)
Receive Shift Register
(Data register) DR
PRDATA

TX
RX
SW_RX
IrDA
SIR
ENDEC
block
IRD A_OUT
IRD A_IN

GTPR
GT
PSC
SCLK control
CK

CR3
DMAT DMAR SCEN NACK HD IRLP IREN

CR2
USART Address

CR2
LINE STOP[1:0] CKEN CPOL CPHA LBCL

CR1
UE M WAKE PCE PS PEIE

RTS
CTS
Hardware
flow
controller

Transmit control
Wakeup
unit
Receiver control
Receiver
clock

CR1
TXEIE TCIE RXNE IE IDLE IE TE RE RWU SBK

SR
CTS LBD TXE TC RXNE IDLE ORE NF FE PE

USART
interrupt
control

Transmitter
clock
/ [8 x (2 - OVER8)]
SAMPLING
DIVIDER
fPCLKx(x=1,2)
/USARTDIV

CR1
OVER8

USART_BRR
TE
Transmitter rate
control
DIV_Mantissa DIV_Fraction
15 4 0
Receiver rate
control
RE
Conventional baudrate generator

USARTDIV = DIV_Mantissa + (DIV_Fraction / 8 x (2 – OVER8))
ai16099b

968/1749
RM0090 Rev 18

--- Page 189 ---

RM0090 Universal synchronous asynchronous receiver transmitter (USART)

### 30.3.1 USART character description

Word length may be selected as being either 8 or 9 bits by programming the M bit in the USART_CR1 register (see Figure 297).

The TX pin is in low state during the start bit. It is in high state during the stop bit.

An *Idle character* is interpreted as an entire frame of “1”s followed by the start bit of the next frame which contains data (The number of “1”s will include the number of stop bits).

A *Break character* is interpreted on receiving “0”s for a frame period. At the end of the break frame the transmitter inserts either 1 or 2 stop bits (logic “1” bit) to acknowledge the start bit.

Transmission and reception are driven by a common baud rate generator, the clock for each is generated when the enable bit is set respectively for the transmitter and receiver.

The details of each block is given below.

**Figure 297. Word length programming**

9-bit word length (M bit is set), 1 Stop bit

Data frame
Possible Parity bit
Next data frame
Start bit Bit0 Bit1 Bit2 Bit3 Bit4 Bit5 Bit6 Bit7 Bit8 Stop bit Next Start bit
Clock
** LBCL bit controls last data clock pulse
Idle frame
Break frame
Start bit
Stop bit Start bit

8-bit word length (M bit is reset), 1 Stop bit

Data frame
Possible Parity bit
Next data frame
Start bit Bit0 Bit1 Bit2 Bit3 Bit4 Bit5 Bit6 Bit7 Stop bit Next Start bit
Clock
** LBCL bit controls last data clock pulse
Idle frame
Break frame
Start bit
Stop bit Start bit

MS19822V2

STI
RM0090 Rev 18
969/1749

--- Page 190 ---

Universal synchronous asynchronous receiver transmitter (USART) RM0090


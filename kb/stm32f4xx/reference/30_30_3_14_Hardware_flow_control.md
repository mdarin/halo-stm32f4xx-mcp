**30.3.14 Hardware flow control**

It is possible to control the serial data flow between 2 devices by using the CTS input and
the RTS output. The Figure 317 shows how to connect 2 devices in this mode:

**Figure 317. Hardware flow control between 2 USARTs**

USART 1
TX circuit
RX circuit
TX
CTS
RX
RTS

USART 2
RX circuit
TX circuit
RX
RTS
TX
CTS

MSv31169V2

RTS and CTS flow control can be enabled independently by writing respectively RTSE and
CTSE bits to 1 (in the USART_CR3 register).

ST
RM0090 Rev 18
1003/1749

--- Page 29 ---

Universal synchronous asynchronous receiver transmitter (USART) RM0090

**RTS flow control**

If the RTS flow control is enabled (RTSE=1), then RTS is asserted (tied low) as long as the USART receiver is ready to receive a new data. When the receive register is full, RTS is deasserted, indicating that the transmission is expected to stop at the end of the current frame. *Figure 318* shows an example of communication with RTS flow control enabled.

**Figure 318. RTS flow control**

RX Start bit Data 1 Stop bit Idle Start bit Data 2 Stop bit
RTS RXNE Data 1 read Data 2 can now be transmitted RXNE
MSv31168V2

**CTS flow control**

If the CTS flow control is enabled (CTSE=1), then the transmitter checks the CTS input before transmitting the next frame. If CTS is asserted (tied low), then the next data is transmitted (assuming that a data is to be transmitted, in other words, if TXE=0), else the transmission does not occur. When CTS is deasserted during a transmission, the current transmission is completed before the transmitter stops.

When CTSE=1, the CTSIF status bit is automatically set by hardware as soon as the CTS input toggles. It indicates when the receiver becomes ready or not ready for communication. An interrupt is generated if the CTSIE bit in the USART_CR3 register is set. The figure below shows an example of communication with CTS flow control enabled.

1004/1749 RM0090 Rev 18 ST

--- Page 30 ---

RM0090
Universal synchronous asynchronous receiver transmitter (USART)

**Figure 319. CTS flow control**

CTS
Transmit data register
TDR
TX
Data 2
empty
Data 3
empty
Data 1
Stop bit
Start bit
Data 2
Stop bit
Idle
Start bit
Data 3
CTS
CTS
Writing data 3 in TDR
Transmission of Data 3 is delayed until CTS = 0
MSv31167V2

Note: **Special behavior of break frames**: when the CTS flow is enabled, the transmitter does not check the CTS input state to send a break.

ST
RM0090 Rev 18
1005/1749

--- Page 31 ---

Universal synchronous asynchronous receiver transmitter (USART) RM0090

### 30.4 USART interrupts

Table 147. USART interrupt requests

| Interrupt event | Event flag | Enable control bit |
|---|---|---|
| Transmit Data Register Empty | TXE | TXEIE |
| CTS flag | CTS | CTSIE |
| Transmission Complete | TC | TCIE |
| Received Data Ready to be Read | RXNE | RXNEIE |
| Overrun Error Detected | ORE |  |
| Idle Line Detected | IDLE | IDLEIE |
| Parity Error | PE | PEIE |
| Break Flag | LBD | LBDIE |
| Noise Flag, Overrun error and Framing Error in multibuffer communication | NF or ORE or FE | EIE |


The USART interrupt events are connected to the same interrupt vector (see Figure 320).

* During transmission: Transmission Complete, Clear to Send or Transmit Data Register empty interrupt.
* While receiving: Idle Line detection, Overrun error, Receive Data register not empty, Parity error, LIN break detection, Noise Flag (only in multi buffer communication) and Framing Error (only in multi buffer communication).

These events generate an interrupt if the corresponding Enable Control Bit is set.

Figure 320. USART interrupt mapping diagram

TC
TCIE
TXE
TXEIE
CTSIF
CTSIE
IDLE
IDLEIE
RXNEIE
ORE
RXNEIE
RXNE
PE
PEIE
LBD
LBDIE
FE
NE
ORE
EIE
DMAR
USART interrupt
MSv42089V1

1006/1749 RM0090 Rev 18

--- Page 32 ---

RM0090
Universal synchronous asynchronous receiver transmitter (USART)

### 30.5 USART mode configuration

Table 148. USART mode configuration(1)

| USART modes | USART 1 | USART 2 | USART 3 | UART4 | UART5 | USART 6 |
|---|---|---|---|---|---|---|
| Asynchronous mode | X |  |  |  |  |  |
| Hardware flow control | X | NA | X |  |  |  |
| Multibuffer communication (DMA) | X |  |  |  |  |  |
| Multiprocessor communication | X |  |  |  |  |  |
| Synchronous | X | NA | X |  |  |  |
| Smartcard | X | NA | X |  |  |  |
| Half-duplex (single-wire mode) | X |  |  |  |  |  |
| IrDA | X |  |  |  |  |  |
| LIN | X |  |  |  |  |  |


1. X = supported; NA = not applicable.


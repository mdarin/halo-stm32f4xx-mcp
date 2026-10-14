### 30.3.11 Smartcard

The Smartcard mode is selected by setting the SCEN bit in the USART_CR3 register. In smartcard mode, the following bits must be kept cleared:
* LINEN bit in the USART_CR2 register,
* HDSEL and IREN bits in the USART_CR3 register.

Moreover, the CLKEN bit may be set in order to provide a clock to the smartcard.

The Smartcard interface is designed to support asynchronous protocol Smartcards as defined in the ISO 7816-3 standard. The USART should be configured as:
* 8 bits plus parity: where M=1 and PCE=1 in the USART_CR1 register
* 1.5 stop bits when transmitting and receiving: where STOP=11 in the USART_CR2 register.

**Note:** It is also possible to choose 0.5 stop bit for receiving but it is recommended to use 1.5 stop bits for both transmitting and receiving to avoid switching between the two configurations.

Figure 311 shows examples of what can be seen on the data line with and without parity error.

**Figure 311. ISO 7816-3 asynchronous protocol**

Without Parity error
Guard time
S 0 1 2 3 4 5 6 7 P
Start bit

With Parity error
Guard time
S 0 1 2 3 4 5 6 7 P
Start bit
Line pulled low by receiver during stop in case of parity error

MSv31162V1

When connected to a Smartcard, the TX output of the USART drives a bidirectional line that is also driven by the Smartcard. The TX pin must be configured as open-drain.

Smartcard is a single wire half duplex communication protocol.
* Transmission of data from the transmit shift register is guaranteed to be delayed by a minimum of 1/2 baud clock. In normal operation a full transmit shift register will start

ST
RM0090 Rev 18 997/1749

--- Page 23 ---

**Universal synchronous asynchronous receiver transmitter (USART)** RM0090

shifting on the next baud clock edge. In Smartcard mode this transmission is further delayed by a guaranteed 1/2 baud clock.

*   If a parity error is detected during reception of a frame programmed with a 0.5 or 1.5 stop bit period, the transmit line is pulled low for a baud clock period after the completion of the receive frame. This is to indicate to the Smartcard that the data transmitted to USART has not been correctly received. This NACK signal (pulling transmit line low for 1 baud clock) will cause a framing error on the transmitter side (configured with 1.5 stop bits). The application can handle re-sending of data according to the protocol. A parity error is ‘NACK’ed by the receiver if the NACK control bit is set, otherwise a NACK is not transmitted.
*   The assertion of the TC flag can be delayed by programming the Guard Time register. In normal operation, TC is asserted when the transmit shift register is empty and no further transmit requests are outstanding. In Smartcard mode an empty transmit shift register triggers the guard time counter to count up to the programmed value in the Guard Time register. TC is forced low during this time. When the guard time counter reaches the programmed value TC is asserted high.
*   The de-assertion of TC flag is unaffected by Smartcard mode.
*   If a framing error is detected on the transmitter end (due to a NACK from the receiver), the NACK will not be detected as a start bit by the receive block of the transmitter. According to the ISO protocol, the duration of the received NACK can be 1 or 2 baud clock periods.
*   On the receiver side, if a parity error is detected and a NACK is transmitted the receiver will not detect the NACK as a start bit.

**Note:** A break character is *not significant* in Smartcard mode. A 0x00 data with a framing error will be treated as data and not as a break.

No Idle frame is transmitted when toggling the TE bit. The Idle frame (as defined for the other configurations) is not defined by the ISO protocol.

**Figure 312** details how the NACK signal is sampled by the USART. In this example the USART is transmitting a data and is configured with 1.5 stop bits. The receiver part of the USART is enabled in order to check the integrity of the data and the NACK signal.

**Figure 312. Parity error detection using the 1.5 stop bits**

Bit 7 Parity bit 1.5 Stop bit
1 bit time 1.5 bit time
Sampling at 8th, 9th, 10th Sampling at 8th, 9th, 10th
0.5 bit time
Sampling at 8th, 9th, 10th Sampling at 8th, 9th, 10th
MSv31163V1

The USART can provide a clock to the smartcard through the CK output. In smartcard mode, CK is not associated to the communication but is simply derived from the internal peripheral input clock through a 5-bit prescaler. The division ratio is configured in the

998/1749 RM0090 Rev 18 ST

--- Page 24 ---

RM0090 Universal synchronous asynchronous receiver transmitter (USART)

prescaler register USART_GTPR. CK frequency can be programmed from fCK/2 to fCK/62, where fCK is the peripheral input clock.


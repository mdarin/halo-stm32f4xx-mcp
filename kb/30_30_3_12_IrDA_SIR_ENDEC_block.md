### 30.3.12 IrDA SIR ENDEC block

The IrDA mode is selected by setting the IREN bit in the USART_CR3 register. In IrDA mode, the following bits must be kept cleared:

* LINEN, STOP and CLKEN bits in the USART_CR2 register,
* SCEN and HDSEL bits in the USART_CR3 register.

The IrDA SIR physical layer specifies use of a Return to Zero, Inverted (RZI) modulation scheme that represents logic 0 as an infrared light pulse (see Figure 313).

The SIR Transmit encoder modulates the Non Return to Zero (NRZ) transmit bit stream output from USART. The output pulse stream is transmitted to an external output driver and infrared LED. USART supports only bit rates up to 115.2Kbps for the SIR ENDEC. In normal mode the transmitted pulse width is specified as 3/16 of a bit period.

The SIR receive decoder demodulates the return-to-zero bit stream from the infrared detector and outputs the received NRZ serial bit stream to USART. The decoder input is normally HIGH (marking state) in the Idle state. The transmit encoder output has the opposite polarity to the decoder input. A start bit is detected when the decoder input is low.

* IrDA is a half duplex communication protocol. If the Transmitter is busy (i.e. the USART is sending data to the IrDA encoder), any data on the IrDA receive line will be ignored by the IrDA decoder and if the Receiver is busy (USART is receiving decoded data from the USART), data on the TX from the USART to IrDA will not be encoded by IrDA. While receiving data, transmission should be avoided as the data to be transmitted could be corrupted.
* A '0' is transmitted as a high pulse and a '1' is transmitted as a '0'. The width of the pulse is specified as 3/16th of the selected bit period in normal mode (see Figure 314).
* The SIR decoder converts the IrDA compliant receive signal into a bit stream for USART.
* The SIR receive logic interprets a high state as a logic one and low pulses as logic zeros.
* The transmit encoder output has the opposite polarity to the decoder input. The SIR output is in low state when Idle.
* The IrDA specification requires the acceptance of pulses greater than 1.41 us. The acceptable pulse width is programmable. Glitch detection logic on the receiver end filters out pulses of width less than 2 PSC periods (PSC is the prescaler value programmed in the IrDA low-power Baud Register, USART_GTPR). Pulses of width less than 1 PSC period are always rejected, but those of width greater than one and less than two periods may be accepted or rejected, those greater than 2 periods will be accepted as a pulse. The IrDA encoder/decoder doesn't work when PSC=0.
* The receiver can communicate with a low-power transmitter.
* In IrDA mode, the STOP bits in the USART_CR2 register must be configured to "1 stop bit".

ST
RM0090 Rev 18 999/1749

--- Page 25 ---

Universal synchronous asynchronous receiver transmitter (USART)
RM0090

**IrDA low-power mode**

**Transmitter:**
In low-power mode the pulse width is not maintained at 3/16 of the bit period. Instead, the width of the pulse is 3 times the low-power baud rate which can be a minimum of 1.42 MHz. Generally this value is 1.8432 MHz (1.42 MHz < PSC < 2.12 MHz). A low-power mode programmable divisor divides the system clock to achieve this value.

**Receiver:**
Receiving in low-power mode is similar to receiving in normal mode. For glitch detection the USART should discard pulses of duration shorter than 1/PSC. A valid low is accepted only if its duration is greater than 2 periods of the IrDA low-power Baud clock (PSC value in USART_GTPR).

**Note:**
A pulse of width less than two and greater than one PSC period(s) may or may not be rejected.
The receiver set up time should be managed by software. The IrDA physical layer specification specifies a minimum of 10 ms delay between transmission and reception (IrDA is a half duplex protocol).

**Figure 313. IrDA SIR ENDEC- block diagram**

TX
USART
RX
SIREN
OR
USART_TX
IrDA_OUT
SIR Transmit Encoder
SIR Receive DEcoder
IrDA_IN
USART_RX
MSv31164V2

**Figure 314. IrDA data modulation (3/16) -Normal mode**

TX
Start bit
0 1 0 1 0 0 1 1 0 1
Stop bit
IrDA_OUT
IrDA_IN
Bit period
3/16
RX
0 1 0 1 0 0 1 1 0 1
MSv31165V1

1000/1749
RM0090 Rev 18
ST

--- Page 26 ---

RM0090 Universal synchronous asynchronous receiver transmitter (USART)


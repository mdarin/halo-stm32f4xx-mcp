### 30.3.8 LIN (local interconnection network) mode

The LIN mode is selected by setting the LINEN bit in the USART_CR2 register. In LIN mode, the following bits must be kept cleared:

*   STOP[1:0] and CLKEN in the USART_CR2 register
*   SCEN, HDSEL and IREN in the USART_CR3 register.

#### LIN transmission

The same procedure explained in Section 30.3.2 has to be applied for LIN Master transmission than for normal USART transmission with the following differences:

*   Clear the M bit to configure 8-bit word length.
*   Set the LINEN bit to enter LIN mode. In this case, setting the SBK bit sends 13 '0 bits as a break character. Then a bit of value '1 is sent to allow the next start detection.

#### LIN reception

A break detection circuit is implemented on the USART interface. The detection is totally independent from the normal USART receiver. A break can be detected whenever it occurs, during Idle state or during a frame.

When the receiver is enabled (RE=1 in USART_CR1), the circuit looks at the RX input for a start signal. The method for detecting start bits is the same when searching break characters or data. After a start bit has been detected, the circuit samples the next bits exactly like for the data (on the 8th, 9th and 10th samples). If 10 (when the LBDL = 0 in USART_CR2) or 11 (when LBDL=1 in USART_CR2) consecutive bits are detected as '0, and are followed by a delimiter character, the LBD flag is set in USART_SR. If the LBDIE bit=1, an interrupt is generated. Before validating the break, the delimiter is checked for as it signifies that the RX line has returned to a high level.

If a '1 is sampled before the 10 or 11 have occurred, the break detection circuit cancels the current detection and searches for a start bit again.

If the LIN mode is disabled (LINEN=0), the receiver continues working as normal USART, without taking into account the break detection.

If the LIN mode is enabled (LINEN=1), as soon as a framing error occurs (i.e. stop bit detected at '0, which will be the case for any break frame), the receiver stops until the break detection circuit receives either a '1, if the break word was not complete, or a delimiter character if a break has been detected.

The behavior of the break detector state machine and the break flag is shown on the Figure 305: Break detection in LIN mode (11-bit break length - LBDL bit is set) on page 993.

Examples of break frames are given on Figure 306: Break detection in LIN mode vs. Framing error detection on page 994.

992/1749 RM0090 Rev 18

--- Page 18 ---

RM0090
Universal synchronous asynchronous receiver transmitter (USART)

Figure 305. Break detection in LIN mode (11-bit break length - LBDL bit is set)

**Case 1: break signal not long enough => break discarded, LBD is not set**

RX line
Break frame

Capture strobe

Break state machine


Read samples
0 0 0 0 0 0 0 0 0 0 1

**Case 2: break signal just long enough => break detected, LBD is set**

RX line
Break frame

Capture strobe
Delimiter is immediate

Break state machine


Read samples
0 0 0 0 0 0 0 0 0 0 0

LBD

**Case 3: break signal long enough => break detected, LBD is set**

RX line
Break frame

Capture strobe

Break state machine


Read samples
0 0 0 0 0 0 0 0 0 0 0

LBD

MSv40883V1

ST
RM0090 Rev 18
993/1749

--- Page 19 ---

Universal synchronous asynchronous receiver transmitter (USART) RM0090

Figure 306. Break detection in LIN mode vs. Framing error detection

Case 1: break occurring after an Idle

RX line data 1 IDLE BREAK data 2 (0x55) data 3 (header)
1 data time 1 data time

RXNE /FE

LBDF

Case 2: break occurring while data is being received

RX line data 1 data2 BREAK data 2 (0x55) data 3 (header)
1 data time 1 data time

RXNE /FE

LBDF

MSv31157V1


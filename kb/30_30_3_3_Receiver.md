**30.3.3 Receiver**
The USART can receive data words of either 8 or 9 bits depending on the M bit in the USART_CR1 register.

**Start bit detection**
The start bit detection sequence is the same when oversampling by 16 or by 8.
In the USART, the start bit is detected when a specific sequence of samples is recognized.
This sequence is: 1 1 1 0 X 0 X 0 X 0 0 0 0.

Figure 300. Start bit detection when oversampling by 16 or 8

Note:
If the sequence is not complete, the start bit detection aborts and the receiver returns to the idle state (no flag is set) where it waits for a falling edge.
The start bit is confirmed (RXNE flag set, interrupt generated if RXNEIE=1) if the 3 sampled bits are at 0 (first sampling on the 3rd, 5th and 7th bits finds the 3 bits at 0 and second sampling on the 8th, 9th and 10th bits also finds the 3 bits at 0).
The start bit is validated (RXNE flag set, interrupt generated if RXNEIE=1) but the NE noise flag is set if, for both samplings, at least 2 out of the 3 sampled bits are at 0 (sampling on the

STI
RM0090 Rev 18
973/1749

--- Page 194 ---

Universal synchronous asynchronous receiver transmitter (USART) RM0090

3rd, 5th and 7th bits and sampling on the 8th, 9th and 10th bits). If this condition is not met, the start detection aborts and the receiver returns to the idle state (no flag is set).
If, for one of the samplings (sampling on the 3rd, 5th and 7th bits or sampling on the 8th, 9th and 10th bits), 2 out of the 3 bits are found at 0, the start bit is validated but the NE noise flag bit is set.

**Character reception**

During an USART reception, data shifts in least significant bit first through the RX pin. In this mode, the USART_DR register consists of a buffer (RDR) between the internal bus and the received shift register.

Procedure:
1. Enable the USART by writing the UE bit in USART_CR1 register to 1.
2. Program the M bit in USART_CR1 to define the word length.
3. Program the number of stop bits in USART_CR2.
4. Select DMA enable (DMAR) in USART_CR3 if multibuffer communication is to take place. Configure the DMA register as explained in multibuffer communication. STEP 3
5. Select the desired baud rate using the baud rate register USART_BRR
6. Set the RE bit USART_CR1. This enables the receiver which begins searching for a start bit.

When a character is received
* The RXNE bit is set. It indicates that the content of the shift register is transferred to the RDR. In other words, data has been received and can be read (as well as its associated error flags).
* An interrupt is generated if the RXNEIE bit is set.
* The error flags can be set if a frame error, noise or an overrun error has been detected during reception.
* In multibuffer, RXNE is set after every byte received and is cleared by the DMA read to the Data Register.
* In single buffer mode, clearing the RXNE bit is performed by a software read to the USART_DR register. The RXNE flag can also be cleared by writing a zero to it. The RXNE bit must be cleared before the end of the reception of the next character to avoid an overrun error.

Note: The RE bit should not be reset while receiving data. If the RE bit is disabled during reception, the reception of the current byte will be aborted.

**Break character**

When a break character is received, the USART handles it as a framing error.

**Idle character**

When an idle frame is detected, there is the same procedure as a data received character plus an interrupt if the IDLEIE bit is set.

974/1749 RM0090 Rev 18

--- Page 195 ---

RM0090 Universal synchronous asynchronous receiver transmitter (USART)

**Overrun error**

An overrun error occurs when a character is received when RXNE has not been reset. Data can not be transferred from the shift register to the RDR register until the RXNE bit is cleared.

The RXNE flag is set after every byte received. An overrun error occurs if RXNE flag is set when the next data is received or the previous DMA request has not been serviced. When an overrun error occurs:

*   The ORE bit is set.
*   The RDR content will not be lost. The previous data is available when a read to USART_DR is performed.
*   The shift register will be overwritten. After that point, any data received during overrun is lost.
*   An interrupt is generated if either the RXNEIE bit is set or both the EIE and DMAR bits are set.
*   The ORE bit is reset by a read to the USART_SR register followed by a USART_DR register read operation.

**Note:** The ORE bit, when set, indicates that at least 1 data has been lost. There are two possibilities:

*   if RXNE=1, then the last valid data is stored in the receive register RDR and can be read,
*   if RXNE=0, then it means that the last valid data has already been read and thus there is nothing to be read in the RDR. This case can occur when the last valid data is read in the RDR at the same time as the new (and lost) data is received. It may also occur when the new data is received during the reading sequence (between the USART_SR register read access and the USART_DR read access).

**Selecting the proper oversampling method**

The receiver implements different user-configurable oversampling techniques (except in synchronous mode) for data recovery by discriminating between valid incoming data and noise.

The oversampling method can be selected by programming the OVER8 bit in the USART_CR1 register and can be either 16 or 8 times the baud rate clock (Figure 301 and Figure 302).

Depending on the application:

*   select oversampling by 8 (OVER8=1) to achieve higher speed (up to fPCLK/8). In this case the maximum receiver tolerance to clock deviation is reduced (refer to Section 30.3.5: USART receiver tolerance to clock deviation on page 988)
*   select oversampling by 16 (OVER8=0) to increase the tolerance of the receiver to clock deviations. In this case, the maximum speed is limited to maximum fPCLK/16

ST
RM0090 Rev 18
975/1749
--- Page 1 ---

**Universal synchronous asynchronous receiver transmitter (USART)** RM0090

Programming the ONEBIT bit in the USART_CR3 register selects the method used to evaluate the logic level. There are two options:

*   the majority vote of the three samples in the center of the received bit. In this case, when the 3 samples used for the majority vote are not equal, the NF bit is set
*   a single sample in the center of the received bit

Depending on the application:

*   select the three samples' majority vote method (ONEBIT=0) when operating in a noisy environment and reject the data when a noise is detected (refer to Figure 133) because this indicates that a glitch occurred during the sampling.
*   select the single sample method (ONEBIT=1) when the line is noise-free to increase the receiver's tolerance to clock deviations (see Section 30.3.5: USART receiver tolerance to clock deviation on page 988). In this case the NF bit will never be set.

When noise is detected in a frame:

*   The NF bit is set at the rising edge of the RXNE bit.
*   The invalid data is transferred from the Shift register to the USART_DR register.
*   No interrupt is generated in case of single byte communication. However this bit rises at the same time as the RXNE bit which itself generates an interrupt. In case of multibuffer communication an interrupt will be issued if the EIE bit is set in the USART_CR3 register.

The NF bit is reset by a USART_SR register read operation followed by a USART_DR register read operation.

**Note:** Oversampling by 8 is not available in the Smartcard, IrDA and LIN modes. In those modes, the OVER8 bit is forced to '0 by hardware.

**Figure 301. Data sampling when oversampling by 16**

RX line
Sample clock
sampled values
1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16
7/16
6/16
7/16
One bit time

MSv31152V1

976/1749 RM0090 Rev 18

--- Page 2 ---

RM0090
Universal synchronous asynchronous receiver transmitter (USART)

Figure 302. Data sampling when oversampling by 8

RX line
Sample clock (x8)
1 2 3 4 5 6 7 8
sampled values
3/8
One bit time
2/8
3/8
MSv31153V1

Table 133. Noise detection from sampled data
| Sampled value | NE status | Received bit value |
|---|---|---|
| 000 | 0 |  |
| 001 | 1 | 0 |
| 010 | 1 | 0 |
| 011 | 1 |  |
| 100 | 1 | 0 |
| 101 | 1 |  |
| 110 | 1 |  |
| 111 | 0 | 1 |


**Framing error**

A framing error is detected when:
The stop bit is not recognized on reception at the expected time, following either a de-synchronization or excessive noise.

When the framing error is detected:
* The FE bit is set by hardware
* The invalid data is transferred from the Shift register to the USART_DR register.
* No interrupt is generated in case of single byte communication. However this bit rises at the same time as the RXNE bit which itself generates an interrupt. In case of multibuffer communication an interrupt will be issued if the EIE bit is set in the USART_CR3 register.

The FE bit is reset by a USART_SR register read operation followed by a USART_DR register read operation.

ST
RM0090 Rev 18
977/1749

--- Page 3 ---

Universal synchronous asynchronous receiver transmitter (USART) RM0090

**Configurable stop bits during reception**

The number of stop bits to be received can be configured through the control bits of Control Register 2 - it can be either 1 or 2 in normal mode and 0.5 or 1.5 in Smartcard mode.

1. **0.5 stop bit (reception in Smartcard mode):** No sampling is done for 0.5 stop bit. As a consequence, no framing error and no break frame can be detected when 0.5 stop bit is selected.
2. **1 stop bit:** Sampling for 1 stop Bit is done on the 8th, 9th and 10th samples.
3. **1.5 stop bits (Smartcard mode):** When transmitting in smartcard mode, the device must check that the data is correctly sent. Thus the receiver block must be enabled (RE =1 in the USART_CR1 register) and the stop bit is checked to test if the smartcard has detected a parity error. In the event of a parity error, the smartcard forces the data signal low during the sampling - NACK signal-, which is flagged as a framing error. Then, the FE flag is set with the RXNE at the end of the 1.5 stop bit. Sampling for 1.5 stop bits is done on the 16th, 17th and 18th samples (1 baud clock period after the beginning of the stop bit). The 1.5 stop bit can be decomposed into 2 parts: one 0.5 baud clock period during which nothing happens, followed by 1 normal stop bit period during which sampling occurs halfway through. Refer to Section 30.3.11: Smartcard on page 997 for more details.
4. **2 stop bits:** Sampling for 2 stop bits is done on the 8th, 9th and 10th samples of the first stop bit. If a framing error is detected during the first stop bit the framing error flag will be set. The second stop bit is not checked for framing error. The RXNE flag will be set at the end of the first stop bit.


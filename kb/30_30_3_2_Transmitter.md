### 30.3.2 Transmitter

The transmitter can send data words of either 8 or 9 bits depending on the M bit status. When the transmit enable bit (TE) is set, the data in the transmit shift register is output on the TX pin and the corresponding clock pulses are output on the CK pin.

#### Character transmission

During an USART transmission, data shifts out least significant bit first on the TX pin. In this mode, the USART_DR register consists of a buffer (TDR) between the internal bus and the transmit shift register (see Figure 296).

Every character is preceded by a start bit which is a logic level low for one bit period. The character is terminated by a configurable number of stop bits.

The following stop bits are supported by USART: 0.5, 1, 1.5 and 2 stop bits.

**Note:** The TE bit should not be reset during transmission of data. Resetting the TE bit during the transmission will corrupt the data on the TX pin as the baud rate counters will get frozen. The current data being transmitted will be lost.

An idle frame will be sent after the TE bit is enabled.

#### Configurable stop bits

The number of stop bits to be transmitted with every character can be programmed in Control register 2, bits 13,12.

*   **1 stop bit:** This is the default value of number of stop bits.
*   **2 Stop bits:** This will be supported by normal USART, single-wire and modem modes.
*   **0.5 stop bit:** To be used when receiving data in Smartcard mode.
*   **1.5 stop bits:** To be used when transmitting and receiving data in Smartcard mode.

An idle frame transmission will include the stop bits.

A break transmission will be 10 low bits followed by the configured number of stop bits (when m = 0) and 11 low bits followed by the configured number of stop bits (when m = 1). It is not possible to transmit long breaks (break of length greater than 10/11 low bits).

970/1749 RM0090 Rev 18 ST

--- Page 191 ---

RM0090
Universal synchronous asynchronous receiver transmitter (USART)

Figure 298. Configurable stop bits

8-bit Word length (M bit is reset)
Data frame
Possible parity bit
Next data frame
Start Bit Bit0 Bit1 Bit2 Bit3 Bit4 Bit5 Bit6 Bit7 Stop bit Next start bit
CLOCK
** LBCL bit controls last data clock pulse

a) 1 Stop Bit

Data frame
Possible Parity Bit
Next data frame
Start Bit Bit0 Bit1 Bit2 Bit3 Bit4 Bit5 Bit6 Bit7 Next start bit
b) 1 1/2 stop Bits
1 1/2 stop bits

Data frame
Possible parity bit
Next data frame
Start Bit Bit0 Bit1 Bit2 Bit3 Bit4 Bit5 Bit6 Bit7 2 Stop Bits Next Start Bit
c) 2 Stop Bits

Data frame
Possible parity bit
Next Start Bit
Next data frame
Start Bit Bit0 Bit1 Bit2 Bit3 Bit4 Bit5 Bit6 Bit7 Next Start Bit
d) 1/2 Stop Bit
1/2 Stop Bit

MSv42088V1

Procedure:
1. Enable the USART by writing the UE bit in USART_CR1 register to 1.
2. Program the M bit in USART_CR1 to define the word length.
3. Program the number of stop bits in USART_CR2.
4. Select DMA enable (DMAT) in USART_CR3 if Multi buffer Communication is to take place. Configure the DMA register as explained in multibuffer communication.
5. Select the desired baud rate using the USART_BRR register.
6. Set the TE bit in USART_CR1 to send an idle frame as first transmission.
7. Write the data to send in the USART_DR register (this clears the TXE bit). Repeat this for each data to be transmitted in case of single buffer.
8. After writing the last data into the USART_DR register, wait until TC=1. This indicates that the transmission of the last frame is complete. This is required for instance when the USART is disabled or enters the Halt mode to avoid corrupting the last transmission.

**Single byte communication**

Clearing the TXE bit is always performed by a write to the data register.

The TXE bit is set by hardware and it indicates:
* The data has been moved from TDR to the shift register and the data transmission has started.
* The TDR register is empty.
* The next data can be written in the USART_DR register without overwriting the previous data.

This flag generates an interrupt if the TXEIE bit is set.

ST
RM0090 Rev 18
971/1749

--- Page 192 ---

**Universal synchronous asynchronous receiver transmitter (USART)** RM0090

When a transmission is taking place, a write instruction to the USART_DR register stores the data in the TDR register and which is copied in the shift register at the end of the current transmission.

When no transmission is taking place, a write instruction to the USART_DR register places the data directly in the shift register, the data transmission starts, and the TXE bit is immediately set.

If a frame is transmitted (after the stop bit) and the TXE bit is set, the TC bit goes high. An interrupt is generated if the TCIE bit is set in the USART_CR1 register.

After writing the last data into the USART_DR register, it is mandatory to wait for TC=1 before disabling the USART or causing the microcontroller to enter the low-power mode (see Figure 299: TC/TXE behavior when transmitting).

The TC bit is cleared by the following software sequence:
1. A read from the USART_SR register
2. A write to the USART_DR register

**Note:** The TC bit can also be cleared by writing a '0' to it. This clearing sequence is recommended only for Multibuffer communication.

**Figure 299. TC/TXE behavior when transmitting**

| MS48961V1 |  |  |  |  |
|---|---|---|---|---|
| TX line | Idle preamble | Frame 1 | Frame 2 | Frame 3 |
| TXE flag |  | set by hardware cleared by software | Set by hardware |  |
| USART_DR |  | F1 | F2 | F3 |
| TC flag |  |  |  | set by hardware |
| Software enables the USART |  |  |  |  |
| Software waits until TXE=1 and writes F1 into DR |  |  |  |  |
| Software waits until TXE=1 and writes F2 into DR |  |  |  |  |
| Software waits until TXE=1 and writes F1 into DR |  |  |  |  |
| TC is not set because TXE=0 |  |  |  |  |
| TC is not set because TXE=0 |  |  |  |  |
| TC is not set because TXE=1 |  |  |  |  |
| Software wait until TC=1 |  |  |  |  |


**Break characters**

Setting the SBK bit transmits a break character. The break frame length depends on the M bit (see Figure 297).

If the SBK bit is set to '1 a break character is sent on the TX line after completing the current character transmission. This bit is reset by hardware when the break character is completed (during the stop bit of the break character). The USART inserts a logic 1 bit at the end of the last break frame to guarantee the recognition of the start bit of the next frame.

**Note:** If the software resets the SBK bit before the commencement of break transmission, the break character will not be transmitted. For two consecutive breaks, the SBK bit should be set after the stop bit of the previous break.

972/1749 RM0090 Rev 18 ST

--- Page 193 ---

RM0090 Universal synchronous asynchronous receiver transmitter (USART)

**Idle characters**
Setting the TE bit drives the USART to send an idle frame before the first data frame.


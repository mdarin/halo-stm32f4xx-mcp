### 30.6 USART registers

Refer to Section 1.1: List of abbreviations for registers for registers for a list of abbreviations used in register descriptions.

The peripheral registers have to be accessed by half-words (16 bits) or words (32 bits).

#### 30.6.1 Status register (USART_SR)

Address offset: 0x00
Reset value: 0x0000 00C0

| Reserved |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|
| Reserved | CTSrc_w0 | LBDrc_w0 | TXEr | TCrc_w0 | RXNErc_w0 | IDLEr | OREr | NFr | FEr | PEr |


ST
RM0090 Rev 18
1007/1749

--- Page 33 ---

**Universal synchronous asynchronous receiver transmitter (USART)**

RM0090

Bits 31:10 Reserved, must be kept at reset value

**Bit 9 CTS: CTS flag**

This bit is set by hardware when the CTS input toggles, if the CTSE bit is set. It is cleared by software (by writing it to 0). An interrupt is generated if CTISIE=1 in the USART_CR3 register.

0: No change occurred on the CTS status line
1: A change occurred on the CTS status line

*Note: This bit is not available for UART4 & UART5.*

**Bit 8 LBD: LIN break detection flag**

This bit is set by hardware when the LIN break is detected. It is cleared by software (by writing it to 0). An interrupt is generated if LBDIE = 1 in the USART_CR2 register.

0: LIN Break not detected
1: LIN break detected

*Note: An interrupt is generated when LBD=1 if LBDIE=1*

**Bit 7 TXE: Transmit data register empty**

This bit is set by hardware when the content of the TDR register has been transferred into the shift register. An interrupt is generated if the TXEIE bit =1 in the USART_CR1 register. It is cleared by a write to the USART_DR register.

0: Data is not transferred to the shift register
1: Data is transferred to the shift register)

*Note: This bit is used during single buffer transmission.*

**Bit 6 TC: Transmission complete**

This bit is set by hardware if the transmission of a frame containing data is complete and if TXE is set. An interrupt is generated if TCIE=1 in the USART_CR1 register. It is cleared by a software sequence (a read from the USART_SR register followed by a write to the USART_DR register). The TC bit can also be cleared by writing a '0' to it. This clearing sequence is recommended only for multibuffer communication.

0: Transmission is not complete
1: Transmission is complete

**Bit 5 RXNE: Read data register not empty**

This bit is set by hardware when the content of the RDR shift register has been transferred to the USART_DR register. An interrupt is generated if RXNEIE=1 in the USART_CR1 register. It is cleared by a read to the USART_DR register. The RXNE flag can also be cleared by writing a zero to it. This clearing sequence is recommended only for multibuffer communication.

0: Data is not received
1: Received data is ready to be read.

**Bit 4 IDLE: IDLE line detected**

This bit is set by hardware when an Idle Line is detected. An interrupt is generated if the IDLEIE=1 in the USART_CR1 register. It is cleared by a software sequence (an read to the USART_SR register followed by a read to the USART_DR register).

0: No Idle Line is detected
1: Idle Line is detected

*Note: The IDLE bit will not be set again until the RXNE bit has been set itself (i.e. a new idle line occurs).*

1008/1749 RM0090 Rev 18

--- Page 34 ---

RM0090 Universal synchronous asynchronous receiver transmitter (USART)

**Bit 3 ORE: Overrun error**

This bit is set by hardware when the word currently being received in the shift register is ready to be transferred into the RDR register while RXNE=1. An interrupt is generated if RXNEIE=1 in the USART_CR1 register. It is cleared by a software sequence (an read to the USART_SR register followed by a read to the USART_DR register).

0: No Overrun error
1: Overrun error is detected

*Note: When this bit is set, the RDR register content will not be lost but the shift register will be overwritten. An interrupt is generated on ORE flag in case of Multi Buffer communication if the EIE bit is set.*

**Bit 2 NF: Noise detected flag**

This bit is set by hardware when noise is detected on a received frame. It is cleared by a software sequence (an read to the USART_SR register followed by a read to the USART_DR register).

0: No noise is detected
1: Noise is detected

*Note: This bit does not generate interrupt as it appears at the same time as the RXNE bit which itself generates an interrupting interrupt is generated on NF flag in case of Multi Buffer communication if the EIE bit is set.*

*Note: When the line is noise-free, the NF flag can be disabled by programming the ONEBIT bit to 1 to increase the USART tolerance to deviations (Refer to Section 30.3.5: USART receiver tolerance to clock deviation on page 988).*

**Bit 1 FE: Framing error**

This bit is set by hardware when a de-synchronization, excessive noise or a break character is detected. It is cleared by a software sequence (an read to the USART_SR register followed by a read to the USART_DR register).

0: No Framing error is detected
1: Framing error or break character is detected

*Note: This bit does not generate interrupt as it appears at the same time as the RXNE bit which itself generates an interrupt. If the word currently being transferred causes both frame error and overrun error, it will be transferred and only the ORE bit will be set.*

*An interrupt is generated on FE flag in case of Multi Buffer communication if the EIE bit is set.*

**Bit 0 PE: Parity error**

This bit is set by hardware when a parity error occurs in receiver mode. It is cleared by a software sequence (a read from the status register followed by a read or write access to the USART_DR data register). The software must wait for the RXNE flag to be set before clearing the PE bit.

An interrupt is generated if PEIE = 1 in the USART_CR1 register.

0: No parity error
1: Parity error

STI
RM0090 Rev 18
1009/1749

--- Page 35 ---

Universal synchronous asynchronous receiver transmitter (USART) RM0090

### 30.6.2 Data register (USART_DR)

Address offset: 0x04
Reset value: 0xXXXX XXXX

Bits 31:9 Reserved, must be kept at reset value
Bits 8:0 **DR[8:0]**: Data value
Contains the Received or Transmitted data character, depending on whether it is read from or written to.
The Data register performs a double function (read and write) since it is composed of two registers, one for transmission (TDR) and one for reception (RDR)
The TDR register provides the parallel interface between the internal bus and the output shift register (see Figure 1).
The RDR register provides the parallel interface between the input shift register and the internal bus.
When transmitting with the parity enabled (PCE bit set to 1 in the USART_CR1 register), the value written in the MSB (bit 7 or bit 8 depending on the data length) has no effect because it is replaced by the parity.
When receiving with the parity enabled, the value read in the MSB bit is the received parity bit.

### 30.6.3 Baud rate register (USART_BRR)

*Note:* The baud counters stop counting if the TE or RE bits are disabled respectively.

Address offset: 0x08
Reset value: 0x0000 0000

| 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 |  |
|---|---|
| Reserved |  |
| 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0 |  |
| DIV_Mantissa[11:0] | DIV_Fraction[3:0] |
| rw |  |


Bits 31:16 Reserved, must be kept at reset value
Bits 15:4 **DIV_Mantissa[11:0]**: mantissa of USARTDIV
These 12 bits define the mantissa of the USART Divider (USARTDIV)
Bits 3:0 **DIV_Fraction[3:0]**: fraction of USARTDIV
These 4 bits define the fraction of the USART Divider (USARTDIV). When OVER8=1, the DIV_Fraction3 bit is not considered and must be kept cleared.

### 30.6.4 Control register 1 (USART_CR1)

Address offset: 0x0C
Reset value: 0x0000 0000

| 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| OVER8 | Reserved | UE | M | WAKE | PCE | PS | PEIE | TXEIE | TCIE | RXNEIE | IDLEIE | TE | RE | RWU | SBK |
| rw | Res. | rw |  |  |  |  |  |  |  |  |  |  |  |  |  |


1010/1749 RM0090 Rev 18

--- Page 36 ---

**RM0090** **Universal synchronous asynchronous receiver transmitter (USART)**

Bits 31:16 Reserved, must be kept at reset value

**Bit 15 OVER8:** Oversampling mode
0: oversampling by 16
1: oversampling by 8
*Note: Oversampling by 8 is not available in the Smartcard, IrDA and LIN modes: when SCEN=1,IREN=1 or LINEN=1 then OVER8 is forced to '0 by hardware.*

Bit 14 Reserved, must be kept at reset value

**Bit 13 UE:** USART enable
When this bit is cleared, the USART prescalers and outputs are stopped and the end of the current byte transfer in order to reduce power consumption. This bit is set and cleared by software.
0: USART prescaler and outputs disabled
1: USART enabled

**Bit 12 M:** Word length
This bit determines the word length. It is set or cleared by software.
0: 1 Start bit, 8 Data bits, n Stop bit
1: 1 Start bit, 9 Data bits, n Stop bit
*Note: The M bit must not be modified during a data transfer (both transmission and reception)*

**Bit 11 WAKE:** Wakeup method
This bit determines the USART wakeup method, it is set or cleared by software.
0: Idle Line
1: Address Mark

**Bit 10 PCE:** Parity control enable
This bit selects the hardware parity control (generation and detection). When the parity control is enabled, the computed parity is inserted at the MSB position (9th bit if M=1; 8th bit if M=0) and parity is checked on the received data. This bit is set and cleared by software. Once it is set, PCE is active after the current byte (in reception and in transmission).
0: Parity control disabled
1: Parity control enabled

**Bit 9 PS:** Parity selection
This bit selects the odd or even parity when the parity generation/detection is enabled (PCE bit set). It is set and cleared by software. The parity will be selected after the current byte.
0: Even parity
1: Odd parity

**Bit 8 PEIE:** PE interrupt enable
This bit is set and cleared by software.
0: Interrupt is inhibited
1: An USART interrupt is generated whenever PE=1 in the USART_SR register

**Bit 7 TXEIE:** TXE interrupt enable
This bit is set and cleared by software.
0: Interrupt is inhibited
1: An USART interrupt is generated whenever TXE=1 in the USART_SR register

**Bit 6 TCIE:** Transmission complete interrupt enable
This bit is set and cleared by software.
0: Interrupt is inhibited
1: An USART interrupt is generated whenever TC=1 in the USART_SR register

ST
RM0090 Rev 18
1011/1749

--- Page 37 ---

Universal synchronous asynchronous receiver transmitter (USART) RM0090

**Bit 5 RXNEIE**: RXNE interrupt enable
This bit is set and cleared by software.
0: Interrupt is inhibited
1: An USART interrupt is generated whenever ORE=1 or RXNE=1 in the USART_SR register

**Bit 4 IDLEIE**: IDLE interrupt enable
This bit is set and cleared by software.
0: Interrupt is inhibited
1: An USART interrupt is generated whenever IDLE=1 in the USART_SR register

**Bit 3 TE**: Transmitter enable
This bit enables the transmitter. It is set and cleared by software.
0: Transmitter is disabled
1: Transmitter is enabled
*Note: During transmission, a “0” pulse on the TE bit (“0” followed by “1”) sends a preamble (idle line) after the current word, except in smartcard mode.*
*When TE is set, there is a 1 bit-time delay before the transmission starts.*

**Bit 2 RE**: Receiver enable
This bit enables the receiver. It is set and cleared by software.
0: Receiver is disabled
1: Receiver is enabled and begins searching for a start bit

**Bit 1 RWU**: Receiver wakeup
This bit determines if the USART is in mute mode or not. It is set and cleared by software and can be cleared by hardware when a wakeup sequence is recognized.
0: Receiver in active mode
1: Receiver in mute mode
*Note: Before selecting Mute mode (by setting the RWU bit) the USART must first receive a data byte, otherwise it cannot function in Mute mode with wakeup by Idle line detection.*
*In Address Mark Detection wakeup configuration (WAKE bit=1) the RWU bit cannot be modified by software while the RXNE bit is set.*

**Bit 0 SBK**: Send break
This bit set is used to send break characters. It can be set and cleared by software. It should be set by software, and will be reset by hardware during the stop bit of break.
0: No break character is transmitted
1: Break character will be transmitted

1012/1749 RM0090 Rev 18 STI

--- Page 38 ---

RM0090 Universal synchronous asynchronous receiver transmitter (USART)

### 30.6.5 Control register 2 (USART_CR2)

Address offset: 0x10
Reset value: 0x0000 0000

| Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 |
| 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
| Res. | LINEN | STOP[1:0] | CLKEN | CPOL | CPHA | LBCL | Res. | LBDIE | LBDL | Res. | ADD[3:0] |  |  |  |  |
| rw |  | rw |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bits 31:15 Reserved, must be kept at reset value

**Bit 14 LINEN: LIN mode enable**
This bit is set and cleared by software.
0: LIN mode disabled
1: LIN mode enabled
The LIN mode enables the capability to send LIN Synch Breaks (13 low bits) using the SBK bit in the USART_CR1 register, and to detect LIN Sync breaks.

**Bits 13:12 STOP: STOP bits**
These bits are used for programming the stop bits.
00: 1 Stop bit
01: 0.5 Stop bit
10: 2 Stop bits
11: 1.5 Stop bit
*Note: The 0.5 Stop bit and 1.5 Stop bit are not available for UART4 & UART5.*

**Bit 11 CLKEN: Clock enable**
This bit allows the user to enable the CK pin.
0: CK pin disabled
1: CK pin enabled
This bit is not available for UART4 & UART5.

**Bit 10 CPOL: Clock polarity**
This bit allows the user to select the polarity of the clock output on the CK pin in synchronous mode. It works in conjunction with the CPHA bit to produce the desired clock/data relationship
0: Steady low value on CK pin outside transmission window.
1: Steady high value on CK pin outside transmission window.
This bit is not available for UART4 & UART5.

**Bit 9 CPHA: Clock phase**
This bit allows the user to select the phase of the clock output on the CK pin in synchronous mode. It works in conjunction with the CPOL bit to produce the desired clock/data relationship (see figures 308 to 309)
0: The first clock transition is the first data capture edge
1: The second clock transition is the first data capture edge
*Note: This bit is not available for UART4 & UART5.*

STI
RM0090 Rev 18
1013/1749

--- Page 39 ---

Universal synchronous asynchronous receiver transmitter (USART) RM0090

Bit 8 **LBCL**: Last bit clock pulse
This bit allows the user to select whether the clock pulse associated with the last data bit transmitted (MSB) has to be output on the CK pin in synchronous mode.
0: The clock pulse of the last data bit is not output to the CK pin
1: The clock pulse of the last data bit is output to the CK pin
*Note: 1: The last bit is the 8th or 9th data bit transmitted depending on the 8 or 9 bit format selected by the M bit in the USART_CR1 register.*
*2: This bit is not available for UART4 & UART5.*

Bit 7 Reserved, must be kept at reset value

Bit 6 **LBDIE**: LIN break detection interrupt enable
Break interrupt mask (break detection using break delimiter).
0: Interrupt is inhibited
1: An interrupt is generated whenever LBD=1 in the USART_SR register

Bit 5 **LBDL**: lin break detection length
This bit is for selection between 11 bit or 10 bit break detection.
0: 10-bit break detection
1: 11-bit break detection

Bit 4 Reserved, must be kept at reset value

Bits 3:0 **ADD[3:0]**: Address of the USART node
This bit-field gives the address of the USART node.
This is used in multiprocessor communication during mute mode, for wake up with address mark detection.

*Note: These 3 bits (CPOL, CPHA, LBCL) should not be written while the transmitter is enabled.*

### 30.6.6 Control register 3 (USART_CR3)

Address offset: 0x14
Reset value: 0x0000 0000

| Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 |  |  |  |  |
| Reserved | ONEBIT | CTSIE | CTSE | RTSE | DMAT | DMAR | SCEN | NACK | HDSEL | IRLP | IREN | EIE |  |  |  |  |  |  |  |
|  |  |  |  | rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |


Bits 31:12 Reserved, must be kept at reset value

Bit 11 **ONEBIT**: One sample bit method enable
This bit allows the user to select the sample method. When the one sample bit method is selected the noise detection flag (NF) is disabled.
0: Three sample bit method
1: One sample bit method
*Note: The ONEBIT feature applies only to data bits. It does not apply to START bit.*

Bit 10 **CTSIE**: CTS interrupt enable
0: Interrupt is inhibited
1: An interrupt is generated whenever CTS=1 in the USART_SR register
*Note: This bit is not available for UART4 & UART5.*

1014/1749 RM0090 Rev 18

--- Page 40 ---

RM0090
Universal synchronous asynchronous receiver transmitter (USART)

**Bit 9 CTSE**: CTS enable
0: CTS hardware flow control disabled
1: CTS mode enabled, data is only transmitted when the CTS input is asserted (tied to 0). If the CTS input is deasserted while a data is being transmitted, then the transmission is completed before stopping. If a data is written into the data register while CTS is deasserted, the transmission is postponed until CTS is asserted.
*Note: This bit is not available for UART4 & UART5.*

**Bit 8 RTSE**: RTS enable
0: RTS hardware flow control disabled
1: RTS interrupt enabled, data is only requested when there is space in the receive buffer. The transmission of data is expected to cease after the current character has been transmitted. The RTS output is asserted (tied to 0) when a data can be received.
*Note: This bit is not available for UART4 & UART5.*

**Bit 7 DMAT**: DMA enable transmitter
This bit is set/reset by software
1: DMA mode is enabled for transmission.
0: DMA mode is disabled for transmission.

**Bit 6 DMAR**: DMA enable receiver
This bit is set/reset by software
1: DMA mode is enabled for reception
0: DMA mode is disabled for reception

**Bit 5 SCEN**: Smartcard mode enable
This bit is used for enabling Smartcard mode.
0: Smartcard Mode disabled
1: Smartcard Mode enabled
*Note: This bit is not available for UART4 & UART5.*

**Bit 4 NACK**: Smartcard NACK enable
0: NACK transmission in case of parity error is disabled
1: NACK transmission during parity error is enabled
*Note: This bit is not available for UART4 & UART5.*

**Bit 3 HDSEL**: Half-duplex selection
Selection of Single-wire Half-duplex mode
0: Half duplex mode is not selected
1: Half duplex mode is selected

ST
RM0090 Rev 18
1015/1749

--- Page 41 ---

**Universal synchronous asynchronous receiver transmitter (USART)** RM0090

**Bit 2 IRLP:** IrDA low-power
This bit is used for selecting between normal and low-power IrDA modes
0: Normal mode
1: Low-power mode

**Bit 1 IREN:** IrDA mode enable
This bit is set and cleared by software.
0: IrDA disabled
1: IrDA enabled

**Bit 0 EIE:** Error interrupt enable
Error Interrupt Enable Bit is required to enable interrupt generation in case of a framing error, overrun error or noise flag (FE=1 or ORE=1 or NF=1 in the USART_SR register) in case of Multi Buffer Communication (DMAR=1 in the USART_CR3 register).
0: Interrupt is inhibited
1: An interrupt is generated whenever DMAR=1 in the USART_CR3 register and FE=1 or ORE=1 or NF=1 in the USART_SR register.

1016/1749 RM0090 Rev 18 ST

--- Page 42 ---

RM0090 Universal synchronous asynchronous receiver transmitter (USART)

### 30.6.7 Guard time and prescaler register (USART_GTPR)

Address offset: 0x18
Reset value: 0x0000 0000

| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
| GT[7:0] | PSC[7:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bits 31:16 Reserved, must be kept at reset value

Bits 15:8 **GT[7:0]**: Guard time value
This bit-field gives the Guard time value in terms of number of baud clocks.
This is used in Smartcard mode. The Transmission Complete flag is set after this guard time value.
*Note: This bit is not available for UART4 & UART5.*

Bits 7:0 **PSC[7:0]**: Prescaler value
- **In IrDA Low-power mode:**
  **PSC[7:0]** = IrDA Low-Power Baud Rate
  Used for programming the prescaler for dividing the system clock to achieve the low-power frequency:
  The source clock is divided by the value given in the register (8 significant bits):
  00000000: Reserved - do not program this value
  00000001: divides the source clock by 1
  00000010: divides the source clock by 2
  ...
- **In normal IrDA mode:** PSC must be set to 00000001.
- **In smartcard mode:**
  **PSC[4:0]**: Prescaler value
  Used for programming the prescaler for dividing the system clock to provide the smartcard clock.
  The value given in the register (5 significant bits) is multiplied by 2 to give the division factor of the source clock frequency:
  00000: Reserved - do not program this value
  00001: divides the source clock by 2
  00010: divides the source clock by 4
  00011: divides the source clock by 6
  ...
*Note: 1: Bits [7:5] have no effect if Smartcard mode is used.*
*2: This bit is not available for UART4 & UART5.*

STI
RM0090 Rev 18
1017/1749

--- Page 43 ---

Universal synchronous asynchronous receiver transmitter (USART)
RM0090

### 30.6.8 USART register map

The table below gives the USART register map and reset values.

**Table 149. USART register map and reset values**

| Offset | Register | 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 0x00 | USART_SR | Reserved | CTS | LBD | TXE | TC | RXNE | IDLE | ORE | NF | FE | PE |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 0 | 1 | 0 |  |  |  |  |  |  |
| 0x04 | USART_DR | Reserved | DR[8:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 0 0 0 0 0 0 0 0 0 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x08 | USART_BRR | Reserved | DIV_Mantissa[15:4] | DIV_Fraction [3:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 0 0 0 0 0 0 0 0 0 0 | 0 0 0 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x0C | USART_CR1 | Reserved | OVER8 | Reserved | UE | M | WAKE | PCE | PS | PEIE | TXEIE | TCIE | RXNEIE | IDLEIE | TE | RE | RWU | SBK |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  | 0 |  | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x10 | USART_CR2 | Reserved | LINEN | STOP [1:0] | CLKEN | CPOL | CPHA | LBCL | Reserved | LBDIE | LBDL | Reserved | ADD[3:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  | 0 | 0 0 0 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x14 | USART_CR3 | Reserved | ONEBI | CTSIE | CTSE | RTSE | DMAT | DMAR | SCEN | NACK | HDSSEL | IRLP | IREN | EIE |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value |  |  |  |  |  |  |  |  |  |  |  | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x18 | USART_GTPR | Reserved | GT[7:0] | PSC[7:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 0 0 0 0 0 0 0 0 0 0 | 0 0 0 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Refer to Section 2.3: Memory map for the register boundary addresses.

1018/1749
RM0090 Rev 18
ST

--- Page 44 ---

RM0090
Secure digital input/output interface (SDIO)


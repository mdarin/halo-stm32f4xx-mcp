### 30.3.6 Multiprocessor communication

There is a possibility of performing multiprocessor communication with the USART (several USARTs connected in a network). For instance one of the USARTs can be the master, its TX output is connected to the RX input of the other USART. The others are slaves, their respective TX outputs are logically ANDed together and connected to the RX input of the master.

In multiprocessor configurations it is often desirable that only the intended message recipient should actively receive the full message contents, thus reducing redundant USART service overhead for all non addressed receivers.

The non addressed devices may be placed in mute mode by means of the muting function. In mute mode:

*   None of the reception status bits can be set.
*   All the receive interrupts are inhibited.
*   The RWU bit in USART_CR1 register is set to 1. RWU can be controlled automatically by hardware or written by the software under certain conditions.

The USART can enter or exit from mute mode using one of two methods, depending on the WAKE bit in the USART_CR1 register:

*   Idle Line detection if the WAKE bit is reset,
*   Address Mark detection if the WAKE bit is set.

### Idle line detection (WAKE=0)

The USART enters mute mode when the RWU bit is written to 1.

It wakes up when an Idle frame is detected. Then the RWU bit is cleared by hardware but the IDLE bit is not set in the USART_SR register. RWU can also be written to 0 by software.

An example of mute mode behavior using Idle line detection is given in *Figure 303*.

STI RM0090 Rev 18 989/1749

--- Page 15 ---

**Universal synchronous asynchronous receiver transmitter (USART)**
RM0090

**Figure 303. Mute mode using Idle line detection**

RX
Data 1 Data 2 Data 3 Data 4 IDLE Data 5 Data 6
RXNE RXNE

RWU
RWU written to 1
Mute mode
Idle frame detected
Normal mode

MSv40881V1

**Address mark detection (WAKE=1)**

In this mode, bytes are recognized as addresses if their MSB is a '1' else they are considered as data. In an address byte, the address of the targeted receiver is put on the 4 LSB. This 4-bit word is compared by the receiver with its own address which is programmed in the ADD bits in the USART_CR2 register.

The USART enters mute mode when an address character is received which does not match its programmed address. In this case, the RWU bit is set by hardware. The RXNE flag is not set for this address byte and no interrupt nor DMA request is issued as the USART would have entered mute mode.

It exits from mute mode when an address character is received which matches the programmed address. Then the RWU bit is cleared and subsequent bytes are received normally. The RXNE bit is set for the address character since the RWU bit has been cleared.

The RWU bit can be written to as 0 or 1 when the receiver buffer contains no data (RXNE=0 in the USART_SR register). Otherwise the write attempt is ignored.

An example of mute mode behavior using address mark detection is given in Figure 304.

**Figure 304. Mute mode using address mark detection**

In this example, the current address of the receiver is 1
(programmed in the USART_CR2 register)

RX
IDLE Addr=0 Data 1 Data 2 IDLE Addr=1 Data 3 Data 4 Addr=2 Data 5
RXNE RXNE RXNE

RWU
RWU written to 1
(RXNE was cleared)
Non-matching address
Mute mode
Matching address
Normal mode
Non-matching address
Mute mode

MSv40882V1

990/1749
RM0090 Rev 18
STI

--- Page 16 ---

RM0090 Universal synchronous asynchronous receiver transmitter (USART)

### 30.3.7 Parity control

Parity control (generation of parity bit in transmission and parity checking in reception) can be enabled by setting the PCE bit in the USART_CR1 register. Depending on the frame length defined by the M bit, the possible USART frame formats are as listed in Table 146.

| M bit | PCE bit | USART frame(1) |
|---|---|---|
| 0 | | SB | 8 bit data | STB | |  |
| 0 | 1 | | SB | 7-bit data | PB | STB | |
| 1 | 0 | | SB | 9-bit data | STB | |
| 1 | | SB | 8-bit data PB | STB | |  |


1. Legends: SB: start bit, STB: stop bit, PB: parity bit.

**Even parity**

The parity bit is calculated to obtain an even number of “1s” inside the frame made of the 7 or 8 LSB bits (depending on whether M is equal to 0 or 1) and the parity bit.

E.g.: data=00110101; 4 bits set => parity bit will be 0 if even parity is selected (PS bit in USART_CR1 = 0).

**Odd parity**

The parity bit is calculated to obtain an odd number of “1s” inside the frame made of the 7 or 8 LSB bits (depending on whether M is equal to 0 or 1) and the parity bit.

E.g.: data=00110101; 4 bits set => parity bit will be 1 if odd parity is selected (PS bit in USART_CR1 = 1).

**Parity checking in reception**

If the parity check fails, the PE flag is set in the USART_SR register and an interrupt is generated if PEIE is set in the USART_CR1 register. The PE flag is cleared by a software sequence (a read from the status register followed by a read or write access to the USART_DR data register).

*Note:* In case of wakeup by an address mark: the MSB bit of the data is taken into account to identify an address but not the parity bit. And the receiver does not check the parity of the address data (PE is not set in case of a parity error).

**Parity generation in transmission**

If the PCE bit is set in USART_CR1, then the MSB bit of the data written in the data register is transmitted but is changed by the parity bit (even number of “1s” if even parity is selected (PS=0) or an odd number of “1s” if odd parity is selected (PS=1)).

*Note:* The software routine that manages the transmission can activate the software sequence which clears the PE flag (a read from the status register followed by a read or write access to the data register). When operating in half-duplex mode, depending on the software, this can cause the PE flag to be unexpectedly cleared.

STI RM0090 Rev 18 991/1749

--- Page 17 ---

Universal synchronous asynchronous receiver transmitter (USART) RM0090


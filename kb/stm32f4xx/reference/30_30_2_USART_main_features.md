# 30 Universal synchronous asynchronous receiver transmitter (USART)

This section applies to the whole STM32F4xx family, unless otherwise specified.

## 30.1 USART introduction

The universal synchronous asynchronous receiver transmitter (USART) offers a flexible means of full-duplex data exchange with external equipment requiring an industry standard NRZ asynchronous serial data format. The USART offers a very wide range of baud rates using a fractional baud rate generator.

It supports synchronous one-way communication and half-duplex single wire communication. It also supports the LIN (local interconnection network), Smartcard Protocol and IrDA (infrared data association) SIR ENDEC specifications, and modem operations (CTS/RTS). It allows multiprocessor communication.

High speed data communication is possible by using the DMA for multibuffer configuration.

## 30.2 USART main features

*   Full duplex, asynchronous communications
*   NRZ standard format (Mark/Space)
*   Configurable oversampling method by 16 or by 8 to give flexibility between speed and clock tolerance
*   Fractional baud rate generator systems
    *   Common programmable transmit and receive baud rate (refer to the datasheets for the value of the baud rate at the maximum APB frequency.
*   Programmable data word length (8 or 9 bits)
*   Configurable stop bits - support for 1 or 2 stop bits
*   LIN Master Synchronous Break send capability and LIN slave break detection capability
    *   13-bit break generation and 10/11 bit break detection when USART is hardware configured for LIN
*   Transmitter clock output for synchronous transmission
*   IrDA SIR encoder decoder
    *   Support for 3/16 bit duration for normal mode
*   Smartcard emulation capability
    *   The Smartcard interface supports the asynchronous protocol Smartcards as defined in the ISO 7816-3 standards
    *   0.5, 1.5 stop bits for Smartcard operation
*   Single-wire half-duplex communication
*   Configurable multibuffer communication using DMA (direct memory access)
    *   Buffering of received/transmitted bytes in reserved SRAM using centralized DMA
*   Separate enable bits for transmitter and receiver

ST
RM0090 Rev 18 965/1749

--- Page 186 ---

Universal synchronous asynchronous receiver transmitter (USART) RM0090

* Transfer detection flags:
  - Receive buffer full
  - Transmit buffer empty
  - End of transmission flags
* Parity control:
  - Transmits parity bit
  - Checks parity of received data byte
* Four error detection flags:
  - Overrun error
  - Noise detection
  - Frame error
  - Parity error
* Ten interrupt sources with flags:
  - CTS changes
  - LIN break detection
  - Transmit data register empty
  - Transmission complete
  - Receive data register full
  - Idle line received
  - Overrun error
  - Framing error
  - Noise error
  - Parity error
* Multiprocessor communication - enter into mute mode if address match does not occur
* Wake up from mute mode (by idle line detection or address mark detection)
* Two receiver wakeup modes: Address bit (MSB, 9th bit), Idle line


**2.6 USART peripheral limitations**

**2.6.1 Idle frame is not detected if receiver clock speed is deviated**

**Description**

If the USART receives an idle frame followed by a character, and the clock of the transmitter device is faster than the USART receiver clock, the USART receive signal falls too early when receiving the character start bit, with the result that the idle frame is not detected (IDLE flag is not set).



--- Page 21 ---

**STM32F40x and STM32F41x STM32F40x and STM32F41x silicon limitations**

**Workaround**

None.

**2.6.2 In full duplex mode, the Parity Error (PE) flag can be cleared by writing to the data register**

**Description**

In full duplex mode, when the Parity Error flag is set by the receiver at the end of a reception, it may be cleared while transmitting by reading the USART_SR register to check the TXE or TC flags and writing data to the data register.

Consequently, the software receiver can read the PE flag as '0' even if a parity error occurred.

**Workaround**

The Parity Error flag should be checked after the end of reception and before transmission.

**2.6.3 Parity Error (PE) flag is not set when receiving in Mute mode using address mark detection**

**Description**

The USART receiver is in Mute mode and is configured to exit the Mute mode using the address mark detection. When the USART receiver recognizes a valid address with a parity error, it exits the Mute mode without setting the Parity Error flag.

**Workaround**

None.

**2.6.4 Break frame is transmitted regardless of nCTS input line status**

**Description**

When CTS hardware flow control is enabled (CTSE = 1) and the Send Break bit (SBK) is set, the transmitter sends a break frame at the end of the current transmission regardless of nCTS input line status.

Consequently, if an external receiver device is not ready to accept a frame, the transmitted break frame is lost.

**Workaround**

None.

**2.6.5 nRTS signal abnormally driven low after a protocol violation**

**Description**

When RTS hardware flow control is enabled, the nRTS signal goes high when data is received. If this data was not read and new data is sent to the USART (protocol violation), the nRTS signal goes back to low level at the end of this new data.



37

--- Page 22 ---

**STM32F40x and STM32F41x silicon limitations STM32F40x and STM32F41x**

Consequently, the sender gets the wrong information that the USART is ready to receive further data.

On USART side, an overrun is detected, which indicates that data has been lost.

**Workaround**

Workarounds are required only if the other USART device violates the communication protocol, which is not the case in most applications.

Two workarounds can be used:

• After data reception and before reading the data in the data register, the software takes

over the control of the nRTS signal as a GPIO and holds it high as long as needed. If the USART device is not ready, the software holds the nRTS pin high, and releases it when the device is ready to receive new data.

• The time required by the software to read the received data must always be lower than

the duration of the second data reception. For example, this can be ensured by treating all the receptions by DMA mode.

**2.6.6 Start bit detected too soon when sampling for NACK signal from the smartcard**

**Description**

According to ISO/IEC 7816-3 standard, when a character parity error is detected, the receiver shall transmit a NACK error signal 10.5 ± 0.2 ETUs after the character START bit falling edge. In this case, the transmitter should be able to detect correctly the NACK signal until 11 ± 0.2 ETUs after the character START bit falling edge.

In Smartcard mode, the USART peripheral monitors the NACK signal during the receiver time frame (10.5 ± 0.2 ETUs), while it should wait for it during the transmitter one (11 ± 0.2 ETUs). In real cases, this would not be a problem as the card itself needs to respect a 10.7 ETU period when sending the NACK signal. However this may be an issue to undertake a certification.

**Workaround**

None

**2.6.7 Break request can prevent the Transmission Complete flag (TC) from being set**

**Description**

After the end of transmission of a data (D1), the Transmission Complete (TC) flag will not be set if the following conditions are met:

• CTS hardware flow control is enabled.

• D1 is being transmitted.

• A break transfer is requested before the end of D1 transfer.

• nCTS is de-asserted before the end of D1 data transfer.



--- Page 23 ---

**STM32F40x and STM32F41x STM32F40x and STM32F41x silicon limitations**

**Workaround**

If the application needs to detect the end of a data transfer, the break request should be issued after checking that the TC flag is set.

**2.6.8 Guard time is not respected when data are sent on TXE events**

**Description**

In smartcard mode, when sending a data on TXE event, the programmed guard time is not respected i.e. the data written in the data register is transferred on the bus without waiting the completion of the guardtime duration corresponding to the previous transmitted data.

**Workaround**

Write the data after TC is set because in smartcard mode, the TC flag is set at the end of the guard time duration.

**2.6.9 nRTS is active while RE or UE = 0**

**Description**

The nRTS line is driven low as soon as RTSE bit is set even if the USART is disabled (UE =

0) or if the receiver is disabled (RE=0) i.e. not ready to receive data.

**Workaround**

Configure the I/O used for nRTS as an alternate function after setting the UE and RE bits.

**2.7 bxCAN limitations**

**2.7.1 bxCAN time triggered communication mode not supported**

**Description**

The time triggered communication mode described in the reference manual is not supported. As a result timestamp values are not available. TTCM bit must be kept cleared in the CAN_MCR register (time triggered communication mode disabled).

**Workaround**

None


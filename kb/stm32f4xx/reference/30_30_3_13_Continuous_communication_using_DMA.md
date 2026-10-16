### 30.3.13 Continuous communication using DMA

The USART is capable of continuous communication using the DMA. The DMA requests for Rx buffer and Tx buffer are generated independently.

#### Transmission using DMA

DMA mode can be enabled for transmission by setting DMAT bit in the USART_CR3 register. Data is loaded from a SRAM area configured using the DMA peripheral (refer to the DMA specification) to the USART_DR register whenever the TXE bit is set. To map a DMA channel for USART transmission, use the following procedure (x denotes the channel number):

1. Write the USART_DR register address in the DMA control register to configure it as the destination of the transfer. The data will be moved to this address from memory after each TXE event.
2. Write the memory address in the DMA control register to configure it as the source of the transfer. The data will be loaded into the USART_DR register from this memory area after each TXE event.
3. Configure the total number of bytes to be transferred to the DMA control register.
4. Configure the channel priority in the DMA register
5. Configure DMA interrupt generation after half/ full transfer as required by the application.
6. Clear the TC bit in the SR register by writing 0 to it.
7. Activate the channel in the DMA register.

When the number of data transfers programmed in the DMA Controller is reached, the DMA controller generates an interrupt on the DMA channel interrupt vector.

In transmission mode, once the DMA has written all the data to be transmitted (the TCIF flag is set in the DMA_ISR register), the TC flag can be monitored to make sure that the USART communication is complete. This is required to avoid corrupting the last transmission before disabling the USART or entering the Stop mode. The software must wait until TC=1. The TC flag remains cleared during all data transfers and it is set by hardware at the last frame's end of transmission.

STI RM0090 Rev 18 1001/1749

--- Page 27 ---

Universal synchronous asynchronous receiver transmitter (USART)
RM0090

Figure 315. Transmission using DMA

Idle preamble
Frame 1
Frame 2
Frame 3

TX line
TXE flag
set by hardware cleared by DMA read
set by hardware cleared by DMA read
set by hardware

DMA request
ignored by the DMA because DMA transfer is complete

USART_DR
F1
F2
F3

TC flag
set by hardware

DMA writes USART_DR
flag DMA TCIF (Transfer complete)
software configures the DMA to send 3 data and enables the USART
DMA writes F1 into USART_DR
DMA writes F2 into USART_DR
DMA writes F3 into USART_DR
set by hardware
The DMA transfer is complete (TCIF=1 in DMA_ISR)
clear by software
software waits until TC=1

ai17192b

Reception using DMA

DMA mode can be enabled for reception by setting the DMAR bit in USART_CR3 register. Data is loaded from the USART_DR register to a SRAM area configured using the DMA peripheral (refer to the DMA specification) whenever a data byte is received. To map a DMA channel for USART reception, use the following procedure:

1. Write the USART_DR register address in the DMA control register to configure it as the source of the transfer. The data will be moved from this address to the memory after each RXNE event.
2. Write the memory address in the DMA control register to configure it as the destination of the transfer. The data will be loaded from USART_DR to this memory area after each RXNE event.
3. Configure the total number of bytes to be transferred in the DMA control register.
4. Configure the channel priority in the DMA control register
5. Configure interrupt generation after half/ full transfer as required by the application.
6. Activate the channel in the DMA control register.

When the number of data transfers programmed in the DMA Controller is reached, the DMA controller generates an interrupt on the DMA channel interrupt vector. The DMAR bit should be cleared by software in the USART_CR3 register during the interrupt subroutine.

1002/1749
RM0090 Rev 18

--- Page 28 ---

RM0090
Universal synchronous asynchronous receiver transmitter (USART)

**Figure 316. Reception using DMA**

TX line
Frame 1
Frame 2
Frame 3

RXNE flag
set by hardware
cleared by DMA read

DMA request

USART_DR
F1
F2
F3

DMA reads USART_DR

DMA TCIF flag
(Transfer complete)
set by hardware
cleared by software

software configures the
DMA to receive 3 data
blocks and enables
the USART

DMA reads F1
from
USART_DR

DMA reads F2
from
USART_DR

DMA reads F3
from
USART_DR

The DMA transfer
is complete
(TCIF=1 in
DMA_ISR)

ai17193b

**Error flagging and interrupt generation in multibuffer communication**

In case of multibuffer communication if any error occurs during the transaction the error flag
will be asserted after the current byte. An interrupt will be generated if the interrupt enable
flag is set. For framing error, overrun error and noise flag which are asserted with RXNE in
case of single byte reception, there will be separate error flag interrupt enable bit (EIE bit in
the USART_CR3 register), which if set will issue an interrupt after the current byte with
either of these errors.


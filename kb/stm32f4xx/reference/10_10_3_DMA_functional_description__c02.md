> [RM0090 ch.10 §10.3] DMA functional description


### 10.3.1 General description

*Figure 32* shows the block diagram of a DMA.

**Figure 32. DMA block diagram**

DMA controller

REQ STR0_CH0
REQ STR0_CH1
REQ STR0_CH7
REQ STR1_CH0
REQ STR1_CH1
REQ STR1_CH7
REQ STREAM0
REQ STREAM1
REQ STREAM2
REQ STREAM3
REQ STREAM4
REQ STREAM5
REQ STREAM6
REQ STREAM7
REQ STR7_CH0
REQ STR7_CH1
REQ STR7_CH7
Channel selection

Arbiter

STREAM 0
STREAM 1
STREAM 2
STREAM 3
STREAM 4
STREAM 5
STREAM 6
STREAM 7

FIFO
FIFO
FIFO
FIFO
FIFO
FIFO
FIFO
FIFO

STREAM 0
STREAM 1
STREAM 2
STREAM 3
STREAM 4
STREAM 5
STREAM 6
STREAM 7

AHB master
Memory port

AHB master
Peripheral port

AHB slave
programming
interface
Programming port

ai15945

The DMA controller performs direct memory transfer: as an AHB master, it can take the control of the AHB bus matrix to initiate AHB transactions.

It can carry out the following transactions:

*   peripheral-to-memory
*   memory-to-peripheral
*   memory-to-memory

The DMA controller provides two AHB master ports: the *AHB memory port*, intended to be connected to memories and the *AHB peripheral port*, intended to be connected to peripherals. However, to allow memory-to-memory transfers, the *AHB peripheral port* must also have access to the memories.

The AHB slave port is used to program the DMA controller (it supports only 32-bit accesses).

304/1749
RM0090 Rev 18
ST

--- Page 110 ---

RM0090
DMA controller (DMA)

See Figure 33 and Figure 34 for the implementation of the system of two DMA controllers.

Figure 33. System implementation of the two DMA controllers
(STM32F405xx/07xx and STM32F415xx/17xx)

Bus matrix (AHB multilayer)
DCODE
ICODE
Flash memory
112 KB SRAM
16 KB SRAM
AHB1 peripherals
AHB-APB bridge2 (dual AHB)
APB2
APB2 peripherals
AHB-APB bridge1 (dual AHB)
APB1
APB1 peripherals
AHB2 peripherals
External memory controller (FSMC)
DMA controller 2
To AHB2 peripherals
AHB slave
Arbiter
FIFO
AHB memory port
AHB peripheral port
DMA request
DMA controller 1
To AHB2 peripherals
AHB slave
Arbiter
FIFO
AHB memory port
AHB peripheral port

MS19927V2

1. The DMA1 controller AHB peripheral port is not connected to the bus matrix like DMA2 controller. As a result, only DMA2 streams are able to perform memory-to-memory transfers.

ST
RM0090 Rev 18
305/1749

--- Page 111 ---

DMA controller (DMA)
RM0090

Figure 34. System implementation of the two DMA controllers (STM32F42xxx and STM32F43xxx)

Bus Matrix (AHB multilayer)
DCODE
ICODE
Flash memory
112 KB SRAM
16 KB SRAM
64 KB SRAM
AHB1 peripherals
AHB-APB bridge2 (dual AHB)
APB2 peripherals
AHB-APB bridge1 (dual AHB)
APB1 peripherals
AHB2 peripherals
External memory controller (FMC)
DMA controller 2
DMA request
DMA controller 1
To AHB2 peripherals
To AHB2 peripherals

MS30438V1

1. The DMA1 controller AHB peripheral port is not connected to the bus matrix like in the case of the DMA2 controller, thus only DMA2 streams are able to perform memory-to-memory transfers.

> [RM0090 ch.10 §10.3] DMA functional description

### 10.3.2 DMA transactions

### 10.3.2 DMA transactions

A DMA transaction consists of a sequence of a given number of data transfers. The number of data items to be transferred and their width (8-bit, 16-bit or 32-bit) are software-programmable.

Each DMA transfer consists of three operations:

* A loading from the peripheral data register or a location in memory, addressed through the DMA_SxPAR or DMA_SxMOAR register
* A storage of the data loaded to the peripheral data register or a location in memory addressed through the DMA_SxPAR or DMA_SxMOAR register
* A post-decrement of the DMA_SxNDTR register, which contains the number of transactions that still have to be performed

306/1749
RM0090 Rev 18
ST

--- Page 112 ---

RM0090
DMA controller (DMA)

After an event, the peripheral sends a request signal to the DMA controller. The DMA controller serves the request depending on the channel priorities. As soon as the DMA controller accesses the peripheral, an Acknowledge signal is sent to the peripheral by the DMA controller. The peripheral releases its request as soon as it gets the Acknowledge signal from the DMA controller. Once the request has been deasserted by the peripheral, the DMA controller releases the Acknowledge signal. If there are more requests, the peripheral can initiate the next transaction.


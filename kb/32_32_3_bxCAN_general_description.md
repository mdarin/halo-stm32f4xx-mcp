### 32.3 bxCAN general description

In today's CAN applications, the number of nodes in a network is increasing and often several networks are linked together via gateways. Typically the number of messages in the system (and thus to be handled by each node) has significantly increased. In addition to the application messages, Network Management and Diagnostic messages have been introduced.

*   An enhanced filtering mechanism is required to handle each type of message.

Furthermore, application tasks require more CPU time, therefore real-time constraints caused by message reception have to be reduced.

*   A receive FIFO scheme allows the CPU to be dedicated to application tasks for a long time period without losing messages.

The standard HLP (Higher Layer Protocol) based on standard CAN drivers requires an efficient interface to the CAN controller.

Figure 334. CAN network topology

CAN node 1
MCU
Application
CAN Controller
CAN Rx
CAN Tx
CAN Transceiver
CAN High
CAN Low
CAN Bus
CAN node 2
CAN node n

### 32.3.1 CAN 2.0B active core

The bxCAN module handles the transmission and the reception of CAN messages fully autonomously. Standard identifiers (11-bit) and extended identifiers (29-bit) are fully supported by hardware.

ST
RM0090 Rev 18
1077/1749

--- Page 103 ---

Controller area network (bxCAN) RM0090

### 32.3.2 Control, status and configuration registers

The application uses these registers to:

* Configure CAN parameters, e.g. baud rate
* Request transmissions
* Handle receptions
* Manage interrupts
* Get diagnostic information

### 32.3.3 Tx mailboxes

Three transmit mailboxes are provided to the software for setting up messages. The transmission Scheduler decides which mailbox has to be transmitted first.

### 32.3.4 Acceptance filters

The bxCAN provides 28 scalable/configurable identifier filter banks for selecting the incoming messages the software needs and discarding the others.

**Receive FIFO**

Two receive FIFOs are used by hardware to store the incoming messages. Three complete messages can be stored in each FIFO. The FIFOs are managed completely by hardware.

1078/1749 RM0090 Rev 18 ST

--- Page 104 ---

RM0090
Controller area network (bxCAN)

**Figure 335. Dual CAN block diagram**

CAN1 (Master) with 512 bytes SRAM

Control/Status/Configuration
Master Control
Master Status
Tx Status
Rx FIFO 0 Status
Rx FIFO 1 Status
Interrupt Enable
Error Status
Bit Timing
Filter Master
Filter Mode
Filter Scale
Filter FIFO Assign
Filter Activation

CAN 2.0B Active Core

Master Tx Mailboxes
Mailbox 0
Transmission Scheduler

Master Receive FIFO 0
Mailbox 0

Master Receive FIFO 1
Mailbox 0

Memory Access Controller

Acceptance Filters
Filter 0 1 2 3 ... 26 27
Master Filters (0 to 27)
Slave Filters (0 to 27)

Transmission Scheduler
Slave Tx Mailboxes
Mailbox 0

Slave Receive FIFO 0
Mailbox 0

Slave Receive FIFO 1
Mailbox 0

CAN2 (Slave)

Control/Status/Configuration
Master Control
Master Status
Tx Status
Rx FIFO 0 Status
Rx FIFO 1 Status
Interrupt Enable
Error Status
Bit Timing

CAN 2.0B Active Core

Note: CAN 2 start filter bank number n is configurable by writing to the CAN2SB[5:0] bits in the CAN_FMR register.

ai16094b


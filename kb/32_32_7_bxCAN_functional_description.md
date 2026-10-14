### 32.7 bxCAN functional description

#### 32.7.1 Transmission handling

In order to transmit a message, the application must select one **empty** transmit mailbox, set up the identifier, the data length code (DLC) and the data before requesting the transmission by setting the corresponding TXRQ bit in the CAN_TxR register. Once the mailbox has left **empty** state, the software no longer has write access to the mailbox registers. Immediately after the TXRQ bit has been set, the mailbox enters **pending** state and waits to become the highest priority mailbox, see *Transmit Priority*. As soon as the mailbox has the highest priority it will be **scheduled** for transmission. The transmission of the message of the scheduled mailbox will start (enter **transmit** state) when the CAN bus becomes idle. Once the mailbox has been successfully transmitted, it will become **empty** again. The hardware indicates a successful transmission by setting the RQCP and TXOK bits in the CAN_TSR register.

If the transmission fails, the cause is indicated by the ALST bit in the CAN_TSR register in case of an Arbitration Lost, and/or the TERR bit, in case of transmission error detection.

ST
RM0090 Rev 18
1083/1749

--- Page 109 ---

Controller area network (bxCAN)
RM0090

**Transmit priority**

**By identifier**
When more than one transmit mailbox is pending, the transmission order is given by the identifier of the message stored in the mailbox. The message with the lowest identifier value has the highest priority according to the arbitration of the CAN protocol. If the identifier values are equal, the lower mailbox number will be scheduled first.

**By transmit request order**
The transmit mailboxes can be configured as a transmit FIFO by setting the TXFP bit in the CAN_MCR register. In this mode the priority order is given by the transmit request order. This mode is very useful for segmented transmission.

**Abort**

A transmission request can be aborted by the user setting the ABRQ bit in the CAN_TSR register. In **pending** or **scheduled** state, the mailbox is aborted immediately. An abort request while the mailbox is in **transmit** state can have two results. If the mailbox is transmitted successfully the mailbox becomes **empty** with the TXOK bit set in the CAN_TSR register. If the transmission fails, the mailbox becomes **scheduled**, the transmission is aborted and becomes **empty** with TXOK cleared. In all cases the mailbox will become **empty** again at least at the end of the current transmission.

**Nonautomatic retransmission mode**

This mode has been implemented in order to fulfil the requirement of the Time Triggered Communication option of the CAN standard. To configure the hardware in this mode the NART bit in the CAN_MCR register must be set.

In this mode, each transmission is started only once. If the first attempt fails, due to an arbitration loss or an error, the hardware will not automatically restart the message transmission.

At the end of the first transmission attempt, the hardware considers the request as completed and sets the RQCP bit in the CAN_TSR register. The result of the transmission is indicated in the CAN_TSR register by the TXOK, ALST and TERR bits.

1084/1749
RM0090 Rev 18
ST

--- Page 110 ---

RM0090
Controller area network (bxCAN)

**Figure 340. Transmit mailbox states**

EMPTY
RQCP=X
TXOK=X
TME = 1

TXRQ=1

PENDING
RQCP=0
TXOK=0
TME = 0

Mailbox has highest priority

Mailbox does not have highest priority

SCHEDULED
RQCP=0
TXOK=0
TME = 0

ABRQ=1

EMPTY
RQCP=1
TXOK=0
TME = 1

ABRQ=1

CAN Bus = IDLE

TRANSMIT
RQCP=0
TXOK=0
TME = 0

Transmit failed * NART

Transmit failed * NART

EMPTY
RQCP=1
TXOK=1
TME = 1

Transmit succeeded

### 32.7.2 Time triggered communication mode

In this mode, the internal counter of the CAN hardware is activated and used to generate the Time Stamp value stored in the CAN_RDTxR/CAN_TDTxR registers, respectively (for Rx and Tx mailboxes). The internal counter is incremented each CAN bit time (refer to Section 32.7.7). The internal counter is captured on the sample point of the Start Of Frame bit in both reception and transmission.

### 32.7.3 Reception handling

For the reception of CAN messages, three mailboxes organized as a FIFO are provided. In order to save CPU load, simplify the software and guarantee data consistency, the FIFO is managed completely by hardware. The application accesses the messages stored in the FIFO through the FIFO output mailbox.

**Valid message**

A received message is considered as valid **when** it has been received correctly according to the CAN protocol (no error until the last but one bit of the EOF field) **and** It passed through the identifier filtering successfully, see Section 32.7.4.

ST
RM0090 Rev 18
1085/1749

--- Page 111 ---

Controller area network (bxCAN)
RM0090

Figure 341. Receive FIFO states

EMPTY
FMP=0x00
FOVR=0

Valid Message Received

PENDING_1
FMP=0x01
FOVR=0

Release Mailbox

Valid Message Received

PENDING_2
FMP=0x10
FOVR=0

Release Mailbox
RFOM=1

Valid Message Received

PENDING_3
FMP=0x11
FOVR=0

Release Mailbox
RFOM=1

Valid Message Received

OVERRUN
FMP=0x11
FOVR=1

Release Mailbox
RFOM=1

Valid Message Received

**FIFO management**

Starting from the **empty** state, the first valid message received is stored in the FIFO which becomes **pending_1**. The hardware signals the event setting the FMP[1:0] bits in the CAN_RFR register to the value 01b. The message is available in the FIFO output mailbox. The software reads out the mailbox content and releases it by setting the RFOM bit in the CAN_RFR register. The FIFO becomes **empty** again. If a new valid message has been received in the meantime, the FIFO stays in **pending_1** state and the new message is available in the output mailbox.

If the application does not release the mailbox, the next valid message will be stored in the FIFO which enters **pending_2** state (FMP[1:0] = 10b). The storage process is repeated for the next valid message putting the FIFO into **pending_3** state (FMP[1:0] = 11b). At this point, the software must release the output mailbox by setting the RFOM bit, so that a mailbox is free to store the next valid message. Otherwise the next valid message received will cause a loss of message.

Refer also to Section 32.7.5

**Overrun**

Once the FIFO is in **pending_3** state (i.e. the three mailboxes are full) the next valid message reception will lead to an **overrun** and a message will be lost. The hardware

1086/1749
RM0090 Rev 18

--- Page 112 ---

RM0090
Controller area network (bxCAN)

signals the overrun condition by setting the FOVR bit in the CAN_RFR register. Which message is lost depends on the configuration of the FIFO:

*   If the FIFO lock function is disabled (RFLM bit in the CAN_MCR register cleared) the last message stored in the FIFO will be overwritten by the new incoming message. In this case the latest messages will be always available to the application.
*   If the FIFO lock function is enabled (RFLM bit in the CAN_MCR register set) the most recent message will be discarded and the software will have the three oldest messages in the FIFO available.

**Reception related interrupts**

Once a message has been stored in the FIFO, the FMP[1:0] bits are updated and an interrupt request is generated if the FMPIE bit in the CAN_IER register is set.

When the FIFO becomes full (i.e. a third message is stored) the FULL bit in the CAN_RFR register is set and an interrupt is generated if the FFIE bit in the CAN_IER register is set.

On overrun condition, the FOVR bit is set and an interrupt is generated if the FOVIE bit in the CAN_IER register is set.

**32.7.4 Identifier filtering**

In the CAN protocol the identifier of a message is not associated with the address of a node but related to the content of the message. Consequently a transmitter broadcasts its message to all receivers. On message reception a receiver node decides - depending on the identifier value - whether the software needs the message or not. If the message is needed, it is copied into the SRAM. If not, the message must be discarded without intervention by the software.

To fulfill this requirement, the bxCAN Controller provides 28 configurable and scalable filter banks (27-0) to the application. This hardware filtering saves CPU resources which would be otherwise needed to perform filtering by software. Each filter bank x consists of two 32-bit registers, CAN_FxR0 and CAN_FxR1.

**Scalable width**

To optimize and adapt the filters to the application needs, each filter bank can be scaled independently. Depending on the filter scale a filter bank provides:

*   One 32-bit filter for the STDID[10:0], EXTID[17:0], IDE and RTR bits.
*   Two 16-bit filters for the STDID[10:0], RTR, IDE and EXTID[17:15] bits.

Refer to Figure 342.

Furthermore, the filters can be configured in mask mode or in identifier list mode.

**Mask mode**

In **mask** mode the identifier registers are associated with mask registers specifying which bits of the identifier are handled as “must match” or as “don’t care”.

**Identifier list mode**

In **identifier list** mode, the mask registers are used as identifier registers. Thus instead of defining an identifier and a mask, two identifiers are specified, doubling the number of single identifiers. All bits of the incoming identifier must match the bits specified in the filter registers.

ST
RM0090 Rev 18
1087/1749

--- Page 113 ---

Controller area network (bxCAN)
RM0090

**Filter bank scale and mode configuration**

The filter banks are configured by means of the corresponding CAN_FMR register. To configure a filter bank it must be deactivated by clearing the FACT bit in the CAN_FAR register. The filter scale is configured by means of the corresponding FSCx bit in the CAN_FS1R register, refer to Figure 342. The **identifier list** or **identifier mask** mode for the corresponding Mask/Identifier registers is configured by means of the FBMx bits in the CAN_FMR register.

To filter a group of identifiers, configure the Mask/Identifier registers in mask mode.

To select single identifiers, configure the Mask/Identifier registers in identifier list mode.

Filters not used by the application should be left deactivated.

Each filter within a filter bank is numbered (called the *Filter Number*) from 0 to a maximum dependent on the mode and the scale of each of the filter banks.

Concerning the filter configuration, refer to Figure 342.

Figure 342. Filter bank scale configuration - register organization

| Filter Bank Scale Config. Bits1 | Filter Bank Mode2 | Filter Num. |  |  |  |  |
|---|---|---|---|---|---|---|
| One 32-Bit Filter - Identifier Mask |  |  |  |  |  |  |
| FSCx = 1 | FBMx = 0 | ID | CAN_FxR1[31:24] | CAN_FxR1[23:16] | CAN_FxR1[15:8] | CAN_FxR1[7:0] |
| Mask | CAN_FxR2[31:24] | CAN_FxR2[23:16] | CAN_FxR2[15:8] | CAN_FxR2[7:0] |  |  |
| Mapping | STID[10:3] STID[2:0] EXID[17:13] EXID[12:5] EXID[4:0] IDE RTR 0 |  |  |  |  |  |
|  |  | n |  |  |  |  |
| Two 32-Bit Filters - Identifier List |  |  |  |  |  |  |
| FSCx = 1 | FBMx = 1 | ID | CAN_FxR1[31:24] | CAN_FxR1[23:16] | CAN_FxR1[15:8] | CAN_FxR1[7:0] |
| ID | CAN_FxR2[31:24] | CAN_FxR2[23:16] | CAN_FxR2[15:8] | CAN_FxR2[7:0] |  |  |
| Mapping | STID[10:3] STID[2:0] EXID[17:13] EXID[12:5] EXID[4:0] IDE RTR 0 |  |  |  |  |  |
|  |  | nn+1 |  |  |  |  |
| Two 16-Bit Filters - Identifier Mask |  |  |  |  |  |  |
| FSCx = 0 | FBMx = 0 | ID | CAN_FxR1[15:8] | CAN_FxR1[7:0] |  |  |
| Mask | CAN_FxR1[31:24] | CAN_FxR1[23:16] |  |  |  |  |
| Mapping | STID[10:3] STID[2:0] RTR IDE EXID[17:15] |  |  |  |  |  |
|  |  | nn+1 |  |  |  |  |
| Four 16-Bit Filters - Identifier List |  |  |  |  |  |  |
| FSCx = 0 | FBMx = 1 | ID | CAN_FxR1[15:8] | CAN_FxR1[7:0] |  |  |
| ID | CAN_FxR1[31:24] | CAN_FxR1[23:16] |  |  |  |  |
| FSCx = 0 | FBMx = 1 | ID | CAN_FxR2[15:8] | CAN_FxR2[7:0] |  |  |
| ID | CAN_FxR2[31:24] | CAN_FxR2[23:16] |  |  |  |  |
| Mapping | STID[10:3] STID[2:0] RTR DE EXID[17:15] |  |  |  |  |  |
|  |  | nn+1n+2n+3 |  |  |  |  |


x = filter bank number
ID=Identifier
<sup>1</sup> These bits are located in the CAN_FS1R register
<sup>2</sup> These bits are located in the CAN_FM1R register

**Filter match index**

Once a message has been received in the FIFO it is available to the application. Typically, application data is copied into SRAM locations. To copy the data to the right location the

1088/1749
RM0090 Rev 18
ST

--- Page 114 ---

RM0090
Controller area network (bxCAN)

application has to identify the data by means of the identifier. To avoid this, and to ease the access to the SRAM locations, the CAN controller provides a Filter Match Index.

This index is stored in the mailbox together with the message according to the filter priority rules. Thus each received message has its associated filter match index.

The Filter Match index can be used in two ways:

*   Compare the Filter Match index with a list of expected values.
*   Use the Filter Match Index as an index on an array to access the data destination location.

For nonmasked filters, the software no longer has to compare the identifier.

If the filter is masked the software reduces the comparison to the masked bits only.

The index value of the filter number does not take into account the activation state of the filter banks. In addition, two independent numbering schemes are used, one for each FIFO. Refer to Figure 343 for an example.

Figure 343. Example of filter numbering

| Filter Bank | FIFO0 | Filter Bank | FIFO1 |  |  |
|---|---|---|---|---|---|
| Filter Num. |  | Filter Num. |  |  |  |
| 0 | ID List (32-bit) | 01 | 2 | ID Mask (16-bit) | 01 |
| 1 | ID Mask (32-bit) | 2 | 4 | ID List (32-bit) | 23 |
| 3 | ID List (16-bit) | 3456 | 7 | Deactivated ID Mask (16-bit) | 45 |
| 5 | Deactivated ID List (32-bit) | 78 | 8 | ID Mask (16-bit) | 67 |
| 6 | ID Mask (16-bit) | 910 | 10 | Deactivated ID List (16-bit) | 891011 |
| 9 | ID List (32-bit) | 1112 | 11 | ID List (32-bit) | 1213 |
| 13 | ID Mask (32-bit) | 13 | 12 | ID Mask (32-bit) | 14 |


ID = Identifier

ST
RM0090 Rev 18
1089/1749

--- Page 115 ---

Controller area network (bxCAN)
RM0090

**Filter priority rules**

Depending on the filter combination it may occur that an identifier passes successfully through several filters. In this case the filter match value stored in the receive mailbox is chosen according to the following priority rules:

*   A 32-bit filter takes priority over a 16-bit filter.
*   For filters of equal scale, priority is given to the Identifier List mode over the Identifier Mask mode
*   For filters of equal scale and mode, priority is given by the filter number (the lower the number, the higher the priority).

**Figure 344. Filtering mechanism - Example**

Three filter banks in 32-bit Unidentified List mode,
the remaining in 32-bit Identifier Mask mode

Message Received
Identifier | Ctrl | Data

Filter bank Num
0
2
3

Identifier List
Identifier 0
Identifier 1
Identifier 4

Identifier 5

1
4

Identifier & Mask
Identifier 2
Mask
Identifier 3
Mask

No Match Found

Receive FIFO
Message Stored

Identifier #4 Match

FMI
Filter number stored in the
Filter Match Index field
within the CAN_RDTxR
register

Message Discarded

The example above shows the filtering principle of the bxCAN. On reception of a message, the identifier is compared first with the filters configured in identifier list mode. If there is a match, the message is stored in the associated FIFO and the index of the matching filter is stored in the Filter Match Index. As shown in the example, the identifier matches with Identifier #2 thus the message content and FMI 2 is stored in the FIFO.

If there is no match, the incoming identifier is then compared with the filters configured in mask mode.

If the identifier does not match any of the identifiers configured in the filters, the message is discarded by hardware without disturbing the software.

1090/1749
RM0090 Rev 18
ST

--- Page 116 ---

RM0090
Controller area network (bxCAN)

### 32.7.5 Message storage

The interface between the software and the hardware for the CAN messages is implemented by means of mailboxes. A mailbox contains all information related to a message; identifier, data, control, status and time stamp information.

#### Transmit mailbox

The software sets up the message to be transmitted in an empty transmit mailbox. The status of the transmission is indicated by hardware in the CAN_TSR register.

**Table 182. Transmit mailbox mapping**

| Offset to transmit mailbox base address (bytes) | Register name |
|---|---|
| 0 | CAN_TIxR |
| 4 | CAN_TDTxR |
| 8 | CAN_TDLxR |
| 12 | CAN_TDHxR |


#### Receive mailbox

When a message has been received, it is available to the software in the FIFO output mailbox. Once the software has handled the message (e.g. read it) the software must release the FIFO output mailbox by means of the RFOM bit in the CAN_RFR register to make the next incoming message available. The filter match index is stored in the MFMI field of the CAN_RDTxR register. The 16-bit time stamp value is stored in the TIME[15:0] field of CAN_RDTxR.

**Table 183. Receive mailbox mapping**

| Offset to receive mailbox base address (bytes) | Register name |
|---|---|
| 0 | CAN_RIxR |
| 4 | CAN_RDTxR |
| 8 | CAN_RDLxR |
| 12 | CAN_RDHxR |


ST
RM0090 Rev 18
1091/1749

--- Page 117 ---

Controller area network (bxCAN)
RM0090

Figure 345. CAN error state diagram

ERROR ACTIVE
ERROR PASSIVE
BUS OFF

When TEC or REC > 127
When TEC and REC < 128,
When TEC > 255
When 128 * 11 recessive bits occur:

ai15903

1092/1749
RM0090 Rev 18

--- Page 118 ---

RM0090
Controller area network (bxCAN)

### 32.7.6 Error management

The error management as described in the CAN protocol is handled entirely by hardware using a Transmit Error Counter (TEC value, in CAN_ESR register) and a Receive Error Counter (REC value, in the CAN_ESR register), which get incremented or decremented according to the error condition. For detailed information about TEC and REC management, refer to the CAN standard.

Both of them may be read by software to determine the stability of the network. Furthermore, the CAN hardware provides detailed information on the current error status in CAN_ESR register. By means of the CAN_IER register (ERRIE bit, etc.), the software can configure the interrupt generation on error detection in a very flexible way.

**Bus-Off recovery**

The Bus-Off state is reached when TEC is greater than 255, this state is indicated by BOFF bit in CAN_ESR register. In Bus-Off state, the bxCAN is no longer able to transmit and receive messages.

Depending on the ABOM bit in the CAN_MCR register bxCAN will recover from Bus-Off (become error active again) either automatically or on software request. But in both cases the bxCAN has to wait at least for the recovery sequence specified in the CAN standard (128 occurrences of 11 consecutive recessive bits monitored on CANRX).

If ABOM is set, the bxCAN will start the recovering sequence automatically after it has entered Bus-Off state.

If ABOM is cleared, the software must initiate the recovering sequence by requesting bxCAN to enter and to leave initialization mode.

*Note: In initialization mode, bxCAN does not monitor the CANRX signal, therefore it cannot complete the recovery sequence. To recover, bxCAN must be in normal mode.*

### 32.7.7 Bit timing

The bit timing logic monitors the serial bus-line and performs sampling and adjustment of the sample point by synchronizing on the start-bit edge and resynchronizing on the following edges.

Its operation may be explained simply by splitting nominal bit time into three segments as follows:

*   **Synchronization segment (SYNC_SEG):** a bit change is expected to occur within this time segment. It has a fixed length of one time quantum (1 x t_q).
*   **Bit segment 1 (BS1):** defines the location of the sample point. It includes the PROP_SEG and PHASE_SEG1 of the CAN standard. Its duration is programmable between 1 and 16 time quanta but may be automatically lengthened to compensate for positive phase drifts due to differences in the frequency of the various nodes of the network.
*   **Bit segment 2 (BS2):** defines the location of the transmit point. It represents the PHASE_SEG2 of the CAN standard. Its duration is programmable between 1 and 8 time quanta but may also be automatically shortened to compensate for negative phase drifts.

The resynchronization Jump Width (SJW) defines an upper bound to the amount of lengthening or shortening of the bit segments. It is programmable between 1 and 4 time quanta.

ST
RM0090 Rev 18
1093/1749

--- Page 119 ---

**Controller area network (bxCAN)**

RM0090

A valid edge is defined as the first transition in a bit time from dominant to recessive bus level provided the controller itself does not send a recessive bit.

If a valid edge is detected in BS1 instead of SYNC_SEG, BS1 is extended by up to SJW so that the sample point is delayed.

Conversely, if a valid edge is detected in BS2 instead of SYNC_SEG, BS2 is shortened by up to SJW so that the transmit point is moved earlier.

As a safeguard against programming errors, the configuration of the Bit Timing Register (CAN_BTR) is only possible while the device is in Standby mode.

**Note:** *For a detailed description of the CAN bit timing and resynchronization mechanism, refer to the ISO 11898 standard.*

**Figure 346. Bit timing**

BaudRate = 1 / NominalBitTime
NominalBitTime = 1 × tq + tBS1 + tBS2

with:
tBS1 = tq × (TS1[3:0] + 1),
tBS2 = tq × (TS2[2:0] + 1),
tq = (BRP[9:0] + 1) × tPCLK

where tq refers to the Time quantum
tPCLK = time period of the APB clock,

BRP[9:0], TS1[3:0] and TS2[2:0] are defined in the CAN_BTR Register.

1094/1749
RM0090 Rev 18

--- Page 120 ---

RM0090
Controller area network (bxCAN)

**Figure 347. CAN frames**

Inter-Frame Space
Data Frame (Standard identifier)
44 + 8 * N
Inter-Frame Space or Overload Frame
Arbitration Field Ctrl Field Data Field CRC Field Ack Field
32 6 8 * N 16 2 7
ID DLC CRC EOF
SOF RTR IDE r0 ACK

Inter-Frame Space
Data Frame (Extended Identifier)
64 + 8 * N
Inter-Frame Space or Overload Frame
Arbitration Field Arbitration Field Ctrl Field Data Field CRC Field Ack Field
32 32 6 8 * N 16 2 7
ID DLC CRC EOF
SRR IDE RTR r0 ACK

Inter-Frame Space
Remote Frame
44
Inter-Frame Space or Overload Frame
Arbitration Field Ctrl Field CRC Field Ack Field
32 6 16 2 7
ID DLC CRC EOF
SOF RTR IDE r0 ACK

Data Frame or Remote Frame
Error Frame
Inter-Frame Space or Overload Frame
Error Flag Flag Echo Error Delimiter
6 6 8

Any Frame
Inter-Frame Space
Data Frame or Remote Frame
Intermission Suspend Transmission Bus Idle
3 8

End Of Frame or Error Delimiter or Overload Delimiter
Overload Frame
Inter-Frame Space or Error Frame
Overload Flag Overload Echo Overload Delimiter
6 6 8

Notes:
0 <= N <= 8
SOF = Start Of Frame
ID = Identifier
RTR = Remote Transmission Request
IDE = Identifier Extension Bit
r0 = Reserved Bit
DLC = Data Length Code
CRC = Cyclic Redundancy Code
Error flag: 6 dominant bits if node is error active else 6 recessive bits.
Suspend transmission: applies to error passive nodes only.
EOF = End of Frame
ACK = Acknowledge bit
Ctrl = Control

ai15154


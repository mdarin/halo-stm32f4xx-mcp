> [RM0090 ch.32 §32.9] CAN registers


### 32.9.3 CAN mailbox registers

This chapter describes the registers of the transmit and receive mailboxes. Refer to *Section 32.7.5: Message storage* for detailed register mapping.

Transmit and receive mailboxes have the same registers except:

*   The FMI field in the CAN_RDTxR register.
*   A receive mailbox is always write protected.
*   A transmit mailbox is write-enabled only while empty, corresponding TME bit in the CAN_TSR register set.

There are three TX Mailboxes and two RX Mailboxes , as shown in *Figure 349*. Each RX Mailbox allows access to a 3-level depth FIFO, the access being offered only to the oldest received message in the FIFO. Each mailbox consist of four registers.

**Figure 349. RX and TX mailboxes**

| FIFO0 | FIFO1 | Three Tx Mailboxes |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|
| Register |  |  |  |  |  |  |  |  |  |
| CAN_RI0R |  | CAN_RI1R |  | CAN_TI0R |  | CAN_TI1R |  | CAN_TI2R |  |
| CAN_RDT0R |  | CAN_RDT1R |  | CAN_TDT0R |  | CAN_TDT1R |  | CAN_TDT2R |  |
| CAN_RL0R |  | CAN_RL1R |  | CAN_TDL0R |  | CAN_TDL1R |  | CAN_TDL2R |  |
| CAN_RH0R |  | CAN_RH1R |  | CAN_TDH0R |  | CAN_TDH1R |  | CAN_TDH2R |  |


ST
RM0090 Rev 18
1107/1749

--- Page 133 ---

Controller area network (bxCAN)
RM0090

**CAN TX mailbox identifier register (CAN_TxR) (x=0..2)**

Address offsets: 0x180, 0x190, 0x1A0
Reset value: 0xXXXX XXXX (except bit 0, TXRQ = 0)

All TX registers are write protected when the mailbox is pending transmission (TMEx reset).
This register also implements the TX request control (bit 0) - reset value 0.

| 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 |  |  |  |
|---|---|---|---|
| STID[10:0]/EXID[28:18] | EXID[17:13] |  |  |
| rw |  |  |  |
| 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0 |  |  |  |
| EXID[12:0] | IDE | RTR | TXRQ |
| rw |  |  |  |


**Bits 31:21 STID[10:0]/EXID[28:18]**: Standard identifier or extended identifier
The standard identifier or the MSBs of the extended identifier (depending on the IDE bit value).

**Bits 20:3 EXID[17:0]**: Extended identifier
The LSBs of the extended identifier.

**Bit 2 IDE**: Identifier extension
This bit defines the identifier type of message in the mailbox.
0: Standard identifier.
1: Extended identifier.

**Bit 1 RTR**: Remote transmission request
0: Data frame
1: Remote frame

**Bit 0 TXRQ**: Transmit mailbox request
Set by software to request the transmission for the corresponding mailbox.
Cleared by hardware when the mailbox becomes empty.

1108/1749
RM0090 Rev 18
ST

--- Page 134 ---

RM0090
Controller area network (bxCAN)

> [RM0090 ch.32 §32.9] CAN registers

### CAN mailbox data length control and time stamp register (CAN_TDTxR) (x=0..2)

### CAN mailbox data length control and time stamp register (CAN_TDTxR) (x=0..2)

All bits of this register are write protected when the mailbox is not in empty state.

Address offsets: 0x184, 0x194, 0x1A4
Reset value: 0xXXXX XXXX

| 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 |  |  |  |
|---|---|---|---|
| TIME[15:0] |  |  |  |
| rw |  |  |  |
| 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0 |  |  |  |
| Reserved | TGTrw | Reserved | DLC[3:0]rw rw rw rw |


Bits 31:16 **TIME[15:0]**: Message time stamp
This field contains the 16-bit timer value captured at the SOF transmission.

Bits 15:9 Reserved, must be kept at reset value.

Bit 8 **TGT**: Transmit global time
This bit is active only when the hardware is in the Time Trigger Communication mode, TTCM bit of the CAN_MCR register is set.
0: Time stamp TIME[15:0] is not sent.
1: Time stamp TIME[15:0] value is sent in the last two data bytes of the 8-byte message: TIME[7:0] in data byte 7 and TIME[15:8] in data byte 6, replacing the data written in CAN_TDHxR[31:16] register (DATA6[7:0] and DATA7[7:0]). DLC must be programmed as 8 in order these two bytes to be sent over the CAN bus.

Bits 7:4 Reserved, must be kept at reset value.

Bits 3:0 **DLC[3:0]**: Data length code
This field defines the number of data bytes a data frame contains or a remote frame request.
A message can contain from 0 to 8 data bytes, depending on the value in the DLC field.

ST
RM0090 Rev 18
1109/1749

--- Page 135 ---

Controller area network (bxCAN)
RM0090

> [RM0090 ch.32 §32.9] CAN registers

### CAN mailbox data low register (CAN_TDLxR) (x=0..2)

### CAN mailbox data low register (CAN_TDLxR) (x=0..2)

All bits of this register are write protected when the mailbox is not in empty state.

Address offsets: 0x188, 0x198, 0x1A8
Reset value: 0xXXXX XXXX

| 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 |  |
|---|---|
| DATA3[7:0] | DATA2[7:0] |
| rw |  |
| 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0 |  |
| DATA1[7:0] | DATA0[7:0] |
| rw |  |


Bits 31:24 **DATA3[7:0]**: Data byte 3
Data byte 3 of the message.

Bits 23:16 **DATA2[7:0]**: Data byte 2
Data byte 2 of the message.

Bits 15:8 **DATA1[7:0]**: Data byte 1
Data byte 1 of the message.

Bits 7:0 **DATA0[7:0]**: Data byte 0
Data byte 0 of the message.
A message can contain from 0 to 8 data bytes and starts with byte 0.

> [RM0090 ch.32 §32.9] CAN registers

### CAN mailbox data high register (CAN_TDHxR) (x=0..2)

### CAN mailbox data high register (CAN_TDHxR) (x=0..2)

All bits of this register are write protected when the mailbox is not in empty state.

Address offsets: 0x18C, 0x19C, 0x1AC
Reset value: 0xXXXX XXXX

| 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 |  |
|---|---|
| DATA7[7:0] | DATA6[7:0] |
| rw |  |
| 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0 |  |
| DATA5[7:0] | DATA4[7:0] |
| rw |  |


Bits 31:24 **DATA7[7:0]**: Data byte 7
Data byte 7 of the message.
*Note: If TGT of this message and TTCM are active, DATA7 and DATA6 will be replaced by the TIME stamp value.*

Bits 23:16 **DATA6[7:0]**: Data byte 6
Data byte 6 of the message.

Bits 15:8 **DATA5[7:0]**: Data byte 5
Data byte 5 of the message.

Bits 7:0 **DATA4[7:0]**: Data byte 4
Data byte 4 of the message.

1110/1749
RM0090 Rev 18

--- Page 136 ---

RM0090
Controller area network (bxCAN)


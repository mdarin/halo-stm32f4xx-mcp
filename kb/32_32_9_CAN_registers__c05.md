> [RM0090 ch.32 §32.9] CAN registers


### CAN receive FIFO mailbox identifier register (CAN_RIxR) (x=0..1)

Address offsets: 0x1B0, 0x1C0
Reset value: 0xXXXX XXXX

All RX registers are write protected.

| 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 |  |  |  |
|---|---|---|---|
| STID[10:0]/EXID[28:18] | EXID[17:13] |  |  |
| r |  |  |  |
| 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0 |  |  |  |
| EXID[12:0] | IDE | RTR | Res. |
| r |  |  |  |


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

**Bit 0** Reserved, must be kept at reset value.

STI
RM0090 Rev 18
1111/1749

--- Page 137 ---

Controller area network (bxCAN)
RM0090

**CAN receive FIFO mailbox data length control and time stamp register (CAN_RDTxR) (x=0..1)**

Address offsets: 0x1B4, 0x1C4
Reset value: 0xXXXX XXXX

All RX registers are write protected.

| 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 |  |  |  |  |  |
|---|---|---|---|---|---|
| TIME[15:0] |  |  |  |  |  |
| r |  |  |  |  |  |
| 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0 |  |  |  |  |  |
| FMI[7:0] | Reserved | DLC[3:0] |  |  |  |
| r |  |  |  |  | r |


**Bits 31:16 TIME[15:0]:** Message time stamp
This field contains the 16-bit timer value captured at the SOF detection.

**Bits 15:8 FMI[7:0]:** Filter match index
This register contains the index of the filter the message stored in the mailbox passed through. For more details on identifier filtering refer to Section 32.7.4

**Bits 7:4** Reserved, must be kept at reset value.

**Bits 3:0 DLC[3:0]:** Data length code
This field defines the number of data bytes a data frame contains (0 to 8). It is 0 in the case of a remote frame request.

1112/1749
RM0090 Rev 18
ST

--- Page 138 ---

RM0090
Controller area network (bxCAN)

> [RM0090 ch.32 §32.9] CAN registers

### CAN receive FIFO mailbox data low register (CAN_RDLxR) (x=0..1)

### CAN receive FIFO mailbox data low register (CAN_RDLxR) (x=0..1)

All bits of this register are write protected when the mailbox is not in empty state.
Address offsets: 0x1B8, 0x1C8
Reset value: 0xXXXX XXXX
All RX registers are write protected.

| 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 |  |
|---|---|
| DATA3[7:0] | DATA2[7:0] |
| r |  |
| 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0 |  |
| DATA1[7:0] | DATA0[7:0] |
| r |  |


Bits 31:24 **DATA3[7:0]**: Data Byte 3
Data byte 3 of the message.
Bits 23:16 **DATA2[7:0]**: Data Byte 2
Data byte 2 of the message.
Bits 15:8 **DATA1[7:0]**: Data Byte 1
Data byte 1 of the message.
Bits 7:0 **DATA0[7:0]**: Data Byte 0
Data byte 0 of the message.
A message can contain from 0 to 8 data bytes and starts with byte 0.

> [RM0090 ch.32 §32.9] CAN registers

### CAN receive FIFO mailbox data high register (CAN_RDHxR) (x=0..1)

### CAN receive FIFO mailbox data high register (CAN_RDHxR) (x=0..1)

Address offsets: 0x1BC, 0x1CC
Reset value: 0xXXXX XXXX
All RX registers are write protected.

| 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 |  |
|---|---|
| DATA7[7:0] | DATA6[7:0] |
| r |  |
| 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0 |  |
| DATA5[7:0] | DATA4[7:0] |
| r |  |


Bits 31:24 **DATA7[7:0]**: Data Byte 7
Data byte 3 of the message.
Bits 23:16 **DATA6[7:0]**: Data Byte 6
Data byte 2 of the message.
Bits 15:8 **DATA5[7:0]**: Data Byte 5
Data byte 1 of the message.
Bits 7:0 **DATA4[7:0]**: Data Byte 4
Data byte 0 of the message.

ST
RM0090 Rev 18
1113/1749

--- Page 139 ---

Controller area network (bxCAN)
RM0090

> [RM0090 ch.32 §32.9] CAN registers

### 32.9.4 CAN filter registers

### 32.9.4 CAN filter registers

**CAN filter master register (CAN_FMR)**

Address offset: 0x200
Reset value: 0x2A1C 0E01

All bits of this register are set and cleared by software.

| 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 |  |  |  |
|---|---|---|---|
| Reserved |  |  |  |
| 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0 |  |  |  |
| Reserved | CAN2SB[5:0] | Reserved | FINIT |
| rw |  |  |  |


Bits 31:14 Reserved, must be kept at reset value.

Bits 13:8 **CAN2SB[5:0]**: CAN2 start bank
These bits are set and cleared by software. They define the start bank for the CAN2 interface (Slave) in the range 0 to 27.
*Note: When CAN2SB[5:0] = 28d, all the filters to CAN1 can be used.*
*When CAN2SB[5:0] is set to 0, no filters are assigned to CAN1.*

Bits 7:1 Reserved, must be kept at reset value.

Bit 0 **FINIT**: Filter init mode
Initialization mode for filter banks
0: Active filters mode.
1: Initialization mode for the filters.

1114/1749
RM0090 Rev 18
STI

--- Page 140 ---

**RM0090**
**Controller area network (bxCAN)**

> [RM0090 ch.32 §32.9] CAN registers

### CAN filter mode register (CAN_FM1R)

### CAN filter mode register (CAN_FM1R)

Address offset: 0x204
Reset value: 0x0000 0000

This register can be written only when the filter initialization mode is set (FINIT=1) in the CAN_FMR register.

| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved | FBM27 | FBM26 | FBM25 | FBM24 | FBM23 | FBM22 | FBM21 | FBM20 | FBM19 | FBM18 | FBM17 | FBM16 |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | rw |
| 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |  |
| FBM15 | FBM14 | FBM13 | FBM12 | FBM11 | FBM10 | FBM9 | FBM8 | FBM7 | FBM6 | FBM5 | FBM4 | FBM3 | FBM2 | FBM1 | FBM0 |  |
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


**Note:** *Refer to Figure 342.*

Bits 31:28 Reserved, must be kept at reset value.
Bits 27:0 **FBMx**: Filter mode
Mode of the registers of Filter x.
0: Two 32-bit registers of filter bank x are in Identifier Mask mode.
1: Two 32-bit registers of filter bank x are in Identifier List mode.


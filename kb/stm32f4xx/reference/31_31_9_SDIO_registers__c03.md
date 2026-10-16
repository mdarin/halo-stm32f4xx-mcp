> [RM0090 ch.31 §31.9] SDIO registers**


### 31.9.4 SDIO command register (SDIO\_CMD)

Address offset: 0x0C
Reset value: 0x0000 0000

The SDIO\_CMD register contains the command index and command type bits. The command index is sent to a card as part of a command message. The command type bits control the command path state machine (CPSM).

| Reserved | CE-ATACMD | nIEN | ENCMDcompl | SDIO/Suspend | CPSMEN | WAITPEND | WAITINT | WAITRESP | CMDINDEX |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bits 31:15 Reserved, must be kept at reset value
Bit 14 **ATACMD**: CE-ATA command
If ATACMD is set, the CPSM transfers CMD61.
Bit 13 **nIEN**: not Interrupt Enable
If this bit is 0, interrupts in the CE-ATA device are enabled.
Bit 12 **ENCMDcompl**: Enable CMD completion
If this bit is set, the command completion signal is enabled.

1062/1749
RM0090 Rev 18

--- Page 88 ---

RM0090
Secure digital input/output interface (SDIO)

Bit 11 **SDIOSuspend**: SD I/O suspend command
If this bit is set, the command to be sent is a suspend command (to be used only with SDIO card).

Bit 10 **CPSMEN**: Command path state machine (CPSM) Enable bit
If this bit is set, the CPSM is enabled.

Bit 9 **WAITPEND**: CPSM Waits for ends of data transfer (CmdPend internal signal).
If this bit is set, the CPSM waits for the end of data transfer before it starts sending a command.

Bit 8 **WAITINT**: CPSM waits for interrupt request
If this bit is set, the CPSM disables command timeout and waits for an interrupt request.

Bits 7:6 **WAITRESP**: Wait for response bits
They are used to configure whether the CPSM is to wait for a response, and if yes, which kind of response.
00: No response, expect CMDSENT flag
01: Short response, expect CMDREND or CCRCFAIL flag
10: No response, expect CMDSENT flag
11: Long response, expect CMDREND or CCRCFAIL flag

Bits 5:0 **CMDINDEX**: Command index
The command index is sent to the card as part of a command message.

*Note:* After a data write, data cannot be written to this register for three SDIOCLK clock periods plus two PCLK2 clock periods.
MultiMediaCards can send two kinds of response: short responses, 48 bits long, or long responses, 136 bits long. SD card and SD I/O card can send only short responses, the argument can vary according to the type of response: the software will distinguish the type of response according to the sent command. CE-ATA devices send only short responses.

> [RM0090 ch.31 §31.9] SDIO registers**

### 31.9.5 SDIO command response register (SDIO_RESPCMD)

### 31.9.5 SDIO command response register (SDIO_RESPCMD)

Address offset: 0x10
Reset value: 0x0000 0000

The SDIO_RESPCMD register contains the command index field of the last command response received. If the command response transmission does not contain the command index field (long or OCR response), the RESP CMD field is unknown, although it must contain 111111b (the value of the reserved field from the response).

| Reserved | RESPCMD |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
| r |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bits 31:6 Reserved, must be kept at reset value
Bits 5:0 **RESPCMD**: Response command index
Read-only bit field. Contains the command index of the last command response received.

ST
RM0090 Rev 18
1063/1749

--- Page 89 ---

**Secure digital input/output interface (SDIO)**
RM0090

> [RM0090 ch.31 §31.9] SDIO registers**

### 31.9.6 SDIO response 1..4 register (SDIO_RESPx)

### 31.9.6 SDIO response 1..4 register (SDIO_RESPx)

Address offset: (0x10 + (4 × x)); x = 1..4
Reset value: 0x0000 0000

The SDIO_RESP1/2/3/4 registers contain the status of a card, which is part of the received response.

| CARDSTATUSx |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
| r |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bits 31:0 **CARDSTATUSx**: see Table 180.

The Card Status size is 32 or 127 bits, depending on the response type.

**Table 180. Response type and SDIO_RESPx registers**

| Register | Short response | Long response |
|---|---|---|
| SDIO_RESP1 | Card Status[31:0] | Card Status [127:96] |
| SDIO_RESP2 | Unused | Card Status [95:64] |
| SDIO_RESP3 | Unused | Card Status [63:32] |
| SDIO_RESP4 | Unused | Card Status [31:1]0b |


The most significant bit of the card status is received first. The SDIO_RESP3 register LSB is always 0b.

> [RM0090 ch.31 §31.9] SDIO registers**

### 31.9.7 SDIO data timer register (SDIO_DTIMER)

### 31.9.7 SDIO data timer register (SDIO_DTIMER)

Address offset: 0x24
Reset value: 0x0000 0000

The SDIO_DTIMER register contains the data timeout period, in card bus clock periods.

A counter loads the value from the SDIO_DTIMER register, and starts decrementing when the data path state machine (DPSM) enters the Wait_R or Busy state. If the timer reaches 0 while the DPSM is in either of these states, the timeout status flag is set.

| DATATIME |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bits 31:0 **DATATIME**: Data timeout period
Data timeout period expressed in card bus clock periods.

*Note:* A data transfer must be written to the data timer register and the data length register before being written to the data control register.

1064/1749
RM0090 Rev 18
ST

--- Page 90 ---

RM0090
Secure digital input/output interface (SDIO)

> [RM0090 ch.31 §31.9] SDIO registers**

### 31.9.8 SDIO data length register (SDIO_DLEN)

### 31.9.8 SDIO data length register (SDIO_DLEN)

Address offset: 0x28
Reset value: 0x0000 0000

The SDIO_DLEN register contains the number of data bytes to be transferred. The value is loaded into the data counter when data transfer starts.

| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved | DATALNGTH |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  | rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bits 31:25 Reserved, must be kept at reset value
Bits 24:0 **DATALNGTH**: Data length value
Number of data bytes to be transferred.

**Note:**
For a block data transfer, the value in the data length register must be a multiple of the block size (see SDIO_DCTRL). A data transfer must be written to the data timer register and the data length register before being written to the data control register.
For an SDIO multibyte transfer the value in the data length register must be between 1 and 512.

ST
RM0090 Rev 18
1065/1749

--- Page 91 ---

Secure digital input/output interface (SDIO)
RM0090


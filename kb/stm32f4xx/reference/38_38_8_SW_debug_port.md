### 38.8 SW debug port

#### 38.8.1 SW protocol introduction

This synchronous serial protocol uses two pins:
* SWCLK: clock from host to target
* SWDIO: bidirectional

The protocol allows two banks of registers (DPACC registers and APACC registers) to be read and written to.

Bits are transferred LSB-first on the wire.

For SWDIO bidirectional management, the line must be pulled-up on the board (100 KΩ recommended by Arm®).

Each time the direction of SWDIO changes in the protocol, a turnaround time is inserted where the line is not driven by the host nor the target. By default, this turnaround time is one bit time, however this can be adjusted by configuring the SWCLK frequency.

#### 38.8.2 SW protocol sequence

Each sequence consist of three phases:
1. Packet request (8 bits) transmitted by the host
2. Acknowledge response (3 bits) transmitted by the target
3. Data transfer phase (33 bits) transmitted by the host or the target

Table 302. Packet request (8-bits)

| Bit | Name | Description |
|---|---|---|
| 0 | Start | Must be “1” |
| 1 | APnDP | 0: DP Access1: AP Access |
| 2 | RnW | 0: Write Request1: Read Request |
| 4:3 | A[3:2] | Address field of the DP or AP registers (refer to Table 301) |
| 5 | Parity | Single bit parity of preceding bits |
| 6 | Stop | 0 |
| 7 | Park | Not driven by the host. Must be read as “1” by the target because of the pull-up |


Refer to the Cortex®-M4 with FPU r0p1 TRM for a detailed description of DPACC and APACC registers.

The packet request is always followed by the turnaround time (default 1 bit) where neither the host nor target drive the line.

ST
RM0090 Rev 18
1693/1749

--- Page 134 ---

Debug support (DBG) RM0090

**Table 303. ACK response (3 bits)**

| Bit | Name | Description |
|---|---|---|
| 0..2 | ACK | 001: FAULT010: WAIT100: OK |


The ACK Response must be followed by a turnaround time only if it is a READ transaction or if a WAIT or FAULT acknowledge has been received.

**Table 304. DATA transfer (33 bits)**

| Bit | Name | Description |
|---|---|---|
| 0..31 | WDATA or RDATA | Write or Read data |
| 32 | Parity | Single parity of the 32 data bits |


The DATA transfer must be followed by a turnaround time only if it is a READ transaction.

### 38.8.3 SW-DP state machine (reset, idle states, ID code)

The State Machine of the SW-DP has an internal ID code which identifies the SW-DP. It follows the JEP-106 standard. This ID code is the default Arm® one and is set to **0x2BA01477** (corresponding to Cortex®-M4 with FPU r0p1).

*Note:* Note that the SW-DP state machine is inactive until the target reads this ID code.
* The SW-DP state machine is in RESET STATE either after power-on reset, or after the DP has switched from JTAG to SWD or after the line is high for more than 50 cycles
* The SW-DP state machine is in IDLE STATE if the line is low for at least two cycles after RESET state.
* After RESET state, it is **mandatory** to first enter into an IDLE state AND to perform a READ access of the DP-SW ID CODE register. Otherwise, the target will issue a FAULT acknowledge response on another transactions.

Further details of the SW-DP state machine can be found in the Cortex®-M4 with FPU r0p1 TRM and the CoreSight Design Kit r0p1 TRM.

### 38.8.4 DP and AP read/write accesses

* Read accesses to the DP are not posted: the target response can be immediate (if ACK=OK) or can be delayed (if ACK=WAIT).
* Read accesses to the AP are posted. This means that the result of the access is returned on the next transfer. If the next access to be done is NOT an AP access, then the DP-RDBUFF register must be read to obtain the result. The READOK flag of the DP-CTRL/STAT register is updated on every AP read access or RDBUFF read request to know if the AP read access was successful.
* The SW-DP implements a write buffer (for both DP or AP writes), that enables it to accept a write operation even when other transactions are still outstanding. If the write buffer is full, the target acknowledge response is “WAIT”. With the exception of

1694/1749 RM0090 Rev 18

--- Page 135 ---

RM0090
Debug support (DBG)

IDCODE read or CTRL/STAT read or ABORT write which are accepted even if the write buffer is full.

*   Because of the asynchronous clock domains SWCLK and HCLK, two extra SWCLK cycles are needed after a write transaction (after the parity bit) to make the write effective internally. These cycles should be applied while driving the line low (IDLE state)
    This is particularly important when writing the CTRL/STAT for a power-up request. If the next transaction (requiring a power-up) occurs immediately, it will fail.

### 38.8.5 SW-DP registers

Access to these registers are initiated when APnDP=0

Table 305. SW-DP registers

| A[3:2] | R/W | CTRLSEL bit of SELECT register | Register | Notes |
|---|---|---|---|---|
| 00 | Read | - | IDCODE | The manufacturer code is not set to ST code. 0x2BA01477 (identifies the SW-DP) |
| 00 | Write | - | ABORT | - |
| 01 | Read/Write | 0 | DP-CTRL/STAT | Purpose is to: - request a system or debug power-up - configure the transfer operation for AP accesses - control the pushed compare and pushed verify operations. - read some status flags (overrun, power-up acknowledges) |
| 01 | Read/Write | 1 | WIRE CONTROL | Purpose is to configure the physical serial port protocol (like the duration of the turnaround time) |
| 10 | Read | - | READ RESEND | Enables recovery of the read data from a corrupted debugger transfer, without repeating the original AP transfer. |
| 10 | Write | - | SELECT | The purpose is to select the current access port and the active 4-words register window |
| 11 | Read/Write | - | READ BUFFER | This read buffer is useful because AP accesses are posted (the result of a read AP request is available on the next AP transaction). This read buffer captures data from the AP, presented as the result of a previous read, without initiating a new transaction |


RM0090 Rev 18
1695/1749

--- Page 136 ---

Debug support (DBG)
RM0090

### 38.8.6 SW-AP registers

Access to these registers are initiated when APnDP=1

There are many AP Registers (see AHB-AP) addressed as the combination of:
* The shifted value A[3:2]
* The current value of the DP SELECT register


### 34.16.3 Host-mode registers

Bit values in the register descriptions are expressed in binary unless otherwise specified.

Host-mode registers affect the operation of the core in the host mode. Host mode registers must not be accessed in device mode, as the results are undefined. Host mode registers can be categorized as follows:

#### OTG_FS Host configuration register (OTG_FS_HCFG)

Address offset: 0x400
Reset value: 0x0000 0000

This register configures the core after power-on. Do not make changes to this register after initializing the host.

| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved | FSLSS | FSLSPCS |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | r | r/w |  |


Bits 31:3 Reserved, must be kept at reset value.

**Bit 2 FSLSS:** FS- and LS-only support
The application uses this bit to control the core's enumeration speed. Using this bit, the application can make the core enumerate as an FS host, even if the connected device supports HS traffic. Do not make changes to this field after initial programming.
1: FS/LS-only, even if the connected device can support HS (read-only)

**Bits 1:0 FSLSPCS:** FS/LS PHY clock select
When the core is in FS host mode
01: PHY clock is running at 48 MHz
Others: Reserved
When the core is in LS host mode
00: Reserved
01: Select 48 MHz PHY clock frequency
10: Select 6 MHz PHY clock frequency
11: Reserved

*Note: The FSLSPCS must be set on a connection event according to the speed of the connected device (after changing this bit, a software reset must be performed).*

#### OTG_FS Host frame interval register (OTG_FS_HFIR)

Address offset: 0x404
Reset value: 0x0000 EA60

This register stores the frame interval information for the current speed to which the OTG_FS controller has enumerated.

| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved | FRIVL |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | r/w |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


1292/1749
RM0090 Rev 18

--- Page 123 ---

RM0090
USB on-the-go full-speed (OTG_FS)

Bits 31:16 Reserved, must be kept at reset value.

Bits 15:0 **FRIVL**: Frame interval
The value that the application programs to this field specifies the interval between two consecutive SOFs (FS) or Keep-Alive tokens (LS). This field contains the number of PHY clocks that constitute the required frame interval. The application can write a value to this register only after the Port enable bit of the host port control and status register (PENA bit in OTG_FS_HPRT) has been set. If no value is programmed, the core calculates the value based on the PHY clock specified in the FS/LS PHY Clock Select field of the host configuration register (FSLSPCS in OTG_FS_HCFG). Do not change the value of this field after the initial configuration.
– Frame interval = 1 ms × (FRIVL - 1)

**OTG_FS Host frame number/frame time remaining register (OTG_FS_HFNUM)**

Address offset: 0x408
Reset value: 0x0000 3FFF

This register indicates the current frame number. It also indicates the time remaining (in terms of the number of PHY clocks) in the current frame.

| FTREM | FRNUM |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
| r |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bits 31:16 **FTREM**: Frame time remaining
Indicates the amount of time remaining in the current frame, in terms of PHY clocks. This field decrements on each PHY clock. When it reaches zero, this field is reloaded with the value in the Frame interval register and a new SOF is transmitted on the USB.

Bits 15:0 **FRNUM**: Frame number
This field increments when a new SOF is transmitted on the USB, and is cleared to 0 when it reaches 0x3FFF.

**OTG_FS_Host periodic transmit FIFO/queue status register (OTG_FS_HPTXSTS)**

Address offset: 0x410
Reset value: 0x0008 0100

This read-only register contains the free space information for the periodic TxFIFO and the periodic transmit request queue.

| PTXQTOP | PTXQSAV | PTXFS AVL |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
| r |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


ST
RM0090 Rev 18
1293/1749

--- Page 124 ---

**USB on-the-go full-speed (OTG_FS)**

RM0090

Bits 31:24 **PTXQTOP**: Top of the periodic transmit request queue
This indicates the entry in the periodic Tx request queue that is currently being processed by the MAC.
This register is used for debugging.
Bit 31: Odd/Even frame
- 0: send in even frame
- 1: send in odd frame
Bits 30:27: Channel/endpoint number
Bits 26:25: Type
- 00: IN/OUT
- 01: Zero-length packet
- 11: Disable channel command
Bit 24: Terminate (last entry for the selected channel/endpoint)

Bits 23:16 **PTXQSAV**: Periodic transmit request queue space available
Indicates the number of free locations available to be written in the periodic transmit request queue. This queue holds both IN and OUT requests.
00: Periodic transmit request queue is full
01: 1 location available
10: 2 locations available
bxn: n locations available (0 ≤ n ≤ 8)
Others: Reserved

Bits 15:0 **PTXFSALV**: Periodic transmit data FIFO space available
Indicates the number of free locations available to be written to in the periodic TxFIFO.
Values are in terms of 32-bit words
0000: Periodic TxFIFO is full
0001: 1 word available
0010: 2 words available
bxn: n words available (where 0 ≤ n ≤ PTXFD)
Others: Reserved

**OTG_FS Host all channels interrupt register (OTG_FS_HAINT)**

Address offset: 0x414

Reset value: 0x0000 000

When a significant event occurs on a channel, the host all channels interrupt register interrupts the application using the host channels interrupt bit of the Core interrupt register (HCINT bit in OTG_FS_GINTSTS). This is shown in *Figure 394*. There is one interrupt bit per channel, up to a maximum of 16 bits. Bits in this register are set and cleared when the application sets and clears bits in the corresponding host channel-x interrupt register.

| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved | HAINT |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| r |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bits 31:16 Reserved, must be kept at reset value.

Bits 15:0 **HAINT**: Channel interrupts
One bit per channel: Bit 0 for Channel 0, bit 15 for Channel 15

1294/1749 RM0090 Rev 18

--- Page 125 ---

RM0090
USB on-the-go full-speed (OTG_FS)

**OTG_FS Host all channels interrupt mask register (OTG_FS_HAINTMSK)**

Address offset: 0x418
Reset value: 0x0000 0000

The host all channel interrupt mask register works with the host all channel interrupt register to interrupt the application when an event occurs on a channel. There is one interrupt mask bit per channel, up to a maximum of 16 bits.

| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved | HAINTM |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bits 31:16 Reserved, must be kept at reset value.
Bits 15:0 **HAINTM**: Channel interrupt mask
0: Masked interrupt
1: Unmasked interrupt
One bit per channel: Bit 0 for channel 0, bit 15 for channel 15

**OTG_FS Host port control and status register (OTG_FS_HPRT)**

Address offset: 0x440
Reset value: 0x0000 0000

This register is available only in host mode. Currently, the OTG host supports only one port.

A single register holds USB port-related information such as USB reset, enable, suspend, resume, connect status, and test mode for each port. It is shown in Figure 394. The rc_w1 bits in this register can trigger an interrupt to the application through the host port interrupt bit of the core interrupt register (HPRTINT bit in OTG_FS_GINTSTS). On a Port Interrupt, the application must read this register and clear the bit that caused the interrupt. For the rc_w1 bits, the application must write a 1 to the bit to clear the interrupt.

| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved | PSPD | PTCTL | PPWR | PLSTS | Reserved | PRST | PSUSP | PRES | POCCHNG | POCA | PENCHNG | PENA | PCDET | PCSTS |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  |  | r | rw | r | rw | rs | rw | rc_w1 | r | rc_w1 | rc_w0 | rc_w1 | r |  |  |  |  |  |  |  |


Bits 31:19 Reserved, must be kept at reset value.
Bits 18:17 **PSPD**: Port speed
Indicates the speed of the device attached to this port.
01: Full speed
10: Low speed
11: Reserved

STI
RM0090 Rev 18
1295/1749

--- Page 126 ---

**USB on-the-go full-speed (OTG_FS)**
RM0090

**Bits 16:13 PTCTL: Port test control**
The application writes a nonzero value to this field to put the port into a Test mode, and the corresponding pattern is signaled on the port.
0000: Test mode disabled
0001: Test_J mode
0010: Test_K mode
0011: Test_SE0_NAK mode
0100: Test_Packet mode
0101: Test_Force_Enable
Others: Reserved

**Bit 12 PPWR: Port power**
The application uses this field to control power to this port, and the core clears this bit on an overcurrent condition.
0: Power off
1: Power on

**Bits 11:10 PLSTS: Port line status**
Indicates the current logic level USB data lines
Bit 10: Logic level of OTG_FS_DP
Bit 11: Logic level of OTG_FS_DM

**Bit 9** Reserved, must be kept at reset value.

**Bit 8 PRST: Port reset**
When the application sets this bit, a reset sequence is started on this port. The application must time the reset period and clear this bit after the reset sequence is complete.
0: Port not in reset
1: Port in reset
The application must leave this bit set for a minimum duration of at least 10 ms to start a reset on the port. The application can leave it set for another 10 ms in addition to the required minimum duration, before clearing the bit, even though there is no maximum limit set by the USB standard.

**Bit 7 PSUSP: Port suspend**
The application sets this bit to put this port in Suspend mode. The core only stops sending SOFs when this is set. To stop the PHY clock, the application must set the Port clock stop bit, which asserts the suspend input pin of the PHY.
The read value of this bit reflects the current suspend status of the port. This bit is cleared by the core after a remote wakeup signal is detected or the application sets the Port reset bit or Port resume bit in this register or the Resume/remote wakeup detected interrupt bit or Disconnect detected interrupt bit in the Core interrupt register (WKUINT or DISCINT in OTG_FS_GINTSTS, respectively).
0: Port not in Suspend mode
1: Port in Suspend mode

**Bit 6 PRES: Port resume**
The application sets this bit to drive resume signaling on the port. The core continues to drive the resume signal until the application clears this bit.
If the core detects a USB remote wakeup sequence, as indicated by the Port resume/remote wakeup detected interrupt bit of the Core interrupt register (WKUINT bit in OTG_FS_GINTSTS), the core starts driving resume signaling without application intervention and clears this bit when it detects a disconnect condition. The read value of this bit indicates whether the core is currently driving resume signaling.
0: No resume driven
1: Resume driven

1296/1749
RM0090 Rev 18

--- Page 127 ---

RM0090
USB on-the-go full-speed (OTG_FS)

**Bit 5 POCCHNG**: Port overcurrent change
The core sets this bit when the status of the Port overcurrent active bit (bit 4) in this register changes.

**Bit 4 POCA**: Port overcurrent active
Indicates the overcurrent condition of the port.
0: No overcurrent condition
1: Overcurrent condition

**Bit 3 PENCHNG**: Port enable/disable change
The core sets this bit when the status of the Port enable bit 2 in this register changes.

**Bit 2 PENA**: Port enable
A port is enabled only by the core after a reset sequence, and is disabled by an overcurrent condition, a disconnect condition, or by the application clearing this bit. The application cannot set this bit by a register write. It can only clear it to disable the port. This bit does not trigger any interrupt to the application.
0: Port disabled
1: Port enabled

**Bit 1 PCDET**: Port connect detected
The core sets this bit when a device connection is detected to trigger an interrupt to the application using the host port interrupt bit in the Core interrupt register (HPRTINT bit in OTG_FS_GINTSTS). The application must write a 1 to this bit to clear the interrupt.

**Bit 0 PCSTS**: Port connect status
0: No device is attached to the port
1: A device is attached to the port

ST
RM0090 Rev 18
1297/1749

--- Page 128 ---

USB on-the-go full-speed (OTG_FS)
RM0090

**OTG_FS Host channel-x characteristics register (OTG_FS_HCCHARx)**
(x = 0..7, where x = Channel_number)

Address offset: 0x500 + 0x20 * x
Reset value: 0x0000 0000

| 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0 |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|
| CHENA | CHDIS | ODDFRM | DAD | MCNT | EPTYP | LSDEV | Reserved | EPDIR | EPNUM | MPSIZ |
| rs | rw |  |  |  |  |  |  |  |  |  |


**Bit 31 CHENA: Channel enable**
This field is set by the application and cleared by the OTG host.
0: Channel disabled
1: Channel enabled

**Bit 30 CHDIS: Channel disable**
The application sets this bit to stop transmitting/receiving data on a channel, even before the transfer for that channel is complete. The application must wait for the Channel disabled interrupt before treating the channel as disabled.

**Bit 29 ODDFRM: Odd frame**
This field is set (reset) by the application to indicate that the OTG host must perform a transfer in an odd frame. This field is applicable for only periodic (isochronous and interrupt) transactions.
0: Even frame
1: Odd frame

**Bits 28:22 DAD: Device address**
This field selects the specific device serving as the data source or sink.

**Bits 21:20 MCNT: Multicount**
This field indicates to the host the number of transactions that must be executed per frame for this periodic endpoint. For non-periodic transfers, this field is not used
00: Reserved. This field yields undefined results
01: 1 transaction
10: 2 transactions per frame to be issued for this endpoint
11: 3 transactions per frame to be issued for this endpoint
*Note: This field must be set to at least 01.*

**Bits 19:18 EPTYP: Endpoint type**
Indicates the transfer type selected.
00: Control
01: Isochronous
10: Bulk
11: Interrupt

**Bit 17 LSDEV: Low-speed device**
This field is set by the application to indicate that this channel is communicating to a low-speed device.

**Bit 16** Reserved, must be kept at reset value.

1298/1749
RM0090 Rev 18
STI

--- Page 129 ---

RM0090
USB on-the-go full-speed (OTG_FS)

Bit 15 **EPDIR**: Endpoint direction
Indicates whether the transaction is IN or OUT.
0: OUT
1: IN

Bits 14:11 **EPNUM**: Endpoint number
Indicates the endpoint number on the device serving as the data source or sink.

Bits 10:0 **MPSIZ**: Maximum packet size
Indicates the maximum packet size of the associated endpoint.

**OTG_FS Host channel-x interrupt register (OTG_FS_HCINTx) (x = 0..7, where x = Channel_number)**

Address offset: 0x508 + 0x20 * x
Reset value: 0x0000 0000

This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure 394. The application must read this register when the host channels interrupt bit in the Core interrupt register (HCINT bit in OTG_FS_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_FS_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_FS_HAINT and OTG_FS_GINTSTS registers.

| 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved | DTERR | FRMOR | BBERR | TXERR | Reserved | ACK | NAK | STALL | Reserved | CHH | XFRC |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | rc_w1 | Reserved | rc_w1 | Reserved | rc_w1 |


Bits 31:11 Reserved, must be kept at reset value.

Bit 10 **DTERR**: Data toggle error

Bit 9 **FRMOR**: Frame overrun

Bit 8 **BBERR**: Babble error

Bit 7 **TXERR**: Transaction error
Indicates one of the following errors occurred on the USB.
CRC check failure
Timeout
Bit stuff error
False EOP

Bit 6 Reserved, must be kept at reset value.

Bit 5 **ACK**: ACK response received/transmitted interrupt

Bit 4 **NAK**: NAK response received interrupt

Bit 3 **STALL**: STALL response received interrupt

STI
RM0090 Rev 18
1299/1749

--- Page 130 ---

**USB on-the-go full-speed (OTG_FS)**
RM0090

Bit 2 Reserved, must be kept at reset value.
Bit 1 **CHH**: Channel halted
Indicates the transfer completed abnormally either because of any USB transaction error or in response to disable request by the application.
Bit 0 **XFRC**: Transfer completed
Transfer completed normally without any errors.

**OTG_FS Host channel-x interrupt mask register (OTG_FS_HCINTMSKx)**
(x = 0..7, where x = Channel_number)

Address offset: 0x50C + 0x20 * x
Reset value: 0x0000 0000
This register reflects the mask for each channel status described in the previous section.

| 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0 |
|---|
| Reserved |
| DTERRM FRMORM BBERRM TXERRM Reserved ACKM NAKM STALLM Reserved CHHM XFRC |
| rw rw rw rw rw rw rw rw rw rw rw |


Bits 31:11 Reserved, must be kept at reset value.
Bit 10 **DTERRM**: Data toggle error mask
0: Masked interrupt
1: Unmasked interrupt
Bit 9 **FRMORM**: Frame overrun mask
0: Masked interrupt
1: Unmasked interrupt
Bit 8 **BBERRM**: Babble error mask
0: Masked interrupt
1: Unmasked interrupt
Bit 7 **TXERRM**: Transaction error mask
0: Masked interrupt
1: Unmasked interrupt
Bit 6 Reserved, must be kept at reset value.
Bit 5 **ACKM**: ACK response received/transmitted interrupt mask
0: Masked interrupt
1: Unmasked interrupt
Bit 4 **NAKM**: NAK response received interrupt mask
0: Masked interrupt
1: Unmasked interrupt
Bit 3 **STALLM**: STALL response received interrupt mask
0: Masked interrupt
1: Unmasked interrupt

1300/1749
RM0090 Rev 18

--- Page 131 ---

RM0090
USB on-the-go full-speed (OTG_FS)

Bit 2 Reserved, must be kept at reset value.
Bit 1 **CHHM**: Channel halted mask
0: Masked interrupt
1: Unmasked interrupt
Bit 0 **XFR CM**: Transfer completed mask
0: Masked interrupt
1: Unmasked interrupt

**OTG_FS Host channel-x transfer size register (OTG_FS_HCTSIZx) (x = 0..7, where x = Channel_number)**

Address offset: 0x510 + 0x20 * x
Reset value: 0x0000 0000

| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved | DPID | PKTCNT | XFRSIZ |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bit 31 Reserved, must be kept at reset value.
Bits 30:29 **DPID**: Data PID
The application programs this field with the type of PID to use for the initial transaction. The host maintains this field for the rest of the transfer.
00: DATA0
01: DATA2
10: DATA1
11: MDATA (non-control)/SETUP (control)

Bits 28:19 **PKTCNT**: Packet count
This field is programmed by the application with the expected number of packets to be transmitted (OUT) or received (IN).
The host decrements this count on every successful transmission or reception of an OUT/IN packet. Once this count reaches zero, the application is interrupted to indicate normal completion.

Bits 18:0 **XFRSIZ**: Transfer size
For an OUT, this field is the number of data bytes the host sends during the transfer.
For an IN, this field is the buffer size that the application has reserved for the transfer. The application is expected to program this field as an integer multiple of the maximum packet size for IN transactions (periodic and non-periodic).

ST
RM0090 Rev 18
1301/1749

--- Page 132 ---

USB on-the-go full-speed (OTG_FS) RM0090


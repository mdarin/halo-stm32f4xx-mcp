> [RM0090 ch.35 §35.12.2] OTG_HS global registers


## OTG_HS core interrupt register (OTG_HS_GINTSTS)

Address offset: 0x014
Reset value: 0x0400 0020

This register interrupts the application for system-level events in the current mode (peripheral mode or host mode).

Some of the bits in this register are valid only in host mode, while others are valid in peripheral mode only. This register also indicates the current mode. To clear the interrupt status bits of the rc_w1 type, the application must write 1 into the bit.

The FIFO status interrupts are read-only; once software reads from or writes to the FIFO while servicing these interrupts, FIFO interrupt conditions are cleared automatically.

The application must clear the OTG_HS_GINTSTS register at initialization before unmasking the interrupt bit to avoid any interrupts generated prior to initialization.

| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| WKUINT | SRQINT | DISCINT | CIDSCHG | Reserved | PTXFE | HCINT | HPRTINT | Reserved | DATAFUSP | IPXFR/COMPISOUT | IISOIXFR | OEPINT | IEPINT | Reserved | EQPF | ISOODRP | ENUMDNE | USBRST | USBSUSP | ESUSP | Reserved | GONAKEFF | GINAKEFF | NPTXFE | RXFVL | SOF | OTGINT | MMIS | CMOD |  |  |
| rc_w1 |  |  |  |  | r |  |  | rc_w1 | r |  |  |  | rc_w1 |  | r | rc_w1 | r | rc_w1 | r |  |  |  |  |  |  |  |  |  |  |  |  |


**Bit 31 WKUPINT:** Resume/remote wakeup detected interrupt
In peripheral mode, this interrupt is asserted when a resume is detected on the USB. In host mode, this interrupt is asserted when a remote wakeup is detected on the USB.
*Note: Accessible in both peripheral and host modes.*

**Bit 30 SRQINT:** Session request/new session detected interrupt
In host mode, this interrupt is asserted when a session request is detected from the device. In peripheral mode, this interrupt is asserted when VBUS is in the valid range for a B-device device. Accessible in both peripheral and host modes.

**Bit 29 DISCINT:** Disconnect detected interrupt
Asserted when a device disconnect is detected.
*Note: Only accessible in host mode.*

**Bit 28 CIDSCHG:** Connector ID status change
The core sets this bit when there is a change in connector ID status.
*Note: Accessible in both peripheral and host modes.*

**Bit 27** Reserved, must be kept at reset value.

**Bit 26 PTXFE:** Periodic TxFIFO empty
Asserted when the periodic transmit FIFO is either half or completely empty and there is space for at least one entry to be written in the periodic request queue. The half or completely empty status is determined by the periodic TxFIFO empty level bit in the Core AHB configuration register (PTXFELVL bit in OTG_HS_GAHBCFG).
*Note: Only accessible in host mode.*

1418/1749
RM0090 Rev 18

--- Page 54 ---

RM0090
USB on-the-go high-speed (OTG_HS)

**Bit 25 HCINT:** Host channels interrupt
The core sets this bit to indicate that an interrupt is pending on one of the channels of the core (in host mode). The application must read the host all channels interrupt (OTG_HS_HAINT) register to determine the exact number of the channel on which the interrupt occurred, and then read the corresponding host channel-x interrupt (OTG_HS_HCINTx) register to determine the exact cause of the interrupt. The application must clear the appropriate status bit in the OTG_HS_HCINTx register to clear this bit.
*Note: Only accessible in host mode.*

**Bit 24 HPRTINT:** Host port interrupt
The core sets this bit to indicate a change in port status of one of the OTG_HS controller ports in host mode. The application must read the host port control and status (OTG_HS_HPRT) register to determine the exact event that caused this interrupt. The application must clear the appropriate status bit in the host port control and status register to clear this bit.
*Note: Only accessible in host mode.*

**Bits 23** Reserved, must be kept at reset value.

**Bit 22 DATAFSUSP:** Data fetch suspended
This interrupt is valid only in DMA mode. This interrupt indicates that the core has stopped fetching data for IN endpoints due to the unavailability of TxFIFO space or request queue space. This interrupt is used by the application for an endpoint mismatch algorithm. For example, after detecting an endpoint mismatch, the application:
- Sets a global nonperiodic IN NAK handshake
- Disables IN endpoints
- Flushes the FIFO
- Determines the token sequence from the IN token sequence learning queue
- Re-enables the endpoints
- Clears the global nonperiodic IN NAK handshake If the global nonperiodic IN NAK is cleared, the core has not yet fetched data for the IN endpoint, and the IN token is received: the core generates an “IN token received when FIFO empty” interrupt. The OTG then sends a NAK response to the host. To avoid this scenario, the application can check the FetSusp interrupt in OTG_FS_GINTSTS, which ensures that the FIFO is full before clearing a global NAK handshake. Alternatively, the application can mask the “IN token received when FIFO empty” interrupt when clearing a global IN NAK handshake.

**Bit 21 IPXFR:** Incomplete periodic transfer
In host mode, the core sets this interrupt bit when there are incomplete periodic transactions still pending, which are scheduled for the current frame.
*Note: Only accessible in host mode.*

**INCOMISOOUT:** Incomplete isochronous OUT transfer
In peripheral mode, the core sets this interrupt to indicate that there is at least one isochronous OUT endpoint on which the transfer is not completed in the current frame. This interrupt is asserted along with the End of periodic frame interrupt (EOPF) bit in this register.
*Note: Only accessible in peripheral mode.*

**Bit 20 IISOIXFR:** Incomplete isochronous IN transfer
The core sets this interrupt to indicate that there is at least one isochronous IN endpoint on which the transfer is not completed in the current frame. This interrupt is asserted along with the End of periodic frame interrupt (EOPF) bit in this register.
*Note: Only accessible in peripheral mode.*

ST
RM0090 Rev 18
1419/1749

--- Page 55 ---

**USB on-the-go high-speed (OTG_HS)**

**RM0090**

**Bit 19 OEPINT: OUT endpoint interrupt**
The core sets this bit to indicate that an interrupt is pending on one of the OUT endpoints of the core (in peripheral mode). The application must read the device all endpoints interrupt (OTG_HS_DAINT) register to determine the exact number of the OUT endpoint on which the interrupt occurred, and then read the corresponding device OUT Endpoint-x Interrupt (OTG_HS_DOEPINTx) register to determine the exact cause of the interrupt. The application must clear the appropriate status bit in the corresponding OTG_HS_DOEPINTx register to clear this bit.
*Note: Only accessible in peripheral mode.*

**Bit 18 IEPINT: IN endpoint interrupt**
The core sets this bit to indicate that an interrupt is pending on one of the IN endpoints of the core (in peripheral mode). The application must read the device All Endpoints Interrupt (OTG_HS_DAINT) register to determine the exact number of the IN endpoint on which the interrupt occurred, and then read the corresponding device IN Endpoint-x interrupt (OTG_HS_DIEPINTx) register to determine the exact cause of the interrupt. The application must clear the appropriate status bit in the corresponding OTG_HS_DIEPINTx register to clear this bit.
*Note: Only accessible in peripheral mode.*

**Bits 17:16** Reserved, must be kept at reset value.

**Bit 15 EOPF: End of periodic frame interrupt**
Indicates that the period specified in the periodic frame interval field of the device configuration register (PFIVL bit in OTG_HS_DCFG) has been reached in the current frame.
*Note: Only accessible in peripheral mode.*

**Bit 14 ISOODRP: Isochronous OUT packet dropped interrupt**
The core sets this bit when it fails to write an isochronous OUT packet into the RxFIFO because the RxFIFO does not have enough space to accommodate a maximum size packet for the isochronous OUT endpoint.
*Note: Only accessible in peripheral mode.*

**Bit 13 ENUMDNE: Enumeration done**
The core sets this bit to indicate that speed enumeration is complete. The application must read the device Status (OTG_HS_DSTS) register to obtain the enumerated speed.
*Note: Only accessible in peripheral mode.*

**Bit 12 USBRST: USB reset**
The core sets this bit to indicate that a reset is detected on the USB.
*Note: Only accessible in peripheral mode.*

**Bit 11 USBSUSP: USB suspend**
The core sets this bit to indicate that a suspend was detected on the USB. The core enters the Suspended state when there is no activity on the data lines for a period of 3 ms.
*Note: Only accessible in peripheral mode.*

**Bit 10 ESUSP: Early suspend**
The core sets this bit to indicate that an Idle state has been detected on the USB for 3 ms.
*Note: Only accessible in peripheral mode.*

**Bits 9:8** Reserved, must be kept at reset value.

1420/1749
RM0090 Rev 18

--- Page 56 ---

RM0090
USB on-the-go high-speed (OTG_HS)

**Bit 7 GONAKEFF**: Global OUT NAK effective
Indicates that the Set global OUT NAK bit in the Device control register (SGONAK bit in OTG_HS_DCTL), set by the application, has taken effect in the core. This bit can be cleared by writing the Clear global OUT NAK bit in the Device control register (CGONAK bit in OTG_HS_DCTL).
*Note: Only accessible in peripheral mode.*

**Bit 6 GINAKEFF**: Global IN nonperiodic NAK effective
Indicates that the Set global nonperiodic IN NAK bit in the Device control register (SGINAK bit in OTG_HS_DCTL), set by the application, has taken effect in the core. That is, the core has sampled the Global IN NAK bit set by the application. This bit can be cleared by clearing the Clear global nonperiodic IN NAK bit in the Device control register (CGINAK bit in OTG_HS_DCTL).
This interrupt does not necessarily mean that a NAK handshake is sent out on the USB. The STALL bit takes precedence over the NAK bit.
*Note: Only accessible in peripheral mode.*

**Bit 5 NPTXFE**: Nonperiodic TxFIFO empty
This interrupt is asserted when the nonperiodic TxFIFO is either half or completely empty, and there is space in at least one entry to be written to the nonperiodic transmit request queue. The half or completely empty status is determined by the nonperiodic TxFIFO empty level bit in the OTG_HS_GAHBCFG register (TXFELVL bit in OTG_HS_GAHBCFG).
*Note: Only accessible in host mode.*

**Bit 4 RXFLVL**: RxFIFO nonempty
Indicates that there is at least one packet pending to be read from the RxFIFO.
*Note: Accessible in both host and peripheral modes.*

**Bit 3 SOF**: Start of frame
In host mode, the core sets this bit to indicate that an SOF (FS), or Keep-Alive (LS) is transmitted on the USB. The application must write a 1 to this bit to clear the interrupt.
In peripheral mode, in the core sets this bit to indicate that an SOF token has been received on the USB. The application can read the Device Status register to get the current frame number. This interrupt is seen only when the core is operating in FS.
*Note: Accessible in both host and peripheral modes.*

**Bit 2 OTGINT**: OTG interrupt
The core sets this bit to indicate an OTG protocol event. The application must read the OTG Interrupt Status (OTG_HS_GOTGINT) register to determine the exact event that caused this interrupt. The application must clear the appropriate status bit in the OTG_HS_GOTGINT register to clear this bit.
*Note: Accessible in both host and peripheral modes.*

**Bit 1 MMIS**: Mode mismatch interrupt
The core sets this bit when the application is trying to access:
A host mode register, when the core is operating in peripheral mode
A peripheral mode register, when the core is operating in host mode
The register access is completed on the AHB with an OKAY response, but is ignored by the core internally and does not affect the operation of the core.
*Note: Accessible in both host and peripheral modes.*

**Bit 0 CMOD**: Current mode of operation
Indicates the current mode.
0: Peripheral mode
1: Host mode
*Note: Accessible in both host and peripheral modes.*

ST
RM0090 Rev 18
1421/1749

--- Page 57 ---

USB on-the-go high-speed (OTG_HS)

RM0090

**OTG_HS interrupt mask register (OTG_HS_GINTMSK)**

Address offset: 0x018
Reset value: 0x0000 0000

This register works with the Core interrupt register to interrupt the application. When an interrupt bit is masked, the interrupt associated with that bit is not generated. However, the Core Interrupt (OTG_HS_GINTSTS) register bit corresponding to that interrupt is still set.

| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| WUIM | SRQIM | DISCINT | CIDSCHGM | Reserved | PTXFEM | HCIM | PRTIM | Reserved | FSUSPM | IPXFRM/ISOOCXFRM | IISOIXFRM | OEPINT | IEPINT | Reserved | EOPFM | ISOODRPM | ENUMDNEIM | USBRST | USBSUSPM | ESUSPM | Reserved | GONAKEFFM | GINAKEFFM | NPTXFEM | RXFLVLM | SOFM | OTGINT | MMISM | Reserved |  |  |
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


**Bit 31 WUIM**: Resume/remote wakeup detected interrupt mask
0: Masked interrupt
1: Unmasked interrupt
*Note: Accessible in both host and peripheral modes.*

**Bit 30 SRQIM**: Session request/new session detected interrupt mask
0: Masked interrupt
1: Unmasked interrupt
*Note: Accessible in both host and peripheral modes.*

**Bit 29 DISCINT**: Disconnect detected interrupt mask
0: Masked interrupt
1: Unmasked interrupt
*Note: Accessible in both host and peripheral modes.*

**Bit 28 CIDSCHGM**: Connector ID status change mask
0: Masked interrupt
1: Unmasked interrupt
*Note: Accessible in both host and peripheral modes.*

Bit 27 Reserved, must be kept at reset value.

**Bit 26 PTXFEM**: Periodic TxFIFO empty mask
0: Masked interrupt
1: Unmasked interrupt
*Note: Only accessible in host mode.*

**Bit 25 HCIM**: Host channels interrupt mask
0: Masked interrupt
1: Unmasked interrupt
*Note: Only accessible in host mode.*

**Bit 24 PRTIM**: Host port interrupt mask
0: Masked interrupt
1: Unmasked interrupt
*Note: Only accessible in host mode.*

Bit 23 Reserved, must be kept at reset value.

1422/1749
RM0090 Rev 18

--- Page 58 ---

RM0090
USB on-the-go high-speed (OTG_HS)

Bit 22 **FSUSPM**: Data fetch suspended mask
0: Masked interrupt
1: Unmasked interrupt
*Note: Only accessible in peripheral mode.*

Bit 21 **IPXFRM**: Incomplete periodic transfer mask
0: Masked interrupt
1: Unmasked interrupt
*Note: Only accessible in host mode.*

**IISOXFRM**: Incomplete isochronous OUT transfer mask
0: Masked interrupt
1: Unmasked interrupt
*Note: Only accessible in peripheral mode.*

Bit 20 **IISOIXFRM**: Incomplete isochronous IN transfer mask
0: Masked interrupt
1: Unmasked interrupt
*Note: Only accessible in peripheral mode.*

Bit 19 **OEPINT**: OUT endpoints interrupt mask
0: Masked interrupt
1: Unmasked interrupt
*Note: Only accessible in peripheral mode.*

Bit 18 **IEPINT**: IN endpoints interrupt mask
0: Masked interrupt
1: Unmasked interrupt
*Note: Only accessible in peripheral mode.*

Bits 17:16 Reserved, must be kept at reset value.

Bit 15 **EOPFM**: End of periodic frame interrupt mask
0: Masked interrupt
1: Unmasked interrupt
*Note: Only accessible in peripheral mode.*

Bit 14 **ISOODRPM**: Isochronous OUT packet dropped interrupt mask
0: Masked interrupt
1: Unmasked interrupt
*Note: Only accessible in peripheral mode.*

Bit 13 **ENUMDNEM**: Enumeration done mask
0: Masked interrupt
1: Unmasked interrupt
*Note: Only accessible in peripheral mode.*

Bit 12 **USBRST**: USB reset mask
0: Masked interrupt
1: Unmasked interrupt
*Note: Only accessible in peripheral mode.*

Bit 11 **USBSUSPM**: USB suspend mask
0: Masked interrupt
1: Unmasked interrupt
*Note: Only accessible in peripheral mode.*

RM0090 Rev 18
1423/1749

--- Page 59 ---

**USB on-the-go high-speed (OTG_HS)**

RM0090

Bit 10 **ESUSPM**: Early suspend mask
0: Masked interrupt
1: Unmasked interrupt
*Note: Only accessible in peripheral mode.*

Bits 9:8 Reserved, must be kept at reset value.

Bit 7 **GONAKEFFM**: Global OUT NAK effective mask
0: Masked interrupt
1: Unmasked interrupt
*Note: Only accessible in peripheral mode.*

Bit 6 **GINAKEFFM**: Global nonperiodic IN NAK effective mask
0: Masked interrupt
1: Unmasked interrupt
*Note: Only accessible in peripheral mode.*

Bit 5 **NPTXFEM**: Nonperiodic TxFIFO empty mask
0: Masked interrupt
1: Unmasked interrupt
*Note: Accessible in both peripheral and host modes.*

Bit 4 **RXFLVLM**: Receive FIFO nonempty mask
0: Masked interrupt
1: Unmasked interrupt
*Note: Accessible in both peripheral and host modes.*

Bit 3 **SOFM**: Start of frame mask
0: Masked interrupt
1: Unmasked interrupt
*Note: Accessible in both peripheral and host modes.*

Bit 2 **OTGINT**: OTG interrupt mask
0: Masked interrupt
1: Unmasked interrupt
*Note: Accessible in both peripheral and host modes.*

Bit 1 **MMISM**: Mode mismatch interrupt mask
0: Masked interrupt
1: Unmasked interrupt
*Note: Accessible in both peripheral and host modes.*

Bit 0 Reserved, must be kept at reset value.

1424/1749
RM0090 Rev 18

--- Page 60 ---

RM0090
USB on-the-go high-speed (OTG_HS)

**OTG_HS Receive status debug read/OTG status read and pop registers (OTG_HS_GRXSTSR/OTG_HS_GRXSTSP)**

Address offset for Read: 0x01C
Address offset for Pop: 0x020
Reset value: 0x0000 0000

A read to the Receive status debug read register returns the contents of the top of the Receive FIFO. A read to the Receive status read and pop register additionally pops the top data entry out of the RxFIFO.

The receive status contents must be interpreted differently in host and peripheral modes. The core ignores the receive status pop/read when the receive FIFO is empty and returns a value of 0x0000 0000. The application must only pop the Receive Status FIFO when the Receive FIFO nonempty bit of the Core interrupt register (RXFLVL bit in OTG_HS_GINTSTS) is asserted.

**Host mode:**

| 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0 |  |  |  |  |
|---|---|---|---|---|
| Reserved | PKTSTS | DPID | BCNT | CHNUM |
| r |  |  |  |  |


Bits 31:21 Reserved, must be kept at reset value.

Bits 20:17 **PKTSTS**: Packet status
Indicates the status of the received packet
0010: IN data packet received
0011: IN transfer completed (triggers an interrupt)
0101: Data toggle error (triggers an interrupt)
0111: Channel halted (triggers an interrupt)
Others: Reserved

Bits 16:15 **DPID**: Data PID
Indicates the Data PID of the received packet
00: DATA0
10: DATA1
01: DATA2
11: MDATA

Bits 14:4 **BCNT**: Byte count
Indicates the byte count of the received IN data packet.

Bits 3:0 **CHNUM**: Channel number
Indicates the channel number to which the current received packet belongs.

STI
RM0090 Rev 18
1425/1749

--- Page 61 ---

USB on-the-go high-speed (OTG_HS)
RM0090

**Peripheral mode:**

| Reserved | FRMNUM | PKTSTS | DPID | BCNT | EPTNUM |
|---|---|---|---|---|---|
| r |  |  |  |  |  |


Bits 31:25 Reserved, must be kept at reset value.
Bits 24:21 **FRMNUM**: Frame number
This is the least significant 4 bits of the frame number in which the packet is received on the USB. This field is supported only when isochronous OUT endpoints are supported.
Bits 20:17 **PKTSTS**: Packet status
Indicates the status of the received packet
0001: Global OUT NAK (triggers an interrupt)
0010: OUT data packet received
0011: OUT transfer completed (triggers an interrupt)
0100: SETUP transaction completed (triggers an interrupt)
0110: SETUP data packet received
Others: Reserved
Bits 16:15 **DPID**: Data PID
Indicates the Data PID of the received OUT data packet
00: DATA0
10: DATA1
01: DATA2
11: MDATA
Bits 14:4 **BCNT**: Byte count
Indicates the byte count of the received data packet.
Bits 3:0 **EPTNUM**: Endpoint number
Indicates the endpoint number to which the current received packet belongs.

**OTG_HS Receive FIFO size register (OTG_HS_GRXFSIZ)**
Address offset: 0x024
Reset value: 0x0000 0400
The application can program the RAM size that must be allocated to the RxFIFO.

| Reserved | RXFD |
|---|---|
| rw |  |


Bits 31:16 Reserved, must be kept at reset value.
Bits 15:0 **RXFD**: RxFIFO depth
This value is in terms of 32-bit words.
Minimum value is 16
Maximum value is 1024
The power-on reset value of this register is specified as the largest Rx data FIFO depth.

1426/1749
RM0090 Rev 18
ST

--- Page 62 ---

RM0090
USB on-the-go high-speed (OTG_HS)

**OTG_HS nonperiodic transmit FIFO size/Endpoint 0 transmit FIFO size register (OTG_HS_GNPTXFSIZ/OTG_HS_TX0FSIZ)**

Address offset: 0x028
Reset value: 0x0000 0200

**Host mode:**

| 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0 |  |
|---|---|
| NPTXFD | NPTXFSA |
| rw |  |


Bits 31:16 **NPTXFD**: Nonperiodic TxFIFO depth
This value is in terms of 32-bit words.
Minimum value is 16
Maximum value is 1024

Bits 15:0 **NPTXFSA**: Nonperiodic transmit RAM start address
This field contains the memory start address for nonperiodic transmit FIFO RAM.

**Peripheral mode:**

| 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0 |  |
|---|---|
| TX0FD | TX0FSA |
| r/w |  |


Bits 31:16 **TX0FD**: Endpoint 0 TxFIFO depth
This value is in terms of 32-bit words.
Minimum value is 16
Maximum value is 256

Bits 15:0 **TX0FSA**: Endpoint 0 transmit RAM start address
This field contains the memory start address for Endpoint 0 transmit FIFO RAM.

**OTG_HS nonperiodic transmit FIFO/queue status register (OTG_HS_GNPTXSTS)**

Address offset: 0x02C
Reset value: 0x0008 0400

*Note:* In peripheral mode, this register is not valid.
This read-only register contains the free space information for the nonperiodic TxFIFO and the nonperiodic transmit request queue.

| 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0 |  |  |  |
|---|---|---|---|
| Reserved | NPTXQTOP | NPTXSAV | NPTXFSAV |
| r |  |  |  |


ST
RM0090 Rev 18
1427/1749

--- Page 63 ---

**USB on-the-go high-speed (OTG_HS)**

**RM0090**

Bit 31 Reserved, must be kept at reset value.

Bits 30:24 **NPTXQTOP**: Top of the nonperiodic transmit request queue
Entry in the nonperiodic Tx request queue that is currently being processed by the MAC.
Bits [30:27]: Channel/endpoint number
Bits [26:25]:
- 00: IN/OUT token
- 01: Zero-length transmit packet (device IN/host OUT)
- 10: PING/CSPLIT token
- 11: Channel halt command
Bit [24]: Terminate (last entry for selected channel/endpoint)

Bits 23:16 **NPTXQSAV**: Nonperiodic transmit request queue space available
Indicates the amount of free space available in the nonperiodic transmit request queue.
This queue holds both IN and OUT requests in host mode. Peripheral mode has only IN requests.
00: Nonperiodic transmit request queue is full
01: dx1 location available
10: dx2 locations available
bxn: dxn locations available (0 ≤ n ≤ dx8)
Others: Reserved

Bits 15:0 **NPTXFSAV**: Nonperiodic TxFIFO space available
Indicates the amount of free space available in the nonperiodic TxFIFO.
Values are in terms of 32-bit words.
00: Nonperiodic TxFIFO is full
01: dx1 word available
10: dx2 words available
0xn: dxn words available (where 0 ≤ n ≤ dx1024)
Others: Reserved

**OTG_HS general core configuration register (OTG_HS_GCCFG)**

Address offset: 0x038
Reset value: 0x0000 XXXX

| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved | NOVBUSSENS | SOFOUTEN | VBUSASEN | VBUSBSSEN | I2CPADEN | PWRDWN | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  | rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


1428/1749
RM0090 Rev 18

--- Page 64 ---

RM0090
USB on-the-go high-speed (OTG_HS)

Bits 31:22 Reserved, must be kept at reset value.

Bit 21 **NOVBUSSENS**: VBUS sensing disable option
When this bit is set, VBUS is considered internally to be always at VBUS valid level (5 V). This option removes the need for a dedicated VBUS pad, and leave this pad free to be used for other purposes such as a shared functionality. VBUS connection can be remapped on another general purpose input pad and monitored by software.
This option is only suitable for host-only or device-only applications.
0: VBUS sensing available by hardware
1: VBUS sensing not available by hardware.

Bit 20 **SOFOUTEN**: SOF output enable
0: SOF pulse not available on PAD
1: SOF pulse available on PAD

Bit 19 **VBUSBSSEN**: Enable the VBUS sensing "B" device
0: VBUS sensing "B" disabled
1: VBUS sensing "B" enabled

Bit 18 **VBUSASEN**: Enable the VBUS sensing "A" device
0: VBUS sensing "A" disabled
1: VBUS sensing "A" enabled

Bit 17 **I2CPADEN**: Enable I2C bus connection for the external I2C PHY interface.
0: I2C bus disabled
1: I2C bus enabled

Bit 16 **PWRDWN**: Power down
Used to activate the transceiver in transmission/reception
0: Power down active
1: Power down deactivated ("Transceiver active")

Bits 15:0 Reserved, must be kept at reset value.

**OTG_HS core ID register (OTG_HS_CID)**

Address offset: 0x03C
Reset value: 0x0000 1100
This is a register containing the Product ID as reset value.

| PRODUCT_ID |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bits 31:0 **PRODUCT_ID**: Product ID field
Application-programmable ID field.

**OTG_HS Host periodic transmit FIFO size register (OTG_HS_HPTXFSIZ)**

Address offset: 0x100
Reset value: 0x0200 0800

STI
RM0090 Rev 18
1429/1749

--- Page 65 ---

USB on-the-go high-speed (OTG_HS)
RM0090

| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| PTXFD | PTXSA |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| r |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bits 31:16 **PTXFD**: Host periodic TxFIFO depth
This value is in terms of 32-bit words.
Minimum value is 16
Maximum value is 512

Bits 15:0 **PTXSA**: Host periodic TxFIFO start address
The power-on reset value of this register is the sum of the largest Rx data FIFO depth and largest nonperiodic Tx data FIFO depth.

**OTG_HS device IN endpoint transmit FIFO size register (OTG_HS_DIEPTXFx) (x = 1..5, where x is the FIFO_number)**

Address offset: 0x104 + 0x04 * (x - 1)
Reset value: 0x02000400

| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| INEPTXFD | INEPTXSA |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| r |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bits 31:16 **INEPTXFD**: IN endpoint TxFIFO depth
This value is in terms of 32-bit words.
Minimum value is 16
Maximum value is 512
The power-on reset value of this register is specified as the largest IN endpoint FIFO number depth.

Bits 15:0 **INEPTXSA**: IN endpoint FIFOx transmit RAM start address
This field contains the memory start address for IN endpoint transmit FIFOx. The address must be aligned with a 32-bit memory location.


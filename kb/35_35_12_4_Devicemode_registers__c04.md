> [RM0090 ch.35 §35.12.4] Device-mode registers


## OTG_HS device control OUT endpoint 0 control register (OTG_HS_DOEPCTL0)

Address offset: 0xB00
Reset value: 0x0000 8000

This section describes the device control OUT endpoint 0 control register. Nonzero control endpoints use registers for endpoints 1–15.

| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| EPENA | EPDIS | Reserved | SNAK | CNAK | Reserved | STALL | SNPM | EPTYP | NAKSTS | Reserved | USBAEP | Reserved | MPSIZ |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| w | rs | rw | r |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


**Bit 31 EPENA: Endpoint enable**
The application sets this bit to start transmitting data on endpoint 0.
The core clears this bit before setting any of the following interrupts on this endpoint:
– SETUP phase done
– Endpoint disabled
– Transfer completed

**Bit 30 EPDIS: Endpoint disable**
The application cannot disable control OUT endpoint 0.

**Bits 29:28 Reserved, must be kept at reset value.**

**Bit 27 SNAK: Set NAK**
A write to this bit sets the NAK bit for the endpoint.
Using this bit, the application can control the transmission of NAK handshakes on an endpoint. The core can also set this bit on a Transfer completed interrupt, or after a SETUP is received on the endpoint.

**Bit 26 CNAK: Clear NAK**
A write to this bit clears the NAK bit for the endpoint.

**Bits 25:22 Reserved, must be kept at reset value.**

**Bit 21 STALL: STALL handshake**
The application can only set this bit, and the core clears it, when a SETUP token is received for this endpoint. If a NAK bit or Global OUT NAK is set along with this bit, the STALL bit takes priority. Irrespective of this bit’s setting, the core always responds to SETUP data packets with an ACK handshake.

**Bit 20 SNPM: Snoop mode**
This bit configures the endpoint to Snoop mode. In Snoop mode, the core does not check the correctness of OUT packets before transferring to application memory.

**Bits 19:18 EPTYP: Endpoint type**
Hardcoded to 2'b00 for control.

1460/1749
RM0090 Rev 18

--- Page 96 ---

RM0090
USB on-the-go high-speed (OTG_HS)

Bit 17 **NAKSTS**: NAK status
Indicates the following:
0: The core is transmitting nonNAK handshakes based on the FIFO status.
1: The core is transmitting NAK handshakes on this endpoint.
When either the application or the core sets this bit, the core stops receiving data, even if
there is space in the RxFIFO to accommodate the incoming packet. Irrespective of this bit's
setting, the core always responds to SETUP data packets with an ACK handshake.

Bit 16 Reserved, must be kept at reset value.

Bit 15 **USBAEP**: USB active endpoint
This bit is always set to 1, indicating that a control endpoint 0 is always active in all
configurations and interfaces.

Bits 14:2 Reserved, must be kept at reset value.

Bits 1:0 **MPSIZ**: Maximum packet size
The maximum packet size for control OUT endpoint 0 is the same as what is programmed in
control IN endpoint 0.
00: 64 bytes
01: 32 bytes
10: 16 bytes
11: 8 bytes

**OTG_HS device endpoint-x control register (OTG_HS_DOEPCTLx) (x = 1..5, where x = Endpoint_number)**

Address offset for OUT endpoints: 0xB00 + 0x20 * x
Reset value: 0x0000 0000
The application uses this register to control the behavior of each logical endpoint other than
endpoint 0.

| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| EPENA | EPDIS | SODDFRM | SDOPID/SEVNFRM | SNAK | CNAK | Reserved | Stall | SNPM | EPTYP | NAKSTS | EONUM/DPID | USBAEP | Reserved | MPSIZ |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| rs | w |  |  |  |  |  |  | rw | r | rw |  |  |  | rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


STI
RM0090 Rev 18
1461/1749

--- Page 97 ---

**USB on-the-go high-speed (OTG_HS)**

**RM0090**

**Bit 31 EPENA: Endpoint enable**
Applies to IN and OUT endpoints.
The application sets this bit to start transmitting data on an endpoint.
The core clears this bit before setting any of the following interrupts on this endpoint:
– SETUP phase done
– Endpoint disabled
– Transfer completed

**Bit 30 EPDIS: Endpoint disable**
The application sets this bit to stop transmitting/receiving data on an endpoint, even before the transfer for that endpoint is complete. The application must wait for the Endpoint disabled interrupt before treating the endpoint as disabled. The core clears this bit before setting the Endpoint disabled interrupt. The application must set this bit only if Endpoint enable is already set for this endpoint.

**Bit 29 SODDFRM: Set odd frame**
Applies to isochronous OUT endpoints only.
Writing to this field sets the Even/Odd frame (EONUM) field to odd frame.

**Bit 28 SD0PID: Set DATA0 PID**
Applies to interrupt/bulk OUT endpoints only.
Writing to this field sets the endpoint data PID (DPID) field in this register to DATA0.

**SEVNFRM: Set even frame**
Applies to isochronous OUT endpoints only.
Writing to this field sets the Even/Odd frame (EONUM) field to even frame.

**Bit 27 SNAK: Set NAK**
A write to this bit sets the NAK bit for the endpoint.
Using this bit, the application can control the transmission of NAK handshakes on an endpoint. The core can also set this bit for OUT endpoints on a Transfer Completed interrupt, or after a SETUP is received on the endpoint.

**Bit 26 CNAK: Clear NAK**
A write to this bit clears the NAK bit for the endpoint.

**Bits 25:22** Reserved, must be kept at reset value.

**Bit 21 STALL: STALL handshake**
Applies to noncontrol, nonisochronous OUT endpoints only (access type is rw).
The application sets this bit to stall all tokens from the USB host to this endpoint. If a NAK bit, Global IN NAK, or Global OUT NAK is set along with this bit, the STALL bit takes priority. Only the application can clear this bit, never the core.

**Bit 20 SNPM: Snoop mode**
This bit configures the endpoint to Snoop mode. In Snoop mode, the core does not check the correctness of OUT packets before transferring them to application memory.

**Bits 19:18 EPTYP: Endpoint type**
This is the transfer type supported by this logical endpoint.
00: Control
01: Isochronous
10: Bulk
11: Interrupt

1462/1749 RM0090 Rev 18

--- Page 98 ---

RM0090
USB on-the-go high-speed (OTG_HS)

**Bit 17 NAKSTS: NAK status**
Indicates the following:
0: The core is transmitting nonNAK handshakes based on the FIFO status.
1: The core is transmitting NAK handshakes on this endpoint.

When either the application or the core sets this bit:
The core stops receiving any data on an OUT endpoint, even if there is space in the RxFIFO to accommodate the incoming packet.

Irrespective of this bit's setting, the core always responds to SETUP data packets with an ACK handshake.

**Bit 16 EONUM: Even/odd frame**
Applies to isochronous IN and OUT endpoints only.
Indicates the frame number in which the core transmits/receives isochronous data for this endpoint. The application must program the even/odd frame number in which it intends to transmit/receive isochronous data for this endpoint using the SEVNFRM and SODDFRM fields in this register.
0: Even frame
1: Odd frame

**DPID: Endpoint data PID**
Applies to interrupt/bulk OUT endpoints only.
Contains the PID of the packet to be received or transmitted on this endpoint. The application must program the PID of the first packet to be received or transmitted on this endpoint, after the endpoint is activated. The application uses the SD0PID register field to program either DATA0 or DATA1 PID.
0: DATA0
1: DATA1

**Bit 15 USBAEP: USB active endpoint**
Indicates whether this endpoint is active in the current configuration and interface. The core clears this bit for all endpoints (other than EP 0) after detecting a USB reset. After receiving the SetConfiguration and SetInterface commands, the application must program endpoint registers accordingly and set this bit.

**Bits 14:11** Reserved, must be kept at reset value.

**Bits 10:0 MPSIZ: Maximum packet size**
The application must program this field with the maximum packet size for the current logical endpoint. This value is in bytes.

ST
RM0090 Rev 18
1463/1749

--- Page 99 ---

**USB on-the-go high-speed (OTG_HS)**
RM0090

**OTG_HS device endpoint-x interrupt register (OTG_HS_DIEPINTx) (x = 0..5, where x = Endpoint_number)**

Address offset: 0x908 + 0x20 * x
Reset value: 0x0000 0080

This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure 414. The application must read this register when the IN endpoints interrupt bit of the Core interrupt register (IEPINT in OTG_HS_GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (OTG_HS_DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HS_DAINT and OTG_HS_GINTSTS registers.

| 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved | NAK | Reserved | PKTDRPSTS | Reserved | TXFIOUDRN | TXFE | INEPNE | INEPNM | ITTXFE | TOC | AHBERR | EPDISD | XFRC |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | rcw1 |  | rcw1 |  | rcw1 | r | rcw1 |


Bits 31:14 Reserved, must be kept at reset value.

**Bit 13 NAK: NAK interrupt**
The core generates this interrupt when a NAK is transmitted or received by the device. In case of isochronous IN endpoints the interrupt gets generated when a zero length packet is transmitted due to unavailability of data in the Tx FIFO.

Bit 12 Reserved, must be kept at reset value.

**Bit 11 PKTDRPSTS: Packet dropped status**
This bit indicates to the application that an ISOC OUT packet has been dropped. This bit does not have an associated mask bit and does not generate an interrupt.

Bits 10:9 Reserved, must be kept at reset value.

**Bit 8 TXFIOUDRN: Transmit Fifo Underrun (TxfifoUndrn)** The core generates this interrupt when it detects a transmit FIFO underrun condition for this endpoint.
**Dependency:** This interrupt is valid only when Thresholding is enabled

**Bit 7 TXFE: Transmit FIFO empty**
This interrupt is asserted when the TxFIFO for this endpoint is either half or completely empty. The half or completely empty status is determined by the TxFIFO empty level bit in the Core AHB configuration register (TXFELVL bit in OTG_HS_GAHBCFG).

**Bit 6 INEPNE: IN endpoint NAK effective**
This bit can be cleared when the application clears the IN endpoint NAK by writing to the CNAK bit in OTG_HS_DIEPCTLx.
This interrupt indicates that the core has sampled the NAK bit set (either by the application or by the core). The interrupt indicates that the IN endpoint NAK bit set by the application has taken effect in the core.
This interrupt does not guarantee that a NAK handshake is sent on the USB. A STALL bit takes priority over a NAK bit.

1464/1749
RM0090 Rev 18

--- Page 100 ---

RM0090
USB on-the-go high-speed (OTG_HS)

**Bit 5 INEPNM:** IN token received with EP mismatch
Indicates that the data in the top of the non-periodic TxFIFO belongs to an endpoint other than the one for which the IN token was received. This interrupt is asserted on the endpoint for which the IN token was received.

**Bit 4 ITTXFE:** IN token received when TxFIFO is empty
Applies to nonperiodic IN endpoints only.
Indicates that an IN token was received when the associated TxFIFO (periodic/nonperiodic) was empty. This interrupt is asserted on the endpoint for which the IN token was received.

**Bit 3 TOC:** Timeout condition
Applies only to Control IN endpoints.
Indicates that the core has detected a timeout condition on the USB for the last IN token on this endpoint.

**Bit 2 AHBERR:** AHB error
This is generated only in internal DMA mode when there is an AHB error during an AHB read/write. The application can read the corresponding endpoint DMA address register to get the error address.

**Bit 1 EPDISD:** Endpoint disabled interrupt
This bit indicates that the endpoint is disabled per the application's request.

**Bit 0 XFRC:** Transfer completed interrupt
This field indicates that the programmed transfer is complete on the AHB as well as on the USB, for this endpoint.

ST
RM0090 Rev 18
1465/1749

--- Page 101 ---

USB on-the-go high-speed (OTG_HS)
RM0090

**OTG_HS device endpoint-x interrupt register (OTG_HS_DOEPINTx) (x = 0..5, where x = Endpoint_number)**

Address offset: 0xB08 + 0x20 * x
Reset value: 0x0000 0080

This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure 414. The application must read this register when the OUT Endpoints Interrupt bit of the Core interrupt register (OEPINT bit in OTG_HS_GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (OTG_HS_DAINT) register to get the exact endpoint number for the device Endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HS_DAINT and OTG_HS_GINTSTS registers.

| 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0 |
|---|
| Reserved |
| NYET NAK BERR Reserved OUTPKTERR Reserved B2BSTUP Reserved OTEPDIS STUP AHBERR EPDISD XFRC |
| rc_w1 rc_w1 rc_w1 rc_w1 rc_w1 rc_w1 rc_w1 rc_w1 rc_w1 rc_w1 rc_w1 rc_w1 |


Bits 31:15 Reserved, must be kept at reset value.

**Bit 14 NYET:** NYET interrupt
The core generates this interrupt when a NYET response is transmitted for a nonisochronous OUT endpoint.

**Bit 13 NAK:** NAK input
The core generates this interrupt when a NAK is transmitted or received by the device. In case of isochronous IN endpoints the interrupt gets generated when a zero length packet is transmitted due to unavailability of data in the Tx FIFO.

**Bit 12 BERR:** Babble error interrupt
The core generates this interrupt when babble is received for the endpoint.

Bits 11:9 Reserved, must be kept at reset value.

**Bit 8 OUTPKTERR:** OUT packet error
This interrupt is asserted when the core detects an overflow or a CRC error for an OUT packet. This interrupt is valid only when thresholding is enabled.

Bit 7 Reserved, must be kept at reset value.

**Bit 6 B2BSTUP:** Back-to-back SETUP packets received
Applies to Control OUT endpoint only.
This bit indicates that the core has received more than three back-to-back SETUP packets for this particular endpoint.

Bit 5 Reserved, must be kept at reset value.

**Bit 4 OTEPDIS:** OUT token received when endpoint disabled
Applies only to control OUT endpoint.
Indicates that an OUT token was received when the endpoint was not yet enabled. This interrupt is asserted on the endpoint for which the OUT token was received.

1466/1749
RM0090 Rev 18
ST

--- Page 102 ---

RM0090
USB on-the-go high-speed (OTG_HS)

Bit 3 **STUP**: SETUP phase done
Applies to control OUT endpoints only.
Indicates that the SETUP phase for the control endpoint is complete and no more back-to-back SETUP packets were received for the current control transfer. On this interrupt, the application can decode the received SETUP data packet.

Bit 2 **AHBERR**: AHB error
This is generated only in internal DMA mode when there is an AHB error during an AHB read/write. The application can read the corresponding endpoint DMA address register to get the error address.

Bit 1 **EPDISD**: Endpoint disabled interrupt
This bit indicates that the endpoint is disabled per the application's request.

Bit 0 **XFRC**: Transfer completed interrupt
This field indicates that the programmed transfer is complete on the AHB as well as on the USB, for this endpoint.

**OTG_HS device IN endpoint 0 transfer size register (OTG_HS_DIEPTSIZ0)**

Address offset: 0x910
Reset value: 0x0000 0000

The application must modify this register before enabling endpoint 0. Once endpoint 0 is enabled using the endpoint enable bit in the device control endpoint 0 control registers (EPENA in OTG_HS_DIEPCTL0), the core modifies this register. The application can only read this register once the core has cleared the Endpoint enable bit.

Nonzero endpoints use the registers for endpoints 1–15.

| Reserved | PKTCNT | Reserved | XFRSIZ |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bits 31:21 Reserved, must be kept at reset value.

Bits 20:19 **PKTCNT**: Packet count
Indicates the total number of USB packets that constitute the Transfer Size amount of data for endpoint 0.
This field is decremented every time a packet (maximum size or short packet) is read from the TxFIFO.

Bits 18:7 Reserved, must be kept at reset value.

Bits 6:0 **XFRSIZ**: Transfer size
Indicates the transfer size in bytes for endpoint 0. The core interrupts the application only after it has exhausted the transfer size amount of data. The transfer size can be set to the maximum packet size of the endpoint, to be interrupted at the end of each packet.
The core decrements this field every time a packet from the external memory is written to the TxFIFO.

ST
RM0090 Rev 18
1467/1749

--- Page 103 ---

USB on-the-go high-speed (OTG_HS)
RM0090


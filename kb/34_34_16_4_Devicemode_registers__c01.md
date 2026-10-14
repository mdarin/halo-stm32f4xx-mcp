> [RM0090 ch.34 §34.16.4] Device-mode registers


### 34.16.4 Device-mode registers

**OTG_FS device configuration register (OTG_FS_DCFG)**

Address offset: 0x800
Reset value: 0x0220 0000

This register configures the core in device mode after power-on or after certain control commands or enumeration. Do not make changes to this register after initial programming.

| 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 15 14 13 | 12 | 11 10 9 8 7 | 6 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|
| Reserved | PFI VL | DAD | Reserved | NZLSOHSK | DSPD |  |  |  |
| rw |  |  |  |  |  |  |  |  |


Bits 31:13 Reserved, must be kept at reset value.

**Bits 12:11 PFI VL: Periodic frame interval**
Indicates the time within a frame at which the application must be notified using the end of periodic frame interrupt. This can be used to determine if all the isochronous traffic for that frame is complete.
00: 80% of the frame interval
01: 85% of the frame interval
10: 90% of the frame interval
11: 95% of the frame interval

**Bits 10:4 DAD: Device address**
The application must program this field after every SetAddress control command.

Bit 3 Reserved, must be kept at reset value.

**Bit 2 NZLSOHSK: Non-zero-length status OUT handshake**
The application can use this field to select the handshake the core sends on receiving a nonzero-length data packet during the OUT transaction of a control transfer's Status stage.
1: Send a STALL handshake on a nonzero-length status OUT transaction and do not send the received OUT packet to the application.
0: Send the received OUT packet to the application (zero-length or nonzero-length) and send a handshake based on the NAK and STALL bits for the endpoint in the Device endpoint control register.

**Bits 1:0 DSPD: Device speed**
Indicates the speed at which the application requires the core to enumerate, or the maximum speed the application can support. However, the actual bus speed is determined only after the chirp sequence is completed, and is based on the speed of the USB host to which the core is connected.
00: Reserved
01: Reserved
10: Reserved
11: Full speed (USB 1.1 transceiver clock is 48 MHz)

1302/1749 RM0090 Rev 18 ST

--- Page 133 ---

RM0090
USB on-the-go full-speed (OTG_FS)

# OTG_FS device control register (OTG_FS_DCTL)

Address offset: 0x804
Reset value: 0x0000 0000

| 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved | POPGRDNE | CGONAK | SGONAK | CGINAK | SGINAK | TCTL | GONSTS | GINSTS | SDIS | RWUSG |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | r/w | w | r/w | r | r/w |


Bits 31:12 Reserved, must be kept at reset value.

**Bit 11 POPGRDNE:** Power-on programming done
The application uses this bit to indicate that register programming is completed after a wakeup from power down mode.

**Bit 10 CGONAK:** Clear global OUT NAK
Writing 1 to this field clears the Global OUT NAK.

**Bit 9 SGONAK:** Set global OUT NAK
Writing 1 to this field sets the Global OUT NAK.
The application uses this bit to send a NAK handshake on all OUT endpoints.
The application must set the this bit only after making sure that the Global OUT NAK effective bit in the Core interrupt register (GONAKEFF bit in OTG_FS_GINTSTS) is cleared.

**Bit 8 CGINAK:** Clear global IN NAK
Writing 1 to this field clears the Global IN NAK.

**Bit 7 SGINAK:** Set global IN NAK
Writing 1 to this field sets the Global non-periodic IN NAK. The application uses this bit to send a NAK handshake on all non-periodic IN endpoints.
The application must set this bit only after making sure that the Global IN NAK effective bit in the Core interrupt register (GINAKEFF bit in OTG_FS_GINTSTS) is cleared.

**Bits 6:4 TCTL:** Test control
000: Test mode disabled
001: Test_J mode
010: Test_K mode
011: Test_SE0_NAK mode
100: Test_Packet mode
101: Test_Force_Enable
Others: Reserved

**Bit 3 GONSTS:** Global OUT NAK status
0: A handshake is sent based on the FIFO Status and the NAK and STALL bit settings.
1: No data is written to the RxFIFO, irrespective of space availability. Sends a NAK handshake on all packets, except on SETUP transactions. All isochronous OUT packets are dropped.

STI
RM0090 Rev 18
1303/1749

--- Page 134 ---

**USB on-the-go full-speed (OTG_FS)**
**RM0090**

**Bit 2 GINSTS**: Global IN NAK status
0: A handshake is sent out based on the data availability in the transmit FIFO.
1: A NAK handshake is sent out on all non-periodic IN endpoints, irrespective of the data availability in the transmit FIFO.

**Bit 1 SDIS**: Soft disconnect
The application uses this bit to signal the USB OTG core to perform a soft disconnect. As long as this bit is set, the host does not see that the device is connected, and the device does not receive signals on the USB. The core stays in the disconnected state until the application clears this bit.
0: Normal operation. When this bit is cleared after a soft disconnect, the core generates a device connect event to the USB host. When the device is reconnected, the USB host restarts device enumeration.
1: The core generates a device disconnect event to the USB host.

**Bit 0 RWUSIG**: Remote wakeup signaling
When the application sets this bit, the core initiates remote signaling to wake up the USB host. The application must set this bit to instruct the core to exit the Suspend state. As specified in the USB 2.0 specification, the application must clear this bit 1 ms to 15 ms after setting it.

Table 204 contains the minimum duration (according to device state) for which the Soft disconnect (SDIS) bit must be set for the USB host to detect a device disconnect. To accommodate clock jitter, it is recommended that the application add some extra delay to the specified minimum duration.

**Table 204. Minimum duration for soft disconnect**

| Operating speed | Device state | Minimum duration |
|---|---|---|
| Full speed | Suspended | 1 ms + 2.5 µs |
| Full speed | Idle | 2.5 µs |
| Full speed | Not Idle or Suspended (Performing transactions) | 2.5 µs |


**OTG_FS device status register (OTG_FS_DSTS)**
Address offset: 0x808
Reset value: 0x0000 0010

This register indicates the status of the core with respect to USB-related events. It must be read on interrupts from the device all interrupts (OTG_FS_DAINt) register.

31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0
Reserved FNSOF Reserved EERR ENUMSPD SUSPSTS
r r r r r r r r r r r r r r r r r r r r

Bits 31:22 Reserved, must be kept at reset value.
Bits 21:8 **FNSOF**: Frame number of the received SOF
Bits 7:4 Reserved, must be kept at reset value.

1304/1749
RM0090 Rev 18

--- Page 135 ---

RM0090
USB on-the-go full-speed (OTG_FS)

**Bit 3 EERR**: Erratic error
The core sets this bit to report any erratic errors.
Due to erratic errors, the OTG_FS controller goes into Suspended state and an interrupt is generated to the application with Early suspend bit of the OTG_FS_GINTSTS register (ESUSP bit in OTG_FS_GINTSTS). If the early suspend is asserted due to an erratic error, the application can only perform a soft disconnect recover.

**Bits 2:1 ENUMSPD**: Enumerated speed
Indicates the speed at which the OTG_FS controller has come up after speed detection through a chirp sequence.
01: Reserved
10: Reserved
11: Full speed (PHY clock is running at 48 MHz)
Others: reserved

**Bit 0 SUSPSTS**: Suspend status
In device mode, this bit is set as long as a Suspend condition is detected on the USB. The core enters the Suspended state when there is no activity on the USB data lines for a period of 3 ms. The core comes out of the suspend:
– When there is an activity on the USB data lines
– When the application writes to the Remote wakeup signaling bit in the OTG_FS_DCTL register (RWUSIG bit in OTG_FS_DCTL).

**OTG_FS device IN endpoint common interrupt mask register (OTG_FS_DIEPMSK)**

Address offset: 0x810
Reset value: 0x0000 0000

This register works with each of the OTG_FS_DIEPINTx registers for all endpoints to generate an interrupt per IN endpoint. The IN endpoint interrupt for a specific status in the OTG_FS_DIEPINTx register can be masked by writing to the corresponding bit in this register. Status bits are masked by default.

| Reserved | NAKM | Reserved | INEPNEM | INEPNMM | ITTXFEMSK | TOM | Reserved | EPDM | XFRCM |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 15 14 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |


Bits 31:14 Reserved, must be kept at reset value.

**Bit 13 NAKM**: NAK interrupt mask
0: Masked interrupt
1: Unmasked interrupt

Bits 12:7 Reserved, must be kept at reset value.

**Bit 6 INEPNEM**: IN endpoint NAK effective mask
0: Masked interrupt
1: Unmasked interrupt

ST
RM0090 Rev 18
1305/1749

--- Page 136 ---

**USB on-the-go full-speed (OTG_FS)**

**RM0090**

Bit 5 **INEPNMM**: IN token received with EP mismatch mask
0: Masked interrupt
1: Unmasked interrupt

Bit 4 **ITTXFEMSK**: IN token received when TxFIFO empty mask
0: Masked interrupt
1: Unmasked interrupt

Bit 3 **TOM**: Timeout condition mask (Non-isochronous endpoints)
0: Masked interrupt
1: Unmasked interrupt

Bit 2 Reserved, must be kept at reset value.

Bit 1 **EPDM**: Endpoint disabled interrupt mask
0: Masked interrupt
1: Unmasked interrupt

Bit 0 **XFRCM**: Transfer completed interrupt mask
0: Masked interrupt
1: Unmasked interrupt

**OTG_FS device OUT endpoint common interrupt mask register (OTG_FS_DOEPMSK)**

Address offset: 0x814
Reset value: 0x0000 0000

This register works with each of the OTG_FS_DOEPINTx registers for all endpoints to generate an interrupt per OUT endpoint. The OUT endpoint interrupt for a specific status in the OTG_FS_DOEPINTx register can be masked by writing into the corresponding bit in this register. Status bits are masked by default.

| 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 15 14 | 13 | 12 | 11 10 9 | 8 | 7 6 | 5 | 4 | 3 | 2 1 | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved | NAK | BERRM | Reserved | OUTPKTERRM | Reserved | STSPHSRXM | OTEPDM | STUPM | Reserved | EPDM | XFRCM |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | rw |  |  |  | rw |  |  | rw |  |  | rw |


Bits 31:14 Reserved, must be kept at reset value.

Bit 13 **NAKMSK**: NAK interrupt mask
0: Masked interrupt
1: Unmasked interrupt

Bit 12 **BERRM**: Babble error interrupt mask
0: Masked interrupt
1: Unmasked interrupt

Bits 11:9 Reserved, must be kept at reset value.

Bit 8 **OUTPKTERRM**: Out packet error mask
0: Masked interrupt
1: Unmasked interrupt

1306/1749
RM0090 Rev 18

--- Page 137 ---

RM0090
USB on-the-go full-speed (OTG_FS)

Bits 7:6 Reserved, must be kept at reset value.

Bit 5 **STSPHSRXM**: Status phase received for control write mask
0: Masked interrupt
1: Unmasked interrupt

Bit 4 **OTEPDM**: OUT token received when endpoint disabled mask
Applies to control OUT endpoints only.
0: Masked interrupt
1: Unmasked interrupt

Bit 3 **STUPM**: SETUP phase done mask
Applies to control endpoints only.
0: Masked interrupt
1: Unmasked interrupt

Bit 2 Reserved, must be kept at reset value.

Bit 1 **EPDM**: Endpoint disabled interrupt mask
0: Masked interrupt
1: Unmasked interrupt

Bit 0 **XFRCM**: Transfer completed interrupt mask
0: Masked interrupt
1: Unmasked interrupt

**OTG_FS device all endpoints interrupt register (OTG_FS_DAI NT)**

Address offset: 0x818
Reset value: 0x0000 0000

When a significant event occurs on an endpoint, a OTG_FS_DAI NT register interrupts the application using the Device OUT endpoints interrupt bit or Device IN endpoints interrupt bit of the OTG_FS_GINTSTS register (OEPINT or IEPINT in OTG_FS_GINTSTS, respectively). There is one interrupt bit per endpoint, up to a maximum of 16 bits for OUT endpoints and 16 bits for IN endpoints. For a bidirectional endpoint, the corresponding IN and OUT interrupt bits are used. Bits in this register are set and cleared when the application sets and clears bits in the corresponding Device Endpoint-x interrupt register (OTG_FS_DIEPINTx/OTG_FS_DOEPINTx).

| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| OEPINT | IEPINT |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| r |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bits 31:16 **OEPINT**: OUT endpoint interrupt bits
One bit per OUT endpoint:
Bit 16 for OUT endpoint 0, bit 19 for OUT endpoint 3.

Bits 15:0 **IEPINT**: IN endpoint interrupt bits
One bit per IN endpoint:
Bit 0 for IN endpoint 0, bit 3 for endpoint 3.

STI
RM0090 Rev 18
1307/1749

--- Page 138 ---

USB on-the-go full-speed (OTG_FS)
RM0090

**OTG_FS all endpoints interrupt mask register (OTG_FS_DAINTMSK)**

Address offset: 0x81C
Reset value: 0x0000 0000

The OTG_FS_DAINTMSK register works with the Device endpoint interrupt register to interrupt the application when an event occurs on a device endpoint. However, the OTG_FS_DAINT register bit corresponding to that interrupt is still set.

| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| OEPM | IEPM |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bits 31:16 **OEPM**: OUT EP interrupt mask bits
One per OUT endpoint:
Bit 16 for OUT EP 0, bit 19 for OUT EP 3
0: Masked interrupt
1: Unmasked interrupt

Bits 15:0 **IEPM**: IN EP interrupt mask bits
One bit per IN endpoint:
Bit 0 for IN EP 0, bit 3 for IN EP 3
0: Masked interrupt
1: Unmasked interrupt

**OTG_FS device V<sub>BUS</sub> discharge time register (OTG_FS_DVBUSDIS)**

Address offset: 0x0828
Reset value: 0x0000 17D7

This register specifies the V<sub>BUS</sub> discharge time after V<sub>BUS</sub> pulsing during SRP.

| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved | VBUSDT |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bits 31:16 Reserved, must be kept at reset value.
Bits 15:0 **VBUSDT**: Device V<sub>BUS</sub> discharge time
Specifies the V<sub>BUS</sub> discharge time after V<sub>BUS</sub> pulsing during SRP. This value equals:
V<sub>BUS</sub> discharge time in PHY clocks / 1 024
Depending on your V<sub>BUS</sub> load, this value may need adjusting.

**OTG_FS device V<sub>BUS</sub> pulsing time register (OTG_FS_DVBUSPULSE)**

Address offset: 0x082C
Reset value: 0x0000 05B8

This register specifies the V<sub>BUS</sub> pulsing time during SRP.

1308/1749
RM0090 Rev 18
ST

--- Page 139 ---

RM0090
USB on-the-go full-speed (OTG_FS)

| Reserved | DVBUSP |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bits 31:12 Reserved, must be kept at reset value.
Bits 11:0 **DVBUSP**: Device V<sub>BUS</sub> pulsing time
Specifies the V<sub>BUS</sub> pulsing time during SRP. This value equals:
V<sub>BUS</sub> pulsing time in PHY clocks / 1 024

**OTG_FS device IN endpoint FIFO empty interrupt mask register:**
**(OTG_FS_DIEPEMPMSK)**

Address offset: 0x834
Reset value: 0x0000 0000

This register is used to control the IN endpoint FIFO empty interrupt generation
(TXFE_OTG_FS_DIEPINTx).

| Reserved | INEPTXFEM |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bits 31:16 Reserved, must be kept at reset value.
Bits 15:0 **INEPTXFEM**: IN EP Tx FIFO empty interrupt mask bits
These bits act as mask bits for OTG_FS_DIEPINTx.
TXFE interrupt one bit per IN endpoint:
Bit 0 for IN endpoint 0, bit 3 for IN endpoint 3
0: Masked interrupt
1: Unmasked interrupt

**OTG_FS device control IN endpoint 0 control register (OTG_FS_DIEPCTL0)**

Address offset: 0x900
Reset value: 0x0000 0000

This section describes the OTG_FS_DIEPCTL0 register. Nonzero control endpoints use
registers for endpoints 1–3.

| EPENA | EPDIS | Reserved | SNAK | CNAK | TXFNUM | STALL | Reserved | EPTYP | NAKSTS | Reserved | USBAEP | Reserved | MPSIZ |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
| r |  | w | rw | rs |  | r |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | rw |  |  |  |


STI
RM0090 Rev 18
1309/1749

--- Page 140 ---

**USB on-the-go full-speed (OTG_FS)**
RM0090

**Bit 31 EPENA: Endpoint enable**
The application sets this bit to start transmitting data on the endpoint 0.
The core clears this bit before setting any of the following interrupts on this endpoint:
– Endpoint disabled
– Transfer completed

**Bit 30 EPDIS: Endpoint disable**
The application sets this bit to stop transmitting data on an endpoint, even before the transfer for that endpoint is complete. The application must wait for the Endpoint disabled interrupt before treating the endpoint as disabled. The core clears this bit before setting the Endpoint disabled interrupt. The application must set this bit only if Endpoint enable is already set for this endpoint.

**Bits 29:28** Reserved, must be kept at reset value.

**Bit 27 SNAK: Set NAK**
A write to this bit sets the NAK bit for the endpoint.
Using this bit, the application can control the transmission of NAK handshakes on an endpoint. The core can also set this bit for an endpoint after a SETUP packet is received on that endpoint.

**Bit 26 CNAK: Clear NAK**
**A write to this bit clears the NAK bit for the endpoint.**

**Bits 25:22 TXFNUM: TxFIFO number**
This value is set to the FIFO number that is assigned to IN endpoint 0.

**Bit 21 STALL: STALL handshake**
The application can only set this bit, and the core clears it when a SETUP token is received for this endpoint. If a NAK bit, a Global IN NAK or Global OUT NAK is set along with this bit, the STALL bit takes priority.

**Bit 20** Reserved, must be kept at reset value.

**Bits 19:18 EPTYP: Endpoint type**
Hardcoded to '00' for control.

**Bit 17 NAKSTS: NAK status**
Indicates the following:
0: The core is transmitting non-NAK handshakes based on the FIFO status
1: The core is transmitting NAK handshakes on this endpoint.
When this bit is set, either by the application or core, the core stops transmitting data, even if there are data available in the TxFIFO. Irrespective of this bit's setting, the core always responds to SETUP data packets with an ACK handshake.

**Bit 16** Reserved, must be kept at reset value.

1310/1749
RM0090 Rev 18

--- Page 141 ---

**RM0090**
**USB on-the-go full-speed (OTG_FS)**

Bit 15 **USBAEP**: USB active endpoint
This bit is always set to 1, indicating that control endpoint 0 is always active in all configurations and interfaces.

Bits 14:2 Reserved, must be kept at reset value.

Bits 1:0 **MPSIZ**: Maximum packet size
The application must program this field with the maximum packet size for the current logical endpoint.
00: 64 bytes
01: 32 bytes
10: 16 bytes
11: 8 bytes

**OTG device endpoint x control register (OTG_FS_DIEPCTLx) (x = 1..3, where x = Endpoint_number)**

Address offset: 0x900 + 0x20 * x
Reset value: 0x0000 0000

The application uses this register to control the behavior of each logical endpoint other than endpoint 0.

| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| EPENA | EPDIS | SODDFRM | SD0PID/SEVNFRM | SNAK | CNAK | TXFNUM | STALL | Reserved | EPTYP | NAKSTS | EONUM/DPID | USBAEP | Reserved | MPSIZ |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| rs | w | rw | r | rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bit 31 **EPENA**: Endpoint enable
The application sets this bit to start transmitting data on an endpoint.
The core clears this bit before setting any of the following interrupts on this endpoint:
- SETUP phase done
- Endpoint disabled
- Transfer completed

Bit 30 **EPDIS**: Endpoint disable
The application sets this bit to stop transmitting/receiving data on an endpoint, even before the transfer for that endpoint is complete. The application must wait for the Endpoint disabled interrupt before treating the endpoint as disabled. The core clears this bit before setting the Endpoint disabled interrupt. The application must set this bit only if Endpoint enable is already set for this endpoint.

Bit 29 **SODDFRM**: Set odd frame
Applies to isochronous IN and OUT endpoints only.
Writing to this field sets the Even/Odd frame (EONUM) field to odd frame.

STI
RM0090 Rev 18
1311/1749

--- Page 142 ---

**USB on-the-go full-speed (OTG_FS)**
RM0090

**Bit 28 SD0PID:** Set DATA0 PID
Applies to interrupt/bulk IN endpoints only.
Writing to this field sets the endpoint data PID (DPID) field in this register to DATA0.

**SEVNFRM:** Set even frame
Applies to isochronous IN endpoints only.
Writing to this field sets the Even/Odd frame (EONUM) field to even frame.

**Bit 27 SNAK:** Set NAK
A write to this bit sets the NAK bit for the endpoint.
Using this bit, the application can control the transmission of NAK handshakes on an endpoint. The core can also set this bit for OUT endpoints on a Transfer completed interrupt, or after a SETUP is received on the endpoint.

**Bit 26 CNAK:** Clear NAK
A write to this bit clears the NAK bit for the endpoint.

**Bits 25:22 TXFNUM:** TxFIFO number
These bits specify the FIFO number associated with this endpoint. Each active IN endpoint must be programmed to a separate FIFO number.
This field is valid only for IN endpoints.

**Bit 21 STALL:** STALL handshake
Applies to non-control, non-isochronous IN endpoints only (access type is rw).
The application sets this bit to stall all tokens from the USB host to this endpoint. If a NAK bit, Global IN NAK, or Global OUT NAK is set along with this bit, the STALL bit takes priority.
Only the application can clear this bit, never the core.

**Bit 20** Reserved, must be kept at reset value.

**Bits 19:18 EPTYP:** Endpoint type
This is the transfer type supported by this logical endpoint.
00: Control
01: Isochronous
10: Bulk
11: Interrupt

**Bit 17 NAKSTS:** NAK status
It indicates the following:
0: The core is transmitting non-NAK handshakes based on the FIFO status.
1: The core is transmitting NAK handshakes on this endpoint.
When either the application or the core sets this bit:
For non-isochronous IN endpoints: The core stops transmitting any data on an IN endpoint, even if there are data available in the TxFIFO.
For isochronous IN endpoints: The core sends out a zero-length data packet, even if there are data available in the TxFIFO.
Irrespective of this bit's setting, the core always responds to SETUP data packets with an ACK handshake.

1312/1749
RM0090 Rev 18

--- Page 143 ---

RM0090
USB on-the-go full-speed (OTG_FS)

**Bit 16 EONUM**: Even/odd frame
Applies to isochronous IN endpoints only.
Indicates the frame number in which the core transmits/receives isochronous data for this endpoint. The application must program the even/odd frame number in which it intends to transmit/receive isochronous data for this endpoint using the SEVNFRM and SODDFRM fields in this register.
0: Even frame
1: Odd frame

**DPID**: Endpoint data PID
Applies to interrupt/bulk IN endpoints only.
Contains the PID of the packet to be received or transmitted on this endpoint. The application must program the PID of the first packet to be received or transmitted on this endpoint, after the endpoint is activated. The application uses the SD0PID register field to program either DATA0 or DATA1 PID.
0: DATA0
1: DATA1

**Bit 15 USBAEP**: USB active endpoint
Indicates whether this endpoint is active in the current configuration and interface. The core clears this bit for all endpoints (other than EP 0) after detecting a USB reset. After receiving the SetConfiguration and SetInterface commands, the application must program endpoint registers accordingly and set this bit.

**Bits 14:11** Reserved, must be kept at reset value.

**Bits 10:0 MPSIZ**: Maximum packet size
The application must program this field with the maximum packet size for the current logical endpoint. This value is in bytes.

ST
RM0090 Rev 18
1313/1749

--- Page 144 ---

USB on-the-go full-speed (OTG_FS)

RM0090

**OTG_FS device control OUT endpoint 0 control register (OTG_FS_DOEPCTL0)**

Address offset: 0xB00
Reset value: 0x0000 8000

This section describes the OTG_FS_DOEPCTL0 register. Nonzero control endpoints use registers for endpoints 1–3.

| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| EPENA | EPDIS | Reserved | SNAK | CNAK | Reserved | STALL | SNPM | EPTYP | NAKSTS | Reserved | USBBAEP | Reserved | MPSIZ |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| w | r |  |  | w |  |  |  |  | rs | rw | r |  | r |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | r |  |  |


**Bit 31 EPENA: Endpoint enable**
The application sets this bit to start transmitting data on endpoint 0.
The core clears this bit before setting any of the following interrupts on this endpoint:
- SETUP phase done
- Endpoint disabled
- Transfer completed

**Bit 30 EPDIS: Endpoint disable**
The application cannot disable control OUT endpoint 0.

**Bits 29:28** Reserved, must be kept at reset value.

**Bit 27 SNAK: Set NAK**
A write to this bit sets the NAK bit for the endpoint.
Using this bit, the application can control the transmission of NAK handshakes on an endpoint. The core can also set this bit on a Transfer completed interrupt, or after a SETUP is received on the endpoint.

**Bit 26 CNAK: Clear NAK**
A write to this bit clears the NAK bit for the endpoint.

**Bits 25:22** Reserved, must be kept at reset value.

**Bit 21 STALL: STALL handshake**
The application can only set this bit, and the core clears it, when a SETUP token is received for this endpoint. If a NAK bit or Global OUT NAK is set along with this bit, the STALL bit takes priority. Irrespective of this bit’s setting, the core always responds to SETUP data packets with an ACK handshake.

**Bit 20 SNPM: Snoop mode**
This bit configures the endpoint to Snoop mode. In Snoop mode, the core does not check the correctness of OUT packets before transferring them to application memory.

**Bits 19:18 EPTYP: Endpoint type**
Hardcoded to 2'b00 for control.

1314/1749
RM0090 Rev 18

--- Page 145 ---

RM0090
USB on-the-go full-speed (OTG_FS)

Bit 17 **NAKSTS**: NAK status
Indicates the following:
0: The core is transmitting non-NAK handshakes based on the FIFO status.
1: The core is transmitting NAK handshakes on this endpoint.
When either the application or the core sets this bit, the core stops receiving data, even if there is space in the RxFIFO to accommodate the incoming packet. Irrespective of this bit's setting, the core always responds to SETUP data packets with an ACK handshake.

Bit 16 Reserved, must be kept at reset value.

Bit 15 **USBAEP**: USB active endpoint
This bit is always set to 1, indicating that a control endpoint 0 is always active in all configurations and interfaces.

Bits 14:2 Reserved, must be kept at reset value.

Bits 1:0 **MPSIZ**: Maximum packet size
The maximum packet size for control OUT endpoint 0 is the same as what is programmed in control IN endpoint 0.
00: 64 bytes
01: 32 bytes
10: 16 bytes
11: 8 bytes

**OTG_FS device endpoint-x control register (OTG_FS_DOEPCTLx) (x = 1..3, where x = Endpoint_number)**

Address offset for OUT endpoints: 0xB00 + 0x20 * x
Reset value: 0x0000 0000

The application uses this register to control the behavior of each logical endpoint other than endpoint 0.

| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| EPENA | EPDIS | SODDFRM/SD1PID | SD0PID/SEVNRFRM | SNAK | CNAK | Reserved | STALL | SNPM | EPTYP | NAKSTS | EONUM/DPID | USBAEP | Reserved | MPSIZ |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| rs | w |  |  |  |  |  |  | rw | r | rw |  |  |  |  | rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bit 31 **EPENA**: Endpoint enable
Applies to IN and OUT endpoints.
The application sets this bit to start transmitting data on an endpoint.
The core clears this bit before setting any of the following interrupts on this endpoint:
- SETUP phase done
- Endpoint disabled
- Transfer completed

STI
RM0090 Rev 18
1315/1749

--- Page 146 ---

**USB on-the-go full-speed (OTG_FS)**
RM0090

**Bit 30 EPDIS: Endpoint disable**
The application sets this bit to stop transmitting/receiving data on an endpoint, even before the transfer for that endpoint is complete. The application must wait for the Endpoint disabled interrupt before treating the endpoint as disabled. The core clears this bit before setting the Endpoint disabled interrupt. The application must set this bit only if Endpoint enable is already set for this endpoint.

**Bit 29 SD1PID: Set DATA1 PID**
Applies to interrupt/bulk IN and OUT endpoints only. Writing to this field sets the endpoint data PID (DPID) field in this register to DATA1.

**SODDFRM: Set odd frame**
Applies to isochronous IN and OUT endpoints only. Writing to this field sets the Even/Odd frame (EONUM) field to odd frame.

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
Applies to non-control, non-isochronous OUT endpoints only (access type is rw).
The application sets this bit to stall all tokens from the USB host to this endpoint. If a NAK bit, Global IN NAK, or Global OUT NAK is set along with this bit, the STALL bit takes priority. Only the application can clear this bit, never the core.

**Bit 20 SNPM: Snoop mode**
This bit configures the endpoint to Snoop mode. In Snoop mode, the core does not check the correctness of OUT packets before transferring them to application memory.

**Bits 19:18 EPTYP: Endpoint type**
This is the transfer type supported by this logical endpoint.
00: Control
01: Isochronous
10: Bulk
11: Interrupt

1316/1749
RM0090 Rev 18
ST

--- Page 147 ---

RM0090
USB on-the-go full-speed (OTG_FS)

**Bit 17 NAKSTS: NAK status**
Indicates the following:
0: The core is transmitting non-NAK handshakes based on the FIFO status.
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
Contains the PID of the packet to be received or transmitted on this endpoint. The application must program the PID of the first packet to be received or transmitted on this endpoint, after the endpoint is activated. The application uses the SDOPID register field to program either DATA0 or DATA1 PID.
0: DATA0
1: DATA1

**Bit 15 USBAEP: USB active endpoint**
Indicates whether this endpoint is active in the current configuration and interface. The core clears this bit for all endpoints (other than EP 0) after detecting a USB reset. After receiving the SetConfiguration and SetInterface commands, the application must program endpoint registers accordingly and set this bit.

**Bits 14:11** Reserved, must be kept at reset value.

**Bits 10:0 MPSIZ: Maximum packet size**
The application must program this field with the maximum packet size for the current logical endpoint. This value is in bytes.

ST
RM0090 Rev 18
1317/1749

--- Page 148 ---

USB on-the-go full-speed (OTG_FS)

RM0090

**OTG_FS device endpoint-x interrupt register (OTG_FS_DIEPINTx) (x = 0..3, where x = Endpoint_number)**

Address offset: 0x908 + 0x20 * x
Reset value: 0x0000 0080

This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure 394. The application must read this register when the IN endpoints interrupt bit of the Core interrupt register (IEPINT in OTG_FS_GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (OTG_FS_DAINT) register to get the exact endpoint number for the Device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_FS_DAINT and OTG_FS_GINTSTS registers.

| 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 15 14 | 13 | 12 | 11 | 10 9 8 7 | 6 5 | 4 | 3 | 2 | 1 | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved | NAK | Reserved | PKTDRPSTS | Reserved | TXFE | INEPNE | INEPNM | ITTXFE | TOC | Reserved | EPDISD | XFRC |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | rcw1 |  | rcw1 | r | rcw1 |  | rcw1 |


Bits 31:14 Reserved, must be kept at reset value.

Bit 13 **NAK**: NAK input
The core generates this interrupt when a NAK is transmitted or received by the device. In case of isochronous IN endpoints the interrupt gets generated when a zero length packet is transmitted due to unavailability of data in the Tx FIFO.

Bit 12 Reserved, must be kept at reset value.

Bit 11 **PKTDRPSTS**: Packet dropped status
This bit indicates to the application that an ISOC OUT packet has been dropped. This bit does not have an associated mask bit and does not generate an interrupt.

Bits 10:8 Reserved, must be kept at reset value.

Bit 7 **TXFE**: Transmit FIFO empty
This interrupt is asserted when the TxFIFO for this endpoint is either half or completely empty. The half or completely empty status is determined by the TxFIFO Empty Level bit in the OTG_FS_GAHBCFG register (TXFELVL bit in OTG_FS_GAHBCFG).

Bit 6 **INEPNE**: IN endpoint NAK effective
This bit can be cleared when the application clears the IN endpoint NAK by writing to the CNAK bit in OTG_FS_DIEPCTLx.
This interrupt indicates that the core has sampled the NAK bit set (either by the application or by the core). The interrupt indicates that the IN endpoint NAK bit set by the application has taken effect in the core.
This interrupt does not guarantee that a NAK handshake is sent on the USB. A STALL bit takes priority over a NAK bit.

Bit 5 **INEPNM**: IN token received with EP mismatch.
Indicates that the data in the top of the non-periodic TxFIFO belongs to an endpoint other than the one for which the IN token was received. This interrupt is asserted on the endpoint for which the IN token was received.

1318/1749
RM0090 Rev 18

--- Page 149 ---

RM0090
USB on-the-go full-speed (OTG_FS)

Bit 4 **ITTXFE**: IN token received when TxFIFO is empty
Applies to non-periodic IN endpoints only.
Indicates that an IN token was received when the associated TxFIFO (periodic/non-periodic)
was empty. This interrupt is asserted on the endpoint for which the IN token was received.

Bit 3 **TOC**: Timeout condition
Applies only to Control IN endpoints.
Indicates that the core has detected a timeout condition on the USB for the last IN token on
this endpoint.

Bit 2 Reserved, must be kept at reset value.

Bit 1 **EPDISD**: Endpoint disabled interrupt
This bit indicates that the endpoint is disabled per the application's request.

Bit 0 **XFRC**: Transfer completed interrupt
This field indicates that the programmed transfer is complete on the AHB as well as on the
USB, for this endpoint.

**OTG_FS device endpoint-x interrupt register (OTG_FS_DOEPINTx) (x = 0..3, where x = Endpoint_number)**

Address offset: 0xB08 + 0x20 * x
Reset value: 0x0000 0080

This register indicates the status of an endpoint with respect to USB- and AHB-related
events. It is shown in Figure 394. The application must read this register when the OUT
Endpoints Interrupt bit of the OTG_FS_GINTSTS register (OEPINT bit in
OTG_FS_GINTSTS) is set. Before the application can read this register, it must first read
the OTG_FS_DAINT register to get the exact endpoint number for the OTG_FS_DOEPINTx
register. The application must clear the appropriate bit in this register to clear the
corresponding bits in the OTG_FS_DAINT and OTG_FS_GINTSTS registers.

| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved | NAKrc_w1 | BERRrc_w1 | Reserved | OUTPKTERRrc_w1 | Reserved | STSPHSRXrc_w1 | OTEPDISrc_w1 | STUPrc_w1 | Reserved | EPDISDrc_w1 | XFRCrc_w1 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bits 31:14 Reserved, must be kept at reset value.

Bit 13 **NAK**: NAK input
The core generates this interrupt when a NAK is transmitted or received by the device. In
case of isochronous IN endpoints the interrupt gets generated when a zero length packet is
transmitted due to unavailability of data in the Tx FIFO.

Bit 12 **BERR**: Babble error interrupt
The core generates this interrupt when babble is received for the endpoint.

Bits 11:9 Reserved, must be kept at reset value.

ST
RM0090 Rev 18
1319/1749

--- Page 150 ---

**USB on-the-go full-speed (OTG_FS)**
RM0090

**Bit 8 OUTPKTERR:** OUT packet error
This interrupt is asserted when the core detects an overflow or a CRC error for an OUT packet. This interrupt is valid only when thresholding is enabled.

**Bits 7:6** Reserved, must be kept at reset value.

**Bit 5 STSPHSRX:** Status phase received for control write
This interrupt is generated only after the core has transferred all the data that the host has sent during the data phase of a control write transfer, to the system memory buffer. The interrupt indicates to the application that the host has switched from data phase to the status phase of a control write transfer. The application can use this interrupt to ACK or STALL the status phase, after it has decoded the data phase.

**Bit 4 OTEPDIS:** OUT token received when endpoint disabled
Applies only to control OUT endpoints.
Indicates that an OUT token was received when the endpoint was not yet enabled. This interrupt is asserted on the endpoint for which the OUT token was received.

**Bit 3 STUP:** SETUP phase done
Applies to control OUT endpoint only.
Indicates that the SETUP phase for the control endpoint is complete and no more back-to-back SETUP packets were received for the current control transfer. On this interrupt, the application can decode the received SETUP data packet.

**Bit 2** Reserved, must be kept at reset value.

**Bit 1 EPDISD:** Endpoint disabled interrupt
This bit indicates that the endpoint is disabled per the application's request.

**Bit 0 XFRC:** Transfer completed interrupt
This field indicates that the programmed transfer is complete on the AHB as well as on the USB, for this endpoint.

**OTG_FS device IN endpoint 0 transfer size register (OTG_FS_DIEPTSIZ0)**
Address offset: 0x910
Reset value: 0x0000 0000

The application must modify this register before enabling endpoint 0. Once endpoint 0 is enabled using the endpoint enable bit in the device control endpoint 0 control registers (EPENA in OTG_FS_DIEPCTL0), the core modifies this register. The application can only read this register once the core has cleared the Endpoint enable bit.

Nonzero endpoints use the registers for endpoints 1–3.

| 31 30 29 28 27 26 25 24 23 22 21 20 | 19 18 | 17 16 15 14 13 12 11 10 9 8 7 6 | 5 4 3 2 1 0 |
|---|---|---|---|
| Reserved | PKTCNT | Reserved | XFRSIZ |
| rw |  |  |  |


1320/1749
RM0090 Rev 18

--- Page 151 ---

**RM0090**
**USB on-the-go full-speed (OTG_FS)**

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
1321/1749

--- Page 152 ---

USB on-the-go full-speed (OTG_FS)
RM0090

**OTG_FS device OUT endpoint 0 transfer size register (OTG_FS_DOEPTSIZ0)**

Address offset: 0xB10
Reset value: 0x0000 0000

The application must modify this register before enabling endpoint 0. Once endpoint 0 is enabled using the Endpoint enable bit in the OTG_FS_DOEPCTL0 registers (EPENA bit in OTG_FS_DOEPCTL0), the core modifies this register. The application can only read this register once the core has cleared the Endpoint enable bit.

Nonzero endpoints use the registers for endpoints 1–3.

| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved | STUPCNT | Reserved | PKTCNT | Reserved | XFRSIZ |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  | rw |  |  |  |  |  |  |  |  |  | rw |  |  |  |  |  |  |  |  |  |  |  | rw |  |  |  |  |  |  |  |  |


Bit 31 Reserved, must be kept at reset value.

Bits 30:29 **STUPCNT**: SETUP packet count
This field specifies the number of back-to-back SETUP data packets the endpoint can receive.
01: 1 packet
10: 2 packets
11: 3 packets

Bits 28:20 Reserved, must be kept at reset value.

Bit 19 **PKTCNT**: Packet count
This field is decremented to zero after a packet is written into the RxFIFO.

Bits 18:7 Reserved, must be kept at reset value.

Bits 6:0 **XFRSIZ**: Transfer size
Indicates the transfer size in bytes for endpoint 0. The core interrupts the application only after it has exhausted the transfer size amount of data. The transfer size can be set to the maximum packet size of the endpoint, to be interrupted at the end of each packet.
The core decrements this field every time a packet is read from the RxFIFO and written to the external memory.

1322/1749
RM0090 Rev 18
ST

--- Page 153 ---

RM0090
USB on-the-go full-speed (OTG_FS)

**OTG_FS device endpoint-x transfer size register (OTG_FS\_DIEPTSIZx)**
**(x = 1..3, where x = Endpoint_number)**

Address offset: 0x910 + 0x20 * x
Reset value: 0x0000 0000

The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the Endpoint enable bit in the OTG_FS\_DIEPCTLx registers (EPENA bit in OTG_FS\_DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the Endpoint enable bit.

| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved | PKTCNT | XFRSIZ |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| RW |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bits 31:29 Reserved, must be kept at reset value.

Bit 28:19 **PKTCNT**: Packet count
Indicates the total number of USB packets that constitute the Transfer Size amount of data for this endpoint.
This field is decremented every time a packet (maximum size or short packet) is read from the TxFIFO.

Bits 18:0 **XFRSIZ**: Transfer size
This field contains the transfer size in bytes for the current endpoint. The core only interrupts the application after it has exhausted the transfer size amount of data. The transfer size can be set to the maximum packet size of the endpoint, to be interrupted at the end of each packet.
The core decrements this field every time a packet from the external memory is written to the TxFIFO.

ST
RM0090 Rev 18
1323/1749

--- Page 154 ---

USB on-the-go full-speed (OTG_FS)

RM0090

**OTG_FS device IN endpoint transmit FIFO status register**
**(OTG_FS_DTXFSTSx) (x = 0..3, where x = Endpoint_number)**

Address offset for IN endpoints: 0x918 + 0x20 * x

This read-only register contains the free space information for the Device IN endpoint TxFIFO.

| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved | INEPTFSAV |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | r |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


31:16 Reserved, must be kept at reset value.

15:0 **INEPTFSAV**: IN endpoint TxFIFO space available
Indicates the amount of free space available in the Endpoint TxFIFO.
Values are in terms of 32-bit words:
0x0: Endpoint TxFIFO is full
0x1: 1 word available
0x2: 2 words available
0xn: n words available
Others: Reserved

**OTG_FS device OUT endpoint-x transfer size register (OTG_FS_DOEPTSIx)**
**(x = 1..3, where x = Endpoint_number)**

Address offset: 0xB10 + 0x20 * x

Reset value: 0x0000 0000

The application must modify this register before enabling the endpoint. Once the endpoint is enabled using Endpoint Enable bit of the OTG_FS_DOEPCTLx registers (EPENA bit in OTG_FS_DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the Endpoint enable bit.

| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved | RXDPID/STUPCNT | PKTCNT | XFRSIZ |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| r/rw | rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bit 31 Reserved, must be kept at reset value.

Bits 30:29 **RXDPID**: Received data PID
Applies to isochronous OUT endpoints only.
This is the data PID received in the last packet for this endpoint.
00: DATA0
01: DATA2
10: DATA1
11: MDATA

1324/1749

RM0090 Rev 18

--- Page 155 ---

RM0090
USB on-the-go full-speed (OTG_FS)

**STUPCNT:** SETUP packet count
Applies to control OUT Endpoints only.
This field specifies the number of back-to-back SETUP data packets the endpoint can receive.
01: 1 packet
10: 2 packets
11: 3 packets

Bit 28:19 **PKTCNT:** Packet count
Indicates the total number of USB packets that constitute the Transfer Size amount of data for this endpoint.
This field is decremented every time a packet (maximum size or short packet) is written to the RxFIFO.

Bits 18:0 **XFRSIZ:** Transfer size
This field contains the transfer size in bytes for the current endpoint. The core only interrupts the application after it has exhausted the transfer size amount of data. The transfer size can be set to the maximum packet size of the endpoint, to be interrupted at the end of each packet.
The core decrements this field every time a packet is read from the RxFIFO and written to the external memory.

> [RM0090 ch.34 §34.16.4] Device-mode registers

### 34.16.5 OTG_FS power and clock gating control register (OTG_FS_PCGCCTL)

### 34.16.5 OTG_FS power and clock gating control register (OTG_FS_PCGCCTL)

Address offset: 0xE00
Reset value: 0x0000 0000
This register is available in host and device modes.

| Reserved | PHYSUSP | Reserved | GATEHCLK | STPPCLK |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | rw |  | rw |


Bit 31:5 Reserved, must be kept at reset value.

Bit 4 **PHYSUSP:** PHY Suspended
Indicates that the PHY has been suspended. This bit is updated once the PHY is suspended after the application has set the STPPCLK bit (bit 0).

Bits 3:2 Reserved, must be kept at reset value.

Bit 1 **GATEHCLK:** Gate HCLK
The application sets this bit to gate HCLK to modules other than the AHB Slave and Master and wakeup logic when the USB is suspended or the session is not valid. The application clears this bit when the USB is resumed or a new session starts.

Bit 0 **STPPCLK:** Stop PHY clock
The application sets this bit to stop the PHY clock when the USB is suspended, the session is not valid, or the device is disconnected. The application clears this bit when the USB is resumed or a new session starts.

STI
RM0090 Rev 18
1325/1749

--- Page 156 ---

USB on-the-go full-speed (OTG_FS)
RM0090


> [RM0090 ch.35 §35.12.4] Device-mode registers


### 35.12.4 Device-mode registers

**OTG_HS device configuration register (OTG_HS_DCFG)**

Address offset: 0x800
Reset value: 0x0220 0000

This register configures the core in peripheral mode after power-on or after certain control commands or enumeration. Do not make changes to this register after initial programming.

| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved | PERSCHIVL | Reserved | PFIVL | DAD | Reserved | NZLSOHSK | DSPD |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bits 31:26 Reserved, must be kept at reset value.
Bits 25:24 **PERSCHIVL**: Periodic scheduling interval
This field specifies the amount of time the Internal DMA engine must allocate for fetching periodic IN endpoint data. Based on the number of periodic endpoints, this value must be specified as 25, 50 or 75% of the (micro)frame.
- When any periodic endpoints are active, the internal DMA engine allocates the specified amount of time in fetching periodic IN endpoint data
- When no periodic endpoint is active, then the internal DMA engine services nonperiodic endpoints, ignoring this field
- After the specified time within a (micro)frame, the DMA switches to fetching nonperiodic endpoints

00: 25% of (micro)frame
01: 50% of (micro)frame
10: 75% of (micro)frame
11: Reserved

Bits 23:13 Reserved, must be kept at reset value.

ST
RM0090 Rev 18
1443/1749

--- Page 79 ---

**USB on-the-go high-speed (OTG_HS)**

RM0090

**Bits 12:11 PFIVL: Periodic (micro)frame interval**

Indicates the time within a (micro) frame at which the application must be notified using the end of periodic (micro) frame interrupt. This can be used to determine if all the isochronous traffic for that frame is complete.

00: 80% of the frame interval
01: 85% of the frame interval
10: 90% of the frame interval
11: 95% of the frame interval

**Bits 10:4 DAD: Device address**

The application must program this field after every SetAddress control command.

**Bit 3** Reserved, must be kept at reset value.

**Bit 2 NZLSOHSK: Nonzero-length status OUT handshake**

The application can use this field to select the handshake the core sends on receiving a nonzero-length data packet during the OUT transaction of a control transfer's Status stage.

1: Send a STALL handshake on a nonzero-length status OUT transaction and do not send the received OUT packet to the application.
0: Send the received OUT packet to the application (zero-length or nonzero-length) and send a handshake based on the NAK and STALL bits for the endpoint in the device endpoint control register.

**Bits 1:0 DSPD: Device speed**

Indicates the speed at which the application requires the core to enumerate, or the maximum speed the application can support. However, the actual bus speed is determined only after the chirp sequence is completed, and is based on the speed of the USB host to which the core is connected.

00: High speed
01: Full speed using external ULPI PHY
10: Reserved
11: Full speed using internal embedded PHY

1444/1749
RM0090 Rev 18

--- Page 80 ---

RM0090
USB on-the-go high-speed (OTG_HS)


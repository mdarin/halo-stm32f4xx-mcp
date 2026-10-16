> [RM0090 ch.35 §35.12.4] Device-mode registers


### OTG_HS device control register (OTG_HS_DCTL)

Address offset: 0x804
Reset value: 0x0000 0000

| 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved | POP RG DNE | CGONAK | SGONAK | CGINAK | SGINAK | TCTL | GONSTS | GINSTS | SDIS | RWUSIG |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | r/w | w | r/w | r | r/w |


Bits 31:12 Reserved, must be kept at reset value.

**Bit 11 POPRGDNE:** Power-on programming done
The application uses this bit to indicate that register programming is completed after a wakeup from power down mode.

**Bit 10 CGONAK:** Clear global OUT NAK
Writing 1 to this field clears the Global OUT NAK.

**Bit 9 SGONAK:** Set global OUT NAK
Writing 1 to this field sets the Global OUT NAK.
The application uses this bit to send a NAK handshake on all OUT endpoints.
The application must set the this bit only after making sure that the Global OUT NAK effective bit in the Core interrupt register (GONAKEFF bit in OTG_HS_GINTSTS) is cleared.

**Bit 8 CGINAK:** Clear global IN NAK
Writing 1 to this field clears the Global IN NAK.

**Bit 7 SGINAK:** Set global IN NAK
Writing 1 to this field sets the Global nonperiodic IN NAK. The application uses this bit to send a NAK handshake on all nonperiodic IN endpoints.
The application must set this bit only after making sure that the Global IN NAK effective bit in the Core interrupt register (GINAKEFF bit in OTG_HS_GINTSTS) is cleared.

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

ST
RM0090 Rev 18
1445/1749

--- Page 81 ---

**USB on-the-go high-speed (OTG_HS)**

**RM0090**

**Bit 2 GINSTS:** Global IN NAK status
0: A handshake is sent out based on the data availability in the transmit FIFO.
1: A NAK handshake is sent out on all nonperiodic IN endpoints, irrespective of the data availability in the transmit FIFO.

**Bit 1 SDIS:** Soft disconnect
The application uses this bit to signal the USB OTG core to perform a soft disconnect. As long as this bit is set, the host does not see that the device is connected, and the device does not receive signals on the USB. The core stays in the disconnected state until the application clears this bit.
0: Normal operation. When this bit is cleared after a soft disconnect, the core generates a device connect event to the USB host. When the device is reconnected, the USB host restarts device enumeration.
1: The core generates a device disconnect event to the USB host.

**Bit 0 RWUSIG:** Remote wakeup signaling
When the application sets this bit, the core initiates remote signaling to wake up the USB host. The application must set this bit to instruct the core to exit the Suspend state. As specified in the USB 2.0 specification, the application must clear this bit 1 ms to 15 ms after setting it.

*Table 214* contains the minimum duration (according to device state) for which the Soft disconnect (SDIS) bit must be set for the USB host to detect a device disconnect. To accommodate clock jitter, it is recommended that the application add some extra delay to the specified minimum duration.

**Table 214. Minimum duration for soft disconnect**

| Operating speed | Device state | Minimum duration |
|---|---|---|
| High speed | Not Idle or Suspended (Performing transactions) | 125 µs |
| Full speed | Suspended | 1 ms + 2.5 µs |
| Full speed | Idle | 2.5 µs |
| Full speed | Not Idle or Suspended (Performing transactions) | 2.5 µs |


1446/1749
RM0090 Rev 18

--- Page 82 ---

**RM0090**
**USB on-the-go high-speed (OTG_HS)**


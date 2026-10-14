> [RM0090 ch.35 §35.12.2] OTG_HS global registers


### 35.12.2 OTG_HS global registers

These registers are available in both host and peripheral modes, and do not need to be reprogrammed when switching between these modes.

ST
RM0090 Rev 18
1407/1749

--- Page 43 ---

**USB on-the-go high-speed (OTG_HS)**
RM0090

Bit values in the register descriptions are expressed in binary unless otherwise specified.

**OTG_HS control and status register (OTG_HS_GOTGCTL)**

Address offset: 0x000
Reset value: 0x0001 0000

The OTG control and status register controls the behavior and reflects the status of the OTG function of the core.

| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved | BSVLD | ASVLD | DBCT | CIDSTS | Reserved | DHNPEN | HSHNPEN | HNPRQ | HNGSCS | Reserved | SRQ | SROSCS |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  | r |  |  |  |  | rw | r |  |  |  |  | rw | r |  |  |  |  |  |  |  |


Bits 31:20 Reserved, must be kept at reset value.

**Bit 19 BSVLD: B-session valid**
Indicates the peripheral mode transceiver status.
0: B-session is not valid.
1: B-session is valid.
In OTG mode, you can use this bit to determine if the device is connected or disconnected.
*Note: Only accessible in peripheral mode.*

**Bit 18 ASVLD: A-session valid**
Indicates the host mode transceiver status.
0: A-session is not valid
1: A-session is valid
*Note: Only accessible in host mode.*

**Bit 17 DBCT: Long/short debounce time**
Indicates the debounce time of a detected connection.
0: Long debounce time, used for physical connections (100 ms + 2.5 µs)
1: Short debounce time, used for soft connections (2.5 µs)
*Note: Only accessible in host mode.*

**Bit 16 CIDSTS: Connector ID status**
Indicates the connector ID status on a connect event.
0: The OTG_HS controller is in A-device mode
1: The OTG_HS controller is in B-device mode
*Note: Accessible in both peripheral and host modes.*

Bits 15:12 Reserved, must be kept at reset value.

**Bit 11 DHNPEN: Device HNP enabled**
The application sets this bit when it successfully receives a SetFeature.SetHNPEnable command from the connected USB host.
0: HNP is not enabled in the application
1: HNP is enabled in the application
*Note: Only accessible in peripheral mode.*

1408/1749
RM0090 Rev 18

--- Page 44 ---

RM0090
USB on-the-go high-speed (OTG_HS)

**Bit 10 HSHNPEN**: Host set HNP enable
The application sets this bit when it has successfully enabled HNP (using the SetFeature.SetHNPEnable command) on the connected device.
0: Host Set HNP is not enabled
1: Host Set HNP is enabled
*Note: Only accessible in host mode.*

**Bit 9 HNPRQ**: HNP request
The application sets this bit to initiate an HNP request to the connected USB host. The application can clear this bit by writing a 0 when the host negotiation success status change bit in the OTG interrupt register (HNSSCHG bit in OTG_HS_GOTGINT) is set. The core clears this bit when the HNSSCHG bit is cleared.
0: No HNP request
1: HNP request
*Note: Only accessible in peripheral mode.*

**Bit 8 HNGSCS**: Host negotiation success
The core sets this bit when host negotiation is successful. The core clears this bit when the HNP Request (HNPRQ) bit in this register is set.
0: Host negotiation failure
1: Host negotiation success
*Note: Only accessible in peripheral mode.*

Bits 7:2 Reserved, must be kept at reset value.

**Bit 1 SRQ**: Session request
The application sets this bit to initiate a session request on the USB. The application can clear this bit by writing a 0 when the host negotiation success status change bit in the OTG Interrupt register (HNSSCHG bit in OTG_HS_GOTGINT) is set. The core clears this bit when the HNSSCHG bit is cleared.
If you use the USB 1.1 full-speed serial transceiver interface to initiate the session request, the application must wait until VBUS discharges to 0.2 V, after the B-Session Valid bit in this register (BSVLD bit in OTG_HS_GOTGCTL) is cleared.
0: No session request
1: Session request
*Note: Only accessible in peripheral mode.*

**Bit 0 SRQSCS**: Session request success
The core sets this bit when a session request initiation is successful.
0: Session request failure
1: Session request success
*Note: Only accessible in peripheral mode.*

**OTG_HS interrupt register (OTG_HS_GOTGINT)**
Address offset: 0x04
Reset value: 0x0000 0000

The application reads this register whenever there is an OTG interrupt and clears the bits in this register to clear the OTG interrupt.

ST
RM0090 Rev 18
1409/1749

--- Page 45 ---

USB on-the-go high-speed (OTG_HS)

RM0090

| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved | DBCDNE | ADTOCHG | HNGDET | Reserved | HNSSCHG | SRSSCHG | Reserved | SEDET | Res. |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  | rc_w1 |  |  |  |  |  |  |  | rc_w1 |  |  | rc_w1 |  |  |  |  |  |  |  |  |


Bits 31:20 Reserved, must be kept at reset value.

**Bit 19 DBCDNE: Debounce done**
The core sets this bit when the debounce is completed after the device connect. The application can start driving USB reset after seeing this interrupt. This bit is only valid when the HNP Capable or SRP Capable bit is set in the Core USB Configuration register (HNPCAP bit or SRPCAP bit in OTG_HS_GUSBCFG, respectively).
*Note: Only accessible in host mode.*

**Bit 18 ADTOCHG: A-device timeout change**
The core sets this bit to indicate that the A-device has timed out while waiting for the B-device to connect.
*Note: Accessible in both peripheral and host modes.*

**Bit 17 HNGDET: Host negotiation detected**
The core sets this bit when it detects a host negotiation request on the USB.
*Note: Accessible in both peripheral and host modes.*

Bits 16:10 Reserved, must be kept at reset value.

**Bit 9 HNSSCHG: Host negotiation success status change**
The core sets this bit on the success or failure of a USB host negotiation request. The application must read the host negotiation success bit of the OTG Control and Status register (HNGSCS in OTG_HS_GOTGCTL) to check for success or failure.
*Note: Accessible in both peripheral and host modes.*

Bits 7:3 Reserved, must be kept at reset value.

**Bit 8 SRSSCHG: Session request success status change**
The core sets this bit on the success or failure of a session request. The application must read the session request success bit in the OTG Control and status register (SRQSCS bit in OTG_HS_GOTGCTL) to check for success or failure.
*Note: Accessible in both peripheral and host modes.*

**Bit 2 SEDET: Session end detected**
The core sets this bit to indicate that the level of the voltage on VBUS is no longer valid for a B-device session when VBUS < 0.8 V.

Bits 1:0 Reserved, must be kept at reset value.

1410/1749
RM0090 Rev 18

--- Page 46 ---

RM0090
USB on-the-go high-speed (OTG_HS)


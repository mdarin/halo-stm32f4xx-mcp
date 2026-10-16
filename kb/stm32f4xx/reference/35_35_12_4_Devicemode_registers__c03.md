> [RM0090 ch.35 §35.12.4] Device-mode registers


### OTG_HS device status register (OTG_HS_DSTS)

Address offset: 0x808
Reset value: 0x0000 0010

This register indicates the status of the core with respect to USB-related events. It must be read on interrupts from the device all interrupts (OTG_HS_DAINT) register.

| 31 30 29 28 27 26 25 24 23 22 | 21 20 19 18 17 16 15 14 13 12 11 10 9 8 | 7 6 5 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|
| Reserved | FNSOF | Reserved | EERR | ENUMSPD | SUSPSTS |  |
| r |  |  |  |  |  |  |


**Bits 31:22** Reserved, must be kept at reset value.
**Bits 21:8** **FNSOF**: Frame number of the received SOF
**Bits 7:4** Reserved, must be kept at reset value.
**Bit 3** **EERR**: Erratic error
The core sets this bit to report any erratic errors.
Due to erratic errors, the OTG_HS controller goes into Suspended state and an interrupt is generated to the application with Early suspend bit of the Core interrupt register (ESUSP bit in OTG_HS_GINTSTS). If the early suspend is asserted due to an erratic error, the application can only perform a soft disconnect recover.
**Bits 2:1** **ENUMSPD**: Enumerated speed
Indicates the speed at which the OTG_HS controller has come up after speed detection through a chirp sequence.
00: High speed
01: Reserved
10: Reserved
11: Full speed (PHY clock is running at 48 MHz)
Others: reserved
**Bit 0** **SUSPSTS**: Suspend status
In peripheral mode, this bit is set as long as a Suspend condition is detected on the USB.
The core enters the Suspended state when there is no activity on the USB data lines for a period of 3 ms. The core comes out of the suspend:
– When there is an activity on the USB data lines
– When the application writes to the Remote wakeup signaling bit in the Device control register (RWUSIG bit in OTG_HS_DCTL).

STI
RM0090 Rev 18
1447/1749

--- Page 83 ---

USB on-the-go high-speed (OTG_HS)
RM0090

**OTG_HS device IN endpoint common interrupt mask register (OTG_HS_DIEPMSK)**

Address offset: 0x810
Reset value: 0x0000 0000

This register works with each of the Device IN endpoint interrupt (OTG_HS_DIEPINTx) registers for all endpoints to generate an interrupt per IN endpoint. The IN endpoint interrupt for a specific status in the OTG_HS_DIEPINTx register can be masked by writing to the corresponding bit in this register. Status bits are masked by default.

| 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0 |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved | NAKM | Reserved | TXFURM | Reserved | INEPNEM | INEPNMM | ITTXFEMSK | TOM | AHBERRM | EPDM | XFRM |
| rw |  |  |  |  |  |  |  |  |  |  |  |


Bits 31:10 Reserved, must be kept at reset value.

**Bit 13 NAKM**: NAK interrupt mask
0: Masked interrupt
1: Unmasked interrupt

Bits 12:9 Reserved, must be kept at reset value.

**Bit 8 TXFURM**: FIFO underrun mask
0: Masked interrupt
1: Unmasked interrupt

Bit 7 Reserved, must be kept at reset value.

**Bit 6 INEPNEM**: IN endpoint NAK effective mask
0: Masked interrupt
1: Unmasked interrupt

**Bit 5 INEPNMM**: IN token received with EP mismatch mask
0: Masked interrupt
1: Unmasked interrupt

**Bit 4 ITTXFEMSK**: IN token received when TxFIFO empty mask
0: Masked interrupt
1: Unmasked interrupt

**Bit 3 TOM**: Timeout condition mask (nonisochronous endpoints)
0: Masked interrupt
1: Unmasked interrupt

1448/1749
RM0090 Rev 18
ST

--- Page 84 ---

RM0090
USB on-the-go high-speed (OTG_HS)

Bit 2 **AHBERRM**: AHB error mask
0: Masked interrupt
1: Unmasked interrupt

Bit 1 **EPDM**: Endpoint disabled interrupt mask
0: Masked interrupt
1: Unmasked interrupt

Bit 0 **XFRM**: Transfer completed interrupt mask
0: Masked interrupt
1: Unmasked interrupt

**OTG_HS device OUT endpoint common interrupt mask register (OTG_HS_DOEPMSK)**

Address offset: 0x814
Reset value: 0x0000 0000

This register works with each of the Device OUT endpoint interrupt (OTG_HS_DOEPINTx) registers for all endpoints to generate an interrupt per OUT endpoint. The OUT endpoint interrupt for a specific status in the OTG_HS_DOEPINTx register can be masked by writing into the corresponding bit in this register. Status bits are masked by default.

| 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved | NYETMSK | NAKMSK | BERRM | Reserved | OPEM | Reserved | B2BSTUP | STSPHSRXM | OTEPDM | STUPM | AHBERRM | EPDM | XFRM |
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bits 31:15 Reserved, must be kept at reset value.

Bit 14 **NYETMSK**: NYET interrupt mask
0: Masked interrupt
1: Unmasked interrupt

Bit 13 **NAKMSK**: NAK interrupt mask
0: Masked interrupt
1: Unmasked interrupt

Bit 12 **BERRM**: Babble error interrupt mask
0: Masked interrupt
1: Unmasked interrupt

Bits 11:9 Reserved, must be kept at reset value.

Bit 8 **OPEM**: OUT packet error mask
0: Masked interrupt
1: Unmasked interrupt

Bit 7 Reserved, must be kept at reset value.

ST
RM0090 Rev 18
1449/1749

--- Page 85 ---

**USB on-the-go high-speed (OTG_HS)**
RM0090

Bit 6 **B2BSTUP**: Back-to-back SETUP packets received mask
Applies to control OUT endpoints only.
0: Masked interrupt
1: Unmasked interrupt

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

Bit 2 **AHBERRM**: AHB error mask
0: Masked interrupt
1: Unmasked interrupt

Bit 1 **EPDM**: Endpoint disabled interrupt mask
0: Masked interrupt
1: Unmasked interrupt

Bit 0 **XFRCM**: Transfer completed interrupt mask
0: Masked interrupt
1: Unmasked interrupt

**OTG_HS device all endpoints interrupt register (OTG_HS_DAINIT)**
Address offset: 0x818
Reset value: 0x0000 0000

When a significant event occurs on an endpoint, a device all endpoints interrupt register interrupts the application using the Device OUT endpoints interrupt bit or Device IN endpoints interrupt bit of the Core interrupt register (OEPINT or IEPINT in OTG_HS_GINTSTS, respectively). There is one interrupt bit per endpoint, up to a maximum of 16 bits for OUT endpoints and 16 bits for IN endpoints. For a bidirectional endpoint, the corresponding IN and OUT interrupt bits are used. Bits in this register are set and cleared when the application sets and clears bits in the corresponding Device Endpoint-x interrupt register (OTG_HS_DIEPINTx/OTG_HS_DOEPINTx).

| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| OEPINT | IEPINT |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| r |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


1450/1749
RM0090 Rev 18
ST

--- Page 86 ---

RM0090
USB on-the-go high-speed (OTG_HS)

Bits 31:16 **OEPINT**: OUT endpoint interrupt bits
One bit per OUT endpoint:
Bit 16 for OUT endpoint 0, bit 31 for OUT endpoint 15

Bits 15:0 **IEPINT**: IN endpoint interrupt bits
One bit per IN endpoint:
Bit 0 for IN endpoint 0, bit 15 for endpoint 15

**OTG_HS all endpoints interrupt mask register (OTG_HS_DAINTMSK)**

Address offset: 0x81C
Reset value: 0x0000 0000

The device endpoint interrupt mask register works with the device endpoint interrupt register to interrupt the application when an event occurs on a device endpoint. However, the device all endpoints interrupt (OTG_HS_DAINT) register bit corresponding to that interrupt is still set.

| OEPM | IEPM |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bits 31:16 **OEPM**: OUT EP interrupt mask bits
One per OUT endpoint:
Bit 16 for OUT EP 0, bit 18 for OUT EP 3
0: Masked interrupt
1: Unmasked interrupt

Bits 15:0 **IEPM**: IN EP interrupt mask bits
One bit per IN endpoint:
Bit 0 for IN EP 0, bit 3 for IN EP 3
0: Masked interrupt
1: Unmasked interrupt

**OTG_HS device V<sub>BUS</sub> discharge time register (OTG_HS_DVBUSDIS)**

Address offset: 0x0828
Reset value: 0x0000 17D7

This register specifies the V<sub>BUS</sub> discharge time after V<sub>BUS</sub> pulsing during SRP.

| Reserved | VBUSDT |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bits 31:16 Reserved, must be kept at reset value.
Bits 15:0 **VBUSDT**: Device V<sub>BUS</sub> discharge time
Specifies the V<sub>BUS</sub> discharge time after V<sub>BUS</sub> pulsing during SRP. This value equals:
V<sub>BUS</sub> discharge time in PHY clocks / 1 024
Depending on your V<sub>BUS</sub> load, this value may need adjusting.

ST
RM0090 Rev 18
1451/1749

--- Page 87 ---

USB on-the-go high-speed (OTG_HS)
RM0090

**OTG_HS device V<sub>BUS</sub> pulsing time register (OTG_HS_DVBUSPULSE)**

Address offset: 0x082C
Reset value: 0x0000 05B8

This register specifies the V<sub>BUS</sub> pulsing time during SRP.

| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved | DVBUSP |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | r/w |  |  |  |  |  |  |  |  |  |  |  |


Bits 31:12 Reserved, must be kept at reset value.

Bits 11:0 **DVBUSP**: Device V<sub>BUS</sub> pulsing time
Specifies the V<sub>BUS</sub> pulsing time during SRP. This value equals:
V<sub>BUS</sub> pulsing time in PHY clocks / 1 024

1452/1749
RM0090 Rev 18

--- Page 88 ---

RM0090
USB on-the-go high-speed (OTG_HS)

**OTG_HS Device threshold control register (OTG_HS_DTHRCTL)**

Address offset: 0x0830
Reset value: 0x0000 0000

| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved | ARPEN | Reserved | RXTHRLEN | RXTHREN | Reserved | TXTHRLEN | ISOTHREN | NONISOTHREN |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  | rw |  |  |  |  |  | rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bits 31:28 Reserved, must be kept at reset value.

Bit 27 **ARPEN**: Arbiter parking enable
This bit controls internal DMA arbiter parking for IN endpoints. When thresholding is enabled and this bit is set to one, then the arbiter parks on the IN endpoint for which there is a token received on the USB. This is done to avoid getting into underrun conditions. By default parking is enabled.

Bit 26 Reserved, must be kept at reset value.

Bits 25: 17 **RXTHRLEN**: Receive threshold length
This field specifies the receive thresholding size in words. This field also specifies the amount of data received on the USB before the core can start transmitting on the AHB. The threshold length has to be at least eight words. The recommended value for RXTHRLEN is to be the same as the programmed AHB burst length (HBSTLEN bit in OTG_HS_GAHBCFG).

Bit 16 **RXTHREN**: Receive threshold enable
When this bit is set, the core enables thresholding in the receive direction.

Bits 15: 11 Reserved, must be kept at reset value.

Bits 10:2 **TXTHRLEN**: Transmit threshold length
This field specifies the transmit thresholding size in words. This field specifies the amount of data in bytes to be in the corresponding endpoint transmit FIFO, before the core can start transmitting on the USB. The threshold length has to be at least eight words. This field controls both isochronous and nonisochronous IN endpoint thresholds. The recommended value for TXTHRLEN is to be the same as the programmed AHB burst length (HBSTLEN bit in OTG_HS_GAHBCFG).

Bit 1 **ISOTHREN**: ISO IN endpoint threshold enable
When this bit is set, the core enables thresholding for isochronous IN endpoints.

Bit 0 **NONISOTHREN**: Nonisochronous IN endpoints threshold enable
When this bit is set, the core enables thresholding for nonisochronous IN endpoints.

STI
RM0090 Rev 18
1453/1749

--- Page 89 ---

USB on-the-go high-speed (OTG_HS)
RM0090

**OTG_HS device IN endpoint FIFO empty interrupt mask register: (OTG_HS_DIEPEMPMSK)**

Address offset: 0x834
Reset value: 0x0000 0000

This register is used to control the IN endpoint FIFO empty interrupt generation (TXFE_OTG_HS_DIEPINTx).

| Reserved | INEPTXFEM |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bits 31:16 Reserved, must be kept at reset value.
Bits 15:0 **INEPTXFEM**: IN EP Tx FIFO empty interrupt mask bits
These bits act as mask bits for OTG_HS_DIEPINTx.
TXFE interrupt one bit per IN endpoint:
Bit 0 for IN endpoint 0, bit 15 for IN endpoint 15
0: Masked interrupt
1: Unmasked interrupt

**OTG_HS device each endpoint interrupt register (OTG_HS_DEACHINT)**

Address offset: 0x0838
Reset value: 0x0000 0000

There is one interrupt bit for endpoint 1 IN and one interrupt bit for endpoint 1 OUT.

| Reserved | OEP1INT | Reserved | IEP1INT | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  | r |  |  |  |  |  |  |  |  |  |  |  |  |  |  | r |  |  |


Bits 31:18 Reserved, must be kept at reset value.
Bit 17 **OEP1INT**: OUT endpoint 1 interrupt bit
Bits 16:2 Reserved, must be kept at reset value.
Bit 1 **IEP1INT**: IN endpoint 1 interrupt bit
Bit 0 Reserved, must be kept at reset value.

1454/1749
RM0090 Rev 18

--- Page 90 ---

RM0090
USB on-the-go high-speed (OTG_HS)

**OTG_HS device each endpoint interrupt register mask (OTG_HS_DEACHINTMSK)**
Address offset: 0x083C
Reset value: 0x0000 0000
There is one interrupt bit for endpoint 1 IN and one interrupt bit for endpoint 1 OUT.

| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved | OEP1INTM | Reserved | IEP1INTM | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  | rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  | rw |  |  |


Bits 31:18 Reserved, must be kept at reset value.
Bit 17 **OEP1INTM**: OUT Endpoint 1 interrupt mask bit
Bits 16:2 Reserved, must be kept at reset value.
Bit 1 **IEP1INTM**: IN Endpoint 1 interrupt mask bit
Bit 0 Reserved, must be kept at reset value.

**OTG_HS device each in endpoint-1 interrupt register (OTG_HS_DIEPEACHMSK1)**
Address offset: 0x844
Reset value: 0x0000 0000

| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved | NAKM | Reserved | BIM | TXFURM | Reserved | INEPNEM | INEPNMM | ITTXFEMSK | TOM | AHBERRM | EPDM | XFRCM |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | rw |  | rw |  | rw |  |  |  |  |  |  |  |  |  |


Bits 31:14 Reserved, must be kept at reset value.
Bit 13 **NAKM**: NAK interrupt mask
0: Masked interrupt
1: unmasked interrupt
Bit 12:10 Reserved, must be kept at reset value.
Bit 9 **BIM**: BNA interrupt mask
0: Masked interrupt
1: Unmasked interrupt
Bit 8 **TXFURM**: FIFO underrun mask
0: Masked interrupt
1: Unmasked interrupt
Bit 7 Reserved, must be kept at reset value.

STI
RM0090 Rev 18
1455/1749

--- Page 91 ---

USB on-the-go high-speed (OTG_HS)

RM0090

Bit 6 **INEPNEM**: IN endpoint NAK effective mask
0: Masked interrupt
1: Unmasked interrupt

Bit 5 **INEPNMM**: IN token received with EP mismatch mask
0: Masked interrupt
1: Unmasked interrupt

Bit 4 **ITTXFEMSK**: IN token received when TxFIFO empty mask
0: Masked interrupt
1: Unmasked interrupt

Bit 3 **TOM**: Timeout condition mask (nonisochronous endpoints)
0: Masked interrupt
1: Unmasked interrupt

Bit 2 **AHBERRM**: AHB error mask
0: Masked interrupt
1: Unmasked interrupt

Bit 1 **EPDM**: Endpoint disabled interrupt mask
0: Masked interrupt
1: Unmasked interrupt

Bit 0 **XFRM**: Transfer completed interrupt mask
0: Masked interrupt
1: Unmasked interrupt

**OTG_HS device each OUT endpoint-1 interrupt register (OTG_HS_DOEPEACHMSK1)**

Address offset: 0x884
Reset value: 0x0000 0000

| 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |
| NYETM | NAKM | BERRM | Reserved | BIM | TXFURM | Reserved | INEPNEM | INEPNMM | ITTXFEMSK | TOM | AHBERRM | EPDM | XFRM |
| rw |  |  |  | rw |  |  | rw |  |  |  |  |  |  |


Bits 31:15 Reserved, must be kept at reset value.

Bit 14 **NYETM**: NYET interrupt mask
0: Masked interrupt
1: unmasked interrupt

Bit 13 **NAKM**: NAK interrupt mask
0: Masked interrupt
1: Unmasked interrupt

Bit 12 **BERRM**: Bubble error interrupt mask
0: Masked interrupt
1: Unmasked interrupt

Bit 11:10 Reserved, must be kept at reset value.

1456/1749
RM0090 Rev 18

--- Page 92 ---

RM0090
USB on-the-go high-speed (OTG_HS)

Bit 9 **BIM**: BNA interrupt mask
0: Masked interrupt
1: Unmasked interrupt

Bit 8 **OPEM**: OUT packet error mask
0: Masked interrupt
1: Unmasked interrupt

Bits 7:3 Reserved, must be kept at reset value.

Bit 2 **AHBERRM**: AHB error mask
0: Masked interrupt
1: Unmasked interrupt

Bit 1 **EPDM**: Endpoint disabled interrupt mask
0: Masked interrupt
1: Unmasked interrupt

Bit 0 **XFRM**: Transfer completed interrupt mask
0: Masked interrupt
1: Unmasked interrupt

**OTG device endpoint-x control register (OTG_HS_DIEPCTLx) (x = 0..5, where x = Endpoint_number)**

Address offset: 0x900 + 0x20 * x
Reset value: 0x0000 0000

The application uses this register to control the behavior of each logical endpoint other than endpoint 0.

| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| EPENA | EPDIS | SODDFRM | SDPID/SEVNFRM | SNAK | CNAK | TXFNUM | Stall | Reserved | EPTYP | NAKSTS | EONUM/DPID | USBBAEP | Reserved | MPSIZ |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| rw | rw/rs | rw | r | rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


ST
RM0090 Rev 18
1457/1749

--- Page 93 ---

**USB on-the-go high-speed (OTG_HS)**
RM0090

**Bit 31 EPENA: Endpoint enable**
The application sets this bit to start transmitting data on an endpoint.
The core clears this bit before setting any of the following interrupts on this endpoint:
– SETUP phase done
– Endpoint disabled
– Transfer completed

**Bit 30 EPDIS: Endpoint disable**
The application sets this bit to stop transmitting/receiving data on an endpoint, even before the transfer for that endpoint is complete. The application must wait for the Endpoint disabled interrupt before treating the endpoint as disabled. The core clears this bit before setting the Endpoint disabled interrupt. The application must set this bit only if Endpoint enable is already set for this endpoint.

**Bit 29 SODDFRM: Set odd frame**
Applies to isochronous IN and OUT endpoints only.
Writing to this field sets the Even/Odd frame (EONUM) field to odd frame.

**Bit 28 SD0PID: Set DATA0 PID**
Applies to interrupt/bulk IN endpoints only.
Writing to this field sets the endpoint data PID (DPID) field in this register to DATA0.

**SEVNFRM: Set even frame**
Applies to isochronous IN endpoints only.
Writing to this field sets the Even/Odd frame (EONUM) field to even frame.

**Bit 27 SNAK: Set NAK**
A write to this bit sets the NAK bit for the endpoint.
Using this bit, the application can control the transmission of NAK handshakes on an endpoint. The core can also set this bit for OUT endpoints on a Transfer completed interrupt, or after a SETUP is received on the endpoint.

**Bit 26 CNAK: Clear NAK**
A write to this bit clears the NAK bit for the endpoint.

**Bits 25:22 TXFNUM: TxFIFO number**
These bits specify the FIFO number associated with this endpoint. Each active IN endpoint must be programmed to a separate FIFO number.
This field is valid only for IN endpoints.

**Bit 21 STALL: STALL handshake**
Applies to noncontrol, nonisochronous IN endpoints only (access type is rw).
The application sets this bit to stall all tokens from the USB host to this endpoint. If a NAK bit, Global IN NAK, or Global OUT NAK is set along with this bit, the STALL bit takes priority. Only the application can clear this bit, never the core.

Applies to control endpoints only (access type is rs).
The application can only set this bit, and the core clears it, when a SETUP token is received for this endpoint. If a NAK bit, Global IN NAK, or Global OUT NAK is set along with this bit, the STALL bit takes priority. Irrespective of this bit’s setting, the core always responds to SETUP data packets with an ACK handshake.

**Bit 20** Reserved, must be kept at reset value.

1458/1749
RM0090 Rev 18
STI

--- Page 94 ---

RM0090
USB on-the-go high-speed (OTG_HS)

**Bits 19:18 EPTYP: Endpoint type**
This is the transfer type supported by this logical endpoint.
00: Control
01: Isochronous
10: Bulk
11: Interrupt

**Bit 17 NAKSTS: NAK status**
It indicates the following:
0: The core is transmitting nonNAK handshakes based on the FIFO status.
1: The core is transmitting NAK handshakes on this endpoint.

When either the application or the core sets this bit:
For nonisochronous IN endpoints: The core stops transmitting any data on an IN endpoint, even if there are data available in the TxFIFO.
For isochronous IN endpoints: The core sends out a zero-length data packet, even if there are data available in the TxFIFO.

Irrespective of this bit's setting, the core always responds to SETUP data packets with an ACK handshake.

**Bit 16 EONUM: Even/odd frame**
Applies to isochronous IN endpoints only.
Indicates the frame number in which the core transmits/receives isochronous data for this endpoint. The application must program the even/odd frame number in which it intends to transmit/receive isochronous data for this endpoint using the SEVNFRM and SODDFRM fields in this register.
0: Even frame
1: Odd frame

**DPID: Endpoint data PID**
Applies to interrupt/bulk IN endpoints only.
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
1459/1749

--- Page 95 ---

USB on-the-go high-speed (OTG_HS)
RM0090


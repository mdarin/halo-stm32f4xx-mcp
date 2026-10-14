### 35.12 OTG_HS control and status registers

By reading from and writing to the control and status registers (CSRs) through the AHB slave interface, the application controls the OTG_HS controller. These registers are 32 bits wide, and the addresses are 32-bit block aligned. The OTG_HS registers must be accessed by words (32 bits). CSRs are classified as follows:

* Core global registers
* Host-mode registers
* Host global registers
* Host port CSRs
* Host channel-specific registers
* Device-mode registers
* Device global registers
* Device endpoint-specific registers
* Power and clock-gating registers
* Data FIFO (DFIFO) access registers

Only the Core global, Power and clock-gating, Data FIFO access, and host port control and status registers can be accessed in both host and peripheral modes. When the OTG_HS controller is operating in one mode, either peripheral or host, the application must not access registers from the other mode. If an illegal access occurs, a mode mismatch interrupt is generated and reflected in the Core interrupt register (MMIS bit in the OTG_HS_GINTSTS register). When the core switches from one mode to the other, the registers in the new mode of operation must be reprogrammed as they would be after a power-on reset.

### 35.12.1 CSR memory map

The host and peripheral mode registers occupy different addresses. All registers are implemented in the AHB clock domain.

1402/1749
RM0090 Rev 18
ST

--- Page 38 ---

RM0090
USB on-the-go high-speed (OTG_HS)

Figure 415. CSR memory map

0000h
Core global CSRs (1 Kbyte)
0400h
Host mode CSRs (1 Kbyte)
0800h
Device mode CSRs (1.5 Kbyte)
0E00h
Power and clock gating CSRs (0.5 Kbyte)
1000h
Device EP 0/Host channel 0 FIFO (4 Kbyte)
2000h
Device EP1/Host channel 1 FIFO (4 Kbyte)
3000h
DFIFO push/pop to this region
Device EP (x – 1)(1)/Host channel (x – 1)(1) FIFO (4 Kbyte)
Device EP x(1)/Host channel x(1) FIFO (4 Kbyte)
Reserved
2 0000h
Direct access to data FIFO RAM for debugging (128 Kbyte)
DFIFO debug read/write to this region
3 FFFFh

ai15615b

1. x = 5 in peripheral mode and x = 11 in host mode.

Global CSR map
These registers are available in both host and peripheral modes.

Table 208. Core global control and status registers (CSRs)

| Acronym | Address offset | Register name |
|---|---|---|
| OTG_HS_GOTGCTL | 0x000 | OTG_HS control and status register (OTG_HS_GOTGCTL) on page 1408 |
| OTG_HS_GOTGINT | 0x004 | OTG_HS interrupt register (OTG_HS_GOTGINT) on page 1409 |
| OTG_HS_GAHBCFG | 0x008 | OTG_HS AHB configuration register (OTG_HS_GAHBCFG) on page 1411 |
| OTG_HS_GUSBCFG | 0x00C | OTG_HS USB configuration register (OTG_HS_GUSBCFG) on page 1412 |
| OTG_HS_GRSTCTL | 0x010 | OTG_HS reset register (OTG_HS_GRSTCTL) on page 1415 |
| OTG_HS_GINTSTS | 0x014 | OTG_HS core interrupt register (OTG_HS_GINTSTS) on page 1418 |
| OTG_HS_GINTMSK | 0x018 | OTG_HS interrupt mask register (OTG_HS_GINTMSK) on page 1422 |


ST
RM0090 Rev 18
1403/1749

--- Page 39 ---

USB on-the-go high-speed (OTG_HS)
RM0090

**Table 208. Core global control and status registers (CSRs) (continued)**

| Acronym | Address offset | Register name |
|---|---|---|
| OTG_HS_GRXSTSR | 0x01C | OTG_HS Receive status debug read/OTG status read and pop registers (OTG_HS_GRXSTSR/OTG_HS_GRXSTSP) on page 1425 |
| OTG_HS_GRXSTSP | 0x020 |  |
| OTG_HS_GRXFSIZ | 0x024 | OTG_HS Receive FIFO size register (OTG_HS_GRXFSIZ) on page 1426 |
| OTG_HS_GNPTXFSIZ/OTG_HS_TX0FSIZ | 0x028 | OTG_HS nonperiodic transmit FIFO size/Endpoint 0 transmit FIFO size register (OTG_HS_GNPTXFSIZ/OTG_HS_TX0FSIZ) on page 1427 |
| OTG_HS_GNPTXSTS | 0x02C | OTG_HS nonperiodic transmit FIFO/queue status register (OTG_HS_GNPTXSTS) on page 1427 |
| OTG_HS_GCCFG | 0x038 | OTG_HS general core configuration register (OTG_HS_GCCFG) on page 1428 |
| OTG_HS_CID | 0x03C | OTG_HS core ID register (OTG_HS_CID) on page 1429 |
| OTG_HS_HPTXFSIZ | 0x100 | OTG_HS Host periodic transmit FIFO size register (OTG_HS_HPTXFSIZ) on page 1429 |
| OTG_HS_DIEPTXFx | 0x104 | OTG_HS device IN endpoint transmit FIFO size register (OTG_HS_DIEPTXFx) (x = 1..5, where x is the FIFO_number) on page 1430 |
| 0x108 |  |  |
| ... |  |  |
| 0x114 |  |  |


**Host-mode CSR map**

These registers must be programmed every time the core changes to host mode.

**Table 209. Host-mode control and status registers (CSRs)**

| Acronym | Offset address | Register name |
|---|---|---|
| OTG_HS_HCFG | 0x400 | OTG_HS host configuration register (OTG_HS_HCFG) on page 1430 |
| OTG_HS_HFIR | 0x404 | OTG_HS Host frame interval register (OTG_HS_HFIR) on page 1432 |
| OTG_HS_HFNUM | 0x408 | OTG_HS host frame number/frame time remaining register (OTG_HS_HFNUM) on page 1432 |
| OTG_HS_HPTXSTS | 0x410 | OTG_HS Host periodic transmit FIFO/queue status register (OTG_HS_HPTXSTS) on page 1433 |
| OTG_HS_HAINT | 0x414 | OTG_HS Host all channels interrupt register (OTG_HS_HAINT) on page 1434 |
| OTG_HS_HAINTMSK | 0x418 | OTG_HS host all channels interrupt mask register (OTG_HS_HAINTMSK) on page 1434 |
| OTG_HS_HPRT | 0x440 | OTG_HS host port control and status register (OTG_HS_HPRT) on page 1435 |


1404/1749
RM0090 Rev 18

--- Page 40 ---

RM0090
USB on-the-go high-speed (OTG_HS)

**Table 209. Host-mode control and status registers (CSRs) (continued)**

| Acronym | Offset address | Register name |
|---|---|---|
| OTG_HS_HCCHARx | 0x5000x520...0x660 | OTG_HS host channel-x characteristics register (OTG_HS_HCCHARx) (x = 0..11, where x = Channel_number) on page 1437 |
| OTG_HS_HCSPLTx | 0x504 | OTG_HS host channel-x split control register (OTG_HS_HCSPLTx) (x = 0..11, where x = Channel_number) on page 1439 |
| OTG_HS_HCINTx | 0x508 | OTG_HS host channel-x interrupt register (OTG_HS_HCINTx) (x = 0..11, where x = Channel_number) on page 1440 |
| OTG_HS_HCINTMSKx | 0x50C | OTG_HS host channel-x interrupt mask register (OTG_HS_HCINTMSKx) (x = 0..11, where x = Channel_number) on page 1441 |
| OTG_HS_HCTSIZx | 0x510 | OTG_HS host channel-x transfer size register (OTG_HS_HCTSIZx) (x = 0..11, where x = Channel_number) on page 1442 |
| OTG_HS_HCDMAX | 0x514 | OTG_HS host channel-x DMA address register (OTG_HS_HCDMAX) (x = 0..11, where x = Channel_number) on page 1443 |


**Device-mode CSR map**

These registers must be programmed every time the core changes to peripheral mode.

**Table 210. Device-mode control and status registers**

| Acronym | Offset address | Register name |
|---|---|---|
| OTG_HS_DCFG | 0x800 | OTG_HS device configuration register (OTG_HS_DCFG) on page 1443 |
| OTG_HS_DCTL | 0x804 | OTG_HS device control register (OTG_HS_DCTL) on page 1445 |
| OTG_HS_DSTS | 0x808 | OTG_HS device status register (OTG_HS_DSTS) on page 1447 |
| OTG_HS_DIEPMSK | 0x810 | OTG_HS device IN endpoint common interrupt mask register (OTG_HS_DIEPMSK) on page 1448 |
| OTG_HS_DOEPMSK | 0x814 | OTG_HS device OUT endpoint common interrupt mask register (OTG_HS_DOEPMSK) on page 1449 |
| OTG_HS_DAINT | 0x818 | OTG_HS device all endpoints interrupt register (OTG_HS_DAINT) on page 1450 |
| OTG_HS_DAINTMSK | 0x81C | OTG_HS all endpoints interrupt mask register (OTG_HS_DAINTMSK) on page 1451 |
| OTG_HS_DVBUSDIS | 0x828 | OTG_HS device VBUS discharge time register (OTG_HS_DVBUSDIS) on page 1451 |
| OTG_HS_DVBUSPULSE | 0x82C | OTG_HS device VBUS pulsing time register (OTG_HS_DVBUSPULSE) on page 1452 |
| OTG_HS_DTHRCTL | 0x830 | OTG_HS Device threshold control register (OTG_HS_DTHRCTL) on page 1453 |


ST
RM0090 Rev 18
1405/1749

--- Page 41 ---

USB on-the-go high-speed (OTG_HS)
RM0090

Table 210. Device-mode control and status registers (continued)

| Acronym | Offset address | Register name |
|---|---|---|
| OTG_HS_DIEPEMPMSK | 0x834 | OTG_HS device IN endpoint FIFO empty interrupt mask register (OTG_HS_DIEPEMPMSK) on page 1454 |
| OTG_HS_DEACHINT | 0x838 | OTG_HS device each endpoint interrupt register (OTG_HS_DEACHINT) on page 1454 |
| OTG_HS_DEACHINTMSK | 0x83C | OTG_HS device each endpoint interrupt register mask (OTG_HS_DEACHINTMSK) on page 1455 |
| OTG_HS_DIEPEACHMSK1 | 0x844 | OTG_HS device each in endpoint-1 interrupt register (OTG_HS_DIEPEACHMSK1) on page 1455 |
| OTG_HS_DOEPEACHMSK1 | 0x884 | OTG_HS device each OUT endpoint-1 interrupt register (OTG_HS_DOEPEACHMSK1) on page 1456 |
| OTG_HS_DIEPCTLx | 0x9000x920...0x9A0 | OTG device endpoint-x control register (OTG_HS_DIEPCTLx) (x = 0..5, where x = Endpoint_number) on page 1457 |
| OTG_HS_DIEPINTx | 0x908 | OTG_HS device endpoint-x interrupt register (OTG_HS_DIEPINTx) (x = 0..5, where x = Endpoint_number) on page 1464 |
| OTG_HS_DIEPTSIZ0 | 0x910 | OTG_HS device IN endpoint 0 transfer size register (OTG_HS_DIEPTSIZ0) on page 1467 |
| OTG_HS_DIEPDMAX/OTG_HS_DOEPDMAX | 0x914/0xB14 | OTG_HS device endpoint-x DMA address register (OTG_HS_DIEPDMAX / OTG_HS_DOEPDMAX) (x = 0..5, where x = Endpoint_number) on page 1471 |
| OTG_HS_DTXFSTSx | 0x918 | OTG_HS device IN endpoint transmit FIFO status register (OTG_HS_DTXFSTSx) (x = 0..5, where x = Endpoint_number) on page 1470 |
| OTG_HS_DIEPTSIZx | 0x9300x950...0x9B0 | OTG_HS device endpoint-x transfer size register (OTG_HS_DIEPTSIZx) (x = 1..5, where x = Endpoint_number) on page 1469 |
| OTG_HS_DOEPCTL0 | 0xB00 | OTG_HS device control OUT endpoint 0 control register (OTG_HS_DOEPCTL0) on page 1460 |
| OTG_HS_DOEPSIZ0 | 0xB10 | OTG_HS device OUT endpoint 0 transfer size register (OTG_HS_DOEPSIZ0) on page 1468 |
| OTG_HS_DOEPCTLx | 0xB200xB40...0xBA0 | OTG_HS device endpoint-x control register (OTG_HS_DOEPCTLx) (x = 1..5, where x = Endpoint_number) on page 1461 |


1406/1749
RM0090 Rev 18

--- Page 42 ---

RM0090
USB on-the-go high-speed (OTG_HS)

### Table 210. Device-mode control and status registers (continued)

| Acronym | Offset address | Register name |
|---|---|---|
| OTG_HS_DOEPINTx | 0xB080xB28...0xBA8 | OTG_HS device endpoint-x interrupt register(OTG_HS_DOEPINTx) (x = 0..5, where x = Endpoint_number) on page 1466 |
| 0xB300xB50...0xBB0 | OTG_HS device endpoint-x transfer size register(OTG_HS_DOEPTSIZx) (x = 1..5, where x = Endpoint_number) on page 1470 |  |


### Data FIFO (DFIFO) access register map

These registers, available in both host and peripheral modes, are used to read or write the FIFO space for a specific endpoint or a channel, in a given direction. If a host channel is of type IN, the FIFO can only be read on the channel. Similarly, if a host channel is of type OUT, the FIFO can only be written on the channel.

### Table 211. Data FIFO (DFIFO) access register map

| FIFO access register section | Address range | Access |
|---|---|---|
| Device IN Endpoint 0/Host OUT Channel 0: DFIFO Write AccessDevice OUT Endpoint 0/Host IN Channel 0: DFIFO Read Access | 0x1000–0x1FFC | wr |
| Device IN Endpoint 1/Host OUT Channel 1: DFIFO Write AccessDevice OUT Endpoint 1/Host IN Channel 1: DFIFO Read Access | 0x2000–0x2FFC | wr |
| ... |  |  |
| Device IN Endpoint x(1)/Host OUT Channel x(1): DFIFO Write AccessDevice OUT Endpoint x(1)/Host IN Channel x(1): DFIFO Read Access | 0xX000–0xXFFC | wr |


1. Where x is 5 in peripheral mode and 11 in host mode.

### Power and clock gating CSR map

There is a single register for power and clock gating. It is available in both host and peripheral modes.

### Table 212. Power and clock gating control and status registers

| Register name | Acronym | Offset address: 0xE00–0xFFF |
|---|---|---|
| Power and clock gating control register | OTG_HS_PCGCCTL | 0xE00-0xE04 |
| Reserved | - | 0xE05–0xFFF |



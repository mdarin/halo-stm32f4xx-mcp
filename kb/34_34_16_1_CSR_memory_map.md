### 34.16 OTG_FS control and status registers

By reading from and writing to the control and status registers (CSRs) through the AHB slave interface, the application controls the OTG_FS controller. These registers are 32 bits wide, and the addresses are 32-bit block aligned. The OTG_FS registers must be accessed by words (32 bits).

CSRs are classified as follows:

*   Core global registers
*   Host-mode registers
*   Host global registers
*   Host port CSRs
*   Host channel-specific registers
*   Device-mode registers
*   Device global registers
*   Device endpoint-specific registers
*   Power and clock-gating registers
*   Data FIFO (DFIFO) access registers

Only the Core global, Power and clock-gating, Data FIFO access, and host port control and status registers can be accessed in both host and device modes. When the OTG_FS controller is operating in one mode, either device or host, the application must not access registers from the other mode. If an illegal access occurs, a mode mismatch interrupt is generated and reflected in the Core interrupt register (MMIS bit in the OTG_FS_GINTSTS register). When the core switches from one mode to the other, the registers in the new mode of operation must be reprogrammed as they would be after a power-on reset.

ST
RM0090 Rev 18
1265/1749

--- Page 96 ---

USB on-the-go full-speed (OTG_FS)
RM0090

**34.16.1 CSR memory map**

The host and device mode registers occupy different addresses. All registers are implemented in the AHB clock domain.

**Figure 395. CSR memory map**

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

1. x = 3 in device mode and x = 7 in host mode.

**Global CSR map**

These registers are available in both host and device modes.

**Table 198. Core global control and status registers (CSRs)**

| Acronym | Address offset | Register name |
|---|---|---|
| OTG_FS_GOTGCTL | 0x000 | OTG_FS control and status register (OTG_FS_GOTGCTL) on page 1271 |
| OTG_FS_GOTGINT | 0x004 | OTG_FS interrupt register (OTG_FS_GOTGINT) on page 1272 |
| OTG_FS_GAHBCFG | 0x008 | OTG_FS AHB configuration register (OTG_FS_GAHBCFG) on page 1274 |
| OTG_FS_GUSBCFG | 0x00C | OTG_FS USB configuration register (OTG_FS_GUSBCFG) on page 1275 |
| OTG_FS_GRSTCTL | 0x010 | OTG_FS reset register (OTG_FS_GRSTCTL) on page 1277 |


1266/1749
RM0090 Rev 18
ST

--- Page 97 ---

RM0090
USB on-the-go full-speed (OTG_FS)

Table 198. Core global control and status registers (CSRs) (continued)

| Acronym | Address offset | Register name |
|---|---|---|
| OTG_FS_GINTSTS | 0x014 | OTG_FS core interrupt register (OTG_FS_GINTSTS) on page 1279 |
| OTG_FS_GINTMSK | 0x018 | OTG_FS interrupt mask register (OTG_FS_GINTMSK) on page 1283 |
| OTG_FS_GRXSTSR | 0x01C | OTG_FS Receive status debug read/OTG status read and pop registers (OTG_FS_GRXSTSR/OTG_FS_GRXSTSP) on page 1286 |
| OTG_FS_GRXSTSP | 0x020 |  |
| OTG_FS_GRXFSIZ | 0x024 | OTG_FS Receive FIFO size register (OTG_FS_GRXFSIZ) on page 1287 |
| OTG_FS_HNPTXFSIZ/OTG_FS_DIEPTXF0(1) | 0x028 | OTG_FS Host non-periodic transmit FIFO size register (OTG_FS_HNPTXFSIZ)/Endpoint 0 Transmit FIFO size (OTG_FS_DIEPTXF0) |
| OTG_FS_HNPTXSTS | 0x02C | OTG_FS non-periodic transmit FIFO/queue status register (OTG_FS_HNPTXSTS) on page 1288 |
| OTG_FS_GCCFG | 0x038 | OTG_FS general core configuration register (OTG_FS_GCCFG) on page 1289 |
| OTG_FS_CID | 0x03C | OTG_FS core ID register (OTG_FS_CID) on page 1290 |
| OTG_FS_HPTXFSIZ | 0x100 | OTG_FS Host periodic transmit FIFO size register (OTG_FS_HPTXFSIZ) on page 1291 |
| OTG_FS_DIEPTXFx | 0x104 | OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXFx) (x = 1..3, where x is the FIFO_number) on page 1291 |
| 0x108 |  |  |
| 0x10C |  |  |


1. The general rule is to use OTG_FS_HNPTXFSIZ for host mode and OTG_FS_DIEPTXF0 for device mode.

**Host-mode CSR map**

These registers must be programmed every time the core changes to host mode.

Table 199. Host-mode control and status registers (CSRs)

| Acronym | Offset address | Register name |
|---|---|---|
| OTG_FS_HCFG | 0x400 | OTG_FS Host configuration register (OTG_FS_HCFG) on page 1292 |
| OTG_FS_HFIR | 0x404 | OTG_FS Host frame interval register (OTG_FS_HFIR) on page 1292 |
| OTG_FS_HFNUM | 0x408 | OTG_FS Host frame number/frame time remaining register (OTG_FS_HFNUM) on page 1293 |
| OTG_FS_HPTXSTS | 0x410 | OTG_FS Host periodic transmit FIFO/queue status register (OTG_FS_HPTXSTS) on page 1293 |
| OTG_FS_HAINT | 0x414 | OTG_FS Host all channels interrupt register (OTG_FS_HAINT) on page 1294 |
| OTG_FS_HAINTMSK | 0x418 | OTG_FS Host all channels interrupt mask register (OTG_FS_HAINTMSK) on page 1295 |


ST
RM0090 Rev 18
1267/1749

--- Page 98 ---

USB on-the-go full-speed (OTG_FS)
RM0090

Table 199. Host-mode control and status registers (CSRs) (continued)

| Acronym | Offset address | Register name |
|---|---|---|
| OTG_FS_HPRT | 0x440 | OTG_FS Host port control and status register (OTG_FS_HPRT) on page 1295 |
| OTG_FS_HCCHARx | 0x5000x520...0x5E0 | OTG_FS Host channel-x characteristics register (OTG_FS_HCCHARx) (x = 0..7, where x = Channel_number) on page 1298 |
| OTG_FS_HCINTx | 0x508 | OTG_FS Host channel-x interrupt register (OTG_FS_HCINTx) (x = 0..7, where x = Channel_number) on page 1299 |
| OTG_FS_HCINTMSKx | 0x50C | OTG_FS Host channel-x interrupt mask register (OTG_FS_HCINTMSKx) (x = 0..7, where x = Channel_number) on page 1300 |
| OTG_FS_HCTSIZx | 0x510 | OTG_FS Host channel-x transfer size register (OTG_FS_HCTSIZx) (x = 0..7, where x = Channel_number) on page 1301 |


Device-mode CSR map

These registers must be programmed every time the core changes to device mode.

Table 200. Device-mode control and status registers

| Acronym | Offset address | Register name |
|---|---|---|
| OTG_FS_DCFG | 0x800 | OTG_FS device configuration register (OTG_FS_DCFG) on page 1302 |
| OTG_FS_DCTL | 0x804 | OTG_FS device control register (OTG_FS_DCTL) on page 1303 |
| OTG_FS_DSTS | 0x808 | OTG_FS device status register (OTG_FS_DSTS) on page 1304 |
| OTG_FS_DIEPMSK | 0x810 | OTG_FS device IN endpoint common interrupt mask register (OTG_FS_DIEPMSK) on page 1305 |
| OTG_FS_DOEPMSK | 0x814 | OTG_FS device OUT endpoint common interrupt mask register (OTG_FS_DOEPMSK) on page 1306 |
| OTG_FS_DAINT | 0x818 | OTG_FS device all endpoints interrupt register (OTG_FS_DAINT) on page 1307 |
| OTG_FS_DAINTMSK | 0x81C | OTG_FS all endpoints interrupt mask register (OTG_FS_DAINTMSK) on page 1308 |
| OTG_FS_DVBUSDIS | 0x828 | OTG_FS device VBUS discharge time register (OTG_FS_DVBUSDIS) on page 1308 |
| OTG_FS_DVBUSPULSE | 0x82C | OTG_FS device VBUS pulsing time register (OTG_FS_DVBUSPULSE) on page 1308 |
| OTG_FS_DIEPEMPMSK | 0x834 | OTG_FS device IN endpoint FIFO empty interrupt mask register: (OTG_FS_DIEPEMPMSK) on page 1309 |
| OTG_FS_DIEPCTL0 | 0x900 | OTG_FS device control IN endpoint 0 control register (OTG_FS_DIEPCTL0) on page 1309 |


1268/1749
RM0090 Rev 18

--- Page 99 ---

RM0090
USB on-the-go full-speed (OTG_FS)

Table 200. Device-mode control and status registers (continued)

| Acronym | Offset address | Register name |
|---|---|---|
| OTG_FS_DIEPCTLx | 0x9200x9400x960 | OTG device endpoint x control register (OTG_FS_DIEPCTLx) (x = 1..3, where x = Endpoint_number) on page 1311 |
| OTG_FS_DIEPINTx | 0x908 | OTG_FS device endpoint-x interrupt register (OTG_FS_DIEPINTx) (x = 0..3, where x = Endpoint_number) on page 1318 |
| OTG_FS_DIEPTSIZ0 | 0x910 | OTG_FS device IN endpoint 0 transfer size register (OTG_FS_DIEPTSIZ0) on page 1320 |
| OTG_FS_DTXFSTSx | 0x918 | OTG_FS device IN endpoint transmit FIFO status register (OTG_FS_DTXFSTSx) (x = 0..3, where x = Endpoint_number) on page 1324 |
| OTG_FS_DIEPTSIZx | 0x9300x9500x970 | OTG_FS device OUT endpoint-x transfer size register (OTG_FS_DOEPTSIZx) (x = 1..3, where x = Endpoint_number) on page 1324 |
| OTG_FS_DOEPCTL0 | 0xB00 | OTG_FS device control OUT endpoint 0 control register (OTG_FS_DOEPCTL0) on page 1314 |
| OTG_FS_DOEPCTLx | 0xB200xB400xB60 | OTG device endpoint x control register (OTG_FS_DIEPCTLx) (x = 1..3, where x = Endpoint_number) on page 1311 |
| OTG_FS_DOEPINTx | 0xB08 | OTG_FS device endpoint-x interrupt register (OTG_FS_DOEPINTx) (x = 0..3, where x = Endpoint_number) on page 1319 |
| OTG_FS_DOEPTSIZ0 | 0xB10 | OTG_FS device OUT endpoint 0 transfer size register (OTG_FS_DOEPTSIZ0) on page 1322 |
| OTG_FS_DOEPTSIZx | 0xB300xB500xB70 | OTG_FS device OUT endpoint-x transfer size register (OTG_FS_DOEPTSIZx) (x = 1..3, where x = Endpoint_number) on page 1324 |


**Data FIFO (DFIFO) access register map**

These registers, available in both host and device modes, are used to read or write the FIFO space for a specific endpoint or a channel, in a given direction. If a host channel is of type IN, the FIFO can only be read on the channel. Similarly, if a host channel is of type OUT, the FIFO can only be written on the channel.

Table 201. Data FIFO (DFIFO) access register map

| FIFO access register section | Address range | Access |
|---|---|---|
| Device IN Endpoint 0/Host OUT Channel 0: DFIFO Write AccessDevice OUT Endpoint 0/Host IN Channel 0: DFIFO Read Access | 0x1000–0x1FFC | wr |
| Device IN Endpoint 1/Host OUT Channel 1: DFIFO Write AccessDevice OUT Endpoint 1/Host IN Channel 1: DFIFO Read Access | 0x2000–0x2FFC | wr |


ST
RM0090 Rev 18
1269/1749

--- Page 100 ---

USB on-the-go full-speed (OTG_FS)
RM0090

Table 201. Data FIFO (DFIFO) access register map (continued)

| FIFO access register section | Address range | Access |
|---|---|---|
| ... |  |  |
| Device IN Endpoint x(1)/Host OUT Channel x(1): DFIFO Write AccessDevice OUT Endpoint x(1)/Host IN Channel x(1): DFIFO Read Access | 0xX000–0xXFFC | wr |


1. Where x is 3 in device mode and 7 in host mode.

**Power and clock gating CSR map**

There is a single register for power and clock gating. It is available in both host and device modes.

Table 202. Power and clock gating control and status registers

| Register name | Acronym | Offset address: 0xE00–0xFFF |
|---|---|---|
| Power and clock gating control register | OTG_FS_PCGCCTL | 0xE00-0xE04 |
| Reserved | - | 0xE05–0xFFF |


1270/1749
RM0090 Rev 18
STI

--- Page 101 ---

RM0090
USB on-the-go full-speed (OTG_FS)


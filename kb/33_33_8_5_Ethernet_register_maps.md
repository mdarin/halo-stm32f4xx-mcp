### 33.8.5 Ethernet register maps

*Table 195* gives the ETH register map and reset values.

**Table 195. Ethernet register map and reset values**

| Offset | Register | 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 0x00 | ETH_MACCR | Reserved | CSTF | reserved | WD | JD | Reserved | IFG | CSD | Reserved | FES | ROD | LM | DM | IPCO | RD | Reserved | APCS | BL | DC | TE | RE | Reserved |  |  |  |  |  |  |  |  |  |  |
| Reset value |  |  |  |  |  |  | 0 |  | 0 |  |  | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x04 | ETH_MACFFR | RA | Reserved | HPF | SAF | SAIF | PCF | BFD | PAM | DAIF | HM | HU | PM |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 0 |  |  |  |  |  |  |  |  |  |  |  |
| 0x08 | ETH_MACHTHR | HTH[31:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x0C | ETH_MACTLR | HTL[31:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x10 | ETH_MACMIAR | Reserved | PA | MR | RC | MW | MB |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 0 |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x14 | ETH_MACMIIDR | Reserved | MD |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 0 |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x18 | ETH_MACFCR | PT | Reserved | ZQPD | Reserved | PLT | UPFD | RFCE | TFCE | FCB/BPA |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x1C | ETH_MACVLANTR | Reserved | VLANTC | VLANTI |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x28 | ETH_MACRWUFFR | Frame filter reg0\Frame filter reg1\Frame filter reg2\Frame filter reg3\Frame filter reg4\...\Frame filter reg7 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


1236/1749
RM0090 Rev 18

--- Page 67 ---

RM0090
Ethernet (ETH): media access control (MAC) with DMA controller

Table 195. Ethernet register map and reset values (continued)

| Offset | Register | 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 0x2C | ETH_MACPMTCSR | WFFPR | Reserved | GU | Reserved | WFR | MPR | Reserved | WFE | MPE | PD |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 0 | Reserved | 0 | Reserved | 0 |  |  |  |  |  |  |
| 0x34 | ETH_MACDBGR | Reserved | TFF | TFNEGU | Reserved | TFWA | TFRS | MTP | MTFCS | MMTEA | Reserved | RFFL | Reserved | RFRCS | RFWRA | Reserved | MSFRWCS | MMRPEA |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value |  |  |  |  | 0 | Reserved | 0 |  |  |  |  |  |  | 0 | Reserved | 0 | Reserved | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x38 | ETH_MACSR |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | Reserved | TSTS | Reserved | MMCRS | MMCS | PMTS | Reserved |  |  |  |  |  |  |  |  |  |
| Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 0 | Reserved | 0 | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x3C | ETH_MACIMR |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | Reserved | TSTIM | Reserved | PMTIM | Reserved |  |  |  |  |  |  |  |  |  |  |  |
| Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 0 |  |  |  |  | 0 | Reserved |  |  |  |  |  |  |  |  |  |  |
| 0x40 | ETH_MACA0HR | MO | Reserved | MACA0H |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 1 | 0 | 1 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x44 | ETH_MACA0LR | MACA0L |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 1 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x48 | ETH_MACA1HR | AE | SA | MBC[6:0] | Reserved | MACA1H |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  | 1 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x4C | ETH_MACA1LR | MACA1L |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 1 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x50 | ETH_MACA2HR | AE | SA | MBC | Reserved | MACA2H |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  | 1 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x54 | ETH_MACA2LR | MACA2L |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 1 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x58 | ETH_MACA3HR | AE | SA | MBC | Reserved | MACA3H |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  | 1 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x5C | ETH_MACA3LR | MACA3L |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 1 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x100 | ETH_MMCCR | Reserved | MCFHP | MCP | MCF | ROR | CSR | CR |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 0 |  |  |  |  |  |  |  |  |
| 0x104 | ETH_MMCIR | Reserved | RGUFS | Reserved | RFAES | RFCEs | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 0 |  |  |  |  |  |  | 0 | Reserved |  |  |  |  |  |  |  |  |
| 0x108 | ETH_MMTIR | Reserved | TGFS | Reserved | TGFMSCS | TGFSCS | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value |  |  |  |  |  |  |  |  |  |  |  |  | 0 |  |  | 0 | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x10C | ETH_MMCIMR | Reserved | RGUJM | Reserved | RFAEM | RFCEM | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 0 |  |  |  |  |  |  | 0 | Reserved |  |  |  |  |  |  |  |  |


ST
RM0090 Rev 18
1237/1749

--- Page 68 ---

Ethernet (ETH): media access control (MAC) with DMA controller
RM0090

Table 195. Ethernet register map and reset values (continued)

| Offset | Register | 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 0x110 | ETH_MMCTIMR | Reserved | TGFM | Reserved | TGFMSCM | TGFSCM | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value |  |  |  |  |  |  |  |  |  |  | 0 |  |  |  | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x14C | ETH_MMCTGFS CCR | TGFSCC |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x150 | ETH_MMCTGF MSCCR | TGFMSCC |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x168 | ETH_MMCTGFC R | TGFC |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x194 | ETH_MMRFC ECR | RFCEC |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x198 | ETH_MMRFAE CR | RFAEC |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x1C4 | ETH_MMRGU FCR | RGUFC |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x700 | ETH_PTPSCR | Reserved | TSPFFMAE | TSCNT | TSSMRME | TSSME | TSSIPV4FE | TSSIPV6FE | TSSPTPOEFE | TSPPTPSV2E | TSSSR | TSSARFE | Reserved | TTSARU | TSITE | TSSTU | TSSTI | TSCFU | TSE |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 0 | 1 | 0 |  | 0 |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x704 | ETH_PTPSSIR | Reserved | STSSI |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 0 |  |  |  |  |
| 0x708 | ETH_PTPSHR | STS[31:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x70C | ETH_PTPSLR | STPNS | STSS |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x710 | ETH_PTPSHU R | TSUS |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x714 | ETH_PTPSLU R | TSUPNS | TSUSS |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x718 | ETH_PTPSAR | TSA |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x71C | ETH_PTPHR | TTSH |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x720 | ETH_PPTTLR | TTSL |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


1238/1749
RM0090 Rev 18
ST

--- Page 69 ---

RM0090
Ethernet (ETH): media access control (MAC) with DMA controller

Table 195. Ethernet register map and reset values (continued)

| Offset | Register | 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 0x728 | ETH_PTPTSSR | Reserved | TSTR | TSSO |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 0 |  |
| 0x72C | ETH_PTPPPSR | Reserved | PPS FREQ |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 0 |  |
| 0x1000 | ETH_DMABMR | Reserved | MB | AAB | FPM | USP | RDP | FB | PM | PBL | EDFE | DSL | DA | SR |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value |  |  |  |  | 0 | 1 | 0 | 1 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x1004 | ETH_DMATPDR | TPD |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x1008 | ETH_DMARPDR | RPD |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x100C | ETH_DMARDLAR | SRL |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x1010 | ETH_DMATDLAR | STL |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x1014 | ETH_DMASR | Reserved | TSTS | PMTS | MMCS | Reserved | EBS | TPS | RPS | NIS | AIS | ERS | FBES | Reserved | ETS | RWTS | RPSS | RBUS | RS | TUS | ROS | TJTS | TBUS | TPSS | TS |  |  |  |  |  |  |  |  |
| Reset value |  |  |  |  | 0 |  |  | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x1018 | ETH_DMAOMR | Reserved | DTCEFD | RSF | DFRF | Reserved | TSF | FTF | Reserved | TTC | ST | Reserved | FEF | FUGF | Reserved | RTC | OSF | SR | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value |  |  |  |  | 0 |  |  | 0 |  |  | 0 |  |  |  |  | 0 | Reserved | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x101C | ETH_DMAIER | Reserved | NISE | AISE | ERIE | FBEIE | Reserved | ETIE | RWTIE | RPSIE | RBUJIE | RIE | TUIE | ROIE | TJITE | TBUJIE | TPSIE | TIE |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 0 |  |  | 0 |  |  |  |  |  |
| 0x1020 | ETH_DMAMFB_OCR | Reserved | OFOC | MFA | OMFC | MFC |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value |  |  | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x1024 | ETH_DMARSWTR | Reserved | RSWTC |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 0 |  |  |  |  |  |  |  |  |
| 0x1048 | ETH_DMACHTDR | HTDAP |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x104C | ETH_DMACHRDR | HRDAP |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x1050 | ETH_DMACHTBAR | HTBAP |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x1054 | ETH_DMACHRBAR | HRBAP |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Refer to Section 2.3: Memory map for the register boundary addresses.

ST
RM0090 Rev 18
1239/1749

--- Page 70 ---

USB on-the-go full-speed (OTG_FS)
RM0090


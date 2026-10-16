### 34.16.6 OTG_FS register map

The table below gives the USB OTG register map and reset values.

**Table 205. OTG_FS register map and reset values**

| Offset | Register | 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 0x000 | OTG_FS_GOTGCTL | Reserved | BSVLD | ASVLD | DBCT | CIDSTS | Reserved | DHNPEN | HSHNPEN | HNPRQ | HNGSCS | Reserved | SRQ | SRQSCS |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value |  |  |  |  |  |  |  |  |  |  |  |  | 0 | 1 |  |  | 0 |  |  |  | 0 |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x004 | OTG_FS_GOTGINT | Reserved | DBCDNE | ADTOCHG | HNGDET | Reserved | HNSSCHG | SRSSCHG | Reserved | SEDET | Res. |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value |  |  |  |  |  |  |  |  |  |  |  |  | 0 |  |  | 0 |  |  |  | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x008 | OTG_FS_GAHBCFG | Reserved | PTXFELVL | TXFELVL | Reserved | GINTMSK |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 0 |  |  | 0 |  |  |  |  |  |  |  |
| 0x00C | OTG_FS_GUSBCFG | CTXPKT | FDMOD | FHMOD | Reserved | TRDT | HNPCA | SRPCAP | Reserved | PHYSEL | Reserved | TOCOL |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 0 | 1 | 0 | 1 | 0 |  |  | 0 |  |  |  |  |  |  |  |  |  |
| 0x010 | OTG_FS_GRSTCTL | AHBDL | Reserved | TXFNUM | TXFFLSH | RXFFLSH | Reserved | FCRST | HSRST | CSRST |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 1 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 0 |  |  |  |  |  |  |  |  |  |  |  |
| 0x014 | OTG_FS_GINTSTS | WKUINT | SRQINT | DISCINT | CIDSCHG | Reserved | PTXFE | HCINT | HPRTINT | Reserved | IPXFRINCOMPISOOUT | IISOIXFR | OEPINT | IEPINT | Reserved | EOPF | ISOODRP | ENUMDNE | USBST | USBSUSP | ESUSP | Reserved | GONAKEFF | GINAKEFF | NPTXFE | RXFLVL | SOF | OTGINT | MMIS | CMOD |  |  |  |
| Reset value | 0 |  | 1 | 0 |  | 0 |  | 0 |  | 0 | 1 | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x018 | OTG_FS_GINTMSK | WUIM | SRQIM | DISCINT | CIDSCHGM | Reserved | PTXFEM | HCIM | PRTIM | Reserved | IPXFRMISOIXFRM | IISOIXFRM | OEPINTM | IEPINTM | Reserved | EOPFM | ISOODRPM | ENUMDNEM | USBST | USBSUSPM | ESUSPM | Reserved | GONAKEFFM | GINAKEFFM | NPTXFEM | RXFLVLM | SOFM | OTGINT | MMISM | Reserved |  |  |  |
| Reset value | 0 |  | 0 |  | 0 |  | 0 |  | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x01C | OTG_FS_GRXSTSR (host mode) | Reserved | PKTSTS | DPID | BCNT | CHNUM |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value |  |  |  |  |  |  |  |  |  |  |  |  | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x01C | OTG_FS_GRXSTSR (Device mode) | Reserved | FRMNUM | PKTSTS | DPID | BCNT | EPNUM |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value |  |  |  |  |  |  |  |  |  |  |  |  | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


1326/1749
RM0090 Rev 18

--- Page 157 ---

RM0090
USB on-the-go full-speed (OTG_FS)

Table 205. OTG_FS register map and reset values (continued)

| Offset | Register | 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 0x020 | OTG_FS_GRXS TSR (host mode) | Reserved | PKTSTS | DPID | BCNT | CHNUM |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value |  |  |  |  |  |  |  |  |  |  |  | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  | OTG_FS_GRXS TSPR (Device mode) | Reserved | FRMNUM | PKTSTS | DPID | BCNT | EPNUM |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value |  |  |  |  |  |  |  |  | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x024 | OTG_FS_GRXF SIZ | Reserved | RXFD |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  | Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 0 | 1 | 0 |  |  |  |  |  |  |  |  |
| 0x028 | OTG_FS_HNPT XFSIZ/ OTG_FS_DIEPT XF0 | NPTXFD/TX0FD | NPTXFSA/TX0FSA |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 | 1 | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x02C | OTG_FS_HNPT XSTS | Res. | NPTXQTOP | NPTQXSAV | NPTXSAV |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 | 1 | 0 | 1 | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x038 | OTG_FS_GCCFG | Reserved | NOVBUSSSEN SOFOUTEN VBUSSEN VBUSASEN Reserved | .PWROWN | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value |  |  |  |  |  |  |  |  |  |  | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x03C | OTG_FS_CID | PRODUCT_ID |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 | 1 | 0 | 1 | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x100 | OTG_FS_HPTX FSIZ | PTXFSIZ | PTXSA |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 | 1 | 0 | 1 | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x104 | OTG_FS_DIEPT XF1 | INEPTXFD | INEPTXSA |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 | 1 | 0 | 1 | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x108 | OTG_FS_DIEPT XF2 | INEPTXFD | INEPTXSA |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 | 1 | 0 | 1 | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x10C | OTG_FS_DIEPT XF3 | INEPTXFD | INEPTXSA |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 | 1 | 0 | 1 | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x400 | OTG_FS_HCFG | Reserved | FSLS | FSLSPCS |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 0 |  |  |  |  |  |  |
| 0x404 | OTG_FS_HFIR | Reserved | FRIVL |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 1 | 0 | 1 | 0 | 1 | 0 | 1 | 0 |  |  |  |
| 0x408 | OTG_FS_HFNU M | FTREM | FRNUM |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 | 1 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


ST
RM0090 Rev 18
1327/1749

--- Page 158 ---

USB on-the-go full-speed (OTG_FS)
RM0090

Table 205. OTG_FS register map and reset values (continued)

| Offset | Register | 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 0x410 | OTG_FS_HPTXSTS | PTXQTOP | PTXQSAV | PTXSAVL |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 | Y |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x414 | OTG_FS_HAINT | Reserved | HAINT |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 0 |  |  |  |  |  |  |  |  |
| 0x418 | OTG_FS_HAINTMSK | Reserved | HAINTM |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 0 |  |  |  |  |  |  |  |  |
| 0x440 | OTG_FS_HPRT | Reserved | PSPD | PTCTL | PPWR | PLSTS | Reserved | PRST | PSUSP | PPRES | POCCHG | POCA | PENCHG | PENA | PCDET | PCSTS |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x500 | OTG_FS_HCCHAR0 | CHENA | CHDIS | CHPID | ODDFRM | DAD | MCNT | EPTYP | LSDEV | Reserved | EPDIR | EPNUM | MPSIZ |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x520 | OTG_FS_HCCHAR1 | CHENA | CHDIS | CHPID | ODDFRM | DAD | MCNT | EPTYP | LSDEV | Reserved | EPDIR | EPNUM | MPSIZ |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x540 | OTG_FS_HCCHAR2 | CHENA | CHDIS | CHPID | ODDFRM | DAD | MCNT | EPTYP | LSDEV | Reserved | EPDIR | EPNUM | MPSIZ |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x560 | OTG_FS_HCCHAR3 | CHENA | CHDIS | CHPID | ODDFRM | DAD | MCNT | EPTYP | LSDEV | Reserved | EPDIR | EPNUM | MPSIZ |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x580 | OTG_FS_HCCHAR4 | CHENA | CHDIS | CHPID | ODDFRM | DAD | MCNT | EPTYP | LSDEV | Reserved | EPDIR | EPNUM | MPSIZ |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x5A0 | OTG_FS_HCCHAR5 | CHENA | CHDIS | CHPID | ODDFRM | DAD | MCNT | EPTYP | LSDEV | Reserved | EPDIR | EPNUM | MPSIZ |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x5C0 | OTG_FS_HCCHAR6 | CHENA | CHDIS | CHPID | ODDFRM | DAD | MCNT | EPTYP | LSDEV | Reserved | EPDIR | EPNUM | MPSIZ |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x5E0 | OTG_FS_HCCHAR7 | CHENA | CHDIS | CHPID | ODDFRM | DAD | MCNT | EPTYP | LSDEV | Reserved | EPDIR | EPNUM | MPSIZ |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x508 | OTG_FS_HCINT0 | Reserved | DTERR | FRMOR | BBERR | TXERR | Reserved | ACK | NAK | STALL | Reserved | CHH | XFRC |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 0 |  |  |  |  |  |  |  |  |


1328/1749
RM0090 Rev 18

--- Page 159 ---

RM0090
USB on-the-go full-speed (OTG_FS)

Table 205. OTG_FS register map and reset values (continued)

| Offset | Register | 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 0x528 | OTG_FS_HCINT1 | Reserved | DTERR | FRMOR | BBERR | TXERR | Reserved | ACK | NAK | STALL | Reserved | CHH | XFRC |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 0 | Reserved | 0 | Reserved | 0 |  |  |  |  |  |  |  |
| 0x548 | OTG_FS_HCINT2 | Reserved | DTERR | FRMOR | BBERR | TXERR | Reserved | ACK | NAK | STALL | Reserved | CHH | XFRC |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 0 | Reserved | 0 | Reserved | 0 |  |  |  |  |  |  |  |
| 0x568 | OTG_FS_HCINT3 | Reserved | DTERR | FRMOR | BBERR | TXERR | Reserved | ACK | NAK | STALL | Reserved | CHH | XFRC |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 0 | Reserved | 0 | Reserved | 0 |  |  |  |  |  |  |  |
| 0x588 | OTG_FS_HCINT4 | Reserved | DTERR | FRMOR | BBERR | TXERR | Reserved | ACK | NAK | STALL | Reserved | CHH | XFRC |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 0 | Reserved | 0 | Reserved | 0 |  |  |  |  |  |  |  |
| 0x5A8 | OTG_FS_HCINT5 | Reserved | DTERR | FRMOR | BBERR | TXERR | Reserved | ACK | NAK | STALL | Reserved | CHH | XFRC |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 0 | Reserved | 0 | Reserved | 0 |  |  |  |  |  |  |  |
| 0x5C8 | OTG_FS_HCINT6 | Reserved | DTERR | FRMOR | BBERR | TXERR | Reserved | ACK | NAK | STALL | Reserved | CHH | XFRC |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 0 | Reserved | 0 | Reserved | 0 |  |  |  |  |  |  |  |
| 0x5E8 | OTG_FS_HCINT7 | Reserved | DTERR | FRMOR | BBERR | TXERR | Reserved | ACK | NAK | STALL | Reserved | CHH | XFRC |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 0 | Reserved | 0 | Reserved | 0 |  |  |  |  |  |  |  |
| 0x50C | OTG_FS_HCINTMSK0 | Reserved | DTERRM | FRMORM | BBERRM | TXERRM | Reserved | ACKM | NAKM | STALLM | Reserved | CHHM | XFRCM |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 0 | Reserved | 0 | Reserved | 0 |  |  |  |  |  |  |  |
| 0x52C | OTG_FS_HCINTMSK1 | Reserved | DTERRM | FRMORM | BBERRM | TXERRM | Reserved | ACKM | NAKM | STALLM | Reserved | CHHM | XFRCM |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 0 | Reserved | 0 | Reserved | 0 |  |  |  |  |  |  |  |
| 0x54C | OTG_FS_HCINTMSK2 | Reserved | DTERRM | FRMORM | BBERRM | TXERRM | Reserved | ACKM | NAKM | STALLM | Reserved | CHHM | XFRCM |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 0 | Reserved | 0 | Reserved | 0 |  |  |  |  |  |  |  |
| 0x56C | OTG_FS_HCINTMSK3 | Reserved | DTERRM | FRMORM | BBERRM | TXERRM | Reserved | ACKM | NAKM | STALLM | Reserved | CHHM | XFRCM |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 0 | Reserved | 0 | Reserved | 0 |  |  |  |  |  |  |  |
| 0x58C | OTG_FS_HCINTMSK4 | Reserved | DTERRM | FRMORM | BBERRM | TXERRM | Reserved | ACKM | NAKM | STALLM | Reserved | CHHM | XFRCM |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 0 | Reserved | 0 | Reserved | 0 |  |  |  |  |  |  |  |


ST
RM0090 Rev 18
1329/1749

--- Page 160 ---

USB on-the-go full-speed (OTG_FS)

RM0090

Table 205. OTG_FS register map and reset values (continued)

| Offset | Register | 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved | DTERRM | FRMORM | BBERRM | TXERRM | Reserved | ACKM | NAKM | STALLM | Reserved | CHHM | XFRCM |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x5AC | OTG_FS_HCINTMSK5 | Reserved | DTERRM | FRMORM | BBERRM | TXERRM | Reserved | ACKM | NAKM | STALLM | Reserved | CHHM | XFRCM |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 0 |  |  |  |  |  |  |  |  |  |  |  |
| 0x5CC | OTG_FS_HCINTMSK6 | Reserved | DTERRM | FRMORM | BBERRM | TXERRM | Reserved | ACKM | NAKM | STALLM | Reserved | CHHM | XFRCM |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 0 |  |  |  |  |  |  |  |  |  |  |  |
| 0x5EC | OTG_FS_HCINTMSK7 | Reserved | DTERRM | FRMORM | BBERRM | TXERRM | Reserved | ACKM | NAKM | STALLM | Reserved | CHHM | XFRCM |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 0 |  |  |  |  |  |  |  |  |  |  |  |
| 0x510 | OTG_FS_HCITSIZ0 | Reserved | DPID | PKTCNT | XFRSIZ |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | Reserved | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x530 | OTG_FS_HCITSIZ1 | Reserved | DPID | PKTCNT | XFRSIZ |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | Reserved | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x550 | OTG_FS_HCITSIZ2 | Reserved | DPID | PKTCNT | XFRSIZ |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | Reserved | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x570 | OTG_FS_HCITSIZ3 | Reserved | DPID | PKTCNT | XFRSIZ |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | Reserved | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x590 | OTG_FS_HCITSIZ4 | Reserved | DPID | PKTCNT | XFRSIZ |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | Reserved | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x5B0 | OTG_FS_HCITSIZ5 | Reserved | DPID | PKTCNT | XFRSIZ |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | Reserved | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x5D0 | OTG_FS_HCITSIZ6 | Reserved | DPID | PKTCNT | XFRSIZ |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | Reserved | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x5F0 | OTG_FS_HCITSIZ7 | Reserved | DPID | PKTCNT | XFRSIZ |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | Reserved | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x800 | OTG_FS_DCFG | Reserved | PFIVL | DAD | Reserved | NZLSOHSK | DSPD |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x804 | OTG_FS_DCTL | Reserved | POPRGDNE | CGONAK | SGONAK | CGINAK | SGINAK | TCTL | GONSTS | GINSTS | SDIS | RWUSIG |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


1330/1749

RM0090 Rev 18

ST

--- Page 161 ---

RM0090
USB on-the-go full-speed (OTG_FS)

Table 205. OTG_FS register map and reset values (continued)

| Offset | Register | 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 0x808 | OTG_FS_DSTS | Reserved | FNSOF | Reserved | EERR | ENUMSPD | SUSPSTS |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value |  |  |  |  |  |  |  |  |  |  |  |  | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x810 | OTG_FS_DIEPM SK | Reserved | NAK M | Reserved | INEP NEM | INEP NMM | ITTXFEMSK | TOM | EPDM | XFRCM |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 0 |  |  | 0 |  |  |  |  |  |  |  |  |  |
| 0x814 | OTG_FS_DOEP MSK | Reserved | NAKMSK | BERRM | Reserved | OUTPKTERRM | Reserved | STSPHSRXM | OTEPDM | STUPM | EPDM | XFRCM |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 0 |  |  | 0 | Reserved | 0 |  |  |  |  |  |  |  |  |  |  |  |
| 0x818 | OTG_FS_DAINT | OEPINT | IEPINT |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x81C | OTG_FS_DAINT MSK | OEP M | IEPM |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x828 | OTG_FS_DVBU SDIS | Reserved | VBUSDT |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 0 | 1 | 0 | 1 | 0 | 1 | 0 | 1 |  |
| 0x82C | OTG_FS_DVBU SPULSE | Reserved | DVBUSP |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 0 | 1 | 0 | 1 | 0 | 1 | 0 |  |  |
| 0x834 | OTG_FS_DIEPE MPMSK | Reserved | INEPTXFEM |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 0 |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x900 | OTG_FS_DIEPC TL0 | EPENA | EPDIS | Reserved | SNAK | CNAK | TXFNUM | STALL | Reserved | EMPTY P | NAKSTS | Reserved | USBAEP | Reserved | MPSI Z |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  | 0 | 1 |  |  |  |  |  |  |  |  |  |  | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x918 | TG_FS_DTXFST S0 | Reserved | INEPTFSAV |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 0 | 1 | 0 |  |  |  |  |  |  |  |  |  |  |
| 0x920 | OTG_FS_DIEPC TL1 | EPENA | EPDIS | SODDFRM/SD1PID | SDPID/SEVNRFM | SNAK | CNAK | TXFNUM | STALL | Reserved | EPTYP | NAKSTS | EONUM/DPID | USBAEP | Reserved | MPSIZ |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x938 | TG_FS_DTXFST S1 | Reserved | INEPTFSAV |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 0 | 1 | 0 |  |  |  |  |  |  |  |  |  |  |


RM0090 Rev 18
1331/1749

--- Page 162 ---

USB on-the-go full-speed (OTG_FS)
RM0090

Table 205. OTG_FS register map and reset values (continued)

| Offset | Register | 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 0x940 | OTG_FS_DIEPC TL2 | EPENA | EPDIS | SODDFRM | SD0PID/SEVNF RM | SNAK | CNAK | TXFNUM | STALL | Reserved | EPTYP | NAKSTS | EONUM/DPID | USBBAEP | Reserved | MPSIZ |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x958 | TG_FS_DTXFST S2 | Reserved | INEPTFSAV |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 0 | 1 | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x960 | OTG_FS_DIEPC TL3 | EPENA | EPDIS | SODDFRM | SD0PID/SEVNF RM | SNAK | CNAK | TXFNUM | STALL | Reserved | EPTYP | NAKSTS | EONUM/DPID | USBBAEP | Reserved | MPSIZ |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x978 | TG_FS_DTXFST S3 | Reserved | INEPTFSAV |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 0 | 1 | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0xB00 | OTG_FS_DOEP CTL0 | EPENA | EPDIS | Reserved | SNAK | CNAK | Reserved | STALL | SNPM | EPTY p | NAKSTS | Reserved | USBBAEP | Reserved | MPSI Z |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  | 0 |  |  | 0 | 1 |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 0 |  |  |  |  |  |  |  |  |  |  |
| 0xB20 | OTG_FS_DOEP CTL1 | EPENA | EPDIS | SODDFRM | SD0PID/SEVNF RM | SNAK | CNAK | Reserved | STALL | SNPM | EPTYP | NAKSTS | EONUM/DPID | USBBAEP | Reserved | MPSIZ |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0xB40 | OTG_FS_DOEP CTL2 | EPENA | EPDIS | SODDFRM | SD0PID/SEVNF RM | SNAK | CNAK | Reserved | STALL | SNPM | EPTYP | NAKSTS | EONUM/DPID | USBBAEP | Reserved | MPSIZ |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0xB60 | OTG_FS_DOEP CTL3 | EPENA | EPDIS | SODDFRM | SD0PID/SEVNF RM | SNAK | CNAK | Reserved | STALL | SNPM | EPTYP | NAKSTS | EONUM/DPID | USBBAEP | Reserved | MPSIZ |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x908 | OTG_FS_DIEPI NT0 | Reserved | NAK | Reserved | PKTDRPSTS | Reserved | TXFE | INEPNE | INEPNM | ITTXFE | TOC | Reserved | EPDISD | XFRC |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 0 |  |  | 1 | 0 |  |  |  |  |  |  |  |  |  |  |  |  |


1332/1749
RM0090 Rev 18

--- Page 163 ---

RM0090
USB on-the-go full-speed (OTG_FS)

Table 205. OTG_FS register map and reset values (continued)

| Offset | Register | 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 0x928 | OTG_FS_DIEPINT1 | Reserved | NAK | Reserved | PKTDRPSTS | Reserved | TXFE | INEPNE | ITTXFE | TOC | Reserved | EPDISD | XFRC |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 0 |  |  | 1 | 0 |  |  |  |  |  |  |  |  |  |
| 0x948 | OTG_FS_DIEPINT2 | Reserved | NAK | Reserved | PKTDRPSTS | Reserved | TXFE | INEPNE | ITTXFE | TOC | Reserved | EPDISD | XFRC |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 0 |  |  | 1 | 0 |  |  |  |  |  |  |  |  |  |
| 0x968 | OTG_FS_DIEPINT3 | Reserved | NAK | Reserved | PKTDRPSTS | Reserved | TXFE | INEPNE | ITTXFE | TOC | Reserved | EPDISD | XFRC |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 0 |  |  | 1 | 0 |  |  |  |  |  |  |  |  |  |
| 0xB08 | OTG_FS_DOEPINT0 | Reserved | NAK | BERR | Reserved | OUTPKTERR | Reserved | STSPHSRX | OTEPDIS | STUP | Reserved | EPDISD | XFRC |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 0 |  |  | 0 |  |  |  |  |  |  |  |  |  |  |
| 0xB28 | OTG_FS_DOEPINT1 | Reserved | NAK | BERR | Reserved | OUTPKTERR | Reserved | STSPHSRX | OTEPDIS | STUP | Reserved | EPDISD | XFRC |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 0 |  |  | 0 |  |  |  |  |  |  |  |  |  |  |
| 0xB48 | OTG_FS_DOEPINT2 | Reserved | NAK | BERR | Reserved | OUTPKTERR | Reserved | STSPHSRX | OTEPDIS | STUP | Reserved | EPDISD | XFRC |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 0 |  |  | 0 |  |  |  |  |  |  |  |  |  |  |
| 0xB68 | OTG_FS_DOEPINT3 | Reserved | NAK | BERR | Reserved | OUTPKTERR | Reserved | STSPHSRX | OTEPDIS | STUP | Reserved | EPDISD | XFRC |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 0 |  |  | 0 |  |  |  |  |  |  |  |  |  |  |
| 0x910 | OTG_FS_DIEPTSIZ0 | Reserved | PKTCNT | Reserved | XFRSIZ |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 0 0 |  |  |  |  |  |  | 0 0 0 0 0 0 |  |  |  |  |  |  |  |  |  |  |
| 0x930 | OTG_FS_DIEPTSIZ1 | Reserved | PKTCNT | XFRSIZ |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value |  |  |  |  |  | 0 0 0 0 0 0 0 0 0 0 | 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x950 | OTG_FS_DIEPTSIZ2 | Reserved | PKTCNT | XFRSIZ |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value |  |  |  |  |  | 0 0 0 0 0 0 0 0 0 0 | 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x970 | OTG_FS_DIEPTSIZ3 | Reserved | PKTCNT | XFRSIZ |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value |  |  |  |  |  | 0 0 0 0 0 0 0 0 0 0 | 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


ST
RM0090 Rev 18
1333/1749

--- Page 164 ---

USB on-the-go full-speed (OTG_FS)

RM0090

Table 205. OTG_FS register map and reset values (continued)

| Offset | Register | 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 0xB10 | OTG_FS_DOEP TSIZ0 | Reserved | STUP CNT | Reserved | PKTCNT | Reserved | XFRSIZ |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value |  |  | 0 0 |  |  |  |  |  |  |  |  |  |  |  | 0 |  |  |  |  |  |  |  |  |  |  |  | 0 0 0 0 0 0 |  |  |  |  |  |  |
| 0xB30 | OTG_FS_DOEP TSIZ1 | Reserved | RXDPID/ STUPCNT | PKTCNT | XFRSIZ |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value |  |  | 0 0 | 0 0 0 0 0 0 0 0 0 0 0 | 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0xB50 | OTG_FS_DOEP TSIZ2 | Reserved | RXDPID/ STUPCNT | PKTCNT | XFRSIZ |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value |  |  | 0 0 | 0 0 0 0 0 0 0 0 0 0 0 | 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0xB70 | OTG_FS_DOEP TSIZ3 | Reserved | RXDPID/ STUPCNT | PKTCNT | XFRSIZ |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value |  |  | 0 0 | 0 0 0 0 0 0 0 0 0 0 0 | 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0xE00 | OTG_FS_PCGC CTL | Reserved | PHYSUSP | Reserved | GATEHCLK | STPPCLK |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Refer to Section 2.3: Memory map for the register boundary addresses.

1334/1749 RM0090 Rev 18

--- Page 165 ---

RM0090
USB on-the-go full-speed (OTG_FS)


### 35.12.6 OTG_HS register map

The table below gives the USB OTG register map and reset values.

**Table 215. OTG_HS register map and reset values**

| Offset | Register | 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 0x000 | OTG_HS_GOTGCTL | Reserved | BSVLD | ASVLD | DBCT | CIDSTS | Reserved | DHNPEN | HSHNPEN | HNPRQ | HNGSCS | Reserved | SRQ | SROSCS |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 0 | 1 |  |  |  |  | 0 |  |  |  |  |  | 0 |  |  |  |  |
| 0x004 | OTG_HS_GOTGINT | Reserved | DBCDNE | ADTOCHG | HNGDET | Reserved | HNSSCHG | SRSSCHG | Reserved | SEDET | Res. |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 0 |  |  |  |  | 0 |  |  |  |  | 0 |  |  |  |  |  |  |
| 0x008 | OTG_HS_GAHBCFG | Reserved | PTXFELVL | TXFELVL | Reserved | GINT |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 0 |  |  |  |  | 0 |  |  |  |


1472/1749
RM0090 Rev 18

--- Page 108 ---

RM0090
USB on-the-go high-speed (OTG_HS)

Table 215. OTG_HS register map and reset values (continued)

| Offset | Register | 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 0x00C | OTG_HS_G | SBCFG | CTXPKT | EFMOD | FHMOD | Reserved | ULPIPPD | PTCI | PCCI | TSDPS | ULPIEVBSI | ULPIEVT | ULPICSM | ULPPAR | ULPIFSLS | Reserved | PHYLPCS | Reserved | TRD | 0 | 1 | 0 | 1 | 0 | 1 | Reserved | SRPCAP | HNP_CAP | PHYSel | Reserved | TOCAL |  |  |
| 0x010 | OTG_HS_G | STCTL | AHBDL | DMARC | Reset value | Reserved | TXFN | NUM | TXFFLSH | RXFLLSH | Reserved | FCRST | HSRST | CSRST |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x014 | OTG_HS_GIN | TSTS | WKNINT | SROINT | DISINT | CIDSCHG | Reserved | PTXFE | HCINT | HPTINT | Reserved | DATAFSUP | IPXFR/INCOMPISOUT | IPXFRM/ISOXFRM | IISOIXFR | OEPINT | IEPINT | Reserved | EOFPM | ISOODRP | ENUMDNE | USBSTP | USBSUSP | ESUSP | Reserved | GONAKEFF | GNAK | NPTXFE | RXFVL | SOFM | OTGINT | NMIS | CMOD |
| 0x018 | OTG_HS_GIN | TMASK | WUIM | SROIM | DISCINT | CIDSCHG | Reserved | PTXFEM | HCIM | PRTIM | Reserved | FSUSP | IPXFRM/ISOXFRM | IISOIXFR | OEPINT | IEPINT | Reserved | EOFPM | ISOODRP | ENUMDNE | USBSTP | USBSUSP | ESUSP | Reserved | GONAKEFF | GNAK | NPTXFE | RXFVL | SOFM | OTGINT | NMIS | Reserved |  |
| Offset | Register | 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x028 | OTG_HS_GNPTXFSIZ(host mode) | 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | TX0FD |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x02C | OTG_HS_GNPTXSTS | 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Res. | NPTXQTOP NPTQXSAV NPTXSAV |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x030 | OTG_HS_G12CCTL | 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| BSTYNERWReservedI2CDATASE0I2CDEVADDRReservedACKI2CEN | ADDR |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| REGADDR | RWDATA |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x038 | OTG_HS_GCCFG | 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| NOVBUSSENSSOFOUTENVBUSBSENVBUSASEN12CPADEN.PWDOWN | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x03C | OTG_HS_CIDPRODUCT_ID | 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | PTXFD PTXSA |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x040 | OTG_HS_HPTXFSIZ | 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | PTXFD INEPTXSA |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x044 | OTG_HS_HPTPTXF1 | 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | PTXFD INEPTXSA |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x048 | OTG_HS_HPTPTXF2 | 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | PTXFD INEPTXSA |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x04C | OTG_HS_HPTPTXF3 | 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | PTXFD INEPTXSA |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x050 | OTG_HS_HPTPTXF4 | 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | PTXFD INEPTXSA |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x054 | OTG_HS_HCFG | 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| FSLSFSLSPCS | 0 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x058 | OTG_HS_HFIR | 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| FRIVL | 1 1 1 0 1 0 1 0 0 1 1 0 0 0 0 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x05C | OTG_HS_HFNUM | 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | FTREM FRNUM |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x060 | OTG_HS_HPTXTSTS | 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | PTXQTOP PTXQSAV PTXFSAVL |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


1474/1749
RM0090 Rev 18

--- Page 110 ---

RM0090
USB on-the-go high-speed (OTG_HS)

Table 215. OTG_HS register map and reset values (continued)

| Offset | Register | 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 0x414 | OTG_HS_HAI NT | Reserved | HAINT |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x418 | OTG_HS_HAI NTMSK | Reserved | HAINTM |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x440 | OTG_HS_HPR T | Reserved | PSP D | PTCTL | PPWR | PLSTS | Reserved | PRST | PSUSP | PRES | POCA | PCONHNG | PENGA | PCDET | PCSTS |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x500 | OTG_HS_HC CHAR0 | CHENA | CHDIS | ODDFRM | DAD | MC | EPTYP | LSDEV | Reserved | EPDIR | EPNUM | MPSIZ |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x520 | OTG_HS_HC CHAR1 | CHENA | CHDIS | ODDFRM | DAD | MC | EPTYP | LSDEV | Reserved | EPDIR | EPNUM | MPSIZ |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x540 | OTG_HS_HC CHAR2 | CHENA | CHDIS | ODDFRM | DAD | MC | EPTYP | LSDEV | Reserved | EPDIR | EPNUM | MPSIZ |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x560 | OTG_HS_HC CHAR3 | CHENA | CHDIS | ODDFRM | DAD | MC | EPTYP | LSDEV | Reserved | EPDIR | EPNUM | MPSIZ |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x580 | OTG_HS_HC CHAR4 | CHENA | CHDIS | ODDFRM | DAD | MC | EPTYP | LSDEV | Reserved | EPDIR | EPNUM | MPSIZ |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x5A0 | OTG_HS_HC CHAR5 | CHENA | CHDIS | ODDFRM | DAD | MC | EPTYP | LSDEV | Reserved | EPDIR | EPNUM | MPSIZ |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x5C0 | OTG_HS_HC CHAR6 | CHENA | CHDIS | ODDFRM | DAD | MC | EPTYP | LSDEV | Reserved | EPDIR | EPNUM | MPSIZ |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x5E0 | OTG_HS_HC CHAR7 | CHENA | CHDIS | ODDFRM | DAD | MC | EPTYP | LSDEV | Reserved | EPDIR | EPNUM | MPSIZ |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x600 | OTG_HS_HC CHAR8 | CHENA | CHDIS | ODDFRM | DAD | MC | EPTYP | LSDEV | Reserved | EPDIR | EPNUM | MPSIZ |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


ST
RM0090 Rev 18
1475/1749

--- Page 111 ---

USB on-the-go high-speed (OTG_HS)
RM0090
1476/1749
RM0090 Rev 18

Table 215. OTG_HS register map and reset values (continued)

| Offset | Register | Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |  |
| 0x620 | OTG_HS_HCCHAR9 | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  | CHENA | CHDIS | ODDFRM | DAD | MC | EPTYP | LSDEV | Reserved | EPDIR | EPNUM | MPSIZ |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x640 | OTG_HS_HCCHAR10 | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  | CHENA | CHDIS | ODDFRM | DAD | MC | EPTYP | LSDEV | Reserved | EPDIR | EPNUM | MPSIZ |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x660 | OTG_HS_HCCHAR11 | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  | CHENA | CHDIS | ODDFRM | DAD | MC | EPTYP | LSDEV | Reserved | EPDIR | EPNUM | MPSIZ |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x504 | OTG_HS_HCTSIZ0 | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  | SPLITEN | Reserved | COMPLSPLIT | XACTPOS | HUBADDR | PRIDADDR |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x508 | OTG_HS_HCI0 | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  | DTERR | FRMOR | BBERR | TXERR | NYET | ACK | NAK | STALL | AHBERR | CHH | XFRC |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x524 | OTG_HS_HCTSIZ1 | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  | SPLITEN | Reserved | COMPLSPLIT | XACTPOS | HUBADDR | PRIDADDR |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x528 | OTG_HS_HCI1 | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  | DTERR | FRMOR | BBERR | TXERR | NYET | ACK | NAK | STALL | AHBERR | CHH | XFRC |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x544 | OTG_HS_HCTSIZ2 | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  | SPLITEN | Reserved | COMPLSPLIT | XACTPOS | HUBADDR | PRIDADDR |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x548 | OTG_HS_HCI2 | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  | DTERR | FRMOR | BBERR | TXERR | NYET | ACK | NAK | STALL | AHBERR | CHH | XFRC |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x564 | OTG_HS_HCTSIZ3 | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  | SPLITEN | Reserved | COMPLSPLIT | XACTPOS | HUBADDR | PRIDADDR |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x568 | OTG_HS_HCI3 | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  | DTERR | FRMOR | BBERR | TXERR | NYET | ACK | NAK | STALL | AHBERR | CHH | XFRC |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


--- Page 112 ---

RM0090
USB on-the-go high-speed (OTG_HS)

Table 215. OTG_HS register map and reset values (continued)

| Offset | Register | 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0 |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|
| OTG_HS_HCS | PLT4 | SPLIT | Reset value | Reserved | COMPLSPL | XACTPOS | HUBADDR | PRADDR |
| 0x584 | OTG_HS_HCI | NT4 | 0 | Reset value | Reserved | 0 |  |  |
| 0x588 | OTG_HS_HCI | NT4 | 0 | Reset value | Reserved | 0 |  |  |
| 0x544 | OTG_HS_HCS | PLT5 | SPLIT | Reset value | Reserved | 0 |  |  |
| 0x548 | OTG_HS_HCI | NT5 | 0 | Reset value | Reserved | 0 |  |  |
| 0x5C4 | OTG_HS_HCS | PLT6 | SPLIT | Reset value | Reserved | 0 |  |  |
| 0x5C8 | OTG_HS_HCI | NT6 | 0 | Reset value | Reserved | 0 |  |  |
| 0x5E4 | OTG_HS_HCS | PLT7 | SPLIT | Reset value | Reserved | 0 |  |  |
| 0x5E8 | OTG_HS_HCI | NT7 | 0 | Reset value | Reserved | 0 |  |  |
| 0x604 | OTG_HS_HCS | PLT8 | SPLIT | Reset value | Reserved | 0 |  |  |
| 0x608 | OTG_HS_HCI | NT8 | 0 | Reset value | Reserved | 0 |  |  |


RM0090 Rev 18
1477/1749

--- Page 113 ---

USB on-the-go high-speed (OTG_HS)
RM0090

Table 215. OTG_HS register map and reset values (continued)

| Offset | Register | 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
| 0x624 | OTG_HS_HCSPLT9 | SPLTEN | Reset value | Reserved | COMPLSPLT | XACTPOS | HUBADDR | PRADDR | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x628 | OTG_HS_HCINT9 | DTERR | Reset value | Reserved | FRMOR | BBER | TXERR | NYET | ACK | NAK | STALL | AHBERERR | CHHM | XFRCM | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x644 | OTG_HS_HCSPLT10 | SPLTEN | Reset value | Reserved | COMPLSPLT | XACTPOS | HUBADDR | PRADDR | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x648 | OTG_HS_HCINT10 | DTERR | Reset value | Reserved | FRMOR | BBER | TXERR | NYET | ACK | NAK | STALL | AHBERERR | CHHM | XFRCM | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x664 | OTG_HS_HCSPLT11 | SPLTEN | Reset value | Reserved | COMPLSPLT | XACTPOS | HUBADDR | PRADDR | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x668 | OTG_HS_HCINT11 | DTERR | Reset value | Reserved | FRMOR | BBER | TXERR | NYET | ACK | NAK | STALL | AHBERERR | CHHM | XFRCM | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x50C | OTG_HS_HCINTM0 | DTERR | Reset value | Reserved | FRMOR | BBER | TXERR | NYET | ACK | NAK | STALL | AHBERERR | CHHM | XFRCM | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x52C | OTG_HS_HCINTM1 | DTERR | Reset value | Reserved | FRMOR | BBER | TXERR | NYET | ACK | NAK | STALL | AHBERERR | CHHM | XFRCM | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x54C | OTG_HS_HCINTM2 | DTERR | Reset value | Reserved | FRMOR | BBER | TXERR | NYET | ACK | NAK | STALL | AHBERERR | CHHM | XFRCM | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x56C | OTG_HS_HCINTM3 | DTERR | Reset value | Reserved | FRMOR | BBER | TXERR | NYET | ACK | NAK | STALL | AHBERERR | CHHM | XFRCM | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x58C | OTG_HS_HCINTM4 | DTERR | Reset value | Reserved | FRMOR | BBER | TXERR | NYET | ACK | NAK | STALL | AHBERERR | CHHM | XFRCM | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


1478/1749
RM0090 Rev 18

--- Page 114 ---

RM0090
USB on-the-go high-speed (OTG_HS)

Table 215. OTG_HS register map and reset values (continued)

| Offset | Register | Bit positions (31 down to 0) |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
| 0x5AC | OTG_HS_HCI | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  | Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  | OTG_HS_HCI | DTERM | FRMORM | BBERM | TXERM | NVET | ACKM | NAKM | STALL | AHBERM | CHHM | XFRCM |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x5CC | NTMSK6 | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  | Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  | OTG_HS_HCI | DTERM | FRMORM | BBERM | TXERM | NVET | ACKM | NAKM | STALL | AHBERM | CHHM | XFRCM |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x5EC | NTMSK7 | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  | Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  | OTG_HS_HCI | DTERM | FRMORM | BBERM | TXERM | NVET | ACKM | NAKM | STALL | AHBERM | CHHM | XFRCM |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x60C | NTMSK8 | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  | Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  | OTG_HS_HCI | DTERM | FRMORM | BBERM | TXERM | NVET | ACKM | NAKM | STALL | AHBERM | CHHM | XFRCM |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x62C | NTMSK9 | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  | Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  | OTG_HS_HCI | DTERM | FRMORM | BBERM | TXERM | NVET | ACKM | NAKM | STALL | AHBERM | CHHM | XFRCM |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x64C | NTMSK10 | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  | Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  | OTG_HS_HCI | DTERM | FRMORM | BBERM | TXERM | NVET | ACKM | NAKM | STALL | AHBERM | CHHM | XFRCM |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x66C | NTMSK11 | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  | Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  | OTG_HS_HCI | DTERM | FRMORM | BBERM | TXERM | NVET | ACKM | NAKM | STALL | AHBERM | CHHM | XFRCM |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x510 | OTG_HS_HCT | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  | Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  | OTG_HS_HCT | PKTCNT | XFRSIZ |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x530 | OTG_HS_HCT | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  | Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  | OTG_HS_HCT | PKTCNT | XFRSIZ |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x550 | OTG_HS_HCT | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  | Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  | OTG_HS_HCT | PKTCNT | XFRSIZ |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x570 | OTG_HS_HCT | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  | Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  | OTG_HS_HCT | PKTCNT | DPID | XFRSIZ |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


RM0090 Rev 18
1479/1749

--- Page 115 ---

USB on-the-go high-speed (OTG_HS)

RM0090

Table 215. OTG_HS register map and reset values (continued)

| Offset | Register | 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 0x590 | OTG_HS_HCT SIZ4 | DOPING | DPID | PKTCNT | XFRSIZ |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x5B0 | OTG_HS_HCT SIZ5 | DOPING | DPID | PKTCNT | XFRSIZ |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x5D0 | OTG_HS_HCT SIZ6 | DOPING | DPID | PKTCNT | XFRSIZ |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x5F0 | OTG_HS_HCT SIZ7 | DOPING | DPID | PKTCNT | XFRSIZ |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x610 | OTG_HS_HCT SIZ8 | DOPING | DPID | PKTCNT | XFRSIZ |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x630 | OTG_HS_HCT SIZ9 | DOPING | DPID | PKTCNT | XFRSIZ |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x650 | OTG_HS_HCT SIZ10 | DOPING | DPID | PKTCNT | XFRSIZ |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x670 | OTG_HS_HCT SIZ11 | DOPING | DPID | PKTCNT | XFRSIZ |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x514 | OTG_HS_HC DMA0 | DMAADDR |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x524 | OTG_HS_HC DMA1 | DMAADDR |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x544 | OTG_HS_HC DMA2 | DMAADDR |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x564 | OTG_HS_HC DMA3 | DMAADDR |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x584 | OTG_HS_HC DMA4 | DMAADDR |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x5A4 | OTG_HS_HC DMA5 | DMAADDR |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


1480/1749

RM0090 Rev 18

ST

--- Page 116 ---

RM0090
USB on-the-go high-speed (OTG_HS)

Table 215. OTG_HS register map and reset values (continued)

| Offset | Register | 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 0x5C4 | OTG_HS_HC_DMA6 | DMAADDR |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Offset | Register | Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x82C | OTG_HS_DVBUSPULSE | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  | 0 | 1 | 0 | 1 | 0 | 1 | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x830 | OTG_HS_DTHRCTL | Reserved | AR PEN | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  | RXTHRLEN | RXTHREN | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  | TXHRLEN | ISOTHREN | NONISOTHREN |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x834 | OTG_HS_DIEPEPMSK | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  | INEPTXEM |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x838 | OTG_HS_DEACHINT | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x83C | OTG_HS_DEACHINTMSK | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x844 | OTG_HS_DIEPEACHMSK1 | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x844 | OTG_HS_DOEPEACHMSK | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x884 | OTG_HS_DOEPEACHMSK | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x900 | OTG_HS_DIEPCTL0 | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x918 | TG_FS_DTXFSTS0 | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  | INEPTFSAV |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x920 | OTG_HS_DIEPCTL1 | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x938 | TG_FS_DTXFSTS1 | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  | INEPTFSAV |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


--- Page 118 ---

RM0090
USB on-the-go high-speed (OTG_HS)

Table 215. OTG_HS register map and reset values (continued)

| Offset | Register | Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
| 0x940 | OTG_HS_DIEPCTL2 | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| EPENA | EPDIS | SODDFRM | SDPID/SEVNF RM | SNAK | CN AK | TXFNUM | Stall | Reserved | EPTYP | NAKSTS | ECNUM/DPID | USBAEP | MPSIZ |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x958 | TG_FS_DTXFSTS2 | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Offset | Register | 0xB20 | 0xB40 | 0xB60 | 0x908 | 0x928 | 0x948 | 0x968 | 0x988 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| OTG_HS_DO_EPCTL1 | Reset value | OTG_HS_DO_EPCTL2 | Reset value | OTG_HS_DO_EPCTL3 | Reset value | OTG_HS_DI_EPINT0 | Reset value | OTG_HS_DI_EPINT1 | Reset value | OTG_HS_DI_EPINT2 | Reset value | OTG_HS_DI_EPINT3 | Reset value | OTG_HS_DI_EPINT4 | Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 31 | EPEA | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 30 | EPDIS | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 29 | SODFRM | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 28 | SDPIDSEVNFRM | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 27 | SNAK | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 26 | CNAK | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 25 | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 24 | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 23 | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 22 | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 21 | STALL | 0 | STALL | 0 | STALL | 0 | Reserved | 0 | Reserved | 0 | Reserved | 0 | Reserved | 0 | Reserved | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 20 | SNPM | 0 | SNPM | 0 | SNPM | 0 | Reserved | 0 | Reserved | 0 | Reserved | 0 | Reserved | 0 | Reserved | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 19 | EPTYP | 0 | EPTYP | 0 | EPTYP | 0 | Reserved | 0 | Reserved | 0 | Reserved | 0 | Reserved | 0 | Reserved | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 18 | NAKSTS | 0 | NAKSTS | 0 | NAKSTS | 0 | Reserved | 0 | Reserved | 0 | Reserved | 0 | Reserved | 0 | Reserved | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 17 | EPNUM | 0 | EPNUM | 0 | EPNUM | 0 | Reserved | 0 | Reserved | 0 | Reserved | 0 | Reserved | 0 | Reserved | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 16 | EPNUM/EPID | 0 | EPNUM/EPID | 0 | EPNUM/EPID | 0 | Reserved | 0 | Reserved | 0 | Reserved | 0 | Reserved | 0 | Reserved | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 15 | USBBAEP | 0 | USBBAEP | 0 | USBBAEP | 0 | Reserved | 0 | Reserved | 0 | Reserved | 0 | Reserved | 0 | Reserved | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 14 | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 13 | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 12 | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 11 | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 10 | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 9 | TXFFOUDRN | 0 | TXFFOUDRN | 0 | TXFFOUDRN | 0 | TXFFOUDRN | 0 | TXFFOUDRN | 0 | TXFFOUDRN | 0 | TXFFOUDRN | 0 | TXFFOUDRN | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 8 | TXFE | 1 | TXFE | 1 | TXFE | 1 | TXFE | 1 | TXFE | 1 | TXFE | 1 | TXFE | 1 | TXFE | 1 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 7 | INEPNE | 0 | INEPNE | 0 | INEPNE | 0 | INEPNE | 0 | INEPNE | 0 | INEPNE | 0 | INEPNE | 0 | INEPNE | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 6 | INEPNM | 0 | INEPNM | 0 | INEPNM | 0 | INEPNM | 0 | INEPNM | 0 | INEPNM | 0 | INEPNM | 0 | INEPNM | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 5 | ITTXFE | 0 | ITTXFE | 0 | ITTXFE | 0 | ITTXFE | 0 | ITTXFE | 0 | ITTXFE | 0 | ITTXFE | 0 | ITTXFE | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 4 | TOC | 0 | TOC | 0 | TOC | 0 | TOC | 0 | TOC | 0 | TOC | 0 | TOC | 0 | TOC | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 3 | AHBERR | 0 | AHBERR | 0 | AHBERR | 0 | AHBERR | 0 | AHBERR | 0 | AHBERR | 0 | AHBERR | 0 | AHBERR | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 2 | EPDISD | 0 | EPDISD | 0 | EPDISD | 0 | EPDISD | 0 | EPDISD | 0 | EPDISD | 0 | EPDISD | 0 | EPDISD | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 1 | XFRC | 0 | XFRC | 0 | XFRC | 0 | XFRC | 0 | XFRC | 0 | XFRC | 0 | XFRC | 0 | XFRC | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0 | MPSIZ |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


1484/1749
RM0090 Rev 18

--- Page 120 ---

RM0090
USB on-the-go high-speed (OTG_HS)

Table 215. OTG_HS register map and reset values (continued)

| Offset | Register | 31 ... 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
| 0x9A8 | OTG_HS_DIEPINT5 | Reserved | Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  | 0 | 1 | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  | TXFIFODRN | TXFE | INEPNM | INEPM | ITTXFE | TOC | EPBRRE | EPDISD | XFRC |  |  |  |  |  |  |  |  |  |  |  | Reserved | PKTDRPSTS | NAK | 0 |  |  |  |  |  |  |
| 0x9C8 | OTG_HS_DIEPINT6 | Reserved | Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  | 0 | 1 | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  | TXFIFODRN | TXFE | INEPNM | INEPM | ITTXFE | TOC | EPBRRE | EPDISD | XFRC |  |  |  |  |  |  |  |  |  |  |  | Reserved | PKTDRPSTS | NAK | 0 |  |  |  |  |  |  |
| 0x9E8 | OTG_HS_DIEPINT7 | Reserved | Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  | 0 | 1 | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  | TXFIFODRN | TXFE | INEPNM | INEPM | ITTXFE | TOC | EPBRRE | EPDISD | XFRC |  |  |  |  |  |  |  |  |  |  |  | Reserved | PKTDRPSTS | NAK | 0 |  |  |  |  |  |  |
| 0xB08 | OTG_HS_DOEPINT0 | Reserved | Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  | 0 | 1 | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  | OUTPKTERR | Reserved | B2BSTUP | Reserved | OTEPDIS | STUP | AHBERR | EPDISD | XFRC |  |  |  |  |  |  |  |  |  |  |  | Reserved | NAK | NYET | 0 |  |  |  |  |  |  |
| 0xB28 | OTG_HS_DOEPINT1 | Reserved | Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  | OUTPKTERR | Reserved | B2BSTUP | Reserved | OTEPDIS | STUP | AHBERR | EPDISD | XFRC |  |  |  |  |  |  |  |  |  |  |  | Reserved | NAK | NYET | 0 |  |  |  |  |  |  |
| 0xB48 | OTG_HS_DOEPINT2 | Reserved | Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  | OUTPKTERR | Reserved | B2BSTUP | Reserved | OTEPDIS | STUP | AHBERR | EPDISD | XFRC |  |  |  |  |  |  |  |  |  |  |  | Reserved | NAK | NYET | 0 |  |  |  |  |  |  |
| 0xB68 | OTG_HS_DOEPINT3 | Reserved | Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  | OUTPKTERR | Reserved | B2BSTUP | Reserved | OTEPDIS | STUP | AHBERR | EPDISD | XFRC |  |  |  |  |  |  |  |  |  |  |  | Reserved | NAK | NYET | 0 |  |  |  |  |  |  |
| 0xB88 | OTG_HS_DOEPINT4 | Reserved | Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  | OUTPKTERR | Reserved | B2BSTUP | Reserved | OTEPDIS | STUP | AHBERR | EPDISD | XFRC |  |  |  |  |  |  |  |  |  |  |  | Reserved | NAK | NYET | 0 |  |  |  |  |  |  |
| 0xBA8 | OTG_HS_DOEPINT5 | Reserved | Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  | OUTPKTERR | Reserved | B2BSTUP | Reserved | OTEPDIS | STUP | AHBERR | EPDISD | XFRC |  |  |  |  |  |  |  |  |  |  |  | Reserved | NAK | NYET | 0 |  |  |  |  |  |  |


1485/1749
RM0090 Rev 18

--- Page 121 ---

USB on-the-go high-speed (OTG_HS)
RM0090

Table 215. OTG_HS register map and reset values (continued)

| Offset | Register | 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x8C8 | OTG_HS_DOEPINT6 | Reserved | 0 | NYET | NAK | BERR | 0 | OUTPKTERR | Reserved | 0 | B2BSTUP | Reserved | 0 | OTEPDIS | STUP | AHBERR | EPDISD | XFRC |
| 0xBE8 | OTG_HS_DOEPINT7 | Reserved | 0 | NYET | NAK | BERR | 0 | OUTPKTERR | Reserved | 0 | B2BSTUP | Reserved | 0 | OTEPDIS | STUP | AHBERR | EPDISD | XFRC |
| 0x910 | OTG_HS_DIEPTSIZ0 | Reserved | PKT CNT | 0 | Reserved | XFRSIZ |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x930 | OTG_HS_DIEPTSIZ1 | Reserved | MCN T | 0 | PKTCNT | XFRSIZ |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x934 | OTG_HS_DIEPDMA1 | DMABADDR |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x93C | OTG_HS_DIEPDMAB1 | Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x950 | OTG_HS_DIEPTSIZ2 | Reserved | MCN T | 0 | PKTCNT | XFRSIZ |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x954 | OTG_HS_DIEPDMA2 | DMABADDR |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x95C | OTG_HS_DIEPDMAB2 | Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x970 | OTG_HS_DIEPTSIZ3 | Reserved | MCN T | 0 | PKTCNT | XFRSIZ |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x974 | OTG_HS_DIEPDMA3 | DMABADDR |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x97C | OTG_HS_DIEPDMAB3 | Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0xB10 | OTG_HS_DOEPTSIZ0 | Reserved | PCN T | 0 | PKTCNT | XFRSIZ |  |  |  |  |  |  |  |  |  |  |  |  |
| 0xB30 | OTG_HS_DOEPTSIZ1 | Reserved | RXPID/ STUPCNT | 0 | PKTCNT | XFRSIZ |  |  |  |  |  |  |  |  |  |  |  |  |
| 0xB34 | OTG_HS_DOEPDMA1 | DMABADDR |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0xB49 | OTG_HS_DOEPDMAB1 | Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


--- Page 122 ---

RM0090
USB on-the-go high-speed (OTG_HS)

Table 215. OTG_HS register map and reset values (continued)

| Offset | Register | 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 0xB3C | OTG_HS_DOEPDMAB1 | DMABADDR |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0xB50 | OTG_HS_DOEPTSIZ2 | Reserved | RXDPID/STUPCNT | PKTCNT | XFRSIZ |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0xB54 | OTG_HS_DOEPDMA2 | DMAADDR |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0xB5C | OTG_HS_DOEPDMAB2 | DMABADDR |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0xB70 | OTG_HS_DOEPTSIZ3 | Reserved | RXDPID/STUPCNT | PKTCNT | XFRSIZ |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0xB74 | OTG_HS_DOEPDMA3 | DMAADDR |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0xB7C | OTG_HS_DOEPDMAB3 | DMABADDR |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0xE00 | OTG_HS_PCGCTL | Reserved | PHYSUSP | Reserved | GATEHCLK | STPCLK |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Refer to Section 2.3: Memory map for the register boundary addresses.


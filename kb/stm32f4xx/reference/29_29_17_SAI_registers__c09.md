> [RM0090 ch.29 §29.17] SAI registers


### 29.17.9 SAI register map

The following table summarizes the SAI registers.

**Table 132. SAI register map and reset values**

| Offset | Register and reset value | 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 0x0004 or 0x0024 | SAI_xCR1 | Reserved | MCJDIV[3:0] | NODIV | Res. | DMAEN | SAxEN | Reserved. | OutDri | MONO | SYNCEN[1:0] | CKSTR | LSBFIRST | DS[2:0] | Res. | PRTCFG[1:0] | MODE[1:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 0 | 0 | Reserved. | 0 | 1 | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x0008 or 0x0028 | SAI_xCR2 | Reserved | COMP[1:0] | CPL | MUTE CN[5:0] | MUTE VAL | MUTE | TRIS | FFLUS | FTH |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 | 0 0 0 0 0 0 | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x000C or 0x002C | SAI_xFRCR | Reserved | FSOFF | FSPOL | FSDEF | Reserved. | FSALL[6:0] | FRL[7:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 | Reserved. | 0 0 0 0 0 0 0 | 0 0 1 1 1 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x0010 or 0x0030 | SAI_xSLOTR | SLOTEN[15:0] | Reserved. | NBSLOT[3:0] | SLOTSZ[1:0] | Reserved. | FBOFF[4:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 | Reserved. | 0 0 0 0 | 0 0 | Reserved. | 0 0 0 0 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x0014 or 0x0034 | SAI_xIM | Reserved | LFSDET | AFSDET | AFSDETIE | CNRDYIE | FREQIE | WCKCFG | MUTEDET | OVRUDRE |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x0018 or 0x0038 | SAI_xSR | Reserved | FLVL[2:0] | Reserved. | LFSDET | AFSDET | CNRDY | FREQ | WCKCFG | MUTEDET | OVRUDR |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 0 0 | Reserved. | 0 | 1 | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


ST
RM0090 Rev 18
963/1749

--- Page 184 ---

**Serial audio interface (SAI)**
RM0090

**Table 132. SAI register map and reset values (continued)**

| Offset | Register and reset value | 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 0x001Cor 0x003C | SAI_xCLRFR | Reserved | LFSDET | CAFSDDET | CNRDY | Res. | WCKCFG | MUTEDET | OVRUDR |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x0020or 0x0040 | SAI_XDR | DATA[31:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Refer to Section 2.3: Memory map for the register boundary addresses.

964/1749
RM0090 Rev 18

--- Page 185 ---

RM0090 Universal synchronous asynchronous receiver transmitter (USART)


> [RM0090 ch.16 §16.7] LTDC registers**

### 16.7.26 LTDC register map**

**16.7.26 LTDC register map**

The following table summarizes the LTDC registers. Refer to the register boundary addresses table for the LTDC register base address.

**Table 92. LTDC register map and reset values**

| Offset Regist | er 31 30 29 | 28 | 27 | 26 | 25 24 | 23 | 22 | 21 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| LTDC SS 0x0008 _ Reset va | CR Reserved lue |  |  |  | HSW[9:0] |  |  |  |  |  |  |  |  |  | Reserved |  |  |  |  | V |  |  |  |  | SH[10:0] |  |  |  |  |  |  |
| 0 0 | 0 | 0 0 | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| LTDC BP 0x000C _ Reset va | CR Reserved lue | AHBP[11:0] |  |  |  |  |  |  | Reserved |  | AV | BP[10:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0 | 0 0 | 0 | 0 0 | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| LTDC AW 0x0010 _ Reset va | CR Reserved lue | AAV[11:0] |  |  |  |  |  |  | Reserved |  | A | AH[10:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0 | 9 | 0 0 | 0 | 0 0 | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| LTDC TW 0x0014 _ Reset va | CR Reserved lue |  |  |  | TOTALW[11:0] |  |  |  |  |  |  |  |  |  |  |  | TALH[10:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reserved |  | TO |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0 | 0 0 | 0 | 0 0 | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| LTDC G 0x0018 _ Reset va | HSPOL VSPOL DEPOL CR lue 0 0 0 | PCPOL | Reserve | DEN | Reserved |  | :0] |  | :0] |  |  | DBW[2:0] | Reserved | LTDCEN |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  | DRW[2 | Reserved | DGW[2 | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0 | 1 | 0 | 1 | 0 | 1 | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| LTDC SR 0x0024 _ Reset va | CR Reserved lue |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | VBR | IMR |  |  |  |  |  |
| 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Состоян LTDC BC 0x002C _ | ие Пр CR Reserved | ичина и анализ | Исправление BC[23:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset va | lue | 0 | 0 0 | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | E | RIE | E |  |
| LTDC I 0x0034 _ Reset va | ER Reserved lue |  |  |  |  |  |  |  |  |  | RRI | TER | FUI | LIE |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| LTDC I 0x0038 _ Reset va | SR Reserved lue |  |  |  |  |  |  |  |  |  | RIF | RRIF | UIF | IF |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  | R | TE | F | L |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| LTDC I _ | CR |  |  |  |  |  |  |  |  |  | RRIF | ERRIF | FUIF | LIF |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x003C Reset va | Reserved lue |  |  |  |  |  |  |  |  |  | C | CT | C |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| LTDC LI 0x0040 _ Reset va | а PCR Reserved lue |  |  | LIPOS[10:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Доба | вьт 0 | е ко 0 | нс 0 | ист 0 | ент 0 | ну 0 | ю с 0 | маз 0 | ку 0 | или 0 | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |




--- Page 123 ---

**RM0090 LCD-TFT controller (LTDC)**

**Table 92. LTDC register map and reset values (continued)**

| Offset | Register | 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 0x008C | LTDC L1WVPCR _ Reset value | Reserved | WVSPPOS[10:0] | Reserved | WVSTPOS[10:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x0090 | LTDC L1CKCR _ Reset value | Reserved | CKRED[7:0] | CKGREEN[7:0] | CKBLUE[7:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x0094 | LTDC L1PFCR _ Reset value | Reserved | PF[2:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x0098 | LTDC L1CACR _ Reset value | Reserved | CONSTA[7:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 1 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x009C | LTDC L1DCCR _ Reset value | DCALPHA[7:0] | DCRED[7:0] | DCGREEN[7:0] | DCBLUE[7:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x00A0 | LTDC L1BFCR _ Reset value | Reserved | BF1[2:0] | Reserved | BF2[2:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 1 | 0 | 1 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x00AC | LTDC L1CFBAR _ Reset value | CFBADD[31:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x00B0 | LTDC L1CFBLR _ Reset value | Reserved | CFBP[12:0] | Reserved | CFBLL[12:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x00B4 | LTDC L1CFBLNR _ Reset value | Reserved | CFBLNBR[10:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x00C4 | LTDC L1CLUTWR _ | CLUTADD[7:0] | RED[7:0] | GREEN[7:0] | BLUE[7:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x0104 | LTDC L2CR _ Reset value | Reserved | CLUTEN | Reserved | COLKEN | LEN |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x0108 | LTDC L2WHPCR _ Reset value | Reserved | WHSPPOS[11:0] | Reserved | WHSTPOS[11:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x010C | LTDC L2WVPCR _ Reset value | Reserved | WVSPPOS[10:0] | Reserved | WVSTPOS[10:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x0110 | LTDC L2CKCR _ Reset value | Reserved | CKRED[7:0] | CKGREEN[7:0] | CKBLUE[7:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x0114 | LTDC L2PFCR _ Reset value | Reserved | PF[2:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x0118 | LTDC L2CACR _ Reset value | Reserved | CONSTA[7:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 1 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x011C | LTDC L2DCCR _ Reset value | DCALPHA[7:0] | DCRED[7:0] | DCGREEN[7:0] | DCBLUE[7:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x0120 | LTDC L2BFCR _ Reset value | Reserved | BF1[2:0] | Reserved | BF2[2:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 1 | 0 | 1 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x012C | LTDC L2CFBAR _ Reset value | CFBADD[31:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x0130 | LTDC L2CFBLR _ Reset value | Reserved | CFBP[12:0] | Reserved | CFBLL[12:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |




514

--- Page 124 ---

**LCD-TFT controller (LTDC) RM0090**

**Table 92. LTDC register map and reset values (continued)**

| Offset | Register | 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 0x0134 | LTDC L2CFBLNR _ | поршня Утечка масла Проверьте соедине Reserved | е CFBLNBR[10:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | ни 0 | е и 0 | м 0 | ас 0 | ля 0 | ну 0 | ю 0 | тр 0 | уб0 | ку0 | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x0144 | LTDC L2CLUTWR _ Reset value | Смешение или пов CLUTADD[7:0] | реждение RED[7:0] | GREEN[7:0] | BLUE[7:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0 | ма 0 | сла 0 | во 0 | вн 0 | утр 0 | енн 0 | ей 0 | 0 | З 0 | ам 0 | ен 0 | ит 0 | е 0 | уп 0 | ло 0 | тн 0 | ите 0 | ль 0 | но 0 | е 0 | ко 0 | ль 0 | цо 0 | ил 0 | и 0 | 0 |  |  |  |  |  |  |  |




--- Page 125 ---

**RM0090 Advanced-control timers (TIM1 and TIM8)**


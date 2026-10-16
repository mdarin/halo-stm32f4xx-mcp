### 37.8 FMC register map

The following table summarizes the FMC registers.

Table 297. FMC register map

| Offset | Register | 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 0x00 | FMC_BCR1 | Reserved | CCLKEN | CBURSTRW | CPSIZE[2:0] | ASYNCWAIT | EXTMOD | WAITEN | WREN | WAITCFG | WRAPMOD | WAITPOL | BURSTEN | Reserved | FACCEN | Reserved | MWID[1:0] | MTYP[1:0] | MUXEN | MBKEN |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x08 | FMC_BCR2 | Reserved | CBURSTRW | CPSIZE[2:0] | ASYNCWAIT | EXTMOD | WAITEN | WREN | WAITCFG | WRAPMOD | WAITPOL | BURSTEN | Reserved | FACCEN | Reserved | MWID[1:0] | MTYP[1:0] | MUXEN | MBKEN |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x10 | FMC_BCR3 | Reserved | CBURSTRW | CPSIZE[2:0] | ASYNCWAIT | EXTMOD | WAITEN | WREN | WAITCFG | WRAPMOD | WAITPOL | BURSTEN | Reserved | FACCEN | Reserved | MWID[1:0] | MTYP[1:0] | MUXEN | MBKEN |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x18 | FMC_BCR4 | Reserved | CBURSTRW | CPSIZE[2:0] | ASYNCWAIT | EXTMOD | WAITEN | WREN | WAITCFG | WRAPMOD | WAITPOL | BURSTEN | Reserved | FACCEN | Reserved | MWID[1:0] | MTYP[1:0] | MUXEN | MBKEN |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x04 | FMC_BTR1 | Res. | ACC MOD[1:0] | DATLAT[3:0] | CLKDIV[3:0] | BUSTURN[3:0] | DATAST[7:0] | ADDHLD[3:0] | ADDSET[3:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x0C | FMC_BTR2 | Res. | ACC MOD[1:0] | DATLAT[3:0] | CLKDIV[3:0] | BUSTURN[3:0] | DATAST[7:0] | ADDHLD[3:0] | ADDSET[3:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x14 | FMC_BTR3 | Res. | ACC MOD[1:0] | DATLAT[3:0] | CLKDIV[3:0] | BUSTURN[3:0] | DATAST[7:0] | ADDHLD[3:0] | ADDSET[3:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x1C | FMC_BTR4 | Res. | ACC MOD[1:0] | DATLAT[3:0] | CLKDIV[3:0] | BUSTURN[3:0] | DATAST[7:0] | ADDHLD[3:0] | ADDSET[3:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x104 | FMC_BWTR1 | Res. | ACC MOD[1:0] | Res. | BUSTURN[3:0] | DATAST[7:0] | ADDHLD[3:0] | ADDSET[3:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


1680/1749

RM0090 Rev 18

ST

--- Page 121 ---

RM0090
Flexible memory controller (FMC)
Table 297. FMC register map (continued)

| Offset | Register | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
|  |  | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27 | 28 | 29 | 30 | 31 |
| 0x10C | FMC_BWTR2 | Res. | ACCMOD1:0/ACCMOD1:0/ACCMOD1:0 | Res. | BUSTURN[3:0] | DATAST[7:0] | ADDHLD[3:0] | ADDSET[3:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x104 | FMC_BWTR3 | Res. | BUSTURN[3:0] | DATAST[7:0] | ADDHLD[3:0] | ADDSET[3:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x10C | FMC_BWTR4 | Res. | BUSTURN[3:0] | DATAST[7:0] | ADDHLD[3:0] | ADDSET[3:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x60 | FMC_PCR2 | Reserved | TAR[3:0] | TCLR[3:0] | Res. | ECCEN | PWID[1:0] | PTYP | PBKEN | PWATEN | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x80 | FMC_PCR3 | Reserved | TAR[3:0] | TCLR[3:0] | Res. | ECCEN | PWID[1:0] | PTYP | PBKEN | PWATEN | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0xA0 | FMC_PCR4 | Reserved | TAR[3:0] | TCLR[3:0] | Res. | ECCEN | PWID[1:0] | PTYP | PBKEN | PWATEN | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x64 | FMC_SR2 | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x84 | FMC_SR3 | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0xA4 | FMC_SR4 | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x68 | FMC_PMEM2 | MEMHIZ[7:0] | MEMWAIT[7:0] | MEMSET[7:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x88 | FMC_PMEM3 | MEMHIZ[7:0] | MEMWAIT[7:0] | MEMSET[7:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0xA8 | FMC_PMEM4 | MEMHIZ[7:0] | MEMWAIT[7:0] | MEMSET[7:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x6C | FMC_PATT2 | ATTHIZ[7:0] | ATTWAIT[7:0] | ATTSET[7:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x8C | FMC_PATT3 | ATTHIZ[7:0] | ATTWAIT[7:0] | ATTSET[7:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0xAC | FMC_PATT4 | ATTHIZ[7:0] | ATTWAIT[7:0] | ATTSET[7:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x80 | FMC_IOPI4 | IOHIZ[7:0] | IOWAIT[7:0] | IOSET[7:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x74 | FMC_ECCR2 | ECC[31:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x94 | FMC_ECCR3 | ECC[31:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x140 | FMC_SDCR_1 | Reserved | RPPE[1:0] | RBURST | clk[1:0] | WP | CAS[1:0] | NB | MWID[1:0] | NR[1:0] | NC[1:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x144 | FMC_SDCR_2 | Reserved | clk[1:0] | WP | CAS[1:0] | NB | MWID[1:0] | NR[1:0] | NC[1:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x148 | FMC_SDTR1 | Reserved | TRCD[3:0] | TRP[3:0] | TWR[3:0] | TRC[3:0] | TRAS[3:0] | TXSR[3:0] | TMRD[3:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x14C | FMC_SDTR2 | Reserved | TRCD[3:0] | TRP[3:0] | TWR[3:0] | TRC[3:0] | TRAS[3:0] | TXSR[3:0] | TMRD[3:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x150 | FMC_SDCMR | Reserved | MRD[12:0] | NRF[3:0] | CTB1 | CTB2 | MODE[2:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


--- Page 122 ---

Flexible memory controller (FMC)
RM0090

Table 297. FMC register map (continued)

| Offset | Register | 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 0x154 | FMC_SDRTR | Reserved | REIE | COUNT[12:0] | CRE |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x158 | FMC_SDSR | Reserved | BUSY | MODES2[1:0] | MODES1[1:0] | RE |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


1682/1749
RM0090 Rev 18

--- Page 123 ---

RM0090
Debug support (DBG)


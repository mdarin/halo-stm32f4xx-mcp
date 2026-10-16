> [RM0090 ch.23 §23.4] CRYP interrupts

### 23.6.12 CRYP context swap registers (CRYP_CSGCMCCM0..7R and CRYP_CSGCM0..7R) for STM32F42xxx and STM32F43xxx**

**23.6.12 CRYP context swap registers (CRYP_CSGCMCCM0..7R and CRYP_CSGCM0..7R) for STM32F42xxx and STM32F43xxx**

Address offset:
* CRYP_CSGCMCCM0..7: 0x050 to 0x06C: used for GCM/GMAC or CCM/CMAC algorithm only
* CRYP_CSGCM0..7: 0x070 to 0x08C: used for GCM/GMAC algorithm only

Reset value: 0x0000 0000

These registers contain the complete internal register states of the CRYP processor when the GCM/GMAC or CCM/CMAC algorithm is selected. They are useful when a context swap has to be performed because a high-priority task needs the cryptographic processor while it is already in use by another task.

When such an event occurs, the CRYP_CSGCMCCM0..7R and CRYP_CSGCM0..7R (in GCM/GMAC mode) or CRYP_CSGCMCCM0..7R (in CCM/CMAC mode) registers have to be read and the values retrieved have to be saved in the system memory space. The cryptographic processor can then be used by the preemptive task, and when the cryptographic computation is complete, the saved context can be read from memory and written back into the corresponding context swap registers.

*Note:* These registers are used only when GCM/GMAC or CCM/CMAC algorithm mode is selected.

**CRYP_CSGCMCCMxR: where x=[7:0]**

| 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 |
|---|
| CRYP_CSGCMCCMxR |
| rw |
| 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0 |
| CRYP_CSGCMCCMxR |
| rw |


**CRYP_CSGCMxR: where x=[7:0]**

| 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 |
|---|
| CRYP_CSGCMxR |
| rw |
| 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0 |
| CRYP_CSGCMxR |
| rw |


762/1749
RM0090 Rev 18

--- Page 178 ---

RM0090
Crytographic processor (CRYP)

23.6.13 **CRYP register map**

Table 114. CRYP register map and reset values for STM32F415/417xx

| Offset | Register name and reset value | bit value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 0x00 | CRYP_CR | Reserved | 0 | CRYPEN | 0 | FFLUSH | Reserved | 0 | KEYSIZE | 0 | DATATYPE | 0 | ALGOMODE[2:0] | 0 | ALGODIR | 0 | Res. | 0 |
| 0x04 | CRYP_SR | Reserved | IFEM | 1 | IFNF | 0 | OFU | 0 | OFNE | 0 | BUSY | 0 |  |  |  |  |  |  |
| 0x08 | CRYP_DIN | Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x0C | CRYP_DOUT | Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x10 | CRYP_DMACH | Reserved | DEN | 0 | OUTM | 0 |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x14 | CRYP_IMSCR | Reserved | NIM | 0 | OUTIM | 0 |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x18 | CRYP_RISR | Reserved | INRIS | 1 | OUTRIS | 0 |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x1C | CRYP_MISR | Reserved | IN%MIS | 0 | OUTMIS | 0 |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x20 | CRYP_KOLR | Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x24 | CRYP_KORR | Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x28 | CRYP_KOLR | ... |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x38 | CRYP_K3LR | Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x3C | CRYP_K3RR | Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x40 | CRYP_IVOLR | Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


--- Page 179 ---

Cryptographic processor (CRYP)
RM0090

Table 114. CRYP register map and reset values for STM32F415/417xx (continued)

| Offset | Register name and reset value | 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 0x44 | CRYP_IV0RR |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x48 | CRYP_IV1LR |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x4C | CRYP_IV1RR |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Table 115. CRYP register map and reset values for STM32F43xxx

| Offset | Register name reset value | 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 0x00 0x00 | CRYP_CR | Reserved | ALGOMODE[3] | Res. | GCM_CCMPH | CRYPEN | FFLUSH | Reserved | KEYSIZE | DATATYPE | ALOMODE[2:0] | ALGODIR | Res.. |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 0 |  |  | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x04 | CRYP_SR | Reserved | BUSY | OFFU | OFNE | IFNF | IFEM |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 0 | 1 |  |  |  |  |
| 0x08 | CRYP_DIN | DATAIN |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x0C | CRYP_DOUT | DATAOUT |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x10 | CRYP_DMACR | Reserved | DOEN | DIEN |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 0 |  |  |  |  |
| 0x14 | CRYP_IMSCR | Reserved | OUTRIS | OUTM | INRIS | INIM |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 0 |  |  |  |  |
| 0x18 | CRYP_RISR | Reserved | OUTRIS | INRIS |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 0 | 1 |  |  |  |


764/1749
RM0090 Rev 18

--- Page 180 ---

RM0090
Cryptographic processor (CRYP)

Table 115. CRYP register map and reset values for STM32F43xxx (continued)

| Offset | Register namereset value | 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 0x1C | CRYP_MISR | Reserved | OUTMIS | IN%IS |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 0 |  |
| 0x20 | CRYP_K0LR |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x24 | CRYP_K0RR |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| ... |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x38 | CRYP_K3LR |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x3C | CRYP_K3RR |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x40 | CRYP_IV0LR |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x44 | CRYP_IV0RR |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x48 | CRYP_IV1LR |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x4C | CRYP_IV1RR |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x50 | CRYP_CSGCMCCM R | CRYP_CSGCMCCM0R |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x54 | CRYP_CSGCMCCM 1R | CRYP_CSGCMCCM1R |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x58 | CRYP_CSGCMCCM 2R | CRYP_CSGCMCCM2R |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x5C | CRYP_CSGCMCCM 3R | CRYP_CSGCMCCM3R |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


ST
RM0090 Rev 18
765/1749

--- Page 181 ---

Crytographic processor (CRYP)
RM0090

Table 115. CRYP register map and reset values for STM32F43xxx (continued)

Refer to Section 2.3: Memory map for the register boundary addresses.
766/1749
RM0090 Rev 18

| Offset | Register name | 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 0x60 | CRYP_CSGCMCCM4R | Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| CRYP_CSGCMCCM4R | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x64 | CRYP_CSGCMCCM5R | Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| CRYP_CSGCMCCM5R | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x68 | CRYP_CSGCMCCM6R | Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| CRYP_CSGCMCCM6R | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x6C | CRYP_CSGCMCCM7R | Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| CRYP_CSGCMCCM7R | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x70 | CRYP_CSGCM0R | Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| CRYP_CSGCM0R | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x74 | CRYP_CSGCM1R | Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| CRYP_CSGCM1R | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x78 | CRYP_CSGCM2R | Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| CRYP_CSGCM2R | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x7C | CRYP_CSGCM3R | Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| CRYP_CSGCM3R | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x80 | CRYP_CSGCM4R | Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| CRYP_CSGCM4R | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x84 | CRYP_CSGCM5R | Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| CRYP_CSGCM5R | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x88 | CRYP_CSGCM6R | Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| CRYP_CSGCM6R | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x8C | CRYP_CSGCM7R | Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| CRYP_CSGCM7R | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


--- Page 182 ---

RM0090
Random number generator (RNG)


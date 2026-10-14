> [RM0090 ch.23 §23.4] CRYP interrupts

### 23.6.11 CRYP initialization vector registers (CRYP_IV0...1(L/R)R)**

**23.6.11 CRYP initialization vector registers (CRYP_IV0...1(L/R)R)**

Address offset: 0x40 to 0x4C
Reset value: 0x0000 0000

The CRYP_IV0...1(L/R)R are the left-word and right-word registers for the initialization vector (64 bits for DES/TDES and 128 bits for AES) and are used in the CBC (Cipher block chaining) and Counter (CTR) modes. After each computation round of the TDES or AES Core, the CRYP_IV0...1(L/R)R registers are updated as described in Section : DES and TDES Cipher block chaining (DES/TDES-CBC) mode on page 726, Section : AES Cipher block chaining (AES-CBC) mode on page 730 and Section : AES counter mode (AES-CTR) mode on page 732.

IV0 is the leftmost bit whereas IV63 (DES, TDES) or IV127 (AES) are the rightmost bits of the initialization vector. IV1(L/R)R is used only in the AES.

**CRYP_IV0LR (address offset: 0x40)**

| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| IV0 | IV1 | IV2 | IV3 | IV4 | IV5 | IV6 | IV7 | IV8 | IV9 | IV10 | IV11 | IV12 | IV13 | IV14 | IV15 |
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
| IV16 | IV17 | IV18 | IV19 | IV20 | IV21 | IV22 | IV23 | IV24 | IV25 | IV26 | IV27 | IV28 | IV29 | IV30 | IV31 |
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


**CRYP_IV0RR (address offset: 0x44)**

| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| IV32 | IV33 | IV34 | IV35 | IV36 | IV37 | IV38 | IV39 | IV40 | IV41 | IV42 | IV43 | IV44 | IV45 | IV46 | IV47 |
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
| IV48 | IV49 | IV50 | IV51 | IV52 | IV53 | IV54 | IV55 | IV56 | IV57 | IV58 | IV59 | IV60 | IV61 | IV62 | IV63 |
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


760/1749
RM0090 Rev 18

--- Page 176 ---

RM0090
Cryptographic processor (CRYP)

> [RM0090 ch.23 §23.4] CRYP interrupts

### CRYP_IV1LR (address offset: 0x48)

### CRYP_IV1LR (address offset: 0x48)

| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| IV64 | IV65 | IV66 | IV67 | IV68 | IV69 | IV70 | IV71 | IV72 | IV73 | IV74 | IV75 | IV76 | IV77 | IV78 | IV79 |
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
| IV80 | IV81 | IV82 | IV83 | IV84 | IV85 | IV86 | IV87 | IV88 | IV89 | IV90 | IV91 | IV92 | IV93 | IV94 | IV95 |
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


> [RM0090 ch.23 §23.4] CRYP interrupts

### CRYP_IV1RR (address offset: 0x4C)

### CRYP_IV1RR (address offset: 0x4C)

| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| IV96 | IV97 | IV98 | IV99 | IV100 | IV101 | IV102 | IV103 | IV104 | IV105 | IV106 | IV107 | IV108 | IV109 | IV110 | IV111 |
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
| IV112 | IV113 | IV114 | IV115 | IV116 | IV117 | IV118 | IV119 | IV120 | IV121 | IV122 | IV123 | IV124 | IV125 | IV126 | IV127 |
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


**Note:**
*In DES/3DES modes, only CRYP_IV0(L/R) is used.*
*Write access to these registers are disregarded when the cryptographic processor is busy (bit BUSY = 1 in the CRYP_SR register).*

ST
RM0090 Rev 18
761/1749

--- Page 177 ---

Cryptographic processor (CRYP)
RM0090


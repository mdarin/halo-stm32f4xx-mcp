> [RM0090 ch.31 §31.9] SDIO registers**


### 31.9.12 SDIO interrupt clear register (SDIO\_ICR)

Address offset: 0x38
Reset value: 0x0000 0000

The SDIO\_ICR register is a write-only register. Writing a bit with 1b clears the corresponding bit in the SDIO\_STA Status register.

| 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved | CEATAENDC | SDIOITC | Reserved | DBCKENDC | STBITERRC | DATAENDC | CMDSENTC | CMDRENDC | RXOVERRC | TXUNDERRC | DTIMEOUTC | CTIMEOUTC | DCRCFAILC | CCRCFAILC |
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bits 31:24 Reserved, must be kept at reset value
Bit 23 **CEATAENDC**: CEATAEND flag clear bit
Set by software to clear the CEATAEND flag.
0: CEATAEND not cleared
1: CEATAEND cleared
Bit 22 **SDIOITC**: SDIOIT flag clear bit
Set by software to clear the SDIOIT flag.
0: SDIOIT not cleared
1: SDIOIT cleared
Bits 21:11 Reserved, must be kept at reset value
Bit 10 **DBCKENDC**: DBCKEND flag clear bit
Set by software to clear the DBCKEND flag.
0: DBCKEND not cleared
1: DBCKEND cleared
Bit 9 **STBITERRC**: STBITERR flag clear bit
Set by software to clear the STBITERR flag.
0: STBITERR not cleared
1: STBITERR cleared
Bit 8 **DATAENDC**: DATAEND flag clear bit
Set by software to clear the DATAEND flag.
0: DATAEND not cleared
1: DATAEND cleared

ST
RM0090 Rev 18
1069/1749

--- Page 95 ---

**Secure digital input/output interface (SDIO)**

RM0090

**Bit 7 CMDSENTC**: CMDSENT flag clear bit
Set by software to clear the CMDSENT flag.
0: CMDSENT not cleared
1: CMDSENT cleared

**Bit 6 CMDRENDC**: CMDREND flag clear bit
Set by software to clear the CMDREND flag.
0: CMDREND not cleared
1: CMDREND cleared

**Bit 5 RXOVERRC**: RXOVERR flag clear bit
Set by software to clear the RXOVERR flag.
0: RXOVERR not cleared
1: RXOVERR cleared

**Bit 4 TXUNDERRC**: TXUNDER flag clear bit
Set by software to clear the TXUNDER flag.
0: TXUNDER not cleared
1: TXUNDER cleared

**Bit 3 DTIMEOUTC**: DTIMEOUT flag clear bit
Set by software to clear the DTIMEOUT flag.
0: DTIMEOUT not cleared
1: DTIMEOUT cleared

**Bit 2 CTIMEOUTC**: CTIMEOUT flag clear bit
Set by software to clear the CTIMEOUT flag.
0: CTIMEOUT not cleared
1: CTIMEOUT cleared

**Bit 1 DCRFAILC**: DCRFAIL flag clear bit
Set by software to clear the DCRFAIL flag.
0: DCRFAIL not cleared
1: DCRFAIL cleared

**Bit 0 CCRFAILC**: CCRFAIL flag clear bit
Set by software to clear the CCRFAIL flag.
0: CCRFAIL not cleared
1: CCRFAIL cleared

1070/1749
RM0090 Rev 18

--- Page 96 ---

RM0090
Secure digital input/output interface (SDIO)


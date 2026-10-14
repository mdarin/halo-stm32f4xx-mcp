> [RM0090 ch.31 §31.9] SDIO registers**

### 31.9.16 SDIO register map**

**31.9.16 SDIO register map**

The following table summarizes the SDIO registers.

**Table 181. SDIO register map**

| Offset | Register | 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 0x00 | SDIO_POWER | Reserved | PWRCTRL |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x04 | SDIO_CLKCR | Reserved | HWFC_EN | NEGEDGE | WIDBUS | BYPASS | PWRSV | CLKEN | CLKDIV |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x08 | SDIO_ARG | CMDARG |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x0C | SDIO_CMD | Reserved | CE-ATACMD | nIEN | ENCMDcompl | SDIOSuspend | CPSMEN | WAITPEND | WAITINT | WAITRESP | CMDINDEX |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x10 | SDIO_RESPCMD | Reserved | RESPCMD |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x14 | SDIO_RESP1 | CARDSTATUS1 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x18 | SDIO_RESP2 | CARDSTATUS2 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x1C | SDIO_RESP3 | CARDSTATUS3 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x20 | SDIO_RESP4 | CARDSTATUS4 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x24 | SDIO_DTIMER | DATATIME |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x28 | SDIO_DLEN | Reserved | DATALENGTH |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x2C | SDIO_DCTRL | Reserved | SDIOEN | RWMOD | RWSTOP | RWSTART | DBLOCKSIZE | DMAEN | DTMODE | DTDIR | DTEN |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x30 | SDIO_DCOUNT | Reserved | DATACOUNT |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


1074/1749
RM0090 Rev 18

--- Page 100 ---

RM0090
Secure digital input/output interface (SDIO)

Table 181. SDIO register map (continued)

| Offset | Register | SDIO_STA | SDIO_ICR | SDIO_MASK | FIFO0Data |
|---|---|---|---|---|---|
| SDIO_FIFOCNT | SDIO_FIFO |  |  |  |  |
| 0x34 | SDIO_STA | CEATAEND | Reserved | SDIOITIE | SDIOIT |
| 0x38 | SDIO_ICR | RXDAVLIE | RXDAVL |  |  |
| 0x3C | SDIO_MASK | TXDAVLIE | TXDAVL |  |  |
| 0x48 | SDIO_FIFOCNT | RXFIFOEIE | RXFIFOE |  |  |
| 0x80 | SDIO_FIFO | TXFIFOEIE | TXFIFOE |  |  |
|  |  | RXFIFOIE | RXFIFO |  |  |
|  |  | TXFIFOIE | TXFIFO |  |  |
|  |  | RXFIFOHFIE | RXFIFOHF |  |  |
|  |  | TXFIFOHEIE | TXFIFOHE |  |  |
|  |  | RXACTIE | RXACT |  |  |
| 12 |  | TXACTIE | TXACT |  |  |
| 13 |  | CMDACTIE | CMDACT |  |  |
| 14 |  | DBCKENDIE | DBCKEND |  |  |
| 15 |  | STBITERRIE | STBITERR |  |  |
| 16 |  | DATAENDIE | DATAEND |  |  |
| 17 |  | CMDSENTIE | CMDSENT |  |  |
| 18 |  | CMDRENDIE | CMDREND |  |  |
| 19 |  | RXOVERRIE | RXOVERR |  |  |
| 20 |  | TXUNDERRIE | TXUNDERR |  |  |
| 21 |  | DTIMEOUTIE | DTIMEOUT |  |  |
| 22 |  | CTIMEOUTIE | CTIMEOUT |  |  |
| 23 |  | DCRFAILIE | DCRFAIL |  |  |
| 24 |  | CCRFAILIE | CCRFAIL |  |  |
| 25 |  |  |  |  |  |
| 26 |  |  |  |  |  |
| 27 |  |  |  |  |  |
| 28 |  |  |  |  |  |
| 29 |  |  |  |  |  |
| 30 |  |  |  |  |  |
| 31 |  |  |  |  |  |


RM0090 Rev 18
1075/1749

--- Page 101 ---

Controller area network (bxCAN)
RM0090


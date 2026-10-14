> [RM0090 ch.31 §31.9] SDIO registers**


### 31.9.9 SDIO data control register (SDIO_DCTRL)

Address offset: 0x2C
Reset value: 0x0000 0000

The SDIO_DCTRL register control the data path state machine (DPSM).

| 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 15 14 13 12 | 11 | 10 | 9 | 8 | 7 6 5 4 | 3 | 2 | 1 | 0 |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved | SDIOEN | RWMOD | RWSTOP | RWSTART | DBLOCKSIZE | DMAEN | DTMODE | DTDIR | DTEN |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | rw |


Bits 31:12 Reserved, must be kept at reset value

**Bit 11 SDIOEN:** SD I/O enable functions
If this bit is set, the DPSM performs an SD I/O-card-specific operation.

**Bit 10 RWMOD:** Read wait mode
0: Read Wait control stopping SDIO_D2
1: Read Wait control using SDIO_CK

**Bit 9 RWSTOP:** Read wait stop
0: Read wait in progress if RWSTART bit is set
1: Enable for read wait stop if RWSTART bit is set

**Bit 8 RWSTART:** Read wait start
If this bit is set, read wait operation starts.

**Bits 7:4 DBLOCKSIZE:** Data block size
Define the data block length when the block data transfer mode is selected:
0000: (0 decimal) lock length = 2^0 = 1 byte
0001: (1 decimal) lock length = 2^1 = 2 bytes
0010: (2 decimal) lock length = 2^2 = 4 bytes
0011: (3 decimal) lock length = 2^3 = 8 bytes
0100: (4 decimal) lock length = 2^4 = 16 bytes
0101: (5 decimal) lock length = 2^5 = 32 bytes
0110: (6 decimal) lock length = 2^6 = 64 bytes
0111: (7 decimal) lock length = 2^7 = 128 bytes
1000: (8 decimal) lock length = 2^8 = 256 bytes
1001: (9 decimal) lock length = 2^9 = 512 bytes
1010: (10 decimal) lock length = 2^10 = 1024 bytes
1011: (11 decimal) lock length = 2^11 = 2048 bytes
1100: (12 decimal) lock length = 2^12 = 4096 bytes
1101: (13 decimal) lock length = 2^13 = 8192 bytes
1110: (14 decimal) lock length = 2^14 = 16384 bytes
1111: (15 decimal) reserved

**Bit 3 DMAEN:** DMA enable bit
0: DMA disabled.
1: DMA enabled.

1066/1749
RM0090 Rev 18

--- Page 92 ---

RM0090
Secure digital input/output interface (SDIO)

Bit 2 **DTMODE**: Data transfer mode selection 1: Stream or SDIO multibyte data transfer.
0: Block data transfer
1: Stream or SDIO multibyte data transfer

Bit 1 **DTDIR**: Data transfer direction selection
0: From controller to card.
1: From card to controller.

Bit 0 **DTEN**: Data transfer enabled bit
Data transfer starts if 1b is written to the DTEN bit. Depending on the direction bit, DTDIR, the DPSM moves to the Wait_S, Wait_R state or Readwait if RW Start is set immediately at the beginning of the transfer. It is not necessary to clear the enable bit after the end of a data transfer but the SDIO_DCTRL must be updated to enable a new data transfer

*Note:* After a data write, data cannot be written to this register for three SDIOCLK clock periods plus two PCLK2 clock periods.

*The meaning of the DTMODE bit changes according to the value of the SDIOEN bit. When SDIOEN=0 and DTMODE=1, the MultiMediaCard stream mode is enabled, and when SDIOEN=1 and DTMODE=1, the peripheral enables an SDIO multibyte transfer.*

> [RM0090 ch.31 §31.9] SDIO registers**

### 31.9.10 SDIO data counter register (SDIO_DCOUNT)

### 31.9.10 SDIO data counter register (SDIO_DCOUNT)

Address offset: 0x30
Reset value: 0x0000 0000

The SDIO_DCOUNT register loads the value from the data length register (see SDIO_DLEN) when the DPSM moves from the Idle state to the Wait_R or Wait_S state. As data is transferred, the counter decrements the value until it reaches 0. The DPSM then moves to the Idle state and the data status end flag, DATAEND, is set.

| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved | DATACOUNT |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| r |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bits 31:25 **Reserved**, must be kept at reset value
Bits 24:0 **DATACOUNT**: Data count value
When this bit is read, the number of remaining data bytes to be transferred is returned. Write has no effect.

*Note:* This register should be read only when the data transfer is complete.

ST
RM0090 Rev 18
1067/1749

--- Page 93 ---

Secure digital input/output interface (SDIO)
RM0090


> [RM0090 ch.31 §31.9] SDIO registers**

### 31.9 SDIO registers**

**31.9 SDIO registers**

The device communicates to the system via 32-bit-wide control registers accessible via APB2.

The peripheral registers have to be accessed by words (32 bits).

> [RM0090 ch.31 §31.9] SDIO registers**

### 31.9.1 SDIO power control register (SDIO_POWER)**

**31.9.1 SDIO power control register (SDIO_POWER)**

Address offset: 0x00
Reset value: 0x0000 0000

| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved | PWRC | TRL |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | rw |  |


Bits 31:2 Reserved, must be kept at reset value
Bits 1:0 **PWRCTRL**: Power supply control bits.
These bits are used to define the current functional state of the card clock:
00: Power-off: the clock to card is stopped.
01: Reserved
10: Reserved power-up
11: Power-on: the card is clocked.

*Note:* At least seven HCLK clock periods are needed between two write accesses to this register. After a data write, data cannot be written to this register for three SDIOCLK clock periods plus two PCLK2 clock periods.

1060/1749
RM0090 Rev 18

--- Page 86 ---

RM0090
Secure digital input/output interface (SDIO)


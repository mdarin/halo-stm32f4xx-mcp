> [RM0090 ch.31 §31.9] SDIO registers**


### 31.9.2 SDI clock control register (SDIO_CLKCR)

Address offset: 0x04
Reset value: 0x0000 0000
The SDIO_CLKCR register controls the SDIO_CK output clock.

| 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 15 | HWFC_EN | NEGEDGE | WID BUS | BYPASS | PWRSV/ | CLKEN | CLKDIV |
|---|---|---|---|---|---|---|---|
| Reserved | r/w |  |  |  |  |  |  |


Bits 31:15 Reserved, must be kept at reset value

**Bit 14 HWFC_EN:** HW Flow Control enable
0b: HW Flow Control is disabled
1b: HW Flow Control is enabled
When HW Flow Control is enabled, the meaning of the TXFIFOE and RXFIFOF interrupt signals, please see SDIO Status register definition in Section 31.9.11.

**Bit 13 NEGEDGE:**SDIO_CK dephasing selection bit
0b: SDIO_CK generated on the rising edge of the master clock SDIOCLK
1b: SDIO_CK generated on the falling edge of the master clock SDIOCLK

**Bits 12:11 WIDBUS:** Wide bus mode enable bit
00: Default bus mode: SDIO_D0 used
01: 4-wide bus mode: SDIO_D[3:0] used
10: 8-wide bus mode: SDIO_D[7:0] used

**Bit 10 BYPASS:** Clock divider bypass enable bit
0: Disable bypass: SDIOCLK is divided according to the CLKDIV value before driving the SDIO_CK output signal.
1: Enable bypass: SDIOCLK directly drives the SDIO_CK output signal.

**Bit 9 PWRSV:** Power saving configuration bit
For power saving, the SDIO_CK clock output can be disabled when the bus is idle by setting PWRSV:
0: SDIO_CK clock is always enabled
1: SDIO_CK is only enabled when the bus is active

**Bit 8 CLKEN:** Clock enable bit
0: SDIO_CK is disabled
1: SDIO_CK is enabled

**Bits 7:0 CLKDIV:** Clock divide factor
This field defines the divide factor between the input clock (SDIOCLK) and the output clock (SDIO_CK): SDIO_CK frequency = SDIOCLK / [CLKDIV + 2].

ST
RM0090 Rev 18
1061/1749

--- Page 87 ---

**Secure digital input/output interface (SDIO)**
RM0090

**Note:**
While the SD/SDIO card or MultiMediaCard is in identification mode, the SDIO\_CK frequency must be less than 400 kHz.
The clock frequency can be changed to the maximum card bus frequency when relative card addresses are assigned to all cards.
After a data write, data cannot be written to this register for three SDIOCLK clock periods plus two PCLK2 clock periods. SDIO\_CK can also be stopped during the read wait interval for SD I/O cards: in this case the SDIO\_CLKCR register does not control SDIO\_CK.

> [RM0090 ch.31 §31.9] SDIO registers**

### 31.9.3 SDIO argument register (SDIO\_ARG)

### 31.9.3 SDIO argument register (SDIO\_ARG)

Address offset: 0x08
Reset value: 0x0000 0000

The SDIO\_ARG register contains a 32-bit command argument, which is sent to a card as part of a command message.

| CMDARG |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bits 31:0 **CMDARG**: Command argument
Command argument sent to a card as part of a command message. If a command contains an argument, it must be loaded into this register before writing a command to the command register.


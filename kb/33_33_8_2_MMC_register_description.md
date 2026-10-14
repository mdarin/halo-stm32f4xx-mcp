### 33.8.2 MMC register description

**Ethernet MMC control register (ETH_MMCCR)**

Address offset: 0x0100
Reset value: 0x0000 0000

The Ethernet MMC Control register establishes the operating mode of the management counters.

| 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 15 14 13 12 11 10 9 8 7 6 | 5 | 4 | 3 | 2 | 1 | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved | MCFHP | MCP | MCF | ROR | CSR | CR |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | rw |


Bits 31:6 Reserved, must be kept at reset value.

**MCFHP: MMC counter Full-Half preset**
Bit 5 When MCFHP is low and bit4 is set, all MMC counters get preset to almost-half value. All frame-counters get preset to 0x7FFF_FFF0 (half - 16)
When MCFHP is high and bit4 is set, all MMC counters get preset to almost-full value. All frame-counters get preset to 0xFFFF_FFF0 (full - 16)

**MCP: MMC counter preset**
Bit 4 When set, all counters will be initialized or preset to almost full or almost half as per Bit5 above. This bit will be cleared automatically after 1 clock cycle. This bit along with bit5 is useful for debugging and testing the assertion of interrupts due to MMC counter becoming half-full or full.

Bit 3 **MCF: MMC counter freeze**
When set, this bit freezes all the MMC counters to their current value. (None of the MMC counters are updated due to any transmitted or received frame until this bit is cleared to 0. If any MMC counter is read with the Reset on Read bit set, then that counter is also cleared in this mode.)

Bit 2 **ROR: Reset on read**
When this bit is set, the MMC counters is reset to zero after read (self-clearing after reset). The counters are cleared when the least significant byte lane (bits [7:0]) is read.

Bit 1 **CSR: Counter stop rollover**
When this bit is set, the counter does not roll over to zero after it reaches the maximum value.

Bit 0 **CR: Counter reset**
When it is set, all counters are reset. This bit is cleared automatically after 1 clock cycle.

**Ethernet MMC receive interrupt register (ETH_MMCRIR)**

Address offset: 0x0104
Reset value: 0x0000 0000

The Ethernet MMC receive interrupt register maintains the interrupts generated when receive statistic counters reach half their maximum values. (MSB of the counter is set.) It is a 32-bit wide register. An interrupt bit is cleared when the respective MMC counter that

1210/1749
RM0090 Rev 18

--- Page 41 ---

RM0090
Ethernet (ETH): media access control (MAC) with DMA controller

caused the interrupt is read. The least significant byte lane (bits [7:0]) of the respective counter must be read in order to clear the interrupt bit.

| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved | RGUFS | Reserved | RFAES | RFCES | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  | rc_r |  |  |  |  |  |  |  |  |  | rc_r |  |  |  |  |  |  |  |


Bits 31:18 Reserved, must be kept at reset value.
Bit 17 **RGUFS**: Received Good Unicast Frames Status
This bit is set when the received, good unicast frames, counter reaches half the maximum value.
Bits 16:7 Reserved, must be kept at reset value.
Bit 6 **RFAES**: Received frames alignment error status
This bit is set when the received frames, with alignment error, counter reaches half the maximum value.
Bit 5 **RFCES**: Received frames CRC error status
This bit is set when the received frames, with CRC error, counter reaches half the maximum value.
Bits 4:0 Reserved, must be kept at reset value.

**Ethernet MMC transmit interrupt register (ETH_MMCTIR)**

Address offset: 0x0108
Reset value: 0x0000 0000

The Ethernet MMC transmit Interrupt register maintains the interrupts generated when transmit statistic counters reach half their maximum values. (MSB of the counter is set.) It is a 32-bit wide register. An interrupt bit is cleared when the respective MMC counter that caused the interrupt is read. The least significant byte lane (bits [7:0]) of the respective counter must be read in order to clear the interrupt bit.

| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved | TGFS | Reserved | TGFMSCS | TGFSCS | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  | rc_r |  |  |  |  | rc_r |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bits 31:22 Reserved, must be kept at reset value.
Bit 21 **TGFS**: Transmitted good frames status
This bit is set when the transmitted, good frames, counter reaches half the maximum value.
Bits 20:16 Reserved, must be kept at reset value.

STI
RM0090 Rev 18
1211/1749

--- Page 42 ---

**Ethernet (ETH): media access control (MAC) with DMA controller**

RM0090

Bit 15 **TGFMSCS**: Transmitted good frames more single collision status
This bit is set when the transmitted, good frames after more than a single collision, counter reaches half the maximum value.

Bit 14 **TGFSCS**: Transmitted good frames single collision status
This bit is set when the transmitted, good frames after a single collision, counter reaches half the maximum value.

Bits 13:0 Reserved, must be kept at reset value.

**Ethernet MMC receive interrupt mask register (ETH_MMCRIIMR)**

Address offset: 0x010C
Reset value: 0x0000 0000

The Ethernet MMC receive interrupt mask register maintains the masks for interrupts generated when the receive statistic counters reach half their maximum value. (MSB of the counter is set.) It is a 32-bit wide register.

| 31 30 29 28 27 26 25 24 23 22 21 20 19 18 | 17 | 16 15 14 13 12 11 10 9 8 7 | 6 | 5 | 4 3 2 1 0 |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved | RGUFM | Reserved | RFAEM | RFCEM | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  | rw |  | rw |  |


Bits 31:18 Reserved, must be kept at reset value.

Bit 17 **RGUFM**: Received good unicast frames mask
Setting this bit masks the interrupt when the received, good unicast frames, counter reaches half the maximum value.

Bits 16:7 Reserved, must be kept at reset value.

Bit 6 **RFAEM**: Received frames alignment error mask
Setting this bit masks the interrupt when the received frames, with alignment error, counter reaches half the maximum value.

Bit 5 **RFCEM**: Received frame CRC error mask
Setting this bit masks the interrupt when the received frames, with CRC error, counter reaches half the maximum value.

Bits 4:0 Reserved, must be kept at reset value.

1212/1749
RM0090 Rev 18

--- Page 43 ---

RM0090
Ethernet (ETH): media access control (MAC) with DMA controller

**Ethernet MMC transmit interrupt mask register (ETH_MMCTIMR)**

Address offset: 0x0110
Reset value: 0x0000 0000

The Ethernet MMC transmit interrupt mask register maintains the masks for interrupts generated when the transmit statistic counters reach half their maximum value. (MSB of the counter is set). It is a 32-bit wide register.

| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved | TGFM | Reserved | TGFMSCM | TGFSCM | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  | rw |  |  |  |  |  | rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bits 31:22 Reserved, must be kept at reset value.
Bit 21 **TGFM**: Transmitted good frames mask
Setting this bit masks the interrupt when the transmitted, good frames, counter reaches half the maximum value.
Bits 20:16 Reserved, must be kept at reset value.
Bit 15 **TGFMSCM**: Transmitted good frames more single collision mask
Setting this bit masks the interrupt when the transmitted good frames after more than a single collision counter reaches half the maximum value.
Bit 14 **TGFSCM**: Transmitted good frames single collision mask
Setting this bit masks the interrupt when the transmitted good frames after a single collision counter reaches half the maximum value.
Bits 13:0 Reserved, must be kept at reset value.

**Ethernet MMC transmitted good frames after a single collision counter register (ETH_MMCTGFSCCR)**

Address offset: 0x014C
Reset value: 0x0000 0000

This register contains the number of successfully transmitted frames after a single collision in Half-duplex mode.

| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| TGSCC |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| r |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bits 31:0 **TGSCC**: Transmitted good frames single collision counter
Transmitted good frames after a single collision counter.

STi
RM0090 Rev 18
1213/1749

--- Page 44 ---

Ethernet (ETH): media access control (MAC) with DMA controller
RM0090

**Ethernet MMC transmitted good frames after more than a single collision counter register (ETH_MMCTGFMSCCR)**

Address offset: 0x0150
Reset value: 0x0000 0000

This register contains the number of successfully transmitted frames after more than a single collision in Half-duplex mode.

| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| TGFMSCC |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| r |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bits 31:0 **TGFMSCC**: Transmitted good frames more single collision counter
Transmitted good frames after more than a single collision counter

**Ethernet MMC transmitted good frames counter register (ETH_MMCTGFCR)**

Address offset: 0x0168
Reset value: 0x0000 0000

This register contains the number of good frames transmitted.

| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| TGFC |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| r |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bits 31:0 **TGFC**: Transmitted good frames counter

**Ethernet MMC received frames with CRC error counter register (ETH_MMCRFCECR)**

Address offset: 0x0194
Reset value: 0x0000 0000

This register contains the number of frames received with CRC error.

| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| RFCEC |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| r |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bits 31:0 **RFCEC**: Received frames CRC error counter
Received frames with CRC error counter

1214/1749
RM0090 Rev 18

--- Page 45 ---

RM0090
Ethernet (ETH): media access control (MAC) with DMA controller

**Ethernet MMC received frames with alignment error counter register (ETH_MMCRFAECR)**

Address offset: 0x0198
Reset value: 0x0000 0000
This register contains the number of frames received with alignment (dribble) error.

| RFAEC |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
| r |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bits 31:0 **RFAEC**: Received frames alignment error counter
Received frames with alignment error counter

**MMC received good unicast frames counter register (ETH_MMCRGUFCCR)**

Address offset: 0x01C4
Reset value: 0x0000 0000
This register contains the number of good unicast frames received.

| RGUFC |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
| r |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bits 31:0 **RGUFC**: Received good unicast frames counter


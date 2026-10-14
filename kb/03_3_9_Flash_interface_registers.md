**3.9 Flash interface registers**

**3.9.1 Flash access control register (FLASH_ACR) for STM32F405xx/07xx and STM32F415xx/17xx**

The Flash access control register is used to enable/disable the acceleration features and

control the Flash memory access time according to CPU frequency. Address offset: 0x00

Reset value: 0x0000 0000

Access: no wait state, word, half-word and byte access

31 30 29 28 27 26 25 24

23 22 21 20 19 18 17 16 Reserved

15 14 13 12 11 10 9 8

7 6 5 4 3 2 1 0

| Reserved | DCRST | ICRST | DCEN | ICEN | PRFTEN | Reserved | LATENCY[2:0] |
|---|---|---|---|---|---|---|---|
| rw | w | rw |  |  |  |  |  |


Bits 31:13 Reserved, must be kept cleared.

Bit 12 DCRST: Data cache reset

0: Data cache is not reset

1: Data cache is reset

This bit can be written only when the D cache is disabled. Bit 11 ICRST: Instruction cache reset

0: Instruction cache is not reset

1: Instruction cache is reset

This bit can be written only when the I cache is disabled. Bit 10 DCEN: Data cache enable

0: Data cache is disabled

1: Data cache is enabled

Bit 9 ICEN: Instruction cache enable

0: Instruction cache is disabled

1: Instruction cache is enabled

Bit 8 PRFTEN: Prefetch enable

0: Prefetch is disabled

1: Prefetch is enabled

Bits 7:3 Reserved, must be kept cleared.

Bits 2:0 LATENCY[2:0]: Latency

These bits represent the ratio of the CPU clock period to the Flash memory access time.

000: Zero wait state

001: One wait state

010: Two wait states

011: Three wait states

100: Four wait states

101: Five wait states

110: Six wait states

111: Seven wait states



--- Page 99 ---

**RM0090**

**Embedded Flash memory interface**

**3.9.2 Flash access control register (FLASH_ACR) for STM32F42xxx and STM32F43xxx**

The Flash access control register is used to enable/disable the acceleration features and

control the Flash memory access time according to CPU frequency. Address offset: 0x00

Reset value: 0x0000 0000

Access: no wait state, word, half-word and byte access

31 30 29 28 27 26 25 24

23 22 21 20 19 18 17 16 Reserved

15 14 13 12 11 10 9 8

7 6 5 4 3 2 1 0

| Reserved | DCRST | ICRST | DCEN | ICEN | PRFTEN | Reserved | LATENCY[3:0] |
|---|---|---|---|---|---|---|---|
| rw | w | rw |  |  |  |  |  |


Bits 31:11 Reserved, must be kept cleared.

Bit 12 DCRST: Data cache reset

0: Data cache is not reset

1: Data cache is reset

This bit can be written only when the D cache is disabled. Bit 11 ICRST: Instruction cache reset

0: Instruction cache is not reset

1: Instruction cache is reset

This bit can be written only when the I cache is disabled. Bit 10 DCEN: Data cache enable

0: Data cache is disabled

1: Data cache is enabled

Bit 9 ICEN: Instruction cache enable

0: Instruction cache is disabled

1: Instruction cache is enabled

Bit 8 PRFTEN: Prefetch enable

0: Prefetch is disabled

1: Prefetch is enabled

Bits 7:4 Reserved, must be kept cleared.

Bits 3:0 LATENCY[3:0]: Latency

These bits represent the ratio of the CPU clock period to the Flash memory access time.

0000: Zero wait state

0001: One wait state

0010: Two wait states

...

1110: Fourteen wait states

1111: Fifteen wait states



112

--- Page 100 ---

**Embedded Flash memory interface**

**RM0090**

**3.9.3 Flash key register (FLASH_KEYR)**

The Flash key register is used to allow access to the Flash control register and so, to allow

program and erase operations.

Address offset: 0x04

Reset value: 0x0000 0000

Access: no wait state, word access

31 30 29 28 27 26 25 24

23 22 21 20 19 18 17 16

| KEY[31:16] |
|---|
| w |


15 14 13 12 11 10 9 8

7 6 5 4 3 2 1 0

| KEY[15:0] |
|---|
| w |


Bits 31:0 FKEYR[31:0] : FPEC key

The following values must be programmed consecutively to unlock the FLASH_CR register

and allow programming/erasing it:

a) KEY1 = 0x45670123

b) KEY2 = 0xCDEF89AB

**3.9.4 Flash option key register (FLASH_OPTKEYR)**

The Flash option key register is used to allow program and erase operations in the user

configuration sector.

Address offset: 0x08

Reset value: 0x0000 0000

Access: no wait state, word access

31 30 29 28 27 26 25 24

23 22 21 20 19 18 17 16

| OPTKEYR[31:16 |
|---|
| w |


15 14 13 12 11 10 9 8

7 6 5 4 3 2 1 0

| OPTKEYR[15:0] |
|---|
| w |


Bits 31:0 OPTKEYR[31:0] : Option byte key

The following values must be programmed consecutively to unlock the FLASH_OPTCR

register and allow programming it:

a) OPTKEY1 = 0x08192A3B

b) OPTKEY2 = 0x4C5D6E7F



--- Page 101 ---

**RM0090**

**Embedded Flash memory interface**

**3.9.5 Flash status register (FLASH_SR) for STM32F405xx/07xx and STM32F415xx/17xx**

The Flash status register gives information on ongoing program and erase operations.

Address offset: 0x0C

Reset value: 0x0000 0000

Access: no wait state, word, half-word and byte access

31 30 29 28 27 26 25 24 23

22 21 20 19 18 17 16

| Reserved | BSY |
|---|---|
| r |  |


15 14 13 12 11 10 9 8 7

6 5 4 3 2 1 0

| Reserved | PGSERR | PGPERR | PGAERR | WRPERR | Reserved | OPERR | EOP |
|---|---|---|---|---|---|---|---|
| rc w1 _ |  |  |  |  |  |  |  |


Bits 31:17 Reserved, must be kept cleared.

Bit 16 BSY: Busy

This bit indicates that a Flash memory operation is in progress. It is set at the beginning of a Flash memory operation and cleared when the operation finishes or an error occurs.

0: no Flash memory operation ongoing 1: Flash memory operation ongoing Bits 15:8 Reserved, must be kept cleared.

Bit 7 PGSERR: Programming sequence error

Set by hardware when a write access to the Flash memory is performed by the code while the

control register has not been correctly configured. Cleared by writing 1.

Bit 6 PGPERR: Programming parallelism error

Set by hardware when the size of the access (byte, half-word, word, double word) during the program sequence does not correspond to the parallelism configuration PSIZE (x8, x16, x32,

x64).

Cleared by writing 1.

Bit 5 PGAERR: Programming alignment error

Set by hardware when the data to program cannot be contained in the same 128-bit Flash

memory row.

Cleared by writing 1.

Bit 4 WRPERR: Write protection error

Set by hardware when an address to be erased/programmed belongs to a write-protected part

of the Flash memory.

Cleared by writing 1.



112

--- Page 102 ---

**Embedded Flash memory interface**

**RM0090**

Bits 3:2 Reserved, must be kept cleared.

Bit 1 OPERR: Operation error

Set by hardware when a flash operation (programming / erase /read) request is detected and can not be run because of parallelism, alignment, or write protection error. This bit is set only if

error interrupts are enabled (ERRIE = 1).

Bit 0 EOP: End of operation

Set by hardware when one or more Flash memory operations (program/erase) has/have completed successfully. It is set only if the end of operation interrupts are enabled (EOPIE = 1).

Cleared by writing a 1.

**3.9.6 Flash status register (FLASH_SR) for STM32F42xxx and STM32F43xxx**

The Flash status register gives information on ongoing program and erase operations.

Address offset: 0x0C

Reset value: 0x0000 0000

Access: no wait state, word, half-word and byte access

31 30 29 28 27 26 25 24 23

22 21 20 19 18 17 16

| Reserved | BSY |
|---|---|
| r |  |


15 14 13 12 11 10 9 8 7

6 5 4 3 2 1 0

| Reserved | RDERR | PGSERR | PGPERR | PGAERR | WRPERR | Reserved | OPERR | EOP |
|---|---|---|---|---|---|---|---|---|
| rc w1 _ |  |  |  |  |  |  |  |  |


Bits 31:17 Reserved, must be kept cleared.

Bit 16 BSY: Busy

This bit indicates that a Flash memory operation is in progress to/from one bank. It is set at the beginning of a Flash memory operation and cleared when the operation finishes or an error

occurs.

0: no Flash memory operation ongoing 1: Flash memory operation ongoing Bits 15:9 Reserved, must be kept cleared.

Bit 8 RDERR: Proprietary readout protection (PCROP) error

Set by hardware when a read access through the D-bus is performed to an address belonging

to a proprietary readout protected Flash sector. Cleared by writing 1.

Bit 7 PGSERR: Programming sequence error

Set by hardware when a write access to the Flash memory is performed by the code while the

control register has not been correctly configured. Cleared by writing 1.

102/1749

RM0090 Rev 18

--- Page 103 ---

**RM0090 Embedded Flash memory interface**

Bit 6 PGPERR: Programming parallelism error

Set by hardware when the size of the access (byte, half-word, word, double word) during the program sequence does not correspond to the parallelism configuration PSIZE (x8, x16, x32, x64).

Cleared by writing 1.

Bit 5 PGAERR: Programming alignment error

Set by hardware when the data to program cannot be contained in the same 128-bit Flash memory row.

Cleared by writing 1.

Bit 4 WRPERR: Write protection error

Set by hardware when an address to be erased/programmed belongs to a write-protected part of the Flash memory.

Cleared by writing 1.

Bits 3:2 Reserved, must be kept cleared.

Bit 1 OPERR: Operation error

Set by hardware when a flash operation (programming/erase/read) request is detected and can not be run because of parallelism, alignment, write or read (PCROP) protection error. This bit is set only if error interrupts are enabled (ERRIE = 1).

Bit 0 EOP: End of operation

Set by hardware when one or more Flash memory operations (program/erase) has/have completed successfully. It is set only if the end of operation interrupts are enabled (EOPIE = 1). Cleared by writing a 1.

**3.9.7 Flash control register (FLASH_CR) for**

**STM32F405xx/07xx and STM32F415xx/17xx**

The Flash control register is used to configure and start Flash memory operations.

Address offset: 0x10

Reset value: 0x8000 0000

Access: no wait state when no Flash memory operation is ongoing, word, half-word and byte access.

31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16

| LOCK | Reserved | ERRIE | EOPIE | Reserved | STRT |
|---|---|---|---|---|---|
| rs | rw | rs |  |  |  |


15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0

| Reserved | PSIZE[1:0] | Res. | SNB[3:0] | MER | SER | PG |
|---|---|---|---|---|---|---|
| rw |  |  |  |  |  |  |


RM0090 Rev 18 103/1749

112

--- Page 104 ---

**Embedded Flash memory interface**

**RM0090**

Bit 31 LOCK: Lock

Write to 1 only. When it is set, this bit indicates that the FLASH_CR register is locked. It is

cleared by hardware after detecting the unlock sequence.

In the event of an unsuccessful unlock operation, this bit remains set until the next reset.

Bits 30:26 Reserved, must be kept cleared.

Bit 25 ERRIE: Error interrupt enable

This bit enables the interrupt generation when the OPERR bit in the FLASH_SR register is set

to 1.

0: Error interrupt generation disabled 1: Error interrupt generation enabled Bit 24 EOPIE: End of operation interrupt enable

This bit enables the interrupt generation when the EOP bit in the FLASH_SR register goes to

1.

0: Interrupt generation disabled

1: Interrupt generation enabled

Bits 23:17 Reserved, must be kept cleared.

Bit 16 STRT: Start

This bit triggers an erase operation when set. It is set only by software and cleared when the

BSY bit is cleared.

Bits 15:10 Reserved, must be kept cleared.

Bits 9:8 PSIZE[1:0]: Program size

These bits select the program parallelism.

00 program x8

01 program x16

10 program x32

11 program x64

Bit 7 Reserved, must be kept cleared.

Bits 6:3 SNB[3:0]: Sector number

These bits select the sector to erase.

0000 sector 0

0001 sector 1

...

1011 sector 11

Others not allowed

Bit 2 MER: Mass Erase

Erase activated for all user sectors.

Bit 1 SER: Sector Erase

Sector Erase activated.

Bit 0 PG: Programming

Flash programming activated.

104/1749

RM0090 Rev 18

--- Page 105 ---

**RM0090**

**Embedded Flash memory interface**

**3.9.8 Flash control register (FLASH_CR) for STM32F42xxx and STM32F43xxx**

The Flash control register is used to configure and start Flash memory operations.

Address offset: 0x10

Reset value: 0x8000 0000

Access: no wait state when no Flash memory operation is ongoing, word, half-word and

byte access.

31 30 29 28 27 26 25 24

23 22 21 20 19 18 17 16

| LOCK | Reserved | ERRIE | EOPIE | Reserved | STRT |
|---|---|---|---|---|---|
| rs | rw | rs |  |  |  |


15 14 13 12 11 10 9 8

7 6 5 4 3 2 1 0

| MER1 | Reserved | PSIZE[1:0] | SNB[4:0] | MER | SER | PG |
|---|---|---|---|---|---|---|
| rw |  |  |  |  |  |  |


Bit 31 LOCK: Lock

Write to 1 only. When it is set, this bit indicates that the FLASH_CR register is locked. It is

cleared by hardware after detecting the unlock sequence.

In the event of an unsuccessful unlock operation, this bit remains set until the next reset.

Bits 30:26 Reserved, must be kept cleared.

Bit 25 ERRIE: Error interrupt enable

This bit enables the interrupt generation when the OPERR bit in the FLASH_SR register is set

to 1.

0: Error interrupt generation disabled 1: Error interrupt generation enabled Bit 24 EOPIE: End of operation interrupt enable

This bit enables the interrupt generation when the EOP bit in the FLASH_SR register goes to

1.

0: Interrupt generation disabled

1: Interrupt generation enabled

Bits 23:17 Reserved, must be kept cleared.

Bit 16 STRT: Start

This bit triggers an erase operation when set. It is set only by software and cleared when the

BSY bit is cleared.

Bit 15 MER1: Mass Erase of bank 2 sectors

Erase activated for bank 2 user sectors 12 to 23. Bits 14:10 Reserved, must be kept cleared.

Bits 9:8 PSIZE[1:0]: Program size

These bits select the program parallelism.

00 program x8

01 program x16

10 program x32

11 program x64



112

--- Page 106 ---

**Embedded Flash memory interface**

**RM0090**

Bits 7:3 SNB[3:0]: Sector number

These bits select the sector to erase.

0000: sector 0

0001: sector 1

...

01011: sector 11

01100: not allowed

01101: not allowed

01110: not allowed

01111: not allowed

10000: section 12

10001: section 13

...

11011 sector 23

11100: not allowed

11101: not allowed

11110: not allowed

11111: not allowed

Bit 2 MER: Mass Erase of bank 1 sectors

Erase activated of bank 1 sectors.

Bit 1 SER: Sector Erase

Sector Erase activated.

Bit 0 PG: Programming

Flash programming activated.

**3.9.9 Flash option control register (FLASH_OPTCR) for STM32F405xx/07xx and STM32F415xx/17xx**

The FLASH_OPTCR register is used to modify the user option bytes. Address offset: 0x14

Reset value: 0x0FFF AAED. The option bits are loaded with values from Flash memory at

reset release.

Access: no wait state when no Flash memory operation is ongoing, word, half-word and

byte access.

31 30 29 28 27 26 25 24

23 22 21 20 19 18 17 16

| Reserved | nWRP[11:0] |
|---|---|
| rw |  |


15 14 13 12 11 10 9 8

7 6 5 4 3 2 1 0

| RDP[7:0] | nRST _ STDBY | nRST _ STOP | WDG _ SW | Reserve d | BOR LEV _ | OPTST RT | OPTLO CK |
|---|---|---|---|---|---|---|---|
| rw | rs |  |  |  |  |  |  |


106/1749

RM0090 Rev 18

--- Page 107 ---

**RM0090 Embedded Flash memory interface**

Bits 31:28 Reserved, must be kept cleared.

Bits 27:16 nWRP[11:0]: Not write protect

These bits contain the value of the write-protection option bytes after reset. They can be written to program a new write protect value into Flash memory.

0: Write protection active on selected sector

1: Write protection inactive on selected sector

Bits 15:8 RDP[7:0]: Read protect

These bits contain the value of the read-protection option level after reset. They can be written to program a new read protection value into Flash memory.

0xAA: Level 0, read protection not active

0xCC: Level 2, chip read protection active

Others: Level 1, read protection of memories active

Bits 7:5 USER[2:0]: User option bytes

These bits contain the value of the user option byte after reset. They can be written to program a new user option byte value into Flash memory.

Bit 7: nRST_STDBY

Bit 6: nRST_STOP

Bit 5: WDG_SW

Note: When changing the WDG mode from hardware to software or from software to

hardware, a system reset is required to make the change effective.

Bit 4 Reserved, must be kept cleared. Always read as “0”.

Bits 3:2 BOR_LEV[1:0]: BOR reset Level

These bits contain the supply level threshold that activates/releases the reset. They can be written to program a new BOR level. By default, BOR is off. When the supply voltage (V DD ) drops below the selected BOR level, a device reset is generated.

00: BOR Level 3 (VBOR3), brownout threshold level 3

01: BOR Level 2 (VBOR2), brownout threshold level 2

10: BOR Level 1 (VBOR1), brownout threshold level 1

11: BOR off, POR/PDR reset threshold level is applied

Note: For full details about BOR characteristics, refer to the “Electrical characteristics” section

in the device datasheet.

Bit 1 OPTSTRT: Option start

This bit triggers a user option operation when set. It is set only by software and cleared when the BSY bit is cleared.

Bit 0 OPTLOCK: Option lock

Write to 1 only. When this bit is set, it indicates that the FLASH_OPTCR register is locked. This bit is cleared by hardware after detecting the unlock sequence.

In the event of an unsuccessful unlock operation, this bit remains set until the next reset.

RM0090 Rev 18 107/1749

112

--- Page 108 ---

**Embedded Flash memory interface RM0090**

**3.9.10 Flash option control register (FLASH_OPTCR)**

**for STM32F42xxx and STM32F43xxx**

The FLASH_OPTCR register is used to modify the user option bytes.

Address offset: 0x14

Reset value: 0x0FFF AAED. The option bits are loaded with values from Flash memory at reset release.

Access: no wait state when no Flash memory operation is ongoing, word, half-word and byte access.

31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16

| SPR MOD | DB1M | Reserved | nWRP[11:0] |
|---|---|---|---|
| rw |  |  |  |


15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0

| RDP[7:0] | nRST _ STDBY | nRST _ STOP | WDG _ SW | BFB2 | BOR LEV _ | OPTST RT | OPTLO CK |
|---|---|---|---|---|---|---|---|
| rw | rs |  |  |  |  |  |  |


Bit 31 SPRMOD : Selection of protection mode for nWPRi bits

0: PCROP disabled. nWPRi bits used for Write protection on sector i.

1: PCROP enabled. nWPRi bits used for PCROP protection on sector i Bit 30 DB1M : Dual-bank on 1 Mbyte Flash memory devices

0: 1 Mbyte single bank Flash memory (contiguous addresses in bank1)

1: 1 Mbyte dual bank Flash memory. The Flash memory is organized as two banks of 512 Kbytes each (see Table 7: 1 Mbyte Flash memory single bank vs dual bank organization (STM32F42xxx and STM32F43xxx) and Table 9: 1 Mbyte dual bank Flash memory organization (STM32F42xxx and STM32F43xxx) ). To perform an erase operation, the right sector must be programmed (see Table 7 for information on the sector numbering scheme). Note: If DB1M is set and an erase operation is performed on Bank 2 while the default sector

number is selected (as an example, sector 8 is configured instead of sector 12), the erase operation on Bank 2 sector is not performed.

Bits 29:28 Reserved, must be kept cleared.

Bits 27:16 nWRP[11:0]: Not write protect

These bits contain the value of the write-protection and read-protection (PCROP) option bytes for sectors 0 to 11 after reset. They can be written to program a new write-protect or PCROP value into Flash memory.

If SPRMOD is reset:

0: Write protection active on sector i

1: Write protection not active on sector i

If SPRMOD is set:

0: PCROP protection not active on sector i

1: PCROP protection active on sector i

Bits 15:8 RDP[7:0]: Read protect

These bits contain the value of the read-protection option level after reset. They can be written to program a new read protection value into Flash memory.

0xAA: Level 0, read protection not active

0xCC: Level 2, chip read protection active

Others: Level 1, read protection of memories active



--- Page 109 ---

**RM0090 Embedded Flash memory interface**

Bits 7:5 USER: User option bytes

These bits contain the value of the user option byte after reset. They can be written to program a new user option byte value into Flash memory.

Bit 7: nRST_STDBY

Bit 6: nRST_STOP

Bit 5: WDG_SW

Note: When changing the WDG mode from hardware to software or from software to

hardware, a system reset is required to make the change effective.

Bit 4 BFB2 : Dual-bank Boot option byte

0: Dual-bank boot disabled. Boot can be performed either from Flash memory bank 1 or from system memory depending on boot pin state (default)

1: Dual-bank boot enabled. Boot is always performed from system memory.

Note: For STM32F42xx and STM32F43xx 1MB part numbers, this option bit must be kept

cleared when DB1M=0.

Bits 3:2 BOR_LEV: BOR reset Level

These bits contain the supply level threshold that activates/releases the reset. They can be written to program a new BOR level. By default, BOR is off. When the supply voltage (V DD ) drops below the selected BOR level, a device reset is generated.

00: BOR Level 3 (VBOR3), brownout threshold level 3

01: BOR Level 2 (VBOR2), brownout threshold level 2

10: BOR Level 1 (VBOR1), brownout threshold level 1

11: BOR off, POR/PDR reset threshold level is applied

Note: For full details on BOR characteristics, refer to the “Electrical characteristics” section of

the product datasheet.

Bit 1 OPTSTRT: Option start

This bit triggers a user option operation when set. It is set only by software and cleared when the BSY bit is cleared.

Bit 0 OPTLOCK: Option lock

Write to 1 only. When this bit is set, it indicates that the FLASH_OPTCR register is locked. This bit is cleared by hardware after detecting the unlock sequence.

In the event of an unsuccessful unlock operation, this bit remains set until the next reset.

RM0090 Rev 18 109/1749

112

--- Page 110 ---

**Embedded Flash memory interface**

**RM0090**

**3.9.11 Flash option control register (FLASH_OPTCR1) for STM32F42xxx and STM32F43xxx**

This register is available only on STM32F42xxx and STM32F43xxx.

The FLASH_OPTCR1 register is used to modify the user option bytes for bank 2.

Address offset: 0x18

Reset value: 0x0FFF 0000. The option bits are loaded with values from Flash memory at

reset release.

Access: no wait state when no Flash memory operation is ongoing, word, half-word and

byte access.

31 30 29 28 27 26 25 24

23 22 21 20 19 18 17 16

| Reserved | nWRP[11:0] |
|---|---|
| rw |  |


15 14 13 12 11 10 9 8

7 6 5 4 3 2 1 0 Reserved

Bits 31:28 Reserved, must be kept cleared.

Bits 27:16 nWRP[11:0]: Not write protect

These bits contain the value of the write-protection and read-protection (PCROP) option bytes for sectors 0 to 11 after reset. They can be written to program a new write-protect or PCROP

value into Flash memory.

If SPRMOD is reset (default value):

0: Write protection active on sector i.

1: Write protection not active on sector i.

If SPRMOD is set:

0: PCROP protection not active on sector i.

1: PCROP protection active on sector i.

Bits 15:0 Reserved, must be kept cleared.



--- Page 111 ---

**RM0090 Embedded Flash memory interface**

**3.9.12 Flash interface register map**

**Table 19. Flash register map and reset values**

**(STM32F405xx/07xx and STM32F415xx/17xx)**

| Offset | Register | 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 0x00 | FLASH ACR _ | Reserved | DCRST | ICRST | DCEN | ICEN | PRFTEN | Reserved | LATENCY [2:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x04 | FLASH _ KEYR | KEY[31:16] | KEY[15:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x08 | FLASH OPT _ KEYR | OPTKEYR[31:16] | OPTKEYR[15:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x0C | FLASH SR _ | Reserved | BSY | Reserved | PGSERR | PGPERR | PGAERR | WRPERR | Reserved | OPERR | EOP |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x10 | FLASH CR _ | LOCK | Reserved | EOPIE | Reserved | STRT | Reserved | PSIZE[1:0] | Reserved | SNB[3:0] | MER | SER | PG |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 1 | 0 |  | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x14 | FLASH _ OPTCR | Reserved | nWRP[11:0] | RDP[7:0] | nRST_STDBY | nRST_STOP | WDG_SW | Reserved | BOR_LEV[1:0] | OPTSTRT | OPTLOCK |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 1 | 0 | 1 | 0 | 1 | 0 | 1 | 0 | 1 | 0 | 1 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


**Table 20. Flash register map and reset values (STM32F42xxx and STM32F43xxx)**

| Offset | Register | 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 0x00 | FLASH ACR _ | Reserved | DCRST | ICRST | DCEN | ICEN | PRFTEN | Reserved | LATENCY[3:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x04 | FLASH KEYR _ | KEY[31:16] | KEY[15:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x08 | FLASH _ OPTKEYR | OPTKEYR[31:16] | OPTKEYR[15:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x0C | FLASH SR _ | Reserved | BSY | Reserved | RDERR | PGSERR | PGPERR | PGAERR | WRPERR | Reserved | OPERR | EOP |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |




112

--- Page 112 ---

**Embedded Flash memory interface RM0090**

**Table 20. Flash register map and reset values (STM32F42xxx and STM32F43xxx) (continued)**

| Offset | Register | 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 0x10 | FLASH CR _ | LOCK | Reserved | EOPIE | Reserved | STRT | MER1 | Reserved | PSIZE[1:0] | SNB[4:0] | MER | SER | PG |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 1 | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x14 | FLASH OPTCR _ | SPRMOD | DB1M | Reserved | nWRP[11:0] | RDP[7:0] | nRST_STDBY | nRST_STOP | WDG_SW | BFB2 | BOR_LEV[1:0] | OPTSTRT | OPTLOCK |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 | 1 | 0 | 1 | 0 | 1 | 0 | 1 | 0 | 1 | 0 | 1 | 0 | 1 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x18 | FLASH _ OPTCR1 | Reserved | nWRP[11:0] | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 1 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |




--- Page 113 ---

**RM0090 CRC calculation unit**


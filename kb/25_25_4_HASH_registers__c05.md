> [RM0090 ch.25 §25.4] HASH registers


### HASH_HR2

Address offset: 0x14 and 0x318

| H2 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 |
| r |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


790/1749
RM0090 Rev 18

--- Page 11 ---

RM0090
Hash processor (HASH)

| H2 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
| r |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


**HASH_HR3**
Address offset: 0x18 and 0x31C

| H3 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 |
| r |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


| H3 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
| r |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


**HASH_HR4**
Address offset: 0x1C and 0x320

| H4 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 |
| r |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


| H4 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
| r |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


**HASH_HR5**
Address offset: 0x324

| H5 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 |
| r |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


| H5 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
| r |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


**HASH_HR6**
Address offset: 0x328

| H6 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 |
| r |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


| H6 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
| r |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


**HASH_HR7**
Address offset: 0x32C

ST
RM0090 Rev 18
791/1749

--- Page 12 ---

Hash processor (HASH)
RM0090

| 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 |
|---|
| H7 |
| r |
| 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0 |
| H7 |
| r |


Note: When starting a digest computation for a new bit stream (by writing the INIT bit to 1), these registers assume their reset values.

> [RM0090 ch.25 §25.4] HASH registers

### 25.4.6 HASH interrupt enable register (HASH_IMR)**

**25.4.6 HASH interrupt enable register (HASH_IMR)**

Address offset: 0x20
Reset value: 0x0000 0000

| 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reserved | DCIE | DINIE |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  | rw |


Bits 31:2 Reserved, forced by hardware to 0.

Bit 1 **DCIE**: Digest calculation completion interrupt enable
0: Digest calculation completion interrupt disabled
1: Digest calculation completion interrupt enabled.

Bit 0 **DINIE**: Data input interrupt enable
0: Data input interrupt disabled
1: Data input interrupt enabled

792/1749
RM0090 Rev 18
ST

--- Page 13 ---

RM0090
Hash processor (HASH)

> [RM0090 ch.25 §25.4] HASH registers

### 25.4.7 HASH status register (HASH_SR)

### 25.4.7 HASH status register (HASH_SR)

Address offset: 0x24
Reset value: 0x0000 0001

| 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 15 14 13 12 11 10 9 8 7 6 5 4 | 3 | 2 | 1 | 0 |  |  |  |  |  |  |  |  |  |
| Reserved | BUSY | DMAS | DCIS | DINIS |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  | r | rc_w0 |


Bits 31:4 Reserved, forced by hardware to 0.

**Bit 3 BUSY:** Busy bit
0: No block is currently being processed
1: The hash core is processing a block of data

**Bit 2 DMAS:** DMA Status
This bit provides information on the DMA interface activity. It is set with DMAE and cleared when DMAE=0 and no DMA transfer is ongoing. No interrupt is associated with this bit.
0: DMA interface is disabled (DMAE=0) and no transfer is ongoing
1: DMA interface is enabled (DMAE=1) or a transfer is ongoing

**Bit 1 DCIS:** Digest calculation completion interrupt status
This bit is set by hardware when a digest becomes ready (the whole message has been processed). It is cleared by writing it to 0 or by writing the INIT bit to 1 in the HASH_CR register.
0: No digest available in the HASH_Hx registers
1: Digest calculation complete, a digest is available in the HASH_Hx registers. An interrupt is generated if the DCIE bit is set in the HASH_IMR register.

**Bit 0 DINIS:** Data input interrupt status
This bit is set by hardware when the input buffer is ready to get a new block (16 locations are free). It is cleared by writing it to 0 or by writing the HASH_DIN register.
0: Less than 16 locations are free in the input buffer
1: A new block can be entered into the input buffer. An interrupt is generated if the DINIE bit is set in the HASH_IMR register.

RM0090 Rev 18
793/1749

--- Page 14 ---

Hash processor (HASH)
RM0090

> [RM0090 ch.25 §25.4] HASH registers

### 25.4.8 HASH context swap registers (HASH_CSRx)

## 25.4.8 HASH context swap registers (HASH_CSRx)

Address offset: 0x0F8 to 0x1C0

* For HASH_CSR0 register: Reset value is 0x0000 0002.
* For others registers: Reset value is 0x0000 0000 , except for STM32F43xxx devices where the HASH_CSR2 register reset value is 0x2000 0000

Additional registers are available from 0x1C1 to 0x1CC on STM32F43xxx:

* Reset value: 0x0000 0000.

These registers contain the complete internal register states of the hash processor, and are useful when a context swap has to be done because a high-priority task has to use the hash processor while it is already in use by another task.

When such an event occurs, the HASH_CSRx registers have to be read and the read values have to be saved somewhere in the system memory space. Then the hash processor can be used by the preemptive task, and when hash computation is finished, the saved context can be read from memory and written back into these HASH_CSRx registers.


> [RM0090 ch.23 §23.4] CRYP interrupts


### 23.6.5 CRYP data output register (CRYP_DOUT)

Address offset: 0x0C
Reset value: 0x0000 0000

The CRYP_DOUT register is the data output register. It is read-only and 32-bit wide. It is used to retrieve up to four 64-bit (TDES mode) or two 128-bit (AES mode) ciphertext (when encrypting) or plaintext (when decrypting) blocks from the output FIFO, one 32-bit word at a time.

Like for the input data, the MSB of the output block is the first word read from the output FIFO. The LSB of the output block is read at the end. Disregarding data swapping, this gives:

*   In the DES/TDES modes: Bit 1 (leftmost bit) corresponds to the MSB (bit 31) of the first word read from the FIFO, bit 64 (rightmost bit) corresponds to the LSB (bit 0) of the second word read from the FIFO.
*   In the AES mode: Bit 0 (leftmost bit) corresponds to the MSB (bit 31) of the first word read from the FIFO, bit 127 (rightmost bit) corresponds to the LSB (bit 0) of the 4th word read from the FIFO.

To fit different data sizes, the data can be swapped after processing by configuring the DATATYPE bits in the CRYP_CR register. Refer to Section 23.3.3: Data type on page 739 for more details.

When CRYP_DOUT register is read, the last data entered into the output FIFO (pointed to by the read pointer) is returned.

| DATAOUT |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 |
| r |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


| DATAOUT |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
| r |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bits 31:0 **DATAOUT**: Data output
Read = returns output FIFO content.
Write = No effect.

STI
RM0090 Rev 18
755/1749

--- Page 171 ---

Cryptographic processor (CRYP)
RM0090

> [RM0090 ch.23 §23.4] CRYP interrupts

### 23.6.6 CRYP DMA control register (CRYP_DMACR)

### 23.6.6 CRYP DMA control register (CRYP_DMACR)

Address offset: 0x10
Reset value: 0x0000 0000

| 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 15 14 13 12 11 10 9 8 7 6 5 4 3 2 | 1 | 0 |  |  |  |  |  |  |  |  |  |  |  |  |
| Reserved | DOEN | DIEN |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  | rw |


Bits 31:2 Reserved, must be kept at reset value
Bit 1 **DOEN**: DMA output enable
0: DMA for outgoing data transfer is disabled
1: DMA for outgoing data transfer is enabled
Bit 0 **DIEN**: DMA input enable
0: DMA for incoming data transfer is disabled
1: DMA for incoming data transfer is enabled

> [RM0090 ch.23 §23.4] CRYP interrupts

### 23.6.7 CRYP interrupt mask set/clear register (CRYP_IMSCR)

### 23.6.7 CRYP interrupt mask set/clear register (CRYP_IMSCR)

Address offset: 0x14
Reset value: 0x0000 0000

The CRYP_IMSCR register is the interrupt mask set or clear register. It is a read/write register. On a read operation, this register gives the current value of the mask on the relevant interrupt. Writing 1 to the particular bit sets the mask, enabling the interrupt to be read. Writing 0 to this bit clears the corresponding mask. All the bits are cleared to 0 when the peripheral is reset.

| 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 15 14 13 12 11 10 9 8 7 6 5 4 3 2 | 1 | 0 |  |  |  |  |  |  |  |  |  |  |  |  |
| Reserved | OUTIM | INIM |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  | rw |


Bits 31:2 Reserved, must be kept at reset value
Bit 1 **OUTIM**: Output FIFO service interrupt mask
0: Output FIFO service interrupt is masked
1: Output FIFO service interrupt is not masked
Bit 0 **INIM**: Input FIFO service interrupt mask
0: Input FIFO service interrupt is masked
1: Input FIFO service interrupt is not masked

756/1749
RM0090 Rev 18
ST

--- Page 172 ---

RM0090
Cryptographic processor (CRYP)

> [RM0090 ch.23 §23.4] CRYP interrupts

### 23.6.8 CRYP raw interrupt status register (CRYP_RISR)

### 23.6.8 CRYP raw interrupt status register (CRYP_RISR)

Address offset: 0x18
Reset value: 0x0000 0001

The CRYP_RISR register is the raw interrupt status register. It is a read-only register. On a read, this register gives the current raw status of the corresponding interrupt prior to masking. A write has no effect.

| 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 15 14 13 12 11 10 9 8 7 6 5 4 3 2 | 1 | 0 |  |  |  |  |  |  |  |  |  |  |  |  |
| Reserved | OUTRIS | INRIS |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  | r |


Bits 31:2 Reserved, must be kept at reset value

Bit 1 **OUTRIS**: Output FIFO service raw interrupt status
Gives the raw interrupt state prior to masking of the output FIFO service interrupt.
0: Raw interrupt not pending
1: Raw interrupt pending

Bit 0 **INRIS**: Input FIFO service raw interrupt status
Gives the raw interrupt state prior to masking of the Input FIFO service interrupt.
0: Raw interrupt not pending
1: Raw interrupt pending

> [RM0090 ch.23 §23.4] CRYP interrupts

### 23.6.9 CRYP masked interrupt status register (CRYP_MISR)

### 23.6.9 CRYP masked interrupt status register (CRYP_MISR)

Address offset: 0x1C
Reset value: 0x0000 0000

The CRYP_MISR register is the masked interrupt status register. It is a read-only register. On a read, this register gives the current masked status of the corresponding interrupt prior to masking. A write has no effect.

| 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 15 14 13 12 11 10 9 8 7 6 5 4 3 2 | 1 | 0 |  |  |  |  |  |  |  |  |  |  |  |  |
| Reserved | OUTMIS | INMIS |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  | r |


STI
RM0090 Rev 18
757/1749

--- Page 173 ---

**Cryptographic processor (CRYP)**
RM0090

Bits 31:2 Reserved, must be kept at reset value

Bit 1 **OUTMIS**: Output FIFO service masked interrupt status
Gives the interrupt state after masking of the output FIFO service interrupt.
0: Interrupt not pending
1: Interrupt pending

Bit 0 **INMIS**: Input FIFO service masked interrupt status
Gives the interrupt state after masking of the input FIFO service interrupt.
0: Interrupt not pending
1: Interrupt pending when CRY PEN = 1


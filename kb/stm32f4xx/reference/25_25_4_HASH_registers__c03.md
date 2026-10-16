> [RM0090 ch.25 §25.4] HASH registers


### 25.4.2 HASH control register (HASH_CR) for STM32F43xxx

Address offset: 0x00
Reset value: 0x0000 0000

| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved |  |  |  |  |  |  |  |  |  |  |  | ALGO[1] | Reserved | LKEY |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | rw |  | rw |
| 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |  |  |  |  |  |  |  |  |  |  |  |
| Reserved | MDMAT | DINNE | NBW | ALGO[0] | MODE | DATATYPE | DMAE | INIT | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  | rw | r | rw | w |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bits 31:19 Reserved, forced by hardware to 0.
Bit 17 Reserved, forced by hardware to 0.
Bit 16 **LKEY**: Long key selection
This bit selects between short key (≤64 bytes) or long key (> 64 bytes) in HMAC mode
0: Short key (≤64 bytes)
1: Long key (> 64 bytes)
*Note: This selection is only taken into account when the INIT bit is set and MODE = 1. Changing this bit during a computation has no effect.*

Bits 15:14 Reserved, forced by hardware to 0.
Bit 13 **MDMAT**: Multiple DMA Transfers
This bit is set when hashing large files when multiple DMA transfers are needed.
0: DCAL is automatically set at the end of a DMA transfer.
1: DCAL is not automatically set at the end of a DMA transfer.

Bit 12 **DINNE**: DIN not empty
This bit is set when the HASH_DIN register holds valid data (that is after being written at least once). It is cleared when either the INIT bit (initialization) or the DCAL bit (completion of the previous message processing) is written to 1.
0: No data are present in the data input buffer
1: The input buffer contains at least one word of data

RM0090 Rev 18
785/1749

--- Page 6 ---

**Hash processor (HASH)**

RM0090

Bits 11:8 **NBW**: Number of words already pushed

This bitfield reflects the number of words in the message that have already been pushed into the IN FIFO.
NBW increments (+1) when a write access is performed to the HASH_DIN register while DINNE = 1.
It goes to 0000 when the INIT bit is written to 1 or when a digest calculation starts (DCAL written to 1 or DMA end of transfer).

" If the DMA is not used:
0000 and DINNE=0: no word has been pushed into the DIN buffer (the buffer is empty, both the HASH_DIN register and the IN FIFO are empty)
0000 and DINNE=1: 1 word has been pushed into the DIN buffer (The HASH_DIN register contains 1 word, the IN FIFO is empty)
0001: 2 words have been pushed into the DIN buffer (the HASH_DIN register and the IN FIFO contain 1 word each)
...
1111: 16 words have been pushed into the DIN buffer

" If the DMA is used, NBW is the exact number of words that have been pushed into the IN FIFO.

Bit 18 and bit 7 **ALGO[1:0]**: Algorithm selection

These bits selects the SHA-1, SHA-224, SHA256 or the MD5 algorithm:
00: SHA-1 algorithm selected
01: MD5 algorithm selected
10: SHA224 algorithm selected
11: SHA256 algorithm selected

*Note: This selection is only taken into account when the INIT bit is set. Changing this bit during a computation has no effect.*

Bit 6 **MODE**: Mode selection

This bit selects the HASH or HMAC mode for the selected algorithm:
0: Hash mode selected
1: HMAC mode selected. LKEY must be set if the key being used is longer than 64 bytes.

*Note: This selection is only taken into account when the INIT bit is set. Changing this bit during a computation has no effect.*

Bits 5:4 **DATATYPE**: Data type selection

Defines the format of the data entered into the HASH_DIN register:
00: 32-bit data. The data written into HASH_DIN are directly used by the HASH processing, without reordering.
01: 16-bit data, or half-word. The data written into HASH_DIN are considered as 2 half-words, and are swapped before being used by the HASH processing.
10: 8-bit data, or bytes. The data written into HASH_DIN are considered as 4 bytes, and are swapped before being used by the HASH processing.
11: bit data, or bit-string. The data written into HASH_DIN are considered as 32 bits (1st bit of the sting at position 0), and are swapped before being used by the HASH processing (1st bit of the string at position 31).

786/1749 RM0090 Rev 18

--- Page 7 ---

RM0090
Hash processor (HASH)

Bit 3 **DMAE**: DMA enable
0: DMA transfers disabled
1: DMA transfers enabled. A DMA request is sent as soon as the HASH core is ready to receive data.
*Note:* 1: This bit is cleared by hardware when the DMA asserts the DMA terminal count signal (while transferring the last data of the message). This bit is not cleared when the INIT bit is written to 1.
2: If this bit is written to 0 while a DMA transfer has already been requested to the DMA, DMAE is cleared but the current transfer is not aborted. Instead, the DMA interface remains internally enabled until the transfer is complete or INIT is written to 1.

Bit 2 **INIT**: Initialize message digest calculation
Writing this bit to 1 resets the hash processor core, so that the HASH is ready to compute the message digest of a new message.
Writing this bit to 0 has no effect.
Reading this bit always return 0.

Bits 1:0 Reserved, must be kept cleared.

ST
RM0090 Rev 18
787/1749

--- Page 8 ---

Hash processor (HASH) RM0090

> [RM0090 ch.25 §25.4] HASH registers

### 25.4.3 HASH data input register (HASH_DIN)

### 25.4.3 HASH data input register (HASH_DIN)

Address offset: 0x04
Reset value: 0x0000 0000

HASH_DIN is the data input register. It is 32-bit wide. It is used to enter the message by blocks of 512 bits. When the HASH_DIN register is written to, the value presented on the AHB databus is 'pushed' into the HASH core and the register takes the new value presented on the AHB databus. The DATATYPE bits must previously have been configured in the HASH_CR register to get a correct message representation.

When a block of 16 words has been written to the HASH_DIN register, an intermediate digest calculation is launched:
* by writing new data into the HASH_DIN register (the first word of the next block) if the DMA is not used (intermediate digest calculation)
* automatically if the DMA is used

When the last block has been written to the HASH_DIN register, the final digest calculation (including padding) is launched:
* by writing the DCAL bit to 1 in the HASH_STR register (final digest calculation)
* automatically if the DMA is used and MDMAT bit is set to '0'.

When a digest calculation (intermediate or final) is in progress, any new write access to the HASH_DIN register is extended (by wait-state insertion on the AHB bus) until the HASH calculation completes.

When the HASH_DIN register is read, the last word written in this location is accessed (zero after reset).

| 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 |
|---|
| DATAIN |
| rw |
| 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0 |
| DATAIN |
| rw |


Bits 31:0 **DATAIN**: Data input
Read = returns the current register content.
Write = the current register content is pushed into the IN FIFO, and the register takes the new value presented on the AHB databus.

788/1749 RM0090 Rev 18 ST

--- Page 9 ---

RM0090 Hash processor (HASH)


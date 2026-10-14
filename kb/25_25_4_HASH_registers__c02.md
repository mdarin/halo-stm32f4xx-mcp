> [RM0090 ch.25 §25.4] HASH registers


### 25.4.1 HASH control register (HASH_CR) for STM32F415/417xx

Address offset: 0x00
Reset value: 0x0000 0000

| 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved | LKEY |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | rw |
| 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reserved | DINNE | NBW | ALGO[0] | MODE | DATATYPE | DMAE | INIT | Reserved |  |  |  |  |  |  |  |
| r | rw | w |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bits 31:17 Reserved, forced by hardware to 0.

Bit 16 **LKEY**: Long key selection
This bit selects between short key (≤64 bytes) or long key (> 64 bytes) in HMAC
mode
0: Short key (≤64 bytes)
1: Long key (> 64 bytes)
*Note: This selection is only taken into account when the INIT bit is set and MODE
= 1. Changing this bit during a computation has no effect.*

Bits 15:13 Reserved, forced by hardware to 0.

782/1749
RM0090 Rev 18
ST

--- Page 3 ---

RM0090
Hash processor (HASH)

**Bit 12 DINNE:** DIN not empty
This bit is set when the HASH_DIN register holds valid data (that is after being written at least once). It is cleared when either the INIT bit (initialization) or the DCAL bit (completion of the previous message processing) is written to 1.
0: No data are present in the data input buffer
1: The input buffer contains at least one word of data

**Bits 11:8 NBW:** Number of words already pushed
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

**Bit 7 ALGO[1:0]:** Algorithm selection
These bits selects the SHA-1 or the MD5 algorithm:
0: SHA-1 algorithm selected
1: MD5 algorithm selected
*Note: This selection is only taken into account when the INIT bit is set. Changing this bit during a computation has no effect.*

**Bit 6 MODE:** Mode selection
This bit selects the HASH or HMAC mode for the selected algorithm:
0: Hash mode selected
1: HMAC mode selected. LKEY must be set if the key being used is longer than 64 bytes.
*Note: This selection is only taken into account when the INIT bit is set. Changing this bit during a computation has no effect.*

**Bits 5:4 DATATYPE:** Data type selection
Defines the format of the data entered into the HASH_DIN register:
00: 32-bit data. The data written into HASH_DIN are directly used by the HASH processing, without reordering.
01: 16-bit data, or half-word. The data written into HASH_DIN are considered as 2 half-words, and are swapped before being used by the HASH processing.
10: 8-bit data, or bytes. The data written into HASH_DIN are considered as 4 bytes, and are swapped before being used by the HASH processing.
11: bit data, or bit-string. The data written into HASH_DIN are considered as 32 bits (1st bit of the sting at position 0), and are swapped before being used by the HASH processing (1st bit of the string at position 31).

ST
RM0090 Rev 18
783/1749

--- Page 4 ---

Hash processor (HASH)
RM0090

Bit 3 **DMAE**: DMA enable
0: DMA transfers disabled
1: DMA transfers enabled. A DMA request is sent as soon as the HASH core is ready to receive data.
Note: 1: This bit is cleared by hardware when the DMA asserts the DMA terminal count signal (while transferring the last data of the message). This bit is not cleared when the INIT bit is written to 1.
2: If this bit is written to 0 while a DMA transfer has already been requested to the DMA, DMAE is cleared but the current transfer is not aborted. Instead, the DMA interface remains internally enabled until the transfer is complete or INIT is written to 1.

Bit 2 **INIT**: Initialize message digest calculation
Writing this bit to 1 resets the hash processor core, so that the HASH is ready to compute the message digest of a new message.
Writing this bit to 0 has no effect.
Reading this bit always return 0.

Bits 1:0 Reserved, must be kept cleared.

784/1749
RM0090 Rev 18
ST

--- Page 5 ---

RM0090
Hash processor (HASH)


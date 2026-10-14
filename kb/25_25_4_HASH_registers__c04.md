> [RM0090 ch.25 §25.4] HASH registers


### 25.4.4 HASH start register (HASH_STR)

Address offset: 0x08
Reset value: 0x0000 0000

The HASH_STR register has two functions:

*   It is used to define the number of valid bits in the last word of the message entered in the hash processor (that is the number of valid least significant bits in the last data written into the HASH_DIN register)
*   It is used to start the processing of the last block in the message by writing the DCAL bit to 1

| 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved |  |  |  |  |  |  |  |  |  |  |  |
| 15 14 13 12 11 10 9 8 | 7 6 5 | 4 3 2 1 0 |  |  |  |  |  |  |  |  |  |
| Reserved | DCAL | Reserved | NBLW |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  | w |  |  | rw |


Bits 31:9 Reserved, forced by hardware to 0.

**Bit 8 DCAL**: Digest calculation
Writing this bit to 1 starts the message padding, using the previously written value of NBLW, and starts the calculation of the final message digest with all data words written to the IN FIFO since the INIT bit was last written to 1.
Reading this bit returns 0.
Note

Bits 7:5 Reserved, forced by hardware to 0.

**Bits 4:0 NBLW**: Number of valid bits in the last word of the message in the bit string organization of hash processor
When these bits are written and DCAL is at '0', they take the value on the AHB databus:
0x00: All 32 bits of the last data written in the bit string organization of hash processor (after data swapping) are valid.
0x01: Only bit [31] of the last data written in the bit string organization of hash processor (after data swapping) are valid
0x02: Only bits [31:30] of the last data written in the bit string organization of hash processor (after data swapping) are valid
0x03: Only bits [31:29] of the last data written in the bit string organization of hash processor (after data swapping) are valid
...
0x1F: Only bits [0] of the last data written in the bit string organization of hash processor (after data swapping) are valid
When these bits are written and DCAL is at '1', the bitfield is not changed.
Reading them returns the last value written to NBLW.

*Note: These bits must be configured before setting the DCAL bit, else they are not taken into account. Especially, it is not possible to configure NBLW and set DCAL at the same time.*

STI
RM0090 Rev 18 789/1749

--- Page 10 ---

Hash processor (HASH)
RM0090

> [RM0090 ch.25 §25.4] HASH registers

### 25.4.5 HASH digest registers (HASH_HR0..4/5/6/7)

## 25.4.5 HASH digest registers (HASH_HR0..4/5/6/7)

Address offset: 0x0C to 0x1C (STM32F415/417xx), plus 0x310 to 0x32C (STM32F43xxx)
Reset value: 0x0000 0000

These registers contain the message digest result named as:

1. H0, H1, H2, H3 and H4, respectively, in the SHA1 algorithm description
   Note that in this case, the HASH_H5 to HASH_H7 register is not used, and is read as zero.
2. A, B, C and D, respectively, in the MD5 algorithm description
   Note that in this case, the HASH_H4 to HASH_H7 register is not used, and is read as zero.
3. H0 to H6, respectively, in the SHA224 algorithm description,
   Note that in this case, the HASH_H7 register is not used, and is read as zero.
4. H0 to H7, respectively, in the SHA256 algorithm description,

If a read access to one of these registers occurs while the HASH core is calculating an intermediate digest or a final message digest (that is when the DCAL bit has been written to 1), then the read is stalled until the completion of the HASH calculation.

*Note:* H0, H1, H2, H3 and H4 mapping are duplicated in two region.

> [RM0090 ch.25 §25.4] HASH registers

### HASH_HR0

### HASH_HR0

Address offset: 0x0C and 0x310

| H0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 |
| r |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| H0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
| r |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


> [RM0090 ch.25 §25.4] HASH registers

### HASH_HR1

### HASH_HR1

Address offset: 0x10 and 0x314

| H1 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 |
| r |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| H1 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
| r |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |



> [RM0090 ch.23 §23.4] CRYP interrupts

### 23.6.2 CRYP control register (CRYP_CR) for STM32F415/417xx**

**23.6.2 CRYP control register (CRYP_CR) for STM32F415/417xx**

Address offset: 0x00
Reset value: 0x0000 0000

750/1749 RM0090 Rev 18

--- Page 166 ---

RM0090
Cryptographic processor (CRYP)

| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved | ALGO MODE [3] | Res. | GCM_CCMPH |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  | rw |  | rw |  |
| 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
| CRYPEN | FFLUSH | Reserved | KEYSIZE | DATATYPE | ALGOMODE[2:0] | ALGODIR | Reserved |  |  |  |  |  |  |  |  |
| rw | w |  |  |  |  |  |  | rw |  |  |  |  |  |  |  |


Bits 31:20 Reserved, forced by hardware to 0.
Bit 18 Reserved, forced by hardware to 0.
Bits 17:16 GCM_CCMPH[1:0]: no effect if “GCM or CCM algorithm” is not set
00: GCM_CCM init Phase
01: GCM_CCM header phase
10: GCM_CCM payload phase
11: GCM_CCM final phase

Bit 15 **CRYPEN**: Cryptographic processor enable
0: CRYP processor is disabled
1: CRYP processor is enabled
*Note: The CRYPEN bit is automatically cleared by hardware when the key preparation process ends (ALGOMODE=111b) or GCM_CCM init Phase*

Bit 14 **FFLUSH**: FIFO flush
When CRYPEN = 0, writing this bit to 1 flushes the IN and OUT FIFOs (that is read and write pointers of the FIFOs are reset. Writing this bit to 0 has no effect. When CRYPEN = 1, writing this bit to 0 or 1 has no effect.
Reading this bit always returns 0.

Bits 13:10 Reserved, forced by hardware to 0.
Bits 9:8 **KEYSIZE[1:0]**: Key size selection (AES mode only)
This bitfield defines the bit-length of the key used for the AES cryptographic core.
This bitfield is ‘don’t care’ in the DES or TDES modes.
00: 128 bit key length
01: 192 bit key length
10: 256 bit key length
11: Reserved, do not use this value

Bits 7:6 **DATATYPE[1:0]**: Data type selection
This bitfield defines the format of data entered in the CRYP_DIN register (refer to Section 23.3.3: *Data type*).
00: 32-bit data. No swapping of each word. First word pushed into the IN FIFO (or popped off the OUT FIFO) forms bits 1...32 of the data block, the second word forms bits 33...64.
01: 16-bit data, or half-word. Each word pushed into the IN FIFO (or popped off the OUT FIFO) is considered as 2 half-words, which are swapped with each other.
10: 8-bit data, or bytes. Each word pushed into the IN FIFO (or popped off the OUT FIFO) is considered as 4 bytes, which are swapped with each other.
11: bit data, or bit-string. Each word pushed into the IN FIFO (or popped off the OUT FIFO) is considered as 32 bits (1st bit of the string at position 0), which are swapped with each other.

STI
RM0090 Rev 18
751/1749

--- Page 167 ---

**Cryptographic processor (CRYP)**
RM0090

Bits 19 and 5:3 **ALGOMODE[3:0]**: Algorithm mode

0000: TDES-ECB (triple-DES Electronic codebook): no feedback between blocks of data. Initialization vectors (CRYP_IV0(L/R)) are not used, three key vectors (K1, K2, and K3) are used (K0 is not used).
0001: TDES-CBC (triple-DES Cipher block chaining): output block is XORed with the subsequent input block before its entry into the algorithm. Initialization vectors (CRYP_IV0(L/R)) must be initialized, three key vectors (K1, K2, and K3) are used (K0 is not used).
0010: DES-ECB (simple DES Electronic codebook): no feedback between blocks of data. Initialization vectors (CRYP_IV0(L/R)) are not used, only one key vector (K1) is used (K0, K2, K3 are not used).
0011: DES-CBC (simple DES Cipher block chaining): output block is XORed with the subsequent input block before its entry into the algorithm. Initialization vectors (CRYP_IV0(L/R)) must be initialized. Only one key vector (K1) is used (K0, K2, K3 are not used).
0100: AES-ECB (AES Electronic codebook): no feedback between blocks of data. Initialization vectors (CRYP_IV0/L/R...1/L/R) are not used. All four key vectors (K0...K3) are used.
0101: AES-CBC (AES Cipher block chaining): output block is XORed with the subsequent input block before its entry into the algorithm. Initialization vectors (CRYP_IV0/L/R...1/L/R) must be initialized. All four key vectors (K0...K3) are used.
0110: AES-CTR (AES Counter mode): output block is XORed with the subsequent input block before its entry into the algorithm. Initialization vectors (CRYP_IV0/L/R...1/L/R) must be initialized. All four key vectors (K0...K3) are used. CTR decryption does not differ from CTR encryption, since the core always encrypts the current counter block to produce the key stream that will be XORed with the plaintext or cipher in input. Thus, ALGODIR is don't care when ALGOMODE = 110b, and the key must NOT be unrolled (prepared) for decryption.
0111: AES key preparation for decryption mode. Writing this value when CRYPTEN = 1 immediately starts an AES round for key preparation. The secret key must have previously been loaded into the K0...K3 registers. The BUSY bit in the CRYP_SR register is set during the key preparation. After key processing, the resulting key is copied back into the K0...K3 registers, and the BUSY bit is cleared.
1000: Galois Counter Mode (GCM). This algorithm mode is also used for the GMAC algorithm.
1001: Counter with CBC-MAC (CCM). This algorithm mode is also used for the CMAC algorithm.

Bit 2 **ALGODIR**: Algorithm direction
0: Encrypt
1: Decrypt

Bits 1:0 Reserved, must be kept to 0.

**Note:**
Writing to the KEYSIZE, DATATYPE, ALGOMODE and ALGODIR bits while BUSY=1 has no effect. These bits can only be configured when BUSY=0.
The FFLUSH bit has to be set only when BUSY=0. If not, the FIFO is flushed, but the block being processed may be pushed into the output FIFO just after the flush operation, resulting in a nonempty FIFO condition.

752/1749
RM0090 Rev 18

--- Page 168 ---

RM0090
Cryptographic processor (CRYP)

> [RM0090 ch.23 §23.4] CRYP interrupts

### 23.6.3 CRYP status register (CRYP_SR)

### 23.6.3 CRYP status register (CRYP_SR)

Address offset: 0x04
Reset value: 0x0000 0003

| 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved |  |  |  |  |  |  |  |  |  |  |  |  |
| 15 14 13 12 11 10 9 8 7 6 5 4 | 3 | 2 | 1 | 0 |  |  |  |  |  |  |  |  |
| Reserved | BUSY | OFFU | OFNE | IFNF | IFEM |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  | r |


Bits 31:5 Reserved, must be kept at reset value

**Bit 4 BUSY**: Busy bit
0: The CRYP Core is not processing any data. The reason is either that:
- the CRYP core is disabled (CRYPEN=0 in the CRYP_CR register) and the last processing has completed, or
- The CRYP core is waiting for enough data in the input FIFO or enough free space in the output FIFO (that is in each case at least 2 words in the DES, 4 words in the AES).
1: The CRYP core is currently processing a block of data or a key preparation (for AES decryption).

**Bit 3 OFFU**: Output FIFO full
0: Output FIFO is not full
1: Output FIFO is full

**Bit 2 OFNE**: Output FIFO not empty
0: Output FIFO is empty
1: Output FIFO is not empty

**Bit 1 IFNF**: Input FIFO not full
0: Input FIFO is full
1: Input FIFO is not full

**Bit 0 IFEM**: Input FIFO empty
0: Input FIFO is not empty
1: Input FIFO is empty

ST
RM0090 Rev 18
753/1749

--- Page 169 ---

Cryptographic processor (CRYP)
RM0090


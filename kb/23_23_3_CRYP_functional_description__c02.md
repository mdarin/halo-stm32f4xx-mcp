> [RM0090 ch.23 §23.3] CRYP functional description

### 23.3.3 Data type**

**23.3.3 Data type**
Data enter the CRYP processor 32 bits (word) at a time as they are written into the CRYP_DIN register. The principle of the DES is that streams of data are processed 64 bits by 64 bits and, for each 64-bit block, the bits are numbered from M1 to M64, with M1 the left-most bit and M64 the right-most bit of the block. The same principle is used for the AES, but with a 128-bit block size.

RM0090 Rev 18
739/1749

--- Page 155 ---

**Cryptographic processor (CRYP)**

RM0090

The system memory organization is little-endian: whatever the data type (bit, byte, 16-bit half-word, 32-bit word) used, the least-significant data occupy the lowest address locations. A bit, byte, or half-word swapping operation (depending on the kind of data to be encrypted) therefore has to be performed on the data read from the IN FIFO before they enter the CRYP processor. The same swapping operation should be performed on the CRYP data before they are written into the OUT FIFO. For example, the operation would be byte swapping for an ASCII text stream.

The kind of data to be processed is configured with the DATATYPE bitfield in the CRYP control register (CRYP_CR).

**Table 113. Data types**

| DATATYPE in CRYP_CR | Swapping performed | System memory data (plaintext or cypher) |
|---|---|---|
| 00b | No swapping | Example: TDES block value 0xABCD77206973FE01 is represented in system memory as: TDES block size = 64bit = 2x 32 bit 0xABCD7720 6973FE01 system memory 0xABCD7720 @ 0x6973FE01 @+4 |
| 01b | Half-word (16-bit) swapping | Example: TDES block value 0xABCD77206973FE01 is represented in system memory as: TDES block size = 64bit = 2x 32 bit 0xABCD 7720 6973 FE01 system memory 0x7720 ABCD @ 0xFE01 6973 @+4 |
| 10b | Byte (8-bit) swapping | Example: TDES block value 0xABCD77206973FE01 is represented in system memory as: TDES block size = 64bit = 2x 32 bit 0xAB CD 77 20 69 73 FE 01 system memory 0x 20 77 CD AB @ 0x 01 FE 73 69 @+4 |
| 11b | Bit swapping | TDES block value 0x4E6F772069732074 is represented in system memory as: TDES Bloc size = 64bit = 2x 32 bit 0x4E 6F 77 20 69 73 20 74 system memory 0x04 EE F6 72 @ 0x2E 04 CE 96 @+4 0000 0100 1110 1110 1111 0110 0100 0000 0010 1110 0000 0100 1100 1110 1001 0110 @+4 |


*Figure 229* shows how the 64-bit data block M1...64 is constructed from two consecutive 32-bit words popped off the IN FIFO by the CRYP processor, according to the DATATYPE value. The same schematic can easily be extended to form the 128-bit block for the AES

740/1749 RM0090 Rev 18 STI

--- Page 156 ---

RM0090
Cryptographic processor (CRYP)

cryptographic algorithm (for the AES, the block length is four 32-bit words, but swapping
only takes place at word level, so it is identical to the one described here for the TDES).
The same swapping is performed between the IN_FIFO and the CRYP data block, and
between the CRYP data block and the OUT_FIFO.

Note:

Figure 229. 64-bit block construction according to DATATYPE

DATATYPE = 11b
bit swapping operation
IN FIFO
second word written into the CRYP_DIN register
first word written into the CRYP_DIN register
bit 31 bit 30
bit 2 bit 1
bit 0
bit 31 bit 30
bit 2 bit 1
bit 0
bit string
M1 M2 M30 M31 M32 M33 M34 M52 M53 M62 M63 M64

DATATYPE = 10b
byte swapping operation
IN FIFO
second word written into the CRYP_DIN register
first word written into the CRYP_DIN register
Byte 1,0 bits 7..0
Byte 1,1 bits 7..0
Byte 1,2 bits 7..0
Byte 1,3 bits 7..0
Byte 0,0 bits 7..0
Byte 0,1 bits 7..0
Byte 0,2 bits 7..0
Byte 0,3 bits 7..0
(bit ordering within byte is unchanged)
bit string
M1..8 Byte 0,0 M9..16 Byte 0,1 M17..24 Byte 0,2 M25..32 Byte 0,3 M33..40 Byte 1,0 M41..48 Byte 1,1 M49..56 Byte 1,2 M57..64 Byte 1,3

DATATYPE = 01b
half-word swapping operation
IN FIFO
second word written into the CRYP_DIN register
first word written into the CRYP_DIN register
Half-word 1,0 bits 15..0
Half-word 0,1 bits 15..0
Half-word 1,1 bits 15..0
Half-word 0,0 bits 15..0
bit string
M1..16 Half-word 0,0 M17..32 Half-word 0,1 M33..48 Half-word 1,0 M49..64 Half-word 1,1

DATATYPE = 00b
No swapping operation
IN FIFO
second word written into the CRYP_DIN register
first word written into the CRYP_DIN register
Word 1 bits 31..0
Word 0 bits 31..0
bit string
M1..32 Word 0 M33..64 Word 1

ar16075
RM0090 Rev 18
741/1749

--- Page 157 ---

Cryptographic processor (CRYP)
RM0090


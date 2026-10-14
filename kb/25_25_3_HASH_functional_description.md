## 25.3 HASH functional description

*Figure 1* shows the block diagram of the hash processor.

**Figure 233. Block diagram for STM32F415/417xx**

32-bit AHB2 bus
IN buffer
Data register
HASH_DIN
write into HASH_DIN or write DCAL bit to 1 or 1 complete block transferred by the DMA
16 x 32-bit IN FIFO
swapping
IN FIFO full or DCAL written to 1
Control and status registers
Interrupt registers
HASH_IMR
HASH_SR
Control register
HASH_CR
Start register
HASH_STR
Context swapping
HASH_CSR0..50
Message digest
HASH_H0..H4
context
digest
SHA-1 / MD5 Hash / HMAC processor core

ai16081

ST
RM0090 Rev 18
773/1749

--- Page 189 ---

Hash processor (HASH)
RM0090

Figure 234. Block diagram for STM32F43xxx

32-bit AHB2 bus
IN buffer
Data register
Control and status registers
Interrupt registers
HASH_IMR
HASH_SR
Control register
HASH_CR
Start register
HASH_STR
Context swapping
HASH_CSR0..53
Message digest
HASH_H0..H7
HASH_DIN
write into HASH_DIN or write DCAL bit to 1 or 1 complete block transferred by the DMA
16 x 32-bit IN FIFO
swapping
IN FIFO full or DCAL written to 1
context
digest
SHA-1, SHA-224, SHA-256 and MD5 Hash / HMAC processor core
MS30442V1

The FIPS PUB 180-2 standard and the IETF RFC 1321 publication specify the SHA-1, SHA-224 and SHA-256 and MD5 secure hash algorithms, respectively, for computing a condensed representation of a message or data file. When a message of any length below 2^64 bits is provided on input, the SHA-1, SHA-224 and SHA-256 and MD5 produce respective a 160-bit, 224 bit, 256 bit and 128-bit output string, respectively, called a message digest. The message digest can then be processed with a digital signature algorithm in order to generate or verify the signature for the message. Signing the message digest rather than the message often improves the efficiency of the process because the message digest is usually much smaller in size than the message. The verifier of a digital signature has to use the same hash algorithm as the one used by the creator of the digital signature.

The SHA-1, SHA-224 and SHA-256 and MD5 are qualified as "secure" because it is computationally infeasible to find a message that corresponds to a given message digest, or to find two different messages that produce the same message digest. Any change to a message in transit will, with very high probability, result in a different message digest, and the signature will fail to verify. For more detail on the SHA-1 or SHA-224 and SHA-256 algorithm, please refer to the FIPS PUB 180-2 (Federal Information Processing Standards Publication 180-2), 2002 august 1.

The current implementation of this standard works with little-endian input data convention. For example, the C string "abc" must be represented in memory as the 24-bit hexadecimal value 0x434241.

A message or data file to be processed by the hash processor should be considered a bit string. The length of the message is the number of bits in the message (the empty message

774/1749
RM0090 Rev 18
ST

--- Page 190 ---

RM0090 Hash processor (HASH)

has length 0). You can consider that 32 bits of this bit string forms a 32-bit word. Note that the FIPS PUB 180-1 standard uses the convention that bit strings grow from left to right, and bits can be grouped as bytes (8 bits) or words (32 bits) (but some implementations also use half-words (16 bits), and implicitly, uses the big-endian byte (half-word) ordering. This convention is mainly important for padding (see Section 1.3.4: Message padding on page 12).

### 25.3.1 Duration of the processing

The computation of an intermediate block of a message takes:

*   66 HCLK clock cycles in SHA-1
*   50 HCLK clock cycles in SHA-224
*   50 HCLK clock cycles in SHA-256
*   50 HCLK clock cycles in MD5

to which you must add the time needed to load the 16 words of the block into the processor (at least 16 clock cycles for a 512-bit block).

The time needed to process the last block of a message (or of a key in HMAC) can be longer. This time depends on the length of the last block and the size of the key (in HMAC mode). Compared to the processing of an intermediate block, it can be increased by a factor of:

*   1 to 2.5 for a hash message
*   around 2.5 for an HMAC input-key
*   1 to 2.5 for an HMAC message
*   around 2.5 for an HMAC output key in case of a short key
*   3.5 to 5 for an HMAC output key in case of a long key

### 25.3.2 Data type

Data are entered into the hash processor 32 bits (word) at a time, by writing them into the HASH_DIN register. But the original bit-string can be organized in bytes, half-words or words, or even be represented as bits. As the system memory organization is little-endian and SHA1, SHA-224 and SHA-256 computation is big-endian, depending on the way the original bit string is grouped, a bit, byte, or half-word swapping operation is performed automatically by the hash processor.

The kind of data to be processed is configured with the DATATYPE bitfield in the HASH control register (HASH_CR).

STI RM0090 Rev 18 775/1749

--- Page 191 ---

Hash processor (HASH)
RM0090

Figure 235. Bit, byte and half-word swapping

A-In case of binary data hash, all bits should be swapped as below

Bit swapping operation
DATATYPE =bx 11

HASH_DIN
bit 31 bit 30 bit 2 bit 1 bit 0
Bits entred with little-Endian format

bit string organization in
Hash processor: Big-Endian
bit 0 bit 1 bit 29 bit 30 bit 31

"1" "0s"
padding is performed
on this side of the
bit string.

bit-string grows
in this direction
as defined by
FIPSPUB 180-2 std.

B-In case of byte data hash, all bytes should be swapped as below

Byte swapping operation
DATATYPE =bx 10

HASH_DIN
Byte 3 bits 7...0 Byte 2 bits 7...0 Byte 1 bits 7...0 Byte 0 bits 7...0
Bytes entred with little-Endian format

Bit string organization in
Hash processor: Big-Endian
Byte 0 bits 7...0 Byte 1 bits 7...0 Byte 2 bits 7...0 Byte 3 bits 7...0

bit-string grows
in this direction
as defined by
FIPSPUB 180-2 std.

C-In case of half-word hash, all half-word should be swapped as below

Half-word swapping operation
DATATYPE =bx 01

HASH_DIN
Half-word 1 bits 15...0 Half-word 0 bits 15...0
half word entred with little-Endian format

Bit string organization in
Hash processor: Big-Endian
Half-word 0 bits 15...0 Half-word 1 bits 15...0

bit-string grows
in this direction
as defined by
FIPS PUB 180-2 std.

ai16082b

776/1749
RM0090 Rev 18

--- Page 192 ---

RM0090
Hash processor (HASH)

The least significant bit of the message has to be at position 0 (right) in the first word entered into the hash processor, the 32nd bit of the bit string has to be at position 0 in the second word entered into the hash processor and so on.

### 25.3.3 Message digest computing

The HASH sequentially processes blocks of 512 bits when computing the message digest. Thus, each time 16 × 32-bit words (= 512 bits) have been written by the DMA or the CPU, into the hash processor, the HASH automatically starts computing the message digest. This operation is known as a partial digest computation.

The message to be processed is entered into the peripheral by 32-bit words written into the HASH_DIN register. The current contents of the HASH_DIN register are transferred to the input FIFO (IN FIFO) each time the register is written with new data. HASH_DIN and the input FIFO form a FIFO of a 17-word length (named the IN buffer).

The processing of a block can start only once the last value of the block has entered the IN FIFO. The peripheral must get the information as to whether the HASH_DIN register contains the last bits of the message or not. Two cases may occur:

*   When the DMA is not used:
    *   In case of a partial digest computation, this is done by writing an additional word into the HASH_DIN register (actually the first word of the next block). Then the software must wait until the processor is ready again (when DINIS=1) before writing new data into HASH_DIN.
    *   In case of a final digest computation (last block entered), this is done by writing the DCAL bit to 1.
*   When the DMA is used:
    The contents of the HASH_DIN register are interpreted automatically with the information sent by the DMA controller.
    *   In case of a single DMA transfer: Multiple DMA transfer (MDMAT) bit should be cleared on STM32F43xxx. When the last block has been transferred to the HASH_DIN register via DMA channel, DCAL bit will be set to automatically to 1 in the HASH_STR register in order to launch the final digest calculation.
    *   In case of a multiple DMA transfer (available only on STM32F43xxx): Multiple DMA transfer (MDMAT) bit should be set to 1 by software so DCAL bit does not get set automatically by HW, in this case the final digest calculation for hash and for each phases for HMAC (for more details about HMAC phases please refer to HMAC operation section) will not be launched a the end of the DMA transfer request, allowing the processor to receive a new DMA transfer. During the last DMA transfer, Multiple DMA transfer (MDMAT) bit should be cleared by software in order to set automatically DCAL bit at the end of the last bloc and lunch the final digest.
    *   The contents of the HASH_DIN register are interpreted automatically with the information sent by the DMA controller.

This process —data entering + partial digest computation— continues until the last bits of the original message are written to the HASH_DIN register. As the length (number of bits) of a message can be any integer value, the last word written into the HASH processor may have a valid number of bits between 1 and 32. This number of valid bits in the last word, NBLW, has to be written into the HASH_STR register, so that message padding is correctly performed before the final message digest computation.

ST
RM0090 Rev 18
777/1749

--- Page 193 ---

Hash processor (HASH)
RM0090

Once this is done, writing into HASH_STR with bit DCAL = 1 starts the processing of the last entered block of message by the hash processor. This processing consists in:

*   Automatically performing the message padding operation: the purpose of this operation is to make the total length of a padded message a multiple of 512. The HASH sequentially processes blocks of 512 bits when computing the message digest
*   Computing the final message digest

When the DMA is enabled, it provides the information to the hash processor when it is transferring the last data word. Then the padding and digest computation are performed automatically as if DCAL had been written to 1.

### 25.3.4 Message padding

Message padding consists in appending a “1” followed by m “0”s followed by a 64-bit integer to the end of the original message to produce a padded message block of length 512. The “1” is added to the last word written into the HASH_DIN register at the bit position defined by the NBLW bitfield, and the remaining upper bits are cleared (“0”s).

Example: let us assume that the original message is the ASCII binary-coded form of “abc”, of length L = 24:

```
byte 0    byte 1    byte 2    byte 3
01100001 01100010 01100011 UUUUUUUU
<-- 1st word written to HASH_DIN -->
```

NBLW has to be loaded with the value 24: a “1” is appended at bit location 24 in the bit string (starting counting from left to right in the above bit string), which corresponds to bit 31 in the HASH_DIN register (little-endian convention):

```
01100001 01100010 01100011 1UUUUUUU
```

Since L = 24, the number of bits in the above bit string is 25, and 423 “0”s are appended, making now 448. This gives (in hexadecimal, big-endian format):

```
61626380 00000000 00000000 00000000
00000000 00000000 00000000 00000000
00000000 00000000 00000000 00000000
00000000 00000000
```

The L value, in two-word representation (that is 00000000 00000018) is appended. Hence the final padded message in hexadecimal:

```
61626380 00000000 00000000 00000000
00000000 00000000 00000000 00000000
00000000 00000000 00000000 00000000
00000000 00000000 00000000 00000028
```

If the HASH is programmed to use the little-endian byte input format, the above message has to be entered by doing the following steps:

1.  0xUU636261 is written into the HASH_DIN register (where ‘U’ means don’t care)
2.  0x18 is written into the HASH_STR register (the number of valid bits in the last word written into the HASH_DIN register is 24, as the original message length is 24 bits)
3.  0x10 is written into the HASH_STR register to start the message padding and digest computation. When NBLW ≠ 0x00, the message padding puts a “1” into the HASH_DIN register at the bit position defined by the NBLW value, and inserts “0”s at bit locations [31:(NBLW+1)]. When NBLW == 0x00, the message padding inserts one new word with

778/1749
RM0090 Rev 18
ST

--- Page 194 ---

RM0090
Hash processor (HASH)

value 0x0000 0001. Then an all zero word (0x0000 0000) is added and the message length in a two-word representation, to get a block of 16 x 32-bit words.

4. The HASH computing is performed, and the message digest is then available in the HASH_Hx registers (x = 0...4) for the SHA-1 algorithm. For example:

H0 = 0xA9993E36
H1 = 0x4706816A
H2 = 0xBA3E2571
H3 = 0x7850C26C
H4 = 0x9CD0D89D

### 25.3.5 Hash operation

The hash function (SHA-1, SHA-224, SHA-256 and MD5) is selected when the INIT bit is written to '1' in the HASH_CR register while the MODE bit is at '0' in HASH_CR. The algorithm (SHA-1, SHA-224, SHA-256 or MD5) is selected at the same time (that is when the INIT bit is set) using the ALGO bits.

The message can then be sent by writing it word by word into the HASH_DIN register. When a block of 512 bits —that is 16 words— has been written, a partial digest computation starts upon writing the first data of the next block. The hash processor remains busy for 66 cycles for the SHA-1 algorithm, or 50 cycles for the MD5 algorithm, SHA-224 algorithm and SHA-256 algorithm.

The process can then be repeated until the last word of the message. If DMA transfers are used, refer to the *Procedure where the data are loaded by DMA* section. Otherwise, if the message length is not an exact multiple of 512 bits, then the HASH_STR register has to be written to launch the computation of the final digest.

Once computed, the digest can be read from the HASH_H0...HASH_H4 registers (for the MD5 algorithm, HASH_H4 is not relevant) on STM32F415/417xx, and from the HASH_H0...HASH_H7 registers on STM32F43xxx where:

HASH_H4..HASH_H7 are not relevant when the MD5 algorithm is selected,
HASH_H5.. HASH_H7 are not relevant when the SHA-1algorithm is selected,
HASH_H7 is not relevant when the SHA-224 algorithm is selected.

### 25.3.6 HMAC operation

The HMAC algorithm is used for message authentication, by irreversibly binding the message being processed to a key chosen by the user. For HMAC specifications, refer to "HMAC: keyed-hashing for message authentication, H. Krawczyk, M. Bellare, R. Canetti, February 1997.

Basically, the algorithm consists of two nested hash operations:

HMAC (message) = Hash [ ( key | pad) XOR 0x5C)
| Hash (( key | pad) XOR 0x36) | message ) ]

where:

* pad is a sequence of zeroes needed to extend the key to the length of the underlying hash function data block (that is 512 bits for both the SHA-1, SHA224, SHA-256 and MD5 hash algorithms)
* | represents the concatenation operator

To compute the HMAC, four different phases are required:

ST
RM0090 Rev 18
779/1749

--- Page 195 ---

Hash processor (HASH)
RM0090

1. The block is initialized by writing the INIT bit to '1' with the MODE bit at '1' and the ALGO bits set to the value corresponding to the desired algorithm. The LKEY bit must also be set during this phase if the key being used is longer than 64 bytes (in this case, the HMAC specifications specify that the hash of the key should be used in place of the real key).
2. The key (to be used for the inner hash function) is then given to the core. This operation follows the same mechanism as the one used to send the message in the hash operation (that is, by writing into HASH_DIN and, finally, into HASH_STR).
3. Once the last word has been entered and computation has started, the hash processor elaborates the key. It is then ready to accept the message text using the same mechanism as the one used to send the message in the hash operation.
4. After the first hash round, the hash processor returns "ready" to indicate that it is ready to receive the key to be used for the outer hash function (normally, this key is the same as the one used for the inner hash function). When the last word of the key is entered and computation starts, the HMAC result is made available in the HASH_H0...HASH_H4 registers on STM32F415/417xx and on HASH_H0...HASH_H7 registers on STM32F43xxx.

Note: 1 The computation latency of the HMAC primitive depends on the lengths of the keys and message. You could the HMAC as two nested underlying hash functions with the same key length (long or short).

**25.3.7 Context swapping**

It is possible to interrupt a hash/HMAC process to perform another processing with a higher priority, and to complete the interrupted process later on, when the higher-priority task is complete. To do so, the context of the interrupted task must be saved from the hash registers to memory, and then be restored from memory to the hash registers.

The procedures where the data flow is controlled by software or by DMA are described below.

780/1749
RM0090 Rev 18
ST
--- Page 1 ---

RM0090
Hash processor (HASH)

**Procedure where the data are loaded by software**

The context can be saved only when no block is currently being processed. That is, you must wait for DINIS = 1 (the last block has been processed and the input FIFO is empty) or NBW ≠ 0 (the FIFO is not full and no processing is ongoing).

*   Context saving:
    Store the contents of the following registers into memory:
    - HASH_IMR
    - HASH_STR
    - HASH_CR
    - HASH_CSR0 to HASH_CSR50 on STM32F415/417xx, and HASH_CSR0 to HASH_CSR53 on STM32F43xxx.
*   Context restoring:
    The context can be restored when the high-priority task is complete. Please follow the order of the sequence below.
    a) Write the following registers with the values saved in memory: HASH_IMR, HASH_STR and HASH_CR
    b) Initialize the hash processor by setting the INIT bit in the HASH_CR register
    c) Write the HASH_CSR0 to HASH_CSR50 (STM32F415/417xx), and HASH_CSR0 to HASH_CSR53 (STM32F43xxx) registers with the values saved in memory
    You can now restart the processing from the point where it has been interrupted.

**Procedure where the data are loaded by DMA**

In this case it is not possible to predict if a DMA transfer is in progress or if the process is ongoing. Thus, you must stop the DMA transfers, then wait until the HASH is ready in order to interrupt the processing of a message.

*   Interrupting a processing:
    - Clear the DMAE bit to disable the DMA interface
    - Wait until the current DMA transfer is complete (wait for DMAES = 0 in the HASH_SR register). Note that the block may or not have been totally transferred to the HASH.
    - Disable the corresponding channel in the DMA controller
    - Wait until the hash processor is ready (no block is being processed), that is wait for DINIS = 1
*   The context saving and context restoring phases are the same as above (see *Procedure where the data are loaded by software*).

Reconfigure the DMA controller so that it transfers the end of the message. You can now restart the processing from the point where it was interrupted by setting the DMAE bit.

*Note:*
If context swapping does not involve HMAC operations, the HASH_CSR38 to HASH_CSR50 (STM32F415/417xx) and HASH_CSR38 to HASH_CSR53 (STM32F43xxx) registers do not have to be saved and restored.

If context swapping occurs between two blocks (the last block was completely processed and the next block has not yet been pushed into the IN FIFO, NBW = 000 in the HASH_CR register), the HASH_CSR22 to HASH_CSR37 registers do not have to be saved and restored.

ST
RM0090 Rev 18
781/1749

--- Page 2 ---

Hash processor (HASH)
RM0090

### 25.3.8 HASH interrupt

There are two individual maskable interrupt sources generated by the HASH processor.
They are connected to the same interrupt vector.

You can enable or disable the interrupt sources individually by changing the mask bits in the
HASH_IMR register. Setting the appropriate mask bit to 1 enables the interrupt.

The status of the individual interrupt sources can be read from the HASH_SR register.

Figure 236. HASH interrupt mapping diagram

DCIS
DCIM
DINIS
DINIM
HASH interrupt to NVIC
ai16086


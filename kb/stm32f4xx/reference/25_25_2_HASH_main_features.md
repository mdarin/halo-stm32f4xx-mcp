# 25 Hash processor (HASH)

This section applies to STM32F415/417xx and STM32F43xxx devices.

## 25.1 HASH introduction

The hash processor is a fully compliant implementation of the secure hash algorithm (SHA-1, SHA-224, SHA-256), the MD5 (message-digest algorithm 5) hash algorithm and the HMAC (keyed-hash message authentication code) algorithm suitable for a variety of applications. It computes a message digest (160 bits for the SHA-1 algorithm, 256 bits for the SHA-256 algorithm and 224 bits for the SHA-224 algorithm, 128 bits for the MD5 algorithm) for messages of up to ($2^{64} - 1$) bits, while HMAC algorithms provide a way of authenticating messages by means of hash functions. HMAC algorithms consist in calling the SHA-1, SHA-224, SHA-256 or MD5 hash function twice.

## 25.2 HASH main features

* Suitable for data authentication applications, compliant with:
    * FIPS PUB 180-2 (Federal Information Processing Standards Publication 180-2)
    * Secure Hash Standard specifications (SHA-1, SHA-224 and SHA-256)
    * IETF RFC 1321 (Internet Engineering Task Force Request For Comments number 1321) specifications (MD5)
* Fast computation of SHA-1, SHA-224 and SHA-256, and MD5 (SHA-224 and SHA-256 are available on STM32F43xxx only)
* AHB slave peripheral
* 32-bit data words for input data, supporting word, half-word, byte and bit bit-string representations, with little-endian data representation only.
* Automatic swapping to comply with the big-endian SHA1, SHA-224 and SHA-256 computation standard with little-endian input bit-string representation
* Automatic padding to complete the input bit string to fit modulo 512 ($16 \times 32$ bits) message digest computing
* 5x 32-bit words (H0 to H5) on STM32F415/417xx and 8 x 32-bit words (H0 to H7) on STM32F43xxx for output message digest, reload able to continue interrupted message digest computation.
* Corresponding 32-bit words of the digest from consecutive message blocks are added to each other to form the digest of the whole message
* Automatic data flow control with support for direct memory access (DMA)

**Note:** Padding, as defined in the SHA-1, SHA-224 and SHA-256 algorithm, consists in adding a bit at bx1 followed by N bits at bx0 to get a total length congruent to 448 modulo 512. After this, the message is completed with a 64-bit integer which is the binary representation of the original message length.
For this hash processor, the quanta for entering the message is a 32-bit word, so an additional information must be provided at the end of the message entry, which is the number of valid bits in the last 32-bit word entered.

772/1749
RM0090 Rev 18
ST

--- Page 188 ---

RM0090
Hash processor (HASH)


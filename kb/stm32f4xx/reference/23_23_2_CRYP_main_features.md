# 23 Cryptographic processor (CRYP)

This section applies to STM32F415/417xx and STM32F43xxx devices.

## 23.1 CRYP introduction

The cryptographic processor can be used to both encipher and decipher data using the DES, Triple-DES or AES (128, 192, or 256) algorithms. It is a fully compliant implementation of the following standards:

*   The data encryption standard (DES) and Triple-DES (TDES) as defined by Federal Information Processing Standards Publication (FIPS PUB 46-3, 1999 October 25). It follows the American National Standards Institute (ANSI) X9.52 standard.
*   The advanced encryption standard (AES) as defined by Federal Information Processing Standards Publication (FIPS PUB 197, 2001 November 26)

The CRYP processor performs data encryption and decryption using DES and TDES algorithms in Electronic codebook (ECB) or Cipher block chaining (CBC) mode.

The CRYP peripheral is a 32-bit AHB2 peripheral. It supports DMA transfer for incoming and processed data, and has input and output FIFOs (each 8 words deep).

## 23.2 CRYP main features

*   Suitable for AES, DES and TDES enciphering and deciphering operations
*   AES
    *   Supports the ECB, CBC, CTR, CCM and GCM chaining algorithms (CCM and GCM are available on STM32F42xxx and STM32F43xxx only)
    *   Supports 128-, 192- and 256-bit keys
    *   4 × 32-bit initialization vectors (IV) used in the CBC, CTR, CCM and GCM modes

Table 111. Number of cycles required to process each 128-bit block (STM32F415/417xx)

| Algorithm / Key size | ECB | CBC | CTR |
|---|---|---|---|
| 128b | 14 |  |  |
| 192b | 16 |  |  |
| 256b | 18 |  |  |


Table 112. Number of cycles required to process each 128-bit block (STM32F43xxx)

| Algorithm / Key size | ECB | CBC | CTR | GCM | CCM |  |  |  |
|---|---|---|---|---|---|---|---|---|
| Init | Header | Payload | Tag | Init | Header | Payload | Tag |  |
| 128b | 14 | 24 | 10 | 14 | 12 | 14 | 25 | 14 |


720/1749
RM0090 Rev 18
ST

--- Page 136 ---

RM0090
Cryptographic processor (CRYP)

Table 112. Number of cycles required to process each 128-bit block (STM32F43xxx)

| 192b | 16 | 28 | 10 | 16 | 14 | 16 | 29 | 16 |
|---|---|---|---|---|---|---|---|---|
| 256b | 18 | 32 | 10 | 18 | 16 | 18 | 33 | 18 |


*   DES/TDES
    *   Direct implementation of simple DES algorithms (a single key, K1, is used)
    *   Supports the ECB and CBC chaining algorithms
    *   Supports 64-, 128- and 192-bit keys (including parity)
    *   2 × 32-bit initialization vectors (IV) used in the CBC mode
    *   16 HCLK cycles to process one 64-bit block in DES
    *   48 HCLK cycles to process one 64-bit block in TDES
*   Common to DES/TDES and AES
    *   IN and OUT FIFO (each with an 8-word depth, a 32-bit width, corresponding to 4 DES blocks or 2 AES blocks)
    *   Automatic data flow control with support of direct memory access (DMA) (using 2 channels, one for incoming data the other for processed data)
    *   Data swapping logic to support 1-, 8-, 16- or 32-bit data

ST
RM0090 Rev 18
721/1749

--- Page 137 ---

Cryptographic processor (CRYP)
RM0090


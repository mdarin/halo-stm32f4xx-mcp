> [RM0090 ch.23 §23.3] CRYP functional description


## 23.3 CRYP functional description

The cryptographic processor implements a Triple-DES (TDES, that also supports DES) core and an AES cryptographic core. Section 23.3.1 and Section 23.3.2 provide details on these cores.

Since the TDES and the AES algorithms use block ciphers, incomplete input data blocks have to be padded prior to encryption (extra bits should be appended to the trailing end of the data string). After decryption, the padding has to be discarded. The hardware does not manage the padding operation, the software has to handle it.

Figure 216 shows the block diagram of the cryptographic processor.

**Figure 216. Block diagram (STM32F415/417xx)**

32-bit AHB2 bus

Status
CRYP_SR
DMA control register
CRYP_DMACR
Interrupt registers
CRYP_IMSCR
CRYP_RIS
CRYP_MISR
Control register
CRYP_CR

CRYP_DIN
8 x 32-bit
IN FIFO
swapping

CRYP_DOUT
8 x 32-bit
OUT FIFO
swapping

Initialization vectors
CRYP_IV0...IV1
IV0...IV127
Key
CRYP_K0...K3
k255...k0

DES/TDES/AES
Processor core

ai16068b

722/1749
RM0090 Rev 18

--- Page 138 ---

RM0090
Cryptographic processor (CRYP)

Figure 217. Block diagram (STM32F43xxx)

32-bit AHB2 bus

Status
CRYP_SR
DMA control register
CRYP_DMACR
Interrupt registers
CRYP_IMSCR
CRYP_RIS
CRYP_MISR
Control register
CRYP_CR

CRYP_DIN
8 × 32-bit
IN FIFO

CRYP_DOUT
8 × 32-bit
OUT FIFO

swapping
swapping

Initialization vectors
CRYP_IV0...IV1
IV0...IV127
Key
CRYP_K0...K3
k255...k0

DES/TDES/AES
Processor core

Context swapping
CRYP_CSGCMCCM0..7
CRYP_CSGCM0..7

MS30441V1

23.3.1
DES/TDES cryptographic core

The DES/Triple-DES cryptographic core consists of three components:
* The DES algorithm (DEA)
* Multiple keys (1 for the DES algorithm, 1 to 3 for the TDES algorithm)
* The initialization vector (used in the CBC mode)

The basic processing involved in the TDES is as follows: an input block is read in the DEA and encrypted using the first key, K1 (K0 is not used in TDES mode). The output is then decrypted using the second key, K2, and encrypted using the third key, K3. The key depends on the algorithm which is used:
* DES mode: Key = [K1]
* TDES mode: Key = [K3 K2 K1]

where Kx=[KxR KxL], R = right, L = left

According to the mode implemented, the resultant output block is used to calculate the ciphertext.

Note that the outputs of the intermediate DEA stages is never revealed outside the cryptographic boundary.

ST
RM0090 Rev 18
723/1749

--- Page 139 ---

**Cryptographic processor (CRYP)**
RM0090

The TDES allows three different keying options:

*   **Three independent keys**
    The first option specifies that all the keys are independent, that is, K1, K2 and K3 are independent. FIPS PUB 46-3 – 1999 (and ANSI X9.52 – 1998) refers to this option as the Keying Option 1 and, to the TDES as 3-key TDES.
*   **Two independent keys**
    The second option specifies that K1 and K2 are independent and K3 is equal to K1, that is, K1 and K2 are independent, K3 = K1. FIPS PUB 46-3 – 1999 (and ANSI X9.52 – 1998) refers to this second option as the Keying Option 2 and, to the TDES as 2-key TDES.
*   **Three equal keys**
    The third option specifies that K1, K2 and K3 are equal, that is, K1 = K2 = K3. FIPS PUB 46-3 – 1999 (and ANSI X9.52 – 1998) refers to the third option as the Keying Option 3. This "1-key" TDES is equivalent to single DES.

FIPS PUB 46-3 – 1999 (and ANSI X9.52-1998) provides a thorough explanation of the processing involved in the four operation modes supplied by the TDEA (TDES algorithm): TDES-ECB encryption, TDES-ECB decryption, TDES-CBC encryption and TDES-CBC decryption.
This reference manual only gives a brief explanation of each mode.

**DES and TDES Electronic codebook (DES/TDES-ECB) mode**

*   **DES/TDES-ECB mode encryption**
    *Figure 218* illustrates the encryption in DES and TDES Electronic codebook (DES/TDES-ECB) mode. A 64-bit plaintext data block (P) is used after bit/byte/half-word swapping (refer to *Section 23.3.3: Data type on page 739*) as the input block (I). The input block is processed through the DEA in the encrypt state using K1. The output of this process is fed back directly to the input of the DEA where the DES is performed in the decrypt state using K2. The output of this process is fed back directly to the input of the DEA where the DES is performed in the encrypt state using K3. The resultant 64-bit output block (O) is used, after bit/byte/half-word swapping, as ciphertext (C) and it is pushed into the OUT FIFO.
*   **DES/TDES-ECB mode decryption**
    *Figure 219* illustrates the DES/TDES-ECB decryption. A 64-bit ciphertext block (C) is used, after bit/byte/half-word swapping, as the input block (I). The keying sequence is reversed compared to that used in the encryption process. The input block is processed through the DEA in the decrypt state using K3. The output of this process is fed back directly to the input of the DEA where the DES is performed in the encrypt state using K2. The new result is directly fed to the input of the DEA where the DES is performed in the decrypt state using K1. The resultant 64-bit output block (O), after bit/byte/half-word swapping, produces the plaintext (P).

724/1749
RM0090 Rev 18

--- Page 140 ---

RM0090
Cryptographic processor (CRY)

Figure 218. DES/TDES-ECB mode encryption

IN FIFO
plaintext P
P, 64 bits
DATATYPE
swapping
K1
64
DEA, encrypt
K2
64
DEA, decrypt
K3
64
DEA, encrypt
O, 64 bits
DATATYPE
swapping
C, 64 bits
OUT FIFO
ciphertext C
ai16069b

1. K: key; C: cipher text; I: input block; O: output block; P: plain text.

Figure 219. DES/TDES-ECB mode decryption

IN FIFO
ciphertext C
C, 64 bits
DATATYPE
swapping
I, 64 bits
K3
64
DEA, decrypt
K2
64
DEA, encrypt
K1
64
DEA, decrypt
O, 64 bits
DATATYPE
swapping
P, 64 bits
OUT FIFO
plaintext P
MS19021V1

1. K: key; C: cipher text; I: input block; O: output block; P: plain text.

STI
RM0090 Rev 18
725/1749

--- Page 141 ---

Cryptographic processor (CRYP)
RM0090

**DES and TDES Cipher block chaining (DES/TDES-CBC) mode**

*   DES/TDES-CBC mode encryption
    Figure 220 illustrates the DES and Triple-DES Cipher block chaining (DES/TDES-CBC) mode encryption. This mode begins by dividing a plaintext message into 64-bit data blocks. In TCBC encryption, the first input block (I1), obtained after bit/byte/half-word swapping (refer to Section 23.3.3: Data type on page 739), is formed by exclusive-ORing the first plaintext data block (P1) with a 64-bit initialization vector IV (I1 = IV ⊕ P1). The input block is processed through the DEA in the encrypt state using K1. The output of this process is fed back directly to the input of the DEA, which performs the DES in the decrypt state using K2. The output of this process is fed directly to the input of the DEA, which performs the DES in the encrypt state using K3. The resultant 64-bit output block (O1) is used directly as the ciphertext (C1), that is, C1 = O1. This first ciphertext block is then exclusive-ORed with the second plaintext data block to produce the second input block, (I2) = (C1 ⊕ P2). Note that I2 and P2 now refer to the second block. The second input block is processed through the TDEA to produce the second ciphertext block. This encryption process continues to “chain” successive cipher and plaintext blocks together until the last plaintext block in the message is encrypted. If the message does not consist of an integral number of data blocks, then the final partial data block should be encrypted in a manner specified for the application.

*   DES/TDES-CBC mode decryption
    In DES/TDES-CBC decryption (see Figure 221), the first ciphertext block (C1) is used directly as the input block (I1). The keying sequence is reversed compared to that used for the encrypt process. The input block is processed through the DEA in the decrypt state using K3. The output of this process is fed directly to the input of the DEA where the DES is processed in the encrypt state using K2. This resulting value is directly fed to the input of the DEA where the DES is processed in the decrypt state using K1. The resulting output block is exclusive-ORed with the IV (which must be the same as that used during encryption) to produce the first plaintext block (P1 = O1 ⊕ IV). The second ciphertext block is then used as the next input block and is processed through the TDEA. The resulting output block is exclusive-ORed with the first ciphertext block to produce the second plaintext data block (P2 = O2 ⊕ C1). (Note that P2 and O2 refer to the second block of data.) The TCBC decryption process continues in this manner until the last complete ciphertext block has been decrypted. Ciphertext representing a partial data block must be decrypted in a manner specified for the application.

726/1749
RM0090 Rev 18

--- Page 142 ---

RM0090
Cryptographic processor (CRYP)

**Figure 220. DES/TDES-CBC mode encryption**

ai16070b

1. K: key; C: cipher text; I: input block; O: output block; Ps: plain text before swapping (when decoding) or after swapping (when encoding); P: plain text; IV: initialization vectors.

ST
RM0090 Rev 18
727/1749

--- Page 143 ---

Cryptographic processor (CRYP)
RM0090

Figure 221. DES/TDES-CBC mode decryption

IN FIFO
ciphertext C
C, 64 bits
DATATYPE
swapping
I, 64 bits

I is written back into IV at the same time as P is pushed into the OUT FIFO

K3
64
DEA, decrypt
K2
64
DEA, encrypt
K1
64
DEA, decrypt

AHB2 data write (before CRYP is enabled)
IV0(L/R)
64
O, 64 bits
+
Ps, 64 bits
DATATYPE
swapping
P, 64 bits
OUT FIFO
plaintext P

MS19022V1

1. K: key; C: cipher text; I: input block; O: output block; Ps: plain text before swapping (when decoding) or after swapping (when encoding); P: plain text; IV: initialization vectors.

23.3.2 AES cryptographic core

The AES cryptographic core consists of three components:
* The AES algorithm (AEA: advanced encryption algorithm)
* Multiple keys
* Initialization vector(s) or Nonce

The AES utilizes keys of 3 possible lengths: 128, 192 or 256 bits and, depending on the operation mode used, zero or one 128-bit initialization vector (IV).

The basic processing involved in the AES is as follows: an input block of 128 bits is read from the input FIFO and sent to the AEA to be encrypted using the key (K0...3). The key format depends on the key size:
* If Key size = 128: Key = [K3 K2]
* If Key size = 192: Key = [K3 K2 K1]
* If Key size = 256: Key = [K3 K2 K1 K0]

where Kx=[KxR KxL], R=right, L=left

According to the mode implemented, the resultant output block is used to calculate the ciphertext.

FIPS PUB 197 (November 26, 2001) provides a thorough explanation of the processing involved in the four operation modes supplied by the AES core: AES-ECB encryption, AES-

728/1749
RM0090 Rev 18
ST

--- Page 144 ---

RM0090
Cryptographic processor (CRYP)

ECB decryption, AES-CBC encryption and AES-CBC decryption. This reference manual only gives a brief explanation of each mode.

**AES Electronic codebook (AES-ECB) mode**

*   AES-ECB mode encryption
    *Figure 222* illustrates the AES Electronic codebook (AES-ECB) mode encryption.
    In AES-ECB encryption, a 128- bit plaintext data block (P) is used after bit/byte/half-word swapping (refer to *Section 23.3.3: Data type on page 739*) as the input block (I). The input block is processed through the AEA in the encrypt state using the 128, 192 or 256-bit key. The resultant 128-bit output block (O) is used after bit/byte/half-word swapping as ciphertext (C). It is then pushed into the OUT FIFO.
*   AES-ECB mode decryption
    *Figure 223* illustrates the AES Electronic codebook (AES-ECB) mode encryption.
    To perform an AES decryption in the ECB mode, the secret key has to be prepared (it is necessary to execute the complete key schedule for encryption) by collecting the last round key, and using it as the first round key for the decryption of the ciphertext. This preparation function is computed by the AES core. Refer to *Section 23.3.6: Procedure to perform an encryption or a decryption* for more details on how to prepare the key.
    In AES-ECB decryption, a 128-bit ciphertext block (C) is used after bit/byte/half-word swapping as the input block (I). The keying sequence is reversed compared to that of the encryption process. The resultant 128-bit output block (O), after bit/byte or half-word swapping, produces the plaintext (P).

Figure 222. AES-ECB mode encryption

1. K: key; C: cipher text; I: input block; O: output block; P: plain text.
2. If Key size = 128: Key = [K3 K2].
   If Key size = 192: Key = [K3 K2 K1]
   If Key size = 256: Key = [K3 K2 K1 K0].

ST
RM0090 Rev 18
729/1749

--- Page 145 ---

Cryptographic processor (CRYP)
RM0090

Figure 223. AES-ECB mode decryption

IN FIFO
ciphertext C
C, 128 bits
DATATYPE
swapping
I, 128 bits
K0...3(1)
128/192
or 256
AEA, decrypt
O, 128 bits
DATATYPE
swapping
P, 128 bits
OUT FIFO
plaintext P

MS19023V1

1. K: key; C: cipher text; I: input block; O: output block; P: plain text.
2. If Key size = 128 => Key = [K3 K2].
   If Key size = 192 => Key = [K3 K2 K1].
   If Key size = 256 => Key = [K3 K2 K1 K0].

**AES Cipher block chaining (AES-CBC) mode**

*   **AES-CBC mode encryption**
    The AES Cipher block chaining (AES-CBC) mode decryption is shown on Figure 224.
    In AES-CBC encryption, the first input block (I1) obtained after bit/byte/half-word swapping (refer to Section 23.3.3: Data type on page 739) is formed by exclusive-ORing the first plaintext data block (P1) with a 128-bit initialization vector IV (I1 = IV ⊕ P1). The input block is processed through the AEA in the encrypt state using the 128-, 192- or 256-bit key (K0...K3). The resultant 128-bit output block (O1) is used directly as ciphertext (C1), that is, C1 = O1. This first ciphertext block is then exclusive-ORed with the second plaintext data block to produce the second input block, (I2) = (C1 ⊕ P2). Note that I2 and P2 now refer to the second block. The second input block is processed through the AEA to produce the second ciphertext block. This encryption process continues to “chain” successive cipher and plaintext blocks together until the last plaintext block in the message is encrypted. If the message does not consist of an integral number of data blocks, then the final partial data block should be encrypted in a manner specified for the application.
    In the CBC mode, like in the ECB mode, the secret key must be prepared to perform an AES decryption. Refer to Section 23.3.6: Procedure to perform an encryption or a decryption on page 744 for more details on how to prepare the key.

*   **AES-CBC mode decryption**
    In AES-CBC decryption (see Figure 225), the first 128-bit ciphertext block (C1) is used directly as the input block (I1). The input block is processed through the AEA in the decrypt state using the 128-, 192- or 256-bit key. The resulting output block is exclusive-ORed with the 128-bit initialization vector IV (which must be the same as that used during encryption) to produce the first plaintext block (P1 = O1 ⊕ IV). The second ciphertext block is then used as the next input block and is processed through the AEA. The resulting output block is exclusive-ORed with the first ciphertext block to produce the second plaintext data block (P2 = O2 ⊕ C1). (Note that P2 and O2 refer to the second

730/1749
RM0090 Rev 18
ST

--- Page 146 ---

RM0090
Cryptographic processor (CRY)

block of data.) The AES-CBC decryption process continues in this manner until the last complete ciphertext block has been decrypted. Ciphertext representing a partial data block must be decrypted in a manner specified for the application.

Figure 224. AES-CBC mode encryption

IN FIFO
plaintext P
P, 128 bits
AHB2 data write
DATATYPE
(before CRY
swapping
is enabled)
128
Ps, 128 bits
IV=[IV1 IV0](2)
128, 192
or 256
K0...3(3)
I, 128 bits
AEA, encrypt
O is written
back into IV
at the same time
as it is pushed
into the OUT FIFO
O, 128 bits
DATATYPE
swapping
C, 128 bits
OUT FIFO
ciphertext C
ai16072

1. K: key; C: cipher text; I: input block; O: output block; Ps: plain text before swapping (when decoding) or after swapping (when encoding); P: plain text; IV: Initialization vectors.
2. IVx=[IVxR IVxL], R=right, L=left.
3. If Key size = 128 => Key = [K3 K2].
If Key size = 192 => Key = [K3 K2 K1].
If Key size = 256 => Key = [K3 K2 K1 K0].

ST
RM0090 Rev 18
731/1749

--- Page 147 ---

Cryptographic processor (CRYP)
RM0090

Figure 225. AES-CBC mode decryption

IN FIFO
ciphertext C
C, 128 bits
DATATYPE
swapping
I, 128 bits
K0...3(3)
128, 192
or 256
AEA, decrypt
I is written
back into IV
at the same
time as P is pushed
into the OUT FIFO
AHB2 data write
(before CRYP
is enabled)
IV=[IV1 IV0](2)
128
O, 128 bits
Ps, 128 bits
DATATYPE
swapping
P, 128 bits
OUT FIFO
plaintext P
MS19024V1

1. K: key; C: cipher text; I: input block; O: output block; Ps: plain text before swapping (when decoding) or after swapping (when encoding); P: plain text; IV: Initialization vectors.
2. IVx=[IVxR IVxL], R=right, L=left.
3. If Key size = 128 => Key = [K3 K2].
If Key size = 192 => Key = [K3 K2 K1].
If Key size = 256 => Key = [K3 K2 K1 K0].

**AES counter mode (AES-CTR) mode**

The AES counter mode uses the AES block as a key stream generator. The generated keys are then XORed with the plaintext to obtain the cipher. For this reason, it makes no sense to speak of different CTR encryption/decryption, since the two operations are exactly the same.

In fact, given:
* Plaintext: P[0], P[1], ..., P[n] (128 bits each)
* A key K to be used (the size does not matter)
* An initial counter block (call it ICB but it has the same functionality as the IV of CBC)

The cipher is computed as follows:
C[i] = enc(iv[i]) xor P[i], where:
iv[0] = ICB and iv[i+1] = func(iv[i]), where func is an update function applied to the previous iv block; func is basically an increment of one of the fields composing the iv block.

Given that the ICB for decryption is the same as the one for encryption, the key stream generated during decryption is the same as the one generated during encryption. Then, the ciphertext is XORed with the key stream in order to retrieve the original plaintext. The decryption operation therefore acts exactly in the same way as the encryption operation.

732/1749
RM0090 Rev 18
ST

--- Page 148 ---

RM0090
Cryptographic processor (CRYP)

*Figure 226* and *Figure 227* illustrate AES-CTR encryption and decryption, respectively.

**Figure 226. AES-CTR mode encryption**

IN FIFO
plaintext P
P, 128 bits
swapping
DATATYPE
AHB2 data write
(before CRYP
is enabled)
IV0...1(L/R)
+1
K0...3
128, 192
or 256
AEA, encrypt
(I + 1) is written
back into IV
at same time
than C is pushed
in OUT FIFO
I, 128 bits
Ps, 128 bits
O, 128 bits
Cs, 128 bit
swapping
DATATYPE
C, 128 bits
OUT FIFO
ciphertext C
ai16073b

1. K: key; C: cipher text; I: input Block; o: output block; Ps: plain text before swapping (when decoding) or after swapping (when encoding); Cs: cipher text after swapping (when decoding) or before swapping (when encoding); P: plain text; IV: Initialization vectors.

STi
RM0090 Rev 18
733/1749

--- Page 149 ---

Cryptographic processor (CRYP)
RM0090

**Figure 227. AES-CTR mode decryption**

1. K: key; C: cipher text; I: input Block; o: output block; Ps: plain text before swapping (when decoding) or after swapping (when encoding); Cs: cipher text after swapping (when decoding) or before swapping (when encoding); P: plain text; IV: Initialization vectors.

*Figure 228* shows the structure of the IV block as defined by the standard [2]. It is composed of three distinct fields.

**Figure 228. Initial counter block structure for the Counter mode**

| Field | Size |
|---|---|
| Nonce | 32 bits |
| Initialization vector (IV) | 64 bits |
| Counter | 32 bits |


*   Nonce is a 32-bit, single-use value. A new nonce should be assigned to each different communication.
*   The initialization vector (IV) is a 64-bit value and the standard specifies that the encryptor must choose IV so as to ensure that a given value is used only once for a given key
*   The counter is a 32-bit big-endian integer that is incremented each time a block has been encrypted. The initial value of the counter should be set to '1'.

The block increments the least significant 32 bits, while it leaves the other (most significant) 96 bits unchanged.

734/1749
RM0090 Rev 18

--- Page 150 ---

RM0090
Cryptographic processor (CRYP)

**AES Galois/counter mode (GCM)**

The AES Galois/counter mode (GCM) allows encrypting and authenticating the plaintext, and generating the correspondent ciphertext and tag (also known as message authentication code or message integrity check). This algorithm is based on AES counter mode to ensure confidentiality. It uses a multiplier over a fixed finite field to generate the tag. An initialization vector is required at the beginning of the algorithm.

The message to be processed is split into 2 parts:

*   The header (also knows as additional authentication data): data which is authenticated but no protected (such as information for routing the packet)
*   The payload (also knows as plaintext or ciphertext): the message itself which is authenticated and encrypted.

**Note:** *The header must precede the payload and the two parts cannot be mixed together.*

The GCM standard requires to pass, at the end of the message, a specific 128-bit block composed of the size of the header (64 bits) and the size of the payload (64 bits). During the computation, the header blocks must be distinguished from the payload blocks.

In GCM mode, four steps are required to perform an encryption/decryption:

ST
RM0090 Rev 18
735/1749

--- Page 151 ---

**Cryptographic processor (CRYP)**
RM0090

1. GCM init phase
During this first step, the HASH key is calculated and saved internally to be used for processing all the blocks. It is recommended to follow the sequence below:
a) Make sure that the cryptographic processor is disabled by clearing the CRY PEN bit in the CRY P_CR register.
b) Select the GCM chaining mode by programming ALGOMODE bits to '01000' in CRY P_CR.
c) Configure GCM_CCMPH bits to '00' in CRY P_CR to start the GCM Init phase.
d) Initialize the key registers (128,192 and 256 bits) in CRY P_KEYRx as well as the initialization vector (IV).
e) Set CRY PEN bit to '1' to start the calculation of the HASH key.
f) Wait for the CRY PEN bit to be cleared to '0' before moving on to the next phase.
g) Set the CRY PEN bit to '1'.

2. GCM header phase
This step must be performed after the GCM Init phase:
h) Set the GCM_CCMPH bits to '01' in CRY P_CR to indicate that the header phase has started.
i) Write the header data. Three methods can be used:
– Program the data by blocks of 32 bits into the CRY P_DIN register, and use the IFNF flag to determine if the input FIFO can receive data. The size of the header must be a multiple of 128 bits (4 words).
– Program the data into the CRY P_DIN register by blocks of 8 words, and use the IFEM flag to determine if the input FIFO can receive data (IFEM='1'). The size of the header must be a multiple of 128 bits (4 words).
– Use the DMA.
j) Once all header data have been supplied, wait until the BUSY bit is cleared in the CRY P_SR register.

3. GCM payload phase (encryption/decryption)
This step must be performed after the GCM header phase:
k) Configure GCM_CCMPH to '10' in the CRY P_CR register.
l) Select the algorithm direction (encryption or decryption) by using the ALGODIR bit in CRY P_CR.
m) Program the payload message into the CRY P_DIN register, and use the IFNF flag to determine if the input FIFO can receive data. Alternatively, the data could be programmed into the CRY P_DIN register by blocks of 8 words and the IFEM flag used to determine if the input FIFO can receive data (IFEM='1'). In parallel, the

736/1749
RM0090 Rev 18

--- Page 152 ---

RM0090
Cryptographic processor (CRYP)

OFNE/OFFU flag of the CRYP_DOUT register can be monitored to check if the output FIFO is not empty.

n) Repeat the previous step until all payload blocks have been encrypted or decrypted. Alternatively, DMA could be used.

4. GCM final phase
This step generates the authentication tag:
o) Configure GCM_CCMPH[1:0] to '11' in CRYP_CR.
p) Write the input into the CRYP_DIN register 4 times. The input must contain the number of bits in the header (64 bits) concatenated with the number of bits in the payload (64 bits).
q) Wait till the OFNE flag (FIFO output not empty) is set to '1' in the CRYP_SR register.
r) Read the CRYP_DOUT register 4 times: the output corresponds to the authentication tag.
s) Disable the cryptographic processor (CRYPEN bit in CRYP_CR = '0')

Note:
When a decryption is performed, it is not required to compute the key at the beginning. At the end of the decryption, the generated tag should be compared with the expected tag passed with the message. In addition, the ALGODIR bit (algorithm direction) must be set to '1'. No need to disable/enable CRYP processor when moving from header phase to tag phase.

AES Galois message authentication code (GMAC)
The cryptographic processor also supports GMAC to authenticate the plaintext. It uses the GCM algorithm and a multiplier over a fixed finite field to generate the corresponding tag.

An initialization vector is required at the beginning of the algorithm.

Actually, the GMAC algorithm corresponds to the GCM algorithm applied on a message composed of the header only. As a consequence, the payload phase is not required.

AES combined cipher machine (CCM)
The CCM algorithm allows encrypting and authenticating the plaintext, as well as generating the corresponding ciphertext and tag (also known as message authentication code or message integrity check). This algorithm is based on AES counter mode to ensure confidentiality. It uses the AES CBC mode to generate a 128-bit tag.

The CCM standard (RFC 3610 Counter with CBC-MAC (CCM) dated September 2003) defines particular encoding rules for the first authentication block (called B0 in the standard). In particular, the first block includes flags, a nonce and the payload length expressed in bytes. The CCM standard specifies another format, called A or counter, for encryption/decryption. The counter is incremented during the payload phase and its 32 LSB bits are initialized to '1' during the tag generation (called A0 packet in the CCM standard).

Note:
The hardware does not perform the formatting operation of the B0 packet. It should be handled by the software.

As for the GCM algorithm, the message to be processed is split into 2 parts:
* The header (also knows as additional authentication data): data which is authenticated but no protected (such as information for routing the packet)
* The payload (also knows as plaintext or ciphertext): the message itself which is authenticated and encrypted.

RM0090 Rev 18
737/1749

--- Page 153 ---

**Cryptographic processor (CRYP)**
RM0090

*Note:* The header part must precede the payload and the two parts cannot be mixed together.

In CCM mode, 4 steps are required to perform and encryption or decryption:

1. CCM init phase
   In this first step, the B0 packet of the CCM message (1st packet) is programmed into the CRYP_DIN register. During this phase, the CRYP_DOUT register does not contain any output data.
   The following sequence must be followed:
   a) Make sure that the cryptographic processor is disabled by clearing the CRYPTEN bit in the CRYP_CR register.
   b) Select the CCM chaining mode by programming the ALGOMODE bits to '01001' in the CRYP_CR register.
   c) Configure the GCM_CCMPH bits to '00' in CRYP_CR to start the CCM Init phase.
   d) Initialize the key registers (128, 192 and 256 bits) in CRYP_KEYRx as well as the initialization vector (IV).
   e) Set the CRYPTEN bit to '1' in CRYP_CR.
   f) Program the B0 packet into the input data register.
   g) Wait for the CRYPTEN bit to be cleared before moving on to the next phase.
   h) Set CRYPTEN to '1'.

2. CCM header phase
   This step must be performed after the CCM Init phase. The sequence is identical for encryption and decryption.
   During this phase, the CRYP_DOUT register does not contain any output data.
   This phase can be skipped if there is no additional authenticated data.
   The following sequence must be followed:
   i) Set the GCM_CCMPH bit to '01' in CRYP_CR to indicate that the header phase has started.
   j) Three methods can be used:
      - Program the header data by blocks of 32 bits into the CRYP_DIN register, and use the IFNF flag to determine if the input FIFO can receive data. The size of the header must be a multiple of 128 bits (4 words).
      - Program the header data into the CRYP_DIN register by blocks of 8 words, and use the IFEM flag to determine if the input FIFO can receive data (IFEM='1'). The size of the header must be a multiple of 128 bits (4 words).
      - Use the DMA.

738/1749
RM0090 Rev 18

--- Page 154 ---

**RM0090**
**Cryptographic processor (CRYP)**

**Note:**
*The first block B1 must be formatted with the header length. This task should be handled by software.*

k) Once all header data have been supplied, wait until the BUSY flag is cleared.

3. CCM payload phase (encryption/decryption)
This step must be performed after the CCM header phase. During this phase, the encrypted/decrypted payload is stored in the CRYP_DOUT register.
The following sequence must be followed:

l) Configure GCM_CCMPH bits to '10' in CRYP_CR.

m) Select the algorithm direction (encryption or decryption) by using the ALGODIR bit in CRYP_CR.

n) Program the payload message into the CRYP_DIN register, and use the IFNF flag to determine if the input FIFO can receive data. Alternatively, the data could be programmed into the CRYP_DIN register by blocks of 8 words and the IFEM flag used to determine if the input FIFO can receive data (IFEM='1'). In parallel, the OFNE/OFFU flag of the CRYP_DOUT register can be monitored to check if the output FIFO is not empty.

o) Repeat the previous step until all payload blocks have been encrypted or decrypted. Alternatively, DMA could be used.

4. CCM final phase
This step generates the authentication tag. During this phase, the authentication tag of the message is generated and stored in the CRYP_DOUT register.

p) Configure GCM_CCMPH[1:0] bits to '11' in CRYP_CR.

q) Load the A0 initialized counter, and program the 128-bit A0 value by writing 4 times 32 bits into the CRYP_DIN register.

r) Wait till the OFNE flag (FIFO output not empty) is set to '1' in the CRYP_SR register.

s) Read the CRYP_DOUT register 4 times: the output corresponds to the encrypted authentication tag.

t) Disable the cryptographic processor (CRYPEN bit in CRYP_CR = '0')

**Note:**
*The hardware does not perform the formatting of the original B0 and B1 packets and the tag comparison between encryption and decryption. They have to be handled by software.*
*The cryptographic processor does not need to be disabled/enabled when moving from the header phase to the tag phase.*

AES cipher message authentication code (CMAC)
The CMAC algorithm allows authenticating the plaintext, and generating the corresponding tag. The CMAC sequence is identical to the CCM one, except that the payload phase is skipped.


> [RM0090 ch.23 §23.3] CRYP functional description

### 23.3.7 Context swapping**

**23.3.7 Context swapping**

If a context switching is needed because a new task launched by the OS requires this resource, the following tasks have to be performed for full context restoration (example when the DMA is used):

ST
RM0090 Rev 18
745/1749

--- Page 161 ---

Cryptographic processor (CRYP)
RM0090

**Case of the AES and DES**

1. Context saving
    a) Stop DMA transfers on the IN FIFO by clearing the DIEN bit in the CRYP_DMACR register.
    b) Wait until both the IN and OUT FIFOs are empty (IFEM=1 and OFNE=0 in the CRYP_SR register) and the BUSY bit is cleared.
    c) Stop DMA transfers on the OUT FIFO by writing the DOEN bit to 0 in the CRYP_DMACR register and clear the CRYPEN bit.
    d) Save the current configuration (bits [9:2] and bits 19 in the CRYP_CR register) and, if not in ECB mode, the initialization vectors. The key value must already be available in the memory. When needed, save the DMA status (pointers for IN and OUT messages, number of remaining bytes, etc.).
    Additional bits should be saved when GCM/GMAC or CCM/CMAC algorithms are used:
    - bits [17:16] in the CRYP_CR register
    - context swap registers:
    CRYP_CSGCMCCM0..7 for GCM/GMAC or CCM/CMAC algorithm
    CRYP_CSGCM0..7 for GCM/GMAC algorithm.
2. Configure and execute the other processing.
3. Context restoration
    a) Configure the processor as in Section 23.3.6: Procedure to perform an encryption or a decryption on page 744, Initialization with the saved configuration. For the AES-ECB or AES-CBC decryption, the key must be prepared again.
    b) If needed, reconfigure the DMA controller to transfer the rest of the message.
    c) Enable the processor by setting the CRYPEN bit and, the DMA requests by setting the DIEN and DOEN bits.

**Case of the TDES**

Context swapping can be done in the TDES in the same way as in the AES. But as the input FIFO can contain up to 4 unprocessed blocks and as the processing duration per block is higher, it can be faster in certain cases to interrupt the processing without waiting for the IN FIFO to be empty.

1. Context saving
    a) Stop DMA transfers on the IN FIFO by clearing the DIEN bit in the CRYP_DMACR register.
    b) Disable the processor by clearing the CRYPEN bit (the processing will stop at the end of the current block).
    c) Wait until the OUT FIFO is empty (OFNE=0 in the CRYP_SR register) and the BUSY bit is cleared.
    d) Stop DMA transfers on the OUT FIFO by writing the DOEN bit to 0 in the CRYP_DMACR register.
    e) Save the current configuration (bits [9:2] and bits 19 in the CRYP_CR register) and, if not in ECB mode, the initialization vectors. The key value must already be available in the memory. When needed, save the DMA status (pointers for IN and OUT messages, number of remaining bytes, etc.). Read back the data loaded in

746/1749
RM0090 Rev 18

--- Page 162 ---

RM0090
Cryptographic processor (CRYP)

the IN FIFO that have not been processed and save them in the memory until the FIFO is empty.

**Note:** In GCM/GMAC or CCM/CMAC mode, bits [17:16] of the CRYP_CR register should also be saved.

2. Configure and execute the other processing.
3. Context restoration
    a) Configure the processor as in Section 23.3.6: Procedure to perform an encryption or a decryption on page 744, Initialization with the saved configuration. For the AES-ECB or AES-CBC decryption, the key must be prepared again.
    b) Write the data that were saved during context saving into the IN FIFO.
    c) If needed, reconfigure the DMA controller to transfer the rest of the message.
    d) Enable the processor by setting the CRYPEN bit and, the DMA requests by setting the DIEN and DOEN bits.


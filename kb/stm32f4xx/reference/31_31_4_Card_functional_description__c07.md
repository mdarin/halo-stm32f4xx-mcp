> [RM0090 ch.31 §31.4] Card functional description**


### 31.4.12 SD status register

The SD status contains status bits that are related to the SD memory card proprietary features and may be used for future application-specific usage. The size of the SD Status is one data block of 512 bits. The contents of this register are transmitted to the SDIO card host if ACMD13 is sent (CMD55 followed with CMD13). ACMD13 can be sent to a card in transfer state only (card is selected).

Table 159 defines the different entries of the SD status register. The type and clear condition fields in the table are abbreviated as follows:

Type:
* E: error bit
* S: status bit
* R: detected and set for the actual command response
* X: detected and set during command execution. The SDIO card Host must poll the card by issuing the status command to read these bits

ST
RM0090 Rev 18
1045/1749

--- Page 71 ---

**Secure digital input/output interface (SDIO)**
RM0090

Clear condition:
*   A: according to the card current state
*   B: always related to the previous command. Reception of a valid command clears it (with a delay of one command)
*   C: clear by read

**Table 159. SD status**

| Bits | Identifier | Type | Value | Description | Clear condition |
|---|---|---|---|---|---|
| 511: 510 | DAT_BUS_WIDTH | S R | '00'= 1 (default)'01'= reserved'10'= 4 bit width'11'= reserved | Shows the currently defined databus width that was defined by SET_BUS_WIDTH command | A |
| 509 | SECURED_MODE | S R | '0'= Not in the mode'1'= In Secured Mode | Card is in Secured Mode of operation (refer to the "SD Security Specification"). | A |
| 508: 496 | Reserved |  |  |  |  |
| 495: 480 | SD_CARD_TYPE | S R | '00xxh'= SD Memory Cards as defined in Physical Spec Ver1.01-2.00 ('x'= don't care). The following cards are currently defined:'0000'= Regular SD RD/WR Card.'0001'= SD ROM Card | In the future, the 8 LSBs will be used to define different variations of an SD memory card (each bit will define different SD types). The 8 MSBs will be used to define SD Cards that do not comply with current SD physical layer specification. | A |
| 479: 448 | SIZE_OF_PROTECTED_AREA | S R | Size of protected area (See below) | (See below) | A |
| 447: 440 | SPEED_CLASS | S R | Speed Class of the card (See below) | (See below) | A |
| 439: 432 | PERFORMANCE_MOVE | S R | Performance of move indicated by 1 [MB/s] step.(See below) | (See below) | A |
| 431:428 | AU_SIZE | S R | Size of AU(See below) | (See below) | A |
| 427:424 | Reserved |  |  |  |  |
| 423:408 | ERASE_SIZE | S R | Number of AUs to be erased at a time | (See below) | A |
| 407:402 | ERASE_TIMEOUT | S R | Timeout value for erasing areas specified by UNIT_OF_ERASE_AU | (See below) | A |
| 401:400 | ERASE_OFFSET | S R | Fixed offset value added to erase time. | (See below) | A |
| 399:312 | Reserved |  |  |  |  |
| 311:0 | Reserved for Manufacturer |  |  |  |  |


1046/1749
RM0090 Rev 18

--- Page 72 ---

RM0090
Secure digital input/output interface (SDIO)

**SIZE_OF_PROTECTED_AREA**
Setting this field differs between standard- and high-capacity cards. In the case of a standard-capacity card, the capacity of protected area is calculated as follows:
Protected area = SIZE_OF_PROTECTED_AREA * MULT * BLOCK_LEN.
SIZE_OF_PROTECTED_AREA is specified by the unit in MULT*BLOCK_LEN.
In the case of a high-capacity card, the capacity of protected area is specified in this field:
Protected area = SIZE_OF_PROTECTED_AREA
SIZE_OF_PROTECTED_AREA is specified by the unit in bytes.

**SPEED_CLASS**
This 8-bit field indicates the speed class and the value can be calculated by Pw/2 (where Pw is the write performance).

Table 160. Speed class code field
| SPEED_CLASS | Value definition |
|---|---|
| 00h | Class 0 |
| 01h | Class 2 |
| 02h | Class 4 |
| 03h | Class 6 |
| 04h – FFh | Reserved |


**PERFORMANCE_MOVE**
This 8-bit field indicates Pm (performance move) and the value can be set by 1 [MB/sec] steps. If the card does not move used RUs (recording units), Pm should be considered as infinity. Setting the field to FFh means infinity.

Table 161. Performance move field
| PERFORMANCE_MOVE | Value definition |
|---|---|
| 00h | Not defined |
| 01h | 1 [MB/sec] |
| 02h | 02h 2 [MB/sec] |
| -------- |  |
| FEh | 254 [MB/sec] |
| FFh | Infinity |


ST
RM0090 Rev 18
1047/1749

--- Page 73 ---

Secure digital input/output interface (SDIO)
RM0090

**AU_SIZE**

This 4-bit field indicates the AU size and the value can be selected in the power of 2 base from 16 KB.

Table 162. AU_SIZE field

| AU_SIZE | Value definition |
|---|---|
| 00h | Not defined |
| 01h | 16 KB |
| 02h | 32 KB |
| 03h | 64 KB |
| 04h | 128 KB |
| 05h | 256 KB |
| 06h | 512 KB |
| 07h | 1 MB |
| 08h | 2 MB |
| 09h | 4 MB |
| Ah – Fh | Reserved |


The maximum AU size, which depends on the card capacity, is defined in Table 163. The card can be set to any AU size between RU size and maximum AU size.

Table 163. Maximum AU size

| Capacity | 16 MB-64 MB | 128 MB-256 MB | 512 MB | 1 GB-32 GB |
|---|---|---|---|---|
| Maximum AU Size | 512 KB | 1 MB | 2 MB | 4 MB |


**ERASE_SIZE**

This 16-bit field indicates NERASE. When NERASE numbers of AUs are erased, the timeout value is specified by ERASE_TIMEOUT (Refer to ERASE_TIMEOUT). The host should determine the proper number of AUs to be erased in one operation so that the host can show the progress of the erase operation. If this field is set to 0, the erase timeout calculation is not supported.

Table 164. Erase size field

| ERASE_SIZE | Value definition |
|---|---|
| 0000h | Erase timeout calculation is not supported. |
| 0001h | 1 AU |
| 0002h | 2 AU |
| 0003h | 3 AU |
| -------- |  |
| FFFFh | 65535 AU |


1048/1749
RM0090 Rev 18

--- Page 74 ---

RM0090
Secure digital input/output interface (SDIO)

> [RM0090 ch.31 §31.4] Card functional description**

### ERASE_TIMEOUT

### ERASE_TIMEOUT

This 6-bit field indicates TERASE and the value indicates the erase timeout from offset when multiple AUs are being erased as specified by ERASE_SIZE. The range of ERASE_TIMEOUT can be defined as up to 63 seconds and the card manufacturer can choose any combination of ERASE_SIZE and ERASE_TIMEOUT depending on the implementation. Determining ERASE_TIMEOUT determines the ERASE_SIZE.

Table 165. Erase timeout field

| ERASE_TIMEOUT | Value definition |
|---|---|
| 00 | Erase timeout calculation is not supported. |
| 01 | 1 [sec] |
| 02 | 2 [sec] |
| 03 | 3 [sec] |
| -------- |  |
| 63 | 63 [sec] |


> [RM0090 ch.31 §31.4] Card functional description**

### ERASE_OFFSET

### ERASE_OFFSET

This 2-bit field indicates TOFFSET and one of four values can be selected. This field is meaningless if the ERASE_SIZE and ERASE_TIMEOUT fields are set to 0.

Table 166. Erase offset field

| ERASE_OFFSET | Value definition |
|---|---|
| 0h | 0 [sec] |
| 1h | 1 [sec] |
| 2h | 2 [sec] |
| 3h | 3 [sec] |


> [RM0090 ch.31 §31.4] Card functional description**

### 31.4.13 SD I/O mode

### 31.4.13 SD I/O mode


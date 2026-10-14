### 31.5 Response formats

All responses are sent via the MCCMD command line SDIO_CMD. The response transmission always starts with the left bit of the bit string corresponding to the response code word. The code length depends on the response type.

A response always starts with a start bit (always 0), followed by the bit indicating the direction of transmission (card = 0). A value denoted by x in the tables below indicates a variable entry. All responses, except for the R3 response type, are protected by a CRC. Every command code word is terminated by the end bit (always 1).

There are five types of responses. Their formats are defined as follows:

#### 31.5.1 R1 (normal response command)

Code length = 48 bits. The 45:40 bits indicate the index of the command to be responded to, this value being interpreted as a binary-coded number (between 0 and 63). The status of the card is coded in 32 bits.

Table 173. R1 response

| Bit position | Width (bits | Value | Description |
|---|---|---|---|
| 47 | 1 | 0 | Start bit |
| 46 | 1 | 0 | Transmission bit |
| [45:40] | 6 | X | Command index |
| [39:8] | 32 | X | Card status |
| [7:1] | 7 | X | CRC7 |
| 0 | 1 | End bit |  |


#### 31.5.2 R1b

It is identical to R1 with an optional busy signal transmitted on the data line. The card may become busy after receiving these commands based on its state prior to the command reception.

#### 31.5.3 R2 (CID, CSD register)

Code length = 136 bits. The contents of the CID register are sent as a response to the CMD2 and CMD10 commands. The contents of the CSD register are sent as a response to

1054/1749
RM0090 Rev 18

--- Page 80 ---

RM0090
Secure digital input/output interface (SDIO)

CMD9. Only the bits [127...1] of the CID and CSD are transferred, the reserved bit [0] of
these registers is replaced by the end bit of the response. The card indicates that an erase
is in progress by holding MCDAT low. The actual erase time may be quite long, and the host
may issue CMD7 to deselect the card.

Table 174. R2 response

| Bit position | Width (bits | Value | Description |
|---|---|---|---|
| 135 | 1 | 0 | Start bit |
| 134 | 1 | 0 | Transmission bit |
| [133:128] | 6 | '111111' | Command index |
| [127:1] | 127 | X | Card status |
| 0 | 1 | End bit |  |


### 31.5.4 R3 (OCR register)

Code length: 48 bits. The contents of the OCR register are sent as a response to CMD1.
The level coding is as follows: restricted voltage windows = low, card busy = low.

Table 175. R3 response

| Bit position | Width (bits | Value | Description |
|---|---|---|---|
| 47 | 1 | 0 | Start bit |
| 46 | 1 | 0 | Transmission bit |
| [45:40] | 6 | '111111' | Reserved |
| [39:8] | 32 | X | OCR register |
| [7:1] | 7 | '1111111' | Reserved |
| 0 | 1 | End bit |  |


### 31.5.5 R4 (Fast I/O)

Code length: 48 bits. The argument field contains the RCA of the addressed card, the
register address to be read from or written to, and its content.

Table 176. R4 response

| Bit position | Width (bits | Value | Description |  |
|---|---|---|---|---|
| 47 | 1 | 0 | Start bit |  |
| 46 | 1 | 0 | Transmission bit |  |
| [45:40] | 6 | '100111' | CMD39 |  |
| [39:8] Argument field | [31:16] | 16 | X | RCA |
| [15:8] | 8 | X | register address |  |
| [7:0] | 8 | X | read register contents |  |


STI
RM0090 Rev 18
1055/1749

--- Page 81 ---

Secure digital input/output interface (SDIO)
RM0090

Table 176. R4 response (continued)
| Bit position | Width (bits) | Value | Description |
|---|---|---|---|
| [7:1] | 7 | X | CRC7 |
| 0 | 1 | End bit |  |


### 31.5.6 R4b

For SD I/O only: an SDIO card receiving the CMD5 will respond with a unique SDIO response R4. The format is:

Table 177. R4b response
| Bit position | Width (bits) | Value | Description |  |
|---|---|---|---|---|
| 47 | 1 | 0 | Start bit |  |
| 46 | 1 | 0 | Transmission bit |  |
| [45:40] | 6 | x | Reserved |  |
| [39:8] Argument field | 39 | 16 | X | Card is ready |
| [38:36] | 3 | X | Number of I/O functions |  |
| 35 | 1 | X | Present memory |  |
| [34:32] | 3 | X | Stuff bits |  |
| [31:8] | 24 | X | I/O ORC |  |
| [7:1] | 7 | X | Reserved |  |
| 0 | 1 | End bit |  |  |


Once an SD I/O card has received a CMD5, the I/O portion of that card is enabled to respond normally to all further commands. This I/O enable of the function within the I/O card will remain set until a reset, power cycle or CMD52 with write to I/O reset is received by the card. Note that an SD memory-only card may respond to a CMD5. The proper response for a memory-only card would be *Present memory* = 1 and *Number of I/O functions* = 0. A memory-only card built to meet the SD Memory Card specification version 1.0 would detect the CMD5 as an illegal command and not respond. The I/O aware host will send CMD5. If the card responds with response R4, the host determines the card's configuration based on the data contained within the R4 response.

### 31.5.7 R5 (interrupt request)

Only for MultiMediaCard. Code length: 48 bits. If the response is generated by the host, the RCA field in the argument will be 0x0.

Table 178. R5 response
| Bit position | Width (bits) | Value | Description |
|---|---|---|---|
| 47 | 1 | 0 | Start bit |
| 46 | 1 | 0 | Transmission bit |
| [45:40] | 6 | '101000' | CMD40 |


1056/1749
RM0090 Rev 18
ST

--- Page 82 ---

RM0090
Secure digital input/output interface (SDIO)

### Table 178. R5 response (continued)

| Bit position | Width (bits) | Value | Description |  |
|---|---|---|---|---|
| [39:8] Argument field | [31:16] | 16 | X | RCA [31:16] of winning card or of the host |
| [15:0] | 16 | X | Not defined. May be used for IRQ data |  |
| [7:1] |  | 7 | X | CRC7 |
| 0 |  | 1 | End bit |  |


### 31.5.8 R6

Only for SD I/O. The normal response to CMD3 by a memory device. It is shown in Table 179.

### Table 179. R6 response

| Bit position | Width (bits) | Value | Description |  |
|---|---|---|---|---|
| 47 | 1 | 0 | Start bit |  |
| 46 | 1 | 0 | Transmission bit |  |
| [45:40] | 6 | '101000' | CMD40 |  |
| [39:8] Argument field | [31:16] | 16 | X | RCA [31:16] of winning card or of the host |
| [15:0] | 16 | X | Not defined. May be used for IRQ data |  |
| [7:1] |  | 7 | X | CRC7 |
| 0 |  | 1 | End bit |  |


The card [23:8] status bits are changed when CMD3 is sent to an I/O-only card. In this case, the 16 bits of response are the SD I/O-only values:

* Bit [15] COM_CRC_ERROR
* Bit [14] ILLEGAL_COMMAND
* Bit [13] ERROR
* Bits [12:0] Reserved


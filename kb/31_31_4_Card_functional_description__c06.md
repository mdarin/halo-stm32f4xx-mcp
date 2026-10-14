> [RM0090 ch.31 §31.4] Card functional description**

### 31.4.11 Card status register**

**31.4.11 Card status register**

The response format R1 contains a 32-bit field named card status. This field is intended to transmit the card status information (which may be stored in a local status register) to the host. If not specified otherwise, the status entries are always related to the previously issued command.

Table 158 defines the different entries of the status. The type and clear condition fields in the table are abbreviated as follows:

1042/1749 RM0090 Rev 18 ST

--- Page 68 ---

RM0090
Secure digital input/output interface (SDIO)

Type:
* E: error bit
* S: status bit
* R: detected and set for the actual command response
* X: detected and set during command execution. The SDIO card host must poll the card by issuing the status command to read these bits.

Clear condition:
* A: according to the card current state
* B: always related to the previous command. Reception of a valid command clears it (with a delay of one command)
* C: clear by read

Table 158. Card status

| Bits | Identifier | Type | Value | Description | Clear condition |
|---|---|---|---|---|---|
| 31 | ADDRESS_OUT_OF_RANGE | E R X | '0'= no error'1'= error | The command address argument was out of the allowed range for this card.A multiple block or stream read/write operation is (although started in a valid address) attempting to read or write beyond the card capacity. | C |
| 30 | ADDRESS_MISALIGN | - | '0'= no error'1'= error | The commands address argument (in accordance with the currently set block length) positions the first data block misaligned to the card physical blocks.A multiple block read/write operation (although started with a valid address/block-length combination) is attempting to read or write a data block which is not aligned with the physical blocks of the card. | C |
| 29 | BLOCK_LEN_ERROR | - | '0'= no error'1'= error | Either the argument of a SET_BLOCKLEN command exceeds the maximum value allowed for the card, or the previously defined block length is illegal for the current command (e.g. the host issues a write command, the current block length is smaller than the maximum allowed value for the card and it is not allowed to write partial blocks) | C |
| 28 | ERASE_SEQ_ERROR | - | '0'= no error'1'= error | An error in the sequence of erase commands occurred. | C |
| 27 | ERASE_PARAM | E X | '0'= no error'1'= error | An invalid selection of erase groups for erase occurred. | C |
| 26 | WP_VIOLATION | E X | '0'= no error'1'= error | Attempt to program a write-protected block. | C |


ST
RM0090 Rev 18
1043/1749

--- Page 69 ---

Secure digital input/output interface (SDIO)
RM0090

Table 158. Card status (continued)

| Bits | Identifier | Type | Value | Description | Clear condition |
|---|---|---|---|---|---|
| 25 | CARD_IS_LOCKED | S R | '0' = card unlocked'1' = card locked | When set, signals that the card is locked by the host | A |
| 24 | LOCK_UNLOCK_FAILED | E X | '0'= no error'1'= error | Set when a sequence or password error has been detected in lock/unlock card command | C |
| 23 | COM_CRC_ERROR | E R | '0'= no error'1'= error | The CRC check of the previous command failed. | B |
| 22 | ILLEGAL_COMMAND | E R | '0'= no error'1'= error | Command not legal for the card state | B |
| 21 | CARD_ECC_FAILED | E X | '0'= success'1'= failure | Card internal ECC was applied but failed to correct the data. | C |
| 20 | CC_ERROR | E R | '0'= no error'1'= error | (Undefined by the standard) A card error occurred, which is not related to the host command. | C |
| 19 | ERROR | E X | '0'= no error'1'= error | (Undefined by the standard) A generic card error related to the (and detected during) execution of the last host command (e.g. read or write failures). | C |
| 18 | Reserved |  |  |  |  |
| 17 | Reserved |  |  |  |  |
| 16 | CID/CSD_OVERWRITE | E X | '0'= no error '1'= error | Can be either of the following errors:– The CID register has already been written and cannot be overwritten– The read-only section of the CSD does not match the card contents– An attempt to reverse the copy (set as original) or permanent WP (unprotected) bits was made | C |
| 15 | WP_ERASE_SKIP | E X | '0'= not protected'1'= protected | Set when only partial address space was erased due to existing write | C |
| 14 | CARD_ECC_DISABLED | S X | '0'= enabled'1'= disabled | The command has been executed without using the internal ECC. | A |
| 13 | ERASE_RESET | - | '0'= cleared'1'= set | An erase sequence was cleared before executing because an out of erase sequence command was received (commands other than CMD35, CMD36, CMD38 or CMD13) | C |


1044/1749
RM0090 Rev 18

--- Page 70 ---

RM0090
Secure digital input/output interface (SDIO)

Table 158. Card status (continued)

| Bits | Identifier | Type | Value | Description | Clear condition |
|---|---|---|---|---|---|
| 12:9 | CURRENT_STATE | S R | 0 = Idle1 = Ready2 = Ident3 = Stby4 = Tran5 = Data6 = Rcv7 = Prg8 = Dis9 = Btst10-15 = reserved | The state of the card when receiving the command. If the command execution causes a state change, it will be visible to the host in the response on the next command. The four bits are interpreted as a binary number between 0 and 15. | B |
| 8 | READY_FOR_DATA | S R | '0' = not ready '1' = ready | Corresponds to buffer empty signalling on the bus | - |
| 7 | SWITCH_ERROR | E X | '0' = no error '1' = switch error | If set, the card did not switch to the expected mode as requested by the SWITCH command | B |
| 6 | Reserved |  |  |  |  |
| 5 | APP_CMD | S R | '0' = Disabled '1' = Enabled | The card will expect ACMD, or an indication that the command has been interpreted as ACMD | C |
| 4 | Reserved for SD I/O Card |  |  |  |  |
| 3 | AKE_SEQ_ERROR | E R | '0' = no error '1' = error | Error in the sequence of the authentication process | C |
| 2 | Reserved for application specific commands |  |  |  |  |
| 1 | Reserved for manufacturer test mode |  |  |  |  |
| 0 |  |  |  |  |  |



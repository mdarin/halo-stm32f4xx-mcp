> [RM0090 ch.31 §31.4] Card functional description**


### Commands for the MultiMediaCard/SD module

Table 167. Block-oriented write commands

| CMD index | Type | Argument | Response format | Abbreviation | Description |
|---|---|---|---|---|---|
| CMD23 | ac | [31:16] set to 0 [15:0] number of blocks | R1 | SET_BLOCK_COUNT | Defines the number of blocks which are going to be transferred in the multiple-block read or write command that follows. |
| CMD24 | adtc | [31:0] data address | R1 | WRITE_BLOCK | Writes a block of the size selected by the SET_BLOCKLEN command. |


ST
RM0090 Rev 18
1051/1749

--- Page 77 ---

Secure digital input/output interface (SDIO)
RM0090

Table 167. Block-oriented write commands (continued)

| CMD index | Type | Argument | Response format | Abbreviation | Description |
|---|---|---|---|---|---|
| CMD25 | adtc | [31:0] data address | R1 | WRITE_MULTIPLE_BLOCK | Continuously writes blocks of data until a STOP_TRANSMISSION follows or the requested number of blocks has been received. |
| CMD26 | adtc | [31:0] stuff bits | R1 | PROGRAM_CID | Programming of the card identification register. This command must be issued only once per card. The card contains hardware to prevent this operation after the first programming. Normally this command is reserved for manufacturer. |
| CMD27 | adtc | [31:0] stuff bits | R1 | PROGRAM_CSD | Programming of the programmable bits of the CSD. |


Table 168. Block-oriented write protection commands

| CMD index | Type | Argument | Response format | Abbreviation | Description |
|---|---|---|---|---|---|
| CMD28 | ac | [31:0] data address | R1b | SET_WRITE_PROT | If the card has write protection features, this command sets the write protection bit of the addressed group. The properties of write protection are coded in the card-specific data (WP_GRP_SIZE). |
| CMD29 | ac | [31:0] data address | R1b | CLR_WRITE_PROT | If the card provides write protection features, this command clears the write protection bit of the addressed group. |
| CMD30 | adtc | [31:0] write protect data address | R1 | SEND_WRITE_PROT | If the card provides write protection features, this command asks the card to send the status of the write protection bits. |
| CMD31 | Reserved |  |  |  |  |


Table 169. Erase commands

| CMD index | Type | Argument | Response format | Abbreviation | Description |
|---|---|---|---|---|---|
| CMD32 ... CMD34 | Reserved. These command indexes cannot be used in order to maintain backward compatibility with older versions of the MultiMediaCard. |  |  |  |  |
| CMD35 | ac | [31:0] data address | R1 | ERASE_GROUP_START | Sets the address of the first erase group within a range to be selected for erase. |
| CMD36 | ac | [31:0] data address | R1 | ERASE_GROUP_END | Sets the address of the last erase group within a continuous range to be selected for erase. |


1052/1749
RM0090 Rev 18
ST

--- Page 78 ---

RM0090
Secure digital input/output interface (SDIO)

Table 169. Erase commands (continued)
| CMD index | Type | Argument | Response format | Abbreviation | Description |
|---|---|---|---|---|---|
| CMD37 | Reserved. This command index cannot be used in order to maintain backward compatibility with older versions of the MultiMediaCards |  |  |  |  |
| CMD38 | ac | [31:0] stuff bits | R1 | ERASE | Erases all previously selected write blocks. |


Table 170. I/O mode commands
| CMD index | Type | Argument | Response format | Abbreviation | Description |
|---|---|---|---|---|---|
| CMD39 | ac | [31:16] RCA [15:15] register write flag [14:8] register address [7:0] register data | R4 | FAST_IO | Used to write and read 8-bit (register) data fields. The command addresses a card and a register and provides the data for writing if the write flag is set. The R4 response contains data read from the addressed register. This command accesses application-dependent registers that are not defined in the MultiMediaCard standard. |
| CMD40 | bcr | [31:0] stuff bits | R5 | GO_IRQ_STATE | Places the system in the interrupt mode. |
| CMD41 | Reserved |  |  |  |  |


Table 171. Lock card
| CMD index | Type | Argument | Response format | Abbreviation | Description |
|---|---|---|---|---|---|
| CMD42 | adtc | [31:0] stuff bits | R1b | LOCK_UNLOCK | Sets/resets the password or locks/unlocks the card. The size of the data block is set by the SET_BLOCK_LEN command. |
| CMD43 ... CMD54 | Reserved |  |  |  |  |


Table 172. Application-specific commands
| CMD index | Type | Argument | Response format | Abbreviation | Description |
|---|---|---|---|---|---|
| CMD55 | ac | [31:16] RCA [15:0] stuff bits | R1 | APP_CMD | Indicates to the card that the next command bits is an application specific command rather than a standard command |
| CMD56 | adtc | [31:1] stuff bits [0]: RD/WR |  |  | Used either to transfer a data block to the card or to get a data block from the card for general purpose/application-specific commands. The size of the data block shall be set by the SET_BLOCK_LEN command. |


ST
RM0090 Rev 18
1053/1749

--- Page 79 ---

Secure digital input/output interface (SDIO)
RM0090

Table 172. Application-specific commands (continued)

| CMD index | Type | Argument | Response format | Abbreviation | Description |
|---|---|---|---|---|---|
| CMD57 ... CMD59 | Reserved. |  |  |  |  |
| CMD60 ... CMD63 | Reserved for manufacturer. |  |  |  |  |



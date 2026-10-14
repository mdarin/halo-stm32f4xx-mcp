> [RM0090 ch.31 §31.4] Card functional description**

### 31.4 Card functional description**

**31.4 Card functional description**

> [RM0090 ch.31 §31.4] Card functional description**

### 31.4.1 Card identification mode**

**31.4.1 Card identification mode**
While in card identification mode the host resets all cards, validates the operation voltage range, identifies cards and sets a relative card address (RCA) for each card on the bus. All data communications in the card identification mode use the command line (CMD) only.

> [RM0090 ch.31 §31.4] Card functional description**

### 31.4.2 Card reset**

**31.4.2 Card reset**
The GO_IDLE_STATE command (CMD0) is the software reset command and it puts the MultiMediaCard and SD memory in the Idle state. The IO_RW_DIRECT command (CMD52) resets the SD I/O card. After power-up or CMD0, all cards output bus drivers are in the high-impedance state and the cards are initialized with a default relative card address (RCA=0x0001) and with a default driver stage register setting (lowest speed, highest driving current capability).

> [RM0090 ch.31 §31.4] Card functional description**

### 31.4.3 Operating voltage range validation**

**31.4.3 Operating voltage range validation**
All cards can communicate with the SDIO card host using any operating voltage within the specification range. The supported minimum and maximum VDD values are defined in the operation conditions register (OCR) on the card.

1034/1749
RM0090 Rev 18
ST

--- Page 60 ---

RM0090
Secure digital input/output interface (SDIO)

Cards that store the card identification number (CID) and card specific data (CSD) in the payload memory are able to communicate this information only under data-transfer VDD conditions. When the SDIO card host module and the card have incompatible VDD ranges, the card is not able to complete the identification cycle and cannot send CSD data. For this purpose, the special commands, SEND_OP_COND (CMD1), SD_APP_OP_COND (ACMD41 for SD Memory), and IO_SEND_OP_COND (CMD5 for SD I/O), are designed to provide a mechanism to identify and reject cards that do not match the VDD range desired by the SDIO card host. The SDIO card host sends the required VDD voltage window as the operand of these commands. Cards that cannot perform data transfer in the specified range disconnect from the bus and go to the inactive state.

By using these commands without including the voltage range as the operand, the SDIO card host can query each card and determine the common voltage range before placing out-of-range cards in the inactive state. This query is used when the SDIO card host is able to select a common voltage range or when the user requires notification that cards are not usable.


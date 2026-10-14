## 31.2 SDIO bus topology

Communication over the bus is based on command and data transfers.

The basic transaction on the MultiMediaCard/SD/SD I/O bus is the command/response transaction. These types of bus transaction transfer their information directly within the command or response structure. In addition, some operations have a data token.

Data transfers to/from SD/SDIO memory cards are done in data blocks. Data transfers to/from MMC are done data blocks or streams. Data transfers to/from the CE-ATA Devices are done in data blocks.

**Figure 321. SDIO “no response” and “no data” operations**

From host to card(s
From host to card
From card to host

SDMMC_CMD
Command
Command
Response

SDMMC_D

Operation (no response)
Operation (no data)

ai14734b

**Figure 322. SDIO (multiple) block read operation**

From host to card
From card to host
data from card to host
Stop command
stops data transfer

SDMMC_CMD
Command
Response
Command
Response

SDMMC_D
Data block crc
Data block crc
Data block crc

Block read operation
Multiple block read operation
Data stop operation

ai14735b

1020/1749
RM0090 Rev 18

--- Page 46 ---

RM0090
Secure digital input/output interface (SDIO)

**Figure 323. SDIO (multiple) block write operation**

From host to card From card to host
data from host to card
SDMMC_CMD Command Response
SDMMC_D Data block crc Busy
Stop command stops data transfer
Command Response
Data block crc Busy
Block write operation
Data stop operation
Multiple block write operation
ai14737b

*Note:* The SDIO will not send any data as long as the Busy signal is asserted (SDIO_D0 pulled low).

**Figure 324. SDIO sequential read operation**

From host to card(s) From card to host
Data from card to host
SDMMC_CMD Command Response
SDMMC_D Data stream
Stop command stops data transfer
Command Response
Data transfer operation
Data stop operation
ai14738b

**Figure 325. SDIO sequential write operation**

From host to card(s) From card to host
Data from host to card
SDMMC_CMD Command Response
SDMMC_D Data stream
Stop command stops data transfer
Command Response
Data transfer operation
Data stop operation
ai14739b

STi
RM0090 Rev 18
1021/1749

--- Page 47 ---

Secure digital input/output interface (SDIO)
RM0090


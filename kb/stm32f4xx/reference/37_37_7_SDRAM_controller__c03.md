> [RM0090 ch.37 §37.7] SDRAM controller


## SDRAM controller read cycle

The SDRAM controller accepts single and burst read requests and translates them into single memory accesses. In both cases, the SDRAM controller keeps track of the active row in each bank to be able to perform consecutive read accesses in different banks (Multibank ping-pong access).

**Figure 479. Burst read SDRAM access**

MS30449V3

The FMC SDRAM controller features a Cacheable read FIFO (6 lines x 32 bits). It is used to store data read in advance during the CAS latency period and during the RPIPE delay. The following the formula is applied:

Number of anticipated data = CAS latency + 1 + (RPIPE delay)/ 2

The RBURST bit must be set in the FMC_SDCR1 register to anticipate the next read access.

Example:
* CAS latency = 3, RPIPE delay = 0: 4 data (not committed) are stored in the FIFO.
* CAS latency = 3, RPIPE delay = 2: 5 data (not committed) are stored in the FIFO.

The read FIFO features a 14-bit address tag to each line to identify its content: 11 bits for the column address, 2 bits to select the internal bank and the active row, and 1 bit to select the SDRAM device

When the end of the row is reached in advance during an AHB burst read, the data read in advance (not committed) are not stored in the read FIFO. For single read access, data are correctly stored in the FIFO.

1666/1749
RM0090 Rev 18

--- Page 107 ---

RM0090
Flexible memory controller (FMC)

Each time a read request occurs, the SDRAM controller checks:

*   If the address matches one of the address tags, data are directly read from the FIFO and the corresponding address tag/ line content is cleared and the remaining data in the FIFO are compacted to avoid empty lines.
*   Otherwise, a new read command is issued to the memory and the FIFO is updated with new data. If the FIFO is full, the older data are lost.

**Figure 480. Logic diagram of Read access with RBURST bit set (CAS=2, RPIPE=0)**

1st Read access: Requested data is not in the FIFO

FMC SDRAM Controller
AHB Master
read request@0x00
Data 1
SDRAM Device
(CAS = 2)
6 lines FIFO
@0x04 Data 2
@0x08 Data 3
... ...
Add. Tag read FIFO
Data stored in FIFO in advance during the CAS latency period

2nd Read access : Requested data was previously stored in the FIFO

Address matches with one of the address tags
FMC SDRAM Controller
AHB Master
read request@0x04
Data 2
SDRAM Device
(CAS = 2)
6 lines FIFO
@0x04 Data 2
@0x08 Data 3
... ...
Add. Tag read FIFO
Data read from FIFO

MS30445V1

During a write access or a Precharge command, the read FIFO is flushed and ready to be filled with new data.

After the first read request, if the current access was not performed to a row boundary, the SDRAM controller anticipates the next read access during the CAS latency period and the RPIPE delay (if configured). This is done by incrementing the memory address. The following condition must be met:

*   RBURST control bit should be set to '1' in the FMC_SDCR1 register.

ST
RM0090 Rev 18
1667/1749

--- Page 108 ---

**Flexible memory controller (FMC)**
RM0090

The address management depends on the next AHB request:
*   Next AHB request is sequential (AHB Burst)
    In this case, the SDRAM controller increments the address.
*   Next AHB request is not sequential
    *   If the new read request targets the same row or another active row, the new address is passed to the memory and the master is stalled for the CAS latency period, waiting for the new data from memory.
    *   If the new read request does not target an active row, the SDRAM controller generates a Precharge command, activates the new row, and initiates a read command.

If the RURST is reset, the read FIFO is not used.

**Row and bank boundary management**

When a read or write access crosses a row boundary, if the next read or write access is sequential and the current access was performed to a row boundary, the SDRAM controller executes the following operations:
1.  Precharge of the active row,
2.  Activation of the new row
3.  Start of a read/write command.

At a row boundary, the automatic activation of the next row is supported for all columns and data bus width configurations.

If necessary, the SDRAM controller inserts additional clock cycles between the following commands:
*   Between Precharge and Active commands to match TRP parameter (only if the next access is in a different row in the same bank),
*   Between Active and Read commands to match the TRCD parameter.

These parameters are defined into the FMC_SDTRx register.

Refer to Figure 481 and Figure 482 for read and burst write access crossing a row boundary.

1668/1749
RM0090 Rev 18
ST

--- Page 109 ---

RM0090
Flexible memory controller (FMC)

**Figure 481. Read access crossing row boundary**

TRP = 3
TRCD = 3
CAS latency = 2

SDNE
SDCLK
A[12:0]
NRAS
NCAS
NWE
Data[31:0]

Row n
Col a
Row n +1
Col a
Col b
Dna
Dn+1 a

Precharge
Activate Row
Read Command

MS30446V2

**Figure 482. Write access crossing row boundary**

TRP = 3
TRCD = 3

SDNE
SDCLK
A[12:0]
NRAS
NCAS
NWE
Data[31:0]

Cna
Colb
Row n+1
Cola
Colb
Dna
Dnb
Dn+1a
Dn+1b

Precharge
Activate Row
Write command

MS30447V2

ST
RM0090 Rev 18
1669/1749

--- Page 110 ---

**Flexible memory controller (FMC)**
RM0090

If the next access is sequential and the current access crosses a bank boundary, the SDRAM controller activates the first row in the next bank and initiates a new read/write command. Two cases are possible:

*   If the current bank is not the last one, the active row in the new bank must be precharged. At a bank boundary, the automatic activation of the next row is supported for all rows/columns and data bus width configuration.
*   For 13-bit row address, 11-bit column address, 4 internal banks and bus width 32-bit SDRAM memories, if the current bank is the last one and the selected SDRAM device is connected to Bank 1, the SDRAM controller continues to read/write from the second SDRAM device (assuming it has been initialized):
    a) The SDRAM controller activates the first row (after precharging the active row, if there is already an active row in the first internal bank, and initiates a new read/write command.
    b) If the first row is already activated, the SDRAM controller just initiates a read/write command.

**Note:** At bank boundary, if the current bank is the last one, the automatic activation of the next row is supported only when addressing 13-bit rows, 11-bit columns, 4 internal banks and 32-bit data bus SDRAM devices. Otherwise, the SDRAM address range is violated and an AHB error is generated.

**SDRAM controller refresh cycle**

The Auto-refresh command is used to refresh the SDRAM device content. The SDRAM controller periodically issues auto-refresh commands. An internal counter is loaded with the COUNT value in the register FMC_SDRTR. This value defines the number of memory clock cycles between the refresh cycles (refresh rate). When this counter reaches zero, an internal pulse is generated.

If a memory access is ongoing, the auto-refresh request is delayed. However, if the memory access and the auto-refresh requests are generated simultaneously, the auto-refresh request takes precedence.

If the memory access occurs during an auto-refresh operation, the request is buffered and processed when the auto-refresh is complete.

If a new auto-refresh request occurs while the previous one was not served, the RE (Refresh Error) bit is set in the Status register. An Interrupt is generated if it has been enabled (REIE = '1').

If SDRAM lines are not in idle state (not all row are closed), the SDRAM controller generates a PALL (Precharge ALL) command before the auto-refresh.

If the Auto-refresh command is generated by the FMC_SDCMR Command Mode register (Mode bits = '011'), a PALL command (Mode bits = '010') must be issued first.

1670/1749
RM0090 Rev 18
ST

--- Page 111 ---

RM0090
Flexible memory controller (FMC)


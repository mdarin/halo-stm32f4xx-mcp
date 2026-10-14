### 33.6.7 Tx DMA configuration

**TxDMA operation: default (non-OSF) mode**

The transmit DMA engine in default mode proceeds as follows:

1. The user sets up the transmit descriptor (TDES0-TDES3) and sets the OWN bit (TDES0[31]) after setting up the corresponding data buffer(s) with Ethernet frame data.
2. Once the ST bit (ETH_DMAOMR register[13]) is set, the DMA enters the Run state.
3. While in the Run state, the DMA polls the transmit descriptor list for frames requiring transmission. After polling starts, it continues in either sequential descriptor ring order or chained order. If the DMA detects a descriptor flagged as owned by the CPU, or if an error condition occurs, transmission is suspended and both the Transmit Buffer

1166/1749 RM0090 Rev 18 ST

--- Page 192 ---

RM0090
Ethernet (ETH): media access control (MAC) with DMA controller

Unavailable (ETH_DMASR register[2]) and Normal Interrupt Summary (ETH_DMASR register[16]) bits are set. The transmit engine proceeds to Step 9.

4. If the acquired descriptor is flagged as owned by DMA (TDES0[31] is set), the DMA decodes the transmit data buffer address from the acquired descriptor.
5. The DMA fetches the transmit data from the STM32F4xx memory and transfers the data.
6. If an Ethernet frame is stored over data buffers in multiple descriptors, the DMA closes the intermediate descriptor and fetches the next descriptor. Steps 3, 4, and 5 are repeated until the end of Ethernet frame data is transferred.
7. When frame transmission is complete, if IEEE 1588 time stamping was enabled for the frame (as indicated in the transmit status) the time stamp value is written to the transmit descriptor (TDES2 and TDES3) that contains the end-of-frame buffer. The status information is then written to this transmit descriptor (TDES0). Because the OWN bit is cleared during this step, the CPU now owns this descriptor. If time stamping was not enabled for this frame, the DMA does not alter the contents of TDES2 and TDES3.
8. Transmit Interrupt (ETH_DMASR register [0]) is set after completing the transmission of a frame that has Interrupt on Completion (TDES1[31]) set in its last descriptor. The DMA engine then returns to Step 3.
9. In the Suspend state, the DMA tries to re-acquire the descriptor (and thereby returns to Step 3) when it receives a transmit poll demand, and the Underflow Interrupt Status bit is cleared.

*Figure 377* shows the TxDMA transmission flow in default mode.

ST
RM0090 Rev 18
1167/1749

--- Page 193 ---

Ethernet (ETH): media access control (MAC) with DMA controller
RM0090

Figure 377. TxDMA operation in Default mode

Start TxDMA
Stop TxDMA
(Re-)fetch next descriptor
Poll demand
(AHB) error?
TxDMA suspended
Own bit set?
Transfer data from buffer(s)
(AHB) error?
Frame xfer complete?
Close intermediate descriptor
Wait for Tx status
Time stamp present?
Write time stamp to TDES2 and TDES3
(AHB) error?
Write status word to TDES0
(AHB) error?

ai15639

**TxDMA operation: OSF mode**

While in the Run state, the transmit process can simultaneously acquire two frames without closing the Status descriptor of the first (if the OSF bit is set in ETH_DMAOMR register[2]). As the transmit process finishes transferring the first frame, it immediately polls the transmit descriptor list for the second frame. If the second frame is valid, the transmit process transfers this frame before writing the first frame's status information. In OSF mode, the Run-state transmit DMA operates according to the following sequence:

1168/1749
RM0090 Rev 18
ST

--- Page 194 ---

RM0090
Ethernet (ETH): media access control (MAC) with DMA controller

1. The DMA operates as described in steps 1–6 of the TxDMA (default mode).
2. Without closing the previous frame’s last descriptor, the DMA fetches the next descriptor.
3. If the DMA owns the acquired descriptor, the DMA decodes the transmit buffer address in this descriptor. If the DMA does not own the descriptor, the DMA goes into Suspend mode and skips to Step 7.
4. The DMA fetches the Transmit frame from the STM32F4xx memory and transfers the frame until the end of frame data are transferred, closing the intermediate descriptors if this frame is split across multiple descriptors.
5. The DMA waits for the transmission status and time stamp of the previous frame. When the status is available, the DMA writes the time stamp to TDES2 and TDES3, if such time stamp was captured (as indicated by a status bit). The DMA then writes the status, with a cleared OWN bit, to the corresponding TDES0, thus closing the descriptor. If time stamping was not enabled for the previous frame, the DMA does not alter the contents of TDES2 and TDES3.
6. If enabled, the Transmit interrupt is set, the DMA fetches the next descriptor, then proceeds to Step 3 (when Status is normal). If the previous transmission status shows an underflow error, the DMA goes into Suspend mode (Step 7).
7. In Suspend mode, if a pending status and time stamp are received by the DMA, it writes the time stamp (if enabled for the current frame) to TDES2 and TDES3, then writes the status to the corresponding TDES0. It then sets relevant interrupts and returns to Suspend mode.
8. The DMA can exit Suspend mode and enter the Run state (go to Step 1 or Step 2 depending on pending status) only after receiving a Transmit Poll demand (ETH_DMATPDR register).

*Figure 378* shows the basic flowchart in OSF mode.

ST
RM0090 Rev 18
1169/1749

--- Page 195 ---

Ethernet (ETH): media access control (MAC) with DMA controller
RM0090

Figure 378. TxDMA operation in OSF mode

Start TxDMA
Stop TxDMA
(Re-)fetch next descriptor
(AHB error?
Yes
No
Own bit set?
Yes
Transfer data from buffer(s)
(AHB error?
Yes
No
Frame xfer complete?
Yes
Second frame?
Yes
No
Wait for previous frame's Tx status
Write time stamp to TDES2 & TDES3 for previous frame
Time stamp present?
Yes
No
Write status word to prev. frame's TDES0
(AHB error?
Yes
No
Close intermediate descriptor
Time stamp present?
Yes
No
Write status word to prev. frame's TDES0
(AHB error?
Yes
No
TxDMA suspended
Poll demand
Previous frame status available
Time stamp present?
Yes
No
Write time stamp to TDES2 & TDES3 for previous frame
(AHB error?
Yes
No
Write status word to prev. frame's TDES0
(AHB error?
Yes
No

ai15640

**Transmit frame processing**

The transmit DMA expects that the data buffers contain complete Ethernet frames, excluding preamble, pad bytes, and FCS fields. The DA, SA, and Type/Len fields contain valid data. If the transmit descriptor indicates that the MAC core must disable CRC or pad insertion, the buffer must have complete Ethernet frames (excluding preamble), including the CRC bytes. Frames can be data-chained and span over several buffers. Frames have to be delimited by the first descriptor (TDES0[28]) and the last descriptor (TDES0[29]). As the transmission starts, TDES0[28] has to be set in the first descriptor. When this occurs, the frame data are transferred from the memory buffer to the Transmit FIFO. Concurrently, if the last descriptor (TDES0[29]) of the current frame is cleared, the transmit process attempts to acquire the next descriptor. The transmit process expects TDES0[28] to be cleared in this descriptor. If TDES0[29] is cleared, it indicates an intermediary buffer. If TDES0[29] is set, it

1170/1749
RM0090 Rev 18
ST
--- Page 1 ---

RM0090
Ethernet (ETH): media access control (MAC) with DMA controller

indicates the last buffer of the frame. After the last buffer of the frame has been transmitted, the DMA writes back the final status information to the transmit descriptor 0 (TDES0) word of the descriptor that has the last segment set in transmit descriptor 0 (TDES0[29]). At this time, if Interrupt on Completion (TDES0[30]) is set, Transmit Interrupt (in ETH_DMASR register [0]) is set, the next descriptor is fetched, and the process repeats. Actual frame transmission begins after the Transmit FIFO has reached either a programmable transmit threshold (ETH_DMAOMR register[16:14]), or a full frame is contained in the FIFO. There is also an option for the Store and forward mode (ETH_DMAOMR register[21]). Descriptors are released (OWN bit TDES0[31] is cleared) when the DMA finishes transferring the frame.

**Transmit polling suspended**

Transmit polling can be suspended by either of the following conditions:

*   The DMA detects a descriptor owned by the CPU (TDES0[31]=0) and the Transmit buffer unavailable flag is set (ETH_DMASR register[2]). To resume, the driver must give descriptor ownership to the DMA and then issue a Poll Demand command.
*   A frame transmission is aborted when a transmit error due to underflow is detected. The appropriate Transmit Descriptor 0 (TDES0) bit is set. If the second condition occurs, both the Abnormal Interrupt Summary (in ETH_DMASR register [15]) and Transmit Underflow bits (in ETH_DMASR register[5]) are set, and the information is written to Transmit Descriptor 0, causing the suspension. If the DMA goes into Suspend state due to the first condition, then both the Normal Interrupt Summary (ETH_DMASR register [16]) and Transmit Buffer Unavailable (ETH_DMASR register[2]) bits are set. In both cases, the position in the transmit list is retained. The retained position is that of the descriptor following the last descriptor closed by the DMA. The driver must explicitly issue a Transmit Poll Demand command after rectifying the suspension cause.

**Normal Tx DMA descriptors**

The normal transmit descriptor structure consists of four 32-bit words as shown in Figure 379. The bit descriptions of TDES0, TDES1, TDES2 and TDES3 are given below.

Note that enhanced descriptors must be used if time stamping is activated (ETH_PTPTSCR bit 0, TSE=1) or if IPv4 checksum offload is activated (ETH_MACCR bit 10, IPCO=1).

Figure 379. Normal transmit descriptor

| 31 | 0 |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|
| TDES 0 | TDES 1 | TDES 2 | TDES 3 |  |  |  |  |
| OWN | Ctrl [30:26] | T T S E | Res. 24 | Ctrl [23:20] | Reserved [19:18] | T T S | Status [16:0] |
| Reserved [31:29] | Buffer 2 byte count [28:16] | Reserved [15:13] | Buffer 1 byte count [12:0] |  |  |  |  |
| Buffer 1 address [31:0] / Time stamp low [31:0] |  |  |  |  |  |  |  |
| Buffer 2 address [31:0] or Next descriptor address [31:0] / Time stamp high [31:0] |  |  |  |  |  |  |  |


ai15642b

ST
RM0090 Rev 18
1171/1749

--- Page 2 ---

**Ethernet (ETH): media access control (MAC) with DMA controller**

RM0090

*   **TDES0: Transmit descriptor Word0**

The application software has to program the control bits [30:26]+[23:20] plus the OWN bit [31] during descriptor initialization. When the DMA updates the descriptor (or writes it back), it resets all the control bits plus the OWN bit, and reports only the status bits.

| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| OWN | IC | LS | FS | DC | DP | TTSE | Res | CIC | TER | TCH | Res. | TTSS | IHE |  |  |
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
| ES | JT | FF | IPE | LCA | NC | LCO | EC | VF | CC | ED | UF | DB |  |  |  |
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


**Bit 31 OWN:** Own bit

When set, this bit indicates that the descriptor is owned by the DMA. When this bit is reset, it indicates that the descriptor is owned by the CPU. The DMA clears this bit either when it completes the frame transmission or when the buffers allocated in the descriptor are read completely. The ownership bit of the frame's first descriptor must be set after all subsequent descriptors belonging to the same frame have been set.

**Bit 30 IC:** Interrupt on completion

When set, this bit sets the Transmit Interrupt (Register 5[0]) after the present frame has been transmitted.

**Bit 29 LS:** Last segment

When set, this bit indicates that the buffer contains the last segment of the frame.

**Bit 28 FS:** First segment

When set, this bit indicates that the buffer contains the first segment of a frame.

**Bit 27 DC:** Disable CRC

When this bit is set, the MAC does not append a cyclic redundancy check (CRC) to the end of the transmitted frame. This is valid only when the first segment (TDES0[28]) is set.

**Bit 26 DP:** Disable pad

When set, the MAC does not automatically add padding to a frame shorter than 64 bytes. When this bit is reset, the DMA automatically adds padding and CRC to a frame shorter than 64 bytes, and the CRC field is added despite the state of the DC (TDES0[27]) bit. This is valid only when the first segment (TDES0[28]) is set.

**Bit 25 TTSE:** Transmit time stamp enable

When TTSE is set and when TSE is set (ETH_PTPTSCR bit 0), IEEE1588 hardware time stamping is activated for the transmit frame described by the descriptor. This field is only valid when the First segment control bit (TDES0[28]) is set.

**Bit 24** Reserved, must be kept at reset value.

**Bits 23:22 CIC:** Checksum insertion control

These bits control the checksum calculation and insertion. Bit encoding is as shown below:

00: Checksum Insertion disabled
01: Only IP header checksum calculation and insertion are enabled
10: IP header checksum and payload checksum calculation and insertion are enabled, but pseudo-header checksum is not calculated in hardware
11: IP Header checksum and payload checksum calculation and insertion are enabled, and pseudo-header checksum is calculated in hardware.

1172/1749
RM0090 Rev 18

--- Page 3 ---

RM0090
Ethernet (ETH): media access control (MAC) with DMA controller

**Bit 21 TER:** Transmit end of ring
When set, this bit indicates that the descriptor list reached its final descriptor. The DMA returns to the base address of the list, creating a descriptor ring.

**Bit 20 TCH:** Second address chained
When set, this bit indicates that the second address in the descriptor is the next descriptor address rather than the second buffer address. When TDES0[20] is set, TBS2 (TDES1[28:16]) is a "don't care" value. TDES0[21] takes precedence over TDES0[20].

**Bits 19:18** Reserved, must be kept at reset value.

**Bit 17 TTSS:** Transmit time stamp status
This field is used as a status bit to indicate that a time stamp was captured for the described transmit frame. When this bit is set, TDES2 and TDES3 have a time stamp value captured for the transmit frame. This field is only valid when the descriptor's Last segment control bit (TDES0[29]) is set.
Note that when enhanced descriptors are enabled (EDFE=1 in ETH_DMABMR), TTSS=1 indicates that TDES6 and TDES7 have the time stamp value.

**Bit 16 IHE:** IP header error
When set, this bit indicates that the MAC transmitter detected an error in the IP datagram header. The transmitter checks the header length in the IPv4 packet against the number of header bytes received from the application and indicates an error status if there is a mismatch. For IPv6 frames, a header error is reported if the main header length is not 40 bytes. Furthermore, the Ethernet length/type field value for an IPv4 or IPv6 frame must match the IP header version received with the packet. For IPv4 frames, an error status is also indicated if the Header Length field has a value less than 0x5.

**Bit 15 ES:** Error summary
Indicates the logical OR of the following bits:
TDES0[14]: Jabber timeout
TDES0[13]: Frame flush
TDES0[11]: Loss of carrier
TDES0[10]: No carrier
TDES0[9]: Late collision
TDES0[8]: Excessive collision
TDES0[2]: Excessive deferral
TDES0[1]: Underflow error
TDES0[16]: IP header error
TDES0[12]: IP payload error

**Bit 14 JT:** Jabber timeout
When set, this bit indicates that the MAC transmitter has experienced a jabber timeout. This bit is only set when the MAC configuration register's JD bit is not set.

**Bit 13 FF:** Frame flushed
When set, this bit indicates that the DMA/MTL flushed the frame due to a software Flush command given by the CPU.

**Bit 12 IPE:** IP payload error
When set, this bit indicates that MAC transmitter detected an error in the TCP, UDP, or ICMP IP datagram payload. The transmitter checks the payload length received in the IPv4 or IPv6 header against the actual number of TCP, UDP or ICMP packet bytes received from the application and issues an error status in case of a mismatch.

ST
RM0090 Rev 18
1173/1749

--- Page 4 ---

**Ethernet (ETH): media access control (MAC) with DMA controller**
RM0090

**Bit 11 LCA: Loss of carrier**
When set, this bit indicates that a loss of carrier occurred during frame transmission (that is, the MII_CRS signal was inactive for one or more transmit clock periods during frame transmission). This is valid only for the frames transmitted without collision when the MAC operates in Half-duplex mode.

**Bit 10 NC: No carrier**
When set, this bit indicates that the Carrier Sense signal form the PHY was not asserted during transmission.

**Bit 9 LCO: Late collision**
When set, this bit indicates that frame transmission was aborted due to a collision occurring after the collision window (64 byte times, including preamble, in MII mode). This bit is not valid if the Underflow Error bit is set.

**Bit 8 EC: Excessive collision**
When set, this bit indicates that the transmission was aborted after 16 successive collisions while attempting to transmit the current frame. If the RD (Disable retry) bit in the MAC Configuration register is set, this bit is set after the first collision, and the transmission of the frame is aborted.

**Bit 7 VF: VLAN frame**
When set, this bit indicates that the transmitted frame was a VLAN-type frame.

**Bits 6:3 CC: Collision count**
This 4-bit counter value indicates the number of collisions occurring before the frame was transmitted. The count is not valid when the Excessive collisions bit (TDES0[8]) is set.

**Bit 2 ED: Excessive deferral**
When set, this bit indicates that the transmission has ended because of excessive deferral of over 24 288 bit times if the Deferral check (DC) bit in the MAC Control register is set high.

**Bit 1 UF: Underflow error**
When set, this bit indicates that the MAC aborted the frame because data arrived late from the RAM memory. Underflow error indicates that the DMA encountered an empty transmit buffer while transmitting the frame. The transmission process enters the Suspended state and sets both Transmit underflow (Register 5[5]) and Transmit interrupt (Register 5[0]).

**Bit 0 DB: Deferred bit**
When set, this bit indicates that the MAC defers before transmission because of the presence of the carrier. This bit is valid only in Half-duplex mode.

*   **TDES1: Transmit descriptor Word1**

| 31 30 29 | 28 27 26 25 24 23 22 21 20 19 18 17 16 | 15 14 13 | 12 11 10 9 8 7 6 5 4 3 2 1 0 |
|---|---|---|---|
| Reserved | TBS2 | Reserved | TBS1 |
|  | rw |  | rw |


31:29 Reserved, must be kept at reset value.

1174/1749
RM0090 Rev 18

--- Page 5 ---

RM0090
Ethernet (ETH): media access control (MAC) with DMA controller

28:16 **TBS2**: Transmit buffer 2 size
These bits indicate the second data buffer size in bytes. This field is not valid if TDES0[20] is set.

15:13 Reserved, must be kept at reset value.

12:0 **TBS1**: Transmit buffer 1 size
These bits indicate the first data buffer byte size, in bytes. If this field is 0, the DMA ignores this buffer and uses Buffer 2 or the next descriptor, depending on the value of TCH (TDES0[20]).

*   **TDES2: Transmit descriptor Word2**
    TDES2 contains the address pointer to the first buffer of the descriptor or it contains time stamp data.

| 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0 |
|---|
| TBAP1/TBAP/TTSL |
| rw |


Bits 31:0 **TBAP1**: Transmit buffer 1 address pointer / Transmit frame time stamp low
These bits have two different functions: they indicate to the DMA the location of data in memory, and after all data are transferred, the DMA can then use these bits to pass back time stamp data.

**TBAP**: When the software makes this descriptor available to the DMA (at the moment that the OWN bit is set to 1 in TDES0), these bits indicate the physical address of Buffer 1. There is no limitation on the buffer address alignment. See *Host data buffer alignment* for further details on buffer address alignment.

**TTSL**: Before it clears the OWN bit in TDES0, the DMA updates this field with the 32 least significant bits of the time stamp captured for the corresponding transmit frame (overwriting the value for TBAP1). This field has the time stamp only if time stamping is activated for this frame (see TTSE, TDES0 bit 25) and if the Last segment control bit (LS) in the descriptor is set.

*   **TDES3: Transmit descriptor Word3**
    TDES3 contains the address pointer either to the second buffer of the descriptor or the next descriptor, or it contains time stamp data.

| 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0 |
|---|
| TBAP2/TBAP2/TTSH |
| rw |


ST
RM0090 Rev 18
1175/1749

--- Page 6 ---

**Ethernet (ETH): media access control (MAC) with DMA controller** RM0090

Bits 31:0 **TBAP2**: Transmit buffer 2 address pointer (Next descriptor address) / Transmit frame time stamp high

These bits have two different functions: they indicate to the DMA the location of data in memory, and after all data are transferred, the DMA can then use these bits to pass back time stamp data.

**TBAP2**: When the software makes this descriptor available to the DMA (at the moment when the OWN bit is set to 1 in TDES0), these bits indicate the physical address of Buffer 2 when a descriptor ring structure is used. If the Second address chained (TDES1 [20]) bit is set, this address contains the pointer to the physical memory where the next descriptor is present. The buffer address pointer must be aligned to the bus width only when TDES1 [20] is set. (LSBs are ignored internally.)

**TTSH**: Before it clears the OWN bit in TDES0, the DMA updates this field with the 32 most significant bits of the time stamp captured for the corresponding transmit frame (overwriting the value for TBAP2). This field has the time stamp only if time stamping is activated for this frame (see TDES0 bit 25, TTSE) and if the Last segment control bit (LS) in the descriptor is set.

**Enhanced Tx DMA descriptors**

Enhanced descriptors (enabled with EDFE=1, ETHDMABMR bit 7), must be used if time stamping is activated (TSE=1, ETH_PTPTSCR bit 0) or if IPv4 checksum offload is activated (IPCO=1, ETH_MACCR bit 10).

Enhanced descriptors comprise eight 32-bit words, twice the size of normal descriptors. TDES0, TDES1, TDES2 and TDES3 have the same definitions as for normal transmit descriptors (refer to *Normal Tx DMA descriptors*). TDES6 and TDES7 hold the time stamp. TDES4, TDES5, TDES6 and TDES7 are defined below.

When the Enhanced descriptor mode is selected, the software needs to allocate 32-bytes (8 words) of memory for every descriptor. When time stamping or IPv4 checksum offload are not being used, the enhanced descriptor format may be disabled and the software can use normal descriptors with the default size of 16 bytes.

1176/1749 RM0090 Rev 18 STI

--- Page 7 ---

RM0090
Ethernet (ETH): media access control (MAC) with DMA controller

Figure 380. Enhanced transmit descriptor

| TDES 0 | OWN | Ctrl [30:26] | T T S E | Res. 24 | Ctrl [23:20] | Reserved [19:18] | T T S S | Status [16:0] |
|---|---|---|---|---|---|---|---|---|
| TDES 1 | Reserved [31:29] | Buffer 2 byte count [28:16] | Reserved [15:13] | Buffer 1 byte count [12:0] |  |  |  |  |
| TDES 2 | Buffer 1 address [31:0] |  |  |  |  |  |  |  |
| TDES 3 | Buffer 2 address [31:0] or Next descriptor address [31:0] |  |  |  |  |  |  |  |
| TDES 4 | Reserved |  |  |  |  |  |  |  |
| TDES 5 | Reserved |  |  |  |  |  |  |  |
| TDES 6 | Time stamp low [31:0] |  |  |  |  |  |  |  |
| TDES 7 | Time stamp high [31:0] |  |  |  |  |  |  |  |


ai17105b

*   **TDES4**: Transmit descriptor Word4
    Reserved
*   **TDES5**: Transmit descriptor Word5
    Reserved
*   **TDES6**: Transmit descriptor Word6

| 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0 |
|---|
| TTSL |
| rw |


Bits 31:0 **TTSL**: Transmit frame time stamp low
This field is updated by DMA with the 32 least significant bits of the time stamp captured for the corresponding transmit frame. This field has the time stamp only if the Last segment control bit (LS) in the descriptor is set.

STI
RM0090 Rev 18
1177/1749

--- Page 8 ---

Ethernet (ETH): media access control (MAC) with DMA controller RM0090

*   **TDES7:** Transmit descriptor Word7

| 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0 |
|---|
| TTSH |
| rw |


Bits 31:0 **TTSH**: Transmit frame time stamp high
This field is updated by DMA with the 32 most significant bits of the time stamp captured for the corresponding transmit frame. This field has the time stamp only if the Last segment control bit (LS) in the descriptor is set.


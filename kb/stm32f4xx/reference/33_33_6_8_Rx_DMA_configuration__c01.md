> [RM0090 ch.33 §33.6.8] Rx DMA configuration


### 33.6.8 Rx DMA configuration

The Receive DMA engine's reception sequence is illustrated in *Figure 381* and described below:

1. The CPU sets up Receive descriptors (RDES0-RDES3) and sets the OWN bit (RDES0[31]).
2. Once the SR (ETH_DMAOMR register[1]) bit is set, the DMA enters the Run state. While in the Run state, the DMA polls the receive descriptor list, attempting to acquire free descriptors. If the fetched descriptor is not free (is owned by the CPU), the DMA enters the Suspend state and jumps to Step 9.
3. The DMA decodes the receive data buffer address from the acquired descriptors.
4. Incoming frames are processed and placed in the acquired descriptor's data buffers.
5. When the buffer is full or the frame transfer is complete, the Receive engine fetches the next descriptor.
6. If the current frame transfer is complete, the DMA proceeds to step 7. If the DMA does not own the next fetched descriptor and the frame transfer is not complete (EOF is not yet transferred), the DMA sets the Descriptor error bit in RDES0 (unless flushing is disabled). The DMA closes the current descriptor (clears the OWN bit) and marks it as intermediate by clearing the Last segment (LS) bit in the RDES1 value (marks it as last descriptor if flushing is not disabled), then proceeds to step 8. If the DMA owns the next descriptor but the current frame transfer is not complete, the DMA closes the current descriptor as intermediate and returns to step 4.
7. If IEEE 1588 time stamping is enabled, the DMA writes the time stamp (if available) to the current descriptor's RDES2 and RDES3. It then takes the received frame's status and writes the status word to the current descriptor's RDES0, with the OWN bit cleared and the Last segment bit set.
8. The Receive engine checks the latest descriptor's OWN bit. If the CPU owns the descriptor (OWN bit is at 0) the Receive buffer unavailable bit (in ETH_DMASR register[7]) is set and the DMA Receive engine enters the Suspended state (step 9). If the DMA owns the descriptor, the engine returns to step 4 and awaits the next frame.
9. Before the Receive engine enters the Suspend state, partial frames are flushed from the Receive FIFO (you can control flushing using bit 24 in the ETH_DMAOMR register).
10. The Receive DMA exits the Suspend state when a Receive Poll demand is given or the start of next frame is available from the Receive FIFO. The engine proceeds to step 2 and re-fetches the next descriptor.

1178/1749 RM0090 Rev 18 STI

--- Page 9 ---

**RM0090**
**Ethernet (ETH): media access control (MAC) with DMA controller**

The DMA does not acknowledge accepting the status until it has completed the time stamp write-back and is ready to perform status write-back to the descriptor. If software has enabled time stamping through CSR, when a valid time stamp value is not available for the frame (for example, because the receive FIFO was full before the time stamp could be written to it), the DMA writes all ones to RDES2 and RDES3. Otherwise (that is, if time stamping is not enabled), RDES2 and RDES3 remain unchanged.

**Figure 381. Receive DMA operation**

Start RxDMA
Start
Stop RxDMA
Poll demand/ new frame available
(Re-)Fetch next descriptor
RxDMA suspended
(AHB) error?
Yes
No
Frame transfer complete?
No
Own bit set?
Yes
Flush disabled?
Yes
No
Frame data available?
No
Yes
Flush the remaining frame
Write data to buffer(s)
Wait for frame data
(AHB) error?
Yes
No
Fetch next descriptor
(AHB) error?
Yes
No
Flush disabled?
No
Own bit set for next desc?
No
Frame transfer complete?
No
Yes
Set descriptor error
Close RDES0 as intermediate descriptor
Time stamp present?
Yes
Write time stamp to RDES2 & RDES3
No
Close RDES0 as last descriptor
(AHB) error?
Yes
No
(AHB) error?
No
Yes
ai15643

ST
RM0090 Rev 18
1179/1749

--- Page 10 ---

**Ethernet (ETH): media access control (MAC) with DMA controller** RM0090

**Receive descriptor acquisition**

The receive engine always attempts to acquire an extra descriptor in anticipation of an incoming frame. Descriptor acquisition is attempted if any of the following conditions is/are satisfied:

*   The receive Start/Stop bit (ETH_DMAOMR register[1]) has been set immediately after the DMA has been placed in the Run state.
*   The data buffer of the current descriptor is full before the end of the frame currently being transferred
*   The controller has completed frame reception, but the current receive descriptor has not yet been closed.
*   The receive process has been suspended because of a CPU-owned buffer (RDES0[31] = 0) and a new frame is received.
*   A Receive poll demand has been issued.

**Receive frame processing**

The MAC transfers the received frames to the STM32F4xx memory only when the frame passes the address filter and the frame size is greater than or equal to the configurable threshold bytes set for the Receive FIFO, or when the complete frame is written to the FIFO in Store-and-forward mode. If the frame fails the address filtering, it is dropped in the MAC block itself (unless Receive All ETH_MACFFR [31] bit is set). Frames that are shorter than 64 bytes, because of collision or premature termination, can be purged from the Receive FIFO. After 64 (configurable threshold) bytes have been received, the DMA block begins transferring the frame data to the receive buffer pointed to by the current descriptor. The DMA sets the first descriptor (RDES0[9]) after the DMA AHB Interface becomes ready to receive a data transfer (if DMA is not fetching transmit data from the memory), to delimit the frame. The descriptors are released when the OWN (RDES0[31]) bit is reset to 0, either as the data buffer fills up or as the last segment of the frame is transferred to the receive buffer. If the frame is contained in a single descriptor, both the last descriptor (RDES0[8]) and first descriptor (RDES0[9]) bits are set. The DMA fetches the next descriptor, sets the last descriptor (RDES0[8]) bit, and releases the RDES0 status bits in the previous frame descriptor. Then the DMA sets the receive interrupt bit (ETH_DMASR register [6]). The same process repeats unless the DMA encounters a descriptor flagged as being owned by the CPU. If this occurs, the receive process sets the receive buffer unavailable bit (ETH_DMASR register[7]) and then enters the Suspend state. The position in the receive list is retained.

**Receive process suspended**

If a new receive frame arrives while the receive process is in Suspend state, the DMA re-fetches the current descriptor in the STM32F4xx memory. If the descriptor is now owned by the DMA, the receive process re-enters the Run state and starts frame reception. If the descriptor is still owned by the host, by default, the DMA discards the current frame at the top of the Rx FIFO and increments the missed frame counter. If more than one frame is stored in the Rx FIFO, the process repeats. The discarding or flushing of the frame at the top of the Rx FIFO can be avoided by setting the DMA Operation mode register bit 24 (DFRF). In such conditions, the receive process sets the receive buffer unavailable status bit and returns to the Suspend state.

1180/1749 RM0090 Rev 18 STI

--- Page 11 ---

RM0090
Ethernet (ETH): media access control (MAC) with DMA controller

**Normal Rx DMA descriptors**

The normal receive descriptor structure consists of four 32-bit words (16 bytes). These are shown in Figure 382. The bit descriptions of RDES0, RDES1, RDES2 and RDES3 are given below.

Note that enhanced descriptors must be used if time stamping is activated (TSE=1, ETH_PTPTSCR bit 0) or if IPv4 checksum offload is activated (IPCO=1, ETH_MACCR bit 10).

Figure 382. Normal Rx DMA descriptor structure

| 31 | 0 |
|---|---|
| RDES 0 | OWNStatus [30:0] |
| RDES 1 | CTRL RLReserved [30:29]Buffer 2 byte count [28:16]CTRL [15:14]Res.Buffer 1 byte count [12:0] |
| RDES 2 | Buffer 1 address [31:0] |
| RDES 3 | Buffer 2 address [31:0] or Next descriptor address [31:0] |


ai15644

*   **RDES0: Receive descriptor Word0**
    RDES0 contains the received frame status, the frame length and the descriptor ownership information.

| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| OWN | AFM | FL | ES | DE | SAF | LE | OE | VLAN | FS | LS | IPHCE/TSV | LCO | FT | RWT | RE | DE | CE | PCE/ESA |  |  |  |  |  |  |  |  |  |  |  |  |  |
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bit 31 **OWN**: Own bit
When set, this bit indicates that the descriptor is owned by the DMA of the MAC Subsystem. When this bit is reset, it indicates that the descriptor is owned by the Host. The DMA clears this bit either when it completes the frame reception or when the buffers that are associated with this descriptor are full.

Bit 30 **AFM**: Destination address filter fail
When set, this bit indicates a frame that failed the DA filter in the MAC Core.

Bits 29:16 **FL**: Frame length
These bits indicate the byte length of the received frame that was transferred to host memory (including CRC). This field is valid only when last descriptor (RDES0[8]) is set and descriptor error (RDES0[14]) is reset.
This field is valid when last descriptor (RDES0[8]) is set. When the last descriptor and error summary bits are not set, this field indicates the accumulated number of bytes that have been transferred for the current frame.

ST
RM0090 Rev 18
1181/1749

--- Page 12 ---

Ethernet (ETH): media access control (MAC) with DMA controller
RM0090

Bit 15 **ES**: Error summary
Indicates the logical OR of the following bits:
RDES0[1]: CRC error
RDES0[3]: Receive error
RDES0[4]: Watchdog timeout
RDES0[6]: Late collision
RDES0[7]: Giant frame (This is not applicable when RDES0[7] indicates an IPV4 header checksum error.)
RDES0[11]: Overflow error
RDES0[14]: Descriptor error.
This field is valid only when the last descriptor (RDES0[8]) is set.

Bit 14 **DE**: Descriptor error
When set, this bit indicates a frame truncation caused by a frame that does not fit within the current descriptor buffers, and that the DMA does not own the next descriptor. The frame is truncated.
This field is valid only when the last descriptor (RDES0[8]) is set.

Bit 13 **SAF**: Source address filter fail
When set, this bit indicates that the SA field of frame failed the SA filter in the MAC Core.

Bit 12 **LE**: Length error
When set, this bit indicates that the actual length of the received frame does not match the value in the Length/ Type field. This bit is valid only when the Frame type (RDES0[5]) bit is reset.

Bit 11 **OE**: Overflow error
When set, this bit indicates that the received frame was damaged due to buffer overflow.

Bit 10 **VLAN**: VLAN tag
When set, this bit indicates that the frame pointed to by this descriptor is a VLAN frame tagged by the MAC core.

Bit 9 **FS**: First descriptor
When set, this bit indicates that this descriptor contains the first buffer of the frame. If the size of the first buffer is 0, the second buffer contains the beginning of the frame. If the size of the second buffer is also 0, the next descriptor contains the beginning of the frame.

Bit 8 **LS**: Last descriptor
When set, this bit indicates that the buffers pointed to by this descriptor are the last buffers of the frame.

Bit 7 **IPHCE/TSV**: IPv header checksum error / time stamp valid
If IPHCE is set, it indicates an error in the IPv4 or IPv6 header. This error can be due to inconsistent Ethernet Type field and IP header Version field values, a header checksum mismatch in IPv4, or an Ethernet frame lacking the expected number of IP header bytes. This bit can take on special meaning as specified in Table 193.
If enhanced descriptor format is enabled (EDFE=1, bit 7 of ETH_DMABMR), this bit takes on the TSV function (otherwise it is IPHCE). When TSV is set, it indicates that a snapshot of the timestamp is written in descriptor words 6 (RDES6) and 7 (RDES7). TSV is valid only when the Last descriptor bit (RDES0[8]) is set.

Bit 6 **LCO**: Late collision
When set, this bit indicates that a late collision has occurred while receiving the frame in Half-duplex mode.

1182/1749
RM0090 Rev 18

--- Page 13 ---

RM0090
Ethernet (ETH): media access control (MAC) with DMA controller

**Bit 5 FT:** Frame type
When set, this bit indicates that the Receive frame is an Ethernet-type frame (the LT field is greater than or equal to 0x0600). When this bit is reset, it indicates that the received frame is an IEEE802.3 frame. This bit is not valid for Runt frames less than 14 bytes. When the normal descriptor format is used (ETH_DMABMR EDFE=0), FT can take on special meaning as specified in Table 193.

**Bit 4 RWT:** Receive watchdog timeout
When set, this bit indicates that the Receive watchdog timer has expired while receiving the current frame and the current frame is truncated after the watchdog timeout.

**Bit 3 RE:** Receive error
When set, this bit indicates that the RX_ERR signal is asserted while RX_DV is asserted during frame reception.

**Bit 2 DE:** Dribble bit error
When set, this bit indicates that the received frame has a non-integer multiple of bytes (odd nibbles). This bit is valid only in MII mode.

**Bit 1 CE:** CRC error
When set, this bit indicates that a cyclic redundancy check (CRC) error occurred on the received frame. This field is valid only when the last descriptor (RDES0[8]) is set.

**Bit 0 PCE/ESA:** Payload checksum error / extended status available
When set, it indicates that the TCP, UDP or ICMP checksum the core calculated does not match the received encapsulated TCP, UDP or ICMP segment's Checksum field. This bit is also set when the received number of payload bytes does not match the value indicated in the Length field of the encapsulated IPv4 or IPv6 datagram in the received Ethernet frame. This bit can take on special meaning as specified in Table 193.
If the enhanced descriptor format is enabled (EDFE=1, bit 7 in ETH_DMABMR), this bit takes on the ESA function (otherwise it is PCE). When ESA is set, it indicates that the extended status is available in descriptor word 4 (RDES4). ESA is valid only when the last descriptor bit (RDES0[8]) is set.

Bits 5, 7, and 0 reflect the conditions discussed in Table 193.

ST
RM0090 Rev 18
1183/1749

--- Page 14 ---

Ethernet (ETH): media access control (MAC) with DMA controller
RM0090

Table 193. Receive descriptor 0 - encoding for bits 7, 5 and 0 (normal descriptor format only, EDFE=0)

| Bit 5: frame type | Bit 7: IPC checksum error | Bit 0: payload checksum error | Frame status |
|---|---|---|---|
| 0 | IEEE 802.3 Type frame (Length field value is less than 0x0600.) |  |  |
| 1 | 0 | IPv4/IPv6 Type frame, no checksum error detected |  |
| 1 | 0 | 1 | IPv4/IPv6 Type frame with a payload checksum error (as described for PCE) detected |
| 1 | 0 | IPv4/IPv6 Type frame with an IP header checksum error (as described for IPC CE) detected |  |
| 1 | IPv4/IPv6 Type frame with both IP header and payload checksum errors detected |  |  |
| 0 | 1 | IPv4/IPv6 Type frame with no IP header checksum error and the payload check bypassed, due to an unsupported payload |  |
| 0 | 1 | A Type frame that is neither IPv4 or IPv6 (the checksum offload engine bypasses checksum completely.) |  |
| 0 | 1 | 0 | Reserved |


*   **RDES1: Receive descriptor Word1**

| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| CD | RBS2 | RER | RCH | Reserved | RBS |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


**Bit 31 DIC**: Disable interrupt on completion
When set, this bit prevents setting the Status register's RS bit (CSR5[6]) for the received frame ending in the buffer indicated by this descriptor. This, in turn, disables the assertion of the interrupt to Host due to RS for that frame.

Bits 30:29 Reserved, must be kept at reset value.

Bits 28:16 **RBS2**: Receive buffer 2 size
These bits indicate the second data buffer size, in bytes. The buffer size must be a multiple of 4, 8, or 16, depending on the bus widths (32, 64 or 128, respectively), even if the value of RDES3 (buffer2 address pointer) is not aligned to bus width. If the buffer size is not an appropriate multiple of 4, 8 or 16, the resulting behavior is undefined. This field is not valid if RDES1 [14] is set.

Bit 15 **RER**: Receive end of ring
When set, this bit indicates that the descriptor list reached its final descriptor. The DMA returns to the base address of the list, creating a descriptor ring.

1184/1749
RM0090 Rev 18

--- Page 15 ---

RM0090
Ethernet (ETH): media access control (MAC) with DMA controller

Bit 14 **RCH**: Second address chained
When set, this bit indicates that the second address in the descriptor is the next descriptor address rather than the second buffer address. When this bit is set, RBS2 (RDES1[28:16]) is a “don’t care” value. RDES1[15] takes precedence over RDES1[14].

Bit 13 Reserved, must be kept at reset value.

Bits 12:0 **RBS1**: Receive buffer 1 size
Indicates the first data buffer size in bytes. The buffer size must be a multiple of 4, 8 or 16, depending upon the bus widths (32, 64 or 128), even if the value of RDES2 (buffer1 address pointer) is not aligned. When the buffer size is not a multiple of 4, 8 or 16, the resulting behavior is undefined. If this field is 0, the DMA ignores this buffer and uses Buffer 2 or next descriptor depending on the value of RCH (bit 14).

*   **RDES2**: Receive descriptor Word2
    RDES2 contains the address pointer to the first data buffer in the descriptor, or it contains time stamp data.

| RBP1 / RTSL |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
| r/w |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bits 31:0 **RBAP1 / RTSL**: Receive buffer 1 address pointer / Receive frame time stamp low
These bits take on two different functions: the application uses them to indicate to the DMA where to store the data in memory, and then after transferring all the data the DMA may use these bits to pass back time stamp data.

**RBAP1**: When the software makes this descriptor available to the DMA (at the moment that the OWN bit is set to 1 in RDES0), these bits indicate the physical address of Buffer 1. There are no limitations on the buffer address alignment except for the following condition: the DMA uses the configured value for its address generation when the RDES2 value is used to store the start of frame. Note that the DMA performs a write operation with the RDES2[3/2/1/0] bits as 0 during the transfer of the start of frame but the frame data is shifted as per the actual Buffer address pointer. The DMA ignores RDES2[3/2/1/0] (corresponding to bus width of 128/64/32) if the address pointer is to a buffer where the middle or last part of the frame is stored.

**RTSL**: Before it clears the OWN bit in RDES0, the DMA updates this field with the 32 least significant bits of the time stamp captured for the corresponding receive frame (overwriting the value for RBAP1). This field has the time stamp only if time stamping is activated for this frame and if the Last segment control bit (LS) in the descriptor is set.

STI
RM0090 Rev 18
1185/1749

--- Page 16 ---

Ethernet (ETH): media access control (MAC) with DMA controller
RM0090

*   **RDES3**: Receive descriptor Word3
    RDES3 contains the address pointer either to the second data buffer in the descriptor or to the next descriptor, or it contains time stamp data.

| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| RBP2 / RTSH |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| r/w |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bits 31:0 **RBAP2 / RTSH**: Receive buffer 2 address pointer (next descriptor address) / Receive frame time stamp high

These bits take on two different functions: the application uses them to indicate to the DMA the location of where to store the data in memory, and then after transferring all the data the DMA may use these bits to pass back time stamp data.

**RBAP1**: When the software makes this descriptor available to the DMA (at the moment that the OWN bit is set to 1 in RDES0), these bits indicate the physical address of buffer 2 when a descriptor ring structure is used. If the second address chained (RDES1 [24]) bit is set, this address contains the pointer to the physical memory where the next descriptor is present. If RDES1 [24] is set, the buffer (next descriptor) address pointer must be bus width-aligned (RDES3[3, 2, or 1:0] = 0, corresponding to a bus width of 128, 64 or 32. LSBs are ignored internally.) However, when RDES1 [24] is reset, there are no limitations on the RDES3 value, except for the following condition: the DMA uses the configured value for its buffer address generation when the RDES3 value is used to store the start of frame. The DMA ignores RDES3[3, 2, or 1:0] (corresponding to a bus width of 128, 64 or 32) if the address pointer is to a buffer where the middle or last part of the frame is stored.

**RTSH**: Before it clears the OWN bit in RDES0, the DMA updates this field with the 32 most significant bits of the time stamp captured for the corresponding receive frame (overwriting the value for RBAP2). This field has the time stamp only if time stamping is activated and if the Last segment control bit (LS) in the descriptor is set.

**Enhanced Rx DMA descriptors format with IEEE1588 time stamp**

Enhanced descriptors (enabled with EDFE=1, ETHDMABMR bit 7), must be used if time stamping is activated (TSE=1, ETH_PTPSCR bit 0) or if IPv4 checksum offload is activated (IPCO=1, ETH_MACCR bit 10).

Enhanced descriptors comprise eight 32-bit words, twice the size of normal descriptors. RDES0, RDES1, RDES2 and RDES3 have the same definitions as for normal receive descriptors (refer to *Normal Rx DMA descriptors*). RDES4 contains extended status while RDES6 and RDES7 hold the time stamp. RDES4, RDES5, RDES6 and RDES7 are defined below.

When the Enhanced descriptor mode is selected, the software needs to allocate 32 bytes (8 words) of memory for every descriptor. When time stamping or IPv4 checksum offload are not being used, the enhanced descriptor format may be disabled and the software can use normal descriptors with the default size of 16 bytes.

1186/1749
RM0090 Rev 18
ST

--- Page 17 ---

RM0090
Ethernet (ETH): media access control (MAC) with DMA controller

Figure 383. Enhanced receive descriptor field format with IEEE1588 time stamp enabled

| RDES 0 | 31 | 0 |  |  |  |  |
|---|---|---|---|---|---|---|
| O | W | N | Status [30:0] |  |  |  |
| RDES 1 | CT RL | Reserved [30:29] | Buffer 2 byte count [28:16] | CTRL [15:14] | Res. | Buffer 1 byte count [12:0] |
| RDES 2 | Buffer 1 address [31:0] |  |  |  |  |  |
| RDES 3 | Buffer 2 address [31:0] or Next descriptor address [31:0] |  |  |  |  |  |
| RDES 4 | Extended Status [31:0] |  |  |  |  |  |
| RDES 5 | Reserved |  |  |  |  |  |
| RDES 6 | Time stamp low [31:0] |  |  |  |  |  |
| RDES 7 | Time stamp high [31:0] |  |  |  |  |  |


ai17104

*   RDES4: Receive descriptor Word4

The extended status, shown below, is valid only when there is status related to IPv4 checksum or time stamp available as indicated by bit 0 in RDES0.

| 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 15 14 | 13 12 11 10 9 8 7 6 5 4 3 2 1 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved | PV | PFT | PMT | IPv6PR | IPv4PR | IPCB | IPPE | IPHE | IPPT |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | rw |


Bits 31:14 Reserved, must be kept at reset value.

Bit 13 **PV**: PTP version
When set, indicates that the received PTP message uses the IEEE 1588 version 2 format. When cleared, it uses version 1 format. This is valid only if the message type is non-zero.

Bit 12 **PFT**: PTP frame type
When set, this bit indicates that the PTP message is sent directly over Ethernet. When this bit is cleared and the message type is non-zero, it indicates that the PTP message is sent over UDP-IPv4 or UDP-IPv6. The information on IPv4 or IPv6 can be obtained from bits 6 and 7.

STI
RM0090 Rev 18
1187/1749

--- Page 18 ---

**Ethernet (ETH): media access control (MAC) with DMA controller** RM0090

**Bits 11:8 PMT: PTP message type**
These bits are encoded to give the type of the message received.
- 0000: No PTP message received
- 0001: SYNC (all clock types)
- 0010: Follow_Up (all clock types)
- 0011: Delay_Req (all clock types)
- 0100: Delay_Resp (all clock types)
- 0101: Pdelay_Req (in peer-to-peer transparent clock) or Announce (in ordinary or boundary clock)
- 0110: Pdelay_Resp (in peer-to-peer transparent clock) or Management (in ordinary or boundary clock)
- 0111: Pdelay_Resp_Follow_Up (in peer-to-peer transparent clock) or Signaling (for ordinary or boundary clock)
- 1xxx - Reserved

**Bit 7 IPV6PR: IPv6 packet received**
When set, this bit indicates that the received packet is an IPv6 packet.

**Bit 6 IPV4PR: IPv4 packet received**
When set, this bit indicates that the received packet is an IPv4 packet.

**Bit 5 IPCB: IP checksum bypassed**
When set, this bit indicates that the checksum offload engine is bypassed.

**Bit 4 IPPE: IP payload error**
When set, this bit indicates that the 16-bit IP payload checksum (that is, the TCP, UDP, or ICMP checksum) that the core calculated does not match the corresponding checksum field in the received segment. It is also set when the TCP, UDP, or ICMP segment length does not match the payload length value in the IP Header field.

**Bit 3 IPHE: IP header error**
When set, this bit indicates either that the 16-bit IPv4 header checksum calculated by the core does not match the received checksum bytes, or that the IP datagram version is not consistent with the Ethernet Type value.

**Bits 2:0 IPPT: IP payload type**
If IPv4 checksum offload is activated (IPCO=1, ETH_MACCR bit 10), these bits indicate the type of payload encapsulated in the IP datagram. These bits are '00' if there is an IP header error or fragmented IP.
- 000: Unknown or did not process IP payload
- 001: UDP
- 010: TCP
- 011: ICMP
- 1xx: Reserved

*   **RDES5: Receive descriptor Word5**
    Reserved.
*   **RDES6: Receive descriptor Word6**
    The table below describes the fields that have different meaning for RDES6 when the receive descriptor is closed and time stamping is enabled.

1188/1749 RM0090 Rev 18 STI

--- Page 19 ---

RM0090
Ethernet (ETH): media access control (MAC) with DMA controller

| RTSL |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bits 31:0 **RTSL**: Receive frame time stamp low

The DMA updates this field with the 32 least significant bits of the time stamp captured for the corresponding receive frame. The DMA updates this field only for the last descriptor of the receive frame indicated by last descriptor status bit (RDES0[8]). When this field and the RTSH field in RDES7 show all ones, the time stamp must be treated as corrupt.

*   **RDES7**: Receive descriptor Word7

The table below describes the fields that have a different meaning for RDES7 when the receive descriptor is closed and time stamping is enabled.

| RTSH |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bits 31:0 **RTSH**: Receive frame time stamp high

The DMA updates this field with the 32 most significant bits of the time stamp captured for the corresponding receive frame. The DMA updates this field only for the last descriptor of the receive frame indicated by last descriptor status bit (RDES0[8]).
When this field and RDES7’s RTSL field show all ones, the time stamp must be treated as corrupt.

> [RM0090 ch.33 §33.6.8] Rx DMA configuration

### 33.6.9 DMA interrupts

### 33.6.9 DMA interrupts

Interrupts can be generated as a result of various events. The ETH_DMASR register contains all the bits that might cause an interrupt. The ETH_DMAIER register contains an enable bit for each of the events that can cause an interrupt.

There are two groups of interrupts, Normal and Abnormal, as described in the ETH_DMASR register. Interrupts are cleared by writing a 1 to the corresponding bit position. When all the enabled interrupts within a group are cleared, the corresponding summary bit is cleared. If the MAC core is the cause for assertion of the interrupt, then any of the TSTS or PMTS bits in the ETH_DMASR register is set high.

Interrupts are not queued and if the interrupt event occurs before the driver has responded to it, no additional interrupts are generated. For example, the Receive Interrupt bit (ETH_DMASR register [6]) indicates that one or more frames were transferred to the STM32F4xx buffer. The driver must scan all descriptors, from the last recorded position to the first one owned by the DMA.

An interrupt is generated only once for simultaneous, multiple events. The driver must scan the ETH_DMASR register for the cause of the interrupt. The interrupt is not generated again unless a new interrupting event occurs, after the driver has cleared the appropriate bit in the ETH_DMASR register. For example, the controller generates a Receive interrupt (ETH_DMASR register[6]) and the driver begins reading the ETH_DMASR register. Next, receive buffer unavailable (ETH_DMASR register[7]) occurs. The driver clears the Receive interrupt. Even then, a new interrupt is generated, due to the active or pending Receive buffer unavailable interrupt.

ST
RM0090 Rev 18
1189/1749

--- Page 20 ---

Ethernet (ETH): media access control (MAC) with DMA controller
RM0090

Figure 384. Interrupt scheme

AI15646


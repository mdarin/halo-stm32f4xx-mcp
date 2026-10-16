### 33.5.2 MAC frame transmission

The DMA controls all transactions for the transmit path. Ethernet frames read from the system memory are pushed into the FIFO by the DMA. The frames are then popped out and transferred to the MAC core. When the end-of-frame is transferred, the status of the transmission is taken from the MAC core and transferred back to the DMA. The Transmit FIFO has a depth of 2 Kbyte. FIFO-fill level is indicated to the DMA so that it can initiate a data fetch in required bursts from the system memory, using the AHB interface. The data from the AHB Master interface is pushed into the FIFO.

When the SOF is detected, the MAC accepts the data and begins transmitting to the MII. The time required to transmit the frame data to the MII after the application initiates transmission is variable, depending on delay factors like IFG delay, time to transmit preamble/SFD, and any back-off delays for Half-duplex mode. After the EOF is transferred to the MAC core, the core completes normal transmission and then gives the status of transmission back to the DMA. If a normal collision (in Half-duplex mode) occurs during transmission, the MAC core makes the transmit status valid, then accepts and drops all further data until the next SOF is received. The same frame should be retransmitted from SOF on observing a Retry request (in the Status) from the MAC. The MAC issues an underflow status if the data are not provided continuously during the transmission. During the normal transfer of a frame, if the MAC receives an SOF without getting an EOF for the previous frame, then the SOF is ignored and the new frame is considered as the continuation of the previous frame.

ST
RM0090 Rev 18
1137/1749

--- Page 163 ---

**Ethernet (ETH): media access control (MAC) with DMA controller**

RM0090

There are two modes of operation for popping data towards the MAC core:

*   In Threshold mode, as soon as the number of bytes in the FIFO crosses the configured threshold level (or when the end-of-frame is written before the threshold is crossed), the data is ready to be popped out and forwarded to the MAC core. The threshold level is configured using the TTC bits of ETH_DMABMR.
*   In Store-and-forward mode, only after a complete frame is stored in the FIFO, the frame is popped towards the MAC core. If the Tx FIFO size is smaller than the Ethernet frame to be transmitted, then the frame is popped towards the MAC core when the Tx FIFO becomes almost full.

The application can flush the Transmit FIFO of all contents by setting the FTF (ETH_DMAOMR register [20]) bit. This bit is self-clearing and initializes the FIFO pointers to the default state. If the FTF bit is set during a frame transfer to the MAC core, then transfer is stopped as the FIFO is considered to be empty. Hence an underflow event occurs at the MAC transmitter and the corresponding Status word is forwarded to the DMA.

### Automatic CRC and pad generation

When the number of bytes received from the application falls below 60 (DA+SA+LT+Data), zeros are appended to the transmitting frame to make the data length exactly 46 bytes to meet the minimum data field requirement of IEEE 802.3. The MAC can be programmed not to append any padding. The cyclic redundancy check (CRC) for the frame check sequence (FCS) field is calculated and appended to the data being transmitted. When the MAC is programmed to not append the CRC value to the end of Ethernet frames, the computed CRC is not transmitted. An exception to this rule is that when the MAC is programmed to append pads for frames (DA+SA+LT+Data) less than 60 bytes, CRC will be appended at the end of the padded frames.

The CRC generator calculates the 32-bit CRC for the FCS field of the Ethernet frame. The encoding is defined by the following polynomial.

G(x) = x^32 + x^26 + x^23 + x^22 + x^16 + x^12 + x^11 + x^10 + x^8 + x^7 + x^5 + x^4 + x^2 + x + 1

### Transmit protocol

The MAC controls the operation of Ethernet frame transmission. It performs the following functions to meet the IEEE 802.3/802.3z specifications. It:

*   generates the preamble and SFD
*   generates the jam pattern in Half-duplex mode
*   controls the Jabber timeout
*   controls the flow for Half-duplex mode (back pressure)
*   generates the transmit frame status
*   contains time stamp snapshot logic in accordance with IEEE 1588

When a new frame transmission is requested, the MAC sends out the preamble and SFD, followed by the data. The preamble is defined as 7 bytes of 0b10101010 pattern, and the SFD is defined as 1 byte of 0b10101011 pattern. The collision window is defined as 1 slot time (512 bit times for 10/100 Mbit/s Ethernet). The jam pattern generation is applicable only to Half-duplex mode, not to Full-duplex mode.

In MII mode, if a collision occurs at any time from the beginning of the frame to the end of the CRC field, the MAC sends a 32-bit jam pattern of 0x5555 5555 on the MII to inform all

1138/1749 RM0090 Rev 18 ST

--- Page 164 ---

RM0090
Ethernet (ETH): media access control (MAC) with DMA controller

other stations that a collision has occurred. If the collision is seen during the preamble transmission phase, the MAC completes the transmission of the preamble and SFD and then sends the jam pattern.

A jabber timer is maintained to cut off the transmission of Ethernet frames if more than 2048 (default) bytes have to be transferred. The MAC uses the deferral mechanism for flow control (back pressure) in Half-duplex mode. When the application requests to stop receiving frames, the MAC sends a JAM pattern of 32 bytes whenever it senses the reception of a frame, provided that transmit flow control is enabled. This results in a collision and the remote station backs off. The application requests flow control by setting the BPA bit (bit 0) in the ETH_MACFCR register. If the application requests a frame to be transmitted, then it is scheduled and transmitted even when back pressure is activated. Note that if back pressure is kept activated for a long time (and more than 16 consecutive collision events occur) then the remote stations abort their transmissions due to excessive collisions. If IEEE 1588 time stamping is enabled for the transmit frame, this block takes a snapshot of the system time when the SFD is put onto the transmit MII bus.

**Transmit scheduler**

The MAC is responsible for scheduling the frame transmission on the MII. It maintains the interframe gap between two transmitted frames and follows the truncated binary exponential backoff algorithm for Half-duplex mode. The MAC enables transmission after satisfying the IFG and backoff delays. It maintains an idle period of the configured interframe gap (IFG bits in the ETH_MACCR register) between any two transmitted frames. If frames to be transmitted arrive sooner than the configured IFG time, the MII waits for the enable signal from the MAC before starting the transmission on it. The MAC starts its IFG counter as soon as the carrier signal of the MII goes inactive. At the end of the programmed IFG value, the MAC enables transmission in Full-duplex mode. In Half-duplex mode and when IFG is configured for 96 bit times, the MAC follows the rule of deference specified in Section 4.2.3.2.1 of the IEEE 802.3 specification. The MAC resets its IFG counter if a carrier is detected during the first two-thirds (64-bit times for all IFG values) of the IFG interval. If the carrier is detected during the final one third of the IFG interval, the MAC continues the IFG count and enables the transmitter after the IFG interval. The MAC implements the truncated binary exponential backoff algorithm when it operates in Half-duplex mode.

**Transmit flow control**

When the Transmit Flow Control Enable bit (TFE bit in ETH_MACFCR) is set, the MAC generates Pause frames and transmits them as necessary, in Full-duplex mode. The Pause frame is appended with the calculated CRC, and is sent. Pause frame generation can be initiated in two ways.

A pause frame is sent either when the application sets the FCB bit in the ETH_MACFCR register or when the receive FIFO is full (packet buffer).

*   If the application has requested flow control by setting the FCB bit in ETH_MACFCR, the MAC generates and transmits a single Pause frame. The value of the pause time in the generated frame contains the programmed pause time value in ETH_MACFCR. To extend the pause or end the pause prior to the time specified in the previously transmitted Pause frame, the application must request another Pause frame transmission after programming the Pause Time value (PT in ETH_MACFCR register) with the appropriate value.
*   If the application has requested flow control when the receive FIFO is full, the MAC generates and transmits a Pause frame. The value of the pause time in the generated frame is the programmed pause time value in ETH_MACFCR. If the receive FIFO

ST
RM0090 Rev 18
1139/1749

--- Page 165 ---

Ethernet (ETH): media access control (MAC) with DMA controller
RM0090

remains full at a configurable number of slot-times (PLT bits in ETH_MACFCR) before
this Pause time runs out, a second Pause frame is transmitted. The process is
repeated as long as the receive FIFO remains full. If this condition is no more satisfied
prior to the sampling time, the MAC transmits a Pause frame with zero pause time to
indicate to the remote end that the receive buffer is ready to receive new data frames.

**Single-packet transmit operation**

The general sequence of events for a transmit operation is as follows:

1. If the system has data to be transferred, the DMA controller fetches them from the
memory through the AHB Master interface and starts forwarding them to the FIFO. It
continues to receive the data until the end of frame is transferred.
2. When the threshold level is crossed or a full packet of data is received into the FIFO,
the frame data are popped and driven to the MAC core. The DMA continues to transfer
data from the FIFO until a complete packet has been transferred to the MAC. Upon
completion of the frame, the DMA controller is notified by the status coming from the
MAC.

**Transmit operation—Two packets in the buffer**

1. Because the DMA must update the descriptor status before releasing it to the Host,
there can be at the most two frames inside a transmit FIFO. The second frame is
fetched by the DMA and put into the FIFO only if the OSF (operate on second frame)
bit is set. If this bit is not set, the next frame is fetched from the memory only after the
MAC has completely processed the frame and the DMA has released the descriptors.
2. If the OSF bit is set, the DMA starts fetching the second frame immediately after
completing the transfer of the first frame to the FIFO. It does not wait for the status
to be updated. In the meantime, the second frame is received into the FIFO while the first
frame is being transmitted. As soon as the first frame has been transferred and the
status is received from the MAC, it is pushed to the DMA. If the DMA has already
completed sending the second packet to the FIFO, the second transmission must wait
for the status of the first packet before proceeding to the next frame.

**Retransmission during collision**

While a frame is being transferred to the MAC, a collision event may occur on the MAC line
interface in Half-duplex mode. The MAC would then indicate a retry attempt by giving the
status even before the end of frame is received. Then the retransmission is enabled and the
frame is popped out again from the FIFO. After more than 96 bytes have been popped
towards the MAC core, the FIFO controller frees up that space and makes it available to the
DMA to push in more data. This means that the retransmission is not possible after this
threshold is crossed or when the MAC core indicates a late collision event.

**Transmit FIFO flush operation**

The MAC provides a control to the software to flush the Transmit FIFO through the use of Bit
20 in the Operation mode register. The Flush operation is immediate and the Tx FIFO and
the corresponding pointers are cleared to the initial state even if the Tx FIFO is in the middle
of transferring a frame to the MAC Core. This results in an underflow event in the MAC
transmitter, and the frame transmission is aborted. The status of such a frame is marked
with both underflow and frame flush events (TDES0 bits 13 and 1). No data are coming to
the FIFO from the application (DMA) during the Flush operation. Transfer transmit status
words are transferred to the application for the number of frames that is flushed (including
partial frames). Frames that are completely flushed have the Frame flush status bit (TDES0
13) set. The Flush operation is completed when the application (DMA) has accepted all of

1140/1749
RM0090 Rev 18
ST

--- Page 166 ---

RM0090
Ethernet (ETH): media access control (MAC) with DMA controller

the Status words for the frames that were flushed. The Transmit FIFO Flush control register bit is then cleared. At this point, new frames from the application (DMA) are accepted. All data presented for transmission after a Flush operation are discarded unless they start with an SOF marker.

**Transmit status word**

At the end of the Ethernet frame transfer to the MAC core and after the core has completed the transmission of the frame, the transmit status is given to the application. The detailed description of the Transmit Status is the same as for bits [23:0] in TDES0. If IEEE 1588 time stamping is enabled, a specific frames' 64-bit time stamp is returned, along with the transmit status.

**Transmit checksum offload**

Communication protocols such as TCP and UDP implement checksum fields, which helps determine the integrity of data transmitted over a network. Because the most widespread use of Ethernet is to encapsulate TCP and UDP over IP datagrams, the Ethernet controller has a transmit checksum offload feature that supports checksum calculation and insertion in the transmit path, and error detection in the receive path. This section explains the operation of the checksum offload feature for transmitted frames.

Note:
The checksum for TCP, UDP or ICMP is calculated over a complete frame, then inserted into its corresponding header field. Due to this requirement, this function is enabled only when the Transmit FIFO is configured for Store-and-forward mode (that is, when the TSF bit is set in the ETH_ETH_DMAOMR register). If the core is configured for Threshold (cut-through) mode, the Transmit checksum offload is bypassed.

You must make sure the Transmit FIFO is deep enough to store a complete frame before that frame is transferred to the MAC Core transmitter. If the FIFO depth is less than the input Ethernet frame size, the payload (TCP/UDP/ICMP) checksum insertion function is bypassed and only the frame's IPv4 Header checksum is modified, even in Store-and-forward mode.

The transmit checksum offload supports two types of checksum calculation and insertion. This checksum can be controlled for each frame by setting the CIC bits (Bits 28:27 in TDES1, described in TDES1: Transmit descriptor Word1).

See IETF specifications RFC 791, RFC 793, RFC 768, RFC 792, RFC 2460 and RFC 4443 for IPv4, TCP, UDP, ICMP, IPv6 and ICMPv6 packet header specifications, respectively.

* IP header checksum

In IPv4 datagrams, the integrity of the header fields is indicated by the 16-bit header checksum field (the eleventh and twelfth bytes of the IPv4 datagram). The checksum offload detects an IPv4 datagram when the Ethernet frame's Type field has the value 0x0800 and the IP datagram's Version field has the value 0x4. The input frame's checksum field is ignored during calculation and replaced by the calculated value. IPv6 headers do not have a checksum field; thus, the checksum offload does not modify IPv6 header fields. The result of this IP header checksum calculation is indicated by the IP Header Error status bit in the Transmit status (Bit 16). This status bit is set whenever the values of the Ethernet Type field and the IP header's Version field are not consistent, or when the Ethernet frame does not have enough data, as indicated by the

ST
RM0090 Rev 18
1141/1749

--- Page 167 ---

**Ethernet (ETH): media access control (MAC) with DMA controller** RM0090

IP header Length field. In other words, this bit is set when an IP header error is asserted under the following circumstances:

a) For IPv4 datagrams:
- The received Ethernet type is 0x0800, but the IP header's Version field does not equal 0x4
- The IPv4 Header Length field indicates a value less than 0x5 (20 bytes)
- The total frame length is less than the value given in the IPv4 Header Length field

b) For IPv6 datagrams:
- The Ethernet type is 0x86DD but the IP header Version field does not equal 0x6
- The frame ends before the IPv6 header (40 bytes) or extension header (as given in the corresponding Header Length field in an extension header) has been completely received. Even when the checksum offload detects such an IP header error, it inserts an IPv4 header checksum if the Ethernet Type field indicates an IPv4 payload.

*   **TCP/UDP/ICMP checksum**

The TCP/UDP/ICMP checksum processes the IPv4 or IPv6 header (including extension headers) and determines whether the encapsulated payload is TCP, UDP or ICMP.

Note that:

a) For non-TCP, -UDP, or -ICMP/ICMPv6 payloads, this checksum is bypassed and nothing further is modified in the frame.

b) Fragmented IP frames (IPv4 or IPv6), IP frames with security features (such as an authentication header or encapsulated security payload), and IPv6 frames with routing headers are bypassed and not processed by the checksum.

The checksum is calculated for the TCP, UDP, or ICMP payload and inserted into its corresponding field in the header. It can work in the following two modes:

- In the first mode, the TCP, UDP, or ICMPv6 pseudo-header is not included in the checksum calculation and is assumed to be present in the input frame's checksum field. The checksum field is included in the checksum calculation, and then replaced by the final calculated checksum.
- In the second mode, the checksum field is ignored, the TCP, UDP, or ICMPv6 pseudo-header data are included into the checksum calculation, and the checksum field is overwritten with the final calculated value.

Note that: for ICMP-over-IPv4 packets, the checksum field in the ICMP packet must always be 0x0000 in both modes, because pseudo-headers are not defined for such packets. If it does not equal 0x0000, an incorrect checksum may be inserted into the packet.

The result of this operation is indicated by the payload checksum error status bit in the Transmit Status vector (bit 12). The payload checksum error status bit is set when either of the following is detected:

- the frame has been forwarded to the MAC transmitter in Store-and-forward mode without the end of frame being written to the FIFO
- the packet ends before the number of bytes indicated by the payload length field in the IP header is received.

When the packet is longer than the indicated payload length, the bytes are ignored as stuff bytes, and no error is reported. When the first type of error is detected, the TCP,

1142/1749 RM0090 Rev 18 STI

--- Page 168 ---

RM0090
Ethernet (ETH): media access control (MAC) with DMA controller

UDP or ICMP header is not modified. For the second error type, still, the calculated checksum is inserted into the corresponding header field.

**MII/RMII transmit bit order**

Each nibble from the MII is transmitted on the RMII a dibit at a time with the order of dibit transmission shown in Figure 362. Lower order bits (D1 and D0) are transmitted first followed by higher order bits (D2 and D3).

Figure 362. Transmission bit order

LSB
D0
D1
D2
D3
MSB
MII_TXD[3:0]
Nibble stream

LSB
D0
D1
MSB
RMII_TXD[1:0]
Bit stream
ai15632

**MII/RMII transmit timing diagrams**

Figure 363. Transmission with no collision

MII_TX_CLK
MII_TX_EN
MII_TXD[3:0]
PR
EA
MB
LE
MII_CS
MII_COL
Low
ai15631

ST
RM0090 Rev 18
1143/1749

--- Page 169 ---

Ethernet (ETH): media access control (MAC) with DMA controller
RM0090

Figure 364. Transmission with collision

ai15651

Figure 365 shows a frame transmission in MII and RMII.

Figure 365. Frame transmission in MMI and RMII modes

ai15652


**33.5.5 MAC filtering**

**Address filtering**

Address filtering checks the destination and source addresses on all received frames and the address filtering status is reported accordingly. Address checking is based on different parameters (Frame filter register) chosen by the application. The filtered frame can also be identified: multicast or broadcast frame.

Address filtering uses the station's physical (MAC) address and the Multicast Hash table for address checking purposes.

**Unicast destination address filter**

The MAC supports up to 4 MAC addresses for unicast perfect filtering. If perfect filtering is selected (HU bit in the Frame filter register is reset), the MAC compares all 48 bits of the received unicast address with the programmed MAC address for any match. Default MacAddr0 is always enabled, other addresses MacAddr1–MacAddr3 are selected with an individual enable bit. Each byte of these other addresses (MacAddr1–MacAddr3) can be masked during comparison with the corresponding received DA byte by setting the corresponding Mask Byte Control bit in the register. This helps group address filtering for the DA. In Hash filtering mode (when HU bit is set), the MAC performs imperfect filtering for unicast addresses using a 64-bit Hash table. For hash filtering, the MAC uses the 6 upper CRC (see note 1 below) bits of the received destination address to index the content of the Hash table. A value of 000000 selects bit 0 in the selected register, and a value of 111111 selects bit 63 in the Hash Table register. If the corresponding bit (indicated by the 6-bit CRC) is set to 1, the unicast frame is said to have passed the Hash filter; otherwise, the frame has failed the Hash filter.

*Note:* This CRC is a 32-bit value coded by the following polynomial (for more details refer to Section 33.5.3):

G(x) = x^32 + x^26 + x^23 + x^22 + x^16 + x^12 + x^11 + x^10 + x^8 + x^7 + x^5 + x^4 + x^2 + x + 1

1150/1749 RM0090 Rev 18 ST

--- Page 176 ---

RM0090
Ethernet (ETH): media access control (MAC) with DMA controller

**Multicast destination address filter**

The MAC can be programmed to pass all multicast frames by setting the PAM bit in the Frame filter register. If the PAM bit is reset, the MAC performs the filtering for multicast addresses based on the HM bit in the Frame filter register. In Perfect filtering mode, the multicast address is compared with the programmed MAC destination address registers (1–3). Group address filtering is also supported. In Hash filtering mode, the MAC performs imperfect filtering using a 64-bit Hash table. For hash filtering, the MAC uses the 6 upper CRC (see note 1 below) bits of the received multicast address to index the content of the Hash table. A value of 000000 selects bit 0 in the selected register and a value of 111111 selects bit 63 in the Hash Table register. If the corresponding bit is set to 1, then the multicast frame is said to have passed the Hash filter; otherwise, the frame has failed the Hash filter.

Note: This CRC is a 32-bit value coded by the following polynomial (for more details refer to Section 33.5.3):

G(x) = x^32 + x^26 + x^23 + x^22 + x^16 + x^12 + x^11 + x^10 + x^8 + x^7 + x^5 + x^4 + x^2 + x + 1

**Hash or perfect address filter**

The DA filter can be configured to pass a frame when its DA matches either the Hash filter or the Perfect filter by setting the HPF bit in the Frame filter register and setting the corresponding HU or HM bits. This configuration applies to both unicast and multicast frames. If the HPF bit is reset, only one of the filters (Hash or Perfect) is applied to the received frame.

**Broadcast address filter**

The MAC does not filter any broadcast frames in the default mode. However, if the MAC is programmed to reject all broadcast frames by setting the BFD bit in the Frame filter register, any broadcast frames are dropped.

**Unicast source address filter**

The MAC can also perform perfect filtering based on the source address field of the received frames. By default, the MAC compares the SA field with the values programmed in the SA registers. The MAC address registers [1:3] can be configured to contain SA instead of DA for comparison, by setting bit 30 in the corresponding register. Group filtering with SA is also supported. The frames that fail the SA filter are dropped by the MAC if the SAF bit in the Frame filter register is set. Otherwise, the result of the SA filter is given as a status bit in the Receive Status word (see RDES0: Receive descriptor Word0).

When the SAF bit is set, the result of the SA and DA filters is AND’ed to decide whether the frame needs to be forwarded. This means that either of the filter fail result will drop the frame. Both filters have to pass the frame for the frame to be forwarded to the application.

**Inverse filtering operation**

For both destination and source address filtering, there is an option to invert the filter-match result at the final output. These are controlled by the DAIF and SAIF bits in the Frame filter register, respectively. The DAIF bit is applicable for both Unicast and Multicast DA frames. The result of the unicast/multicast destination address filter is inverted in this mode. Similarly, when the SAIF bit is set, the result of the unicast SA filter is inverted. Table 191

ST
RM0090 Rev 18
1151/1749

--- Page 177 ---

Ethernet (ETH): media access control (MAC) with DMA controller
RM0090

and Table 192 summarize destination and source address filtering based on the type of frame received.

Table 191. Destination address filtering

| Frame type | PM | HPF | HU | DAIF | HM | PAM | DB | DA filter operation |
|---|---|---|---|---|---|---|---|---|
| Broadcast | 1 | X | Pass |  |  |  |  |  |
| 0 | X | 0 | Pass |  |  |  |  |  |
| 0 | X | 1 | Fail |  |  |  |  |  |
| Unicast | 1 | X | Pass all frames |  |  |  |  |  |
| 0 | X | 0 | X | Pass on perfect/group filter match |  |  |  |  |
| 0 | X | 0 | 1 | X | Fail on perfect/Group filter match |  |  |  |
| 0 | 1 | 0 | X | Pass on hash filter match |  |  |  |  |
| 0 | 1 | X | Fail on hash filter match |  |  |  |  |  |
| 0 | 1 | 0 | X | Pass on hash or perfect/Group filter match |  |  |  |  |
| 0 | 1 | X | Fail on hash or perfect/Group filter match |  |  |  |  |  |
| Multicast | 1 | X | Pass all frames |  |  |  |  |  |
| X | 1 | X | Pass all frames |  |  |  |  |  |
| 0 | X | 0 | X | Pass on Perfect/Group filter match and drop PAUSE control frames if PCF = 0x |  |  |  |  |
| 0 | X | 0 | 1 | 0 | X | Pass on hash filter match and drop PAUSE control frames if PCF = 0x |  |  |
| 0 | 1 | X | 0 | 1 | 0 | X | Pass on hash or perfect/Group filter match and drop PAUSE control frames if PCF = 0x |  |
| 0 | X | 1 | 0 | X | Fail on perfect/Group filter match and drop PAUSE control frames if PCF = 0x |  |  |  |
| 0 | X | 1 | 0 | X | Fail on hash filter match and drop PAUSE control frames if PCF = 0x |  |  |  |
| 0 | 1 | X | 1 | 0 | X | Fail on hash or perfect/Group filter match and drop PAUSE control frames if PCF = 0x |  |  |


1152/1749
RM0090 Rev 18

--- Page 178 ---

RM0090
Ethernet (ETH): media access control (MAC) with DMA controller

Table 192. Source address filtering

| Frame type | PM | SAIF | SAF | SA filter operation |
|---|---|---|---|---|
| Unicast | 1 | X | Pass all frames |  |
| 0 | Pass status on perfect/Group filter match but do not drop frames that fail |  |  |  |
| 0 | 1 | 0 | Fail status on perfect/group filter match but do not drop frame |  |
| 0 | 1 | Pass on perfect/group filter match and drop frames that fail |  |  |
| 0 | 1 | Fail on perfect/group filter match and drop frames that fail |  |  |


### 33.5.6 MAC loopback mode

The MAC supports loopback of transmitted frames onto its receiver. By default, the MAC loopback function is disabled, but this feature can be enabled by programming the Loopback bit in the MAC ETH_MACCR register.

### 33.5.7 MAC management counters: MMC

The MAC management counters (MMC) maintain a set of registers for gathering statistics on the received and transmitted frames. These include a control register for controlling the behavior of the registers, two 32-bit registers containing generated interrupts (receive and transmit), and two 32-bit registers containing masks for the Interrupt register (receive and transmit). These registers are accessible from the application. Each register is 32 bits wide.

Section 33.8 describes the various counters and lists the addresses of each of the statistics counters. This address is used for read/write accesses to the desired transmit/receive counter.

The Receive MMC counters are updated for frames that pass address filtering. Dropped frames statistics are not updated unless the dropped frames are runt frames of less than 6 bytes (DA bytes are not received fully).

**Good transmitted and received frames**

Transmitted frames are considered “good” if transmitted successfully. In other words, a transmitted frame is good if the frame transmission is not aborted due to any of the following errors:

* Jabber Timeout
* No Carrier/Loss of Carrier
* Late Collision
* Frame Underflow
* Excessive Deferral
* Excessive Collision

ST
RM0090 Rev 18
1153/1749

--- Page 179 ---

Ethernet (ETH): media access control (MAC) with DMA controller
RM0090

Received frames are considered "good" if none of the following errors exists:
+ CRC error
+ Runt Frame (shorter than 64 bytes)
+ Alignment error (in 10/ 100 Mbit/s only)
+ Length error (non-Type frames only)
+ Out of Range (non-Type frames only, longer than maximum size)
+ MII_RXER Input error

The maximum frame size depends on the frame type, as follows:
+ Untagged frame maxsize = 1518
+ VLAN Frame maxsize = 1522


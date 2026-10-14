### 33.5 Ethernet functional description: MAC 802.3

The IEEE 802.3 International Standard for local area networks (LANs) employs the CSMA/CD (carrier sense multiple access with collision detection) as the access method.

The Ethernet peripheral consists of a MAC 802.3 (media access control) controller with media independent interface (MII) and a dedicated DMA controller.

The MAC block implements the LAN CSMA/CD sublayer for the following families of systems: 10 Mbit/s and 100 Mbit/s of data rates for baseband and broadband systems. Half- and full-duplex operation modes are supported. The collision detection access method is applied only to the half-duplex operation mode. The MAC control frame sublayer is supported.

The MAC sublayer performs the following functions associated with a data link control procedure:

*   Data encapsulation (transmit and receive)
    *   Framing (frame boundary delimitation, frame synchronization)
    *   Addressing (handling of source and destination addresses)
    *   Error detection
*   Media access management
    *   Medium allocation (collision avoidance)
    *   Contention resolution (collision handling)

ST
RM0090 Rev 18
1133/1749

--- Page 159 ---

Ethernet (ETH): media access control (MAC) with DMA controller
RM0090

Basically there are two operating modes of the MAC sublayer:

*   Half-duplex mode: the stations contend for the use of the physical medium, using the CSMA/CD algorithms.
*   Full duplex mode: simultaneous transmission and reception without contention resolution (CSMA/CD algorithm are unnecessary) when all the following conditions are met:
    *   physical medium capability to support simultaneous transmission and reception
    *   exactly 2 stations connected to the LAN
    *   both stations configured for full-duplex operation

### 33.5.1 MAC 802.3 frame format

The MAC block implements the MAC sublayer and the optional MAC control sublayer (10/100 Mbit/s) as specified by the IEEE 802.3-2002 standard.

Two frame formats are specified for data communication systems using the CSMA/CD MAC:

*   Basic MAC frame format
*   Tagged MAC frame format (extension of the basic MAC frame format)

Figure 360 and Figure 361 describe the frame structure (untagged and tagged) that includes the following fields:

*   Preamble: 7-byte field used for synchronization purposes (PLS circuitry)
    Hexadecimal value: 55-55-55-55-55-55-55
    Bit pattern: 01010101 01010101 01010101 01010101 01010101 01010101 01010101 (right-to-left bit transmission)
*   Start frame delimiter (SFD): 1-byte field used to indicate the start of a frame.
    Hexadecimal value: D5
    Bit pattern: 11010101 (right-to-left bit transmission)
*   Destination and Source Address fields: 6-byte fields to indicate the destination and source station addresses as follows (see Figure 359):
    *   Each address is 48 bits in length
    *   The first LSB bit (I/G) in the destination address field is used to indicate an individual (I/G = 0) or a group address (I/G = 1). A group address could identify none, one or more, or all the stations connected to the LAN. In the source address the first bit is reserved and reset to 0.
    *   The second bit (U/L) distinguishes between locally (U/L = 1) or globally (U/L = 0) administered addresses. For broadcast addresses this bit is also 1.
    *   Each byte of each address field must be transmitted least significant bit first.

The address designation is based on the following types:

*   Individual address: this is the physical address associated with a particular station on the network.
*   Group address. A multidestination address associated with one or more stations on a given network. There are two kinds of multicast address:
    *   Multicast-group address: an address associated with a group of logically related stations.
    *   Broadcast address: a distinguished, predefined multicast address (all 1's in the destination address field) that always denotes all the stations on a given LAN.

1134/1749
RM0090 Rev 18
ST

--- Page 160 ---

RM0090
Ethernet (ETH): media access control (MAC) with DMA controller

**Figure 359. Address field format**

MSB 46-bit address U/L I/G LSB
Bit transmission order (right to left)

I/G = 0 Individual address
I/G = 1 Group address
U/L = 0 Globally administered address
U/L = 1 Locally administered address

ai15628

*   QTag Prefix: 4-byte field inserted between the Source address field and the MAC Client Length/Type field. This field is an extension of the basic frame (untagged) to obtain the tagged MAC frame. The untagged MAC frames do not include this field. The extensions for tagging are as follows:
    *   2-byte constant Length/Type field value consistent with the Type interpretation (greater than 0x0600) equal to the value of the 802.1Q Tag Protocol Type (0x8100 hexadecimal). This constant field is used to distinguish tagged and untagged MAC frames.
    *   2-byte field containing the Tag control information field subdivided as follows: a 3-bit user priority, a canonical format indicator (CFI) bit and a 12-bit VLAN Identifier. The length of the tagged MAC frame is extended by 4 bytes by the QTag Prefix.
*   MAC client length/type: 2-byte field with different meaning (mutually exclusive), depending on its value:
    *   If the value is less than or equal to maxValidFrame (0d1500) then this field indicates the number of MAC client data bytes contained in the subsequent data field of the 802.3 frame (length interpretation).
    *   If the value is greater than or equal to MinTypeValue (0d1536 decimal, 0x0600) then this field indicates the nature of the MAC client protocol (Type interpretation) related to the Ethernet frame.

Regardless of the interpretation of the length/type field, if the length of the data field is less than the minimum required for proper operation of the protocol, a PAD field is added after the data field but prior to the FCS (frame check sequence) field. The length/type field is transmitted and received with the higher-order byte first.

For length/type field values in the range between maxValidLength and minTypeValue (boundaries excluded), the behavior of the MAC sublayer is not specified: they may or may not be passed by the MAC sublayer.

*   Data and PAD fields: n-byte data field. Full data transparency is provided, it means that any arbitrary sequence of byte values may appear in the data field. The size of the PAD, if any, is determined by the size of the data field. Max and min length of the data and PAD field are:
    *   Maximum length = 1500 bytes
    *   Minimum length for untagged MAC frames = 46 bytes
    *   Minimum length for tagged MAC frames = 42 bytes

When the data field length is less than the minimum required, the PAD field is added to match the minimum length (42 bytes for tagged frames, 46 bytes for untagged frames).

*   Frame check sequence: 4-byte field that contains the cyclic redundancy check (CRC) value. The CRC computation is based on the following fields: source address, destination address, QTag prefix, length/type, LLC data and PAD (that is, all fields except the preamble, SFD). The generating polynomial is the following:

G(x) = x^32 + x^26 + x^23 + x^22 + x^16 + x^12 + x^11 + x^10 + x^8 + x^7 + x^5 + x^4 + x^2 + x + 1

ST
RM0090 Rev 18
1135/1749

--- Page 161 ---

**Ethernet (ETH): media access control (MAC) with DMA controller** RM0090

The CRC value of a frame is computed as follows:

*   The first 2 bits of the frame are complemented
*   The n-bits of the frame are the coefficients of a polynomial M(x) of degree (n – 1). The first bit of the destination address corresponds to the x^n-1 term and the last bit of the data field corresponds to the x^0 term
*   M(x) is multiplied by x^32 and divided by G(x), producing a remainder R(x) of degree ≤ 31
*   The coefficients of R(x) are considered as a 32-bit sequence
*   The bit sequence is complemented and the result is the CRC
*   The 32-bits of the CRC value are placed in the frame check sequence. The x^32 term is the first transmitted, the x^0 term is the last one

**Figure 360. MAC frame format**

| Size | Field | Notes |
|---|---|---|
| 7 bytes | Preamble | Bytes within frame transmitted top to bottom |
| 1 byte | SFD |  |
| 6 bytes | Destination address |  |
| 6 bytes | Source address |  |
| 2 bytes | MAC client length/type |  |
| 46-1500 bytes | MAC client data |  |
| PAD |  |  |
| 4 bytes | Frame check sequence |  |


MSB [ ] [ ] [ ] [ ] [ ] [ ] [ ] [ ] LSB
Bit transmission order (right to left)

ai15629

1136/1749 RM0090 Rev 18 STI

--- Page 162 ---

RM0090
Ethernet (ETH): media access control (MAC) with DMA controller

Figure 361. Tagged MAC frame format

Each byte of the MAC frame, except the FCS field, is transmitted low-order bit first.
An invalid MAC frame is defined by one of the following conditions:
* The frame length is inconsistent with the expected value as specified by the length/type field. If the length/type field contains a type value, then the frame length is assumed to be consistent with this field (no invalid frame)
* The frame length is not an integer number of bytes (extra bits)
* The CRC value computed on the incoming frame does not match the included FCS


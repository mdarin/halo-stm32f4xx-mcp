> [RM0090 ch.33 §33.8] Ethernet register descriptions (intro)


## Ethernet MAC frame filter register (ETH_MACFFR)

Address offset: 0x0004
Reset value: 0x0000 0000

The MAC frame filter register contains the filter controls for receiving frames. Some of the controls from this register go to the address check block of the MAC, which performs the first level of address filtering. The second level of filtering is performed on the incoming frame, based on other controls such as pass bad frames and pass control frames.

| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| RA | Reserved | HPF | SAF | SAIF | PCF | BFD | PAM | DAIF | HM | HU | PM |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | rw |


**Bit 31 RA: Receive all**

When this bit is set, the MAC receiver passes all received frames on to the application, irrespective of whether they have passed the address filter. The result of the SA/DA filtering is updated (pass or fail) in the corresponding bits in the receive status word. When this bit is reset, the MAC receiver passes on to the application only those frames that have passed the SA/DA address filter.

**Bits 30:11** Reserved, must be kept at reset value.

**Bit 10 HPF: Hash or perfect filter**

When this bit is set and if the HM or HU bit is set, the address filter passes frames that match either the perfect filtering or the hash filtering.
When this bit is cleared and if the HU or HM bit is set, only frames that match the Hash filter are passed.

**Bit 9 SAF: Source address filter**

The MAC core compares the SA field of the received frames with the values programmed in the enabled SA registers. If the comparison matches, then the SAMatch bit in the RxStatus word is set high. When this bit is set high and the SA filter fails, the MAC drops the frame.
When this bit is reset, the MAC core forwards the received frame to the application. It also forwards the updated SA Match bit in RxStatus depending on the SA address comparison.

**Bit 8 SAIF: Source address inverse filtering**

When this bit is set, the address check block operates in inverse filtering mode for the SA address comparison. The frames whose SA matches the SA registers are marked as failing the SA address filter.
When this bit is reset, frames whose SA does not match the SA registers are marked as failing the SA address filter.

1194/1749
RM0090 Rev 18

--- Page 25 ---

RM0090
Ethernet (ETH): media access control (MAC) with DMA controller

**Bits 7:6 PCF: Pass control frames**
These bits control the forwarding of all control frames (including unicast and multicast PAUSE frames). Note that the processing of PAUSE control frames depends only on RFCE in Flow Control Register[2].
00: MAC prevents all control frames from reaching the application
01: MAC forwards all control frames to application except Pause control frames
10: MAC forwards all control frames to application even if they fail the address filter
11: MAC forwards control frames that pass the address filter.

These bits control the forwarding of all control frames (including unicast and multicast PAUSE frames). Note that the processing of PAUSE control frames depends only on RFCE in Flow Control Register[2].
00 or 01: MAC prevents all control frames from reaching the application
10: MAC forwards all control frames to application even if they fail the address filter
11: MAC forwards control frames that pass the address filter.

**Bit 5 BFD: Broadcast frames disable**
When this bit is set, the address filters filter all incoming broadcast frames.
When this bit is reset, the address filters pass all received broadcast frames.

**Bit 4 PAM: Pass all multicast**
When set, this bit indicates that all received frames with a multicast destination address (first bit in the destination address field is '1') are passed.
When reset, filtering of multicast frame depends on the HM bit.

**Bit 3 DAIF: Destination address inverse filtering**
When this bit is set, the address check block operates in inverse filtering mode for the DA address comparison for both unicast and multicast frames.
When reset, normal filtering of frames is performed.

**Bit 2 HM: Hash multicast**
When set, MAC performs destination address filtering of received multicast frames according to the hash table.
When reset, the MAC performs a perfect destination address filtering for multicast frames, that is, it compares the DA field with the values programmed in DA registers.

**Bit 1 HU: Hash unicast**
When set, MAC performs destination address filtering of unicast frames according to the hash table.
When reset, the MAC performs a perfect destination address filtering for unicast frames, that is, it compares the DA field with the values programmed in DA registers.

**Bit 0 PM: Promiscuous mode**
When this bit is set, the address filters pass all incoming frames regardless of their destination or source address. The SA/DA filter fails status bits in the receive status word are always cleared when PM is set.

**Ethernet MAC hash table high register (ETH_MACHTHR)**
Address offset: 0x0008
Reset value: 0x0000 0000

The 64-bit Hash table is used for group address filtering. For hash filtering, the contents of the destination address in the incoming frame are passed through the CRC logic, and the upper 6 bits in the CRC register are used to index the contents of the Hash table. This CRC is a 32-bit value coded by the following polynomial (for more details refer to Section 33.5.3):
G(x) = x^32 + x^26 + x^23 + x^22 + x^16 + x^12 + x^11 + x^10 + x^8 + x^7 + x^5 + x^4 + x^2 + x + 1

ST
RM0090 Rev 18
1195/1749

--- Page 26 ---

**Ethernet (ETH): media access control (MAC) with DMA controller**

RM0090

The most significant bit determines the register to be used (hash table high/hash table low), and the other 5 bits determine which bit within the register. A hash value of 0b0 0000 selects bit 0 in the selected register, and a value of 0b1 1111 selects bit 31 in the selected register.

For example, if the DA of the incoming frame is received as 0x1F52 419C B6AF (0x1F is the first byte received on the MII interface), then the internally calculated 6-bit Hash value is 0x2C and the HTH register bit[12] is checked for filtering. If the DA of the incoming frame is received as 0xA00A 9800 0045, then the calculated 6-bit Hash value is 0x07 and the HTL register bit[7] is checked for filtering.

If the corresponding bit value in the register is 1, the frame is accepted. Otherwise, it is rejected. If the PAM (pass all multicast) bit is set in the ETH_MACFFR register, then all multicast frames are accepted regardless of the multicast hash values.

The Hash table high register contains the higher 32 bits of the multicast Hash table.

| HTH |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bits 31:0 **HTH**: Hash table high
This field contains the upper 32 bits of Hash table.

**Ethernet MAC hash table low register (ETH_MACHTLR)**

Address offset: 0x000C
Reset value: 0x0000 0000
The Hash table low register contains the lower 32 bits of the multi-cast Hash table.

| HTL |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bits 31:0 **HTL**: Hash table low
This field contains the lower 32 bits of the Hash table.

**Ethernet MAC MII address register (ETH_MACMIIAR)**

Address offset: 0x0010
Reset value: 0x0000 0000
The MII address register controls the management cycles to the external PHY through the management interface.

| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved | PA | MR | Reserved | CR | MW | MB |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| rw | rc_w1 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


1196/1749
RM0090 Rev 18

--- Page 27 ---

RM0090
Ethernet (ETH): media access control (MAC) with DMA controller

Bits 31:16 Reserved, must be kept at reset value.
Bits 15:11 **PA**: PHY address
This field tells which of the 32 possible PHY devices are being accessed.
Bits 10:6 **MR**: MII register
These bits select the desired MII register in the selected PHY device.
Bit 5 Reserved, must be kept at reset value.
Bits 4:2 **CR**: Clock range
The CR clock range selection determines the HCLK frequency and is used to decide the frequency of the MDC clock:
Selection HCLK MDC Clock
000 60-100 MHz HCLK/42
001 100-150 MHz HCLK/62
010 20-35 MHz HCLK/16
011 35-60 MHz HCLK/26
100 150-168 MHz HCLK/102
101, 110, 111 Reserved -
Bit 1 **MW**: MII write
When set, this bit tells the PHY that this will be a Write operation using the MII Data register. If this bit is not set, this will be a Read operation, placing the data in the MII Data register.
Bit 0 **MB**: MII busy
This bit should read a logic 0 before writing to ETH_MACMIAR and ETH_MACMIIDR. This bit must also be reset to 0 during a Write to ETH_MACMIAR. During a PHY register access, this bit is set to 0b1 by the application to indicate that a read or write access is in progress. ETH_MACMIIDR (MII Data) should be kept valid until this bit is cleared by the MAC during a PHY Write operation. The ETH_MACMIIDR is invalid until this bit is cleared by the MAC during a PHY Read operation. The ETH_MACMIAR (MII Address) should not be written to until this bit is cleared.

> [RM0090 ch.33 §33.8] Ethernet register descriptions (intro)

### Ethernet MAC MII data register (ETH_MACMIIDR)

### Ethernet MAC MII data register (ETH_MACMIIDR)

Address offset: 0x0014
Reset value: 0x0000 0000

The MAC MII Data register stores write data to be written to the PHY register located at the address specified in ETH_MACMIAR. ETH_MACMIIDR also stores read data from the PHY register located at the address specified by ETH_MACMIAR.

| Reserved | MD |
|---|---|
| rw |  |


Bits 31:16 Reserved, must be kept at reset value.
Bits 15:0 **MD**: MII data
This contains the 16-bit data value read from the PHY after a Management Read operation, or the 16-bit data value to be written to the PHY before a Management Write operation.

ST
RM0090 Rev 18
1197/1749

--- Page 28 ---

**Ethernet (ETH): media access control (MAC) with DMA controller**

RM0090


> [RM0090 ch.33 §33.8] Ethernet register descriptions (intro)


### Ethernet MAC flow control register (ETH_MACFCR)

Address offset: 0x0018
Reset value: 0x0000 0000

The Flow control register controls the generation and reception of the control (Pause Command) frames by the MAC. A write to a register with the Busy bit set to '1' causes the MAC to generate a pause control frame. The fields of the control frame are selected as specified in the 802.3x specification, and the Pause Time value from this register is used in the Pause Time field of the control frame. The Busy bit remains set until the control frame is transferred onto the cable. The Host must make sure that the Busy bit is cleared before writing to the register.

| PT | Reserved | ZQPD | Reserved | PLT | UPFD | RFCE | TFCE | FCB/BPA |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
| rw | rc_w1/rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


**Bits 31:16 PT: Pause time**

This field holds the value to be used in the Pause Time field in the transmit control frame. If the Pause Time bits is configured to be double-synchronized to the MII clock domain, then consecutive write operations to this register should be performed only after at least 4 clock cycles in the destination clock domain.

**Bits 15:8** Reserved, must be kept at reset value.

**Bit 7 ZQPD: Zero-quanta pause disable**

When set, this bit disables the automatic generation of Zero-quanta pause control frames on the deassertion of the flow-control signal from the FIFO layer.

When this bit is reset, normal operation with automatic Zero-quanta pause control frame generation is enabled.

**Bit 6** Reserved, must be kept at reset value.

**Bits 5:4 PLT: Pause low threshold**

This field configures the threshold of the Pause timer at which the Pause frame is automatically retransmitted. The threshold values should always be less than the Pause Time configured in bits[31:16]. For example, if PT = 100H (256 slot-times), and PLT = 01, then a second PAUSE frame is automatically transmitted if initiated at 228 (256 – 28) slot-times after the first PAUSE frame is transmitted.

Selection Threshold
00 Pause time minus 4 slot times
01 Pause time minus 28 slot times
10 Pause time minus 144 slot times
11 Pause time minus 256 slot times

Slot time is defined as time taken to transmit 512 bits (64 bytes) on the MII interface.

**Bit 3 UPFD: Unicast pause frame detect**

When this bit is set, the MAC detects the Pause frames with the station's unicast address specified in the ETH_MACA0HR and ETH_MACA0LR registers, in addition to detecting Pause frames with the unique multicast address.

When this bit is reset, the MAC detects only a Pause frame with the unique multicast address specified in the 802.3x standard.

1198/1749 RM0090 Rev 18

--- Page 29 ---

RM0090
Ethernet (ETH): media access control (MAC) with DMA controller

Bit 2 **RFCE**: Receive flow control enable
When this bit is set, the MAC decodes the received Pause frame and disables its transmitter for a specified (Pause Time) time.
When this bit is reset, the decode function of the Pause frame is disabled.

Bit 1 **TFCE**: Transmit flow control enable
In Full-duplex mode, when this bit is set, the MAC enables the flow control operation to transmit Pause frames. When this bit is reset, the flow control operation in the MAC is disabled, and the MAC does not transmit any Pause frames.
In Half-duplex mode, when this bit is set, the MAC enables the back-pressure operation. When this bit is reset, the back pressure feature is disabled.

Bit 0 **FCB/BPA**: Flow control busy/back pressure activate
This bit initiates a Pause Control frame in Full-duplex mode and activates the back pressure function in Half-duplex mode if TFCE bit is set.
In Full-duplex mode, this bit should be read as 0 before writing to the Flow control register. To initiate a Pause control frame, the Application must set this bit to 1. During a transfer of the Control frame, this bit continues to be set to signify that a frame transmission is in progress. After completion of the Pause control frame transmission, the MAC resets this bit to 0. The Flow control register should not be written to until this bit is cleared.
In Half-duplex mode, when this bit is set (and TFCE is set), back pressure is asserted by the MAC core. During back pressure, when the MAC receives a new frame, the transmitter starts sending a JAM pattern resulting in a collision. When the MAC is configured to Full-duplex mode, the BPA is automatically disabled.

Ethernet MAC VLAN tag register (ETH_MACVLANTR)
Address offset: 0x001C
Reset value: 0x0000 0000

The VLAN tag register contains the IEEE 802.1Q VLAN Tag to identify the VLAN frames. The MAC compares the 13th and 14th bytes of the receiving frame (Length/Type) with 0x8100, and the following 2 bytes are compared with the VLAN tag; if a match occurs, the received VLAN bit in the receive frame status is set. The legal length of the frame is increased from 1518 bytes to 1522 bytes.

| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved | VLANTC | VLANTI |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


ST
RM0090 Rev 18
1199/1749

--- Page 30 ---

**Ethernet (ETH): media access control (MAC) with DMA controller** RM0090

Bits 31:17 Reserved, must be kept at reset value.

Bit 16 **VLANTC**: 12-bit VLAN tag comparison
When this bit is set, a 12-bit VLAN identifier, rather than the complete 16-bit VLAN tag, is used for comparison and filtering. Bits[11:0] of the VLAN tag are compared with the corresponding field in the received VLAN-tagged frame.
When this bit is reset, all 16 bits of the received VLAN frame's fifteenth and sixteenth bytes are used for comparison.

Bits 15:0 **VLANTI**: VLAN tag identifier (for receive frames)
This contains the 802.1Q VLAN tag to identify VLAN frames, and is compared to the fifteenth and sixteenth bytes of the frames being received for VLAN frames. Bits[15:13] are the user priority. Bit[12] is the canonical format indicator (CFI) and bits[11:0] are the VLAN tag's VLAN identifier (VID) field. When the VLANTC bit is set, only the VID (bits[11:0]) is used for comparison.
If VLANTI (VLANTI[11:0] if VLANTC is set) is all zeros, the MAC does not check the fifteenth and sixteenth bytes for VLAN tag comparison, and declares all frames with a Type field value of 0x8100 as VLAN frames.

**Ethernet MAC remote wakeup frame filter register (ETH_MACRWUFFR)**

Address offset: 0x0028
Reset value: 0x0000 0000

This is the address through which the remote wakeup frame filter registers are written/read by the application. The Wakeup frame filter register is actually a pointer to eight (not transparent) such wakeup frame filter registers. Eight sequential write operations to this address with the offset (0x0028) will write all wakeup frame filter registers. Eight sequential read operations from this address with the offset (0x0028) will read all wakeup frame filter registers. This register contains the higher 16 bits of the 7th MAC address. Refer to *Remote wakeup frame filter register* section for additional information.

**Figure 385. Ethernet MAC remote wakeup frame filter register (ETH_MACRWUFFR)**

| Wakeup frame filter reg0 | Filter 0 Byte Mask |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|
| Wakeup frame filter reg1 | Filter 1 Byte Mask |  |  |  |  |  |  |
| Wakeup frame filter reg2 | Filter 2 Byte Mask |  |  |  |  |  |  |
| Wakeup frame filter reg3 | Filter 3 Byte Mask |  |  |  |  |  |  |
| RSVD | Filter 3 Command | RSVD | Filter 2 Command | RSVD | Filter 1 Command | RSVD | Filter 0 Command |
| Filter 3 Offset | Filter 2 Offset | Filter 1 Offset | Filter 0 Offset |  |  |  |  |
| Filter 1 CRC - 16 | Filter 0 CRC - 16 |  |  |  |  |  |  |
| Filter 3 CRC - 16 | Filter 2 CRC - 16 |  |  |  |  |  |  |
| Wakeup frame filter reg4 |  |  |  |  |  |  |  |
| Wakeup frame filter reg5 |  |  |  |  |  |  |  |
| Wakeup frame filter reg6 |  |  |  |  |  |  |  |
| Wakeup frame filter reg7 |  |  |  |  |  |  |  |


ai15648

1200/1749 RM0090 Rev 18 ST

--- Page 31 ---

RM0090
Ethernet (ETH): media access control (MAC) with DMA controller

**Ethernet MAC PMT control and status register (ETH_MACPMTCSR)**

Address offset: 0x002C
Reset value: 0x0000 0000

The ETH_MACPMTCSR programs the request wakeup events and monitors the wakeup events.

| 31 | 30 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|
| WFRPR | Reserved | GU | Reserved | WFR | MPR | Reserved | WFE | MPE | PD |  |  |
| rs | Res. | rw | rc_r |  |  |  | rw | rs |  |  |  |


**Bit 31 WFRPR: Wakeup frame filter register pointer reset**
When set, it resets the Remote wakeup frame filter register pointer to 0b000. It is automatically cleared after 1 clock cycle.

**Bits 30:10 Reserved, must be kept at reset value.**

**Bit 9 GU: Global unicast**
When set, it enables any unicast packet filtered by the MAC (DAF) address recognition to be a wakeup frame.

**Bits 8:7 Reserved, must be kept at reset value.**

**Bit 6 WFR: Wakeup frame received**
When set, this bit indicates the power management event was generated due to reception of a wakeup frame. This bit is cleared by a read into this register.

**Bit 5 MPR: Magic packet received**
When set, this bit indicates the power management event was generated by the reception of a Magic Packet. This bit is cleared by a read into this register.

**Bits 4:3 Reserved, must be kept at reset value.**

**Bit 2 WFE: Wakeup frame enable**
When set, this bit enables the generation of a power management event due to wakeup frame reception.

**Bit 1 MPE: Magic Packet enable**
When set, this bit enables the generation of a power management event due to Magic Packet reception.

**Bit 0 PD: Power down**
When this bit is set, all received frames will be dropped. This bit is cleared automatically when a magic packet or wakeup frame is received, and Power-down mode is disabled. Frames received after this bit is cleared are forwarded to the application. This bit must only be set when either the Magic Packet Enable or Wakeup Frame Enable bit is set high.

ST
RM0090 Rev 18
1201/1749

--- Page 32 ---

Ethernet (ETH): media access control (MAC) with DMA controller
RM0090

**Ethernet MAC debug register (ETH_MACDBGR)**

Address offset: 0x0034
Reset value: 0x0000 0000

This debug register gives the status of all the main modules of the transmit and receive data paths and the FIFOs. An all-zero status indicates that the MAC core is in Idle state (and FIFOs are empty) and no activity is going on in the data paths.

| 31 30 29 28 27 26 | 25 | 24 | 23 22 | 21 | 20 | 19 | 18 17 | 16 | 15 14 13 12 11 10 | 9 | 8 | 7 6 | 5 | 4 | 3 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved | TFF | TFNE | Reserved | TFWA | TFRS | MTP | MTFCS | MMTEA | Reserved | RFFL | Reserved | RFRCS | RFWRA | Reserved | MSFRWCS | MMRPEA |  |
| ro |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bits 31:26 Reserved, must be kept at reset value.

Bit 25 **TFF**: Tx FIFO full
When high, it indicates that the Tx FIFO is full and hence no more frames will be accepted for transmission.

Bit 24 **TFNE**: Tx FIFO not empty
When high, it indicates that the TxFIFO is not empty and has some data left for transmission.

Bit 23 Reserved, must be kept at reset value.

Bit 22 **TFWA**: Tx FIFO write active
When high, it indicates that the TxFIFO write controller is active and transferring data to the TxFIFO.

Bits 21:20 **TFRS**: Tx FIFO read status
This indicates the state of the TxFIFO read controller:
00: Idle state
01: Read state (transferring data to the MAC transmitter)
10: Waiting for TxStatus from MAC transmitter
11: Writing the received TxStatus or flushing the TxFIFO

Bit 19 **MTP**: MAC transmitter in pause
When high, it indicates that the MAC transmitter is in Pause condition (in full-duplex mode only) and hence will not schedule any frame for transmission

Bits 18:17 **MTFCS**: MAC transmit frame controller status
This indicates the state of the MAC transmit frame controller:
00: Idle
01: Waiting for Status of previous frame or IFG/backoff period to be over
10: Generating and transmitting a Pause control frame (in full duplex mode)
11: Transferring input frame for transmission

Bit 16 **MMTEA**: MAC MII transmit engine active
When high, it indicates that the MAC MII transmit engine is actively transmitting data and that it is not in the Idle state.

Bits 15:10 Reserved, must be kept at reset value.

1202/1749
RM0090 Rev 18

--- Page 33 ---

RM0090
Ethernet (ETH): media access control (MAC) with DMA controller

**Bits 9:8 RFFL**: Rx FIFO fill level
This gives the status of the Rx FIFO fill-level:
00: RxFIFO empty
01: RxFIFO fill-level below flow-control de-activate threshold
10: RxFIFO fill-level above flow-control activate threshold
11: RxFIFO full

**Bit 7** Reserved, must be kept at reset value.

**Bits 6:5 RFRCS**: Rx FIFO read controller status
It gives the state of the Rx FIFO read controller:
00: IDLE state
01: Reading frame data
10: Reading frame status (or time-stamp)
11: Flushing the frame data and status

**Bit 4 RFWRA**: Rx FIFO write controller active
When high, it indicates that the Rx FIFO write controller is active and transferring a received frame to the FIFO.

**Bit 3** Reserved, must be kept at reset value.

**Bits 2:1 MSFRWCS**: MAC small FIFO read / write controllers status
When high, these bits indicate the respective active state of the small FIFO read and write controllers of the MAC receive frame controller module.

**Bit 0 MMRPEA**: MAC MII receive protocol engine active
When high, it indicates that the MAC MII receive protocol engine is actively receiving data and is not in the Idle state.

ST
RM0090 Rev 18
1203/1749

--- Page 34 ---

**Ethernet (ETH): media access control (MAC) with DMA controller**

**RM0090**

**Ethernet MAC interrupt status register (ETH_MACSR)**

Address offset: 0x0038
Reset value: 0x0000 0000

The ETH_MACSR register contents identify the events in the MAC that can generate an interrupt.

| 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0 |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|
| Reserved | TSTS | Reserved | MMCTS | MMCRS | MMCS | PMTS | Reserved |
| rc_r | r |  |  |  |  |  |  |


Bits 15:10 Reserved, must be kept at reset value.

**Bit 9 TSTS:** Time stamp trigger status
This bit is set high when the system time value equals or exceeds the value specified in the Target time high and low registers. This bit is cleared by reading the ETH_PTPTSSR register.

Bits 8:7 Reserved, must be kept at reset value.

**Bit 6 MMCTS:** MMC transmit status
This bit is set high whenever an interrupt is generated in the ETH_MMCTIR Register. This bit is cleared when all the bits in this interrupt register (ETH_MMCTIR) are cleared.

**Bit 5 MMCRS:** MMC receive status
This bit is set high whenever an interrupt is generated in the ETH_MMCRIR register. This bit is cleared when all the bits in this interrupt register (ETH_MMCRIR) are cleared.

**Bit 4 MMCS:** MMC status
This bit is set high whenever any of bits 6:5 is set high. It is cleared only when both bits are low.

**Bit 3 PMTS:** PMT status
This bit is set whenever a Magic packet or Wake-on-LAN frame is received in Power-down mode (see bits 5 and 6 in the ETH_MACPMTCSR register *Ethernet MAC PMT control and status register (ETH_MACPMTCSR)*). This bit is cleared when both bits[6:5], of this last register, are cleared due to a read operation to the ETH_MACPMTCSR register.

Bits 2:0 Reserved, must be kept at reset value.

1204/1749 RM0090 Rev 18 ST

--- Page 35 ---

RM0090
Ethernet (ETH): media access control (MAC) with DMA controller

**Ethernet MAC interrupt mask register (ETH_MACIMR)**

Address offset: 0x003C
Reset value: 0x0000 0000

The ETH_MACIMR register bits make it possible to mask the interrupt signal due to the corresponding event in the ETH_MACSR register.

| 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved | TSTIM | Reserved | PMTIM | Reserved |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  | rw |  |  |  |  |  | rw |  |  |  |


Bits 15:10 Reserved, must be kept at reset value.
Bit 9 **TSTIM**: Time stamp trigger interrupt mask
When set, this bit disables the time stamp interrupt generation.
Bits 8:4 Reserved, must be kept at reset value.
Bit 3 **PMTIM**: PMT interrupt mask
When set, this bit disables the assertion of the interrupt signal due to the setting of the PMT Status bit in ETH_MACSR.
Bits 2:0 Reserved, must be kept at reset value.

**Ethernet MAC address 0 high register (ETH_MACA0HR)**

Address offset: 0x0040
Reset value: 0x8000 FFFF

The MAC address 0 high register holds the upper 16 bits of the 6-byte first MAC address of the station. Note that the first DA byte that is received on the MII interface corresponds to the LS Byte (bits [7:0]) of the MAC address low register. For example, if 0x1122 3344 5566 is received (0x11 is the first byte) on the MII as the destination address, then the MAC address 0 register [47:0] is compared with 0x6655 4433 2211.

| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| MO | Reserved | MACA0H |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 1 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bit 31 **MO**: Always 1.
Bits 30:16 Reserved, must be kept at reset value.
Bits 15:0 **MACA0H**: MAC address0 high [47:32]
This field contains the upper 16 bits (47:32) of the 6-byte MAC address0. This is used by the MAC for filtering for received frames and for inserting the MAC address in the transmit flow control (Pause) frames.

ST
RM0090 Rev 18
1205/1749

--- Page 36 ---

**Ethernet (ETH): media access control (MAC) with DMA controller** RM0090

**Ethernet MAC address 0 low register (ETH_MACA0LR)**

Address offset: 0x0044
Reset value: 0xFFFF FFFF

The MAC address 0 low register holds the lower 32 bits of the 6-byte first MAC address of the station.

| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| MACA0L |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bits 31:0 **MACA0L**: MAC address0 low [31:0]
This field contains the lower 32 bits of the 6-byte MAC address0. This is used by the MAC for filtering for received frames and for inserting the MAC address in the transmit flow control (Pause) frames.

**Ethernet MAC address 1 high register (ETH_MACA1HR)**

Address offset: 0x0048
Reset value: 0x0000 FFFF

The MAC address 1 high register holds the upper 16 bits of the 6-byte second MAC address of the station.

| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| AE | SA | MBC | Reserved | MACA1H |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bit 31 **AE**: Address enable
When this bit is set, the address filters use the MAC address1 for perfect filtering. When this bit is cleared, the address filters ignore the address for filtering.

Bit 30 **SA**: Source address
When this bit is set, the MAC address1[47:0] is used for comparison with the SA fields of the received frame.
When this bit is cleared, the MAC address1[47:0] is used for comparison with the DA fields of the received frame.

Bits 29:24 **MBC**: Mask byte control
These bits are mask control bits for comparison of each of the MAC address1 bytes. When they are set high, the MAC core does not compare the corresponding byte of received DA/SA with the contents of the MAC address1 registers. Each bit controls the masking of the bytes as follows:
– Bit 29: ETH_MACA1HR [15:8]
– Bit 28: ETH_MACA1HR [7:0]
– Bit 27: ETH_MACA1LR [31:24]
...
– Bit 24: ETH_MACA1LR [7:0]

1206/1749 RM0090 Rev 18 ST

--- Page 37 ---

**RM0090**
**Ethernet (ETH): media access control (MAC) with DMA controller**

Bits 23:16 Reserved, must be kept at reset value.
Bits 15:0 **MACA1H**: MAC address1 high [47:32]
This field contains the upper 16 bits (47:32) of the 6-byte second MAC address.

**Ethernet MAC address1 low register (ETH_MACA1LR)**
Address offset: 0x004C
Reset value: 0xFFFF FFFF
The MAC address 1 low register holds the lower 32 bits of the 6-byte second MAC address of the station.

| MACA1L |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bits 31:0 **MACA1L**: MAC address1 low [31:0]
This field contains the lower 32 bits of the 6-byte MAC address1. The content of this field is undefined until loaded by the application after the initialization process.

**Ethernet MAC address 2 high register (ETH_MACA2HR)**
Address offset: 0x0050
Reset value: 0x0000 FFFF
The MAC address 2 high register holds the upper 16 bits of the 6-byte second MAC address of the station.

| AE SA MBC | Reserved | MACA2H |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


STI
RM0090 Rev 18
1207/1749

--- Page 38 ---

**Ethernet (ETH): media access control (MAC) with DMA controller** RM0090

Bit 31 **AE**: Address enable
When this bit is set, the address filters use the MAC address2 for perfect filtering. When reset, the address filters ignore the address for filtering.

Bit 30 **SA**: Source address
When this bit is set, the MAC address 2 [47:0] is used for comparison with the SA fields of the received frame.
When this bit is reset, the MAC address 2 [47:0] is used for comparison with the DA fields of the received frame.

Bits 29:24 **MBC**: Mask byte control
These bits are mask control bits for comparison of each of the MAC address2 bytes. When set high, the MAC core does not compare the corresponding byte of received DA/SA with the contents of the MAC address 2 registers. Each bit controls the masking of the bytes as follows:
– Bit 29: ETH_MACA2HR [15:8]
– Bit 28: ETH_MACA2HR [7:0]
– Bit 27: ETH_MACA2LR [31:24]
...
– Bit 24: ETH_MACA2LR [7:0]

Bits 23:16 Reserved, must be kept at reset value.

Bits 15:0 **MACA2H**: MAC address2 high [47:32]
This field contains the upper 16 bits (47:32) of the 6-byte MAC address2.

**Ethernet MAC address 2 low register (ETH_MACA2LR)**

Address offset: 0x0054
Reset value: 0xFFFF FFFF

The MAC address 2 low register holds the lower 32 bits of the 6-byte second MAC address of the station.

| MACA2L |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
| RW |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bits 31:0 **MACA2L**: MAC address2 low [31:0]
This field contains the lower 32 bits of the 6-byte second MAC address2. The content of this field is undefined until loaded by the application after the initialization process.

1208/1749 RM0090 Rev 18 ST

--- Page 39 ---

RM0090
Ethernet (ETH): media access control (MAC) with DMA controller

> [RM0090 ch.33 §33.8] Ethernet register descriptions (intro)

### Ethernet MAC address 3 high register (ETH_MACA3HR)

### Ethernet MAC address 3 high register (ETH_MACA3HR)

Address offset: 0x0058
Reset value: 0x0000 FFFF
The MAC address 3 high register holds the upper 16 bits of the 6-byte second MAC address of the station.

| 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0 |  |  |  |  |
|---|---|---|---|---|
| AE | SA | MBC | Reserved | MACA3H |
| rw |  |  |  |  |


**Bit 31 AE: Address enable**
When this bit is set, the address filters use the MAC address3 for perfect filtering. When this bit is cleared, the address filters ignore the address for filtering.

**Bit 30 SA: Source address**
When this bit is set, the MAC address 3 [47:0] is used for comparison with the SA fields of the received frame.
When this bit is cleared, the MAC address 3[47:0] is used for comparison with the DA fields of the received frame.

**Bits 29:24 MBC: Mask byte control**
These bits are mask control bits for comparison of each of the MAC address3 bytes. When these bits are set high, the MAC core does not compare the corresponding byte of received DA/SA with the contents of the MAC address 3 registers. Each bit controls the masking of the bytes as follows:
- Bit 29: ETH_MACA3HR [15:8]
- Bit 28: ETH_MACA3HR [7:0]
- Bit 27: ETH_MACA3LR [31:24]
- ...
- Bit 24: ETH_MACA3LR [7:0]

**Bits 23:16** Reserved, must be kept at reset value.

**Bits 15:0 MACA3H: MAC address3 high [47:32]**
This field contains the upper 16 bits (47:32) of the 6-byte MAC address3.

> [RM0090 ch.33 §33.8] Ethernet register descriptions (intro)

### Ethernet MAC address 3 low register (ETH_MACA3LR)

### Ethernet MAC address 3 low register (ETH_MACA3LR)

Address offset: 0x005C
Reset value: 0xFFFF FFFF
The MAC address 3 low register holds the lower 32 bits of the 6-byte second MAC address of the station.

| 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0 |
|---|
| MACA3L |
| rw |


**Bits 31:0 MACA3L: MAC address3 low [31:0]**
This field contains the lower 32 bits of the 6-byte second MAC address3. The content of this field is undefined until loaded by the application after the initialization process.

ST
RM0090 Rev 18
1209/1749

--- Page 40 ---

Ethernet (ETH): media access control (MAC) with DMA controller
RM0090


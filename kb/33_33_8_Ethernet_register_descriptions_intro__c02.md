> [RM0090 ch.33 §33.8] Ethernet register descriptions (intro)


#### 33.8.1 MAC register description

**Ethernet MAC configuration register (ETH_MACCR)**

Address offset: 0x0000
Reset value: 0x0000 8000

The MAC configuration register is the operation mode register of the MAC. It establishes receive and transmit operating modes.

| 31 30 29 28 27 26 | 25 24 | 23 22 | 21 20 | 19 18 | 17 16 | 15 14 | 13 12 | 11 10 | 9 8 | 7 6 | 5 4 | 3 2 | 1 0 |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved | CSTFrw | Reserved | WDrw | JDrw | Reserved | IFGrw | CSDrw | Reserved | FESrw | RODrw | LMrw | DMrw | IPCOrw | RDrw | Reserved | APCSrw | BLrw | DCrw | TErw | RErw | Reserved |


Bits 31:26 Reserved, must be kept at reset value.

**CSTF**: CRC stripping for Type frames
Bit 25 When set, the last 4 bytes (FCS) of all frames of Ether type (type field greater than 0x0600) will be stripped and dropped before forwarding the frame to the application.
Bit 24 Reserved, must be kept at reset value.

Bit 23 **WD**: Watchdog disable
When this bit is set, the MAC disables the watchdog timer on the receiver, and can receive frames of up to 16 384 bytes.
When this bit is reset, the MAC allows no more than 2 048 bytes of the frame being received and cuts off any bytes received after that.

Bit 22 **JD**: Jabber disable
When this bit is set, the MAC disables the jabber timer on the transmitter, and can transfer frames of up to 16 384 bytes.
When this bit is reset, the MAC cuts off the transmitter if the application sends out more than 2 048 bytes of data during transmission.

Bits 21:20 Reserved, must be kept at reset value.

ST
RM0090 Rev 18
1191/1749

--- Page 22 ---

**Ethernet (ETH): media access control (MAC) with DMA controller** RM0090

Bits 19:17 **IFG**: Interframe gap
These bits control the minimum interframe gap between frames during transmission.
000: 96 bit times
001: 88 bit times
010: 80 bit times
...
111: 40 bit times
*Note: In Half-duplex mode, the minimum IFG can be configured for 64 bit times (IFG = 100) only. Lower values are not considered.*

Bit 16 **CSD**: Carrier sense disable
When set high, this bit makes the MAC transmitter ignore the MII CRS signal during frame transmission in Half-duplex mode. No error is generated due to Loss of Carrier or No Carrier during such transmission.
When this bit is low, the MAC transmitter generates such errors due to Carrier Sense and even aborts the transmissions.

Bit 15 Reserved, must be kept at reset value.

Bit 14 **FES**: Fast Ethernet speed
Indicates the speed in Fast Ethernet (MII) mode:
0: 10 Mbit/s
1: 100 Mbit/s

Bit 13 **ROD**: Receive own disable
When this bit is set, the MAC disables the reception of frames in Half-duplex mode.
When this bit is reset, the MAC receives all packets that are given by the PHY while transmitting.
This bit is not applicable if the MAC is operating in Full-duplex mode.

Bit 12 **LM**: Loopback mode
When this bit is set, the MAC operates in loopback mode at the MII. The MII receive clock input (RX_CLK) is required for the loopback to work properly, as the transmit clock is not looped-back internally.

Bit 11 **DM**: Duplex mode
When this bit is set, the MAC operates in a Full-duplex mode where it can transmit and receive simultaneously.

Bit 10 **IPCO**: IPv4 checksum offload
When set, this bit enables IPv4 checksum checking for received frame payloads' TCP/UDP/ICMP headers. When this bit is reset, the checksum offload function in the receiver is disabled and the corresponding PCE and IP HCE status bits (see Table 190) are always cleared.

Bit 9 **RD**: Retry disable
When this bit is set, the MAC attempts only 1 transmission. When a collision occurs on the MII, the MAC ignores the current frame transmission and reports a Frame Abort with excessive collision error in the transmit frame status.
When this bit is reset, the MAC attempts retries based on the settings of BL.
*Note: This bit is applicable only in the Half-duplex mode.*

Bit 8 Reserved, must be kept at reset value.

1192/1749 RM0090 Rev 18 ST

--- Page 23 ---

**RM0090**
**Ethernet (ETH): media access control (MAC) with DMA controller**

**Bit 7 APCS: Automatic pad/CRC stripping**
When this bit is set, the MAC strips the Pad/FCS field on incoming frames only if the length's field value is less than or equal to 1 500 bytes. All received frames with length field greater than or equal to 1 501 bytes are passed on to the application without stripping the Pad/FCS field.
When this bit is reset, the MAC passes all incoming frames unmodified.

**Bits 6:5 BL: Back-off limit**
The Back-off limit determines the random integer number (r) of slot time delays (4 096 bit times for 1000 Mbit/s and 512 bit times for 10/100 Mbit/s) the MAC waits before rescheduling a transmission attempt during retries after a collision.
*Note: This bit is applicable only to Half-duplex mode.*
00: k = min (n, 10)
01: k = min (n, 8)
10: k = min (n, 4)
11: k = min (n, 1),
where n = retransmission attempt. The random integer r takes the value in the range 0 ≤ r < 2^k

**Bit 4 DC: Deferral check**
When this bit is set, the deferral check function is enabled in the MAC. The MAC issues a Frame Abort status, along with the excessive deferral error bit set in the transmit frame status when the transmit state machine is deferred for more than 24 288 bit times in 10/100-Mbit/s mode. Deferral begins when the transmitter is ready to transmit, but is prevented because of an active CRS (carrier sense) signal on the MII. Defer time is not cumulative. If the transmitter defers for 10 000 bit times, then transmits, collides, backs off, and then has to defer again after completion of back-off, the deferral timer resets to 0 and restarts.
When this bit is reset, the deferral check function is disabled and the MAC defers until the CRS signal goes inactive. This bit is applicable only in Half-duplex mode.

**Bit 3 TE: Transmitter enable**
When this bit is set, the transmit state machine of the MAC is enabled for transmission on the MII. When this bit is reset, the MAC transmit state machine is disabled after the completion of the transmission of the current frame, and does not transmit any further frames.

**Bit 2 RE: Receiver enable**
When this bit is set, the receiver state machine of the MAC is enabled for receiving frames from the MII. When this bit is reset, the MAC receive state machine is disabled after the completion of the reception of the current frame, and will not receive any further frames from the MII.

**Bits 1:0** Reserved, must be kept at reset value.

ST
RM0090 Rev 18
1193/1749

--- Page 24 ---

**Ethernet (ETH): media access control (MAC) with DMA controller**

RM0090


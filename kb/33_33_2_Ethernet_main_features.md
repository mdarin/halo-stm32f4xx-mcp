# 33 Ethernet (ETH): media access control (MAC) with DMA controller

This section applies only to STM32F42xx/F43xx and STM32F407x/F417x devices.

## 33.1 Ethernet introduction

Portions Copyright (c) 2004, 2005 Synopsys, Inc. All rights reserved. Used with permission.

The Ethernet peripheral enables the STM32F4xx to transmit and receive data over Ethernet in compliance with the IEEE 802.3-2002 standard.

The Ethernet provides a configurable, flexible peripheral to meet the needs of various applications and customers. It supports two industry standard interfaces to the external physical layer (PHY): the default media independent interface (MII) defined in the IEEE 802.3 specifications and the reduced media independent interface (RMII). It can be used in number of applications such as switches, network interface cards, etc.

The Ethernet is compliant with the following standards:

* IEEE 802.3-2002 for Ethernet MAC
* IEEE 1588-2008 standard for precision networked clock synchronization
* AMBA 2.0 for AHB Master/Slave ports
* RMII specification from RMII consortium

## 33.2 Ethernet main features

The Ethernet (ETH) peripheral includes the following features, listed by category:

1122/1749 RM0090 Rev 18 ST

--- Page 148 ---

RM0090
Ethernet (ETH): media access control (MAC) with DMA controller

### 33.2.1 MAC core features

*   Supports 10/100 Mbit/s data transfer rates with external PHY interfaces
*   IEEE 802.3-compliant MII interface to communicate with an external Fast Ethernet PHY
*   Supports both full-duplex and half-duplex operations
    *   Supports CSMA/CD Protocol for half-duplex operation
    *   Supports IEEE 802.3x flow control for full-duplex operation
    *   Optional forwarding of received pause control frames to the user application in full-duplex operation
    *   Back-pressure support for half-duplex operation
    *   Automatic transmission of zero-quanta pause frame on deassertion of flow control input in full-duplex operation
*   Preamble and start-of-frame data (SFD) insertion in Transmit, and deletion in Receive paths
*   Automatic CRC and pad generation controllable on a per-frame basis
*   Options for automatic pad/CRC stripping on receive frames
*   Programmable frame length to support Standard frames with sizes up to 16 KB
*   Programmable interframe gap (40-96 bit times in steps of 8)
*   Supports a variety of flexible address filtering modes:
    *   Up to four 48-bit perfect (DA) address filters with masks for each byte
    *   Up to three 48-bit SA address comparison check with masks for each byte
    *   64-bit Hash filter (optional) for multicast and unicast (DA) addresses
    *   Option to pass all multicast addressed frames
    *   Promiscuous mode support to pass all frames without any filtering for network monitoring
    *   Passes all incoming packets (as per filter) with a status report
*   Separate 32-bit status returned for transmission and reception packets
*   Supports IEEE 802.1Q VLAN tag detection for reception frames
*   Separate transmission, reception, and control interfaces to the Application
*   Supports mandatory network statistics with RMON/MIB counters (RFC2819/RFC2665)
*   MDIO interface for PHY device configuration and management
*   Detection of LAN wakeup frames and AMD Magic Packet™ frames
*   Receive feature for checksum off-load for received IPv4 and TCP packets encapsulated by the Ethernet frame
*   Enhanced receive feature for checking IPv4 header checksum and TCP, UDP, or ICMP checksum encapsulated in IPv4 or IPv6 datagrams
*   Support Ethernet frame time stamping as described in IEEE 1588-2008. Sixty-four-bit time stamps are given in each frame's transmit or receive status
*   Two sets of FIFOs: a 2-KB Transmit FIFO with programmable threshold capability, and a 2-KB Receive FIFO with a configurable threshold (default of 64 bytes)
*   Receive Status vectors inserted into the Receive FIFO after the EOF transfer enables multiple-frame storage in the Receive FIFO without requiring another FIFO to store those frames' Receive Status
*   Option to filter all error frames on reception and not forward them to the application in

RM0090 Rev 18
1123/1749

--- Page 149 ---

**Ethernet (ETH): media access control (MAC) with DMA controller** RM0090

* Store-and-Forward mode
* Option to forward under-sized good frames
* Supports statistics by generating pulses for frames dropped or corrupted (due to overflow) in the Receive FIFO
* Supports Store and Forward mechanism for transmission to the MAC core
* Automatic generation of PAUSE frame control or back pressure signal to the MAC core based on Receive FIFO-fill (threshold configurable) level
* Handles automatic retransmission of Collision frames for transmission
* Discards frames on late collision, excessive collisions, excessive deferral and underrun conditions
* Software control to flush Tx FIFO
* Calculates and inserts IPv4 header checksum and TCP, UDP, or ICMP checksum in frames transmitted in Store-and-Forward mode
* Supports internal loopback on the MII for debugging

### 33.2.2 DMA features

* Supports all AHB burst types in the AHB Slave Interface
* Software can select the type of AHB burst (fixed or indefinite burst) in the AHB Master interface.
* Option to select address-aligned bursts from AHB master port
* Optimization for packet-oriented DMA transfers with frame delimiters
* Byte-aligned addressing for data buffer support
* Dual-buffer (ring) or linked-list (chained) descriptor chaining
* Descriptor architecture, allowing large blocks of data transfer with minimum CPU intervention;
* each descriptor can transfer up to 8 KB of data
* Comprehensive status reporting for normal operation and transfers with errors
* Individual programmable burst size for Transmit and Receive DMA Engines for optimal host bus utilization
* Programmable interrupt options for different operational conditions
* Per-frame Transmit/Receive complete interrupt control
* Round-robin or fixed-priority arbitration between Receive and Transmit engines
* Start/Stop modes
* Current Tx/Rx Buffer pointer as status registers
* Current Tx/Rx Descriptor pointer as status registers

### 33.2.3 PTP features

* Received and transmitted frames time stamping
* Coarse and fine correction methods
* Trigger interrupt when system time becomes greater than target time
* Pulse per second output (product alternate function output)

1124/1749 RM0090 Rev 18 STI

--- Page 150 ---

RM0090
Ethernet (ETH): media access control (MAC) with DMA controller

### 33.3 Ethernet pins

Table 185 shows the MAC signals and the corresponding MII/RMII signal mapping. All MAC signals are mapped onto AF11, some signals are mapped onto different I/O pins, and should be configured in Alternate function mode (for more details, refer to Section 8.3.2: I/O pin multiplexer and mapping).

Table 185. Alternate function mapping

| Port | AF11 |
|---|---|
| ETH |  |
| PA0-WKUP | ETH_MII_CRS |
| PA1 | ETH_MII_RX_CLK / ETH_RMII_REF_CLK |
| PA2 | ETH_MDIO |
| PA3 | ETH_MII_COL |
| PA7 | ETH_MII_RX_DV / ETH_RMII_CRS_DV |
| PB0 | ETH_MII_RXD2 |
| PB1 | ETH_MII_RXD3 |
| PB5 | ETH_PPS_OUT |
| PB8 | ETH_MII_TXD3 |
| PB10 | ETH_MII_RX_ER |
| PB11 | ETH_MII_TX_EN / ETH_RMII_TX_EN |
| PB12 | ETH_MII_TXD0 / ETH_RMII_TXD0 |
| PB13 | ETH_MII_TXD1 / ETH_RMII_TXD1 |
| PC1 | ETH_MDC |
| PC2 | ETH_MII_TXD2 |
| PC3 | ETH_MII_TX_CLK |
| PC4 | ETH_MII_RXD0 / ETH_RMII_RXD0 |
| PC5 | ETH_MII_RXD1 / ETH_RMII_RXD1 |
| PE2 | ETH_MII_TXD3 |
| PG8 | ETH_PPS_OUT |
| PG11 | ETH_MII_TX_EN / ETH_RMII_TX_EN |
| PG13 | ETH_MII_TXD0 / ETH_RMII_TXD0 |
| PG14 | ETH_MII_TXD1 / ETH_RMII_TXD1 |
| PH2 | ETH_MII_CRS |
| PH3 | ETH_MII_COL |
| PH6 | ETH_MII_RXD2 |
| PH7 | ETH_MII_RXD3 |
| PI10 | ETH_MII_RX_ER |


ST
RM0090 Rev 18
1125/1749

--- Page 151 ---

Ethernet (ETH): media access control (MAC) with DMA controller
RM0090


### 35.10 FIFO RAM allocation

#### 35.10.1 Peripheral mode

**Receive FIFO RAM**

For Receive FIFO RAM, the application should allocate RAM for SETUP packets: 10 locations must be reserved in the receive FIFO to receive SETUP packets on control endpoints. These locations are reserved for SETUP packets and are not used by the core to write any other data.

One location must be allocated for Global OUT NAK. Status information are also written to the FIFO along with each received packet. Therefore, a minimum space of (Largest Packet Size / 4) + 1 must be allocated to receive packets. If a high-bandwidth endpoint or multiple isochronous endpoints are enabled, at least two spaces of (Largest Packet Size / 4) + 1 must be allotted to receive back-to-back packets. Typically, two (Largest Packet Size / 4) + 1 spaces are recommended so that when the previous packet is being transferred to AHB, the USB can receive the subsequent packet.

Along with each endpoints last packet, transfer complete status information are also pushed to the FIFO. Typically, one location for each OUT endpoint is recommended.

**Transmit FIFO RAM**

For Transmit FIFO RAM, the minimum RAM space required for each IN Endpoint Transmit FIFO is the maximum packet size for this IN endpoint.

*Note:* More space allocated in the transmit IN Endpoint FIFO results in a better performance on the USB.

#### 35.10.2 Host mode

**Receive FIFO RAM**

For Receive FIFO RAM allocation, Status information are written to the FIFO along with each received packet. Therefore, a minimum space of (Largest Packet Size / 4) + 1 must be allocated to receive packets. If a high-bandwidth channel or multiple isochronous channels are enabled, at least two spaces of (Largest Packet Size / 4) + 1 must be allocated to receive back-to-back packets. Typically, two (Largest Packet Size / 4) + 1 spaces are recommended so that when the previous packet is being transferred to AHB, the USB can receive the subsequent packet.

Along with each host channels last packet, transfer complete status information are also pushed to the FIFO. As a consequence, one location must be allocated to store this data.

**Transmit FIFO RAM**

For Transmit FIFO RAM allocation, the minimum amount of RAM required for the host nonperiodic Transmit FIFO is the largest maximum packet size for all supported nonperiodic OUT channels. Typically, a space corresponding to two Largest Packet Size is recommended, so that when the current packet is being transferred to the USB, the AHB can transmit the subsequent packet.

The minimum amount of RAM required for Host periodic Transmit FIFO is the largest maximum packet size for all supported periodic OUT channels. If there is at least one High

ST
RM0090 Rev 18
1399/1749

--- Page 35 ---

**USB on-the-go high-speed (OTG_HS)** RM0090

**Note:** Bandwidth Isochronous OUT endpoint, then the space must be at least two times the maximum packet size for that channel.

*More space allocated in the Transmit nonperiodic FIFO results in better performance on the USB.*

*When operating in DMA mode, the DMA address register for each host channel (HCDMAn) is stored in the SPRAM (FIFO). One location for each channel must be reserved for this.*


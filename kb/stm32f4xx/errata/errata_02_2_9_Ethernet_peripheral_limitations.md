**2.9 Ethernet peripheral limitations**

**2.9.1 Incorrect layer 3 (L3) checksum is inserted in transmitted IPv6 packets without TCP, UDP or ICMP payloads**

**Description**

The application provides the per-frame control to instruct the MAC to insert the L3 checksums for TCP, UDP and ICMP packets. When automatic checksum insertion is enabled and the input packet is an IPv6 packet without the TCP, UDP or ICMP payload, then the MAC may incorrectly insert a checksum into the packet. For IPv6 packets without a TCP, UDP or ICMP payload, the MAC core considers the next header (NH) field as the extension header and continues to parse the extension header. Sometimes, the payload data in such packets matches the NH field for TCP, UDP or ICMP and, as a result, the MAC core inserts a checksum.

**Workaround**

When the IPv6 packets have a TCP, UDP or ICMP payload, enable checksum insertion for transmit frames, or bypass checksum insertion by using the CIC (checksum insertion control) bits in TDES0 (bits 23:22).

**2.9.2 The Ethernet MAC processes invalid extension headers in the received IPv6 frames**

**Description**

In IPv6 frames, there can be zero or some extension headers preceding the actual IP payload. The Ethernet MAC processes the following extension headers defined in the IPv6 protocol: Hop-by-Hop Options header, Routing header and Destination Options header. All extension headers, except the Hop-by-Hop extension header, can be present multiple times and in any order before the actual IP payload. The Hop-by-Hop extension header, if present, has to come immediately after the IPv6’s main header.

The Ethernet MAC processes all (valid or invalid) extension headers including the Hop-by- Hop extension headers that are present after the first extension header. For this reason, the GMAC core will accept IPv6 frames with invalid Hop-by-Hop extension headers. As a consequence, it will accept any IP payload as valid IPv6 frames with TCP, UDP or ICMP payload, and then incorrectly update the Receive status of the corresponding frame.

**Workaround**

None.



37

--- Page 26 ---

**STM32F40x and STM32F41x silicon limitations STM32F40x and STM32F41x**

**2.9.3 MAC stuck in the Idle state on receiving the TxFIFO flush command exactly 1 clock cycle after a transmission completes**

**Description**

When the software issues a TxFIFO flush command, the transfer of frame data stops (even in the middle of a frame transfer). The TxFIFO read controller goes into the Idle state (TFRS=00 in ETH_MACDBGR) and then resumes its normal operation.

However, if the TxFIFO read controller receives the TxFIFO flush command exactly one clock cycle after receiving the status from the MAC, the controller remains stuck in the Idle state and stops transmitting frames from the TxFIFO. The system can recover from this state only with a reset (e.g. a soft reset).

**Workaround**

Do not use the TxFIFO flush feature.

If TXFIFO flush is really needed, wait until the TxFIFO is empty prior to using the TxFIFO flush command.

**2.9.4 Transmit frame data corruption**

Frame data corrupted when the TxFIFO is repeatedly transitioning from non-empty to empty and then back to non-empty.

**Description**

Frame data may get corrupted when the TxFIFO is repeatedly transitioning from non-empty to empty for a very short period, and then from empty to non-empty, without causing an underflow.

This transitioning from non-empty to empty and back to non-empty happens when the rate at which the data is being written to the TxFIFO is almost equal to or a little less than the rate at which the data is being read.

This corruption cannot be detected by the receiver when the CRC is inserted by the MAC, as the corrupted data is used for the CRC computation.

**Workaround**

Use the Store-and-Forward mode: TSF=1 (bit 21 in ETH_DMAOMR). In this mode, the data is transmitted only when the whole packet is available in the TxFIFO.

**2.9.5 Successive write operations to the same register might not be fully taken into account**

**Description**

A write to a register might not be fully taken into account if a previous write to the same register is performed within a time period of four TX_CLK/RX_CLK clock cycles. When this error occurs, reading the register returns the most recently written value, but the Ethernet MAC continues to operate as if the latest write operation never occurred.

See Table 5: Impacted registers and bits for the registers and bits impacted by this limitation.



--- Page 27 ---

**STM32F40x and STM32F41x STM32F40x and STM32F41x silicon limitations**

**Table 5. Impacted registers and bits**

| Register name | Bit number | Bit name |
|---|---|---|
| DMA registers |  |  |
| ETH DMABMR _ | 7 | EDFE |
| ETH DMAOMR _ | 26 | DTCEFD |
| 25 | RSF |  |
| 20 | FTF |  |
| 7 | FEF |  |
| 6 | FUGF |  |
| 4:3 | RTC |  |
| GMAC registers |  |  |
| ETH MACCR _ | 25 | CSTF |
| 23 | WD |  |
| 22 | JD |  |
| 19:17 | IFG |  |
| 16 | CSD |  |
| 14 | FES |  |
| 13 | ROD |  |
| 12 | LM |  |
| 11 | DM |  |
| 10 | IPCO |  |
| 9 | RD |  |
| 7 | APCS |  |
| 6:5 | BL |  |
| 4 | DC |  |
| 3 | TE |  |
| 2 | RE |  |
| ETH MACFFR _ |  | MAC frame filter register |
| ETH MACHTHR _ | 31:0 | Hash Table High Register |
| ETH MACHTLR _ | 31:0 | Hash Table Low Register |
| ETH MACFCR _ | 31:16 | PT |
| 7 | ZQPD |  |
| 5:4 | PLT |  |
| 3 | UPFD |  |
| 2 | RFCE |  |
| 1 | TFCE |  |
| 0 | FCB/BPA |  |




37

--- Page 28 ---

**STM32F40x and STM32F41x silicon limitations STM32F40x and STM32F41x**

**Table 5. Impacted registers and bits (continued)**

| Register name | Bit number | Bit name |
|---|---|---|
| ETH MACVLANTR _ | 16 | VLANTC |
| 15:0 | VLANTI |  |
| ETH MACRWUFFR _ | - | all remote wakeup registers |
| ETH MACPMTCSR _ | 31 | WFFRPR |
| 9 | GU |  |
| 2 | WFE |  |
| 1 | MPE |  |
| 0 | PD |  |
| ETH MACA0HR _ | - | MAC address 0 high register |
| ETH MACA0LR _ | - | MAC address 0 low register |
| ETH MACA1HR _ | - | MAC address 1 high register |
| ETH MACA1LR _ | - | MAC address 1 low register |
| ETH MACA2HR _ | - | MAC address 2 high register |
| ETH MACA2LR _ | - | MAC address 2 low register |
| ETH MACA3HR _ | - | MAC address 3 high register |
| ETH MACA3LR _ | - | MAC address 3 low register |
| IEEE 1588 time stamp registers |  |  |
| ETH PTPTSCR _ | 18 | TSPFFMAE |
| 17:16 | TSCNT |  |
| 15 | TSSMRME |  |
| 14 | TSSEME |  |
| 13 | TSSIPV4FE |  |
| 12 | TSSIPV6FE |  |
| 11 | TSSPTPOEFE |  |
| 10 | TSPTPPSV2E |  |
| 9 | TSSSR |  |
| 8 | TSSARFE |  |
| 5 | TSARU |  |
| 3 | TSSTU |  |
| 2 | TSSTI |  |
| 1 | TSFCU |  |
| 0 | TSE |  |




--- Page 29 ---

**STM32F40x and STM32F41x STM32F40x and STM32F41x silicon limitations**

**Workaround**

Two workarounds could be applicable:

• Ensure a delay of four TX_CLK/RX_CLK clock cycles between the successive write

operations to the same register.

• Make several successive write operations without delay, then read the register when all

the operations are complete, and finally reprogram it after a delay of four TX_CLK/RX_CLK clock cycles.


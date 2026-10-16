### 33.8.4 DMA register description

This section defines the bits for each DMA register. Non-32 bit accesses are allowed as long as the address is word-aligned.

**Ethernet DMA bus mode register (ETH_DMABMR)**

Address offset: 0x1000
Reset value: 0x0002 0101

The bus mode register establishes the bus operating modes for the DMA.

| 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0 |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved | MB | AAB | FPM | USP | RDP | BL | PM | PBL | EDFE | DSL | DA | RS |
| rw | rs |  |  |  |  |  |  |  |  |  |  |  |


Bits 31:27 Reserved, must be kept at reset value.

Bit 26 **MB**: Mixed burst
When this bit is set high and the FB bit is low, the AHB master interface starts all bursts of a length greater than 16 with INCR (undefined burst). When this bit is cleared, it reverts to fixed burst transfers (INCRx and SINGLE) for burst lengths of 16 and below.

Bit 25 **AAB**: Address-aligned beats
When this bit is set high and the FB bit equals 1, the AHB interface generates all bursts aligned to the start address LS bits. If the FB bit equals 0, the first burst (accessing the data buffer's start address) is not aligned, but subsequent bursts are aligned to the address.

Bit 24 **FPM**: 4xPBL mode
When set high, this bit multiplies the PBL value programmed (bits [22:17] and bits [13:8]) four times. Thus the DMA transfers data in a maximum of 4, 8, 16, 32, 64 and 128 beats depending on the PBL value.

Bit 23 **USP**: Use separate PBL
When set high, it configures the RxDMA to use the value configured in bits [22:17] as PBL while the PBL value in bits [13:8] is applicable to TxDMA operations only. When this bit is cleared, the PBL value in bits [13:8] is applicable for both DMA engines.

Bits 22:17 **RDP**: Rx DMA PBL
These bits indicate the maximum number of beats to be transferred in one RxDMA transaction. This is the maximum value that is used in a single block read/write operation. The RxDMA always attempts to burst as specified in RDP each time it starts a burst transfer on the host bus. RDP can be programmed with permissible values of 1, 2, 4, 8, 16, and 32. Any other value results in undefined behavior.
These bits are valid and applicable only when USP is set high.

Bit 16 **FB**: Fixed burst
This bit controls whether the AHB Master interface performs fixed burst transfers or not. When set, the AHB uses only SINGLE, INCR4, INCR8 or INCR16 during start of normal burst transfers. When reset, the AHB uses SINGLE and INCR burst transfer operations.

STI
RM0090 Rev 18
1223/1749

--- Page 54 ---

**Ethernet (ETH): media access control (MAC) with DMA controller** RM0090

Bits 15:14 **PM**: Rx Tx priority ratio
**RxDMA requests are given priority over TxDMA requests in the following ratio:**
00: 1:1
01: 2:1
10: 3:1
11: 4:1
This is valid only when the DA bit is cleared.

Bits 13:8 **PBL**: Programmable burst length
These bits indicate the maximum number of beats to be transferred in one DMA transaction. This is the maximum value that is used in a single block read/write operation. The DMA always attempts to burst as specified in PBL each time it starts a burst transfer on the host bus. PBL can be programmed with permissible values of 1, 2, 4, 8, 16, and 32. Any other value results in undefined behavior. When USP is set, this PBL value is applicable for TxDMA transactions only.
The PBL values have the following limitations:
– The maximum number of beats (PBL) possible is limited by the size of the Tx FIFO and Rx FIFO.
– The FIFO has a constraint that the maximum beat supported is half the depth of the FIFO.
– If the PBL is common for both transmit and receive DMA, the minimum Rx FIFO and Tx FIFO depths must be considered.
– Do not program out-of-range PBL values, because the system may not behave properly.

Bit 7 **EDFE**: Enhanced descriptor format enable
When this bit is set, the enhanced descriptor format is enabled and the descriptor size is increased to 32 bytes (8 words). This is required when time stamping is activated (TSE=1, ETH_PTPSCR bit 0) or if IPv4 checksum offload is activated (IPCO=1, ETH_MACCCR bit 10).

Bits 6:2 **DSL**: Descriptor skip length
This bit specifies the number of words to skip between two unchained descriptors. The address skipping starts from the end of current descriptor to the start of next descriptor. When DSL value equals zero, the descriptor table is taken as contiguous by the DMA, in Ring mode.

Bit 1 **DA**: DMA Arbitration
0: Round-robin with Rx:TxDMA priority given in bits [15:14]
1: Rx has priority over Tx

Bit 0 **SR**: Software reset
When this bit is set, the MAC DMA controller resets all MAC Subsystem internal registers and logic. It is cleared automatically after the reset operation has completed in all of the core clock domains. Read a 0 value in this bit before re-programming any register of the core.

**Ethernet DMA transmit poll demand register (ETH_DMATPDR)**

Address offset: 0x1004
Reset value: 0x0000 0000

This register is used by the application to instruct the DMA to poll the transmit descriptor list. The transmit poll demand register enables the Transmit DMA to check whether or not the current descriptor is owned by DMA. The Transmit Poll Demand command is given to wake up the TxDMA if it is in Suspend mode. The TxDMA can go into Suspend mode due to an underflow error in a transmitted frame or due to the unavailability of descriptors owned by

1224/1749 RM0090 Rev 18 STI

--- Page 55 ---

**RM0090**

**Ethernet (ETH): media access control (MAC) with DMA controller**

transmit DMA. You can issue this command anytime and the TxDMA resets it once it starts re-fetching the current descriptor from host memory.

| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| TPD |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| rw_wt |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bits 31:0 **TPD**: Transmit poll demand

When these bits are written with any value, the DMA reads the current descriptor pointed to by the ETH\_DMACHTDR register. If that descriptor is not available (owned by Host), transmission returns to the Suspend state and ETH\_DMASR register bit 2 is asserted. If the descriptor is available, transmission resumes.

**ETHERNET DMA receive poll demand register (ETH\_DMARPDR)**

Address offset: 0x1008

Reset value: 0x0000 0000

This register is used by the application to instruct the DMA to poll the receive descriptor list. The Receive poll demand register enables the receive DMA to check for new descriptors. This command is given to wake up the RxDMA from Suspend state. The RxDMA can go into Suspend state only due to the unavailability of descriptors owned by it.

| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| RPD |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| rw_wt |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bits 31:0 **RPD**: Receive poll demand

When these bits are written with any value, the DMA reads the current descriptor pointed to by the ETH\_DMACHRDR register. If that descriptor is not available (owned by Host), reception returns to the Suspended state and ETH\_DMASR register bit 7 is not asserted. If the descriptor is available, the Receive DMA returns to active state.

**Ethernet DMA receive descriptor list address register (ETH\_DMARDLAR)**

Address offset: 0x100C

Reset value: 0x0000 0000

The Receive descriptor list address register points to the start of the receive descriptor list. The descriptor lists reside in the STM32F4xx's physical memory space and must be word-aligned. The DMA internally converts it to bus-width aligned address by making the corresponding LS bits low. Writing to the ETH\_DMARDLAR register is permitted only when reception is stopped. When stopped, the ETH\_DMARDLAR register must be written to before the receive Start command is given.

| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| SRL |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


STI

RM0090 Rev 18

1225/1749

--- Page 56 ---

Ethernet (ETH): media access control (MAC) with DMA controller RM0090

Bits 31:0 **SRL**: Start of receive list

This field contains the base address of the first descriptor in the receive descriptor list. The LSB bits [1/2/3:0] for 32/64/128-bit bus width) are internally ignored and taken as all-zero by the DMA. Hence these LSB bits are read only.

**Ethernet DMA transmit descriptor list address register (ETH_DMATDLAR)**

Address offset: 0x1010

Reset value: 0x0000 0000

The Transmit descriptor list address register points to the start of the transmit descriptor list. The descriptor lists reside in the STM32F4xx's physical memory space and must be word-aligned. The DMA internally converts it to bus-width-aligned address by taking the corresponding LSB to low. Writing to the ETH_DMATDLAR register is permitted only when transmission has stopped. Once transmission has stopped, the ETH_DMATDLAR register can be written before the transmission Start command is given.

| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| STL |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bits 31:0 **STL**: Start of transmit list

This field contains the base address of the first descriptor in the transmit descriptor list. The LSB bits [1/2/3:0] for 32/64/128-bit bus width) are internally ignored and taken as all-zero by the DMA. Hence these LSB bits are read-only.

**Ethernet DMA status register (ETH_DMASR)**

Address offset: 0x1014

Reset value: 0x0000 0000

The Status register contains all the status bits that the DMA reports to the application. The ETH_DMASR register is usually read by the software driver during an interrupt service routine or polling. Most of the fields in this register cause the host to be interrupted. The ETH_DMASR register bits are not cleared when read. Writing 1 to (unreserved) bits in ETH_DMASR register[16:0] clears them and writing 0 has no effect. Each field (bits [16:0]) can be masked by masking the appropriate bit in the ETH_DMAIER register.

| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved | TSTS | PMTS | MMCS | Reserved | EBS | TPS | RPS | NIS | AIS | ERS | FBES | Reserved | ETS | RWTS | RPSS | RBUS | RS | TUS | ROS | TJTS | TBUS | TPSS | TS |  |  |  |  |  |  |  |  |
| r | rc-w1 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


1226/1749 RM0090 Rev 18 STI

--- Page 57 ---

RM0090
Ethernet (ETH): media access control (MAC) with DMA controller

Bits 31:30 Reserved, must be kept at reset value.

Bit 29 **TSTS**: Time stamp trigger status
This bit indicates an interrupt event in the MAC core's Time stamp generator block. The software must read the MAC core's status register, clearing its source (bit 9), to reset this bit to 0. When this bit is high an interrupt is generated if enabled.

Bit 28 **PMTS**: PMT status
This bit indicates an event in the MAC core's PMT. The software must read the corresponding registers in the MAC core to get the exact cause of interrupt and clear its source to reset this bit to 0. The interrupt is generated when this bit is high if enabled.

Bit 27 **MMCS**: MMC status
This bit reflects an event in the MMC of the MAC core. The software must read the corresponding registers in the MAC core to get the exact cause of interrupt and clear the source of interrupt to make this bit as 0. The interrupt is generated when this bit is high if enabled.

Bit 26 Reserved, must be kept at reset value.

Bits 25:23 **EBS**: Error bits status
These bits indicate the type of error that caused a bus error (error response on the AHB interface). Valid only with the fatal bus error bit (ETH_DMASR register [13]) set. This field does not generate an interrupt.
Bit 23 1 Error during data transfer by TxDMA
0 Error during data transfer by RxDMA
Bit 24 1 Error during read transfer
0 Error during write transfer
Bit 25 1 Error during descriptor access
0 Error during data buffer access

Bits 22:20 **TPS**: Transmit process state
These bits indicate the Transmit DMA FSM state. This field does not generate an interrupt.
000: Stopped; Reset or Stop Transmit Command issued
001: Running; Fetching transmit transfer descriptor
010: Running; Waiting for status
011: Running; Reading Data from host memory buffer and queuing it to transmit buffer (Tx FIFO)
100, 101: Reserved for future use
110: Suspended; Transmit descriptor unavailable or transmit buffer underflow
111: Running; Closing transmit descriptor

Bits 19:17 **RPS**: Receive process state
These bits indicate the Receive DMA FSM state. This field does not generate an interrupt.
000: Stopped; Reset or Stop Receive Command issued
001: Running; Fetching receive transfer descriptor
010: Reserved for future use
011: Running; Waiting for receive packet
100: Suspended; Receive descriptor unavailable
101: Running; Closing receive descriptor
110: Reserved for future use
111: Running; Transferring the receive packet data from receive buffer to host memory

ST
RM0090 Rev 18
1227/1749

--- Page 58 ---

**Ethernet (ETH): media access control (MAC) with DMA controller** RM0090

**Bit 16 NIS:** Normal interrupt summary
The normal interrupt summary bit value is the logical OR of the following when the corresponding interrupt bits are enabled in the ETH_DMAIER register:
- ETH_DMASR [0]: Transmit interrupt
- ETH_DMASR [2]: Transmit buffer unavailable
- ETH_DMASR [6]: Receive interrupt
- ETH_DMASR [14]: Early receive interrupt
Only unmasked bits affect the normal interrupt summary bit.
This is a sticky bit and it must be cleared (by writing a 1 to this bit) each time a corresponding bit that causes NIS to be set is cleared.

**Bit 15 AIS:** Abnormal interrupt summary
The abnormal interrupt summary bit value is the logical OR of the following when the corresponding interrupt bits are enabled in the ETH_DMAIER register:
- ETH_DMASR [1]: Transmit process stopped
- ETH_DMASR [3]: Transmit jabber timeout
- ETH_DMASR [4]: Receive FIFO overflow
- ETH_DMASR [5]: Transmit underflow
- ETH_DMASR [7]: Receive buffer unavailable
- ETH_DMASR [8]: Receive process stopped
- ETH_DMASR [9]: Receive watchdog timeout
- ETH_DMASR [10]: Early transmit interrupt
- ETH_DMASR [13]: Fatal bus error
Only unmasked bits affect the abnormal interrupt summary bit.
This is a sticky bit and it must be cleared each time a corresponding bit that causes AIS to be set is cleared.

**Bit 14 ERS:** Early receive status
This bit indicates that the DMA had filled the first data buffer of the packet. Receive Interrupt ETH_DMASR [6] automatically clears this bit.

**Bit 13 FBES:** Fatal bus error status
This bit indicates that a bus error occurred, as detailed in [25:23]. When this bit is set, the corresponding DMA engine disables all its bus accesses.

**Bits 12:11** Reserved, must be kept at reset value.

**Bit 10 ETS:** Early transmit status
This bit indicates that the frame to be transmitted was fully transferred to the Transmit FIFO.

**Bit 9 RWTS:** Receive watchdog timeout status
This bit is asserted when a frame with a length greater than 2 048 bytes is received.

**Bit 8 RPSS:** Receive process stopped status
This bit is asserted when the receive process enters the Stopped state.

**Bit 7 RBUS:** Receive buffer unavailable status
This bit indicates that the next descriptor in the receive list is owned by the host and cannot be acquired by the DMA. Receive process is suspended. To resume processing receive descriptors, the host should change the ownership of the descriptor and issue a Receive Poll Demand command. If no Receive Poll Demand is issued, receive process resumes when the next recognized incoming frame is received. ETH_DMASR [7] is set only when the previous receive descriptor was owned by the DMA.

1228/1749 RM0090 Rev 18 ST

--- Page 59 ---

RM0090
Ethernet (ETH): media access control (MAC) with DMA controller

Bit 6 **RS**: Receive status
This bit indicates the completion of the frame reception. Specific frame status information has been posted in the descriptor. Reception remains in the Running state.

Bit 5 **TUS**: Transmit underflow status
This bit indicates that the transmit buffer had an underflow during frame transmission. Transmission is suspended and an underflow error TDES0[1] is set.

Bit 4 **ROS**: Receive overflow status
This bit indicates that the receive buffer had an overflow during frame reception. If the partial frame is transferred to the application, the overflow status is set in RDES0[11].

Bit 3 **TJTS**: Transmit jabber timeout status
This bit indicates that the transmit jabber timer expired, meaning that the transmitter had been excessively active. The transmission process is aborted and placed in the Stopped state. This causes the transmit jabber timeout TDES0[14] flag to be asserted.

Bit 2 **TBUS**: Transmit buffer unavailable status
This bit indicates that the next descriptor in the transmit list is owned by the host and cannot be acquired by the DMA. Transmission is suspended. Bits [22:20] explain the transmit process state transitions. To resume processing transmit descriptors, the host should change the ownership of the bit of the descriptor and then issue a Transmit Poll Demand command.

Bit 1 **TPSS**: Transmit process stopped status
This bit is set when the transmission is stopped.

Bit 0 **TS**: Transmit status
This bit indicates that frame transmission is finished and TDES1[31] is set in the first descriptor.

Ethernet DMA operation mode register (ETH_DMAOMR)
Address offset: 0x1018
Reset value: 0x0000 0000

The operation mode register establishes the Transmit and Receive operating modes and commands. The ETH_DMAOMR register should be the last CSR to be written as part of DMA initialization.

| Reserved | DTCEFD | RSF | DFRF | Reserved | TSF | FTF | Reserved | TTC | ST | Reserved | FEF | FUGF | Reserved | RTC | OSF | SR | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |


Bits 31:27 Reserved, must be kept at reset value.

Bit 26 **DTCEFD**: Dropping of TCP/IP checksum error frames disable
When this bit is set, the core does not drop frames that only have errors detected by the receive checksum offload engine. Such frames do not have any errors (including FCS error) in the Ethernet frame received by the MAC but have errors in the encapsulated payload only. When this bit is cleared, all error frames are dropped if the FEF bit is reset.

ST
RM0090 Rev 18
1229/1749

--- Page 60 ---

**Ethernet (ETH): media access control (MAC) with DMA controller**

RM0090

**Bit 25 RSF: Receive store and forward**
When this bit is set, a frame is read from the Rx FIFO after the complete frame has been written to it, ignoring RTC bits. When this bit is cleared, the Rx FIFO operates in Cut-through mode, subject to the threshold specified by the RTC bits.

**Bit 24 DFRF: Disable flushing of received frames**
When this bit is set, the RxDMA does not flush any frames due to the unavailability of receive descriptors/buffers as it does normally when this bit is cleared (see *Receive process suspended*).

**Bits 23:22** Reserved, must be kept at reset value.

**Bit 21 TSF: Transmit store and forward**
When this bit is set, transmission starts when a full frame resides in the Transmit FIFO.
When this bit is set, the TTC values specified by the ETH_DMAOMR register bits [16:14] are ignored.
When this bit is cleared, the TTC values specified by the ETH_DMAOMR register bits [16:14] are taken into account.
This bit should be changed only when transmission is stopped.

**Bit 20 FTF: Flush transmit FIFO**
When this bit is set, the transmit FIFO controller logic is reset to its default values and thus all data in the Tx FIFO are lost/flushed. This bit is cleared internally when the flushing operation is complete. The Operation mode register should not be written to until this bit is cleared.

**Bits 19:17** Reserved, must be kept at reset value.

**Bits 16:14 TTC: Transmit threshold control**
These three bits control the threshold level of the Transmit FIFO. Transmission starts when the frame size within the Transmit FIFO is larger than the threshold. In addition, full frames with a length less than the threshold are also transmitted. These bits are used only when the TSF bit (Bit 21) is cleared.
000: 64
001: 128
010: 192
011: 256
100: 40
101: 32
110: 24
111: 16

**Bit 13 ST: Start/stop transmission**
When this bit is set, transmission is placed in the Running state, and the DMA checks the transmit list at the current position for a frame to be transmitted. Descriptor acquisition is attempted either from the current position in the list, which is the transmit list base address set by the ETH_DMATDLAR register, or from the position retained when transmission was stopped previously. If the current descriptor is not owned by the DMA, transmission enters the Suspended state and the transmit buffer unavailable bit (ETH_DMASR [2]) is set. The Start Transmission command is effective only when transmission is stopped. If the command is issued before setting the DMA ETH_DMATDLAR register, the DMA behavior is unpredictable.

When this bit is cleared, the transmission process is placed in the Stopped state after completing the transmission of the current frame. The next descriptor position in the transmit list is saved, and becomes the current position when transmission is restarted. The Stop Transmission command is effective only when the transmission of the current frame is complete or when the transmission is in the Suspended state.

1230/1749
RM0090 Rev 18

--- Page 61 ---

RM0090
Ethernet (ETH): media access control (MAC) with DMA controller

Bits 12:8 Reserved, must be kept at reset value.

Bit 7 **FEF**: Forward error frames
When this bit is set, all frames except runt error frames are forwarded to the DMA.
When this bit is cleared, the Rx FIFO drops frames with error status (CRC error, collision error, giant frame, watchdog timeout, overflow). However, if the frame's start byte (write) pointer is already transferred to the read controller side (in Threshold mode), then the frames are not dropped. The Rx FIFO drops the error frames if that frame's start byte is not transferred (output) on the ARI bus.

Bit 6 **FUGF**: Forward undersized good frames
When this bit is set, the Rx FIFO forwards undersized frames (frames with no error and length less than 64 bytes) including pad-bytes and CRC).
When this bit is cleared, the Rx FIFO drops all frames of less than 64 bytes, unless such a frame has already been transferred due to lower value of receive threshold (e.g., RTC = 01).

Bit 5 Reserved, must be kept at reset value.

Bits 4:3 **RTC**: Receive threshold control
These two bits control the threshold level of the Receive FIFO. Transfer (request) to DMA starts when the frame size within the Receive FIFO is larger than the threshold. In addition, full frames with a length less than the threshold are transferred automatically.
*Note:* Note that value of 11 is not applicable if the configured Receive FIFO size is 128 bytes.
*Note:* These bits are valid only when the RSF bit is zero, and are ignored when the RSF bit is set to 1.

00: 64
01: 32
10: 96
11: 128

Bit 2 **OSF**: Operate on second frame
When this bit is set, this bit instructs the DMA to process a second frame of Transmit data even before status for first frame is obtained.

Bit 1 **SR**: Start/stop receive
When this bit is set, the receive process is placed in the Running state. The DMA attempts to acquire the descriptor from the receive list and processes incoming frames. Descriptor acquisition is attempted from the current position in the list, which is the address set by the DMA ETH_DMARDLAR register or the position retained when the receive process was previously stopped. If no descriptor is owned by the DMA, reception is suspended and the receive buffer unavailable bit (ETH_DMASR [7]) is set. The Start Receive command is effective only when reception has stopped. If the command was issued before setting the DMA ETH_DMARDLAR register, the DMA behavior is unpredictable.
When this bit is cleared, RxDMA operation is stopped after the transfer of the current frame. The next descriptor position in the receive list is saved and becomes the current position when the receive process is restarted. The Stop Receive command is effective only when the Receive process is in either the Running (waiting for receive packet) or the Suspended state.

Bit 0 Reserved, must be kept at reset value.

ST
RM0090 Rev 18
1231/1749

--- Page 62 ---

**Ethernet (ETH): media access control (MAC) with DMA controller**

RM0090

**Ethernet DMA interrupt enable register (ETH_DMAIER)**

Address offset: 0x101C
Reset value: 0x0000 0000

The Interrupt enable register enables the interrupts reported by ETH_DMASR. Setting a bit to 1 enables a corresponding interrupt. After a hardware or software reset, all interrupts are disabled.

| Reserved | ETIE | RWTIE | RPSIE | RBUIE | RIE | TUIE | ROIE | TJTIE | TBUIE | TPSIE | TIE |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |


Bits 31:17 Reserved, must be kept at reset value.

**Bit 16 NISE**: Normal interrupt summary enable
When this bit is set, a normal interrupt is enabled. When this bit is cleared, a normal interrupt is disabled. This bit enables the following bits:
- ETH_DMASR [0]: Transmit Interrupt
- ETH_DMASR [2]: Transmit buffer unavailable
- ETH_DMASR [6]: Receive interrupt
- ETH_DMASR [14]: Early receive interrupt

**Bit 15 AISE**: Abnormal interrupt summary enable
When this bit is set, an abnormal interrupt is enabled. When this bit is cleared, an abnormal interrupt is disabled. This bit enables the following bits:
- ETH_DMASR [1]: Transmit process stopped
- ETH_DMASR [3]: Transmit jabber timeout
- ETH_DMASR [4]: Receive overflow
- ETH_DMASR [5]: Transmit underflow
- ETH_DMASR [7]: Receive buffer unavailable
- ETH_DMASR [8]: Receive process stopped
- ETH_DMASR [9]: Receive watchdog timeout
- ETH_DMASR [10]: Early transmit interrupt
- ETH_DMASR [13]: Fatal bus error

**Bit 14 ERIE**: Early receive interrupt enable
When this bit is set with the normal interrupt summary enable bit (ETH_DMAIER register[16]), the early receive interrupt is enabled.
When this bit is cleared, the early receive interrupt is disabled.

**Bit 13 FBEIE**: Fatal bus error interrupt enable
When this bit is set with the abnormal interrupt summary enable bit (ETH_DMAIER register[15]), the fatal bus error interrupt is enabled.
When this bit is cleared, the fatal bus error enable interrupt is disabled.

Bits 12:11 Reserved, must be kept at reset value.

**Bit 10 ETIE**: Early transmit interrupt enable
When this bit is set with the abnormal interrupt summary enable bit (ETH_DMAIER register [15]), the early transmit interrupt is enabled.
When this bit is cleared, the early transmit interrupt is disabled.

1232/1749
RM0090 Rev 18

--- Page 63 ---

RM0090
Ethernet (ETH): media access control (MAC) with DMA controller

**Bit 9 RWTIE:** receive watchdog timeout interrupt enable
When this bit is set with the abnormal interrupt summary enable bit (ETH_DMAIER register[15]), the receive watchdog timeout interrupt is enabled.
When this bit is cleared, the receive watchdog timeout interrupt is disabled.

**Bit 8 RPSIE:** Receive process stopped interrupt enable
When this bit is set with the abnormal interrupt summary enable bit (ETH_DMAIER register[15]), the receive stopped interrupt is enabled. When this bit is cleared, the receive stopped interrupt is disabled.

**Bit 7 RBUIE:** Receive buffer unavailable interrupt enable
When this bit is set with the abnormal interrupt summary enable bit (ETH_DMAIER register[15]), the receive buffer unavailable interrupt is enabled.
When this bit is cleared, the receive buffer unavailable interrupt is disabled.

**Bit 6 RIE:** Receive interrupt enable
When this bit is set with the normal interrupt summary enable bit (ETH_DMAIER register[16]), the receive interrupt is enabled.
When this bit is cleared, the receive interrupt is disabled.

**Bit 5 TUIE:** Underflow interrupt enable
When this bit is set with the abnormal interrupt summary enable bit (ETH_DMAIER register[15]), the transmit underflow interrupt is enabled.
When this bit is cleared, the underflow interrupt is disabled.

**Bit 4 ROIE:** Overflow interrupt enable
When this bit is set with the abnormal interrupt summary enable bit (ETH_DMAIER register[15]), the receive overflow interrupt is enabled.
When this bit is cleared, the overflow interrupt is disabled.

**Bit 3 TJTIE:** Transmit jabber timeout interrupt enable
When this bit is set with the abnormal interrupt summary enable bit (ETH_DMAIER register[15]), the transmit jabber timeout interrupt is enabled.
When this bit is cleared, the transmit jabber timeout interrupt is disabled.

**Bit 2 TBUIE:** Transmit buffer unavailable interrupt enable
When this bit is set with the normal interrupt summary enable bit (ETH_DMAIER register[16]), the transmit buffer unavailable interrupt is enabled.
When this bit is cleared, the transmit buffer unavailable interrupt is disabled.

**Bit 1 TPSIE:** Transmit process stopped interrupt enable
When this bit is set with the abnormal interrupt summary enable bit (ETH_DMAIER register[15]), the transmission stopped interrupt is enabled.
When this bit is cleared, the transmission stopped interrupt is disabled.

**Bit 0 TIE:** Transmit interrupt enable
When this bit is set with the normal interrupt summary enable bit (ETH_DMAIER register[16]), the transmit interrupt is enabled.
When this bit is cleared, the transmit interrupt is disabled.

The Ethernet interrupt is generated only when the TSTS or PMTS bits of the DMA Status register is asserted with their corresponding interrupt are unmasked, or when the NIS/AIS Status bit is asserted and the corresponding Interrupt Enable bits (NISE/AISE) are enabled.

ST
RM0090 Rev 18
1233/1749

--- Page 64 ---

**Ethernet (ETH): media access control (MAC) with DMA controller**

RM0090

**Ethernet DMA missed frame and buffer overflow counter register (ETH_DMAMFBOCR)**

Address offset: 0x1020
Reset value: 0x0000 0000

The DMA maintains two counters to track the number of missed frames during reception. This register reports the current value of the counter. The counter is used for diagnostic purposes. Bits [15:0] indicate missed frames due to the STM32F4xx buffer being unavailable (no receive descriptor was available). Bits [27:17] indicate missed frames due to Rx FIFO overflow conditions and runt frames (good frames of less than 64 bytes).

| Reserved | OFOC | MFA | OMFC | MFC |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
| rc_r |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bits 31:29 Reserved, must be kept at reset value.
Bit 28 **OFOC**: Overflow bit for FIFO overflow counter
Bits 27:17 **MFA**: Missed frames by the application
Indicates the number of frames missed by the application
Bit 16 **OMFC**: Overflow bit for missed frame counter
Bits 15:0 **MFC**: Missed frames by the controller
Indicates the number of frames missed by the Controller due to the host receive buffer being unavailable. This counter is incremented each time the DMA discards an incoming frame.

**Ethernet DMA receive status watchdog timer register (ETH_DMARSWTR)**

Address offset: 0x1024
Reset value: 0x0000 0000

This register, when written with a non-zero value, enables the watchdog timer for the receive status (RS, ETH_DMASR[6]).

| Reserved | RSWTC |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bits 31:8 Reserved, must be kept at reset value.
Bits 7:0 **RSWTC**: Receive status (RS) watchdog timer count
Indicates the number of HCLK clock cycles multiplied by 256 for which the watchdog timer is set. The watchdog timer gets triggered with the programmed value after the RxDMA completes the transfer of a frame for which the RS status bit is not set due to the setting of RDES1[31] in the corresponding descriptor. When the watchdog timer runs out, the RS bit is set and the timer is stopped. The watchdog timer is reset when the RS bit is set high due to automatic setting of RS as per RDES1[31] of any received frame.

1234/1749
RM0090 Rev 18
ST

--- Page 65 ---

RM0090
Ethernet (ETH): media access control (MAC) with DMA controller

**Ethernet DMA current host transmit descriptor register (ETH_DMACHTDR)**

Address offset: 0x1048
Reset value: 0x0000 0000

The Current host transmit descriptor register points to the start address of the current transmit descriptor read by the DMA.

| HTDAP |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
| r |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bits 31:0 **HTDAP**: Host transmit descriptor address pointer
Cleared . Pointer updated by DMA during operation.

**Ethernet DMA current host receive descriptor register (ETH_DMACHRDR)**

Address offset: 0x104C
Reset value: 0x0000 0000

The Current host receive descriptor register points to the start address of the current receive descriptor read by the DMA.

| HRDAP |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
| r |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bits 31:0 **HRDAP**: Host receive descriptor address pointer
Cleared On Reset. Pointer updated by DMA during operation.

**Ethernet DMA current host transmit buffer address register (ETH_DMACHTBAR)**

Address offset: 0x1050
Reset value: 0x0000 0000

The Current host transmit buffer address register points to the current transmit buffer address being read by the DMA.

| HTBAP |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
| r |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bits 31:0 **HTBAP**: Host transmit buffer address pointer
Cleared On Reset. Pointer updated by DMA during operation.

ST
RM0090 Rev 18
1235/1749

--- Page 66 ---

Ethernet (ETH): media access control (MAC) with DMA controller
RM0090

**Ethernet DMA current host receive buffer address register (ETH_DMACHRBAR)**

Address offset: 0x1054
Reset value: 0x0000 0000

The current host receive buffer address register points to the current receive buffer address being read by the DMA.

31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0
HRBAP
r r r r r r r r r r r r r r r r r r r r r r r r r r r r r r r r

Bits 31:0 **HRBAP**: Host receive buffer address pointer
Cleared On Reset. Pointer updated by DMA during operation.


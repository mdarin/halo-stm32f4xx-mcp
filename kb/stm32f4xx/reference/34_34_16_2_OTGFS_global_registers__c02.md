> [RM0090 ch.34 §34.16.2] OTG_FS global registers


### OTG_FS AHB configuration register (OTG_FS_GAHBCFG)

Address offset: 0x008
Reset value: 0x0000 0000

This register can be used to configure the core after power-on or a change in mode. This register mainly contains AHB system-related configuration parameters. Do not change this register after the initial programming. The application must program this register before starting any transactions on either the AHB or the USB.

| 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 15 14 13 12 11 10 9 | 8 | 7 | 6 5 4 3 2 1 0 |  |
|---|---|---|---|---|
| Reserved | PTXFELVLrw | TXFELVLrw | Reserved | GINTMSKrw |


Bits 31:9 Reserved, must be kept at reset value.

**Bit 8 PTXFELVL:** Periodic TxFIFO empty level
Indicates when the periodic TxFIFO empty interrupt bit in the OTG_FS_GINTSTS register (PTXFE bit in OTG_FS_GINTSTS) is triggered.
0: PTXFE (in OTG_FS_GINTSTS) interrupt indicates that the Periodic TxFIFO is half empty
1: PTXFE (in OTG_FS_GINTSTS) interrupt indicates that the Periodic TxFIFO is completely empty
*Note: Only accessible in host mode.*

**Bit 7 TXFELVL:** TxFIFO empty level
In device mode, this bit indicates when IN endpoint Transmit FIFO empty interrupt (TXFE in OTG_FS_DIEPINTx) is triggered.
0: the TXFE (in OTG_FS_DIEPINTx) interrupt indicates that the IN Endpoint TxFIFO is half empty
1: the TXFE (in OTG_FS_DIEPINTx) interrupt indicates that the IN Endpoint TxFIFO is completely empty
In host mode, this bit indicates when the nonperiodic Tx FIFO empty interrupt (NPTXFE bit in OTG_FS_GINTSTS) is triggered:
0: the NPTXFE (in OTG_FS_GINTSTS) interrupt indicates that the nonperiodic Tx FIFO is half empty
1: the NPTXFE (in OTG_FS_GINTSTS) interrupt indicates that the nonperiodic Tx FIFO is completely empty

Bits 6:1 Reserved, must be kept at reset value.

**Bit 0 GINTMSK:** Global interrupt mask
The application uses this bit to mask or unmask the interrupt line assertion to itself. Irrespective of this bit's setting, the interrupt status registers are updated by the core.
0: Mask the interrupt assertion to the application.
1: Unmask the interrupt assertion to the application.
*Note: Accessible in both device and host modes.*

1274/1749
RM0090 Rev 18

--- Page 105 ---

RM0090
USB on-the-go full-speed (OTG_FS)

**OTG_FS USB configuration register (OTG_FS_GUSBCFG)**

Address offset: 0x00C
Reset value: 0x0000 1440

This register can be used to configure the core after power-on or a changing to host mode or device mode. It contains USB and USB-PHY related configuration parameters. The application must program this register before starting any transactions on either the AHB or the USB. Do not make changes to this register after the initial programming.

| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| CTXPKT | FDMOD | FHMOD | Reserved | TRDT | HNPCAP | SRPCAP | Res. | PHySEL | Reserved | TOCAL |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | rw |  | r |  |  |  | rw |  |  |  |  |  |  |  |  |


Bit 31 **CTXPKT**: Corrupt Tx packet
This bit is for debug purposes only. Never set this bit to 1.
*Note: Accessible in both device and host modes.*

Bit 30 **FDMOD**: Force device mode
Writing a 1 to this bit forces the core to device mode irrespective of the OTG_FS_ID input pin.
0: Normal mode
1: Force device mode
After setting the force bit, the application must wait at least 25 ms before the change takes effect.
*Note: Accessible in both device and host modes.*

Bit 29 **FHMOD**: Force host mode
Writing a 1 to this bit forces the core to host mode irrespective of the OTG_FS_ID input pin.
0: Normal mode
1: Force host mode
After setting the force bit, the application must wait at least 25 ms before the change takes effect.
*Note: Accessible in both device and host modes.*

Bits 28:14 Reserved, must be kept at reset value.

Bits 13:10 **TRDT**: USB turnaround time
These bits allow setting the turnaround time in PHY clocks. They must be configured according to Table 203: TRDT values, depending on the application AHB frequency. Higher TRDT values allow stretching the USB response time to IN tokens in order to compensate for longer AHB read access latency to the Data FIFO.
*Note: Only accessible in device mode.*

Bit 9 **HNCAP**: HNP-capable
The application uses this bit to control the OTG_FS controller's HNP capabilities.
0: HNP capability is not enabled.
1: HNP capability is enabled.
*Note: Accessible in both device and host modes.*

STI
RM0090 Rev 18
1275/1749

--- Page 106 ---

**USB on-the-go full-speed (OTG_FS)**
RM0090

**Bit 8 SRPCAP**: SRP-capable
The application uses this bit to control the OTG_FS controller’s SRP capabilities. If the core operates as a non-SRP-capable B-device, it cannot request the connected A-device (host) to activate VBUS and start a session.
0: SRP capability is not enabled.
1: SRP capability is enabled.
*Note: Accessible in both device and host modes.*

**Bit 7** Reserved, must be kept at reset value.

**Bit 6 PHYSEL**: Full speed serial transceiver select
This bit is always 1 with read-only access.

**Bits 5:3** Reserved, must be kept at reset value.

**Bits 2:0 TOCAL**: FS timeout calibration
The number of PHY clocks that the application programs in this field is added to the full-speed interpacket timeout duration in the core to account for any additional delays introduced by the PHY. This can be required, because the delay introduced by the PHY in generating the line state condition can vary from one PHY to another.
The USB standard timeout value for full-speed operation is 16 to 18 (inclusive) bit times. The application must program this field based on the speed of enumeration. The number of bit times added per PHY clock is 0.25 bit times.

**Table 203. TRDT values**

| AHB frequency range (MHz) | TRDT minimum value |  |
|---|---|---|
| Min. | Max |  |
| 14.2 | 15 | 0xF |
| 15 | 16 | 0xE |
| 16 | 17.2 | 0xD |
| 17.2 | 18.5 | 0xC |
| 18.5 | 20 | 0xB |
| 20 | 21.8 | 0xA |
| 21.8 | 24 | 0x9 |
| 24 | 27.5 | 0x8 |
| 27.5 | 32 | 0x7 |
| 32 | - | 0x6 |


1276/1749
RM0090 Rev 18
ST

--- Page 107 ---

RM0090
USB on-the-go full-speed (OTG_FS)

**OTG_FS reset register (OTG_FS_GRSTCTL)**

Address offset: 0x010
Reset value: 0x8000 0000

The application uses this register to reset various hardware features inside the core.

| AHBDL | Reserved | TXFNUM | TXFFLSH | RXFFLSH | Reserved | FCRST | HSRST | CSRST |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
| r |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | rw | rs |  | rs |  |  |


**Bit 31 AHBDL**: AHB master idle
Indicates that the AHB master state machine is in the Idle condition.
*Note: Accessible in both device and host modes.*

**Bits 30:11** Reserved, must be kept at reset value.

**Bits 10:6 TXFNUM**: TxFIFO number
This is the FIFO number that must be flushed using the TxFIFO Flush bit. This field must not be changed until the core clears the TxFIFO Flush bit.
00000:
- Non-periodic TxFIFO flush in host mode
- Tx FIFO 0 flush in device mode
00001:
- Periodic TxFIFO flush in host mode
- TXFIFO 1 flush in device mode
00010: TxFIFO 2 flush in device mode
...
00101: TxFIFO 15 flush in device mode
10000: Flush all the transmit FIFOs in device or host mode.
*Note: Accessible in both device and host modes.*

**Bit 5 TXFFLSH**: TxFIFO flush
This bit selectively flushes a single or all transmit FIFOs, but cannot do so if the core is in the midst of a transaction.
The application must write this bit only after checking that the core is neither writing to the TxFIFO nor reading from the TxFIFO. Verify using these registers:
Read—NAK Effective Interrupt ensures the core is not reading from the FIFO
Write—AHBDL bit in OTG_FS_GRSTCTL ensures the core is not writing anything to the FIFO.
*Note: Accessible in both device and host modes.*

**Bit 4 RXFFLSH**: RxFIFO flush
The application can flush the entire RxFIFO using this bit, but must first ensure that the core is not in the middle of a transaction.
The application must only write to this bit after checking that the core is neither reading from the RxFIFO nor writing to the RxFIFO.
The application must wait until the bit is cleared before performing any other operations. This bit requires 8 clocks (slowest of PHY or AHB clock) to clear.
*Note: Accessible in both device and host modes.*

**Bit 3** Reserved, must be kept at reset value.

STI
RM0090 Rev 18
1277/1749

--- Page 108 ---

**USB on-the-go full-speed (OTG_FS)**
**RM0090**

**Bit 2 FCRST:** Host frame counter reset
The application writes this bit to reset the frame number counter inside the core. When the frame counter is reset, the subsequent SOF sent out by the core has a frame number of 0.
*Note: Only accessible in host mode.*

**Bit 1 HSRST:** HCLK soft reset
The application uses this bit to flush the control logic in the AHB Clock domain. Only AHB Clock Domain pipelines are reset.
FIFOs are not flushed with this bit.
All state machines in the AHB clock domain are reset to the Idle state after terminating the transactions on the AHB, following the protocol.
CSR control bits used by the AHB clock domain state machines are cleared.
To clear this interrupt, status mask bits that control the interrupt status and are generated by the AHB clock domain state machine are cleared.
Because interrupt status bits are not cleared, the application can get the status of any core events that occurred after it set this bit.
This is a self-clearing bit that the core clears after all necessary logic is reset in the core. This can take several clocks, depending on the core's current state.
*Note: Accessible in both device and host modes.*

**Bit 0 CSRST:** Core soft reset
Resets the HCLK and PCLK domains as follows:
Clears the interrupts and all the CSR register bits except for the following bits:
– RSTPDMODL bit in OTG_FS_PCGCCTL
– GAYEHCLK bit in OTG_FS_PCGCCTL
– PWRCLMP bit in OTG_FS_PCGCCTL
– STPPCLK bit in OTG_FS_PCGCCTL
– FSLSPCS bit in OTG_FS_HCFG
– DSPD bit in OTG_FS_DCFG
All module state machines (except for the AHB slave unit) are reset to the Idle state, and all the transmit FIFOs and the receive FIFO are flushed.
Any transactions on the AHB Master are terminated as soon as possible, after completing the last data phase of an AHB transfer. Any transactions on the USB are terminated immediately.
The application can write to this bit any time it wants to reset the core. This is a self-clearing bit and the core clears this bit after all the necessary logic is reset in the core, which can take several clocks, depending on the current state of the core. Once this bit has been cleared, the software must wait at least 3 PHY clocks before accessing the PHY domain (synchronization delay). The software must also check that bit 31 in this register is set to 1 (AHB Master is Idle) before starting any operation.
Typically, the software reset is used during software development and also when you dynamically change the PHY selection bits in the above listed USB configuration registers.
When you change the PHY, the corresponding clock for the PHY is selected and used in the PHY domain. Once a new clock is selected, the PHY domain has to be reset for proper operation.
*Note: Accessible in both device and host modes.*

1278/1749
RM0090 Rev 18
ST

--- Page 109 ---

RM0090
USB on-the-go full-speed (OTG_FS)

**OTG_FS core interrupt register (OTG_FS_GINTSTS)**

Address offset: 0x014
Reset value: 0x0400 0020

This register interrupts the application for system-level events in the current mode (device mode or host mode).

Some of the bits in this register are valid only in host mode, while others are valid in device mode only. This register also indicates the current mode. To clear the interrupt status bits of the rc_w1 type, the application must write 1 into the bit.

The FIFO status interrupts are read-only; once software reads from or writes to the FIFO while servicing these interrupts, FIFO interrupt conditions are cleared automatically.

The application must clear the OTG_FS_GINTSTS register at initialization before unmasking the interrupt bit to avoid any interrupts generated prior to initialization.

| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| WKUINT | SRQINT | DISCINT | CIDSCHG | Reserved | PTXFE | HCINT | HPRTINT | Reserved | IPXFR/COMPISOUT | IISOXFR | OEPINT | IEPINT | Reserved | EQPF | ISOODRP | ENUMDNE | USBRST | USBSUSP | ESUSP | Reserved | GONAKEFF | GINAKEFF | NPTXFE | RXFLVL | SOF | OTGINT | MMIS | CMOD |  |  |  |
| rc_w1 |  |  |  |  | r | Res. | rc_w1 | r |  |  |  | rc_w1 |  | r | rc_w1 | r | rc_w1 | r |  |  |  |  |  |  |  |  |  |  |  |  |  |


**Bit 31 WKUPINT**: Resume/remote wakeup detected interrupt
In device mode, this interrupt is asserted when a resume is detected on the USB. In host mode, this interrupt is asserted when a remote wakeup is detected on the USB.
*Note: Accessible in both device and host modes.*

**Bit 30 SRQINT**: Session request/new session detected interrupt
In host mode, this interrupt is asserted when a session request is detected from the device. In device mode, this interrupt is asserted when VBUS is in the valid range for a B-peripheral device. Accessible in both device and host modes.

**Bit 29 DISCINT**: Disconnect detected interrupt
Asserted when a device disconnect is detected.
*Note: Only accessible in host mode.*

**Bit 28 CIDSCHG**: Connector ID status change
The core sets this bit when there is a change in connector ID status.
*Note: Accessible in both device and host modes.*

**Bit 27** Reserved, must be kept at reset value.

**Bit 26 PTXFE**: Periodic TxFIFO empty
Asserted when the periodic transmit FIFO is either half or completely empty and there is space for at least one entry to be written in the periodic request queue. The half or completely empty status is determined by the periodic TxFIFO empty level bit in the OTG_FS_GAHBCFG register (PTXFELVL bit in OTG_FS_GAHBCFG).
*Note: Only accessible in host mode.*

ST
RM0090 Rev 18
1279/1749

--- Page 110 ---

USB on-the-go full-speed (OTG_FS)

RM0090

Bit 25 **HCINT**: Host channels interrupt
The core sets this bit to indicate that an interrupt is pending on one of the channels of the core (in host mode). The application must read the OTG_FS_HAINT register to determine the exact number of the channel on which the interrupt occurred, and then read the corresponding OTG_FS_HCINTx register to determine the exact cause of the interrupt. The application must clear the appropriate status bit in the OTG_FS_HCINTx register to clear this bit.
*Note: Only accessible in host mode.*

Bit 24 **HPRTINT**: Host port interrupt
The core sets this bit to indicate a change in port status of one of the OTG_FS controller ports in host mode. The application must read the OTG_FS_HPRT register to determine the exact event that caused this interrupt. The application must clear the appropriate status bit in the OTG_FS_HPRT register to clear this bit.
*Note: Only accessible in host mode.*

Bits 23:22 Reserved, must be kept at reset value.

Bit 21 **IPXFR**: Incomplete periodic transfer
In host mode, the core sets this interrupt bit when there are incomplete periodic transactions still pending, which are scheduled for the current frame.
**INCOMPISOOUT**: Incomplete isochronous OUT transfer
In device mode, the core sets this interrupt to indicate that there is at least one isochronous OUT endpoint on which the transfer is not completed in the current frame. This interrupt is asserted along with the End of periodic frame interrupt (EOPF) bit in this register.

Bit 20 **ISOIXFR**: Incomplete isochronous IN transfer
The core sets this interrupt to indicate that there is at least one isochronous IN endpoint on which the transfer is not completed in the current frame. This interrupt is asserted along with the End of periodic frame interrupt (EOPF) bit in this register.
*Note: Only accessible in device mode.*

Bit 19 **OEPINT**: OUT endpoint interrupt
The core sets this bit to indicate that an interrupt is pending on one of the OUT endpoints of the core (in device mode). The application must read the OTG_FS_DAINT register to determine the exact number of the OUT endpoint on which the interrupt occurred, and then read the corresponding OTG_FS_DOEPINTx register to determine the exact cause of the interrupt. The application must clear the appropriate status bit in the corresponding OTG_FS_DOEPINTx register to clear this bit.
*Note: Only accessible in device mode.*

Bit 18 **IEPINT**: IN endpoint interrupt
The core sets this bit to indicate that an interrupt is pending on one of the IN endpoints of the core (in device mode). The application must read the OTG_FS_DAINT register to determine the exact number of the IN endpoint on which the interrupt occurred, and then read the corresponding OTG_FS_DIEPINTx register to determine the exact cause of the interrupt. The application must clear the appropriate status bit in the corresponding OTG_FS_DIEPINTx register to clear this bit.
*Note: Only accessible in device mode.*

Bits 17:16 Reserved, must be kept at reset value.

Bit 15 **EOPF**: End of periodic frame interrupt
Indicates that the period specified in the periodic frame interval field of the OTG_FS_DCFG register (PFLV bit in OTG_FS_DCFG) has been reached in the current frame.
*Note: Only accessible in device mode.*

1280/1749
RM0090 Rev 18
ST

--- Page 111 ---

**RM0090**
**USB on-the-go full-speed (OTG_FS)**

**Bit 14 ISOODRP:** Isochronous OUT packet dropped interrupt
The core sets this bit when it fails to write an isochronous OUT packet into the RxFIFO because the RxFIFO does not have enough space to accommodate a maximum size packet for the isochronous OUT endpoint.
*Note: Only accessible in device mode.*

**Bit 13 ENUMDNE:** Enumeration done
The core sets this bit to indicate that speed enumeration is complete. The application must read the OTG_FS_DSTS register to obtain the enumerated speed.
*Note: Only accessible in device mode.*

**Bit 12 USBRST:** USB reset
The core sets this bit to indicate that a reset is detected on the USB.
*Note: Only accessible in device mode.*

**Bit 11 USBSUSP:** USB suspend
The core sets this bit to indicate that a suspend was detected on the USB. The core enters the Suspended state when there is no activity on the data lines for a period of 3 ms.
*Note: Only accessible in device mode.*

**Bit 10 ESUSP:** Early suspend
The core sets this bit to indicate that an Idle state has been detected on the USB for 3 ms.
*Note: Only accessible in device mode.*

**Bits 9:8** Reserved, must be kept at reset value.

**Bit 7 GONAKEFF:** Global OUT NAK effective
Indicates that the Set global OUT NAK bit in the OTG_FS_DCTL register (SGONAK bit in OTG_FS_DCTL), set by the application, has taken effect in the core. This bit can be cleared by writing the Clear global OUT NAK bit in the OTG_FS_DCTL register (CGONAK bit in OTG_FS_DCTL).
*Note: Only accessible in device mode.*

**Bit 6 GINAKEFF:** Global IN non-periodic NAK effective
Indicates that the Set global non-periodic IN NAK bit in the OTG_FS_DCTL register (SGINAK bit in OTG_FS_DCTL), set by the application, has taken effect in the core. That is, the core has sampled the Global IN NAK bit set by the application. This bit can be cleared by clearing the Clear global non-periodic IN NAK bit in the OTG_FS_DCTL register (CGINAK bit in OTG_FS_DCTL).
This interrupt does not necessarily mean that a NAK handshake is sent out on the USB. The STALL bit takes precedence over the NAK bit.
*Note: Only accessible in device mode.*

**Bit 5 NPTXFE:** Non-periodic TxFIFO empty
This interrupt is asserted when the non-periodic TxFIFO is either half or completely empty, and there is space for at least one entry to be written to the non-periodic transmit request queue. The half or completely empty status is determined by the non-periodic TxFIFO empty level bit in the OTG_FS_GAHBCFG register (TXFELVL bit in OTG_FS_GAHBCFG).
*Note: Accessible in host mode only.*

**Bit 4 RXFLVL:** RxFIFO non-empty
Indicates that there is at least one packet pending to be read from the RxFIFO.
*Note: Accessible in both host and device modes.*

ST
RM0090 Rev 18
1281/1749

--- Page 112 ---

**USB on-the-go full-speed (OTG_FS)**
RM0090

**Bit 3 SOF**: Start of frame
In host mode, the core sets this bit to indicate that an SOF (FS), or Keep-Alive (LS) is transmitted on the USB. The application must write a 1 to this bit to clear the interrupt.
In device mode, in the core sets this bit to indicate that an SOF token has been received on the USB. The application can read the Device Status register to get the current frame number. This interrupt is seen only when the core is operating in FS.
*Note: Accessible in both host and device modes.*

**Bit 2 OTGINT**: OTG interrupt
The core sets this bit to indicate an OTG protocol event. The application must read the OTG Interrupt Status (OTG_FS_GOTGINT) register to determine the exact event that caused this interrupt. The application must clear the appropriate status bit in the OTG_FS_GOTGINT register to clear this bit.
*Note: Accessible in both host and device modes.*

**Bit 1 MMIS**: Mode mismatch interrupt
The core sets this bit when the application is trying to access:
- A host mode register, when the core is operating in device mode
- A device mode register, when the core is operating in host mode
The register access is completed on the AHB with an OKAY response, but is ignored by the core internally and does not affect the operation of the core.
*Note: Accessible in both host and device modes.*

**Bit 0 CMOD**: Current mode of operation
Indicates the current mode.
0: Device mode
1: Host mode
*Note: Accessible in both host and device modes.*

1282/1749
RM0090 Rev 18

--- Page 113 ---

RM0090
USB on-the-go full-speed (OTG_FS)

**OTG_FS interrupt mask register (OTG_FS_GINTMSK)**

Address offset: 0x018
Reset value: 0x0000 0000

This register works with the Core interrupt register to interrupt the application. When an interrupt bit is masked, the interrupt associated with that bit is not generated. However, the Core Interrupt (OTG_FS_GINTSTS) register bit corresponding to that interrupt is still set.

| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| WUIM | SRQIM | DISCINT | CIDSCHGM | Reserved | PTXFEM | HCIM | PRTIM | Reserved | IPXFRM/ISOOXFRM | IISOIXFRM | OEPINT | IEPINT | Reserved | EOPFM | ISOODRPM | ENUMDNEM | USBRST | USBSUSPM | ESUSPM | Reserved | GONAKEFFM | GINAKEFFM | NPTXFEM | RXFLVM | SOFM | OTGINT | MMISM | Reserved |  |  |  |
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


**Bit 31 WUIM**: Resume/remote wakeup detected interrupt mask
0: Masked interrupt
1: Unmasked interrupt
*Note: Accessible in both host and device modes.*

**Bit 30 SRQIM**: Session request/new session detected interrupt mask
0: Masked interrupt
1: Unmasked interrupt
*Note: Accessible in both host and device modes.*

**Bit 29 DISCINT**: Disconnect detected interrupt mask
0: Masked interrupt
1: Unmasked interrupt
*Note: Only accessible in device mode.*

**Bit 28 CIDSCHGM**: Connector ID status change mask
0: Masked interrupt
1: Unmasked interrupt
*Note: Accessible in both host and device modes.*

Bit 27 Reserved, must be kept at reset value.

**Bit 26 PTXFEM**: Periodic TxFIFO empty mask
0: Masked interrupt
1: Unmasked interrupt
*Note: Only accessible in host mode.*

**Bit 25 HCIM**: Host channels interrupt mask
0: Masked interrupt
1: Unmasked interrupt
*Note: Only accessible in host mode.*

**Bit 24 PRTIM**: Host port interrupt mask
0: Masked interrupt
1: Unmasked interrupt
*Note: Only accessible in host mode.*

ST
RM0090 Rev 18
1283/1749

--- Page 114 ---

USB on-the-go full-speed (OTG_FS) RM0090

Bits 23:22 Reserved, must be kept at reset value.

Bit 21 **IPXFRM**: Incomplete periodic transfer mask
0: Masked interrupt
1: Unmasked interrupt
*Note: Only accessible in host mode.*

**IISOXFRM**: Incomplete isochronous OUT transfer mask
0: Masked interrupt
1: Unmasked interrupt
*Note: Only accessible in device mode.*

Bit 20 **IISOXIFRM**: Incomplete isochronous IN transfer mask
0: Masked interrupt
1: Unmasked interrupt
*Note: Only accessible in device mode.*

Bit 19 **OEPINT**: OUT endpoints interrupt mask
0: Masked interrupt
1: Unmasked interrupt
*Note: Only accessible in device mode.*

Bit 18 **IEPINT**: IN endpoints interrupt mask
0: Masked interrupt
1: Unmasked interrupt
*Note: Only accessible in device mode.*

Bits 17:16 Reserved, must be kept at reset value.

Bit 15 **EOPFM**: End of periodic frame interrupt mask
0: Masked interrupt
1: Unmasked interrupt
*Note: Only accessible in device mode.*

Bit 14 **ISOODRPM**: Isochronous OUT packet dropped interrupt mask
0: Masked interrupt
1: Unmasked interrupt
*Note: Only accessible in device mode.*

Bit 13 **ENUMDNEM**: Enumeration done mask
0: Masked interrupt
1: Unmasked interrupt
*Note: Only accessible in device mode.*

Bit 12 **USBRST**: USB reset mask
0: Masked interrupt
1: Unmasked interrupt
*Note: Only accessible in device mode.*

Bit 11 **USBSUSPM**: USB suspend mask
0: Masked interrupt
1: Unmasked interrupt
*Note: Only accessible in device mode.*

1284/1749 RM0090 Rev 18 ST

--- Page 115 ---

RM0090
USB on-the-go full-speed (OTG_FS)

Bit 10 **ESUSPM**: Early suspend mask
0: Masked interrupt
1: Unmasked interrupt
*Note: Only accessible in device mode.*

Bits 9:8 Reserved, must be kept at reset value.

Bit 7 **GONAKEFFM**: Global OUT NAK effective mask
0: Masked interrupt
1: Unmasked interrupt
*Note: Only accessible in device mode.*

Bit 6 **GINAKEFFM**: Global non-periodic IN NAK effective mask
0: Masked interrupt
1: Unmasked interrupt
*Note: Only accessible in device mode.*

Bit 5 **NPTXFEM**: Non-periodic TxFIFO empty mask
0: Masked interrupt
1: Unmasked interrupt
*Note: Only accessible in Host mode.*

Bit 4 **RXFLVLM**: Receive FIFO non-empty mask
0: Masked interrupt
1: Unmasked interrupt
*Note: Accessible in both device and host modes.*

Bit 3 **SOFM**: Start of frame mask
0: Masked interrupt
1: Unmasked interrupt
*Note: Accessible in both device and host modes.*

Bit 2 **OTGINT**: OTG interrupt mask
0: Masked interrupt
1: Unmasked interrupt
*Note: Accessible in both device and host modes.*

Bit 1 **MMISM**: Mode mismatch interrupt mask
0: Masked interrupt
1: Unmasked interrupt
*Note: Accessible in both device and host modes.*

Bit 0 Reserved, must be kept at reset value.

ST
RM0090 Rev 18
1285/1749

--- Page 116 ---

USB on-the-go full-speed (OTG_FS)
RM0090

**OTG_FS Receive status debug read/OTG status read and pop registers**
**(OTG_FS_GRXSTSR/OTG_FS_GRXSTSP)**

Address offset for Read: 0x01C
Address offset for Pop: 0x020
Reset value: 0x0000 0000

A read to the Receive status debug read register returns the contents of the top of the Receive FIFO. A read to the Receive status read and pop register additionally pops the top data entry out of the RxFIFO.

The receive status contents must be interpreted differently in host and device modes. The core ignores the receive status pop/read when the receive FIFO is empty and returns a value of 0x0000 0000. The application must only pop the Receive Status FIFO when the Receive FIFO non-empty bit of the Core interrupt register (RXFLVL bit in OTG_FS_GINTSTS) is asserted.

**Host mode**

| 31 30 29 28 27 26 25 24 23 22 21 | 20 19 18 17 | 16 | 15 14 13 12 11 10 | 9 | 8 7 6 5 4 3 | 2 1 0 |
|---|---|---|---|---|---|---|
| Reserved | PKTSTS | DPID | BCNT | CHNUM |  |  |
|  | r |  |  |  |  |  |


Bits 31:21 Reserved, must be kept at reset value.

Bits 20:17 **PKTSTS**: Packet status
Indicates the status of the received packet
0010: IN data packet received
0011: IN transfer completed (triggers an interrupt)
0101: Data toggle error (triggers an interrupt)
0111: Channel halted (triggers an interrupt)
Others: Reserved

Bits 16:15 **DPID**: Data PID
Indicates the Data PID of the received packet
00: DATA0
10: DATA1
01: DATA2
11: MDATA

Bits 14:4 **BCNT**: Byte count
Indicates the byte count of the received IN data packet.

Bits 3:0 **CHNUM**: Channel number
Indicates the channel number to which the current received packet belongs.

1286/1749
RM0090 Rev 18

--- Page 117 ---

RM0090
USB on-the-go full-speed (OTG_FS)

**Device mode**

| Reserved | FRMNUM | PKTSTS | DPID | BCNT | EPNUM |
|---|---|---|---|---|---|
| r |  |  |  |  |  |


Bits 31:25 **Reserved**, must be kept at reset value.

Bits 24:21 **FRMNUM**: Frame number
This is the least significant 4 bits of the frame number in which the packet is received on the USB. This field is supported only when isochronous OUT endpoints are supported.

Bits 20:17 **PKTSTS**: Packet status
Indicates the status of the received packet
0001: Global OUT NAK (triggers an interrupt)
0010: OUT data packet received
0011: OUT transfer completed (triggers an interrupt)
0100: SETUP transaction completed (triggers an interrupt)
0110: SETUP data packet received
Others: Reserved

Bits 16:15 **DPID**: Data PID
Indicates the Data PID of the received OUT data packet
00: DATA0
10: DATA1
01: DATA2
11: MDATA

Bits 14:4 **BCNT**: Byte count
Indicates the byte count of the received data packet.

Bits 3:0 **EPNUM**: Endpoint number
Indicates the endpoint number to which the current received packet belongs.

**OTG_FS Receive FIFO size register (OTG_FS_GRXFSIZ)**

Address offset: 0x024
Reset value: 0x0000 0200

The application can program the RAM size that must be allocated to the RxFIFO.

| Reserved | RXFD |
|---|---|
| rw |  |


Bits 31:16 **Reserved**, must be kept at reset value.

Bits 15:0 **RXFD**: RxFIFO depth
This value is in terms of 32-bit words.
Minimum value is 16
Maximum value is 256
The power-on reset value of this register is specified as the largest Rx data FIFO depth.

STI
RM0090 Rev 18
1287/1749

--- Page 118 ---

USB on-the-go full-speed (OTG_FS)
RM0090

**OTG_FS Host non-periodic transmit FIFO size register (OTG_FS_HNPTXFSIZ)/Endpoint 0 Transmit FIFO size (OTG_FS_DIEPTXF0)**

Address offset: 0x028
Reset value: 0x0000 0200

| 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 | 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0 |
|---|---|
| NPTXFD/TX0FD | NPTXFSA/TX0FSA |
| rw |  |


Host mode

Bits 31:16 **NPTXFD**: Non-periodic TxFIFO depth
This value is in terms of 32-bit words.
Minimum value is 16
Maximum value is 256

Bits 15:0 **NPTXFSA**: Non-periodic transmit RAM start address
This field contains the memory start address for non-periodic transmit FIFO RAM.

Device mode

Bits 31:16 **TX0FD**: Endpoint 0 TxFIFO depth
This value is in terms of 32-bit words.
Minimum value is 16
Maximum value is 256

Bits 15:0 **TX0FSA**: Endpoint 0 transmit RAM start address
This field contains the memory start address for the endpoint 0 transmit FIFO RAM.

**OTG_FS non-periodic transmit FIFO/queue status register (OTG_FS_HNPTXSTS)**

Address offset: 0x02C
Reset value: 0x0008 0200

*Note:* In Device mode, this register is not valid.

This read-only register contains the free space information for the non-periodic TxFIFO and the non-periodic transmit request queue.

| 31 | 30 29 28 27 | 26 25 24 23 | 22 21 20 19 | 18 17 16 15 | 14 13 12 11 | 10 9 8 7 | 6 5 4 3 | 2 1 0 |
|---|---|---|---|---|---|---|---|---|
| Reserved | NPTXQTOP | NPTXSAV |  |  |  |  |  |  |
| r |  |  |  |  |  |  |  |  |


1288/1749
RM0090 Rev 18
STI

--- Page 119 ---

**RM0090**
**USB on-the-go full-speed (OTG_FS)**

Bit 31 Reserved, must be kept at reset value.

Bits 30:24 **NPTXQTOP**: Top of the non-periodic transmit request queue
Entry in the non-periodic Tx request queue that is currently being processed by the MAC.
Bits 30:27: Channel/endpoint number
Bits 26:25:
- 00: IN/OUT token
- 01: Zero-length transmit packet (device IN/host OUT)
- 11: Channel halt command
Bit 24: Terminate (last entry for selected channel/endpoint)

Bits 23:16 **NPTXQSAV**: Non-periodic transmit request queue space available
Indicates the amount of free space available in the non-periodic transmit request queue.
This queue holds both IN and OUT requests in host mode. Device mode has only IN requests.
00: Non-periodic transmit request queue is full
01: 1 location available
10: 2 locations available
bxn: n locations available (0 ≤ n ≤ 8)
Others: Reserved

Bits 15:0 **NPTXFSAV**: Non-periodic TxFIFO space available
Indicates the amount of free space available in the non-periodic TxFIFO.
Values are in terms of 32-bit words.
00: Non-periodic TxFIFO is full
01: 1 word available
10: 2 words available
0xn: n words available (where 0 ≤ n ≤ 256)
Others: Reserved

**OTG_FS general core configuration register (OTG_FS_GCCFG)**
Address offset: 0x038
Reset value: 0x0000 XXXX

| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved | NOVBUSSENSrw | SOFOUTENrw | VBUSBSENrw | VBUSASENrw | Reservedrw | _PWRDWN | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


STI
RM0090 Rev 18
1289/1749

--- Page 120 ---

USB on-the-go full-speed (OTG_FS)

RM0090

Bits 31:22 Reserved, must be kept at reset value.

Bit 21 **NOVBUSSENS**: VBUS sensing disable option
When this bit is set, VBUS is considered internally to be always at VBUS valid level (5 V). This option removes the need for a dedicated VBUS pad, and leave this pad free to be used for other purposes such as a shared functionality. VBUS connection can be remapped on another general purpose input pad and monitored by software.
This option is only suitable for host-only or device-only applications.
0: VBUS sensing available by hardware
1: VBUS sensing not available by hardware.

Bit 20 **SOFOUTEN**: SOF output enable
0: SOF pulse not available on PAD (OTG_FS_SOF)
1: SOF pulse available on PAD (OTG_FS_SOF)

Bit 19 **VBUSBSSEN**: Enable the VBUS sensing “B” device
0: VBUS sensing “B” disabled
1: VBUS sensing “B” enabled

Bit 18 **VBUSASEN**: Enable the VBUS sensing “A” device
0: VBUS sensing “A” disabled
1: VBUS sensing “A” enabled

Bit 17 Reserved, must be kept at reset value.

Bit 16 **PWRDWN**: Power down
Used to activate the transceiver in transmission/reception
0: Power down active
1: Power down deactivated (“Transceiver active”)

Bits 15:0 Reserved, must be kept at reset value.

**OTG_FS core ID register (OTG_FS_CID)**

Address offset: 0x03C
Reset value: 0x0000 1200
This is a register containing the Product ID as reset value.

| PRODUCT_ID |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bits 31:0 **PRODUCT_ID**: Product ID field
Application-programmable ID field.

1290/1749
RM0090 Rev 18

--- Page 121 ---

RM0090
USB on-the-go full-speed (OTG_FS)

> [RM0090 ch.34 §34.16.2] OTG_FS global registers

### OTG_FS Host periodic transmit FIFO size register (OTG_FS_HPTXFSIZ)

### OTG_FS Host periodic transmit FIFO size register (OTG_FS_HPTXFSIZ)

Address offset: 0x100
Reset value: 0x0200 0400

| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| PTXFSIZ | PTXSA |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| r |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bits 31:16 **PTXFD**: Host periodic TxFIFO depth
This value is in terms of 32-bit words.
Minimum value is 16

Bits 15:0 **PTXSA**: Host periodic TxFIFO start address
The power-on reset value of this register is the sum of the largest Rx data FIFO depth and largest non-periodic Tx data FIFO depth.

> [RM0090 ch.34 §34.16.2] OTG_FS global registers

### OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXFx)

### OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXFx)
(x = 1..3, where x is the FIFO_number)

Address offset: 0x104 + 0x04 * (x -1)
Reset value: 0x0200 0200

| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| INEPTXFD | INEPTXSA |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| r |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bits 31:16 **INEPTXFD**: IN endpoint TxFIFO depth
This value is in terms of 32-bit words.
Minimum value is 16
The power-on reset value of this register is specified as the largest IN endpoint FIFO number depth.

Bits 15:0 **INEPTXSA**: IN endpoint FIFOx transmit RAM start address
This field contains the memory start address for IN endpoint transmit FIFOx. The address must be aligned with a 32-bit memory location.

STi
RM0090 Rev 18
1291/1749

--- Page 122 ---

USB on-the-go full-speed (OTG_FS)
RM0090


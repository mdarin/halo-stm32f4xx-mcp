> [RM0090 ch.35 §35.12.2] OTG_HS global registers


### OTG_HS AHB configuration register (OTG_HS_GAHBCFG)

Address offset: 0x008
Reset value: 0x0000 0000

This register can be used to configure the core after power-on or a change in mode. This register mainly contains AHB system-related configuration parameters. Do not change this register after the initial programming. The application must program this register before starting any transactions on either the AHB or the USB.

| 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 15 14 13 12 11 10 9 | 8 | 7 | 6 | 5 | 4 3 2 1 | 0 |  |
|---|---|---|---|---|---|---|---|
| Reserved | PTXFELVLrw | TXFELVLrw | Reserved | DMAENrw | HBSTLENrw | rw | GINTrw |


Bits 31:20 Reserved, must be kept at reset value.

**Bit 8 PTXFELVL: Periodic TxFIFO empty level**
Indicates when the periodic TxFIFO empty interrupt bit in the Core interrupt register (PTXFE bit in OTG_HS_GINTSTS) is triggered.
0: PTXFE (in OTG_HS_GINTSTS) interrupt indicates that the Periodic TxFIFO is half empty
1: PTXFE (in OTG_HS_GINTSTS) interrupt indicates that the Periodic TxFIFO is completely empty
*Note: Only accessible in host mode.*

**Bit 7 TXFELVL: TxFIFO empty level**
In peripheral mode, this bit indicates when the IN endpoint Transmit FIFO empty interrupt (TXFE in OTG_HS_DIEPINTx.) is triggered.
0: TXFE (in OTG_HS_DIEPINTx) interrupt indicates that the IN Endpoint TxFIFO is half empty
1: TXFE (in OTG_HS_DIEPINTx) interrupt indicates that the IN Endpoint TxFIFO is completely empty
*Note: Only accessible in peripheral mode.*

**Bit 6** Reserved, must be kept at reset value.

**Bits5 DMAEN: DMA enable**
0: The core operates in slave mode
1: The core operates in DMA mode

**Bits 4:1 HBSTLEN: Burst length/type**
0000 Single
0001 INCR
0011 INCR4
0101 INCR8
0111 INCR16
Others: Reserved

**Bit 0 GINT: Global interrupt mask**
This bit is used to mask or unmask the interrupt line assertion to the application. Irrespective of this bit setting, the interrupt status registers are updated by the core.
0: Mask the interrupt assertion to the application
1: Unmask the interrupt assertion to the application
*Note: Accessible in both peripheral and host modes.*

STI
RM0090 Rev 18
1411/1749

--- Page 47 ---

USB on-the-go high-speed (OTG_HS)

RM0090

**OTG_HS USB configuration register (OTG_HS_GUSBCFG)**

Address offset: 0x00C
Reset value: 0x0000 1440

This register can be used to configure the core after power-on or a changing to host mode or peripheral mode. It contains USB and USB-PHY related configuration parameters. The application must program this register before starting any transactions on either the AHB or the USB. Do not make changes to this register after the initial programming.

| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| CTXPKT | FDMOD | FHMOD | Reserved | ULPIPD | PTCI | PCCI | TSDPS | ULPIEVBUSI | ULPIEVBUSD | ULPICSM | ULPIAR | ULPIFSLS | Reserved | PHYLPCS | Reserved | TRDT | HNPCAP | SRPCAP | Reserved | PHYSEL | Reserved | TOCAL |  |  |  |  |  |  |  |  |  |
| r/w |  |  |  | r/w |  | r/w |  |  |  | r/w |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


**Bit 31 CTXPKT:** Corrupt Tx packet
This bit is for debug purposes only. Never set this bit to 1.
*Note: Accessible in both peripheral and host modes.*

**Bit 30 FDMOD:** Forced peripheral mode
Writing a 1 to this bit forces the core to peripheral mode irrespective of the OTG_HS_ID input pin.
0: Normal mode
1: Forced peripheral mode
After setting the force bit, the application must wait at least 25 ms before the change takes effect.
*Note: Accessible in both peripheral and host modes.*

**Bit 29 FHMOD:** Forced host mode
Writing a 1 to this bit forces the core to host mode irrespective of the OTG_HS_ID input pin.
0: Normal mode
1: Forced host mode
After setting the force bit, the application must wait at least 25 ms before the change takes effect.
*Note: Accessible in both peripheral and host modes.*

**Bits 28:26** Reserved, must be kept at reset value.

**Bit 25 ULPIPD:** ULPI interface protect disable
This bit controls the circuitry built in the PHY to protect the ULPI interface when the link tri-states stp and data. Any pull-up or pull-down resistors employed by this feature can be disabled. Please refer to the ULPI specification for more details.
0: Enables the interface protection circuit
1: Disables the interface protection circuit

**Bit 24 PTCI:** Indicator pass through
This bit controls whether the complement output is qualified with the internal VBUS valid comparator before being used in the VBUS state in the RX CMD. Please refer to the ULPI specification for more details.
0: Complement Output signal is qualified with the Internal VBUS valid comparator
1: Complement Output signal is not qualified with the Internal VBUS valid comparator

1412/1749
RM0090 Rev 18

--- Page 48 ---

RM0090
USB on-the-go high-speed (OTG_HS)

**Bit 23 PCCI:** Indicator complement
This bit controls the PHY to invert the ExternalVbusIndicator input signal, and generate the complement output. Please refer to the ULPI specification for more details.
0: PHY does not invert the ExternalVbusIndicator signal
1: PHY inverts ExternalVbusIndicator signal

**Bit 22 TSDPS:** TermSel DLine pulsing selection
This bit selects utmi_termselect to drive the data line pulse during SRP (session request protocol).
0: Data line pulsing using utmi_txvalid (default)
1: Data line pulsing using utmi_termsel

**Bit 21 ULPIEBUSI:** ULPI external VBUS indicator
This bit indicates to the ULPI PHY to use an external VBUS overcurrent indicator.
0: PHY uses an internal VBUS valid comparator
1: PHY uses an external VBUS valid comparator

**Bit 20 ULPIEBUSDD:** ULPI External VBUS Drive
This bit selects between internal or external supply to drive 5 V on VBUS, in the ULPI PHY.
0: PHY drives VBUS using internal charge pump (default)
1: PHY drives VBUS using external supply.

**Bit 19 ULPIESM:** ULPI Clock SuspendM
This bit sets the ClockSuspendM bit in the interface control register on the ULPI PHY. This bit applies only in the serial and carkit modes.
0: PHY powers down the internal clock during suspend
1: PHY does not power down the internal clock

**Bit 18 ULPIAR:** ULPI Auto-resume
This bit sets the AutoResume bit in the interface control register on the ULPI PHY.
0: PHY does not use AutoResume feature
1: PHY uses AutoResume feature

**Bit 17 ULPIFSLS:** ULPI FS/LS select
The application uses this bit to select the FS/LS serial interface for the ULPI PHY. This bit is valid only when the FS serial transceiver is selected on the ULPI PHY.
0: ULPI interface
1: ULPI FS/LS serial interface

**Bit 16** Reserved, must be kept at reset value.

**Bit 15 PHYLPCS:** PHY Low-power clock select
This bit selects either 480 MHz or 48 MHz (low-power) PHY mode. In FS and LS modes, the PHY can usually operate on a 48 MHz clock to save power.
0: 480 MHz internal PLL clock
1: 48 MHz external clock
In 480 MHz mode, the UTMI interface operates at either 60 or 30 MHz, depending on whether the 8- or 16-bit data width is selected. In 48 MHz mode, the UTMI interface operates at 48 MHz in FS and LS modes.

**Bit 14** Reserved, must be kept at reset value.

**Bits 13:10 TRDT:** USB turnaround time
These bits allows to set the turnaround time in PHY clocks. They must be configured according to Table 213: TRDT values, depending on the application AHB frequency. Higher TRDT values allow stretching the USB response time to IN tokens in order to compensate for longer AHB read access latency to the Data FIFO.

ST
RM0090 Rev 18
1413/1749

--- Page 49 ---

USB on-the-go high-speed (OTG_HS)
RM0090

Bit 9 **HNPCAP**: HNP-capable
The application uses this bit to control the OTG_HS controller's HNP capabilities.
0: HNP capability is not enabled
1: HNP capability is enabled
*Note: Accessible in both peripheral and host modes.*

Bit 8 **SRPCAP**: SRP-capable
The application uses this bit to control the OTG_HS controller's SRP capabilities. If the core operates as a nonSRP-capable B-device, it cannot request the connected A-device (host) to activate VBUS and start a session.
0: SRP capability is not enabled
1: SRP capability is enabled
*Note: Accessible in both peripheral and host modes.*

Bit 7 Reserved, must be kept at reset value.

Bit 6 **PHYSEL**: USB 2.0 high-speed ULPI PHY or USB 1.1 full-speed serial transceiver select
0: USB 2.0 high-speed ULPI PHY
1: USB 1.1 full-speed serial transceiver

Bits 5:3 Reserved, must be kept at reset value.

Bits 2:0 **TOCAL**: FS timeout calibration
The number of PHY clocks that the application programs in this field is added to the full-speed interpacket timeout duration in the core to account for any additional delays introduced by the PHY. This can be required, because the delay introduced by the PHY in generating the line state condition can vary from one PHY to another.
The USB standard timeout value for full-speed operation is 16 to 18 (inclusive) bit times. The application must program this field based on the speed of enumeration. The number of bit times added per PHY clock is 0.25 bit times.

Table 213. TRDT values

| AHB frequency range (MHz) | TRDT minimum value |  |
|---|---|---|
| Min. | Max |  |
| 30 | - | 0x9 |


1414/1749
RM0090 Rev 18
ST

--- Page 50 ---

RM0090
USB on-the-go high-speed (OTG_HS)


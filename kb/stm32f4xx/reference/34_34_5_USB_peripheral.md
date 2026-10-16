### 34.5 USB peripheral

This section gives the functional description of the OTG_FS in the USB peripheral mode. The OTG_FS works as an USB peripheral in the following circumstances:

*   OTG B-Peripheral
    *   OTG B-device default state if B-side of USB cable is plugged in
*   OTG A-Peripheral
    *   OTG A-device state after the HNP switches the OTG_FS to its peripheral role
*   B-device
    *   If the ID line is present, functional and connected to the B-side of the USB cable, and the HNP-capable bit in the Global USB Configuration register (HNPCAP bit in OTG_FS_GUSBCFG) is cleared (see On-The-Go Rev1.3 par. 6.8.3).
*   Peripheral only (see Figure 388: USB peripheral-only connection)
    *   The force device mode bit in the Global USB configuration register (FDMOD in OTG_FS_GUSBCFG) is set to 1, forcing the OTG_FS core to work as a USB peripheral-only (see On-The-Go Rev1.3 par. 6.8.3). In this case, the ID line is ignored even if present on the USB connector.

**Note:** To build a bus-powered device implementation in case of the B-device or peripheral-only configuration, an external regulator has to be added that generates the VDD chip-supply from VBUS.

The VBUS pin can be freed by disabling the VBUS sensing option. This is done by setting the NOVBUSSENS bit in the OTG_FS_GCCFG register. In this case the VBUS is considered internally to be always at VBUS valid level (5 V).

1246/1749
RM0090 Rev 18
ST

--- Page 77 ---

RM0090
USB on-the-go full-speed (OTG_FS)

**Figure 388. USB peripheral-only connection**

STM32 MCU
VDD
5V to VDD
Voltage regulator
PA9
PA11
PA12
OSC_IN
OSC_OUT
VBUS
DM
DP
VSS
USB Std-B connector

MS19905V4

1. Use a regulator to build a bus-powered device.

**34.5.1 SRP-capable peripheral**

The SRP capable bit in the Global USB configuration register (SRPCAP bit in OTG_FS_GUSBCFG) enables the OTG_FS to support the session request protocol (SRP). In this way, it allows the remote A-device to save power by switching off VBUS while the USB session is suspended.

The SRP peripheral mode program model is described in detail in the *B-device session request protocol* section.

**34.5.2 Peripheral states**

**Powered state**

The VBUS input detects the B-Session valid voltage by which the USB peripheral is allowed to enter the powered state (see USB2.0 par9.1). The OTG_FS then automatically connects the DP pull-up resistor to signal full-speed device connection to the host and generates the session request interrupt (SRQINT bit in OTG_FS_GINTSTS) to notify the powered state.

The VBUS input also ensures that valid VBUS levels are supplied by the host during USB operations. If a drop in VBUS below B-session valid happens to be detected (for instance because of a power disturbance or if the host port has been switched off), the OTG_FS automatically disconnects and the session end detected (SEDET bit in OTG_FS_GOTGINT) interrupt is generated to notify that the OTG_FS has exited the powered state.

In the powered state, the OTG_FS expects to receive some reset signaling from the host. No other USB operation is possible. When a reset signaling is received the reset detected interrupt (USBRST in OTG_FS_GINTSTS) is generated. When the reset signaling is complete, the enumeration done interrupt (ENUMDNE bit in OTG_FS_GINTSTS) is generated and the OTG_FS enters the Default state.

ST
RM0090 Rev 18
1247/1749

--- Page 78 ---

USB on-the-go full-speed (OTG_FS)
RM0090

**Soft disconnect**
The powered state can be exited by software with the soft disconnect feature. The DP pull-up resistor is removed by setting the soft disconnect bit in the device control register (SDIS bit in OTG_FS_DCTL), causing a device disconnect detection interrupt on the host side even though the USB cable was not really removed from the host port.

**Default state**
In the Default state the OTG_FS expects to receive a SET_ADDRESS command from the host. No other USB operation is possible. When a valid SET_ADDRESS command is decoded on the USB, the application writes the corresponding number into the device address field in the device configuration register (DAD bit in OTG_FS_DCFG). The OTG_FS then enters the address state and is ready to answer host transactions at the configured USB address.

**Suspended state**
The OTG_FS peripheral constantly monitors the USB activity. After counting 3 ms of USB idleness, the early suspend interrupt (ESUSP bit in OTG_FS_GINTSTS) is issued, and confirmed 3 ms later, if appropriate, by the suspend interrupt (USBSUSP bit in OTG_FS_GINTSTS). The device suspend bit is then automatically set in the device status register (SUSPSTS bit in OTG_FS_DSTS) and the OTG_FS enters the suspended state.

The suspended state may optionally be exited by the device itself. In this case the application sets the remote wakeup signaling bit in the device control register (RWUSIG bit in OTG_FS_DCTL) and clears it after 1 to 15 ms.

When a resume signaling is detected from the host, the resume interrupt (WKUPINT bit in OTG_FS_GINTSTS) is generated and the device suspend bit is automatically cleared.

**34.5.3 Peripheral endpoints**
The OTG_FS core instantiates the following USB endpoints:
* Control endpoint 0:
    * Bidirectional and handles control messages only
    * Separate set of registers to handle in and out transactions
    * Proper control (OTG_FS_DIEPCTL0/OTG_FS_DOEPCTL0), transfer configuration (OTG_FS_DIEPTSIZ0/OTG_FS_DOPTSIZ0), and status-interrupt (OTG_FS_DIEPINTx/OTG_FS_DOEPINT0) registers. The available set of bits inside the control and transfer size registers slightly differs from that of other endpoints
* 3 IN endpoints
    * Each of them can be configured to support the isochronous, bulk or interrupt transfer type
    * Each of them has proper control (OTG_FS_DIEPCTLx), transfer configuration (OTG_FS_DIEPTSIZx), and status-interrupt (OTG_FS_DIEPINTx) registers
    * The Device IN endpoints common interrupt mask register (OTG_FS_DIEPMSK) is available to enable/disable a single kind of endpoint interrupt source on all of the IN endpoints (EP0 included)
    * Support for incomplete isochronous IN transfer interrupt (IISOIXFR bit in OTG_FS_GINTSTS), asserted when there is at least one isochronous IN endpoint

1248/1749
RM0090 Rev 18

--- Page 79 ---

RM0090
USB on-the-go full-speed (OTG_FS)

on which the transfer is not completed in the current frame. This interrupt is asserted along with the end of periodic frame interrupt (OTG_FS_GINTSTS/EOPF).

*   3 OUT endpoints
    *   Each of them can be configured to support the isochronous, bulk or interrupt transfer type
    *   Each of them has a proper control (OTG_FS_DOEPCTLx), transfer configuration (OTG_FS_DOEPTSIZx) and status-interrupt (OTG_FS_DOEPINTx) register
    *   Device Out endpoints common interrupt mask register (OTG_FS_DOEPMSK) is available to enable/disable a single kind of endpoint interrupt source on all of the OUT endpoints (EP0 included)
    *   Support for incomplete isochronous OUT transfer interrupt (INCOMPISOOUT bit in OTG_FS_GINTSTS), asserted when there is at least one isochronous OUT endpoint on which the transfer is not completed in the current frame. This interrupt is asserted along with the end of periodic frame interrupt (OTG_FS_GINTSTS/EOPF).

### Endpoint control

*   The following endpoint controls are available to the application through the device endpoint-x IN/OUT control register (DIEPCTLx/DOEPCTLx):
    *   Endpoint enable/disable
    *   Endpoint activate in current configuration
    *   Program USB transfer type (isochronous, bulk, interrupt)
    *   Program supported packet size
    *   Program Tx-FIFO number associated with the IN endpoint
    *   Program the expected or transmitted data0/data1 PID (bulk/interrupt only)
    *   Program the even/odd frame during which the transaction is received or transmitted (isochronous only)
    *   Optionally program the NAK bit to always negative-acknowledge the host regardless of the FIFO status
    *   Optionally program the STALL bit to always stall host tokens to that endpoint
    *   Optionally program the SNOOP mode for OUT endpoint not to check the CRC field of received data

### Endpoint transfer

The device endpoint-x transfer size registers (DIEPTSIZx/DOEPTSIZx) allow the application to program the transfer size parameters and read the transfer status. Programming must be done before setting the endpoint enable bit in the endpoint control register. Once the endpoint is enabled, these fields are read-only as the OTG FS core updates them with the current transfer status.

The following transfer parameters can be programmed:

*   Transfer size in bytes
*   Number of packets that constitute the overall transfer size

ST
RM0090 Rev 18
1249/1749

--- Page 80 ---

USB on-the-go full-speed (OTG_FS)
RM0090

**Endpoint status/interrupt**

The device endpoint-x interrupt registers (DIEPINTx/DOEPINTx) indicate the status of an endpoint with respect to USB- and AHB-related events. The application must read these registers when the OUT endpoint interrupt bit or the IN endpoint interrupt bit in the core interrupt register (OEPINT bit in OTG_FS_GINTSTS or IEPINT bit in OTG_FS_GINTSTS, respectively) is set. Before the application can read these registers, it must first read the device all endpoints interrupt (OTG_FS_DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers

The peripheral core provides the following status checks and interrupt generation:

*   Transfer completed interrupt, indicating that data transfer was completed on both the application (AHB) and USB sides
*   Setup stage has been done (control-out only)
*   Associated transmit FIFO is half or completely empty (in endpoints)
*   NAK acknowledge has been transmitted to the host (isochronous-in only)
*   IN token received when Tx-FIFO was empty (bulk-in/interrupt-in only)
*   Out token received when endpoint was not yet enabled
*   Babble error condition has been detected
*   Endpoint disable by application is effective
*   Endpoint NAK by application is effective (isochronous-in only)
*   More than 3 back-to-back setup packets were received (control-out only)
*   Timeout condition detected (control-in only)
*   Isochronous out packet has been dropped, without generating an interrupt


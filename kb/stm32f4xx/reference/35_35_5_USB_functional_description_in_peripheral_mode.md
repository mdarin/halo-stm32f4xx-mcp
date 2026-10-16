### 35.5 USB functional description in peripheral mode

The OTG_HS operates as an USB peripheral in the following circumstances:

*   **OTG B-device**
    OTG B-device default state if the B-side of USB cable is plugged in
*   **OTG A-device**
    OTG A-device state after the HNP switches the OTG_HS to peripheral role
*   **B-Device**
    If the ID line is present, functional and connected to the B-side of the USB cable, and the HNP-capable bit in the Global USB Configuration register (HNPCAP bit in OTG_HS_GUSBCFG) is cleared (see On-The-Go specification Revision 1.3 section 6.8.3).
*   **Peripheral only** (see Figure 388: USB peripheral-only connection)
    The force peripheral mode bit in the Global USB configuration register (FDMOD in OTG_HS_GUSBCFG) is set to 1, forcing the OTG_HS core to operate in USB peripheral-only mode (see On-The-Go specification Revision 1.3 section 6.8.3). In this case, the ID line is ignored even if it is available on the USB connector.

**Note:** To build a bus-powered device architecture in the B-Device or peripheral-only configuration, an external regulator must be added to generate the VDD supply voltage from VBUS.

### 35.5.1 SRP-capable peripheral

The SRP capable bit in the Global USB configuration register (SRPCAP bit in OTG_HS_GUSBCFG) configures the OTG_HS to support the session request protocol (SRP). As a result, it allows the remote A-device to save power by switching VBUS off when the USB session is suspended.

The SRP peripheral mode program model is described in detail in Section : B-device session request protocol.

1386/1749 RM0090 Rev 18 ST

--- Page 22 ---

RM0090
USB on-the-go high-speed (OTG_HS)

### 35.5.2 Peripheral states

**Powered state**

The VBUS input detects the B-session valid voltage used to put the USB peripheral in the Powered state (see USB2.0 specification section 9.1). The OTG_HS then automatically connects the DP pull-up resistor to signal full-speed device connection to the host, and generates the session request interrupt (SRQINT bit in OTG_HS_GINTSTS) to notify the Powered state. The VBUS input also ensures that valid VBUS levels are supplied by the host during USB operations. If VBUS drops below the B-session valid voltage (for example because power disturbances occurred or the host port has been switched off), the OTG_HS automatically disconnects and the session end detected (SEDET bit in OTG_HS_GOTGINT) interrupt is generated to notify that the OTG_HS has exited the Powered state.

In Powered state, the OTG_HS expects a reset from the host. No other USB operations are possible. When a reset is received, the reset detected interrupt (USBRST in OTG_HS_GINTSTS) is generated. When the reset is complete, the enumeration done interrupt (ENUMDNE bit in OTG_HS_GINTSTS) is generated and the OTG_HS enters the Default state.

**Soft disconnect**

The Powered state can be exited by software by using the soft disconnect feature. The DP pull-up resistor is removed by setting the Soft disconnect bit in the device control register (SDIS bit in OTG_HS_DCTL), thus generating a device disconnect detection interrupt on the host side even though the USB cable was not really unplugged from the host port.

**Default state**

In Default state the OTG_HS expects to receive a SET_ADDRESS command from the host. No other USB operations are possible. When a valid SET_ADDRESS command is decoded on the USB, the application writes the corresponding number into the device address field in the device configuration register (DAD bit in OTG_HS_DCFG). The OTG_HS then enters the address state and is ready to answer host transactions at the configured USB address.

**Suspended state**

The OTG_HS peripheral constantly monitors the USB activity. When the USB remains idle for 3 ms, the early suspend interrupt (ESUSP bit in OTG_HS_GINTSTS) is issued. It is confirmed 3 ms later, if appropriate, by generating a suspend interrupt (USBSUSP bit in OTG_HS_GINTSTS). The device suspend bit is then automatically set in the device status register (SUSPSTS bit in OTG_HS_DSTS) and the OTG_HS enters the Suspended state.

The device can also exit from the Suspended state by itself. In this case the application sets the remote wakeup signaling bit in the device control register (RWUSIG bit in OTG_HS_DCTL) and clears it after 1 to 15 ms.

When a resume signaling is detected from the host, the resume interrupt (WKUPINT bit in OTG_HS_GINTSTS) is generated and the device suspend bit is automatically cleared.

ST
RM0090 Rev 18
1387/1749

--- Page 23 ---

USB on-the-go high-speed (OTG_HS)
RM0090

### 35.5.3 Peripheral endpoints

The OTG_HS core instantiates the following USB endpoints:

*   Control endpoint 0
    This endpoint is bidirectional and handles control messages only.
    It has a separate set of registers to handle IN and OUT transactions, as well as dedicated control (OTG_HS_DIEPCTL0/OTG_HS_DOEPCTL0), transfer configuration (OTG_HS_DIEPTSIZ0/OTG_HS_DIEPTSIZ0), and status-interrupt (OTG_HS_DIEPINTx/OTG_HS_DOEPINT0) registers. The bits available inside the control and transfer size registers slightly differ from other endpoints.
*   5 IN endpoints
    *   They can be configured to support the isochronous, bulk or interrupt transfer type.
    *   They feature dedicated control (OTG_HS_DIEPCTLx), transfer configuration (OTG_HS_DIEPTSIZx), and status-interrupt (OTG_HS_DIEPINTx) registers.
    *   The Device IN endpoints common interrupt mask register (OTG_HS_DIEPMSK) allows to enable/disable a single endpoint interrupt source on all of the IN endpoints (EP0 included).
    *   They support incomplete isochronous IN transfer interrupt (IISOIXFR bit in OTG_HS_GINTSTS). This interrupt is asserted when there is at least one isochronous IN endpoint for which the transfer is not completed in the current frame. This interrupt is asserted along with the end of periodic frame interrupt (OTG_HS_GINTSTS/EOPF).
*   5 OUT endpoints
    *   They can be configured to support the isochronous, bulk or interrupt transfer type.
    *   They feature dedicated control (OTG_HS_DOEPCTLx), transfer configuration (OTG_HS_DOEPTSIZx) and status-interrupt (OTG_HS_DOEPINTx) registers.
    *   The Device Out endpoints common interrupt mask register (OTG_HS_DOEPMSK) allows to enable/disable a single endpoint interrupt source on all OUT endpoints (EP0 included).
    *   They support incomplete isochronous OUT transfer interrupt (INCOMPISOOUT bit in OTG_HS_GINTSTS). This interrupt is asserted when there is at least one isochronous OUT endpoint on which the transfer is not completed in the current frame. This interrupt is asserted along with the end of periodic frame interrupt (OTG_HS_GINTSTS/EOPF).

1388/1749
RM0090 Rev 18
ST

--- Page 24 ---

RM0090
USB on-the-go high-speed (OTG_HS)

## Endpoint controls

The following endpoint controls are available through the device endpoint-x IN/OUT control register (DIEPCTLx/DOEPCTLx):

* Endpoint enable/disable
* Endpoint activation in current configuration
* Program the USB transfer type (isochronous, bulk, interrupt)
* Program the supported packet size
* Program the Tx-FIFO number associated with the IN endpoint
* Program the expected or transmitted data0/data1 PID (bulk/interrupt only)
* Program the even/odd frame during which the transaction is received or transmitted (isochronous only)
* Optionally program the NAK bit to always send a negative acknowledgment to the host regardless of the FIFO status
* Optionally program the STALL bit to always stall host tokens to that endpoint
* Optionally program the Snoop mode for OUT endpoint where the received data CRC is not checked

## Endpoint transfer

The device endpoint-x transfer size registers (DIEPTSIZx/DOPTSIZx) allow the application to program the transfer size parameters and read the transfer status.

The programming operation must be performed before setting the endpoint enable bit in the endpoint control register.

Once the endpoint is enabled, these fields are read-only as the OTG FS core updates them with the current transfer status.

The following transfer parameters can be programmed:

* Transfer size in bytes
* Number of packets constituting the overall transfer size.

## Endpoint status/interrupt

The device endpoint-x interrupt registers (DIEPINTx/DOEPINTx) indicate the status of an endpoint with respect to USB- and AHB-related events. The application must read these registers when the OUT endpoint interrupt bit or the IN endpoint interrupt bit in the core interrupt register (OEPINT bit in OTG_HS_GINTSTS or IEPINT bit in OTG_HS_GINTSTS, respectively) is set. Before the application can read these registers, it must first read the device all endpoints interrupt register (OTG_HS_DAINT) to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers.

ST
RM0090 Rev 18
1389/1749

--- Page 25 ---

**USB on-the-go high-speed (OTG_HS)**

RM0090

The peripheral core provides the following status checks and interrupt generation:

*   Transfer completed interrupt, indicating that data transfer has completed on both the application (AHB) and USB sides
*   Setup stage done (control-out only)
*   Associated transmit FIFO is half or completely empty (in endpoints)
*   NAK acknowledge transmitted to the host (isochronous-in only)
*   IN token received when Tx-FIFO was empty (bulk-in/interrupt-in only)
*   OUT token received when endpoint was not yet enabled
*   Babble error condition detected
*   Endpoint disable by application is effective
*   Endpoint NAK by application is effective (isochronous-in only)
*   More than 3 back-to-back setup packets received (control-out only)
*   Timeout condition detected (control-in only)
*   Isochronous out packet dropped without generating an interrupt

1390/1749
RM0090 Rev 18

--- Page 26 ---

RM0090
USB on-the-go high-speed (OTG_HS)


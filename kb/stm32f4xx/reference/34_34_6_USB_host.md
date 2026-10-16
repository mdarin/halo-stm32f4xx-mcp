**34.6 USB host**

This section gives the functional description of the OTG_FS in the USB host mode. The OTG_FS works as a USB host in the following circumstances:

*   OTG A-host
    *   OTG A-device default state when the A-side of the USB cable is plugged in
*   OTG B-host
    *   OTG B-device after HNP switching to the host role
*   A-device
    *   If the ID line is present, functional and connected to the A-side of the USB cable, and the HNP-capable bit is cleared in the Global USB Configuration register (HNPCAP bit in OTG_FS_GUSBCFG). Integrated pull-down resistors are automatically set on the DP/DM lines.
*   Host only (see Figure 389: USB host-only connection).
    *   The force host mode bit in the global USB configuration register (FHMOD bit in OTG_FS_GUSBCFG) forces the OTG_FS core to work as a USB host-only. In this case, the ID line is ignored even if present on the USB connector. Integrated pull-down resistors are automatically set on the DP/DM lines.

*Note:* On-chip 5 V VBUS generation is not supported. For this reason, a charge pump or, if 5 V are available on the application board, a basic power switch must be added externally to drive

1250/1749
RM0090 Rev 18

--- Page 81 ---

RM0090
USB on-the-go full-speed (OTG_FS)

the 5 V VBUS line. The external charge pump can be driven by any GPIO output. This is required for the OTG A-host, A-device and host-only configurations.

The VBUS input ensures that valid VBUS levels are supplied by the charge pump during USB operations while the charge pump overcurrent output can be input to any GPIO pin configured to generate port interrupts. The overcurrent ISR must promptly disable the VBUS generation.

The VBUS pin can be freed by disabling the VBUS sensing option. This is done by setting the NOVBUSSENS bit in the OTG_FS_GCCFG register. In this case the VBUS is considered internally to be always at VBUS valid level (5 V).

Figure 389. USB host-only connection

MSv36915V2

1. STMP52141STR needed only if the application has to support a VBUS powered device. A basic power switch can be used if 5 V are available on the application board.
2. VDD range is between 2 V and 3.6 V.

34.6.1 SRP-capable host

SRP support is available through the SRP capable bit in the global USB configuration register (SRPCAP bit in OTG_FS_GUSBCFG). With the SRP feature enabled, the host can save power by switching off the VBUS power while the USB session is suspended.

The SRP host mode program model is described in detail in the A-device session request protocol section.

34.6.2 USB host states

Host port power

On-chip 5 V VBUS generation is not supported. For this reason, a charge pump or, if 5 V are available on the application board, a basic power switch, must be added externally to drive the 5 V VBUS line. The external charge pump can be driven by any GPIO output. When the application decides to power on VBUS using the chosen GPIO, it must also set the port power bit in the host port control and status register (PPWR bit in OTG_FS_HPRT).

ST
RM0090 Rev 18
1251/1749

--- Page 82 ---

USB on-the-go full-speed (OTG_FS)

RM0090

**V<sub>BUS</sub> valid**

When HNP or SRP is enabled the VBUS sensing pin (PA9) pin should be connected to V<sub>BUS</sub>. The V<sub>BUS</sub> input ensures that valid V<sub>BUS</sub> levels are supplied by the charge pump during USB operations. Any unforeseen V<sub>BUS</sub> voltage drop below the V<sub>BUS</sub> valid threshold (4.25 V) leads to an OTG interrupt triggered by the session end detected bit (SEDET bit in OTG_FS_GOTGINT). The application is then required to remove the V<sub>BUS</sub> power and clear the port power bit.

When HNP and SRP are both disabled, the VBUS sensing pin (PA9) should not be connected to V<sub>BUS</sub>. This pin can be can be used as GPIO.

The charge pump overcurrent flag can also be used to prevent electrical damage. Connect the overcurrent flag output from the charge pump to any GPIO input and configure it to generate a port interrupt on the active level. The overcurrent ISR must promptly disable the V<sub>BUS</sub> generation and clear the port power bit.

**Host detection of a peripheral connection**

If SRP or HNP are enabled, even if USB peripherals or B-devices can be attached at any time, the OTG_FS will not detect any bus connection until V<sub>BUS</sub> is no longer sensed at a valid level (5 V). When V<sub>BUS</sub> is at a valid level and a remote B-device is attached, the OTG_FS core issues a host port interrupt triggered by the device connected bit in the host port control and status register (PCDET bit in OTG_FS_HPRT).

When HNP and SRP are both disabled, USB peripherals or B-device are detected as soon as they are connected. The OTG_FS core issues a host port interrupt triggered by the device connected bit in the host port control and status (PCDET bit in OTG_FS_HPRT).

**Host detection of peripheral a disconnection**

The peripheral disconnection event triggers the disconnect detected interrupt (DISCINT bit in OTG_FS_GINTSTS).

**Host enumeration**

After detecting a peripheral connection the host must start the enumeration process by sending USB reset and configuration commands to the new peripheral.

Before starting to drive a USB reset, the application waits for the OTG interrupt triggered by the debounce done bit (DBCDNE bit in OTG_FS_GOTGINT), which indicates that the bus is stable again after the electrical debounce caused by the attachment of a pull-up resistor on DP (FS) or DM (LS).

The application drives a USB reset signaling (single-ended zero) over the USB by keeping the port reset bit set in the host port control and status register (PRST bit in OTG_FS_HPRT) for a minimum of 10 ms and a maximum of 20 ms. The application takes care of the timing count and then of clearing the port reset bit.

Once the USB reset sequence has completed, the host port interrupt is triggered by the port enable/disable change bit (PENCHNG bit in OTG_FS_HPRT). This informs the application that the speed of the enumerated peripheral can be read from the port speed field in the host port control and status register (PSPD bit in OTG_FS_HPRT) and that the host is starting to drive SOFs (FS) or Keep alives (LS). The host is now ready to complete the peripheral enumeration by sending peripheral configuration commands.

1252/1749

RM0090 Rev 18

ST

--- Page 83 ---

RM0090
USB on-the-go full-speed (OTG_FS)

**Host suspend**

The application decides to suspend the USB activity by setting the port suspend bit in the host port control and status register (PSUSP bit in OTG_FS_HPRT). The OTG_FS core stops sending SOFs and enters the suspended state.

The suspended state can be optionally exited on the remote device's initiative (remote wakeup). In this case the remote wakeup interrupt (WKUPINT bit in OTG_FS_GINTSTS) is generated upon detection of a remote wakeup signaling, the port resume bit in the host port control and status register (PRES bit in OTG_FS_HPRT) self-sets, and resume signaling is automatically driven over the USB. The application must time the resume window and then clear the port resume bit to exit the suspended state and restart the SOF.

If the suspended state is exited on the host initiative, the application must set the port resume bit to start resume signaling on the host port, time the resume window and finally clear the port resume bit.

**34.6.3 Host channels**

The OTG_FS core instantiates 8 host channels. Each host channel supports an USB host transfer (USB pipe). The host is not able to support more than 8 transfer requests at the same time. If more than 8 transfer requests are pending from the application, the host controller driver (HCD) must re-allocate channels when they become available from previous duty, that is, after receiving the transfer completed and channel halted interrupts.

Each host channel can be configured to support in/out and any type of periodic/nonperiodic transaction. Each host channel makes us of proper control (HCCHARx), transfer configuration (HCTSIZx) and status/interrupt (HCINTx) registers with associated mask (HCINTMSKx) registers.

**Host channel control**

* The following host channel controls are available to the application through the host channel-x characteristics register (HCCHARx):
    - Channel enable/disable
    - Program the FS/LS speed of target USB peripheral
    - Program the address of target USB peripheral
    - Program the endpoint number of target USB peripheral
    - Program the transfer IN/OUT direction
    - Program the USB transfer type (control, bulk, interrupt, isochronous)
    - Program the maximum packet size (MPS)
    - Program the periodic transfer to be executed during odd/even frames

**Host channel transfer**

The host channel transfer size registers (HCTSIZx) allow the application to program the transfer size parameters, and read the transfer status. Programming must be done before setting the channel enable bit in the host channel characteristics register. Once the endpoint

ST
RM0090 Rev 18
1253/1749

--- Page 84 ---

**USB on-the-go full-speed (OTG_FS)** RM0090

is enabled the packet count field is read-only as the OTG FS core updates it according to the current transfer status.

*   The following transfer parameters can be programmed:
    *   transfer size in bytes
    *   number of packets making up the overall transfer size
    *   initial data PID

**Host channel status/interrupt**

The host channel-x interrupt register (HCINTx) indicates the status of an endpoint with respect to USB- and AHB-related events. The application must read these register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_FS_GINTSTS) is set. Before the application can read these registers, it must first read the host all channels interrupt (HCAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers. The mask bits for each interrupt source of each channel are also available in the OTG_FS_HCINTMSK-x register.

*   The host core provides the following status checks and interrupt generation:
    *   Transfer completed interrupt, indicating that the data transfer is complete on both the application (AHB) and USB sides
    *   Channel has stopped due to transfer completed, USB transaction error or disable command from the application
    *   Associated transmit FIFO is half or completely empty (IN endpoints)
    *   ACK response received
    *   NAK response received
    *   STALL response received
    *   USB transaction error due to CRC failure, timeout, bit stuff error, false EOP
    *   Babble error
    *   fraMe overrun
    *   dAta toggle error

**34.6.4 Host scheduler**

The host core features a built-in hardware scheduler which is able to autonomously re-order and manage the USB transaction requests posted by the application. At the beginning of each frame the host executes the periodic (isochronous and interrupt) transactions first, followed by the nonperiodic (control and bulk) transactions to achieve the higher level of priority granted to the isochronous and interrupt transfer types by the USB specification.

The host processes the USB transactions through request queues (one for periodic and one for nonperiodic). Each request queue can hold up to 8 entries. Each entry represents a pending transaction request from the application, and holds the IN or OUT channel number along with other information to perform a transaction on the USB. The order in which the requests are written to the queue determines the sequence of the transactions on the USB interface.

At the beginning of each frame, the host processes the periodic request queue first, followed by the nonperiodic request queue. The host issues an incomplete periodic transfer interrupt (IPXFR bit in OTG_FS_GINTSTS) if an isochronous or interrupt transaction scheduled for the current frame is still pending at the end of the current frame. The OTG HS core is fully

1254/1749 RM0090 Rev 18

--- Page 85 ---

RM0090
USB on-the-go full-speed (OTG_FS)

responsible for the management of the periodic and nonperiodic request queues. The periodic transmit FIFO and queue status register (HPTXSTS) and nonperiodic transmit FIFO and queue status register (HNPTXSTS) are read-only registers which can be used by the application to read the status of each request queue. They contain:

*   The number of free entries currently available in the periodic (nonperiodic) request queue (8 max)
*   Free space currently available in the periodic (nonperiodic) Tx-FIFO (out-transactions)
*   IN/OUT token, host channel number and other status information.

As request queues can hold a maximum of 8 entries each, the application can push to schedule host transactions in advance with respect to the moment they physically reach the SB for a maximum of 8 pending periodic transactions plus 8 pending nonperiodic transactions.

To post a transaction request to the host scheduler (queue) the application must check that there is at least 1 entry available in the periodic (nonperiodic) request queue by reading the PTXQSAV bits in the OTG_FS_HNPTXSTS register or NPTQXSAV bits in the OTG_FS_HNPTXSTS register.


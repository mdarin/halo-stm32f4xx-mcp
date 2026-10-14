## 35.2 OTG_HS main features

The main features can be divided into three categories: general, host-mode and peripheral-mode features.

ST
RM0090 Rev 18
1381/1749

--- Page 17 ---

USB on-the-go high-speed (OTG_HS)
RM0090

### 35.2.1 General features

The OTG_HS interface main features are the following:

* It is USB-IF certified in compliance with the Universal Serial Bus Revision 2.0 Specification
* It supports 3 PHY interfaces
    * An on-chip full-speed PHY
    * An ULPI interface for external high-speed PHY.
* It supports the host negotiation protocol (HNP) and the session request protocol (SRP)
* It allows the host to turn V<sub>BUS</sub> off to save power in OTG applications, with no need for external components
* It allows to monitor V<sub>BUS</sub> levels using internal comparators
* It supports dynamic host-peripheral role switching
* It is software-configurable to operate as:
    * An SRP-capable USB HS/FS peripheral (B-device)
    * An SRP-capable USB HS/FS/low-speed host (A-device)
    * An USB OTG FS dual-role device
* It supports HS/FS SOFs as well as low-speed (LS) keep-alive tokens with:
    * SOF pulse PAD output capability
    * SOF pulse internal connection to timer 2 (TIM2)
    * Configurable framing period
    * Configurable end-of-frame interrupt
* It embeds an internal DMA with shareholding support and software selectable AHB burst type in DMA mode
* It has power saving features such as system clock stop during USB suspend, switching off of the digital core internal clock domains, PHY and DFIFO power management
* It features a dedicated 4-Kbyte data RAM with advanced FIFO management:
    * The memory partition can be configured into different FIFOs to allow flexible and efficient use of RAM
    * Each FIFO can contain multiple packets
    * Memory allocation is performed dynamically
    * The FIFO size can be configured to values that are not powers of 2 to allow the use of contiguous memory locations
* It ensures a maximum USB bandwidth of up to one frame without application intervention

1382/1749
RM0090 Rev 18
ST

--- Page 18 ---

RM0090
USB on-the-go high-speed (OTG_HS)

### 35.2.2 Host-mode features

The OTG_HS interface features in host mode are the following:

*   It requires an external charge pump to generate V<sub>BUS</sub>
*   It has up to 12 host channels (pipes), each channel being dynamically reconfigurable to support any kind of USB transfer
*   It features a built-in hardware scheduler holding:
    *   Up to 8 interrupt plus isochronous transfer requests in the periodic hardware queue
    *   Up to 8 control plus bulk transfer requests in the nonperiodic hardware queue
*   It manages a shared RX FIFO, a periodic TX FIFO, and a nonperiodic TX FIFO for efficient usage of the USB data RAM
*   It features dynamic trimming capability of SOF framing period in host mode.

### 35.2.3 Peripheral-mode features

The OTG_HS interface main features in peripheral mode are the following:

*   It has 1 bidirectional control endpoint 0
*   It has 5 IN endpoints (EP) configurable to support bulk, interrupt or isochronous transfers
*   It has 5 OUT endpoints configurable to support bulk, interrupt or isochronous transfers
*   It manages a shared Rx FIFO and a Tx-OUT FIFO for efficient usage of the USB data RAM
*   It manages up to 6 dedicated Tx-IN FIFOs (one for each IN-configured EP) to reduce the application load
*   It features soft disconnect capability


# 34 USB on-the-go full-speed (OTG_FS)

This section applies to the whole STM32F4xx family, unless otherwise specified.

## 34.1 OTG_FS introduction

Portions Copyright (c) 2004, 2005 Synopsys, Inc. All rights reserved. Used with permission.
This section presents the architecture and the programming model of the OTG_FS controller.

The following acronyms are used throughout this section:

| FS | Full-speed |
|---|---|
| LS | Low-speed |
| MAC | Media access controller |
| OTG | On-the-go |
| PFC | Packet FIFO controller |
| PHY | Physical layer |
| USB | Universal serial bus |
| UTMI | USB 2.0 transceiver macrocell interface (UTMI) |


References are made to the following documents:

* USB On-The-Go Supplement, Revision 1.3
* Universal Serial Bus Revision 2.0 Specification

The OTG_FS is a dual-role device (DRD) controller that supports both device and host functions and is fully compliant with the *On-The-Go Supplement to the USB 2.0 Specification*. It can also be configured as a host-only or device-only controller, fully compliant with the *USB 2.0 Specification*. In host mode, the OTG_FS supports full-speed (FS, 12 Mbits/s) and low-speed (LS, 1.5 Mbits/s) transfers whereas in device mode, it only supports full-speed (FS, 12 Mbits/s) transfers. The OTG_FS supports both HNP and SRP. The only external device required is a charge pump for VBUS in host mode.

1240/1749
RM0090 Rev 18
ST

--- Page 71 ---

RM0090
USB on-the-go full-speed (OTG_FS)

### 34.2 OTG_FS main features

The main features can be divided into three categories: general, host-mode and device-mode features.

#### 34.2.1 General features

The OTG_FS interface general features are the following:

*   It is USB-IF certified to the Universal Serial Bus Specification Rev 2.0
*   It includes full support (PHY) for the optional On-The-Go (OTG) protocol detailed in the On-The-Go Supplement Rev 1.3 specification
    *   Integrated support for A-B Device Identification (ID line)
    *   Integrated support for host Negotiation Protocol (HNP) and Session Request Protocol (SRP)
    *   It allows host to turn VBUS off to conserve battery power in OTG applications
    *   It supports OTG monitoring of VBUS levels with internal comparators
    *   It supports dynamic host-peripheral switch of role
*   It is software-configurable to operate as:
    *   SRP capable USB FS Peripheral (B-device)
    *   SRP capable USB FS/LS host (A-device)
    *   USB On-The-Go Full-Speed Dual Role device
*   It supports FS SOF and LS Keep-alives with
    *   SOF pulse PAD connectivity (OTG_FS_SOF)
    *   SOF pulse internal connection to timer2 (TIM2)
    *   Configurable framing period
    *   Configurable end of frame interrupt
*   It includes power saving features such as system stop during USB Suspend, switch-off of clock domains internal to the digital core, PHY and DFIFO power management
*   It features a dedicated RAM of 1.25 Kbytes with advanced FIFO control:
    *   Configurable partitioning of RAM space into different FIFOs for flexible and efficient use of RAM
    *   Each FIFO can hold multiple packets
    *   Dynamic memory allocation
    *   Configurable FIFO sizes that are not powers of 2 to allow the use of contiguous memory locations
*   It guarantees max USB bandwidth for up to one frame (1ms) without system intervention

STI
RM0090 Rev 18
1241/1749

--- Page 72 ---

USB on-the-go full-speed (OTG_FS) RM0090

### 34.2.2 Host-mode features

The OTG_FS interface main features and requirements in host-mode are the following:

*   External charge pump for VBUS voltage generation.
*   Up to 8 host channels (pipes): each channel is dynamically reconfigurable to allocate any type of USB transfer.
*   Built-in hardware scheduler holding:
    *   Up to 8 interrupt plus isochronous transfer requests in the periodic hardware queue
    *   Up to 8 control plus bulk transfer requests in the non-periodic hardware queue
*   Management of a shared RX FIFO, a periodic TX FIFO and a nonperiodic TX FIFO for efficient usage of the USB data RAM.

### 34.2.3 Peripheral-mode features

The OTG_FS interface main features in peripheral-mode are the following:

*   1 bidirectional control endpoint0
*   3 IN endpoints (EPs) configurable to support Bulk, Interrupt or Isochronous transfers
*   3 OUT endpoints configurable to support Bulk, Interrupt or Isochronous transfers
*   Management of a shared Rx FIFO and a Tx-OUT FIFO for efficient usage of the USB data RAM
*   Management of up to 4 dedicated Tx-IN FIFOs (one for each active IN EP) to put less load on the application
*   Support for the soft disconnect feature.

1242/1749 RM0090 Rev 18 ST

--- Page 73 ---

RM0090
USB on-the-go full-speed (OTG_FS)


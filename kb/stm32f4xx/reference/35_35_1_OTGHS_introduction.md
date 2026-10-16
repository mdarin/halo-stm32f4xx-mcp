# 35 USB on-the-go high-speed (OTG_HS)

This section applies to the whole STM32F4xx family, unless otherwise specified.

## 35.1 OTG_HS introduction

Portions Copyright (c) 2004, 2005 Synopsys, Inc. All rights reserved. Used with permission.
This section presents the architecture and the programming model of the OTG_HS controller.

The following acronyms are used throughout the section:

| ACRONYM | MEANING |
|---|---|
| FS | full-speed |
| HS | High-speed |
| LS | Low-speed |
| USB | Universal serial bus |
| OTG | On-the-go |
| PHY | Physical layer |
| MAC | Media access controller |
| PFC | Packet FIFO controller |
| UTMI | USB Transceiver Macrocell Interface |
| ULPI | UTMI+ Low Pin Interface |


References are made to the following documents:
* USB On-The-Go Supplement, Revision 1.3
* Universal Serial Bus Revision 2.0 Specification

The OTG_HS is a dual-role device (DRD) controller that supports both peripheral and host functions and is fully compliant with the *On-The-Go Supplement to the USB 2.0 Specification*. It can also be configured as a host-only or peripheral-only controller, fully compliant with the *USB 2.0 Specification*. In host mode, the OTG_HS supports high-speed (HS, 480 Mbits/s), full-speed (FS, 12 Mbits/s) and low-speed (LS, 1.5 Mbits/s) transfers whereas in peripheral mode, it only supports high-speed (HS, 480Mbits/s) and full-speed (FS, 12 Mbits/s) transfers. The OTG_HS supports both HNP and SRP. The only external device required is a charge pump for VBUS in OTG mode.


# 31 Secure digital input/output interface (SDIO)

This section applies to the whole STM32F4xx family, unless otherwise specified.

## 31.1 SDIO main features

The SD/SDIO MMC card host interface (SDIO) provides an interface between the APB2 peripheral bus and MultiMediaCards (MMCs), SD memory cards, SDIO cards and CE-ATA devices.

The MultiMediaCard system specifications are available through the MultiMediaCard Association website at http://www.jedec.org/, published by the MMCA technical committee.

SD memory card and SD I/O card system specifications are available through the SD card Association website at http://www.sdcard.org.

CE-ATA system specifications are available through the CE-ATA workgroup website.

The SDIO features include the following:

*   Full compliance with *MultiMediaCard System Specification Version 4.2*. Card support for three different databus modes: 1-bit (default), 4-bit and 8-bit
*   Full compatibility with previous versions of MultiMediaCards (forward compatibility)
*   Full compliance with *SD Memory Card Specifications Version 2.0*
*   Full compliance with *SD I/O Card Specification Version 2.0*: card support for two different databus modes: 1-bit (default) and 4-bit
*   Full support of the CE-ATA features (full compliance with *CE-ATA digital protocol Rev1.1*)
*   Data transfer up to 50 MHz for the 8 bit mode
*   Data and command output enable signals to control external bidirectional drivers.

**Note:** *The SDIO does not have an SPI-compatible communication mode.*

The SD memory card protocol is a superset of the MultiMediaCard protocol as defined in the MultiMediaCard system specification V2.11. Several commands required for SD memory devices are not supported by either SD I/O-only cards or the I/O portion of combo cards. Some of these commands have no use in SD I/O devices, such as erase commands, and thus are not supported in the SDIO. In addition, several commands are different between SD memory cards and SD I/O cards and thus are not supported in the SDIO. For details refer to SD I/O card Specification Version 1.0. CE-ATA is supported over the MMC electrical interface using a protocol that utilizes the existing MMC access primitives. The interface electrical and signaling definition is as defined in the MMC reference.

The MultiMediaCard/SD bus connects cards to the controller.

The current version of the SDIO supports only one SD/SDIO/MMC4.2 card at any one time and a stack of MMC4.1 or previous.

ST
RM0090 Rev 18
1019/1749

--- Page 45 ---

Secure digital input/output interface (SDIO)
RM0090


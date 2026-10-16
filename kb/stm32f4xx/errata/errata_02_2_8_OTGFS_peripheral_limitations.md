**2.8 OTG_FS peripheral limitations**

**2.8.1 Data in RxFIFO is overwritten when all channels are disabled simultaneously**

**Description**

If the available RxFIFO is just large enough to host 1 packet + its data status, and is currently occupied by the last received data + its status and, at the same time, the



37

--- Page 24 ---

**STM32F40x and STM32F41x silicon limitations STM32F40x and STM32F41x**

application requests that more IN channels be disabled, the OTG_FS peripheral does not first check for available space before inserting the disabled status of the IN channels. It just inserts them by overwriting the existing data payload.

**Workaround**

Use one of the following recommendations:

1. Configure the RxFIFO to host a minimum of 2 × MPSIZ + 2 × data status entries.

2. The application has to check the RXFLVL bit (RxFIFO non-empty) in the

OTG_FS_GINTSTS register before disabling each IN channel. If this bit is not set, then the application can disable an IN channel at a time. Each time the application disables an IN channel, however, it first has to check that the RXFLVL bit = 0 condition is true.

**2.8.2 OTG host blocks the receive channel when receiving IN packets and no TxFIFO is configured**

**Description**

When receiving data, the OTG_FS core erroneously checks for available TxFIFO space when it should only check for RxFIFO space. If the OTG_FS core cannot see any space allocated for data transmission, it blocks the reception channel and no data is received. Workaround

Set at least one TxFIFO equal to the maximum packet size. In this way, the host application, which intends to supports only IN traffic, also has to allocate some space for the TxFIFO. Since a USB host is expected to support any kind of connected endpoint, it is good practice to always configure enough TxFIFO space for OUT endpoints.

**2.8.3 Host channel-halted interrupt not generated when the channel is disabled**

**Description**

When the application enables, then immediately disables the host channel before the OTG_FS host has had time to begin the transfer sequence, the OTG_FS core, as a host, does not generate a channel-halted interrupt. The OTG_FS core continues to operate normally.

**Workaround**

Do not disable the host channel immediately after enabling it.

**2.8.4 Error in software-read OTG_FS_DCFG register values**

**Description**

When the application writes to the DAD and PFIVL bitfields in the OTG_FS_DCFG register, and then reads the newly written bitfield values, the read values may not be correct. The values written by the application, however, are correctly retained by the core, and the normal operation of the device is not affected.



--- Page 25 ---

**STM32F40x and STM32F41x STM32F40x and STM32F41x silicon limitations**

**Workaround**

Do not read from the OTG_FS_DCFG register’s DAD and PFIVL bitfields just after programming them.


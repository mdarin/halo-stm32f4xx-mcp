### 34.8 OTG low-power modes

Table 197 below defines the STM32 low power modes and their compatibility with the OTG.

**Table 197. Compatibility of STM32 low power modes with the OTG**

| Mode | Description | USB compatibility |
|---|---|---|
| Run | MCU fully active | Required when USB not in suspend state. |
| Sleep | USB suspend exit causes the device to exit Sleep mode. Peripheral registers content is kept. | Available while USB is in suspend state. |
| Stop | USB suspend exit causes the device to exit Stop mode. Peripheral registers content is kept(1). | Available while USB is in suspend state. |
| Standby | Powered-down. The peripheral must be reinitialized after exiting Standby mode. | Not compatible with USB applications. |


1. Within Stop mode there are different possible settings. Some restrictions may also exist, please refer to Section 5: Power controller (PWR) to understand which (if any) restrictions apply when using OTG.

1256/1749 RM0090 Rev 18

--- Page 87 ---

RM0090
USB on-the-go full-speed (OTG_FS)

The power consumption of the OTG PHY is controlled by three bits in the general core configuration register:

*   PHY power down (GCCFG/PWRDWN)
    It switches on/off the full-speed transceiver module of the PHY. It must be preliminarily set to allow any USB operation.
*   A-VBUS sensing enable (GCCFG/VBUSASEN)
    It switches on/off the VBUS comparators associated with A-device operations. It must be set when in A-device (USB host) mode and during HNP.
*   B-VBUS sensing enable (GCCFG/VBUSASEN)
    It switches on/off the VBUS comparators associated with B-device operations. It must be set when in B-device (USB peripheral) mode and during HNP.

Power reduction techniques are available while in the USB suspended state, when the USB session is not yet valid or the device is disconnected.

*   Stop PHY clock (STPPCLK bit in OTG_FS_PCGCCTL)
    When setting the stop PHY clock bit in the clock gating control register, most of the 48 MHz clock domain internal to the OTG full-speed core is switched off by clock gating. The dynamic power consumption due to the USB clock switching activity is cut even if the 48 MHz clock input is kept running by the application
    Most of the transceiver is also disabled, and only the part in charge of detecting the asynchronous resume or remote wakeup event is kept alive.
*   Gate HCLK (GATEHCLK bit in OTG_FS_PCGCCTL)
    When setting the Gate HCLK bit in the clock gating control register, most of the system clock domain internal to the OTG_FS core is switched off by clock gating. Only the register read and write interface is kept alive. The dynamic power consumption due to the USB clock switching activity is cut even if the system clock is kept running by the application for other purposes.
*   USB system stop
    When the OTG_FS is in the USB suspended state, the application may decide to drastically reduce the overall power consumption by a complete shut down of all the clock sources in the system. USB System Stop is activated by first setting the Stop PHY clock bit and then configuring the system deep sleep mode in the power control system module (PWR).
    The OTG_FS core automatically reactivates both system and USB clocks by asynchronous detection of remote wakeup (as an host) or resume (as a device) signaling on the USB.

To save dynamic power, the USB data FIFO is clocked only when accessed by the OTG_FS core.

**34.9 Dynamic update of the OTG_FS_HFIR register**

The USB core embeds a dynamic trimming capability of SOF framing period in host mode allowing to synchronize an external device with the SOF frames.

When the OTG_FS_HFIR register is changed within a current SOF frame, the SOF period correction is applied in the next frame as described in Figure 391.

ST
RM0090 Rev 18
1257/1749

--- Page 88 ---

USB on-the-go full-speed (OTG_FS)
RM0090

Figure 391. Updating OTG_FS_HFIR dynamically

SOF reload
Old OTG_FS_HIFR value = 400 periods
OTG_FS_HIFR value = 450 periods+HIFR write latency
Latency
New OTG_FS_HIFR value = 450 periods

OTG_FS_HIFR write

OTG_FS_HIFR value
400
450

Frame timer
ai18r

### 34.10 USB data FIFOs

The USB system features 1.25 Kbyte of dedicated RAM with a sophisticated FIFO control mechanism. The packet FIFO controller module in the OTG_FS core organizes RAM space into Tx-FIFOs into which the application pushes the data to be temporarily stored before the USB transmission, and into a single Rx FIFO where the data received from the USB are temporarily stored before retrieval (popped) by the application. The number of instructed FIFOs and how these are organized inside the RAM depends on the device's role. In peripheral mode an additional Tx-FIFO is instructed for each active IN endpoint. Any FIFO size is software configured to better meet the application requirements.

1258/1749
RM0090 Rev 18
ST

--- Page 89 ---

RM0090
USB on-the-go full-speed (OTG_FS)


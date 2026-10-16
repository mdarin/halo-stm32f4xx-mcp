### 35.8 OTG_HS low-power modes

Table 207 below defines the STM32 low power modes and their compatibility with the OTG.

**Table 207. Compatibility of STM32 low power modes with the OTG**

| Mode | Description | USB compatibility |
|---|---|---|
| Run | MCU fully active | Required when USB not in suspend state. |
| Sleep | USB suspend exit causes the device to exit Sleep mode. Peripheral registers content is kept. | Available while USB is in suspend state. |
| Stop | USB suspend exit causes the device to exit Stop mode. Peripheral registers content is kept(1). | Available while USB is in suspend state. |
| Standby | Powered-down. The peripheral must be reinitialized after exiting Standby mode. | Not compatible with USB applications. |


1. Within Stop mode there are different possible settings. Some restrictions may also exist, please refer to Section 5: Power controller (PWR) to understand which (if any) restrictions apply when using OTG.

The following bits and procedures reduce power consumption.

The power consumption of the OTG PHY is controlled by three bits in the general core configuration register:

*   PHY power down (GCCFG/PWRDWN)
    This bit switches on/off the PHY full-speed transceiver module. It must be preliminarily set to allow any USB operation.
*   A-VBUS sensing enable (GCCFG/VBUSASEN)
    This bit switches on/off the VBUS comparators associated with A-device operations. It must be set when in A-device (USB host) mode and during HNP.
*   B-VBUS sensing enable (GCCFG/VBUSASEN)
    This bit switches on/off the VBUS comparators associated with B-device operations. It must be set when in B-device (USB peripheral) mode and during HNP.
    Power reduction techniques are available in the USB suspended state, when the USB session is not yet valid or the device is disconnected.
*   Stop PHY clock (STPPCLK bit in OTG_HS_PCGCCTL)
    *   When setting the stop PHY clock bit in the clock gating control register, most of the clock domain internal to the OTG high-speed core is switched off by clock gating.

STI
RM0090 Rev 18
1397/1749

--- Page 33 ---

**USB on-the-go high-speed (OTG_HS)**
RM0090

The dynamic power consumption due to the USB clock switching activity is cut even if the clock input is kept running by the application
- Most of the transceiver is also disabled, and only the part in charge of detecting the asynchronous resume or remote wakeup event is kept alive.
- Gate HCLK (GATEHCLK bit in OTG_HS_PCGCTL)
When setting the Gate HCLK bit in the clock gating control register, most of the system clock domain internal to the OTG_HS core is switched off by clock gating. Only the register read and write interface is kept alive. The dynamic power consumption due to the USB clock switching activity is cut even if the system clock is kept running by the application for other purposes.
- USB system stop
- When the OTG_HS is in USB suspended state, the application can decide to drastically reduce the overall power consumption by shutting down all the clock sources in the system. USB System Stop is activated by first setting the Stop PHY clock bit and then configuring the system deep sleep mode in the powercontrol system module (PWR).
- The OTG_HS core automatically reactivates both system and USB clocks by asynchronous detection of remote wakeup (as an host) or resume (as a Device) signaling on the USB.

**35.9 Dynamic update of the OTG_HS_HFIR register**

The USB core embeds a dynamic trimming capability of micro-SOF framing period in host mode allowing to synchronize an external device with the micro-SOF frames.

When the OTG_HS_HFIR register is changed within a current micro-SOF frame, the SOF period correction is applied in the next frame as described in Figure 413.

Figure 413. Updating OTG_HS_HFIR dynamically

ai18439b

1398/1749
RM0090 Rev 18
ST

--- Page 34 ---

RM0090
USB on-the-go high-speed (OTG_HS)


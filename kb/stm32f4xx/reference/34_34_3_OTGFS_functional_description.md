### 34.3 OTG_FS functional description

Figure 386. OTG full-speed block diagram

Cortex® core

Power and clock controller
USB suspend
USB clock at 48 MHz

USB2.0 OTG FS core
System clock domain
USB clock domain
RAM bus

1.25 Kbyte USB data FIFOs

UTMIFS

OTG FS PHY
OTG_FS_DP
OTG_FS_DM
OTG_FS_ID
OTG_FS_VBUS
Universal serial bus
OTG_FS_SOF

MS19928V4

### 34.3.1 OTG pins

Table 196. OTG_FS input/output pins

| Signal name | Signal type | Description |
|---|---|---|
| OTG_FS_DP | Digital input/output | USB OTG D+ line |
| OTG_FS_DM | Digital input/output | USB OTG D- line |
| OTG_FS_ID | Digital input | USB OTG ID |
| OTG_FS_VBUS | Analog input | USB OTG VBUS |
| OTG_FS_SOF | Digital output | USB OTG Start Of Frame (visibility) |


### 34.3.2 OTG full-speed core

The USB OTG FS receives the 48 MHz ±0.25% clock from the reset and clock controller (RCC), via an external quartz. The USB clock is used for driving the 48 MHz domain at full-speed (12 Mbit/s) and must be enabled prior to configuring the OTG FS core.

The CPU reads and writes from/to the OTG FS core registers through the AHB peripheral bus. It is informed of USB events through the single USB OTG interrupt line described in Section 34.15: OTG_FS interrupts.

STI
RM0090 Rev 18
1243/1749

--- Page 74 ---

**USB on-the-go full-speed (OTG_FS)**
RM0090

The CPU submits data over the USB by writing 32-bit words to dedicated OTG_FS locations (push registers). The data are then automatically stored into Tx-data FIFOs configured within the USB data RAM. There is one Tx-FIFO push register for each in-endpoint (peripheral mode) or out-channel (host mode).

The CPU receives the data from the USB by reading 32-bit words from dedicated OTG_FS addresses (pop registers). The data are then automatically retrieved from a shared Rx-FIFO configured within the 1.25 KB USB data RAM. There is one Rx-FIFO pop register for each out-endpoint or in-channel.

The USB protocol layer is driven by the serial interface engine (SIE) and serialized over the USB by the full/low-speed transceiver module within the on-chip physical layer (PHY).

### 34.3.3 Full-speed OTG PHY

The embedded full-speed OTG PHY is controlled by the OTG FS core and conveys USB control & data signals through the full-speed subset of the UTMI+ Bus (UTMIFS). It provides the physical support to USB connectivity.

The full-speed OTG PHY includes the following components:

*   FS/LS transceiver module used by both host and device. It directly drives transmission and reception on the single-ended USB lines.
*   integrated ID pull-up resistor used to sample the ID line for A/B device identification.
*   DP/DM integrated pull-up and pull-down resistors controlled by the OTG_FS core depending on the current role of the device. As a peripheral, it enables the DP pull-up resistor to signal full-speed peripheral connections as soon as VBUS is sensed to be at a valid level (B-session valid). In host mode, pull-down resistors are enabled on both DP/DM. Pull-up and pull-down resistors are dynamically switched when the device's role is changed via the host negotiation protocol (HNP).
*   Pull-up/pull-down resistor ECN circuit. The DP pull-up consists of 2 resistors controlled separately from the OTG_FS as per the resistor Engineering Change Notice applied to USB Rev2.0. The dynamic trimming of the DP pull-up strength allows for better noise rejection and Tx/Rx signal quality.
*   VBUS sensing comparators with hysteresis used to detect VBUS Valid, A-B Session Valid and session-end voltage thresholds. They are used to drive the session request protocol (SRP), detect valid startup and end-of-session conditions, and constantly monitor the VBUS supply during USB operations.
*   VBUS pulsing method circuit used to charge/discharge VBUS through resistors during the SRP (weak drive).

**Caution:** To guarantee a correct operation for the USB OTG FS peripheral, the AHB frequency should be higher than 14.2 MHz.

1244/1749
RM0090 Rev 18

--- Page 75 ---

RM0090
USB on-the-go full-speed (OTG_FS)


### 35.3 OTG_HS functional description

*Figure 410* shows the OTG_HS interface block diagram.

ST
RM0090 Rev 18
1383/1749

--- Page 19 ---

USB on-the-go high-speed (OTG_HS)
RM0090

Figure 410. USB OTG interface block diagram

CPU
Memory
Peripheral 1
Peripheral 2
AHB (application bus)
AHB master interface
AHB slave interface
OTG_HS (USB OTG HS core)
Data FIFO RAM interface
Data FIFO Single-port RAM (SPRAM)
OTG FS PHY transceiver
serial
OTG detections
EXTI
NVIC
Interrupt: async wakeup
Interrupt: EP1 out
Interrupt: EP1 in
Interrupt: global
ULPI interface (12 pins)
ULPI PHY (external component)
ULPI_CK;
ULPI_DIR;
ULPI_STP;
ULPI_NXT;
ULPI_D0-7
USB2.0 (D+/D-)
OTG_HS_DP
OTG_HS_DM
OTG_HS_ID
OTG_HS_VBUS
OTG_HS_SOF

MSv43325V1

1. The USB DMA cannot directly address the internal Flash memory.

### 35.3.1 OTG pins

Table 206. OTG_HS input/output pins

| Signal name | Signal type | Description |
|---|---|---|
| OTG_HS_DP | Digital input/output | USB OTG D+ line |
| OTG_HS_DM | Digital input/output | USB OTG D- line |
| OTG_HS_ID | Digital input | USB OTG ID |
| OTG_HS_VBUS | Analog input | USB OTG VBUS |
| OTG_HS_SOF | Digital output | USB OTG Start Of Frame (visibility) |
| OTG_HS_ULPI_CK | Digital input | USB OTG ULPI clock |
| OTG_HS_ULPI_DIR | Digital input | USB OTG ULPI data bus direction control |
| OTG_HS_ULPI_STP | Digital output | USB OTG ULPI data stream stop |
| OTG_HS_ULPI_NXT | Digital input | USB OTG ULPI next data stream request |
| OTG_HS_ULPI_D[0..7] | Digital input/output | USB OTG ULPI 8-bit bi-directional data bus |


### 35.3.2 High-speed OTG PHY

The USB OTG HS core embeds an ULPI interface to connect an external HS phy.

1384/1749
RM0090 Rev 18

--- Page 20 ---

RM0090
USB on-the-go high-speed (OTG_HS)

### 35.3.3 Embedded Full-speed OTG PHY

The full-speed OTG PHY includes the following components:

*   FS/LS transceiver module used by both host and Device. It directly drives transmission and reception on the single-ended USB lines.
*   Integrated ID pull-up resistor used to sample the ID line for A/B Device identification.
*   DP/DM integrated pull-up and pull-down resistors controlled by the OTG_HS core depending on the current role of the device. As a peripheral, it enables the DP pull-up resistor to signal full-speed peripheral connections as soon as VBUS is sensed to be at a valid level (B-session valid). In host mode, pull-down resistors are enabled on both DP/DM. Pull-up and pull-down resistors are dynamically switched when the peripheral role is changed via the host negotiation protocol (HNP).
*   Pull-up/pull-down resistor ECN circuit
    The DP pull-up consists of 2 resistors controlled separately from the OTG_HS as per the resistor Engineering Change Notice applied to USB Rev2.0. The dynamic trimming of the DP pull-up strength allows to achieve a better noise rejection and Tx/Rx signal quality.
*   VBUS sensing comparators with hysteresis used to detect VBUS_VALID, A-B Session Valid and session-end voltage thresholds. They are used to drive the session request protocol (SRP), detect valid startup and end-of-session conditions, and constantly monitor the VBUS supply during USB operations.
*   VBUS pulsing method circuit used to charge/discharge VBUS through resistors during the SRP (weak drive).

**Caution:** To guarantee a correct operation for the USB OTG HS peripheral, the AHB frequency should be higher than 30 MHz.


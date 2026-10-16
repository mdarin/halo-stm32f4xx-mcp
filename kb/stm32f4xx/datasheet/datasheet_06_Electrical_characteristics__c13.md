> [DATASHEET ch.6 §6] Electrical characteristics

### 6.3.19 Communications interfaces**

**6.3.19 Communications interfaces**

**I²C interface characteristics**

The I²C interface meets the timings requirements of the I²C-bus specification and user manual rev. 03 for:

*   Standard-mode (Sm): with a bit rate up to 100 kbit/s
*   Fast-mode (Fm): with a bit rate up to 400 kbit/s.

The I²C timings requirements are specified by design when the I2C peripheral is properly configured (refer to RM0090 reference manual).

The SDA and SCL I/O requirements are met with the following restrictions: the SDA and SCL I/O pins are not “true” open-drain. When configured as open-drain, the PMOS connected between the I/O pin and V<sub>DD</sub> is disabled, but is still present. Refer to Section 6.3.16: I/O port characteristics for more details on the I²C I/O characteristics.

All I²C SDA and SCL I/Os embed an analog filter. Refer to the table below for the analog filter characteristics:

**Table 54. I2C analog filter characteristics(1)**

| Symbol | Parameter | Min | Max | Unit |
|---|---|---|---|---|
| tAF | Maximum pulse width of spikes that are suppressed by the analog filter | 50(2) | 260(3) | ns |


1. Specified by design.
2. Spikes with widths below t<sub>AF(min)</sub> are filtered.
3. Spikes with widths above t<sub>AF(max)</sub> are not filtered

ST
DS8626 Rev 12
123/206

--- Page 124 ---

**Electrical characteristics**
STM32F405xx, STM32F407xx

**SPI interface characteristics**

Unless otherwise specified, the parameters given in Table 55 for SPI are derived from tests performed under the ambient temperature, fPCLKx frequency and VDD supply voltage conditions summarized in Table 14 with the following configuration:

*   Output speed is set to OSPEEDRy[1:0] = 10
*   Capacitive load C = 30 pF
*   Measurement points are done at CMOS levels: 0.5 VDD

Refer to Section 6.3.16: I/O port characteristics for more details on the input/output alternate function characteristics (NSS, SCK, MOSI, MISO).

**Table 55. SPI dynamic characteristics(1)**

| Symbol | Parameter | Conditions | Min | Typ | Max | Unit |
|---|---|---|---|---|---|---|
| fSCK | SPI clock frequency | Master mode, SPI1, 2.7V < VDD < 3.6V | - | 42 | MHz |  |
| Slave mode, SPI1, 2.7V < VDD < 3.6V | 42 |  |  |  |  |  |
| 1/tcl(SCK) | Master mode, SPI1/2/3, 1.7V < VDD < 3.6V | - | 21 |  |  |  |
| Slave mode, SPI1/2/3, 1.7V < VDD < 3.6V | 21 |  |  |  |  |  |
| Duty(SCK) | Duty cycle of SPI clock frequency | Slave mode | 30 | 50 | 70 | % |


124/206
DS8626 Rev 12

--- Page 125 ---

STM32F405xx, STM32F407xx
Electrical characteristics

Table 55. SPI dynamic characteristics(1) (continued)

| Symbol | Parameter | Conditions | Min | Typ | Max | Unit |
|---|---|---|---|---|---|---|
| tw(SCKH) | SCK high and low time | Master mode, SPI presc = 2, 2.7V < VDD < 3.6V | TPCLK-0.5 | TPCLK | TPCLK+0.5 | ns |
| tw(SCKL) | Master mode, SPI presc = 2, 1.7V < VDD < 3.6V | TPCLK-2 | TPCLK | TPCLK+2 |  |  |
| tsu(NSS) | NSS setup time | Slave mode, SPI presc = 2 | 4 x TPCLK | - |  |  |
| th(NSS) | NSS hold time | Slave mode, SPI presc = 2 | 2 x TPCLK |  |  |  |
| tsu(MI) | Data input setup time | Master mode | 6.5 | - |  |  |
| tsu(SI) | Slave mode | 2.5 | - |  |  |  |
| th(MI) | Data input hold time | Master mode | 2.5 | - |  |  |
| th(SI) | Slave mode | 4 | - |  |  |  |
| ta(SO)(2) | Data output access time | Slave mode, SPI presc = 2 | 0 | - | 4 x TPCLK |  |
| tdis(SO)(3) | Data output disable time | Slave mode, SPI1, 2.7V < VDD < 3.6V | 0 | - | 7.5 |  |
| Slave mode, SPI1/2/3 1.7V < VDD < 3.6V | 0 | - | 16.5 |  |  |  |
| tv(SO)th(SO) | Data output valid/hold time | Slave mode (after enable edge), SPI1, 2.7V < VDD < 3.6V | - | 11 | 13 |  |
| Slave mode (after enable edge), SPI2/3, 2.7V < VDD < 3.6V | - | 12 | 16.5 |  |  |  |
| Slave mode (after enable edge), SPI1, 1.7V < VDD < 3.6V | - | 15.5 | 19 |  |  |  |
| Slave mode (after enable edge), SPI2/3, 1.7V < VDD < 3.6V | - | 18 | 20.5 |  |  |  |
| tv(MO) | Data output valid time | Master mode (after enable edge), SPI1, 2.7V < VDD < 3.6V | - | 2.5 |  |  |
| Master mode (after enable edge), SPI1/2/3, 1.7V < VDD < 3.6V | - | 4.5 |  |  |  |  |
| th(MO) | Data output hold time | Master mode (after enable edge) | 0 | - |  |  |


1. Evaluated by characterization - not tested in production.
2. Min time is for the minimum time to drive the output and the max time is for the maximum time to validate the data.
3. Min time is for the minimum time to invalidate the output and the max time is for the maximum time to put the data in Hi-Z.

ST
DS8626 Rev 12
125/206

--- Page 126 ---

**Electrical characteristics**
**STM32F405xx, STM32F407xx**

**Figure 39. SPI timing diagram - slave mode and CPHA = 0**

NSS input
SCK input
CPHA=0
CPOL=0
CPHA=0
CPOL=1
MISO output
MOSI input
t_su(NSS)
t_w(SCKH)
t_c(SCK)
t_h(NSS)
t_su(SO)
t_w(SCKL)
t_v(SO)
t_h(SO)
t_dis(SO)
First bit OUT
Next bits OUT
Last bit OUT
t_su(SI)
t_h(SI)
First bit IN
Next bits IN
Last bit IN
MSv41658V2

**Figure 40. SPI timing diagram - slave mode and CPHA = 1**

NSS input
SCK input
CPHA=1
CPOL=0
CPHA=1
CPOL=1
MISO output
MOSI input
t_su(NSS)
t_w(SCKH)
t_c(SCK)
t_h(NSS)
t_su(SO)
t_w(SCKL)
t_v(SO)
t_h(SO)
t_dis(SO)
First bit OUT
Next bits OUT
Last bit OUT
t_su(SI)
t_h(SI)
First bit IN
Next bits IN
Last bit IN
MSv41659V2

126/206
DS8626 Rev 12
ST

--- Page 127 ---

STM32F405xx, STM32F407xx
Electrical characteristics

Figure 41. SPI timing diagram - master mode

High
NSS input
SCK output
CPHA=0
CPOL=0
SCK output
CPHA=0
CPOL=1
SCK output
CPHA=1
CPOL=0
SCK output
CPHA=1
CPOL=1
MISO input
First bit IN
Next bits IN
Last bit IN
MOSI output
First bit OUT
Next bits OUT
Last bit OUT
tC(SCK)
tw(SCKH)
tw(SCKL)
tsu(MI)
th(MI)
tv(MO)
th(MO)
MSv72626V1

ST
DS8626 Rev 12
127/206

--- Page 128 ---

Electrical characteristics
STM32F405xx, STM32F407xx

**I²S interface characteristics**

Unless otherwise specified, the parameters given in Table 56 for the i²S interface are derived from tests performed under the ambient temperature, fPCLKx frequency and VDD supply voltage conditions summarized in Table 14, with the following configuration:

*   Output speed is set to OSPEEDRy[1:0] = 10
*   Capacitive load C = 30 pF
*   Measurement points are done at CMOS levels: 0.5 VDD

Refer to Section 6.3.16: I/O port characteristics for more details on the input/output alternate function characteristics (CK, SD, WS).

Table 56. I²S dynamic characteristics(1)

| Symbol | Parameter | Conditions | Min | Max | Unit |
|---|---|---|---|---|---|
| fMCK | I²S main clock output | - | 256 x 8K | 256 x FS(2) | MHz |
| fCK | I²S clock frequency | Master data: 32 bits | - | 64 x FS | MHz |
| Slave data: 32 bits | - | 64 x FS |  |  |  |
| DCK | I²S clock frequency duty cycle | Slave receiver | 30 | 70 | % |
| tv(WS) | WS valid time | Master mode | 0 | 6 | ns |
| th(WS) | WS hold time | Master mode | 0 | - |  |
| tsu(WS) | WS setup time | Slave mode | 1 | - |  |
| th(WS) | WS hold time | Slave mode | 0 | - |  |
| tsu(SD_MR) | Data input setup time | Master receiver | 7.5 | - |  |
| tsu(SD_SR) | Slave receiver | 2 | - |  |  |
| th(SD_MR) | Data input hold time | Master receiver | 0 | - |  |
| th(SD_SR) | Slave receiver | 0 | - |  |  |
| tv(SD_ST) | Data output valid time | Slave transmitter (after enable edge) | - | 27 |  |
| tv(SD_MT) | Master transmitter (after enable edge) | - | 20 |  |  |
| th(SD_MT) | Data output hold time | Master transmitter (after enable edge) | 2.5 | - |  |


1. Evaluated by characterization - not tested in production.
2. The maximum value of 256 x FS is 42 MHz (APB1 maximum frequency).

**Note:** Refer to the I²S section of RM0090 reference manual for more details on the sampling frequency (FS). fMCK, fCK, and DCK values reflect only the digital peripheral behavior. The value of these parameters might be slightly impacted by the source clock accuracy. DCK depends mainly on the value of ODD bit. The digital contribution leads to a minimum value of I2SDIV / (2 x I2SDIV + ODD) and a maximum value of (I2SDIV + ODD) / (2 x I2SDIV + ODD). FS maximum value is supported for each mode/condition.

128/206
DS8626 Rev 12
ST

--- Page 129 ---

STM32F405xx, STM32F407xx
Electrical characteristics

Figure 42. I²S slave timing diagram (Philips protocol)

ai14881b

1. LSB transmit/receive of the previously transmitted byte. No LSB transmit/receive is sent before the first byte.

Figure 43. I²S master timing diagram (Philips protocol)(1)

ai14884b

1. Evaluated by characterization - not tested in production.
2. LSB transmit/receive of the previously transmitted byte. No LSB transmit/receive is sent before the first byte.

ST
DS8626 Rev 12
129/206

--- Page 130 ---

Electrical characteristics
STM32F405xx, STM32F407xx

USB OTG FS characteristics
This interface is present in both the USB OTG HS and USB OTG FS controllers.

Table 57. USB OTG FS startup time
| Symbol | Parameter | Max | Unit |
|---|---|---|---|
| tSTARTUP(1) | USB OTG FS transceiver startup time | 1 | µs |


1. Specified by design.

Table 58. USB OTG FS DC electrical characteristics
| Symbol | Parameter | Conditions | Min.(1) | Typ. | Max.(1) | Unit |  |
|---|---|---|---|---|---|---|---|
| Input levels | VDD | USB OTG FS operating voltage | - | 3.0(2) | - | 3.6 | V |
| VDI(3) | Differential input sensitivity | I(USB_FS_DP/DM, USB_HS_DP/DM) | 0.2 | - | V |  |  |
| VCM(3) | Differential common mode range | Includes VDI range | 0.8 | - | 2.5 |  |  |
| VSE(3) | Single ended receiver threshold | - | 1.3 | - | 2.0 |  |  |
| Output levels | VOL | Static output level low | RL of 1.5 kΩ to 3.6 V(4) | - | 0.3 | V |  |
| VOH | Static output level high | RL of 15 kΩ to VSS(4) | 2.8 | - | 3.6 |  |  |
| RPD | PA11, PA12, PB14, PB15 (USB_FS_DP/DM, USB_HS_DP/DM) | VIN = VDD | 17 | 21 | 24 | kΩ |  |
| PA9, PB13 (OTG_FS_VBUS, OTG_HS_VBUS) | 0.65 | 1.1 | 2.0 |  |  |  |  |
| RPU | PA12, PB15 (USB_FS_DP, USB_HS_DP) | VIN = VSS | 1.5 | 1.8 | 2.1 |  |  |
| PA9, PB13 (OTG_FS_VBUS, OTG_HS_VBUS) | VIN = VSS | 0.25 | 0.37 | 0.55 |  |  |  |


1. All the voltages are measured from the local ground potential.
2. The STM32F405xx and STM32F407xx USB OTG FS functionality is ensured down to 2.7 V but not the full USB OTG FS electrical characteristics which are degraded in the 2.7-to-3.0 V V<sub>DD</sub> voltage range.
3. Specified by design.
4. R<sub>L</sub> is the load connected on the USB OTG FS drivers

130/206
DS8626 Rev 12

--- Page 131 ---

STM32F405xx, STM32F407xx
Electrical characteristics

Figure 44. USB OTG FS timings: definition of data signal rise and fall time

ai14137b

Table 59. USB OTG FS electrical characteristics(1)

| Driver characteristics |  |  |  |  |  |
|---|---|---|---|---|---|
| Symbol | Parameter | Conditions | Min | Max | Unit |
| tr | Rise time(2) | CL = 50 pF | 4 | 20 | ns |
| tf | Fall time(2) | CL = 50 pF | 4 | 20 | ns |
| trfm | Rise/ fall time matching | tr/tf | 90 | 110 | % |
| VCRS | Output signal crossover voltage | - | 1.3 | 2.0 | V |


1. Specified by design.
2. Measured from 10% to 90% of the data signal. For more detailed informations, please refer to USB Specification - Chapter 7 (version 2.0).

**USB HS characteristics**

Unless otherwise specified, the parameters given in Table 62 for ULPI are derived from tests performed under the ambient temperature, f<sub>HCLK</sub> frequency summarized in Table 61 and V<sub>DD</sub> supply voltage conditions summarized in Table 60, with the following configuration:
* Output speed is set to OSPEEDRy[1:0] = 10
* Capacitive load C = 30 pF
* Measurement points are done at CMOS levels: 0.5V<sub>DD</sub>.

Refer to Section Section 6.3.16: I/O port characteristics for more details on the input/output characteristics.

Table 60. USB HS DC electrical characteristics

| Symbol | Parameter | Min.(1) | Max.(1) | Unit |  |
|---|---|---|---|---|---|
| Input level | VDD | USB OTG HS operating voltage | 2.7 | 3.6 | V |


1. All the voltages are measured from the local ground potential.

Table 61. USB HS clock timing parameters(1)

| Parameter | Symbol | Min | Nominal | Max | Unit |  |
|---|---|---|---|---|---|---|
| fHCLK value to guarantee proper operation of USB HS interface | - | 30 | - | MHz |  |  |
| Frequency (first transition) | 8-bit ±10% | FSTART_8BIT | 54 | 60 | 66 | MHz |


ST
DS8626 Rev 12
131/206

--- Page 132 ---

**Electrical characteristics**
**STM32F405xx, STM32F407xx**

**Table 61. USB HS clock timing parameters(1)**

| Parameter | Symbol | Min | Nominal | Max | Unit |  |
|---|---|---|---|---|---|---|
| Frequency (steady state) ±500 ppm | FSTEADY | 59.97 | 60 | 60.03 | MHz |  |
| Duty cycle (first transition) | 8-bit ±10% | DSTART_8BIT | 40 | 50 | 60 | % |
| Duty cycle (steady state) ±500 ppm | DSTEADY | 49.975 | 50 | 50.025 | % |  |
| Time to reach the steady state frequency and duty cycle after the first transition | TSTEADY | - | 1.4 | ms |  |  |
| Clock startup time after the de-assertion of SuspendM | Peripheral | TSTART_DEV | - | 5.6 | ms |  |
| Host | TSTART_HOST | - |  |  |  |  |
| PHY preparation time after the first transition of the input clock | TPREP | - | µs |  |  |  |


1. Specified by design.

**Table 62. ULPI timing**

| Parameter | Symbol | Value(1) | Unit |  |
|---|---|---|---|---|
| Min. | Max. |  |  |  |
| Control in (ULPI_DIR) setup time | tSC | - | 2.0 | ns |
| Control in (ULPI_NXT) setup time | - | 1.5 |  |  |
| Control in (ULPI_DIR, ULPI_NXT) hold time | tHC | 0 | - |  |
| Data in setup time | tSD | - | 2.0 |  |
| Data in hold time | tHD | 0 | - |  |
| Control out (ULPI_STP) setup time and hold time | tDC | - | 9.2 |  |
| Data out available from clock rising edge | tDD | - | 10.7 |  |


1. V<sub>DD</sub> = 2.7 V to 3.6 V and T<sub>A</sub> = -40 to 85 °C.

**Figure 45. ULPI timing diagram**

Clock
t<sub>SC</sub>
t<sub>HC</sub>
Control In (ULPI_DIR, ULPI_NXT)
t<sub>SD</sub>
t<sub>HD</sub>
data In (8-bit)
t<sub>DC</sub>
t<sub>DC</sub>
Control out (ULPI_STP)
t<sub>DD</sub>
data out (8-bit)
ai17361c

132/206
DS8626 Rev 12
ST

--- Page 133 ---

STM32F405xx, STM32F407xx
Electrical characteristics

**Ethernet characteristics**

Unless otherwise specified, the parameters given in Table 64, Table 65 and Table 66 for SMI, RMII and MII are derived from tests performed under the ambient temperature, fHCLK frequency summarized in Table 14 and VDD supply voltage conditions summarized in Table 63, with the following configuration:

*   Output speed is set to OSPEEDRy[1:0] = 10
*   Capacitive load C = 30 pF
*   Measurement points are done at CMOS levels: 0.5VDD.

Refer to Section 6.3.16: I/O port characteristics for more details on the input/output characteristics.

Table 63. Ethernet DC electrical characteristics

| Symbol | Parameter | Min.(1) | Max.(1) | Unit |  |
|---|---|---|---|---|---|
| Input level | VDD | Ethernet operating voltage | 2.7 | 3.6 | V |


1. All the voltages are measured from the local ground potential.

Table 64 gives the list of Ethernet MAC signals for the SMI (station management interface) and Figure 46 shows the corresponding timing diagram.

Figure 46. Ethernet SMI timing diagram

MS31384V1

Table 64. Dynamic characteristics: Ethernet MAC signals for SMI(1)

| Symbol | Parameter | Min | Typ | Max | Unit |
|---|---|---|---|---|---|
| tMDC | MDC cycle time(2.38 MHz) | 411 | 420 | 425 | ns |
| Td(MDIO) | Write data valid time | 6 | 10 | 13 |  |
| tsu(MDIO) | Read data setup time | 12 | - |  |  |
| th(MDIO) | Read data hold time | 0 | - |  |  |


1. Evaluated by characterization - not tested in production.

Table 65 gives the list of Ethernet MAC signals for the RMII and Figure 47 shows the corresponding timing diagram.

STI
DS8626 Rev 12
133/206

--- Page 134 ---

Electrical characteristics
STM32F405xx, STM32F407xx

Figure 47. Ethernet RMII timing diagram

ai15667

Table 65. Dynamic characteristics: Ethernet MAC signals for RMII

| Symbol | Rating | Min | Typ | Max | Unit |
|---|---|---|---|---|---|
| tsu(RXD) | Receive data setup time | 2 | - | ns |  |
| tih(RXD) | Receive data hold time | 1 | - | ns |  |
| tsu(CRS) | Carrier sense set-up time | 0.5 | - | ns |  |
| tih(CRS) | Carrier sense hold time | 2 | - | ns |  |
| td(TXEN) | Transmit enable valid delay time | 8 | 9.5 | 11 | ns |
| td(TXD) | Transmit data valid delay time | 8.5 | 10 | 11.5 | ns |


Table 66 gives the list of Ethernet MAC signals for MII and Figure 47 shows the corresponding timing diagram.

Figure 48. Ethernet MII timing diagram

ai15668

134/206
DS8626 Rev 12

--- Page 135 ---

STM32F405xx, STM32F407xx
Electrical characteristics

Table 66. Dynamic characteristics: Ethernet MAC signals for MII(1)

| Symbol | Parameter | Min | Typ | Max | Unit |
|---|---|---|---|---|---|
| tsu(RXD) | Receive data setup time | 9 | - | ns |  |
| tih(RXD) | Receive data hold time | 10 | - |  |  |
| tsu(DV) | Data valid setup time | 9 | - |  |  |
| tih(DV) | Data valid hold time | 8 | - |  |  |
| tsu(ER) | Error setup time | 6 | - |  |  |
| tih(ER) | Error hold time | 8 | - |  |  |
| td(TXEN) | Transmit enable valid delay time | 0 | 10 | 14 |  |
| td(TXD) | Transmit data valid delay time | 0 | 10 | 15 |  |


1. Evaluated by characterization - not tested in production.

> [DATASHEET ch.6 §6] Electrical characteristics

### 6.3.20 CAN (controller area network) interface

### 6.3.20 CAN (controller area network) interface

Refer to Section 6.3.16: I/O port characteristics for more details on the input/output alternate function characteristics (CANTX and CANRX).


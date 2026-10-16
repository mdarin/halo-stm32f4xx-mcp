> [DATASHEET ch.6 §6] Electrical characteristics

### 6.3.13 EMC characteristics**

**6.3.13 EMC characteristics**

Susceptibility tests are performed on a sample basis during device characterization.

**Functional EMS (electromagnetic susceptibility)**

While a simple application is executed on the device (toggling 2 LEDs through I/O ports), the device is stressed by two electromagnetic events until a failure occurs. The failure is indicated by the LEDs:

*   **Electrostatic discharge (ESD)** (positive and negative) is applied to all device pins until a functional disturbance occurs. This test is compliant with the IEC 61000-4-2 standard.
*   **FTB**: A burst of fast transient voltage (positive and negative) is applied to V<sub>DD</sub> and V<sub>SS</sub> through a 100 pF capacitor, until a functional disturbance occurs. This test is compliant with the IEC 61000-4-4 standard.

112/206
DS8626 Rev 12
ST

--- Page 113 ---

**STM32F405xx, STM32F407xx**
**Electrical characteristics**

A device reset allows normal operations to be resumed.
The test results are given in Table 43. They are based on the EMS levels and classes defined in application note AN1709.

**Table 43. EMS characteristics**

| Symbol | Parameter | Conditions | Level/Class |
|---|---|---|---|
| VFESD | Voltage limits to be applied on any I/O pin to induce a functional disturbance | VDD = 3.3 V, LQFP176, TA = +25 °C, fHCLK = 168 MHz, conforms to IEC 61000-4-2 | 2B |
| VEFTB | Fast transient voltage burst limits to be applied through 100 pF on VDD and VSS pins to induce a functional disturbance | VDD = 3.3 V, LQFP176, TA = +25 °C, fHCLK = 168 MHz, conforms to IEC 61000-4-2 | 4A |


**Designing hardened software to avoid noise problems**

EMC characterization and optimization are performed at component level with a typical application environment and simplified MCU software. It should be noted that good EMC performance is highly dependent on the user application and the software in particular.

Therefore it is recommended that the user applies EMC software optimization and prequalification tests in relation with the EMC level requested for his application.

**Software recommendations**

The software flowchart must include the management of runaway conditions such as:

*   Corrupted program counter
*   Unexpected reset
*   Critical Data corruption (control registers...)

**Prequalification trials**

Most of the common failures (unexpected reset and program counter corruption) can be reproduced by manually forcing a low state on the NRST pin or the Oscillator pins for 1 second.

To complete these trials, ESD stress can be applied directly on the device, over the range of specification values. When unexpected behavior is detected, the software can be hardened to prevent unrecoverable errors occurring (see application note AN1015).

ST
DS8626 Rev 12
113/206

--- Page 114 ---

Electrical characteristics
STM32F405xx, STM32F407xx

Electromagnetic Interference (EMI)
The electromagnetic field emitted by the device are monitored while a simple application, executing EEMBC code, is running. This emission test is compliant with SAE IEC61967-2 standard which specifies the test board and the pin loading.

Table 44. EMI characteristics for fHSE = 25 MH and fCPU = 168 MHz

| Symbol | Parameter | Conditions | Monitored frequency band | Value | Unit |
|---|---|---|---|---|---|
| SEMI | Peak(1) | VDD = 3.3 V, TA = 25 °C, LQFP176 package, conforming to SAE J1752/3 EEMBC, code running from flash memory with ART accelerator enabled | 0.1 to 30 MHz | 32 | dBµV |
| 30 to 130 MHz | 25 |  |  |  |  |
| 130 MHz to 1GHz | 29 |  |  |  |  |
| Level(2) |  | 0.1 MHz to 2 GHz | 4 | - |  |
| Peak(1) | VDD = 3.3 V, TA = 25 °C, LQFP176 package, conforming to SAE J1752/3 EEMBC, code running from flash memory with ART accelerator and PLL spread spectrum enabled | 0.1 to 30 MHz | 19 | dBµV |  |
| 30 to 130 MHz | 16 |  |  |  |  |
| 130 MHz to 1GHz | 18 |  |  |  |  |
| Level(2) |  | 0.1 MHz to 2 GHz | 3.5 | - |  |


1. Refer to AN1709 "EMI radiated test" chapter.
2. Refer to AN1709 "EMI level classification" chapter.

6.3.14 Absolute maximum ratings (electrical sensitivity)
Stresses above the absolute maximum ratings listed in Table 11: Voltage characteristics, Table 12: Current characteristics, and Table 13: Thermal characteristics may cause permanent damage to the device. These are stress ratings only and the functional operation of the device at these conditions is not implied. Exposure to maximum rating conditions for extended periods may affect device reliability. Device mission profile (application conditions) is compliant with JEDEC JESD47 Qualification Standard, extended mission profiles are available on demand.

Electrostatic discharge (ESD)
Electrostatic discharges (a positive then a negative pulse separated by 1 second) are applied to the pins of each sample according to each pin combination. The sample size depends on the number of supply pins in the device (3 parts × (n+1) supply pins). This test conforms to the JESD22-A114/C101 standard.

Table 45. ESD absolute maximum ratings

| Symbol | Ratings | Conditions | Class | Maximum value(1) | Unit |
|---|---|---|---|---|---|
| VESD(HBM) | Electrostatic discharge voltage (human body model) | TA = +25 °C conforming to JESD22-A114 | 2 | 2000(2) | V |
| VESD(CDM) | Electrostatic discharge voltage (charge device model) | TA = +25 °C conforming to ANSI/ESDA/JEDEC JS-002 | II | 250 |  |


1. Evaluated by characterization - not tested in production.

114/206
DS8626 Rev 12
ST

--- Page 115 ---

**STM32F405xx, STM32F407xx**

**Electrical characteristics**

2. On V_BAT pin, V_ESD(HBM) is limited to 1000 V.

**Static latchup**

Two complementary static tests are required on six parts to assess the latchup performance:

*   A supply overvoltage is applied to each power supply pin
*   A current injection is applied to each input, output and configurable I/O pin

These tests are compliant with EIA/JESD 78A IC latchup standard.

Table 46. Electrical sensitivities

| Symbol | Parameter | Conditions | Class |
|---|---|---|---|
| LU | Static latch-up class | T_A = +105 °C conforming to JESD78A | II level A |



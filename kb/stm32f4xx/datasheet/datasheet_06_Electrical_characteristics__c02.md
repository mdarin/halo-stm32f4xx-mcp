> [DATASHEET ch.6 §6] Electrical characteristics

### 6.1.6 Power supply scheme

## 6.1.6 Power supply scheme

Figure 21. Power supply scheme

MS19911V3

1. Each power supply pair must be decoupled with filtering ceramic capacitors as shown above. These capacitors must be placed as close as possible to, or below, the appropriate pins on the underside of the PCB to ensure the good functionality of the device.
2. To connect BYPASS_REG and PDR_ON pins, refer to Section 3.16: Voltage regulator and Table 3.15: Power supply supervisor.
3. The two 2.2 µF ceramic capacitors should be replaced by two 100 nF decoupling capacitors when the voltage regulator is OFF.
4. The 4.7 µF ceramic capacitor must be connected to one of the VDD pin.
5. VDDA=VDD and VSSA=VSS.

ST
DS8626 Rev 12
79/206

--- Page 80 ---

Electrical characteristics
STM32F405xx, STM32F407xx

6.1.7 Current consumption measurement

Figure 22. Current consumption measurement scheme

ai14126

6.2 Absolute maximum ratings

Stresses above the absolute maximum ratings listed in Table 11: Voltage characteristics, Table 12: Current characteristics, and Table 13: Thermal characteristics may cause permanent damage to the device. These are stress ratings only and functional operation of the device at these conditions is not implied. Exposure to maximum rating conditions for extended periods may affect device reliability. Device mission profile (application conditions) is compliant with JEDEC JESD47 Qualification Standard, extended mission profiles are available on demand.

Table 11. Voltage characteristics

| Symbol | Ratings | Min | Max | Unit |
|---|---|---|---|---|
| VDD–VSS | External main supply voltage (including VDDA, VDD)(1) | –0.3 | 4.0 | V |
| VIN | Input voltage on five-volt tolerant pin(2) | VSS–0.3 | VDD+4 |  |
| Input voltage on any other pin | VSS–0.3 | 4.0 |  |  |
| |ΔVDDx| | Variations between different VDD power pins | - | 50 | mV |
| |VSSX–VSS| | Variations between all the different ground pins including VREF- | - | 50 |  |
| VESD(HBM) | Electrostatic discharge voltage (human body model) | see Section 6.3.14: Absolute maximum ratings (electrical sensitivity) |  |  |


1. All main power (V<sub>DD</sub>, V<sub>DDA</sub>) and ground (V<sub>SS</sub>, V<sub>SSA</sub>) pins must always be connected to the external power supply, in the permitted range.
2. V<sub>IN</sub> maximum value must always be respected. Refer to Table 12 for the values of the maximum allowed injected current.

80/206
DS8626 Rev 12

--- Page 81 ---

STM32F405xx, STM32F407xx
Electrical characteristics

Table 12. Current characteristics
| Symbol | Ratings | Max. | Unit |
|---|---|---|---|
| IVDD | Total current into VDD power lines (source)(1) | 240 | mA |
| IVSS | Total current out of VSS ground lines (sink)(1) | 240 |  |
| IIO | Output current sunk by any I/O and control pin | 25 |  |
| Output current source by any I/Os and control pin | 25 |  |  |
| IINJ(PIN) (2) | Injected current on five-volt tolerant I/O(3) | -5/+0 |  |
| Injected current on any other pin(4) | ±5 |  |  |
| ΣIINJ(PIN) (4) | Total injected current (sum of all I/O and control pins)(5) | ±25 |  |


1. All main power (V<sub>DD</sub>, V<sub>DDA</sub>) and ground (V<sub>SS</sub>, V<sub>SSA</sub>) pins must always be connected to the external power supply, in the permitted range.
2. Negative injection disturbs the analog performance of the device. See note in Section 6.3.21: 12-bit ADC characteristics.
3. Positive injection is not possible on these I/Os. A negative injection is induced by V<sub>IN</sub><V<sub>SS</sub>. I<sub>INJ(PIN)</sub> must never be exceeded. Refer to Table 11 for the values of the maximum allowed input voltage.
4. A positive injection is induced by V<sub>IN</sub>>V<sub>DD</sub> while a negative injection is induced by V<sub>IN</sub><V<sub>SS</sub>. I<sub>INJ(PIN)</sub> must never be exceeded. Refer to Table 11 for the values of the maximum allowed input voltage.
5. When several inputs are submitted to a current injection, the maximum ΣI<sub>INJ(PIN)</sub> is the absolute sum of the positive and negative injected currents (instantaneous values).

Table 13. Thermal characteristics
| Symbol | Ratings | Value | Unit |
|---|---|---|---|
| TSTG | Storage temperature range | -65 to +150 | °C |
| TJ | Maximum junction temperature | 125 | °C |


6.3 Operating conditions
6.3.1 General operating conditions

Table 14. General operating conditions
| Symbol | Parameter | Conditions | Min | Typ | Max | Unit |
|---|---|---|---|---|---|---|
| fHCLK | Internal AHB clock frequency | VOS bit in PWR_CR register = 0(1) | 0 | - | 144 | MHz |
| VOS bit in PWR_CR register= 1 | 0 | - | 168 |  |  |  |
| fPCLK1 | Internal APB1 clock frequency | - | 0 | - | 42 |  |
| fPCLK2 | Internal APB2 clock frequency | - | 0 | - | 84 |  |
| VDD | Standard operating voltage | - | 1.8(2) | - | 3.6 | V |
| VDDA (3)(4) | Analog operating voltage (ADC limited to 1.2 M samples) | Must be the same potential as VDD(5) | 1.8(2) | - | 2.4 | V |
| Analog operating voltage (ADC limited to 1.4 M samples) | 2.4 | - | 3.6 |  |  |  |
| VBAT | Backup operating voltage | - | 1.65 | - | 3.6 | V |


ST
DS8626 Rev 12
81/206

--- Page 82 ---

Electrical characteristics
STM32F405xx, STM32F407xx

Table 14. General operating conditions (continued)

| Symbol | Parameter | Conditions | Min | Typ | Max | Unit |
|---|---|---|---|---|---|---|
| V12 | Regulator ON:1.2 V internal voltage onVCAP_1/VCAP_2 pins | VOS bit in PWR_CR register = 0(1)Max frequency 144MHz | 1.08 | 1.14 | 1.20 | V |
| VOS bit in PWR_CR register= 1Max frequency 168MHz | 1.20 | 1.26 | 1.32 | V |  |  |
| Regulator OFF:1.2 V external voltage must besupplied from external regulatoron VCAP_1/VCAP_2 pins | Max frequency 144MHz | 1.10 | 1.14 | 1.20 | V |  |
| Max frequency 168MHz | 1.20 | 1.26 | 1.30 | V |  |  |
| VIN | Input voltage on RST and FTpins(6) | 2 V ≤ VDD ≤ 3.6 V | –0.3 | - | 5.5 | V |
| VDD ≤ 2 V | –0.3 | - | 5.2 |  |  |  |
| Input voltage on TTa pins | - | –0.3 | - | VDDA+0.3 |  |  |
| Input voltage on B pin | - | 5.5 |  |  |  |  |
| PD | Power dissipation at TA = 85 °Cfor suffix 6 or TA = 105 °C forsuffix 7(7) | LQFP64 | - | 435 | mW |  |
| LQFP100 | - | 465 |  |  |  |  |
| LQFP144 | - | 500 |  |  |  |  |
| LQFP176 | - | 526 |  |  |  |  |
| UFBGA176 | - | 513 |  |  |  |  |
| WLCSP90 | - | 543 |  |  |  |  |
| TA | Ambient temperature for 6 suffixversion | Maximum power dissipation | –40 | - | 85 | °C |
| Low-power dissipation(8) | –40 | - | 105 |  |  |  |
| Ambient temperature for 7 suffixversion | Maximum power dissipation | –40 | - | 105 | °C |  |
| Low-power dissipation(8) | –40 | - | 125 |  |  |  |
| TJ | Junction temperature range | 6 suffix version | –40 | - | 105 | °C |
| 7 suffix version | –40 | - | 125 |  |  |  |


1. The average expected gain in power consumption when VOS = 0 compared to VOS = 1 is around 10% for the whole temperature range, when the system clock frequency is between 30 and 144 MHz.
2. V<sub>DD</sub>/V<sub>DDA</sub> minimum value of 1.7 V is obtained when the device operates in reduced temperature range, and with the use of an external power supply supervisor (refer to Section 3.15.2: Internal reset OFF).
3. When the ADC is used, refer to Table 67: ADC characteristics.
4. If V<sub>REF+</sub> pin is present, it must respect the following condition: V<sub>DDA</sub>-V<sub>REF+</sub> < 1.2 V.
5. It is recommended to power V<sub>DD</sub> and V<sub>DDA</sub> from the same source. A maximum difference of 300 mV between V<sub>DD</sub> and V<sub>DDA</sub> can be tolerated during power-up and power-down operation.
6. To sustain a voltage higher than V<sub>DD</sub>+0.3, the internal pull-up and pull-down resistors must be disabled.
7. If T<sub>A</sub> is lower, higher P<sub>D</sub> values are allowed as long as T<sub>J</sub> does not exceed T<sub>Jmax</sub>.
8. In low-power dissipation state, T<sub>A</sub> can be extended to this range as long as T<sub>J</sub> does not exceed T<sub>Jmax</sub>.

82/206
DS8626 Rev 12

--- Page 83 ---

STM32F405xx, STM32F407xx
Electrical characteristics

Table 15. Limitations depending on the operating power supply range

| Operating power supply range | ADC operation | Maximum flash memory access frequency with no wait state (fFlashmax) | Maximum flash memory access frequency with wait states(1) (2) | I/O operation | Clock output Frequency on I/O pins | Possible flash memory operations |
|---|---|---|---|---|---|---|
| VDD = 1.8 to 2.1 V(3) | Conversion time up to 1.2 Msps | 20 MHz(4) | 160 MHz with 7 wait states | – Degraded speed performance– No I/O compensation | up to 30 MHz | 8-bit erase and program operations only |
| VDD = 2.1 to 2.4 V | Conversion time up to 1.2 Msps | 22 MHz | 168 MHz with 7 wait states | – Degraded speed performance– No I/O compensation | up to 30 MHz | 16-bit erase and program operations |
| VDD = 2.4 to 2.7 V | Conversion time up to 2.4 Msps | 24 MHz | 168 MHz with 6 wait states | – Degraded speed performance– I/O compensation works | up to 48 MHz | 16-bit erase and program operations |
| VDD = 2.7 to 3.6 V(5) | Conversion time up to 2.4 Msps | 30 MHz | 168 MHz with 5 wait states | – Full-speed operation– I/O compensation works | – up to 60 MHz when VDD = 3.0 to 3.6 V– up to 48 MHz when VDD = 2.7 to 3.0 V | 32-bit erase and program operations |


1. It applies only when code executed from flash memory access, when code executed from RAM, no wait state is required.
2. Thanks to the ART accelerator and the 128-bit flash memory, the number of wait states given here does not impact the execution speed from flash memory since the ART accelerator allows to achieve a performance equivalent to 0 wait state program execution.
3. VDD/VDDA minimum value of 1.7 V is obtained when the device operates in reduced temperature range, and with the use of an external power supply supervisor (refer to Section 3.15.2: Internal reset OFF).
4. Prefetch is not available. Refer to AN3430 application note for details on how to adjust performance and power.
5. The voltage range for OTG USB FS can drop down to 2.7 V. However it is degraded between 2.7 and 3 V.

ST
DS8626 Rev 12
83/206

--- Page 84 ---

Electrical characteristics
STM32F405xx, STM32F407xx

**6.3.2** **V<sub>CAP_1</sub>/V<sub>CAP_2</sub> external capacitor**

Stabilization for the main regulator is achieved by connecting an external capacitor C<sub>EXT</sub> to the V<sub>CAP_1</sub>/V<sub>CAP_2</sub> pins. C<sub>EXT</sub> is specified in Table 16.

Figure 23. External capacitor C<sub>EXT</sub>

[Image of circuit diagram showing ESR, C, and R<sub>Leak</sub>]

1. Legend: ESR is the equivalent series resistance.

Table 16. V<sub>CAP_1</sub>/V<sub>CAP_2</sub> operating conditions<sup>(1)</sup>
| Symbol | Parameter | Conditions |
|---|---|---|
| CEXT | Capacitance of external capacitor | 2.2 µF |
| ESR | ESR of external capacitor | < 2 Ω |


1. When bypassing the voltage regulator, the two 2.2 µF V<sub>CAP</sub> capacitors are not required and should be replaced by two 100 nF decoupling capacitors.

**6.3.3** **Operating conditions at power-up / power-down (regulator ON)**

Subject to general operating conditions for T<sub>A</sub>.

Table 17. Operating conditions at power-up / power-down (regulator ON)
| Symbol | Parameter | Min | Max | Unit |
|---|---|---|---|---|
| tVDD | VDD rise time rate | 20 | ∞ | µs/V |
| VDD fall time rate | 20 | ∞ |  |  |


**6.3.4** **Operating conditions at power-up / power-down (regulator OFF)**

Subject to general operating conditions for T<sub>A</sub>.

Table 18. Operating conditions at power-up / power-down (regulator OFF)<sup>(1)</sup>
| Symbol | Parameter | Conditions | Min | Max | Unit |
|---|---|---|---|---|---|
| tVDD | VDD rise time rate | Power-up | 20 | ∞ | µs/V |
| VDD fall time rate | Power-down | 20 | ∞ |  |  |
| tVCAP | VCAP_1 and VCAP_2 rise time rate | Power-up | 20 | ∞ |  |
| VCAP_1 and VCAP_2 fall time rate | Power-down | 20 | ∞ |  |  |


1. To reset the internal logic at power-down, a reset must be applied on pin PA0 when V<sub>DD</sub> reach below minimum value of V<sub>12</sub>.

84/206
DS8626 Rev 12

--- Page 85 ---

STM32F405xx, STM32F407xx
Electrical characteristics


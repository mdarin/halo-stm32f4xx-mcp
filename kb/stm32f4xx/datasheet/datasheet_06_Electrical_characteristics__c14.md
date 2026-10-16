> [DATASHEET ch.6 §6] Electrical characteristics

### 6.3.21 12-bit ADC characteristics

### 6.3.21 12-bit ADC characteristics

Unless otherwise specified, the parameters given in Table 67 are derived from tests performed under the ambient temperature, fPCLK2 frequency and VDDA supply voltage conditions summarized in Table 14.

Table 67. ADC characteristics

| Symbol | Parameter | Conditions | Min | Typ | Max | Unit |
|---|---|---|---|---|---|---|
| VDDA | Power supply | - | 1.8(1) | - | 3.6 | V |
| VREF+ | Positive reference voltage | - | 1.8(1)(2)(3) | - | VDDA |  |
| VREF- | Negative reference voltage | - | 0 | - |  |  |
| fADC | ADC clock frequency | VDDA = 1.8(1)(3) to 2.4 V | 0.6 | 15 | 18 | MHz |
| VDDA = 2.4 to 3.6 V(3) | 0.6 | 30 | 36 | MHz |  |  |
| fTRIG(4) | External trigger frequency | fADC = 30 MHz, 12-bit resolution | - | 1764 | kHz |  |
| - | 17 | 1/fADC |  |  |  |  |
| VAIN | Conversion voltage range(5) | - | 0 (VSSA or VREF- tied to ground) | - | VREF+ | V |
| RAIN(4) | External input impedance | See Equation 1 for details | - | 50 | kΩ |  |
| RADC(4)(6) | Sampling switch resistance | - | 6 | kΩ |  |  |
| CADC(4) | Internal sample and hold capacitor | - | 4 | - | pF |  |


ST
DS8626 Rev 12
135/206

--- Page 136 ---

Electrical characteristics
STM32F405xx, STM32F407xx

Table 67. ADC characteristics (continued)

| Symbol | Parameter | Conditions | Min | Typ | Max | Unit |
|---|---|---|---|---|---|---|
| tlat(4) | Injection trigger conversion latency | fADC = 30 MHz | - | 0.100 | µs |  |
| - | 3(7) | 1/fADC |  |  |  |  |
| tlatr(4) | Regular trigger conversion latency | fADC = 30 MHz | - | 0.067 | µs |  |
| - | 2(7) | 1/fADC |  |  |  |  |
| tS(4) | Sampling time | fADC = 30 MHz | 0.100 | - | 16 | µs |
| - | 3 | - | 480 | 1/fADC |  |  |
| tSTAB(4) | Power-up time | - | 2 | 3 | µs |  |
| tCONV(4) | Total conversion time (including sampling time) | fADC = 30 MHz12-bit resolution | 0.50 | - | 16.40 | µs |
| fADC = 30 MHz10-bit resolution | 0.43 | - | 16.34 | µs |  |  |
| fADC = 30 MHz8-bit resolution | 0.37 | - | 16.27 | µs |  |  |
| fADC = 30 MHz6-bit resolution | 0.30 | - | 16.20 | µs |  |  |
| 9 to 492 (tS for sampling +n-bit resolution for successive approximation) | 1/fADC |  |  |  |  |  |
| fS(4) | Sampling rate(fADC = 30 MHz, and tS = 3 ADC cycles) | 12-bit resolutionSingle ADC | - | 2 | Msps |  |
| 12-bit resolutionInterleave Dual ADC mode | - | 3.75 | Msps |  |  |  |
| 12-bit resolutionInterleave Triple ADC mode | - | 6 | Msps |  |  |  |
| IVREF+(4) | ADC VREF+ DC current consumption in conversion mode | - | 300 | 500 | µA |  |
| IVDDA(4) | ADC VDDA DC current consumption in conversion mode | - | 1.6 | 1.8 | mA |  |


1. V<sub>DD</sub>/V<sub>DDA</sub> minimum value of 1.7 V is obtained when the device operates in reduced temperature range, and with the use of an external power supply supervisor (refer to Section 3.15.2: Internal reset OFF).
2. It is recommended to maintain the voltage difference between V<sub>REF+</sub> and V<sub>DDA</sub> below 1.8 V.
3. V<sub>DDA</sub> -V<sub>REF+</sub> < 1.2 V.
4. Evaluated by characterization - not tested in production.
5. V<sub>REF+</sub> is internally connected to V<sub>DDA</sub> and V<sub>REF-</sub> is internally connected to V<sub>SSA</sub>.
6. R<sub>ADC</sub> maximum value is given for V<sub>DD</sub>=1.8 V, and minimum value for V<sub>DD</sub>=3.3 V.
7. For external triggers, a delay of 1/f<sub>PCLK2</sub> must be added to the latency specified in Table 67.

136/206
DS8626 Rev 12

--- Page 137 ---

**STM32F405xx, STM32F407xx**

**Electrical characteristics**

**Equation 1: RAIN max formula**

$$R_{AIN} = \frac{(k - 0.5)}{f_{ADC} \times C_{ADC} \times \ln(2^{N+2})} - R_{ADC}$$

The formula above (**Equation 1**) is used to determine the maximum external impedance allowed for an error below 1/4 of LSB. N = 12 (from 12-bit resolution) and k is the number of sampling periods defined in the ADC_SMPR1 register.

**Table 68. ADC accuracy at fADC = 30 MHz**

| Symbol | Parameter | Test conditions | Typ | Max(1) | Unit |
|---|---|---|---|---|---|
| ET | Total unadjusted error | fPCLK2 = 60 MHz,fADC = 30 MHz, RAIN < 10 kΩVDDA = 1.8(2) to 3.6 V | ±2 | ±5 | LSB |
| EO | Offset error | ±1.5 | ±2.5 |  |  |
| EG | Gain error | ±1.5 | ±3 |  |  |
| ED | Differential linearity error | ±1 | ±2 |  |  |
| EL | Integral linearity error | ±1.5 | ±3 |  |  |


1. Evaluated by characterization - not tested in production.
2. VDD/VDDA minimum value of 1.7 V is obtained when the device operates in reduced temperature range, and with the use of an external power supply supervisor (refer to Section 3.15.2: *Internal reset OFF*).

**Note:**

**ADC accuracy vs. negative injection current:** injecting a negative current on any analog input pins should be avoided as this significantly reduces the accuracy of the conversion being performed on another analog input. It is recommended to add a Schottky diode (pin to ground) to analog pins which may potentially inject negative currents.

Any positive injection current within the limits specified for IINJ(PIN) and SINJ(PIN) in Section 6.3.16 does not affect the ADC accuracy.

**Figure 49. ADC accuracy characteristics**

(1) Example of an actual transfer curve
(2) Ideal transfer curve
(3) End-point correlation line

n = ADC resolution
ET = total unadjusted error: maximum deviation between the actual and ideal transfer curves
EO = offset error: maximum deviation between the first actual transition and the first ideal one
EG = gain error: deviation between the last ideal transition and the last actual one
ED = differential linearity error: maximum deviation between actual steps and the ideal one
EL = integral linearity error: maximum deviation between any actual transition and the end point correlation line

DS8626 Rev 12
137/206

--- Page 138 ---

Electrical characteristics
STM32F405xx, STM32F407xx

Figure 50. Typical connection diagram when using the ADC with FT/TT pins featuring analog switch function

MSv67871V3

1. Refer to Table 67: ADC characteristics for the values of RAIN, RADC and CADC.
2. Cparasitic represents the capacitance of the PCB (dependent on soldering and PCB layout quality) plus the pad capacitance (refer to Table 48: I/O static characteristics) A high Cparasitic value downgrades conversion accuracy. To remedy this, fADC should be reduced.
3. Refer to Table 48: I/O static characteristics.
4. Refer to Figure 21: Power supply scheme.

138/206
DS8626 Rev 12
ST

--- Page 139 ---

**STM32F405xx, STM32F407xx**
**Electrical characteristics**

**General PCB design guidelines**

Power supply decoupling should be performed as shown in Figure 51 or Figure 52, depending on whether VREF+ is connected to VDDA or not. The 10 nF capacitors should be ceramic (good quality). They should be placed them as close as possible to the chip.

**Figure 51. Power supply and reference decoupling (VREF+ not connected to VDDA)**

STM32F
VREF+ (1)
1 µF // 10 nF
VDDA
1 µF // 10 nF
VSSA/VREF- (1)

ai17535b

1. VREF+ and VREF- inputs are both available on UFBGA176. VREF+ is also available on LQFP100, LQFP144, and LQFP176. When VREF+ and VREF- are not available, they are internally connected to VDDA and VSSA.

ST
DS8626 Rev 12
139/206

--- Page 140 ---

**Electrical characteristics**
**STM32F405xx, STM32F407xx**

**Figure 52. Power supply and reference decoupling (VREF+ connected to VDDA)**

STM32F
VREF+/VDDA(1)
1 µF // 10 nF
VREF-/VSSA(1)
ai17536c

1. VREF+ and VREF- inputs are both available on UFBGA176. VREF+ is also available on LQFP100, LQFP144, and LQFP176. When VREF+ and VREF- are not available, they are internally connected to VDDA and VSSA.

> [DATASHEET ch.6 §6] Electrical characteristics

### 6.3.22 Temperature sensor characteristics**

**6.3.22 Temperature sensor characteristics**

**Table 69. Temperature sensor characteristics**

| Symbol | Parameter | Min | Typ | Max | Unit |
|---|---|---|---|---|---|
| TL(1) | VSENSE linearity with temperature | - | ±1 | ±2 | °C |
| Avg_Slope(1) | Average slope | - | 2.5 | - | mV/°C |
| V25(1) | Voltage at 25 °C | - | 0.76 | - | V |
| tSTART(2) | Startup time | - | 6 | 10 | µs |
| TS_temp(2) | ADC sampling time when reading the temperature (1 °C accuracy) | 10 | - | µs |  |


1. Evaluated by characterization - not tested in production.
2. Specified by design.

**Table 70. Temperature sensor calibration values**

| Symbol | Parameter | Memory address |
|---|---|---|
| TS_CAL1 | TS ADC raw data acquired at temperature of 30 °C, VDDA=3.3 V | 0x1FFF 7A2C - 0x1FFF 7A2D |
| TS_CAL2 | TS ADC raw data acquired at temperature of 110 °C, VDDA=3.3 V | 0x1FFF 7A2E - 0x1FFF 7A2F |


140/206
DS8626 Rev 12

--- Page 141 ---

STM32F405xx, STM32F407xx
Electrical characteristics

> [DATASHEET ch.6 §6] Electrical characteristics

### 6.3.23 VBAT monitoring characteristics

### 6.3.23 VBAT monitoring characteristics

**Table 71. VBAT monitoring characteristics**

| Symbol | Parameter | Min | Typ | Max | Unit |
|---|---|---|---|---|---|
| R | Resistor bridge for VBAT | - | 50 | - | KΩ |
| Q | Ratio on VBAT measurement | - | 2 | - |  |
| Er(1) | Error on Q | −1 | - | +1 | % |
| TS_vbat(2)(2) | ADC sampling time when reading the VBAT 1 mV accuracy | 5 | - | µs |  |


1. Specified by design.
2. Shortest sampling time can be determined in the application by multiple iterations.

> [DATASHEET ch.6 §6] Electrical characteristics

### 6.3.24 Embedded reference voltage

### 6.3.24 Embedded reference voltage

The parameters given in Table 72 are derived from tests performed under ambient temperature and VDD supply voltage conditions summarized in Table 14.

**Table 72. Embedded internal reference voltage**

| Symbol | Parameter | Conditions | Min | Typ | Max | Unit |
|---|---|---|---|---|---|---|
| VREFINT | Internal reference voltage | −40 °C < TA < +105 °C | 1.18 | 1.21 | 1.24 | V |
| TS_vrefint(1) | ADC sampling time when reading the internal reference voltage | - | 10 | - | µs |  |
| VRERINT_s(2) | Internal reference voltage spread over the temperature range | VDD = 3 V | - | 3 | 5 | mV |
| TCoeff(2) | Temperature coefficient | - | 30 | 50 | ppm/°C |  |
| tSTART(2) | Startup time | - | 6 | 10 | µs |  |


1. Shortest sampling time can be determined in the application by multiple iterations.
2. Specified by design.

**Table 73. Internal reference voltage calibration values**

| Symbol | Parameter | Memory address |
|---|---|---|
| VREFIN_CAL | Raw data acquired at temperature of 30 °C, VDDA=3.3 V | 0x1FFF 7A2A - 0x1FFF 7A2B |



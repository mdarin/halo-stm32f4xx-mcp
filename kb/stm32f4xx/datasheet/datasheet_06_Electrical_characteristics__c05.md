> [DATASHEET ch.6 §6] Electrical characteristics

### 6.3.7 Wakeup time from low-power mode**

**6.3.7 Wakeup time from low-power mode**

The wakeup times given in Table 29 is measured on a wakeup phase with a 16 MHz HSI RC oscillator. The clock source used to wake up the device depends from the current operating mode:

* Stop or Standby mode: the clock source is the RC oscillator
* Sleep mode: the clock source is the clock that was set before entering Sleep mode.

All timings are derived from tests performed under ambient temperature and V<sub>DD</sub> supply voltage conditions summarized in Table 14.

100/206
DS8626 Rev 12

--- Page 101 ---

STM32F405xx, STM32F407xx
Electrical characteristics

Table 29. Low-power mode wakeup timings

| Symbol | Parameter | Min(1) | Typ(1) | Max(1) | Unit |
|---|---|---|---|---|---|
| tWUSLEEP(2) | Wakeup from Sleep mode | - | 5 | - | CPU clock cycle |
| tWUSTOP(2) | Wakeup from Stop mode (regulator in Run mode and flash memory in Stop mode) | - | 13 | - | µs |
| Wakeup from Stop mode (regulator in low-power mode and flash memory in Stop mode) | - | 17 | 40 |  |  |
| Wakeup from Stop mode (regulator in Run mode and flash memory in Deep power-down mode) | - | 105 | - |  |  |
| Wakeup from Stop mode (regulator in low-power mode and flash memory in Deep power-down mode) | - | 110 | - |  |  |
| tWUSTDBY(2)(3) | Wakeup from Standby mode | 260 | 375 | 480 | µs |


1. Evaluated by characterization - not tested in production.
2. The wakeup times are measured from the wakeup event to the point in which the application code reads the first instruction.
3. t<sub>WUSTDBY</sub> minimum and maximum values are given at 105 °C and −45 °C, respectively.

6.3.8 External clock source characteristics

High-speed external user clock generated from an external source

The characteristics given in Table 30 result from tests performed using an high-speed external clock source, and under ambient temperature and supply voltage conditions summarized in Table 14.

Table 30. High-speed external user clock characteristics

| Symbol | Parameter | Conditions | Min | Typ | Max | Unit |
|---|---|---|---|---|---|---|
| fHSE_ext | External user clock source frequency(1) | - | 1 | - | 50 | MHz |
| VHSEH | OSC_IN input pin high level voltage | 0.7VDD | - | VDD | V |  |
| VHSEL | OSC_IN input pin low level voltage | VSS | - | 0.3VDD |  |  |
| tw(HSE)tw(HSE) | OSC_IN high or low time(1) | 5 | - | ns |  |  |
| tr(HSE)tf(HSE) | OSC_IN rise or fall time(1) | - | 10 |  |  |  |
| Cin(HSE) | OSC_IN input capacitance(1) | - | 5 | - | pF |  |
| DuCy(HSE) | Duty cycle | - | 45 | - | 55 | % |
| IL | OSC_IN Input leakage current | VSS ≤ VIN ≤ VDD | - | ±1 | µA |  |


1. Specified by design.

ST
DS8626 Rev 12
101/206

--- Page 102 ---

**Electrical characteristics**
STM32F405xx, STM32F407xx

**Low-speed external user clock generated from an external source**

The characteristics given in Table 31 result from tests performed using an low-speed external clock source, and under ambient temperature and supply voltage conditions summarized in Table 14.

**Table 31. Low-speed external user clock characteristics**

| Symbol | Parameter | Conditions | Min | Typ | Max | Unit |
|---|---|---|---|---|---|---|
| fLSE_ext | User External clock source frequency(1) | - | 32.768 | 1000 | kHz |  |
| VLSEH | OSC32_IN input pin high level voltage | 0.7VDD | - | VDD | V |  |
| VLSEL | OSC32_IN input pin low level voltage | VSS | - | 0.3VDD |  |  |
| tW(LSE)tf(LSE) | OSC32_IN high or low time(1) | 450 | - | ns |  |  |
| tr(LSE)tf(LSE) | OSC32_IN rise or fall time(1) | - | 50 |  |  |  |
| Cin(LSE) | OSC32_IN input capacitance(1) | - | 5 | - | pF |  |
| DuCy(LSE) | Duty cycle | - | 30 | - | 70 | % |
| IL | OSC32_IN Input leakage current | VSS ≤ VIN ≤ VDD | - | ±1 | µA |  |


1. Specified by design.

**Figure 30. High-speed external clock source AC timing diagram**

V<sub>HSEH</sub>
90 %
10 %
V<sub>HSEL</sub>

t<sub>r(HSE)</sub>
t<sub>f(HSE)</sub>
t<sub>W(HSE)</sub>
t<sub>W(HSE)</sub>
T<sub>HSE</sub>
t

External clock source
f<sub>HSE_ext</sub>
OSC_IN
I<sub>L</sub>
STM32F

ai17528

102/206
DS8626 Rev 12

--- Page 103 ---

STM32F405xx, STM32F407xx
Electrical characteristics

Figure 31. Low-speed external clock source AC timing diagram

ai17529

High-speed external clock generated from a crystal/ceramic resonator

The high-speed external (HSE) clock can be supplied with a 4 to 26 MHz crystal/ceramic resonator oscillator. All the information given in this paragraph are based on characterization results obtained with typical external components specified in Table 32. In the application, the resonator and the load capacitors have to be placed as close as possible to the oscillator pins in order to minimize output distortion and startup stabilization time. Refer to the crystal resonator manufacturer for more details on the resonator characteristics (frequency, package, accuracy).

Table 32. HSE 4-26 MHz oscillator characteristics (1)

| Symbol | Parameter | Conditions | Min | Typ | Max | Unit |
|---|---|---|---|---|---|---|
| fOSC_IN | Oscillator frequency | - | 4 | - | 26 | MHz |
| RF | Feedback resistor | - | 200 | - | kΩ |  |
| Gm | Oscillator transconductance | Startup | 5 | - | mA/V |  |
| Gmcrilmax | Maximum critical crystal Gm | - | 1 |  |  |  |
| tSU(HSE)(2) | Startup time | VDD is stabilized | - | 2 | - | ms |


1. Specified by design.
2. Evaluated by characterization - not tested in production. t<sub>SU(HSE)</sub> is the startup time measured from the moment it is enabled (by software) to a stabilized 8 MHz oscillation is reached. This value is measured for a standard crystal resonator and can vary significantly with the crystal manufacturer

For C<sub>L1</sub> and C<sub>L2</sub>, it is recommended to use high-quality external ceramic capacitors in the 5 pF to 25 pF range (typ.), designed for high-frequency applications, and selected to match the requirements of the crystal or resonator (see Figure 32). C<sub>L1</sub> and C<sub>L2</sub> are usually the same size. The crystal manufacturer typically specifies a load capacitance which is the series combination of C<sub>L1</sub> and C<sub>L2</sub>. PCB and MCU pin capacitance must be included (10 pF can be used as a rough estimate of the combined pin and board capacitance) when sizing C<sub>L1</sub> and C<sub>L2</sub>.

ST
DS8626 Rev 12
103/206

--- Page 104 ---

**Electrical characteristics**
**STM32F405xx, STM32F407xx**

**Note:**
*For information on electing the crystal, refer to the application note AN2867 “Oscillator design guide for ST microcontrollers” available from the ST website www.st.com.*

**Figure 32. Typical application with an 8 MHz crystal**

Resonator with integrated capacitors
CL1
CL2
8 MHz resonator
REXT(1)
OSC_IN
OSC_OUT
RF
Bias controlled gain
fHSE
STM32F
ai17530

1. REXT value depends on the crystal characteristics.

**Low-speed external clock generated from a crystal/ceramic resonator**

The low-speed external (LSE) clock can be supplied with a 32.768 kHz crystal/ceramic resonator oscillator. All the information given in this paragraph are based on characterization results obtained with typical external components specified in Table 33. In the application, the resonator and the load capacitors have to be placed as close as possible to the oscillator pins in order to minimize output distortion and startup stabilization time. Refer to the crystal resonator manufacturer for more details on the resonator characteristics (frequency, package, accuracy).

**Table 33. LSE oscillator characteristics (fLSE = 32.768 kHz) (1)**

| Symbol | Parameter | Conditions | Min | Typ | Max | Unit |
|---|---|---|---|---|---|---|
| fOSC_IN | Oscillator frequency | - | 32.768 | - | kHz |  |
| RF | Feedback resistor | - | 18.4 | - | MΩ |  |
| IDD | LSE current consumption | - | 1 | µA |  |  |
| Gm | Oscillator transconductance | Startup | 2.8 | - | µA/V |  |
| Gmcritmax | Maximum critical crystal Gm | - | 0.56 |  |  |  |
| tSU(LSE)(2) | Startup time | VDD is stabilized | - | 2 | - | s |


1. Specified by design.
2. Evaluated by characterization - not tested in production. tSU(LSE) is the startup time measured from the moment it is enabled (by software) to a stabilized 32.768 kHz oscillation is reached. This value is measured for a standard crystal resonator and it can vary significantly with the crystal manufacturer

**Note:**
*For information on electing the crystal, refer to the application note AN2867 “Oscillator design guide for ST microcontrollers” available from the ST website www.st.com.*

104/206
DS8626 Rev 12
ST

--- Page 105 ---

STM32F405xx, STM32F407xx
Electrical characteristics

Figure 33. Typical application with a 32.768 kHz crystal

ai17531

6.3.9 Internal clock source characteristics

The parameters given in Table 34 and Table 35 are derived from tests performed under ambient temperature and VDD supply voltage conditions summarized in Table 14.

High-speed internal (HSI) RC oscillator

Table 34. HSI oscillator characteristics (1)

| Symbol | Parameter | Conditions | Min | Typ | Max | Unit |
|---|---|---|---|---|---|---|
| fHSI | Frequency | - | 16 | - | MHz |  |
| ACCHSI | HSI user trimming step(2) | - | 1 | % |  |  |
| Accuracy of the HSI oscillator | TA = -40 to 105 °C(3) | -8 | - | 4.5 | % |  |
| TA = -10 to 85 °C(3) | -4 | - | 4 | % |  |  |
| TA = 25 °C(4) | -1 | - | 1 | % |  |  |
| tsu(HSI)(2) | HSI oscillator startup time | - | 2.2 | 4 | µs |  |
| IDD(HSI)(2) | HSI oscillator power consumption | - | 60 | 80 | µA |  |


1. VDD = 3.3 V, PLL OFF, TA = -40 to 125 °C unless otherwise specified.
2. Specified by design.
3. Evaluated by characterization - not tested in production.
4. Factory calibrated, parts not soldered.

Low-speed internal (LSI) RC oscillator

Table 35. LSI oscillator characteristics (1)

| Symbol | Parameter | Min | Typ | Max | Unit |
|---|---|---|---|---|---|
| fLSI(2) | Frequency | 17 | 32 | 47 | kHz |
| tsu(LSI)(3) | LSI oscillator startup time | - | 15 | 40 | µs |
| IDD(LSI)(3) | LSI oscillator power consumption | - | 0.4 | 0.6 | µA |


1. VDD = 3 V, TA = -40 to 105 °C unless otherwise specified.
2. Evaluated by characterization - not tested in production.
3. Specified by design.

ST
DS8626 Rev 12
105/206

--- Page 106 ---

Electrical characteristics
STM32F405xx, STM32F407xx

Figure 34. ACC$_{LSI}$ versus temperature

MS19013V1


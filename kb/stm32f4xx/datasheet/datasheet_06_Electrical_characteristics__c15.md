> [DATASHEET ch.6 §6] Electrical characteristics

### 6.3.25 DAC electrical characteristics

### 6.3.25 DAC electrical characteristics

**Table 74. DAC characteristics**

| Symbol | Parameter | Min | Typ | Max | Unit | Comments |
|---|---|---|---|---|---|---|
| VDDA | Analog supply voltage | 1.8(1) | - | 3.6 | V | - |
| VREF+ | Reference supply voltage | 1.8(1) | - | 3.6 | V | VREF+ ≤ VDDA |
| VSSA | Ground | 0 | - | 0 | V | - |


ST
DS8626 Rev 12
141/206

--- Page 142 ---

**Electrical characteristics**
**STM32F405xx, STM32F407xx**

**Table 74. DAC characteristics (continued)**

| Symbol | Parameter | Min | Typ | Max | Unit | Comments |
|---|---|---|---|---|---|---|
| RLOAD(2) | Resistive load with buffer ON | 5 | - | kΩ | - |  |
| RO(2) | Impedance output with buffer OFF | - | 15 | kΩ | When the buffer is OFF, the Minimum resistive load between DAC_OUT and VSS to have a 1% accuracy is 1.5 MΩ |  |
| CLOAD(2) | Capacitive load | - | 50 | pF | Maximum capacitive load at DAC_OUT pin (when the buffer is ON). |  |
| DAC_OUTmin(2) | Lower DAC_OUT voltage with buffer ON | 0.2 | - | V | It gives the maximum output excursion of the DAC.It corresponds to 12-bit input code (0x0E0) to (0xF1C) at VREF+ = 3.6 V and (0x1C7) to (0xE38) at VREF+ = 1.8 V |  |
| DAC_OUTmax(2) | Higher DAC_OUT voltage with buffer ON | - | VDDA - 0.2 | V |  |  |
| DAC_OUTmin(2) | Lower DAC_OUT voltage with buffer OFF | - | 0.5 | - | mV | It gives the maximum output excursion of the DAC. |
| DAC_OUTmax(2) | Higher DAC_OUT voltage with buffer OFF | - | VREF+ - 1LSB | V |  |  |
| IVREF+(4) | DAC DC VREF current consumption in quiescent mode (Standby mode) | - | 170 | 240 | µA | With no load, worst code (0x800) at VREF+ = 3.6 V in terms of DC consumption on the inputs |
| - | 50 | 75 | With no load, worst code (0xF1C) at VREF+ = 3.6 V in terms of DC consumption on the inputs |  |  |  |
| IDDA(4) | DAC DC VDDA current consumption in quiescent mode(3) | - | 280 | 380 | µA | With no load, middle code (0x800) on the inputs |
| - | 475 | 625 | µA | With no load, worst code (0xF1C) at VREF+ = 3.6 V in terms of DC consumption on the inputs |  |  |
| DNL(4) | Differential non linearityDifference between two consecutive code-1LSB) | - | ±0.5 | LSB | Given for the DAC in 10-bit configuration. |  |
| - | ±2 | LSB | Given for the DAC in 12-bit configuration. |  |  |  |
| INL(4) | Integral non linearity (difference between measured value at Code i and the value at Code i on a line drawn between Code 0 and last Code 1023) | - | ±1 | LSB | Given for the DAC in 10-bit configuration. |  |
| - | ±4 | LSB | Given for the DAC in 12-bit configuration. |  |  |  |


142/206
DS8626 Rev 12

--- Page 143 ---

STM32F405xx, STM32F407xx
Electrical characteristics

Table 74. DAC characteristics (continued)

| Symbol | Parameter | Min | Typ | Max | Unit | Comments |
|---|---|---|---|---|---|---|
| Offset(4) | Offset error (difference between measured value at Code (0x800) and the ideal value = VREF+/2) | - | ±10 | mV | Given for the DAC in 12-bit configuration |  |
| - | ±3 | LSB | Given for the DAC in 10-bit at VREF+ = 3.6 V |  |  |  |
| - | ±12 | LSB | Given for the DAC in 12-bit at VREF+ = 3.6 V |  |  |  |
| Gain error(4) | Gain error | - | ±0.5 | % | Given for the DAC in 12-bit configuration |  |
| tSETTLING(4) | Settling time (full scale: for a 10-bit input code transition between the lowest and the highest input codes when DAC_OUT reaches final value ±4LSB | - | 3 | 6 | µs | CLOAD ≤ 50 pF, RLOAD ≥ 5 kΩ |
| THD(4) | Total Harmonic Distortion Buffer ON | - | dB | CLOAD ≤ 50 pF, RLOAD ≥ 5 kΩ |  |  |
| Update rate(2) | Max frequency for a correct DAC_OUT change when small variation in the input code (from code i to i+1LSB) | - | 1 | MS/s | CLOAD ≤ 50 pF, RLOAD ≥ 5 kΩ |  |
| tWAKEUP(4) | Wakeup time from off state (Setting the ENx bit in the DAC Control register) | - | 6.5 | 10 | µs | CLOAD ≤ 50 pF, RLOAD ≥ 5 kΩ input code between lowest and highest possible ones. |
| PSRR+(2) | Power supply rejection ratio (to VDDA) (static DC measurement) | - | –67 | –40 | dB | No RLOAD, CLOAD = 50 pF |


1. V<sub>DD</sub>/V<sub>DDA</sub> minimum value of 1.7 V is obtained when the device operates in reduced temperature range, and with the use of an external power supply supervisor (refer to Section 3.15.2: Internal reset OFF).
2. Specified by design.
3. The quiescent mode corresponds to a state where the DAC maintains a stable output level to ensure that no dynamic consumption occurs.
4. Evaluated by characterization - not tested in production.

ST
DS8626 Rev 12
143/206

--- Page 144 ---

**Electrical characteristics**
**STM32F405xx, STM32F407xx**

**Figure 53. 12-bit buffered /non-buffered DAC**

Buffered/Non-buffered DAC
Buffer(1)
12-bit digital to analog converter
DAC_OUTx
R L
C L
ai17157V3

1. The DAC integrates an output buffer that can be used to reduce the output impedance and to drive external loads directly without the use of an external operational amplifier. The buffer can be bypassed by configuring the BOFFx bit in the DAC_CR register.


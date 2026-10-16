> [DATASHEET ch.6 §6] Electrical characteristics

### 6.3.16 I/O port characteristics

### 6.3.16 I/O port characteristics

**General input/output characteristics**

Unless otherwise specified, the parameters given in Table 48 are derived from tests performed under the conditions summarized in Table 14. All I/Os are CMOS and TTL compliant.

*Note:* For information on GPIO configuration, refer to application note AN4899 “STM32 GPIO configuration for hardware settings and low-power consumption” available from the ST website www.st.com.

Table 48. I/O static characteristics

| Symbol | Parameter | Conditions | Min | Typ | Max | Unit |
|---|---|---|---|---|---|---|
| VIL | FT, TTa and NRST I/O input low level voltage | 1.7 V ≤VDD ≤3.6 V | - | 0.3VDD-0.04(1) | V |  |
|  | - | 0.3VDD(2) |  |  |  |  |
| BOOT0 I/O input low level voltage | 1.75 V ≤VDD ≤3.6 V-40 °C≤TA≤105 °C | - | 0.1VDD+0.1(1) |  |  |  |
| 1.7 V ≤VDD ≤3.6 V0 °C≤TA≤105 °C | - |  |  |  |  |  |
| VIH | FT, TTa and NRST I/O input low level voltage | 1.7 V ≤VDD ≤3.6 V | 0.45VDD+0.3(1) | - |  |  |
|  | 0.7VDD(2) | - |  |  |  |  |
| BOOT0 I/O input low level voltage | 1.75 V ≤VDD ≤3.6 V-40 °C≤TA≤105 °C | 0.17VDD+0.7(1) | - |  |  |  |
| 1.7 V ≤VDD ≤3.6 V0 °C≤TA≤105 °C | - |  |  |  |  |  |


116/206
DS8626 Rev 12

--- Page 117 ---

**STM32F405xx, STM32F407xx**
**Electrical characteristics**

**Table 48. I/O static characteristics (continued)**

| Symbol | Parameter | Conditions | Min | Typ | Max | Unit |  |
|---|---|---|---|---|---|---|---|
| VHYS | FT, TTa and NRST I/O input hysteresis | 1.7 V ≤VDD ≤3.6 V | 10%VDD(3) | - | V |  |  |
| BOOT0 I/O input hysteresis | 1.75 V ≤VDD ≤3.6 V-40 °C≤TA ≤105 °C | 0.1 | - |  |  |  |  |
| 1.7 V ≤VDD ≤3.6 V0 °C≤TA ≤105 °C |  |  |  |  |  |  |  |
| Ilkg | I/O input leakage current (4) | VSS ≤VIN ≤VDD | - | ±1 | µA |  |  |
| I/O FT input leakage current (5) | VIN = 5 V | - | 3 |  |  |  |  |
| RPU | Weak pull-up equivalent resistor(6) | All pins except for PA10 and PB12 (OTG_FS_ID, OTG_HS_ID) | VIN = VSS | 30 | 40 | 50 | kΩ |
| PA10 and PB12 (OTG_FS_ID, OTG_HS_ID) | - | 7 | 10 | 14 |  |  |  |
| RPD | Weak pull-down equivalent resistor(7) | All pins except for PA10 and PB12 | VIN = VDD | 30 | 40 | 50 |  |
| PA10 and PB12 | - | 7 | 10 | 14 |  |  |  |
| CIO(8) | I/O pin capacitance | - | 5 | - | pF |  |  |


1. Specified by design.
2. Tested in production.
3. With a minimum of 200 mV.
4. Leakage could be higher than the maximum value, if negative current is injected on adjacent pins. Refer to Table 47: I/O current injection susceptibility
5. To sustain a voltage higher than V<sub>DD</sub> + 0.3 V, the internal pull-up/pull-down resistors must be disabled. Leakage could be higher than the maximum value, if negative current is injected on adjacent pins. Refer to Table 47: I/O current injection susceptibility.
6. Pull-up and pull-down resistors are designed with a true resistance in series with a switchable PMOS. This PMOS contribution to the series resistance is minimum (~10% order).
7. Pull-up and pull-down resistors are designed with a true resistance in series with a switchable NMOS. This NMOS contribution to the series resistance is minimum (~10% order).
8. Hysteresis voltage between Schmitt trigger switching levels. Evaluated by characterization - not tested in production.

All I/Os are CMOS and TTL compliant (no software configuration required). Their characteristics cover more than the strict CMOS-technology or TTL parameters.

ST
DS8626 Rev 12
117/206

--- Page 118 ---

**Electrical characteristics**
**STM32F405xx, STM32F407xx**

**Output driving current**

The GPIOs (general purpose input/outputs) can sink or source up to ±8 mA, and sink or source up to ±20 mA (with a relaxed $V_{OL}/V_{OH}$) except PC13, PC14 and PC15 which can sink or source up to ±3mA. When using the PC13 to PC15 GPIOs in output mode, the speed should not exceed 2 MHz with a maximum load of 30 pF.

In the user application, the number of I/O pins which can drive current must be limited to respect the absolute maximum rating specified in Section 6.2. In particular:

*   The sum of the currents sourced by all the I/Os on $V_{DD}$, plus the maximum Run consumption of the MCU sourced on $V_{DD}$, cannot exceed the absolute maximum rating $I_{VDD}$ (see Table 12).
*   The sum of the currents sunk by all the I/Os on $V_{SS}$ plus the maximum Run consumption of the MCU sunk on $V_{SS}$ cannot exceed the absolute maximum rating $I_{VSS}$ (see Table 12).

**Output voltage levels**

Unless otherwise specified, the parameters given in Table 49 are derived from tests performed under ambient temperature and $V_{DD}$ supply voltage conditions summarized in Table 14. All I/Os are CMOS and TTL compliant.

**Table 49. Output voltage characteristics**(1)

| Symbol | Parameter | Conditions | Min | Max | Unit |
|---|---|---|---|---|---|
| $V_{OL}^{(2)}$ | Output low level voltage | CMOS port$I_{IO}$ = +8 mA2.7 V < $V_{DD}$ < 3.6 V | - | 0.4 | V |
| $V_{OH}^{(3)}$ | Output high level voltage | $V_{DD}$-0.4 | - |  |  |
| $V_{OL}^{(2)}$ | Output low level voltage | TTL port$I_{IO}$ =+ 8mA2.7 V < $V_{DD}$ < 3.6 V | - | 0.4 | V |
| $V_{OH}^{(3)}$ | Output high level voltage | 2.4 | - |  |  |
| $V_{OL}^{(2)(4)}$ | Output low level voltage | $I_{IO}$ = +20 mA2.7 V < $V_{DD}$ < 3.6 V | - | 1.3 | V |
| $V_{OH}^{(3)(4)}$ | Output high level voltage | $V_{DD}$-1.3 | - |  |  |
| $V_{OL}^{(2)(4)}$ | Output low level voltage | $I_{IO}$ = +6 mA2 V < $V_{DD}$ < 2.7 V | - | 0.4 | V |
| $V_{OH}^{(3)(4)}$ | Output high level voltage | $V_{DD}$-0.4 | - |  |  |


1. PC13, PC14, PC15 and PI8 are supplied through the power switch. Since the switch only sinks a limited amount of current (3 mA), the use of GPIOs PC13 to PC15 and PI8 in output mode is limited: the speed should not exceed 2 MHz with a maximum load of 30 pF and these I/Os must not be used as a current source (e.g. to drive an LED).
2. The $I_{IO}$ current sunk by the device must always respect the absolute maximum rating specified in Table 12 and the sum of $I_{IO}$ (I/O ports and control pins) must not exceed $I_{VSS}$.
3. The $I_{IO}$ current sourced by the device must always respect the absolute maximum rating specified in Table 12 and the sum of $I_{IO}$ (I/O ports and control pins) must not exceed $I_{VDD}$.
4. Evaluated by characterization - not tested in production.

118/206
DS8626 Rev 12
ST

--- Page 119 ---

**STM32F405xx, STM32F407xx**
**Electrical characteristics**

**Input/output AC characteristics**

The definition and values of input/output AC characteristics are given in Figure 37 and Table 50, respectively.

Unless otherwise specified, the parameters given in Table 50 are derived from tests performed under the ambient temperature and VDD supply voltage conditions summarized in Table 14.

Table 50. I/O AC characteristics(1)(2)

| OSPEEDRy [1:0] bit value(1) | Symbol | Parameter | Conditions | Min | Typ | Max | Unit |
|---|---|---|---|---|---|---|---|
| 00 | fmax(IO)out | Maximum frequency(3) | CL = 50 pF, VDD > 2.70 V | - | 4 | MHz |  |
| CL = 50 pF, VDD > 1.8 V | - | 2 |  |  |  |  |  |
| CL = 10 pF, VDD > 2.70 V | - | 8 |  |  |  |  |  |
| CL = 10 pF, VDD > 1.8 V | - | 4 |  |  |  |  |  |
| tf(IO)out/ tr(IO)out | Output high to low level fall time and output low to high level rise time | CL = 50 pF, VDD = 1.8 V to 3.6 V | - | 100 | ns |  |  |
| 01 | fmax(IO)out | Maximum frequency(3) | CL = 50 pF, VDD > 2.70 V | - | 25 | MHz |  |
| CL = 50 pF, VDD > 1.8 V | - | 12.5 |  |  |  |  |  |
| CL = 10 pF, VDD > 2.70 V | - | 50(4) |  |  |  |  |  |
| CL = 10 pF, VDD > 1.8 V | - | 20 |  |  |  |  |  |
| tf(IO)out/ tr(IO)out | Output high to low level fall time and output low to high level rise time | CL = 50 pF, VDD >2.7 V | - | 10 | ns |  |  |
| CL = 50 pF, VDD > 1.8 V | - | 20 |  |  |  |  |  |
| CL = 10 pF, VDD > 2.70 V | - | 6 |  |  |  |  |  |
| CL = 10 pF, VDD > 1.8 V | - | 10 |  |  |  |  |  |
| 10 | fmax(IO)out | Maximum frequency(3) | CL = 40 pF, VDD > 2.70 V | - | 50(4) | MHz |  |
| CL = 40 pF, VDD > 1.8 V | - | 25 |  |  |  |  |  |
| CL = 10 pF, VDD > 2.70 V | - | 100(4) |  |  |  |  |  |
| CL = 10 pF, VDD > 1.8 V | - | 50(4) |  |  |  |  |  |
| tf(IO)out/ tr(IO)out | Output high to low level fall time and output low to high level rise time | CL = 40 pF, VDD > 2.70 V | - | 6 | ns |  |  |
| CL = 40 pF, VDD > 1.8 V | - | 10 |  |  |  |  |  |
| CL = 10 pF, VDD > 2.70 V | - | 4 |  |  |  |  |  |
| CL = 10 pF, VDD > 1.8 V | - | 6 |  |  |  |  |  |


ST
DS8626 Rev 12
119/206

--- Page 120 ---

**Electrical characteristics**
STM32F405xx, STM32F407xx

**Table 50. I/O AC characteristics(1)(2) (continued)**

| OSPEEDRy[1:0] bitvalue(1) | Symbol | Parameter | Conditions | Min | Typ | Max | Unit |
|---|---|---|---|---|---|---|---|
| 11 | Fmax(IO)out | Maximum frequency(3) | CL = 30 pF, VDD > 2.70 V | - | 100(4) | MHz |  |
| CL = 30 pF, VDD > 1.8 V | - | 50(4) |  |  |  |  |  |
| CL = 10 pF, VDD > 2.70 V | - | 180(4) |  |  |  |  |  |
| CL = 10 pF, VDD > 1.8 V | - | 100(4) |  |  |  |  |  |
| tf(IO)out/tr(IO)out | Output high to low level falltime and output low to highlevel rise time | CL = 30 pF, VDD > 2.70 V | - | 4 | ns |  |  |
| CL = 30 pF, VDD > 1.8 V | - | 6 |  |  |  |  |  |
| CL = 10 pF, VDD > 2.70 V | - | 2.5 |  |  |  |  |  |
| CL = 10 pF, VDD > 1.8 V | - | 4 |  |  |  |  |  |
| - | tEXTlpw | Pulse width of external signalsdetected by the EXTIcontroller | - | 10 | - | ns |  |


1. Evaluated by characterization - not tested in production.
2. The I/O speed is configured using the OSPEEDRy[1:0] bits. Refer to the STM32F4xx reference manual for a description of the GPIOx_SPEEDR GPIO port output speed register.
3. The maximum frequency is defined in Figure 37.
4. For maximum frequencies above 50 MHz, the compensation cell should be used.

**Figure 37. I/O AC characteristics definition**

EXTERNAL OUTPUT ON CL
tr(IO)out
10%
50%
90%
10%
50%
90%
tf(IO)out
T
Maximum frequency is achieved with a duty cycle at 45-55% when loaded by the specified capacitance
MSv75936V1

120/206
DS8626 Rev 12
ST

--- Page 121 ---

**STM32F405xx, STM32F407xx**
**Electrical characteristics**


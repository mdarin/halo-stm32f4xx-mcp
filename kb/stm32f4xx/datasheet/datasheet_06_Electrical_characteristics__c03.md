> [DATASHEET ch.6 §6] Electrical characteristics

### 6.3.5 Embedded reset and power control block characteristics

### 6.3.5 Embedded reset and power control block characteristics

The parameters given in Table 19 are derived from tests performed under ambient temperature and VDD supply voltage conditions summarized in Table 14.

**Table 19. Embedded reset and power control block characteristics**

| Symbol | Parameter | Conditions | Min | Typ | Max | Unit |
|---|---|---|---|---|---|---|
| VPVD | Programmable voltage detector level selection | PLS[2:0]=000 (rising edge) | 2.09 | 2.14 | 2.19 | V |
| PLS[2:0]=000 (falling edge) | 1.98 | 2.04 | 2.08 | V |  |  |
| PLS[2:0]=001 (rising edge) | 2.23 | 2.30 | 2.37 | V |  |  |
| PLS[2:0]=001 (falling edge) | 2.13 | 2.19 | 2.25 | V |  |  |
| PLS[2:0]=010 (rising edge) | 2.39 | 2.45 | 2.51 | V |  |  |
| PLS[2:0]=010 (falling edge) | 2.29 | 2.35 | 2.39 | V |  |  |
| PLS[2:0]=011 (rising edge) | 2.54 | 2.60 | 2.65 | V |  |  |
| PLS[2:0]=011 (falling edge) | 2.44 | 2.51 | 2.56 | V |  |  |
| PLS[2:0]=100 (rising edge) | 2.70 | 2.76 | 2.82 | V |  |  |
| PLS[2:0]=100 (falling edge) | 2.59 | 2.66 | 2.71 | V |  |  |
| PLS[2:0]=101 (rising edge) | 2.86 | 2.93 | 2.99 | V |  |  |
| PLS[2:0]=101 (falling edge) | 2.75 | 2.84 | 2.92 | V |  |  |
| PLS[2:0]=110 (rising edge) | 2.96 | 3.03 | 3.10 | V |  |  |
| PLS[2:0]=110 (falling edge) | 2.85 | 2.93 | 2.99 | V |  |  |
| PLS[2:0]=111 (rising edge) | 3.07 | 3.14 | 3.21 | V |  |  |
| PLS[2:0]=111 (falling edge) | 2.95 | 3.03 | 3.09 | V |  |  |
| VPVDhyst(1) | PVD hysteresis | - | 100 | - | mV |  |
| VPOR/PDR | Power-on/power-down reset threshold | Falling edge | 1.60 | 1.68 | 1.76 | V |
| Rising edge | 1.64 | 1.72 | 1.80 | V |  |  |
| VPDRhyst(1) | PDR hysteresis | - | 40 | - | mV |  |
| VBOR1 | Brownout level 1 threshold | Falling edge | 2.13 | 2.19 | 2.24 | V |
| Rising edge | 2.23 | 2.29 | 2.33 | V |  |  |


ST
DS8626 Rev 12
85/206

--- Page 86 ---

**Electrical characteristics**
**STM32F405xx, STM32F407xx**

**Table 19. Embedded reset and power control block characteristics (continued)**

| Symbol | Parameter | Conditions | Min | Typ | Max | Unit |
|---|---|---|---|---|---|---|
| VBOR2 | Brownout level 2 threshold | Falling edge | 2.44 | 2.50 | 2.56 | V |
| Rising edge | 2.53 | 2.59 | 2.63 | V |  |  |
| VBOR3 | Brownout level 3 threshold | Falling edge | 2.75 | 2.83 | 2.88 | V |
| Rising edge | 2.85 | 2.92 | 2.97 | V |  |  |
| VBORhyst(1) | BOR hysteresis | - | 100 | - | mV |  |
| TRSTTEMPO(1)(2) | Reset tempornization | - | 0.5 | 1.5 | 3.0 | ms |
| IRUSH(1) | InRush current on voltage regulator power-on (POR or wakeup from Standby) | - | 160 | 200 | mA |  |
| ERUSH(1) | InRush energy on voltage regulator power-on (POR or wakeup from Standby) | VDD = 1.8 V, TA = 105 °C, IRUSH = 171 mA for 31 µs | - | 5.4 | µC |  |


1. Specified by design.
2. The reset tempornization is measured from the power-on (POR reset or wakeup from V<sub>BAT</sub>) to the instant when first instruction is read by the user application code.


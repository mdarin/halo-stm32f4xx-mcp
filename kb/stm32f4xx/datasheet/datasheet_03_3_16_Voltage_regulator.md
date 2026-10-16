### 3.16 Voltage regulator

The regulator has four operating modes:
* Regulator ON
    * Main regulator mode (MR)
    * Low-power regulator (LPR)
    * Power-down
* Regulator OFF

### 3.16.1 Regulator ON

On packages embedding the BYPASS_REG pin, the regulator is enabled by holding BYPASS_REG low. On all other packages, the regulator is always enabled.

There are three power modes configured by software when regulator is ON:
* MR is used in the nominal regulation mode (With different voltage scaling in Run)
  In Main regulator mode (MR mode), different voltage scaling are provided to reach the best compromise between maximum frequency and dynamic power consumption. Refer to Table 14: General operating conditions.
* LPR is used in the Stop modes
  The LP regulator mode is configured by software when entering Stop mode.
* Power-down is used in Standby mode.
  The Power-down mode is activated only when entering in Standby mode. The regulator output is in high impedance and the kernel circuitry is powered down, inducing zero consumption. The contents of the registers and SRAM are lost)

28/206
DS8626 Rev 12
ST

--- Page 29 ---

**STM32F405xx, STM32F407xx**
**Functional overview**

Two external ceramic capacitors should be connected on V<sub>CAP_1</sub> & V<sub>CAP_2</sub> pin. Refer to *Figure 21: Power supply scheme* and *Figure 16: VCAP_1/VCAP_2 operating conditions*.
All packages have regulator ON feature.

### 3.16.2 Regulator OFF

This feature is available only on packages featuring the BYPASS_REG pin. The regulator is disabled by holding BYPASS_REG high. The regulator OFF mode allows to supply externally a V<sub>12</sub> voltage source through V<sub>CAP_1</sub> and V<sub>CAP_2</sub> pins.

Since the internal voltage scaling is not manage internally, the external voltage value must be aligned with the targeted maximum frequency. Refer to *Table 14: General operating conditions*.

The two 2.2 µF ceramic capacitors should be replaced by two 100 nF decoupling capacitors.

Refer to *Figure 21: Power supply scheme*

When the regulator is OFF, there is no more internal monitoring on V<sub>12</sub>. An external power supply supervisor should be used to monitor the V<sub>12</sub> of the logic power domain. PA0 pin should be used for this purpose, and act as power-on reset on V<sub>12</sub> power domain.

In regulator OFF mode the following features are no more supported:

*   PA0 cannot be used as a GPIO pin since it allows to reset a part of the V<sub>12</sub> logic power domain which is not reset by the NRST pin.
*   As long as PA0 is kept low, the debug mode cannot be used under power-on reset. As a consequence, PA0 and NRST pins must be managed separately if the debug connection under reset or pre-reset is required.
*   The standby mode is not available

**Figure 9. Regulator OFF**

V<sub>12</sub>
External V<sub>CAP_1/2</sub> power supply supervisor
Ext. reset controller active when V<sub>CAP_1/2</sub> < Min V<sub>12</sub>
Application reset signal (optional)
PA0
NRST
BYPASS_REG
V<sub>CAP1</sub>
V<sub>CAP2</sub>
(note 1)
V<sub>12</sub>
(1 x 100 nF)
2 x 100 nF
V<sub>DD</sub>
V<sub>DD</sub>
1/2/...N
N x 100 nF
+ 1 x 4.7 µF
V<sub>SS</sub>
1/2/...N
ai18498v5

ST
DS8626 Rev 12
29/206

--- Page 30 ---

**Functional overview**
STM32F405xx, STM32F407xx

The following conditions must be respected:

*   VDD should always be higher than VCAP_1 and VCAP_2 to avoid current injection between power domains.
*   If the time for VCAP_1 and VCAP_2 to reach V12 minimum value is faster than the time for VDD to reach 1.8 V, then PA0 should be kept low to cover both conditions: until VCAP_1 and VCAP_2 reach V12 minimum value and until VDD reaches 1.8 V (see Figure 10).
*   Otherwise, if the time for VCAP_1 and VCAP_2 to reach V12 minimum value is slower than the time for VDD to reach 1.8 V, then PA0 could be asserted low externally (see Figure 11).
*   If VCAP_1 and VCAP_2 go below V12 minimum value and VDD is higher than 1.8 V, then a reset must be asserted on PA0 pin.

**Note:** The minimum value of V12 depends on the maximum frequency targeted in the application (see Table 14: General operating conditions).

**Figure 10. Startup in regulator OFF mode: slow VDD slope - power-down reset risen after VCAP_1/VCAP_2 stabilization**

1. This figure is valid both whatever the internal reset mode (ON or OFF).
2. PDR = 1.7 V for reduced temperature range; PDR = 1.8 V for all temperature ranges.

30/206
DS8626 Rev 12

--- Page 31 ---

STM32F405xx, STM32F407xx
Functional overview

Figure 11. Startup in regulator OFF mode: fast VDD slope
- power-down reset risen before VCAP 1/VCAP 2 stabilization

1. This figure is valid both whatever the internal reset mode (ON or OFF).
2. PDR = 1.7 V for a reduced temperature range; PDR = 1.8 V for all temperature ranges.

### 3.17 Regulator ON/OFF and internal reset ON/OFF availability

Table 3. Regulator ON/OFF and internal reset ON/OFF availability

|  | Regulator ON | Regulator OFF | Internal reset ON | Internal reset OFF |
|---|---|---|---|---|
| LQFP64LQFP100 | Yes | No | Yes | No |
| LQFP144 |  |  |  |  |
| WL CSP90UFBGA176LQFP176 | YesBYPASS_REG set to VSS | YesBYPASS_REG set to VDD | YesPDR_ON set to VDD | YesPDR_ON connected to an external power supply supervisor |



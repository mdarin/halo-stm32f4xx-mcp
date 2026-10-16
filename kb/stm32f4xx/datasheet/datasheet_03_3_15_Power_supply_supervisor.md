### 3.15 Power supply supervisor

#### 3.15.1 Internal reset ON

On packages embedding the PDR_ON pin, the power supply supervisor is enabled by holding PDR_ON high. On all other packages, the power supply supervisor is always enabled.

The device has an integrated power-on reset (POR) / power-down reset (PDR) circuitry coupled with a Brownout reset (BOR) circuitry. At power-on, POR/PDR is always active and ensures proper operation starting from 1.8 V. After the 1.8 V POR threshold level is reached, the option byte loading process starts, either to confirm or modify default BOR threshold levels, or to disable BOR permanently. Three BOR thresholds are available through option bytes. The device remains in reset mode when $V_{DD}$ is below a specified threshold, $V_{POR/PDR}$ or $V_{BOR}$, without the need for an external reset circuit.

The device also features an embedded programmable voltage detector (PVD) that monitors the $V_{DD}/V_{DDA}$ power supply and compares it to the $V_{PVD}$ threshold. An interrupt can be generated when $V_{DD}/V_{DDA}$ drops below the $V_{PVD}$ threshold and/or when $V_{DD}/V_{DDA}$ is higher than the $V_{PVD}$ threshold. The interrupt service routine can then generate a warning message and/or put the MCU into a safe state. The PVD is enabled by software.

#### 3.15.2 Internal reset OFF

This feature is available only on packages featuring the PDR_ON pin. The internal power-on reset (POR) / power-down reset (PDR) circuitry is disabled with the PDR_ON pin.

An external power supply supervisor should monitor $V_{DD}$ and should maintain the device in reset mode as long as $V_{DD}$ is below a specified threshold. PDR_ON should be connected to this external power supply supervisor. Refer to Figure 7: Power supply supervisor interconnection with internal reset OFF.

26/206
DS8626 Rev 12
ST

--- Page 27 ---

STM32F405xx, STM32F407xx
Functional overview

Figure 7. Power supply supervisor interconnection with internal reset OFF

1. PDR = 1.7 V for reduce temperature range; PDR = 1.8 V for all temperature range.

The VDD specified threshold, below which the device must be maintained under reset, is 1.8 V (see Figure 7). This supply voltage can drop to 1.7 V when the device operates in the 0 to 70 °C temperature range.

A comprehensive set of power-saving mode allows to design low-power applications.

When the internal reset is OFF, the following integrated features are no more supported:
* The integrated power-on reset (POR) / power-down reset (PDR) circuitry is disabled
* The brownout reset (BOR) circuitry is disabled
* The embedded programmable voltage detector (PVD) is disabled
* VBAT functionality is no more available and VBAT pin should be connected to VDD

All packages, except for the LQFP64 and LQFP100, allow to disable the internal reset through the PDR_ON signal.

ST
DS8626 Rev 12
27/206

--- Page 28 ---

Functional overview
STM32F405xx, STM32F407xx

Figure 8. PDR_ON and NRST control with internal reset OFF

PDR = 1.7 V
V_DD
time
Reset by a source different from
power supply supervisor
NRST
PDR_ON
PDR_ON
time
MS19009V8

1. PDR = 1.7 V for reduce temperature range; PDR = 1.8 V for all temperature range.


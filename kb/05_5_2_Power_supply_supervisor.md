**5.2 Power supply supervisor**

**5.2.1 Power-on reset (POR)/power-down reset (PDR)**

The device has an integrated POR/PDR circuitry that allows proper operation starting from 1.8 V.

The device remains in Reset mode when V DD /V DDA is below a specified threshold, V POR/PDR , without the need for an external reset circuit. For more details concerning the power on/power-down reset threshold, refer to the electrical characteristics of the datasheet.

**Figure 12. Power-on reset/power-down reset waveform**

VDD/VDDA

PDR

40 mV

hysteresis PDR



Reset

MS30431V1



--- Page 125 ---

**RM0090 Power controller (PWR)**

**5.2.2 Brownout reset (BOR)**

During power on, the Brownout reset (BOR) keeps the device under reset until the supply voltage reaches the specified V BOR threshold.

V BOR is configured through device option bytes. By default, BOR is off. 3 programmable V BOR threshold levels can be selected:

• BOR Level 3 (VBOR3). Brownout threshold level 3.

• BOR Level 2 (VBOR2). Brownout threshold level 2.

• BOR Level 1 (VBOR1). Brownout threshold level 1.

Note: For full details about BOR characteristics, refer to the "Electrical characteristics" section in the device datasheet.

When the supply voltage (V DD ) drops below the selected V BOR threshold, a device reset is generated.

The BOR can be disabled by programming the device option bytes. In this case, the power-on and power-down is then monitored by the POR/ PDR (see Section 5.2.1: Power-

| on reset (POR The BOR thre supply voltage | et (POR OR thre voltage | )/power-down reset (PDR)). shold hysteresis is ~100 mV (between the rising a ). Figure 13. BOR thresholds | e rising a | nd the f | alling ed | ge of the |
|---|---|---|---|---|---|---|
| VDD/VDDA BOR threshold 100 mV hysteresis Reset MS30433V1 |  |  |  |  |  |  |


**5.2.3 Programmable voltage detector (PVD)**

You can use the PVD to monitor the V DD power supply by comparing it to a threshold selected by the PLS[2:0] bits in the PWR power control register (PWR_CR) for



149

--- Page 126 ---

**Power controller (PWR)**

**RM0090**

STM32F405xx/07xx and STM32F415xx/17xx for STM32F42xxx and STM32F43xxx .

and PWR power control register (PWR_CR)

The PVD is enabled by setting the PVDE bit. A PVDO flag is available, in the

PWR power control/status register (PWR_CSR)  for

STM32F405xx/07xx and STM32F415xx/17xx

, to indicate if V DD is higher or lower than the PVD threshold. This event is internally connected to the EXTI line16 and can generate an interrupt if enabled through the EXTI registers. The PVD output interrupt can be generated

when V DD drops below the PVD threshold and/or when V

DD rises above the PVD threshold depending on EXTI line16 rising/falling edge configuration. As an example the service

routine could perform emergency shutdown tasks.

**Figure 14. PVD thresholds**

VDD

PVD threshold



PVD output

MS30432V2


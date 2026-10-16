> [DATASHEET ch.6 §6] Electrical characteristics

### 6.3.15 I/O current injection characteristics**

**6.3.15 I/O current injection characteristics**

As a general rule, current injection to the I/O pins, due to external voltage below V_SS or above V_DD (for standard, 3 V-capable I/O pins) should be avoided during normal product operation. However, in order to give an indication of the robustness of the microcontroller in cases when abnormal injection accidentally happens, susceptibility tests are performed on a sample basis during device characterization.

**Functional susceptibility to I/O current injection**

While a simple application is executed on the device, the device is stressed by injecting current into the I/O pins programmed in floating input mode. While current is injected into the I/O pin, one at a time, the device is checked for functional failures.

The failure is indicated by an out of range parameter: ADC error above a certain limit (>5 LSB TUE), out of conventional limits of induced leakage current on adjacent pins (out of 5 µA/+0 µA range), or other functional failure (for example reset, oscillator frequency deviation).

Negative induced leakage current is caused by negative injection and positive induced leakage current by positive injection.

The test results are given in Table 47.

STI

DS8626 Rev 12

115/206

--- Page 116 ---

Electrical characteristics
STM32F405xx, STM32F407xx

Table 47. I/O current injection susceptibility

| Symbol | Description | Functional susceptibility | Unit |  |
|---|---|---|---|---|
| Negative injection | Positive injection |  |  |  |
| IINJ(1) | Injected current on BOOT0 pin | – 0 | NA | mA |
| Injected current on NRST pin | – 0 | NA |  |  |
| Injected current on PE2, PE3, PE4, PE5, PE6, PI8, PC13, PC14, PC15, PI9, PI10, PI11, PF0, PF1, PF2, PF3, PF4, PF5, PF10, PH0/OSC_IN, PH1/OSC_OUT, PC0, PC1, PC2, PC3, PB6, PB7, PB8, PB9, PE0, PE1, PI4, PI5, PI6, PI7, PDR_ON, BYPASS_REG | – 0 | NA |  |  |
| Injected current on all FT pins | – 5 | NA |  |  |
| Injected current on any other pin | – 5 | +5 |  |  |


1. It is recommended to add a Schottky diode (pin to ground) to analog pins which may potentially inject negative currents.


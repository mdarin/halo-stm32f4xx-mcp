## 14 Digital-to-analog converter (DAC)

This section applies to the whole STM32F4xx family, unless otherwise specifie d.

**14.1 DAC introduction**

The DAC module is a 12-bit, voltage output digital-to-analog converter. The DAC can be configured in 8- or 12-bit mode and may be used in conjunction with the DMA controller. In 12-bit mode, the data could be left- or right-aligned. The DAC has two output channels, each with its own converter. In dual DAC channel mode, conversions could be done independently or simultaneously when both channels are grouped together for synchronous update operations. An input reference pin, V REF+ (shared with ADC) is available for better resolution.

**14.2 DAC main features**

• Two DAC converters: one output channel each

• Left or right data alignment in 12-bit mode

• Synchronized update capability

• Noise-wave generation

• Triangular-wave generation

• Dual DAC channel for independent or simultaneous conversions

• DMA capability for each channel

• DMA underrun error detection

• External triggers for conversion

• Input voltage reference, V REF+

Figure 64 shows the block diagram of a DAC channel and Table 74 gives the pin description.

RM0090 Rev 18 433/1749

454

--- Page 44 ---

**Digital-to-analog converter (DAC) RM0090**

**Figure 64. DAC channel block diagram**



| MAMPx[3: | 0] bits |  |  |  |
|---|---|---|---|---|
| WAVENx[1:0] bits |  |  |  |  |


EXTI_9



V DDA

V SSA Digital-to-analog converterx DAC_OUTx V REF+

ai14708b

**Table 74. DAC pins**

| Name | Signal type | Remarks |
|---|---|---|
| V REF+ | Input, analog reference positive | The higher/positive reference voltage for the DAC, 1.8 V ≤ V ≤ V REF+ DDA |
| V DDA | Input, analog supply | Analog power supply |
| V SSA | Input, analog supply ground | Ground for analog power supply |
| DAC OUTx _ | Analog output signal | DAC channelx analog output |


Note: Once the DAC channelx is enabled, the corresponding GPIO pin (PA4 or PA5) is automatically connected to the analog converter output (DAC_OUTx). In order to avoid parasitic consumption, the PA4 or PA5 pin should first be configured to analog (AIN).



--- Page 45 ---

**RM0090 Digital-to-analog converter (DAC)**


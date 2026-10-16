**13.10 Temperature sensor**

The temperature sensor can be used to measure the ambient temperature (T A ) of the device.

• On STM32F40x and STM32F41x devices, the temperature sensor is internally

connected to ADC1_IN16 channel which is used to convert the sensor output voltage to a digital value.

• On STM32F42x and STM32F43x devices, the temperature sensor is internally

connected to the same input channel, ADC1_IN18, as VBAT: ADC1_IN18 is used to convert the sensor output voltage or VBAT into a digital value. Only one conversion, temperature sensor or VBAT, must be selected at a time. When the temperature sensor and the VBAT conversion are set simultaneously, only the VBAT conversion is performed.

Figure 63 shows the block diagram of the temperature sensor.

When not in use, the sensor can be put in power down mode.

Note: The TSVREFE bit must be set to enable the conversion of both internal channels: the ADC1_IN16 or ADC1_IN18 (temperature sensor) and the ADC1_IN17 (VREFINT).

**Main features**

• Supported temperature range: –40 to 125 °C

• Precision: ±1.5 °C



--- Page 23 ---

**RM0090 Analog-to-digital converter (ADC)**

**Figure 63. Temperature sensor and V REFINT channel block diagram**

TSVREFE control bit

Temperature VSENSE

sensor ADC1_IN16/

ADC1_IN18 (1)

Internal VREFINT ADC1 converted data Address/data bus power block ADC1_IN17

MS31830V1

1. V SENSE is input to ADC1_IN16 for the STM23F40x and STM32F41x devices and to ADC1_IN18 for the STM32F42x and STM32F43x devices.

**Reading the temperature**

To use the sensor:

3. Select ADC1_IN16 or ADC1_IN18 input channel.

4. Select a sampling time greater than the minimum sampling time specified in the datasheet.

5. Set the TSVREFE bit in the ADC_CCR register to wake up the temperature sensor from power down mode

6. Start the ADC conversion by setting the SWSTART bit (or by external trigger)

7. Read the resulting V SENSE data in the ADC data register

8. Calculate the temperature using the following formula:

Temperature (in °C) = {(V SENSE – V 25 ) / Avg_Slope} + 25

Where:

– V 25 = V SENSE value for 25° C

– Avg_Slope = average slope of the temperature vs. V SENSE curve (given in mV/°C

or µV/°C)

Refer to the datasheet’s electrical characteristics section for the actual values of V 25 and Avg_Slope.

Note: The sensor has a startup time after waking from power down mode before it can output V SENSE at the correct level. The ADC also has a startup time after power-on, so to minimize the delay, the ADON and TSVREFE bits should be set at the same time.

The temperature sensor output voltage changes linearly with temperature. The offset of this linear function depends on each chip due to process variation (up to 45 °C from one chip to another).



432

--- Page 24 ---

**Analog-to-digital converter (ADC) RM0090**

The internal temperature sensor is more suited for applications that detect temperature variations instead of absolute temperatures. If accurate temperature reading is required, an external temperature sensor should be used.

**13.11 Battery charge monitoring**

The VBATE bit in the ADC_CCR register is used to switch to the battery voltage. As the V BAT voltage could be higher than V DDA , to ensure the correct operation of the ADC, the V BAT pin is internally connected to a bridge divider.

When the VBATE is set, the bridge is automatically enabled to connect:

• VBAT/2 to the ADC1_IN18 input channel, on STM32F40xx and STM32F41xx devices

• VBAT/4 to the ADC1_IN18 input channel, on STM32F42xx and STM32F43xx devices Note: On STM32F42xx and STM32F43xx devices, VBAT and temperature sensor are connected to the same ADC internal channel (ADC1_IN18). Only one conversion, either temperature sensor or VBAT, must be selected at a time. When both conversion are enabled simultaneously, only the VBAT conversion is performed.

**13.12 ADC interrupts**

An interrupt can be produced on the end of conversion for regular and injected groups, when the analog watchdog status bit is set and when the overrun status bit is set. Separate interrupt enable bits are available for flexibility.

Two other flags are present in the ADC_SR register, but there is no interrupt associated with them:

• JSTRT (Start of conversion for channels of an injected group)

• STRT (Start of conversion for channels of a regular group)

**Table 70. ADC interrupts**

| Interrupt event | Event flag | Enable control bit |
|---|---|---|
| End of conversion of a regular group | EOC | EOCIE |
| End of conversion of an injected group | JEOC | JEOCIE |
| Analog watchdog status bit is set | AWD | AWDIE |
| Overrun | OVR | OVRIE |




--- Page 25 ---

**RM0090 Analog-to-digital converter (ADC)**


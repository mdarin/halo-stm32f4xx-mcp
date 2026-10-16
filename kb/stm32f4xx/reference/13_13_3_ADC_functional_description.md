## 13.3 ADC functional description

Figure 44 shows a single ADC block diagram and Table 65 gives the ADC pin description.

**Figure 44. Single ADC block diagram**

VREF+
VREF-
VDDA
VSSA

ADCx_IN0
ADCx_IN1
...
ADCx_IN15

GPIO ports

Analog mux
up to 4
up to 16

Temp. sensor
VREFINT
V_BAT

JEXTSEL[3:0] bits
TIM1_CH4
TIM1_TRGO
TIM2_CH1
TIM2_TRGO
TIM3_CH2
TIM3_CH4
TIM4_CH1
TIM4_CH2
TIM4_CH3
TIM4_TRGO
TIM5_CH4
TIM5_TRGO
TIM8_CH2
TIM8_CH3
TIM8_CH4

JEXTEN [1:0] bits
Start trigger
(injected group)

EXTSEL[3:0] bits
TIM1_CH1
TIM1_CH2
TIM1_CH3
TIM2_CH2
TIM2_CH3
TIM2_CH4
TIM2_TRGO
TIM3_CH1
TIM3_TRGO
TIM4_CH4
TIM5_CH1
TIM5_CH2
TIM5_CH3
TIM8_CH1
TIM8_TRGO

EXTEN [1:0] bits
Start trigger
(regular group)

EXTI_15
EXTI_11

Flags
DMA overrun
End of conversion
End of injected conversion
Analog watchdog event

OVR
EOC
JEOC
AWD

Interrupt enable bits
OVRIE
EOCIE
JEOCIE
AWDIE

ADC Interrupt to NVIC

Analog watchdog
Compare result
Higher threshold (12 bits)
Lower threshold (12 bits)

Injected data registers
(4 x 16 bits)

Regular data register
(16 bits)

Address/data bus
DMA request

Injected channels
Regular channels

Analog to digital converter
ADCCCLK
From ADC prescaler

ai16046

ST
RM0090 Rev 18
389/1749

--- Page 195 ---

Analog-to-digital converter (ADC)
RM0090

Table 65. ADC pins

| Name | Signal type | Remarks |
|---|---|---|
| VREF+ | Input, analog reference positive | The higher/positive reference voltage for the ADC, 1.8 V ≤ VREF+ ≤ VDDA |
| VDDA | Input, analog supply | Analog power supply equal to VDD and 2.4 V ≤ VDDA ≤ VDD (3.6 V) for full speed 1.8 V ≤ VDDA ≤ VDD (3.6 V) for reduced speed |
| VREF- | Input, analog reference negative | The lower/negative reference voltage for the ADC, VREF- = VSSA |
| VSSA | Input, analog supply ground | Ground for analog power supply equal to VSS |
| ADCx_IN[15:0] | Analog input signals | 16 analog input channels |


### 13.3.1 ADC on-off control

The ADC is powered on by setting the ADON bit in the ADC_CR2 register. When the ADON bit is set for the first time, it wakes up the ADC from the Power-down mode.

Conversion starts when either the SWSTART or the JSWSTART bit is set.

You can stop conversion and put the ADC in power down mode by clearing the ADON bit. In this mode the ADC consumes almost no power (only a few µA).

### 13.3.2 ADC clock

The ADC features two clock schemes:

*   Clock for the analog circuitry: ADCCLK, common to all ADCs
    This clock is generated from the APB2 clock divided by a programmable prescaler that allows the ADC to work at f<sub>PCLK2</sub>/2, /4, /6 or /8. Refer to the datasheets for the maximum value of ADCCLK.
*   Clock for the digital interface (used for registers read/write access)
    This clock is equal to the APB2 clock. The digital interface clock can be enabled/disabled individually for each ADC through the RCC APB2 peripheral clock enable register (RCC_APB2ENR).

### 13.3.3 Channel selection

There are 16 multiplexed channels. It is possible to organize the conversions in two groups: regular and injected. A group consists of a sequence of conversions that can be done on any channel and in any order. For instance, it is possible to implement the conversion sequence in the following order: ADC_IN3, ADC_IN8, ADC_IN2, ADC_IN2, ADC_IN0, ADC_IN2, ADC_IN2, ADC_IN15.

*   A **regular group** is composed of up to 16 conversions. The regular channels and their order in the conversion sequence must be selected in the ADC_SQRx registers. The total number of conversions in the regular group must be written in the L[3:0] bits in the ADC_SQR1 register.
*   An **injected group** is composed of up to 4 conversions. The injected channels and their order in the conversion sequence must be selected in the ADC_JSQR register.

390/1749
RM0090 Rev 18
ST
--- Page 1 ---

**RM0090 Analog-to-digital converter (ADC)**

The total number of conversions in the injected group must be written in the L[1:0] bits in the ADC_JSQR register.

If the ADC_SQRx or ADC_JSQR registers are modified during a conversion, the current conversion is reset and a new start pulse is sent to the ADC to convert the newly chosen group.

**Temperature sensor, V REFINT and V BAT internal channels**

• For the STM32F40x and STM32F41x devices, the temperature sensor is internally

connected to channel ADC1_IN16.

The internal reference voltage VREFINT is connected to ADC1_IN17.

• For the STM32F42x and STM32F43x devices, the temperature sensor is internally

connected to ADC1_IN18 channel which is shared with VBAT. Only one conversion, temperature sensor or VBAT, must be selected at a time. When the temperature sensor and VBAT conversion are set simultaneously, only the VBAT conversion is performed. The internal reference voltage VREFINT is connected to ADC1_IN17.

The V BAT channel (connected to channel ADC1_IN18) can also be converted as an injected or regular channel.

Note: The temperature sensor, V REFINT and the V BAT channel are available only on the master ADC1 peripheral.

**13.3.4 Single conversion mode**

In Single conversion mode the ADC does one conversion. This mode is started with the CONT bit at 0 by either:

• setting the SWSTART bit in the ADC_CR2 register (for a regular channel only)

• setting the JSWSTART bit (for an injected channel)

• external trigger (for a regular or injected channel)

Once the conversion of the selected channel is complete:

• If a regular channel was converted:

– The converted data are stored into the 16-bit ADC_DR register

– The EOC (end of conversion) flag is set

– An interrupt is generated if the EOCIE bit is set

• If an injected channel was converted:

– The converted data are stored into the 16-bit ADC_JDR1 register

– The JEOC (end of conversion injected) flag is set

– An interrupt is generated if the JEOCIE bit is set

Then the ADC stops.



432

--- Page 2 ---

**Analog-to-digital converter (ADC) RM0090**

**13.3.5 Continuous conversion mode**

In continuous conversion mode, the ADC starts a new conversion as soon as it finishes one. This mode is started with the CONT bit at 1 either by external trigger or by setting the SWSTRT bit in the ADC_CR2 register (for regular channels only).

After each conversion:

• If a regular group of channels was converted:

– The last converted data are stored into the 16-bit ADC_DR register

– The EOC (end of conversion) flag is set

– An interrupt is generated if the EOCIE bit is set

Note: Injected channels cannot be converted continuously. The only exception is when an injected channel is configured to be converted automatically after regular channels in continuous mode (using JAUTO bit), refer to Auto-injection section) .

**13.3.6 Timing diagram**

As shown in Figure 45 , the ADC needs a stabilization time of t STAB before it starts converting accurately. After the start of the ADC conversion and after 15 clock cycles, the EOC flag is set and the 16-bit ADC data register contains the result of the conversion.

**Figure 45. Timing diagram**

ADC_CLK

| Start 1st c |  | Start next |
|---|---|---|
|  | onversion |  |
| ADC conversion |  |  |
| tSTAB | Conversion time |  |
|  | (total conv. time) |  |


ADON

SWSTART/

JSWSTART

Start next conversion

Next ADC conversion ADC

EOC

Software clears the EOC bit

ai16047b

**13.3.7 Analog watchdog**

The AWD analog watchdog status bit is set if the analog voltage converted by the ADC is below a lower threshold or above a higher threshold. These thresholds are programmed in the 12 least significant bits of the ADC_HTR and ADC_LTR 16-bit registers. An interrupt can be enabled by using the AWDIE bit in the ADC_CR1 register.

The threshold value is independent of the alignment selected by the ALIGN bit in the ADC_CR2 register. The analog voltage is compared to the lower and higher thresholds before alignment.

Table 66 shows how the ADC_CR1 register should be configured to enable the analog watchdog on one or more channels.



--- Page 3 ---

**RM0090 Analog-to-digital converter (ADC)**

**Figure 46. Analog watchdog’s guarded area**

Analog voltage

Higher threshold HTR

Guarde d area

Lower threshold LTR

ai16048

**Table 66. Analog watchdog channel selection**

| Channels guarded by the analog watchdog | ADC CR1 register control bits (x = don’t care) _ |  |  |  |  |
|---|---|---|---|---|---|
| AWDSGL bit | AWDEN bit | JAWDEN bit |  |  |  |
| None | x | 0 |  |  |  |
| All injected channels | 0 | 1 |  |  |  |
| All regular channels | 0 | 1 | 0 |  |  |
| All regular and injected channels | 0 |  | 1 |  |  |
| Single(1) injected channel | 1 |  | 0 | 1 |  |
| Single(1) regular channel | 1 |  | 1 | 0 |  |
| Single (1) regular or injected channel | 1 |  |  |  |  |


1. Selected by the AWDCH[4:0] bits

**13.3.8 Scan mode**

This mode is used to scan a group of analog channels.

The Scan mode is selected by setting the SCAN bit in the ADC_CR1 register. Once this bit has been set, the ADC scans all the channels selected in the ADC_SQRx registers (for regular channels) or in the ADC_JSQR register (for injected channels). A single conversion is performed for each channel of the group. After each end of conversion, the next channel in the group is converted automatically. If the CONT bit is set, regular channel conversion does not stop at the last selected channel in the group but continues again from the first selected channel.

If the DMA bit is set, the direct memory access (DMA) controller is used to transfer the data converted from the regular group of channels (stored in the ADC_DR register) to SRAM after each regular channel conversion.

The EOC bit is set in the ADC_SR register:

• At the end of each regular group sequence if the EOCS bit is cleared to 0

• At the end of each regular channel conversion if the EOCS bit is set to 1 The data converted from an injected channel are always stored into the ADC_JDRx registers.



432

--- Page 4 ---

**Analog-to-digital converter (ADC) RM0090**

**13.3.9 Injected channel management**

**Triggered injection**

To use triggered injection, the JAUTO bit must be cleared in the ADC_CR1 register.

1. Start the conversion of a group of regular channels either by external trigger or by

setting the SWSTART bit in the ADC_CR2 register.

2. If an external injected trigger occurs or if the JSWSTART bit is set during the

conversion of a regular group of channels, the current conversion is reset and the injected channel sequence switches to Scan-once mode.

3. Then, the regular conversion of the regular group of channels is resumed from the last

interrupted regular conversion.

If a regular event occurs during an injected conversion, the injected conversion is not interrupted but the regular sequence is executed at the end of the injected sequence. Figure 47 shows the corresponding timing diagram.

Note: When using triggered injection, one must ensure that the interval between trigger events is longer than the injection sequence. For instance, if the sequence length is 30 ADC clock cycles (that is two conversions with a sampling time of 3 clock periods), the minimum interval between triggers must be 31 ADC clock cycles.

**Auto-injection**

If the JAUTO bit is set, then the channels in the injected group are automatically converted after the regular group of channels. This can be used to convert a sequence of up to 20 conversions programmed in the ADC_SQRx and ADC_JSQR registers.

In this mode, external trigger on injected channels must be disabled.

If the CONT bit is also set in addition to the JAUTO bit, regular channels followed by injected channels are continuously converted.

Note: It is not possible to use both the auto-injected and discontinuous modes simultaneously.

**Figure 47. Injected conversion latency**

ADCCLK

Injection event

Reset ADC

max latency (1)

SOC

ai16049

1. The maximum latency value can be found in the electrical characteristics of the STM32F40x and STM32F41x datasheets.



--- Page 5 ---

**RM0090 Analog-to-digital converter (ADC)**

**13.3.10 Discontinuous mode**

**Regular group**

This mode is enabled by setting the DISCEN bit in the ADC_CR1 register. It can be used to convert a short sequence of n conversions (n ≤ 8) that is part of the sequence of conversions selected in the ADC_SQRx registers. The value of n is specified by writing to the DISCNUM[2:0] bits in the ADC_CR1 register.

When an external trigger occurs, it starts the next n conversions selected in the ADC_SQRx registers until all the conversions in the sequence are done. The total sequence length is defined by the L[3:0] bits in the ADC_SQR1 register.

Example:

• n = 3, channels to be converted = 0, 1, 2, 3, 6, 7, 9, 10

• 1st trigger: sequence converted 0, 1, 2. An EOC event is generated at each

conversion.

• 2nd trigger: sequence converted 3, 6, 7. An EOC event is generated at each

conversion

• 3rd trigger: sequence converted 9, 10.An EOC event is generated at each conversion

• 4th trigger: sequence converted 0, 1, 2. An EOC event is generated at each conversion Note: When a regular group is converted in discontinuous mode, no rollover occurs.

When all subgroups are converted, the next trigger starts the conversion of the first subgroup. In the example above, the 4th trigger reconverts the channels 0, 1 and 2 in the 1st subgroup.

**Injected group**

This mode is enabled by setting the JDISCEN bit in the ADC_CR1 register. It can be used to convert the sequence selected in the ADC_JSQR register, channel by channel, after an external trigger event.

When an external trigger occurs, it starts the next channel conversions selected in the ADC_JSQR registers until all the conversions in the sequence are done. The total sequence length is defined by the JL[1:0] bits in the ADC_JSQR register.

Example:

n = 1, channels to be converted = 1, 2, 3

1st trigger: channel 1 converted

2nd trigger: channel 2 converted

3rd trigger: channel 3 converted and JEOC event generated

4th trigger: channel 1

Note: When all injected channels are converted, the next trigger starts the conversion of the first injected channel. In the example above, the 4th trigger reconverts the 1st injected channel 1.

It is not possible to use both the auto-injected and discontinuous modes simultaneously. Discontinuous mode must not be set for regular and injected groups at the same time. Discontinuous mode must be enabled only for the conversion of one group.



432

--- Page 6 ---

**Analog-to-digital converter (ADC) RM0090**

**13.4 Data alignment**

The ALIGN bit in the ADC_CR2 register selects the alignment of the data stored after conversion. Data can be right- or left-aligned as shown in Figure 48 and Figure 49 . The converted data value from the injected group of channels is decreased by the user- defined offset written in the ADC_JOFRx registers so the result can be a negative value. The SEXT bit represents the extended sign value.

For channels in a regular group, no offset is subtracted so only twelve bits are significant.

**Figure 48. Right alignment of 12-bit data**

Injected group



Regular group



ai16050

**Figure 49. Left alignment of 12-bit data**

Injected group



Regular group



ai16051

Special case: when left-aligned, the data are aligned on a half-word basis except when the resolution is set to 6-bit. in that case, the data are aligned on a byte basis as shown in Figure 50 .

**Figure 50. Left alignment of 6-bit data**

Injected group



Regular group



ai16052



--- Page 7 ---

**RM0090 Analog-to-digital converter (ADC)**

**13.5 Channel-wise programmable sampling time**

The ADC samples the input voltage for a number of ADCCLK cycles that can be modified using the SMP[2:0] bits in the ADC_SMPR1 and ADC_SMPR2 registers. Each channel can be sampled with a different sampling time.

The total conversion time is calculated as follows:

T conv = Sampling time + 12 cycles

Example:

With ADCCLK = 30 MHz and sampling time = 3 cycles:

T conv = 3 + 12 = 15 cycles = 0.5 µs with APB2 at 60 MHz


> [RM0090 ch.13 §13.13] ADC registers**

### 13.13.3 ADC control register 2 (ADC_CR2)**

**13.13.3 ADC control register 2 (ADC_CR2)**

Address offset: 0x08

Reset value: 0x0000 0000

31 30 29 28 27 26 25 24

23 22 21 20 19 18 17 16



15 14 13 12 11 10 9 8

7 6 5 4 3 2 1 0



Bit 31 Reserved, must be kept at reset value.

Bit 30 SWSTART: Start conversion of regular channels

This bit is set by software to start conversion and cleared by hardware as soon as the

conversion starts.

0: Reset state

1: Starts conversion of regular channels Note:

This bit can be set only when ADON = 1 otherwise no conversion is launched.

Bits 29:28 EXTEN: External trigger enable for regular channels

These bits are set and cleared by software to select the external trigger polarity and enable

the trigger of a regular group.

00: Trigger detection disabled

01: Trigger detection on the rising edge 10: Trigger detection on the falling edge 11: Trigger detection on both the rising and falling edges Bits 27:24 EXTSEL[3:0]: External event select for regular group

These bits select the external event used to trigger the start of conversion of a regular group:

0000: Timer 1 CC1 event

0001: Timer 1 CC2 event

0010: Timer 1 CC3 event

0011: Timer 2 CC2 event

0100: Timer 2 CC3 event

0101: Timer 2 CC4 event

0110: Timer 2 TRGO event

0111: Timer 3 CC1 event

1000: Timer 3 TRGO event

1001: Timer 4 CC4 event

1010: Timer 5 CC1 event

1011: Timer 5 CC2 event

1100: Timer 5 CC3 event

1101: Timer 8 CC1 event

1110: Timer 8 TRGO event

1111: EXTI line11

Bit 23 Reserved, must be kept at reset value.



--- Page 29 ---

**RM0090 Analog-to-digital converter (ADC)**

Bit 22 JSWSTART: Start conversion of injected channels

This bit is set by software and cleared by hardware as soon as the conversion starts.

0: Reset state

1: Starts conversion of injected channels

Note: This bit can be set only when ADON = 1 otherwise no conversion is launched.

Bits 21:20 JEXTEN: External trigger enable for injected channels

These bits are set and cleared by software to select the external trigger polarity and enable the trigger of an injected group.

00: Trigger detection disabled

01: Trigger detection on the rising edge

10: Trigger detection on the falling edge

11: Trigger detection on both the rising and falling edges

Bits 19:16 JEXTSEL[3:0]: External event select for injected group

These bits select the external event used to trigger the start of conversion of an injected group.

0000: Timer 1 CC4 event

0001: Timer 1 TRGO event

0010: Timer 2 CC1 event

0011: Timer 2 TRGO event

0100: Timer 3 CC2 event

0101: Timer 3 CC4 event

0110: Timer 4 CC1 event

0111: Timer 4 CC2 event

1000: Timer 4 CC3 event

1001: Timer 4 TRGO event

1010: Timer 5 CC4 event

1011: Timer 5 TRGO event

1100: Timer 8 CC2 event

1101: Timer 8 CC3 event

1110: Timer 8 CC4 event

1111: EXTI line15

Bits 15:12 Reserved, must be kept at reset value.

Bit 11 ALIGN: Data alignment

This bit is set and cleared by software. Refer to Figure 48 and Figure 49 .

0: Right alignment

1: Left alignment

Bit 10 EOCS: End of conversion selection

This bit is set and cleared by software.

0: The EOC bit is set at the end of each sequence of regular conversions. Overrun detection is enabled only if DMA=1.

1: The EOC bit is set at the end of each regular conversion. Overrun detection is enabled. Bit 9 DDS: DMA disable selection (for single ADC mode)

This bit is set and cleared by software.

0: No new DMA request is issued after the last transfer (as configured in the DMA controller) 1: DMA requests are issued as long as data are converted and DMA=1

Bit 8 DMA: Direct memory access mode (for single ADC mode)

This bit is set and cleared by software. Refer to the DMA controller chapter for more details. 0: DMA mode disabled

1: DMA mode enabled

RM0090 Rev 18 419/1749

432

--- Page 30 ---

**Analog-to-digital converter (ADC)**

**RM0090**

Bits 7:2 Reserved, must be kept at reset value.

Bit 1 CONT: Continuous conversion

This bit is set and cleared by software. If it is set, conversion takes place continuously until it

is cleared.

0: Single conversion mode

1: Continuous conversion mode

Bit 0 ADON: A/D Converter ON / OFF

This bit is set and cleared by software.

Note: 0: Disable ADC conversion and go to power down mode

1: Enable ADC

> [RM0090 ch.13 §13.13] ADC registers**

### 13.13.4 ADC sample time register 1 (ADC_SMPR1)**

**13.13.4 ADC sample time register 1 (ADC_SMPR1)**

Address offset: 0x0C

Reset value: 0x0000 0000

31 30 29 28 27 26 25 24

23 22 21 20 19 18 17 16

| Reserved | SMP18[2:0] | SMP17[2:0] | SMP16[2:0] | SMP15[2:1] |
|---|---|---|---|---|
| rw |  |  |  |  |


15 14 13 12 11 10 9 8

7 6 5 4 3 2 1 0

| SMP15 0 _ | SMP14[2:0] | SMP13[2:0] | SMP12[2:0] | SMP11[2:0] | SMP10[2:0] |
|---|---|---|---|---|---|
| rw |  |  |  |  |  |


Bits 31: 27 Reserved, must be kept at reset value.

Bits 26:0 SMPx[2:0]: Channel x sampling time selection

These bits are written by software to select the sampling time individually for each channel. During sampling cycles, the channel selection bits must remain unchanged.

Note: 000: 3 cycles

001: 15 cycles

010: 28 cycles

011: 56 cycles

100: 84 cycles

101: 112 cycles

110: 144 cycles

111: 480 cycles

> [RM0090 ch.13 §13.13] ADC registers**

### 13.13.5 ADC sample time register 2 (ADC_SMPR2)**

**13.13.5 ADC sample time register 2 (ADC_SMPR2)**

Address offset: 0x10

Reset value: 0x0000 0000

31 30 29 28 27 26 25 24

23 22 21 20 19 18 17 16

| Reserved | SMP9[2:0] | SMP8[2:0] | SMP7[2:0] | SMP6[2:0] | SMP5[2:1] |
|---|---|---|---|---|---|
| rw |  |  |  |  |  |


15 14 13 12 11 10 9 8

7 6 5 4 3 2 1 0

| SMP 5 0 _ | SMP4[2:0] | SMP3[2:0] | SMP2[2:0] | SMP1[2:0] | SMP0[2:0] |
|---|---|---|---|---|---|
| rw |  |  |  |  |  |


420/1749

RM0090 Rev 18

--- Page 31 ---

**RM0090**

**Analog-to-digital converter (ADC)**

Bits 31:30 Reserved, must be kept at reset value.

Bits 29:0 SMPx[2:0]: Channel x sampling time selection

These bits are written by software to select the sampling time individually for each channel. During sample cycles, the channel selection bits must remain unchanged.

Note: 000: 3 cycles

001: 15 cycles

010: 28 cycles

011: 56 cycles

100: 84 cycles

101: 112 cycles

110: 144 cycles

111: 480 cycles

**13.13.6**

**ADC injected channel data offset register x (ADC_JOFRx) (x=1..4)**

Address offset: 0x14-0x20

Reset value: 0x0000 0000

31 30 29 28 27 26 25 24

23 22 21 20 19 18 17 16 Reserved

15 14 13 12 11 10 9 8

7 6 5 4 3 2 1 0

| поступает в редуктор |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|
| с по | мощь | ю воро | нки и |  |  |  |  |  |  |  |  |


Bits 31:12 Reserved, must be kept at reset value.

Bits 11:0 JOFFSETx[11:0]: Data offset for injected channel x

These bits are written by software to define the offset to be subtracted from the raw converted data when converting injected channels. The conversion result can be read from

in the ADC_JDRx registers.

> [RM0090 ch.13 §13.13] ADC registers**

### 13.13.7 ADC watchdog higher threshold register (ADC_HTR)**

**13.13.7 ADC watchdog higher threshold register (ADC_HTR)**

Address offset: 0x24

Reset value: 0x0000 0FFF

31 30 29 28 27 26 25 24

23 22 21 20 19 18 17 16 Reserved

15 14 13 12 11 10 9 8

7 6 5 4 3 2 1 0



Bits 31:12 Reserved, must be kept at reset value.

Bits 11:0 HT[11:0]: Analog watchdog higher threshold

These bits are written by software to define the higher threshold for the analog watchdog.

RM0090 Rev 18 421/1749

432

--- Page 32 ---

**Analog-to-digital converter (ADC)**

**RM0090**

Note:

The software can write to these registers when an ADC conversion is ongoing. The programmed value will be effective when the next conversion is complete. Writing to this register is performed with a write delay that can create uncertainty on the effective time at

which the new value is programmed.

> [RM0090 ch.13 §13.13] ADC registers**

### 13.13.8 ADC watchdog lower threshold register (ADC_LTR)**

**13.13.8 ADC watchdog lower threshold register (ADC_LTR)**

Address offset: 0x28

Reset value: 0x0000 0000

31 30 29 28 27 26 25 24

23 22 21 20 19 18 17 16 Reserved

15 14 13 12 11 10 9 8

7 6 5 4 3 2 1 0

| Reserved | LT[11:0] |
|---|---|
| rw |  |


Bits 31:12 Reserved, must be kept at reset value.

Bits 11:0 LT[11:0]: Analog watchdog lower threshold

These bits are written by software to define the lower threshold for the analog watchdog.

Note:

The software can write to these registers when an ADC conversion is ongoing. The programmed value will be effective when the next conversion is complete. Writing to this register is performed with a write delay that can create uncertainty on the effective time at

which the new value is programmed.

> [RM0090 ch.13 §13.13] ADC registers**

### 13.13.9 ADC regular sequence register 1 (ADC_SQR1)**

**13.13.9 ADC regular sequence register 1 (ADC_SQR1)**

Address offset: 0x2C

Reset value: 0x0000 0000

31 30 29 28 27 26 25 24

23 22 21 20 19 18 17 16

| Reserved | L[3:0] | SQ16[4:1] |
|---|---|---|
| rw |  |  |


15 14 13 12 11 10 9 8

7 6 5 4 3 2 1 0

| SQ16 0 _ | SQ15[4:0] | SQ14[4:0] | SQ13[4:0] |  |
|---|---|---|---|---|
| rw |  |  |  | rw |


Bits 31:24 Reserved, must be kept at reset value.

Bits 23:20 L[3:0]: Regular channel sequence length

These bits are written by software to define the total number of conversions in the regular

channel conversion sequence.

0000: 1 conversion

0001: 2 conversions

...

1111: 16 conversions

Bits 19:15 SQ16[4:0]: 16th conversion in regular sequence

These bits are written by software with the channel number (0..18) assigned as the 16th in

the conversion sequence.



--- Page 33 ---

**RM0090**

**Analog-to-digital converter (ADC)**

Bits 14:10 SQ15[4:0]: 15th conversion in regular sequence Bits 9:5 SQ14[4:0]: 14th conversion in regular sequence Bits 4:0 SQ13[4:0]: 13th conversion in regular sequence

> [RM0090 ch.13 §13.13] ADC registers**

### 13.13.10 ADC regular sequence register 2 (ADC_SQR2)**

**13.13.10 ADC regular sequence register 2 (ADC_SQR2)**

Address offset: 0x30

Reset value: 0x0000 0000

31 30 29 28 27 26 25 24

23 22 21 20 19 18 17 16



15 14 13 12 11 10 9 8

7 6 5 4 3 2 1 0



Bits 31:30 Reserved, must be kept at reset value.

Bits 29:26 SQ12[4:0]: 12th conversion in regular sequence

These bits are written by software with the channel number (0..18 the sequence to be converted.

) assigned as the 12th in

Bits 24:20 SQ11[4:0]: 11th conversion in regular sequence Bits 19:15 SQ10[4:0]: 10th conversion in regular sequence Bits 14:10 SQ9[4:0]: 9th conversion in regular sequence Bits 9:5 SQ8[4:0]: 8th conversion in regular sequence Bits 4:0 SQ7[4:0]: 7th conversion in regular sequence

> [RM0090 ch.13 §13.13] ADC registers**

### 13.13.11 ADC regular sequence register 3 (ADC_SQR3)**

**13.13.11 ADC regular sequence register 3 (ADC_SQR3)**

Address offset: 0x34

Reset value: 0x0000 0000

31 30 29 28 27 26 25 24

23 22 21 20 19 18 17 16



15 14 13 12 11 10 9 8

7 6 5 4 3 2 1 0



Bits 31:30 Reserved, must be kept at reset value.

Bits 29:25 SQ6[4:0]: 6th conversion in regular sequence

These bits are written by software with the channel number (0..18) assigned as the 6th in the

sequence to be converted.

Bits 24:20 SQ5[4:0]: 5th conversion in regular sequence

RM0090 Rev 18 423/1749

432

--- Page 34 ---

**Analog-to-digital converter (ADC) RM0090**

Bits 19:15 SQ4[4:0]: 4th conversion in regular sequence

Bits 14:10 SQ3[4:0]: 3rd conversion in regular sequence

Bits 9:5 SQ2[4:0]: 2nd conversion in regular sequence

Bits 4:0 SQ1[4:0]: 1st conversion in regular sequence

> [RM0090 ch.13 §13.13] ADC registers**

### 13.13.12 ADC injected sequence register (ADC_JSQR)**

**13.13.12 ADC injected sequence register (ADC_JSQR)**

Address offset: 0x38

Reset value: 0x0000 0000

31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16

| Reserved | JL[1:0] | JSQ4[4:1] |
|---|---|---|
| rw |  |  |


15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0

| JSQ4[0] | JSQ3[4:0] | JSQ2[4:0] | JSQ1[4:0] |
|---|---|---|---|
| rw |  |  |  |


Bits 31:22 Reserved, must be kept at reset value.

Bits 21:20 JL[1:0]: Injected sequence length

These bits are written by software to define the total number of conversions in the injected channel conversion sequence.

00: 1 conversion

01: 2 conversions

10: 3 conversions

11: 4 conversions

Bits 19:15 JSQ4[4:0]: 4th conversion in injected sequence (when JL[1:0]=3, see note below)

These bits are written by software with the channel number (0..18) assigned as the 4th in the sequence to be converted.

Bits 14:10 JSQ3[4:0]: 3rd conversion in injected sequence (when JL[1:0]=3, see note below) Bits 9:5 JSQ2[4:0]: 2nd conversion in injected sequence (when JL[1:0]=3, see note below) Bits 4:0 JSQ1[4:0]: 1st conversion in injected sequence (when JL[1:0]=3, see note below)

Note: When JL[1:0]=3 (4 injected conversions in the sequencer), the ADC converts the channels in the following order: JSQ1[4:0], JSQ2[4:0], JSQ3[4:0], and JSQ4[4:0].

When JL=2 (3 injected conversions in the sequencer), the ADC converts the channels in the following order: JSQ2[4:0], JSQ3[4:0], and JSQ4[4:0].

When JL=1 (2 injected conversions in the sequencer), the ADC converts the channels in starting from JSQ3[4:0], and then JSQ4[4:0].

When JL=0 (1 injected conversion in the sequencer), the ADC converts only JSQ4[4:0] channel.

424/1749 RM0090 Rev 18

--- Page 35 ---

**RM0090**

**Analog-to-digital converter (ADC)**

> [RM0090 ch.13 §13.13] ADC registers**

### 13.13.13 ADC injected data register x (ADC_JDRx) (x= 1..4)**

**13.13.13 ADC injected data register x (ADC_JDRx) (x= 1..4)**

Address offset: 0x3C - 0x48

Reset value: 0x0000 0000

31 30 29 28 27 26 25 24

23 22 21 20 19 18 17 16 Reserved

15 14 13 12 11 10 9 8

7 6 5 4 3 2 1 0

| JDATA[15:0] |
|---|
| r |


Bits 31:16 Reserved, must be kept at reset value.

Bits 15:0 JDATA[15:0]: Injected data

These bits are read-only. They contain the conversion result from injected channel x. The

data are left -or right-aligned as shown in

Figure 48 and Figure 49 .

> [RM0090 ch.13 §13.13] ADC registers**

### 13.13.14 ADC regular data register (ADC_DR)**

**13.13.14 ADC regular data register (ADC_DR)**

Address offset: 0x4C

Reset value: 0x0000 0000

31 30 29 28 27 26 25 24

23 22 21 20 19 18 17 16 Reserved

15 14 13 12 11 10 9 8

7 6 5 4 3 2 1 0

| DATA[15:0] |
|---|
| r |


Bits 31:16 Reserved, must be kept at reset value. Bits 15:0 DATA[15:0]: Regular data

These bits are read-only. They contain the conversion result from the regular channels. The data are left- or right-aligned as shown in Figure 48 and

Figure 49 .



432

--- Page 36 ---

**Analog-to-digital converter (ADC) RM0090**


**14.5 DAC registers**

Refer to Section 1.1: List of abbreviations for registers for registers for a list of abbreviations used in register descriptions.

The peripheral registers have to be accessed by words (32 bits).

**14.5.1 DAC control register (DAC_CR)**

Address offset: 0x00

Reset value: 0x0000 0000

31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16



15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0



Bits 31:30 Reserved, must be kept at reset value.

Bits 29 DMAUDRIE2 : DAC channel2 DMA underrun interrupt enable

This bit is set and cleared by software.

0: DAC channel2 DMA underrun interrupt disabled

1: DAC channel2 DMA underrun interrupt enabled

Bit 28 DMAEN2 : DAC channel2 DMA enable

This bit is set and cleared by software.

0: DAC channel2 DMA mode disabled

1: DAC channel2 DMA mode enabled

Bits 27:24 MAMP2[3:0] : DAC channel2 mask/amplitude selector

These bits are written by software to select mask in wave generation mode or amplitude in triangle generation mode.

0000: Unmask bit0 of LFSR/ triangle amplitude equal to 1

0001: Unmask bits[1:0] of LFSR/ triangle amplitude equal to 3

0010: Unmask bits[2:0] of LFSR/ triangle amplitude equal to 7

0011: Unmask bits[3:0] of LFSR/ triangle amplitude equal to 15

0100: Unmask bits[4:0] of LFSR/ triangle amplitude equal to 31

0101: Unmask bits[5:0] of LFSR/ triangle amplitude equal to 63

0110: Unmask bits[6:0] of LFSR/ triangle amplitude equal to 127

0111: Unmask bits[7:0] of LFSR/ triangle amplitude equal to 255

1000: Unmask bits[8:0] of LFSR/ triangle amplitude equal to 511

1001: Unmask bits[9:0] of LFSR/ triangle amplitude equal to 1023

1010: Unmask bits[10:0] of LFSR/ triangle amplitude equal to 2047

≥ 1011: Unmask bits[11:0] of LFSR/ triangle amplitude equal to 4095

Bits 23:22 WAVE2[1:0] : DAC channel2 noise/triangle wave generation enable

These bits are set/reset by software.

00: wave generation disabled

01: Noise wave generation enabled

1x: Triangle wave generation enabled

Note: Only used if bit TEN2 = 1 (DAC channel2 trigger enabled)

RM0090 Rev 18 445/1749

454

--- Page 56 ---

**Digital-to-analog converter (DAC) RM0090**

Bits 21:19 TSEL2[2:0] : DAC channel2 trigger selection

These bits select the external event used to trigger DAC channel2

000: Timer 6 TRGO event

001: Timer 8 TRGO event

010: Timer 7 TRGO event

011: Timer 5 TRGO event

100: Timer 2 TRGO event

101: Timer 4 TRGO event

110: External line9

111: Software trigger

Note: Only used if bit TEN2 = 1 (DAC channel2 trigger enabled).

Bit 18 TEN2 : DAC channel2 trigger enable

This bit is set and cleared by software to enable/disable DAC channel2 trigger 0: DAC channel2 trigger disabled and data written into the DAC_DHRx register are transferred one APB1 clock cycle later to the DAC_DOR2 register

1: DAC channel2 trigger enabled and data from the DAC_DHRx register are transferred three APB1 clock cycles later to the DAC_DOR2 register

Note: When software trigger is selected, the transfer from the DAC_DHRx reg ister to the

DAC_DOR2 register takes only one APB1 clock cycle.

Bit 17 BOFF2 : DAC channel2 output buffer disable

This bit is set and cleared by software to enable/disable DAC channel2 output buffer. 0: DAC channel2 output buffer enabled

1: DAC channel2 output buffer disabled

Bit 16 EN2 : DAC channel2 enable

This bit is set and cleared by software to enable/disable DAC channel2.

0: DAC channel2 disabled

1: DAC channel2 enabled

Bits 15:14 Reserved, must be kept at reset value.

Bit 13 DMAUDRIE1 : DAC channel1 DMA Underrun Interrupt enable

This bit is set and cleared by software.

0: DAC channel1 DMA Underrun Interrupt disabled

1: DAC channel1 DMA Underrun Interrupt enabled

Bit 12 DMAEN1 : DAC channel1 DMA enable

This bit is set and cleared by software.

0: DAC channel1 DMA mode disabled

1: DAC channel1 DMA mode enabled

446/1749 RM0090 Rev 18

--- Page 57 ---

**RM0090 Digital-to-analog converter (DAC)**

Bits 11:8 MAMP1[3:0] : DAC channel1 mask/amplitude selector

These bits are written by software to select mask in wave generation mode or amplitude in triangle generation mode.

0000: Unmask bit0 of LFSR/ triangle amplitude equal to 1

0001: Unmask bits[1:0] of LFSR/ triangle amplitude equal to 3

0010: Unmask bits[2:0] of LFSR/ triangle amplitude equal to 7

0011: Unmask bits[3:0] of LFSR/ triangle amplitude equal to 15

0100: Unmask bits[4:0] of LFSR/ triangle amplitude equal to 31

0101: Unmask bits[5:0] of LFSR/ triangle amplitude equal to 63

0110: Unmask bits[6:0] of LFSR/ triangle amplitude equal to 127

0111: Unmask bits[7:0] of LFSR/ triangle amplitude equal to 255

1000: Unmask bits[8:0] of LFSR/ triangle amplitude equal to 511

1001: Unmask bits[9:0] of LFSR/ triangle amplitude equal to 1023

1010: Unmask bits[10:0] of LFSR/ triangle amplitude equal to 2047

≥ 1011: Unmask bits[11:0] of LFSR/ triangle amplitude equal to 4095

Bits 7:6 WAVE1[1:0] : DAC channel1 noise/triangle wave generation enable

These bits are set and cleared by software.

00: wave generation disabled

01: Noise wave generation enabled

1x: Triangle wave generation enabled

Note: Only used if bit TEN1 = 1 (DAC channel1 trigger enabled).

Bits 5:3 TSEL1[2:0] : DAC channel1 trigger selection

These bits select the external event used to trigger DAC channel1.

000: Timer 6 TRGO event

001: Timer 8 TRGO event

010: Timer 7 TRGO event

011: Timer 5 TRGO event

100: Timer 2 TRGO event

101: Timer 4 TRGO event

110: External line9

111: Software trigger

Note: Only used if bit TEN1 = 1 (DAC channel1 trigger enabled).

Bit 2 TEN1 : DAC channel1 trigger enable

This bit is set and cleared by software to enable/disable DAC channel1 trigger.

0: DAC channel1 trigger disabled and data written into the DAC_DHRx register are transferred one APB1 clock cycle later to the DAC_DOR1 register

1: DAC channel1 trigger enabled and data from the DAC_DHRx register are transferred three APB1 clock cycles later to the DAC_DOR1 register

Note: When software trigger is selected, the transfer from the DAC_DHRx register to the

DAC_DOR1 register takes only one APB1 clock cycle.

Bit 1 BOFF1 : DAC channel1 output buffer disable

This bit is set and cleared by software to enable/disable DAC channel1 output buffer.

0: DAC channel1 output buffer enabled

1: DAC channel1 output buffer disabled

Bit 0 EN1 : DAC channel1 enable

This bit is set and cleared by software to enable/disable DAC channel1.

0: DAC channel1 disabled

1: DAC channel1 enabled

RM0090 Rev 18 447/1749

454

--- Page 58 ---

**Digital-to-analog converter (DAC)**

**RM0090**

**14.5.2 DAC software trigger register (DAC_SWTRIGR)**

Address offset: 0x04

Reset value: 0x0000 0000

31 30 29 28 27 26 25 24

23 22 21 20 19 18 17 16 Reserved

15 14 13 12 11 10 9 8

7 6 5 4 3 2 1 0



Bits 31:2 Reserved, must be kept at reset value.

Bit 1 SWTRIG2 : DAC channel2 software trigger

This bit is set and cleared by software to enable/disable the software trigger.

0: Software trigger disabled

1: Software trigger enabled

Note:

This bit is cleared by hardware (one APB1 clock cycle later) once the DAC_DHR2 register value has been loaded into the DAC_DOR2 register.

Bit 0 SWTRIG1 : DAC channel1 software trigger

This bit is set and cleared by software to enable/disable the software trigger.

0: Software trigger disabled

1: Software trigger enabled

Note:

This bit is cleared by hardware (one APB1 clock cycle later) once the DAC_DHR1 register value has been loaded into the DAC_DOR1 register.

**14.5.3 DAC channel1 12-bit right-aligned data holding register (DAC_DHR12R1)**

Address offset: 0x08

Reset value: 0x0000 0000

31 30 29 28 27 26 25 24

23 22 21 20 19 18 17 16 Reserved

15 14 13 12 11 10 9 8

7 6 5 4 3 2 1 0



Bits 31:12 Reserved, must be kept at reset value.

Bits 11:0 DACC1DHR[11:0]

: DAC channel1 12-bit right-aligned data

These bits are written by software which specifies 12-bit data for DAC channel1.



--- Page 59 ---

**RM0090**

**Digital-to-analog converter (DAC)**

**14.5.4 DAC channel1 12-bit left aligned data holding register**

**(DAC_DHR12L1)**

Address offset: 0x0C

Reset value: 0x0000 0000

31 30 29 28 27 26 25 24

23 22 21 20 19 18 17 16 Reserved

15 14 13 12 11 10 9 8

7 6 5 4 3 2 1 0

| DACC1DHR[11:0] | Reserved |
|---|---|
| rw |  |


Bits 31:16 Reserved, must be kept at reset value.

Bits 15:4 DACC1DHR[11:0] : DAC channel1 12-bit left-aligned data

These bits are written by software which specifies 12-bit data for DAC channel1.

Bits 3:0 Reserved, must be kept at reset value.

**14.5.5 DAC channel1 8-bit right aligned data holding register**

**(DAC_DHR8R1)**

Address offset: 0x10

Reset value: 0x0000 0000

31 30 29 28 27 26 25 24

23 22 21 20 19 18 17 16 Reserved

15 14 13 12 11 10 9 8

7 6 5 4 3 2 1 0

| Reserved | DACC1DHR[7:0] |
|---|---|
| rw |  |


Bits 31:8 Reserved, must be kept at reset value.

Bits 7:0 DACC1DHR[7:0]

: DAC channel1 8-bit right-aligned data

These bits are written by software which specifies 8-bit data for DAC channel1.



454

--- Page 60 ---

**Digital-to-analog converter (DAC)**

**RM0090**

**14.5.6 DAC channel2 12-bit right aligned data holding register (DAC_DHR12R2)**

Address offset: 0x14

Reset value: 0x0000 0000

31 30 29 28 27 26 25 24

23 22 21 20 19 18 17 16 Reserved

15 14 13 12 11 10 9 8

7 6 5 4 3 2 1 0

| Reserved | DACC2DHR[11:0] |
|---|---|
| rw |  |


Bits 31:12 Reserved, must be kept at reset value.

Bits 11:0 DACC2DHR[11:0]

: DAC channel2 12-bit right-aligned data

These bits are written by software which specifies 12-bit data for DAC channel2.

**14.5.7 DAC channel2 12-bit left aligned data holding register (DAC_DHR12L2)**

Address offset: 0x18

Reset value: 0x0000 0000

31 30 29 28 27 26 25 24

23 22 21 20 19 18 17 16 Reserved

15 14 13 12 11 10 9 8

7 6 5 4 3 2 1 0

| DACC2DHR[11:0] | Reserved |
|---|---|
| rw |  |


Bits 31:16 Reserved, must be kept at reset value.

Bits 15:4 DACC2DHR[11:0]

: DAC channel2 12-bit left-aligned data

These bits are written by software which specify 12-bit data for DAC channel2.

Bits 3:0 Reserved, must be kept at reset value.

**14.5.8 DAC channel2 8-bit right-aligned data holding register (DAC_DHR8R2)**

Address offset: 0x1C

Reset value: 0x0000 0000

31 30 29 28 27 26 25 24

23 22 21 20 19 18 17 16 Reserved

15 14 13 12 11 10 9 8

7 6 5 4 3 2 1 0

| Reserved | DACC2DHR[7:0] |
|---|---|
| rw |  |


Bits 31:8 Reserved, must be kept at reset value.

Bits 7:0 DACC2DHR[7:0]

: DAC channel2 8-bit right-aligned data

These bits are written by software which specifies 8-bit data for DAC channel2.



--- Page 61 ---

**RM0090**

**Digital-to-analog converter (DAC)**

**14.5.9 Dual DAC 12-bit right-aligned data holding register (DAC_DHR12RD)**

Address offset: 0x20

Reset value: 0x0000 0000

31 30 29 28 27 26 25 24

23 22 21 20 19 18 17 16

| 12 |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|
| 5 |  |  |  |  |  |  | 5 | Винт |  |  |  |


15 14 13 12 11 10 9 8

7 6 5 4 3 2 1 0

| 6 | 7 Винт |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|
|  |  |  |  |  |  |  | 8 | Подши | пник по | воротно | го стола |


Bits 31:28 Reserved, must be kept at reset value.

Bits 27:16 DACC2DHR[11:0]

: DAC channel2 12-bit right-aligned data

These bits are written by software which specifies 12-bit data for DAC channel2.

Bits 15:12 Reserved, must be kept at reset value.

Bits 11:0 DACC1DHR[11:0]

: DAC channel1 12-bit right-aligned data

These bits are written by software which specifies 12-bit data for DAC channel1.

**14.5.10 DUAL DAC 12-bit left aligned data holding register (DAC_DHR12LD)**

Address offset: 0x24

Reset value: 0x0000 0000

31 30 29 28 27 26 25 24

23 22 21 20 19 18 17 16



15 14 13 12 11 10 9 8

7 6 5 4 3 2 1 0

| 1 5 2 7 | Винт 10 заглушки |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|
|  |  |  |  |  |  |  |  |  | 1 | Ве ше | дущая стерня |


Bits 31:20 DACC2DHR[11:0]

: DAC channel2 12-bit left-aligned data

These bits are written by software which specifies 12-bit data for DAC channel2.

Bits 19:16 Reserved, must be kept at reset value.

Bits 15:4 DACC1DHR[11:0]

: DAC channel1 12-bit left-aligned data

These bits are written by software which specifies 12-bit data for DAC channel1.

Bits 3:0 Reserved, must be kept at reset value.



454

--- Page 62 ---

**Digital-to-analog converter (DAC)**

**RM0090**

**14.5.11 DUAL DAC 8-bit right aligned data holding register (DAC_DHR8RD)**

Address offset: 0x28

Reset value: 0x0000 0000

31 30 29 28 27 26 25 24

23 22 21 20 19 18 17 16 Reserved

15 14 13 12 11 10 9 8

7 6 5 4 3 2 1 0

| DACC2DHR[7:0] | DACC1DHR[7:0] |
|---|---|
| rw |  |


Bits 31:16 Reserved, must be kept at reset value.

Bits 15:8 DACC2DHR[7:0]

: DAC channel2 8-bit right-aligned data

These bits are written by software which specifies 8-bit data for DAC channel2.

Bits 7:0 DACC1DHR[7:0]

: DAC channel1 8-bit right-aligned data

These bits are written by software which specifies 8-bit data for DAC channel1.

**14.5.12 DAC channel1 data output register (DAC_DOR1)**

Address offset: 0x2C

Reset value: 0x0000 0000

31 30 29 28 27 26 25 24

23 22 21 20 19 18 17 16 Reserved

15 14 13 12 11 10 9 8

7 6 5 4 3 2 1 0

| Reserved | DACC1DOR[11:0] |
|---|---|
| r |  |


Bits 31:12 Reserved, must be kept at reset value.

Bit 11:0 DACC1DOR[11:0] : DAC channel1 data output

These bits are read-only, they contain data output for DAC channel1.

**14.5.13 DAC channel2 data output register (DAC_DOR2)**

Address offset: 0x30

Reset value: 0x0000 0000

31 30 29 28 27 26 25 24

23 22 21 20 19 18 17 16 Reserved

15 14 13 12 11 10 9 8

7 6 5 4 3 2 1 0

| Reserved | DACC2DOR[11:0] |
|---|---|
| r |  |


Bits 31:12 Reserved, must be kept at reset value.

Bits 11:0 DACC2DOR[11:0] : DAC channel2 data output

These bits are read-only, they contain data output for DAC channel2.



--- Page 63 ---

**RM0090**

**Digital-to-analog converter (DAC)**

**14.5.14 DAC status register (DAC_SR)**

Address offset: 0x34

Reset value: 0x0000 0000

31 30 29 28 27 26 25 24

23 22 21 20 19 18 17 16

| Reserved | DMAUDR2 | Reserved |
|---|---|---|
| rc w1 _ |  |  |


15 14 13 12 11 10 9 8

7 6 5 4 3 2 1 0

| Reserved | DMAUDR1 | Reserved |
|---|---|---|
| rc w1 _ |  |  |


Bits 31:30 Reserved, must be kept at reset value.

Bit 29 DMAUDR2 : DAC channel2 DMA underrun flag

This bit is set by hardware and cleared by software (by writing it to 1).

0: No DMA underrun error condition occurred for DAC channel2

1: DMA underrun error condition occurred for DAC channel2 (the currently selected trigger is driving DAC channel2 conversion at a frequency higher than the DMA service capability rate)

Bits 28:14 Reserved, must be kept at reset value.

Bit 13 DMAUDR1 : DAC channel1 DMA underrun flag

This bit is set by hardware and cleared by software (by writing it to 1).

0: No DMA underrun error condition occurred for DAC channel1

1: DMA underrun error condition occurred for DAC channel1 (the currently selected trigger is driving DAC channel1 conversion at a frequency higher than the DMA service capability rate)

Bits 12:0 Reserved, must be kept at reset value.

**14.5.15 DAC register map**

Table 76 summarizes the DAC registers.

**Table 76. DAC register map**

| Offset | Register | 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 0x00 | DAC CR _ | Reserved | DMAUDRIE2 | DMAEN2 | MAMP2[3:0] | WAVE 2[2:0] | TSEL2[2:0] | TEN2 | BOFF2 | EN2 | Reserved | DMAUDRIE1 | DMAEN1 | MAMP1[3:0] | WAVE 1[2:0] | TSEL1[2 :0] | TEN1 | BOFF1 | EN1 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x04 | DAC _ SWTRIGR | Reserved | SWTRIG2 | SWTRIG1 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x08 | DAC _ DHR12R1 | Reserved | DACC1DHR[11:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x0C | DAC _ DHR12L1 | Reserved | DACC1DHR[11:0] | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x10 | DAC _ DHR8R1 | Reserved | DACC1DHR[7:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x14 | DAC _ DHR12R2 | Reserved | DACC2DHR[11:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x18 | DAC _ DHR12L2 | Reserved | DACC2DHR[11:0] | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |




454

--- Page 64 ---

**Digital-to-analog converter (DAC)**

**RM0090**

**Table 76. DAC register map (continued)**

| Offset | Register | 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 0x1C | DAC _ DHR8R2 | Reserved | DACC2DHR[7:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x20 | DAC _ DHR12RD | Reserved | DACC2DHR[11:0] | Reserved | DACC1DHR[11:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x24 | DAC _ DHR12LD | DACC2DHR[11:0] | Reserved | DACC1DHR[11:0] | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x28 | DAC _ DHR8RD | Reserved | DACC2DHR[7:0] | DACC1DHR[7:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x2C | DAC _ DOR1 | Reserved | DACC1DOR[11:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x30 | DAC _ DOR2 | Reserved | DACC2DOR[11:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x34 | DAC SR _ | Reserved | DMAUDR2 | Reserved | DMAUDR1 | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Refer to Section 2.3: Memory map

for the register boundary addresses.



--- Page 65 ---

**RM0090 Digital camera interface (DCMI)**


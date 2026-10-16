> [RM0090 ch.18 §18.4] TIM2 to TIM5 registers

### 18.4.9 TIMx capture/compare enable register (TIMx_CCER)**

**18.4.9 TIMx capture/compare enable register (TIMx_CCER)**

Address offset: 0x20
Reset value: 0x0000

| 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| CC4NP | Res. | CC4P | CC4E | CC3NP | Res. | CC3P | CC3E | CC2NP | Res. | CC2P | CC2E | CC1NP | Res. | CC1P | CC1E |
| r/w |  | r/w |  | r/w |  | r/w |  | r/w |  |  |  |  |  |  |  |


Bit 15 **CC4NP**: Capture/Compare 4 output Polarity.
Refer to CC1NP description
Bit 14 Reserved, must be kept at reset value.
Bit 13 **CC4P**: Capture/Compare 4 output Polarity.
refer to CC1P description
Bit 12 **CC4E**: Capture/Compare 4 output enable.
refer to CC1E description
Bit 11 **CC3NP**: Capture/Compare 3 output Polarity.
refer to CC1NP description
Bit 10 Reserved, must be kept at reset value.
Bit 9 **CC3P**: Capture/Compare 3 output Polarity.
refer to CC1P description

640/1749
RM0090 Rev 18
ST

--- Page 56 ---

RM0090
General-purpose timers (TIM2 to TIM5)

Bit 8 **CC3E**: *Capture/Compare 3 output enable.*
refer to CC1E description

Bit 7 **CC2NP**: *Capture/Compare 2 output Polarity.*
refer to CC1NP description

Bit 6 Reserved, must be kept at reset value.

Bit 5 **CC2P**: *Capture/Compare 2 output Polarity.*
refer to CC1P description

Bit 4 **CC2E**: *Capture/Compare 2 output enable.*
refer to CC1E description

Bit 3 **CC1NP**: *Capture/Compare 1 output Polarity.*
CC1 channel configured as output:
CC1NP must be kept cleared in this case.
CC1 channel configured as input:
This bit is used in conjunction with CC1P to define TI1FP1/TI2FP1 polarity. refer to CC1P description.

Bit 2 Reserved, must be kept at reset value.

Bit 1 **CC1P**: *Capture/Compare 1 output Polarity.*
**CC1 channel configured as output:**
0: OC1 active high
1: OC1 active low
**CC1 channel configured as input:**
CC1NP/CC1P bits select TI1FP1 and TI2FP1 polarity for trigger or capture operations.
00: noninverted/rising edge
Circuit is sensitive to TIxFP1 rising edge (capture, trigger in reset, external clock or trigger mode), TIxFP1 is not inverted (trigger in gated mode, encoder mode).
01: inverted/falling edge
Circuit is sensitive to TIxFP1 falling edge (capture, trigger in reset, external clock or trigger mode), TIxFP1 is inverted (trigger in gated mode, encoder mode).
10: reserved, do not use this configuration.
11: noninverted/both edges
Circuit is sensitive to both TIxFP1 rising and falling edges (capture, trigger in reset, external clock or trigger mode), TIxFP1 is not inverted (trigger in gated mode). This configuration must not be used for encoder mode.

Bit 0 **CC1E**: *Capture/Compare 1 output enable.*
**CC1 channel configured as output:**
0: Off - OC1 is not active
1: On - OC1 signal is output on the corresponding output pin
**CC1 channel configured as input:**
This bit determines if a capture of the counter value can actually be done into the input capture/compare register 1 (TIMx_CCR1) or not.
0: Capture disabled
1: Capture enabled

Table 99. Output control bit for standard OCx channels

| CCxE bit | OCx output state |
|---|---|
| 0 | Output Disabled (OCx=0, OCx_EN=0) |
| 1 | OCx=OCxREF + Polarity, OCx_EN=1 |


ST
RM0090 Rev 18
641/1749

--- Page 57 ---

**General-purpose timers (TIM2 to TIM5)**
RM0090

*Note:* The state of the external IO pins connected to the standard OCx channels depends on the OCx channel state and the GPIO registers.

> [RM0090 ch.18 §18.4] TIM2 to TIM5 registers

### 18.4.10 TIMx counter (TIMx_CNT)

### 18.4.10 TIMx counter (TIMx_CNT)

Address offset: 0x24
Reset value: 0x0000 0000

| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| CNT[31:16] (depending on timers) |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
| CNT[15:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bits 31:16 **CNT[31:16]**: High counter value (on TIM2 and TIM5).
Bits 15:0 **CNT[15:0]**: Counter value

> [RM0090 ch.18 §18.4] TIM2 to TIM5 registers

### 18.4.11 TIMx prescaler (TIMx_PSC)

### 18.4.11 TIMx prescaler (TIMx_PSC)

Address offset: 0x28
Reset value: 0x0000

| 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| PSC[15:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bits 15:0 **PSC[15:0]**: Prescaler value
The counter clock frequency CK_CNT is equal to f_CK_PSC / (PSC[15:0] + 1).
PSC contains the value to be loaded in the active prescaler register at each update event (including when the counter is cleared through UG bit of TIMx_EGR register or through trigger controller when configured in "reset mode").

> [RM0090 ch.18 §18.4] TIM2 to TIM5 registers

### 18.4.12 TIMx auto-reload register (TIMx_ARR)

### 18.4.12 TIMx auto-reload register (TIMx_ARR)

Address offset: 0x2C
Reset value: 0xFFFF FFFF

| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| ARR[31:16] (depending on timers) |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
| ARR[15:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


642/1749
RM0090 Rev 18
ST

--- Page 58 ---

RM0090
General-purpose timers (TIM2 to TIM5)

Bits 31:16 **ARR[31:16]**: High auto-reload value (on TIM2 and TIM5).
Bits 15:0 **ARR[15:0]**: Auto-reload value
ARR is the value to be loaded in the actual auto-reload register.
Refer to the *Section 18.3.1: Time-base unit* for more details about ARR update and behavior.
The counter is blocked while the auto-reload value is null.

> [RM0090 ch.18 §18.4] TIM2 to TIM5 registers

### 18.4.13 TIMx capture/compare register 1 (TIMx_CCR1)

### 18.4.13 TIMx capture/compare register 1 (TIMx_CCR1)

Address offset: 0x34
Reset value: 0x0000 0000

| CCR1[31:16] (depending on timers) |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 |
| rw/ro |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


| CCR1[15:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
| rw/ro |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bits 31:16 **CCR1[31:16]**: High Capture/Compare 1 value (on TIM2 and TIM5).
Bits 15:0 **CCR1[15:0]**: Low Capture/Compare 1 value

**If channel CC1 is configured as output:**
CCR1 is the value to be loaded in the actual capture/compare 1 register (preload value).
It is loaded permanently if the preload feature is not selected in the TIMx_CCMR1 register (bit OC1PE). Else the preload value is copied in the active capture/compare 1 register when an update event occurs.
The active capture/compare register contains the value to be compared to the counter TIMx_CNT and signaled on OC1 output.

**If channel CC1 is configured as input:**
CCR1 is the counter value transferred by the last input capture 1 event (IC1). The TIMx_CCR1 register is read-only and cannot be programmed.

> [RM0090 ch.18 §18.4] TIM2 to TIM5 registers

### 18.4.14 TIMx capture/compare register 2 (TIMx_CCR2)

### 18.4.14 TIMx capture/compare register 2 (TIMx_CCR2)

Address offset: 0x38
Reset value: 0x0000 0000

| CCR2[31:16] (depending on timers) |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 |
| rw/ro |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


| CCR2[15:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
| rw/ro |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


ST
RM0090 Rev 18
643/1749

--- Page 59 ---

General-purpose timers (TIM2 to TIM5) RM0090

Bits 31:16 **CCR2[31:16]**: High Capture/Compare 2 value (on TIM2 and TIM5).
Bits 15:0 **CCR2[15:0]**: Low Capture/Compare 2 value

**If channel CC2 is configured as output:**
CCR2 is the value to be loaded in the actual capture/compare 2 register (preload value).
It is loaded permanently if the preload feature is not selected in the TIMx_CCMR register (bit OC2PE). Else the preload value is copied in the active capture/compare 2 register when an update event occurs.
The active capture/compare register contains the value to be compared to the counter TIMx_CNT and signalled on OC2 output.

**If channel CC2 is configured as input:**
CCR2 is the counter value transferred by the last input capture 2 event (IC2). The TIMx_CCR2 register is read-only and cannot be programmed.

> [RM0090 ch.18 §18.4] TIM2 to TIM5 registers

### 18.4.15 TIMx capture/compare register 3 (TIMx_CCR3)

### 18.4.15 TIMx capture/compare register 3 (TIMx_CCR3)

Address offset: 0x3C
Reset value: 0x0000 0000

| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| CCR3[31:16] (depending on timers) |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| rw/ro |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
| CCR3[15:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| rw/ro |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bits 31:16 **CCR3[31:16]**: High Capture/Compare 3 value (on TIM2 and TIM5).
Bits 15:0 **CCR3[15:0]**: Low Capture/Compare value

**If channel CC3 is configured as output:**
CCR3 is the value to be loaded in the actual capture/compare 3 register (preload value).
It is loaded permanently if the preload feature is not selected in the TIMx_CCMR register (bit OC3PE). Else the preload value is copied in the active capture/compare 3 register when an update event occurs.
The active capture/compare register contains the value to be compared to the counter TIMx_CNT and signalled on OC3 output.

**If channel CC3 is configured as input:**
CCR3 is the counter value transferred by the last input capture 3 event (IC3). The TIMx_CCR3 register is read-only and cannot be programmed.

> [RM0090 ch.18 §18.4] TIM2 to TIM5 registers

### 18.4.16 TIMx capture/compare register 4 (TIMx_CCR4)

### 18.4.16 TIMx capture/compare register 4 (TIMx_CCR4)

Address offset: 0x40
Reset value: 0x0000 0000

| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| CCR4[31:16] (depending on timers) |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| rw/ro |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
| CCR4[15:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| rw/ro |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


644/1749 RM0090 Rev 18

--- Page 60 ---

RM0090
General-purpose timers (TIM2 to TIM5)

Bits 31:16 **CCR4[31:16]**: High Capture/Compare 4 value (on TIM2 and TIM5).
Bits 15:0 **CCR4[15:0]**: Low Capture/Compare value
1. if CC4 channel is configured as output (CC4S bits):
CCR4 is the value to be loaded in the actual capture/compare 4 register (preload value).
It is loaded permanently if the preload feature is not selected in the TIMx_CCMR register
(bit OC4PE). Else the preload value is copied in the active capture/compare 4 register
when an update event occurs.
The active capture/compare register contains the value to be compared to the counter
TIMx_CNT and signalled on OC4 output.
2. if CC4 channel is configured as input (CC4S bits in TIMx_CCMR4 register):
CCR4 is the counter value transferred by the last input capture 4 event (IC4). The
TIMx_CCR4 register is read-only and cannot be programmed.

> [RM0090 ch.18 §18.4] TIM2 to TIM5 registers

### 18.4.17 TIMx DMA control register (TIMx_DCR)

### 18.4.17 TIMx DMA control register (TIMx_DCR)

Address offset: 0x48
Reset value: 0x0000

| 15 | 14 | 13 | 12 11 10 9 8 | 7 | 6 | 5 | 4 3 2 1 0 |
|---|---|---|---|---|---|---|---|
| DBL[4:0] | DBA[4:0] |  |  |  |  |  |  |
| Reserved | rw | Reserved | rw |  |  |  |  |


Bits 15:13 Reserved, must be kept at reset value.
Bits 12:8 **DBL[4:0]**: DMA burst length
This 5-bit vector defines the number of DMA transfers (the timer recognizes a burst transfer
when a read or a write access is done to the TIMx_DMAR address).
00000: 1 transfer,
00001: 2 transfers,
00010: 3 transfers,
...
10001: 18 transfers.

Bits 7:5 Reserved, must be kept at reset value.
Bits 4:0 **DBA[4:0]**: DMA base address
This 5-bit vector defines the base-address for DMA transfers (when read/write access are
done through the TIMx_DMAR address). DBA is defined as an offset starting from the
address of the TIMx_CR1 register.
Example:
00000: TIMx_CR1,
00001: TIMx_CR2,
00010: TIMx_SMCR,
...

**Example**: Let us consider the following transfer: DBL = 7 transfers & DBA = TIMx_CR1. In this
case the transfer is done to/from 7 registers starting from the TIMx_CR1 address.

STI
RM0090 Rev 18
645/1749

--- Page 61 ---

General-purpose timers (TIM2 to TIM5)
RM0090


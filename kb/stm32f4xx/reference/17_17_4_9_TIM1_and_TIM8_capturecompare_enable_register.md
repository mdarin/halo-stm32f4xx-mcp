**17.4.9 TIM1 and TIM8 capture/compare enable register (TIMx_CCER)**

Address offset: 0x20

Reset value: 0x0000

15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0

| Reserved | CC4P | CC4E | CC3NP | CC3NE | CC3P | CC3E | CC2NP | CC2NE | CC2P | CC2E | CC1NP | CC1NE | CC1P | CC1E |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bits 15:14 Reserved, must be kept at reset value.

Bit 13 CC4P : Capture/Compare 4 output polarity

refer to CC1P description

Bit 12 CC4E : Capture/Compare 4 output enable

refer to CC1E description

Bit 11 CC3NP : Capture/Compare 3 complementary output polarity

refer to CC1NP description

Bit 10 CC3NE : Capture/Compare 3 complementary output enable

refer to CC1NE description

Bit 9 CC3P : Capture/Compare 3 output polarity

refer to CC1P description

Bit 8 CC3E : Capture/Compare 3 output enable

refer to CC1E description



--- Page 187 ---

**RM0090 Advanced-control timers (TIM1 and TIM8)**

Bit 7 CC2NP : Capture/Compare 2 complementary output polarity

refer to CC1NP description

Bit 6 CC2NE : Capture/Compare 2 complementary output enable

refer to CC1NE description

Bit 5 CC2P : Capture/Compare 2 output polarity

refer to CC1P description

Bit 4 CC2E : Capture/Compare 2 output enable

refer to CC1E description

Bit 3 CC1NP : Capture/Compare 1 complementary output polarity

CC1 channel configured as output:

0: OC1N active high.

1: OC1N active low.

CC1 channel configured as input:

This bit is used in conjunction with CC1P to define the polarity of TI1FP1 and TI2FP1. Refer to CC1P description.

Note: On channels having a complementary output, this bit is preloaded. If the CCPC bit is

set in the TIMx_CR2 register then the CC1NP active bit takes the new value from the preloaded bit only when a Commutation event is generated.

Note: This bit is not writable as soon as LOCK level 2 or 3 has been programmed (LOCK bits

in TIMx_BDTR register) and CC1S=”00” (the channel is configured in output).

Bit 2 CC1NE : Capture/Compare 1 complementary output enable

0: Off - OC1N is not active. OC1N level is then function of MOE, OSSI, OSSR, OIS1, OIS1N and CC1E bits.

1: On - OC1N signal is output on the corresponding output pin depending on MOE, OSSI, OSSR, OIS1, OIS1N and CC1E bits.

Note: On channels having a complementary output, this bit is preloaded. If the CCPC bit is

set in the TIMx_CR2 register then the CC1NE active bit takes the new value from the preloaded bit only when a Commutation event is generated.

RM0090 Rev 18 577/1749

588

--- Page 188 ---

**Advanced-control timers (TIM1 and TIM8) RM0090**

Bit 1 CC1P : Capture/Compare 1 output polarity

**CC1 channel configured as output:**

0: OC1 active high

1: OC1 active low

**CC1 channel configured as input:**

CC1NP/CC1P bits select the active polarity of TI1FP1 and TI2FP1 for trigger or capture operations.

00: non-inverted/rising edge

The circuit is sensitive to TIxFP1 rising edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is not inverted (trigger operation in gated mode or encoder mode).

01: inverted/falling edge

The circuit is sensitive to TIxFP1 falling edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is inverted (trigger operation in gated mode or encoder mode).

10: reserved, do not use this configuration.

11: non-inverted/both edges

The circuit is sensitive to both TIxFP1 rising and falling edges (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is not inverted (trigger operation in gated mode). This configuration must not be used in encoder mode.

Note: On channels having a complementary output, this bit is preloaded. If the CCPC bit is

set in the TIMx_CR2 register then the CC1P active bit takes the new value from the preloaded bit only when a Commutation event is generated.

Note: This bit is not writable as soon as LOCK level 2 or 3 has been programmed (LOCK bits

in TIMx_BDTR register).

Bit 0 CC1E : Capture/Compare 1 output enable

**CC1 channel configured as output:**

0: Off - OC1 is not active. OC1 level is then function of MOE, OSSI, OSSR, OIS1, OIS1N and CC1NE bits.

1: On - OC1 signal is output on the corresponding output pin depending on MOE, OSSI, OSSR, OIS1, OIS1N and CC1NE bits.

**CC1 channel configured as input:**

This bit determines if a capture of the counter value can actually be done into the input capture/compare register 1 (TIMx_CCR1) or not.

0: Capture disabled.

1: Capture enabled.

Note: On channels having a complementary output, this bit is preloaded. If the CCPC bit is

set in the TIMx_CR2 register then the CC1E active bit takes the new value from the preloaded bit only when a Commutation event is generated.

578/1749 RM0090 Rev 18

--- Page 189 ---

**RM0090 Advanced-control timers (TIM1 and TIM8)**

**Table 95. Output control bits for complementary OCx and OCxN channels with**

**break feature**

| Control bits | Output states(1) |  |  |  |  |  |
|---|---|---|---|---|---|---|
| MOE bit | OSSI bit | OSSR bit | CCxE bit | CCxNE bit | OCx output state | OCxN output state |
| 1 | X | 0 | Output Disabled (not driven by the timer), OCx=0, OCx EN=0 _ | Output Disabled (not driven by the timer), OCxN=0, OCxN EN=0 _ |  |  |
| 0 | 1 | Output Disabled (not driven by the timer), OCx=0, OCx EN=0 _ | OCxREF + Polarity OCxN=OCxREF xor CCxNP, OCxN EN=1 _ |  |  |  |
| 0 | 1 | 0 | OCxREF + Polarity OCx=OCxREF xor CCxP, OCx EN=1 _ | Output Disabled (not driven by the timer) OCxN=0, OCxN EN=0 _ |  |  |
| 0 | 1 | OCREF + Polarity + dead-time OCx EN=1 _ | Complementary to OCREF (not OCREF) + Polarity + dead-time OCxN EN=1 _ |  |  |  |
| 1 | 0 | Output Disabled (not driven by the timer) OCx=CCxP, OCx EN=0 _ | Output Disabled (not driven by the timer) OCxN=CCxNP, OCxN EN=0 _ |  |  |  |
| 1 | 0 | 1 | Off-State (output enabled with inactive state) OCx=CCxP, OCx EN=1 _ | OCxREF + Polarity OCxN=OCxREF xor CCxNP, OCxN EN=1 _ |  |  |
| 1 | 0 | OCxREF + Polarity OCx=OCxREF xor CCxP, OCx EN=1 _ | Off-State (output enabled with inactive state) OCxN=CCxNP, OCxN EN=1 _ |  |  |  |
| 1 | OCREF + Polarity + dead-time OCx EN=1 _ | Complementary to OCREF (not OCREF) + Polarity + dead-time OCxN EN=1 _ |  |  |  |  |
| 0 | X | Output disabled (not driven by the timer anymore). The output state is defined by the GPIO controller and can be High, Low or Hi-Z. |  |  |  |  |
| 1 | 0 |  |  |  |  |  |
| 0 | 1 | Off-State (output enabled with inactive state) Asynchronously: OCx=CCxP, OCx EN=1, OCxN=CCxNP, _ OCxN EN=1 _ Then if the clock is present: OCx=OISx and OCxN=OISxN after a dead-time, assuming that OISx and OISxN do not correspond to OCX and OCxN both in active state |  |  |  |  |
| 1 | 0 |  |  |  |  |  |
| 1 |  |  |  |  |  |  |


1. When both outputs of a channel are not used (CCxE = CCxNE = 0), the OISx, OISxN, CCxP and CCxNP bits must be kept cleared.

Note: The state of the external I/O pins connected to the complementary OCx and OCxN channels depends on the OCx and OCxN channel state and the GPIO registers.



588

--- Page 190 ---

**Advanced-control timers (TIM1 and TIM8)**

**RM0090**

**17.4.10 TIM1 and TIM8 counter (TIMx_CNT)**

Address offset: 0x24

Reset value: 0x0000

15 14 13 12 11 10 9 8

7 6 5 4 3 2 1 0

| CNT[15:0] |
|---|
| rw |


Bits 15:0 CNT[15:0] : Counter value

**17.4.11 TIM1 and TIM8 prescaler (TIMx_PSC)**

Address offset: 0x28

Reset value: 0x0000

15 14 13 12 11 10 9 8

7 6 5 4 3 2 1 0

| PSC[15:0] |
|---|
| rw |


Bits 15:0 PSC[15:0] : Prescaler value

The counter clock frequency (CK_CNT) is equal to f

CK_PSC / (PSC[15:0] + 1).

PSC contains the value to be loaded in the active prescaler register at each update event (including when the counter is cleared through UG bit of TIMx_EGR register or through

trigger controller when configured in “reset mode”).

**17.4.12 TIM1 and TIM8 auto-reload register (TIMx_ARR)**

Address offset: 0x2C

Reset value: 0xFFFF

15 14 13 12 11 10 9 8

7 6 5 4 3 2 1 0

| ARR[15:0] |
|---|
| rw |


Bits 15:0 ARR[15:0] : Auto-reload value

ARR is the value to be loaded in the actual auto-reload register. Refer to Section 17.3.1: Time-base unit

for more details about ARR update and behavior.

The counter is blocked while the auto-reload value is null.



--- Page 191 ---

**RM0090 Advanced-control timers (TIM1 and TIM8)**

**17.4.13 TIM1 and TIM8 repetition counter register (TIMx_RCR)**

Address offset: 0x30

Reset value: 0x0000

15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0

| Reserved | REP[7:0] |
|---|---|
| rw |  |


Bits 15:8 Reserved, must be kept at reset value.

Bits 7:0 REP[7:0] : Repetition counter value

These bits allow the user to set-up the update rate of the compare registers (i.e. periodic transfers from preload to active registers) when preload registers are enable, as well as the update interrupt generation rate, if this interrupt is enable.

Each time the REP_CNT related downcounter reaches zero, an update event is generated and it restarts counting from REP value. As REP_CNT is reloaded with REP value only at the repetition update event U_RC, any write to the TIMx_RCR register is not taken in account until the next repetition update event.

It means in PWM mode (REP+1) corresponds to:

– the number of PWM periods in edge-aligned mode

– the number of half PWM period in center-aligned mode.

**17.4.14 TIM1 and TIM8 capture/compare register 1 (TIMx_CCR1)**

Address offset: 0x34

Reset value: 0x0000

15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0

| CCR1[15:0] |
|---|
| rw/ro |


Bits 15:0 CCR1[15:0] : Capture/Compare 1 value

**If channel CC1 is configured as output :**

CCR1 is the value to be loaded in the actual capture/compare 1 register (preload value). It is loaded permanently if the preload feature is not selected in the TIMx_CCMR1 register (bit OC1PE). Else the preload value is copied in the active capture/compare 1 register when an update event occurs.

The active capture/compare register contains the value to be compared to the counter TIMx_CNT and signaled on OC1 output.

**If channel CC1 is configured as input :**

CCR1 is the counter value transferred by the last input capture 1 event (IC1). The TIMx_CCR1 register is read-only and cannot be programmed.



588

--- Page 192 ---

**Advanced-control timers (TIM1 and TIM8) RM0090**

**17.4.15 TIM1 and TIM8 capture/compare register 2 (TIMx_CCR2)**

Address offset: 0x38

Reset value: 0x0000

15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0

| CCR2[15:0] |
|---|
| rw/ro |


Bits 15:0 CCR2[15:0] : Capture/Compare 2 value

**If channel CC2 is configured as output :**

CCR2 is the value to be loaded in the actual capture/compare 2 register (preload value). It is loaded permanently if the preload feature is not selected in the TIMx_CCMR2 register (bit OC2PE). Else the preload value is copied in the active capture/compare 2 register when an update event occurs.

The active capture/compare register contains the value to be compared to the counter TIMx_CNT and signalled on OC2 output.

**If channel CC2 is configured as input :**

CCR2 is the counter value transferred by the last input capture 2 event (IC2). The TIMx_CCR2 register is read-only and cannot be programmed.

**17.4.16 TIM1 and TIM8 capture/compare register 3 (TIMx_CCR3)**

Address offset: 0x3C

Reset value: 0x0000

15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0

| CCR3[15:0] |
|---|
| rw/ro |


Bits 15:0 CCR3[15:0] : Capture/Compare value

**If channel CC3 is configured as output :**

CCR3 is the value to be loaded in the actual capture/compare 3 register (preload value). It is loaded permanently if the preload feature is not selected in the TIMx_CCMR3 register (bit OC3PE). Else the preload value is copied in the active capture/compare 3 register when an update event occurs.

The active capture/compare register contains the value to be compared to the counter TIMx_CNT and signalled on OC3 output.

**If channel CC3 is configured as input :**

CCR3 is the counter value transferred by the last input capture 3 event (IC3). The TIMx_CCR3 register is read-only and cannot be programmed.



--- Page 193 ---

**RM0090 Advanced-control timers (TIM1 and TIM8)**

**17.4.17 TIM1 and TIM8 capture/compare register 4 (TIMx_CCR4)**

Address offset: 0x40

Reset value: 0x0000

15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0

| CCR4[15:0] |
|---|
| rw/ro |


Bits 15:0 CCR4[15:0] : Capture/Compare value

**If channel CC4 is configured as output :**

CCR4 is the value to be loaded in the actual capture/compare 4 register (preload value). It is loaded permanently if the preload feature is not selected in the TIMx_CCMR4 register (bit OC4PE). Else the preload value is copied in the active capture/compare 4 register when an update event occurs.

The active capture/compare register contains the value to be compared to the counter TIMx_CNT and signalled on OC4 output.

**If channel CC4 is configured as input :**

CCR4 is the counter value transferred by the last input capture 4 event (IC4). The TIMx_CCR3 register is read-only and cannot be programmed.


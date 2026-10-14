**17.4.3 TIM1 and TIM8 slave mode control register (TIMx_SMCR)**

Address offset: 0x08

Reset value: 0x0000

15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0

| ETP | ECE | ETPS[1:0] | ETF[3:0] | MSM | TS[2:0] | Res. | SMS[2:0] |
|---|---|---|---|---|---|---|---|
| rw | Res. | rw |  |  |  |  |  |


Bit 15 ETP : External trigger polarity

This bit selects whether ETR or ETR is used for trigger operations

0: ETR is non-inverted, active at high level or rising edge.

1: ETR is inverted, active at low level or falling edge.

Bit 14 ECE : External clock enable

This bit enables External clock mode 2.

0: External clock mode 2 disabled

1: External clock mode 2 enabled. The counter is clocked by any active edge on the ETRF signal.

Note: 1: Setting the ECE bit has the same effect as selecting external clock mode 1 with

TRGI connected to ETRF (SMS=111 and TS=111).

2: It is possible to simultaneously use external clock mode 2 with the following slave modes: reset mode, gated mode and trigger mode. Nevertheless, TRGI must not be connected to ETRF in this case (TS bits must not be 111).

3: If external clock mode 1 and external clock mode 2 are enabled at the same time, the external clock input is ETRF.

Bits 13:12 ETPS[1:0] : External trigger prescaler

External trigger signal ETRP frequency must be at most 1/4 of TIMxCLK frequency. A prescaler can be enabled to reduce ETRP frequency. It is useful when inputting fast external clocks.

00: Prescaler OFF

01: ETRP frequency divided by 2

10: ETRP frequency divided by 4

11: ETRP frequency divided by 8



588

--- Page 176 ---

**Advanced-control timers (TIM1 and TIM8)**

**RM0090**

Bits 11:8 ETF[3:0] : External trigger filter

This bit-field then defines the frequency used to sample ETRP signal and the length of the digital filter applied to ETRP. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output:

0000: No filter, sampling is done at f

DTS

0001: f SAMPLING =f CK_INT , N=2

0010: f SAMPLING =f CK_INT , N=4

0011: f SAMPLING =f CK_INT , N=8

0100: f SAMPLING =f DTS /2, N=6

0101: f SAMPLING =f DTS /2, N=8

0110: f SAMPLING =f DTS /4, N=6

0111: f SAMPLING =f DTS /4, N=8

1000: f SAMPLING =f DTS /8, N=6

1001: f SAMPLING =f DTS /8, N=8

1010: f SAMPLING =f DTS /16, N=5

1011: f SAMPLING =f DTS /16, N=6

1100: f SAMPLING =f DTS /16, N=8

1101: f SAMPLING =f DTS /32, N=5

1110: f SAMPLING =f DTS /32, N=6

1111: f SAMPLING =f DTS /32, N=8

Bit 7 MSM: Master/slave mode

0: No action

1: The effect of an event on the trigger input (TRGI) is delayed to allow a perfect synchronization between the current timer and its slaves (through TRGO). It is useful if we

want to synchronize several timers on a single external event. Bits 6:4 TS[2:0]: Trigger selection

This bit-field selects the trigger input to be used to synchronize the counter.

000: Internal Trigger 0 (ITR0)

001: Internal Trigger 1 (ITR1)

010: Internal Trigger 2 (ITR2)

011: Internal Trigger 3 (ITR3)

100: TI1 Edge Detector (TI1F_ED)

101: Filtered Timer Input 1 (TI1FP1)

110: Filtered Timer Input 2 (TI2FP2)

111: External Trigger input (ETRF)

See Table 94

for more details on ITRx meaning for each Timer.

Note:

These bits must be changed only when they are not used (e.g. when SMS=000) to

avoid wrong edge detections at the transition. Bit 3 Reserved, must be kept at reset value.

566/1749

RM0090 Rev 18

--- Page 177 ---

**RM0090 Advanced-control timers (TIM1 and TIM8)**

Bits 2:0 SMS: Slave mode selection

When external signals are selected the active edge of the trigger signal (TRGI) is linked to the polarity selected on the external input (see Input Control register and Control Register description.

000: Slave mode disabled - if CEN = ‘1’ then the prescaler is clocked directly by the internal clock.

001: Encoder mode 1 - Counter counts up/down on TI2FP1 edge depending on TI1FP2 level.

010: Encoder mode 2 - Counter counts up/down on TI1FP2 edge depending on TI2FP1 level.

011: Encoder mode 3 - Counter counts up/down on both TI1FP1 and TI2FP2 edges depending on the level of the other input.

100: Reset Mode - Rising edge of the selected trigger input (TRGI) reinitializes the counter and generates an update of the registers.

101: Gated Mode - The counter clock is enabled when the trigger input (TRGI) is high. The counter stops (but is not reset) as soon as the trigger becomes low. Both start and stop of the counter are controlled.

110: Trigger Mode - The counter starts at a rising edge of the trigger TRGI (but it is not reset). Only the start of the counter is controlled.

111: External Clock Mode 1 - Rising edges of the selected trigger (TRGI) clock the counter. Note: The gated mode must not be used if TI1F_ED is selected as the trigger input

(TS=’100’). Indeed, TI1F_ED outputs 1 pulse for each transition on TI1F, whereas the gated mode checks the level of the trigger signal.

The clock of the slave timer must be enabled prior to receiving events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer.

**Table 94. TIMx Internal trigger connection**

| Slave TIM | ITR0 (TS = 000) | ITR1 (TS = 001) | ITR2 (TS = 010) | ITR3 (TS = 011) |
|---|---|---|---|---|
| TIM1 | TIM5 TRGO _ | TIM2 TRGO _ | TIM3 TRGO _ | TIM4 TRGO _ |
| TIM8 | TIM1 TRGO _ | TIM2 TRGO _ | TIM4 TRGO _ | TIM5 TRGO _ |


**17.4.4 TIM1 and TIM8 DMA/interrupt enable register (TIMx_DIER)**

Address offset: 0x0C

Reset value: 0x0000

15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0

| Res. | TDE | COMDE | CC4DE | CC3DE | CC2DE | CC1DE | UDE | BIE | TIE | COMIE | CC4IE | CC3IE | CC2IE | CC1IE | UIE |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bit 15 Reserved, must be kept at reset value.

Bit 14 TDE : Trigger DMA request enable

0: Trigger DMA request disabled

1: Trigger DMA request enabled

Bit 13 COMDE : COM DMA request enable

0: COM DMA request disabled

1: COM DMA request enabled

RM0090 Rev 18 567/1749

588

--- Page 178 ---

**Advanced-control timers (TIM1 and TIM8) RM0090**

Bit 12 CC4DE : Capture/Compare 4 DMA request enable

0: CC4 DMA request disabled

1: CC4 DMA request enabled

Bit 11 CC3DE : Capture/Compare 3 DMA request enable

0: CC3 DMA request disabled

1: CC3 DMA request enabled

Bit 10 CC2DE : Capture/Compare 2 DMA request enable

0: CC2 DMA request disabled

1: CC2 DMA request enabled

Bit 9 CC1DE : Capture/Compare 1 DMA request enable

0: CC1 DMA request disabled

1: CC1 DMA request enabled

Bit 8 UDE : Update DMA request enable

0: Update DMA request disabled

1: Update DMA request enabled

Bit 7 BIE : Break interrupt enable

0: Break interrupt disabled

1: Break interrupt enabled

Bit 6 TIE : Trigger interrupt enable

0: Trigger interrupt disabled

1: Trigger interrupt enabled

Bit 5 COMIE : COM interrupt enable

0: COM interrupt disabled

1: COM interrupt enabled

Bit 4 CC4IE : Capture/Compare 4 interrupt enable

0: CC4 interrupt disabled

1: CC4 interrupt enabled

Bit 3 CC3IE : Capture/Compare 3 interrupt enable

0: CC3 interrupt disabled

1: CC3 interrupt enabled

Bit 2 CC2IE : Capture/Compare 2 interrupt enable

0: CC2 interrupt disabled

1: CC2 interrupt enabled

Bit 1 CC1IE : Capture/Compare 1 interrupt enable

0: CC1 interrupt disabled

1: CC1 interrupt enabled

Bit 0 UIE : Update interrupt enable

0: Update interrupt disabled

1: Update interrupt enabled

568/1749 RM0090 Rev 18

--- Page 179 ---

**RM0090**

**Advanced-control timers (TIM1 and TIM8)**


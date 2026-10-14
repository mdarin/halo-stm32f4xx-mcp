> [RM0090 ch.18 §18.4] TIM2 to TIM5 registers


### 18.4.3 TIMx slave mode control register (TIMx_SMCR)

Address offset: 0x08
Reset value: 0x0000

| 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| ETP | ECE | ETPS[1:0] | ETF[3:0] | MSM | TS[2:0] | Res. | SMS[2:0] |  |  |  |  |  |  |  |  |
| rw |  | rw |  |  |  |  |  |  |  |  |  |  |  |  |  |


**Bit 15 ETP: External trigger polarity**
This bit selects whether ETR or $\overline{\text{ETR}}$ is used for trigger operations
0: ETR is noninverted, active at high level or rising edge
1: ETR is inverted, active at low level or falling edge

**Bit 14 ECE: External clock enable**
This bit enables External clock mode 2.
0: External clock mode 2 disabled
1: External clock mode 2 enabled. The counter is clocked by any active edge on the ETRF signal.
1: Setting the ECE bit has the same effect as selecting external clock mode 1 with TRGI connected to ETRF (SMS=111 and TS=111).
2: It is possible to simultaneously use external clock mode 2 with the following slave modes: reset mode, gated mode and trigger mode. Nevertheless, TRGI must not be connected to ETRF in this case (TS bits must not be 111).
3: If external clock mode 1 and external clock mode 2 are enabled at the same time, the external clock input is ETRF.

**Bits 13:12 ETPS: External trigger prescaler**
External trigger signal ETRP frequency must be at most 1/4 of CK_INT frequency. A prescaler can be enabled to reduce ETRP frequency. It is useful when inputting fast external clocks.
00: Prescaler OFF
01: ETRP frequency divided by 2
10: ETRP frequency divided by 4
11: ETRP frequency divided by 8

**Bits 11:8 ETF[3:0]: External trigger filter**
This bit-field then defines the frequency used to sample ETRP signal and the length of the digital filter applied to ETRP. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output:
0000: No filter, sampling is done at $f_{\text{DTS}}$
0001: $f_{\text{SAMPLING}}=f_{\text{CK\_INT}}$, N=2
0010: $f_{\text{SAMPLING}}=f_{\text{CK\_INT}}$, N=4
0011: $f_{\text{SAMPLING}}=f_{\text{CK\_INT}}$, N=8
0100: $f_{\text{SAMPLING}}=f_{\text{DTS}}/2$, N=6
0101: $f_{\text{SAMPLING}}=f_{\text{DTS}}/2$, N=8
0110: $f_{\text{SAMPLING}}=f_{\text{DTS}}/4$, N=6
0111: $f_{\text{SAMPLING}}=f_{\text{DTS}}/4$, N=8
1000: $f_{\text{SAMPLING}}=f_{\text{DTS}}/8$, N=6
1001: $f_{\text{SAMPLING}}=f_{\text{DTS}}/8$, N=8
1010: $f_{\text{SAMPLING}}=f_{\text{DTS}}/16$, N=5
1011: $f_{\text{SAMPLING}}=f_{\text{DTS}}/16$, N=6
1100: $f_{\text{SAMPLING}}=f_{\text{DTS}}/16$, N=8
1101: $f_{\text{SAMPLING}}=f_{\text{DTS}}/32$, N=5
1110: $f_{\text{SAMPLING}}=f_{\text{DTS}}/32$, N=6
1111: $f_{\text{SAMPLING}}=f_{\text{DTS}}/32$, N=8

630/1749
RM0090 Rev 18
ST

--- Page 46 ---

RM0090
General-purpose timers (TIM2 to TIM5)

**Bit 7 MSM: Master/Slave mode**
0: No action
1: The effect of an event on the trigger input (TRGI) is delayed to allow a perfect synchronization between the current timer and its slaves (through TRGO). It is useful if we want to synchronize several timers on a single external event.

**Bits 6:4 TS: Trigger selection**
This bit-field selects the trigger input to be used to synchronize the counter.
000: Internal Trigger 0 (ITR0)
001: Internal Trigger 1 (ITR1).
010: Internal Trigger 2 (ITR2).
011: Internal Trigger 3 (ITR3).
100: TI1 Edge Detector (TI1F_ED)
101: Filtered Timer Input 1 (TI1FP1)
110: Filtered Timer Input 2 (TI2FP2)
111: External Trigger input (ETRF)
See Table 98: TIMx internal trigger connection on page 632 for more details on ITRx meaning for each Timer.
*Note: These bits must be changed only when they are not used (e.g. when SMS=000) to avoid wrong edge detections at the transition.*

**Bit 3** Reserved, must be kept at reset value.

**Bits 2:0 SMS: Slave mode selection**
When external signals are selected the active edge of the trigger signal (TRGI) is linked to the polarity selected on the external input (see Input Control register and Control Register description.
000: Slave mode disabled - if CEN = '1 then the prescaler is clocked directly by the internal clock.
001: Encoder mode 1 - Counter counts up/down on TI2FP1 edge depending on TI1FP2 level.
010: Encoder mode 2 - Counter counts up/down on TI1FP2 edge depending on TI2FP1 level.
011: Encoder mode 3 - Counter counts up/down on both TI1FP1 and TI2FP2 edges depending on the level of the other input.
100: Reset Mode - Rising edge of the selected trigger input (TRGI) reinitializes the counter and generates an update of the registers.
101: Gated Mode - The counter clock is enabled when the trigger input (TRGI) is high. The counter stops (but is not reset) as soon as the trigger becomes low. Both start and stop of the counter are controlled.
110: Trigger Mode - The counter starts at a rising edge of the trigger TRGI (but it is not reset). Only the start of the counter is controlled.
111: External Clock Mode 1 - Rising edges of the selected trigger (TRGI) clock the counter.
*Note: The gated mode must not be used if TI1F_ED is selected as the trigger input (TS=100). Indeed, TI1F_ED outputs 1 pulse for each transition on TI1F, whereas the gated mode checks the level of the trigger signal.*

The clock of the slave timer must be enabled prior to receiving events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer.

STI
RM0090 Rev 18
631/1749

--- Page 47 ---

General-purpose timers (TIM2 to TIM5)
RM0090

Table 98. TIMx internal trigger connection

| Slave TIM | ITR0 (TS = 000) | ITR1 (TS = 001) | ITR2 (TS = 010) | ITR3 (TS = 011) |
|---|---|---|---|---|
| TIM2 | TIM1_TRGO | TIM8_TRGO | TIM3_TRGO | TIM4_TRGO |
| TIM3 | TIM1_TRGO | TIM2_TRGO | TIM5_TRGO | TIM4_TRGO |
| TIM4 | TIM1_TRGO | TIM2_TRGO | TIM3_TRGO | TIM8_TRGO |
| TIM5 | TIM2_TRGO | TIM3_TRGO | TIM4_TRGO | TIM8_TRGO |



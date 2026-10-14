> [RM0090 ch.19 §19.3] TIM9 to TIM14 functional description


### 19.3.6 PWM input mode (only for TIM9/12)

This mode is a particular case of input capture mode. The procedure is the same except:

*   Two ICx signals are mapped on the same TIx input.
*   These 2 ICx signals are active on edges with opposite polarity.
*   One of the two TIxFP signals is selected as trigger input and the slave mode controller is configured in reset mode.

For example, the user can measure the period (in TIMx_CCR1 register) and the duty cycle (in TIMx_CCR2 register) of the PWM applied on T1 using the following procedure (depending on CK_INT frequency and prescaler value):

1.  Select the active input for TIMx_CCR1: write the CC1S bits to '01' in the TIMx_CCMR1 register (TI1 selected).
2.  Select the active polarity for TI1FP1 (used both for capture in TIMx_CCR1 and counter clear): program the CC1P and CC1NP bits to '00' (active on rising edge).
3.  Select the active input for TIMx_CCR2: write the CC2S bits to '10' in the TIMx_CCMR1 register (TI1 selected).
4.  Select the active polarity for TI1FP2 (used for capture in TIMx_CCR2): program the CC2P and CC2NP bits to '11' (active on falling edge).
5.  Select the valid trigger input: write the TS bits to '101' in the TIMx_SMCR register (TI1FP1 selected).
6.  Configure the slave mode controller in reset mode: write the SMS bits to '100' in the TIMx_SMCR register.
7.  Enable the captures: write the CC1E and CC2E bits to '1' in the TIMx_CCER register.

**Figure 196. PWM input mode timing**

T1
TIMx_CNT 0004 0000 0001 0002 0003 0004 0000
TIMx_CCR1 0004
TIMx_CCR2 0002

IC1 capture
IC2 capture
reset counter

IC2 capture
pulse width
measurement

IC1 capture
period
measurement

ai15413

1. The PWM input mode can be used only with the TIMx_CH1/TIMx_CH2 signals due to the fact that only TI1FP1 and TI2FP2 are connected to the slave mode controller.

ST
RM0090 Rev 18
663/1749

--- Page 79 ---

**General-purpose timers (TIM9 to TIM14)**
RM0090

> [RM0090 ch.19 §19.3] TIM9 to TIM14 functional description

### 19.3.7 Forced output mode

### 19.3.7 Forced output mode

In output mode (CCxS bits = '00' in the TIMx_CCMRx register), each output compare signal (OCxREF and then OCx) can be forced to active or inactive level directly by software, independently of any comparison between the output compare register and the counter.

To force an output compare signal (OCxREF/OCx) to its active level, the user just needs to write '101' in the OCxM bits in the corresponding TIMx_CCMRx register. Thus OCxREF is forced high (OCxREF is always active high) and OCx get opposite value to CCxP polarity bit.

For example: CCxP='0' (OCx active high) => OCx is forced to high level.

The OCxREF signal can be forced low by writing the OCxM bits to '100' in the TIMx_CCMRx register.

Anyway, the comparison between the TIMx_CCRx shadow register and the counter is still performed and allows the flag to be set. Interrupt requests can be sent accordingly. This is described in the output compare mode section below.


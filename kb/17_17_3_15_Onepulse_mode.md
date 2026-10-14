**17.3.15 One-pulse mode**

One-pulse mode (OPM) is a particular case of the previous modes. It allows the counter to be started in response to a stimulus and to generate a pulse with a programmable length after a programmable delay.

Starting the counter can be controlled through the slave mode controller. Generating the waveform can be done in output compare mode or PWM mode. Select One-pulse mode by setting the OPM bit in the TIMx_CR1 register. This makes the counter stop automatically at the next update event UEV.

A pulse can be correctly generated only if the compare value is different from the counter initial value. Before starting (when the timer is waiting for the trigger), the configuration must be:

• In upcounting: CNT < CCRx ≤ ARR (in particular, 0 < CCRx)

• In downcounting: CNT > CCRx

**Figure 126. Example of one pulse mode.**

TI2

OC1REF

OC1

TIM1_ARR

**Counter TIM1_CCR1**

0 t DELAY t PULSE t

MS31099V2

For example the user may want to generate a positive pulse on OC1 with a length of t PULSE and after a delay of t DELAY as soon as a positive edge is detected on the TI2 input pin. Let’s use TI2FP2 as trigger 1:

• Map TI2FP2 to TI2 by writing CC2S=’01’ in the TIMx_CCMR1 register.

• TI2FP2 must detect a rising edge, write CC2P=’0’ and CC2NP=’0’ in the TIMx_CCER

register.

• Configure TI2FP2 as trigger for the slave mode controller (TRGI) by writing TS=’110’ in

the TIMx_SMCR register.

• TI2FP2 is used to start the counter by writing SMS to ‘110’ in the TIMx_SMCR register

(trigger mode).



588

--- Page 162 ---

**Advanced-control timers (TIM1 and TIM8) RM0090**

The OPM waveform is defined by writing the compare registers (taking into account the clock frequency and the counter prescaler).

• The t DELAY is defined by the value written in the TIMx_CCR1 register.

• The t PULSE is defined by the difference between the auto-reload value and the compare

value (TIMx_ARR - TIMx_CCR1).

• Let us say the user wants to build a waveform with a transition from ‘0’ to ‘1’ when a

compare match occurs and a transition from ‘1’ to ‘0’ when the counter reaches the auto-reload value. To do this, enable PWM mode 2 by writing OC1M=111 in the TIMx_CCMR1 register. The user can optionally enable the preload registers by writing OC1PE=’1’ in the TIMx_CCMR1 register and ARPE in the TIMx_CR1 register. In this case the compare value must be written in the TIMx_CCR1 register, the auto-reload value in the TIMx_ARR register, generate an update by setting the UG bit and wait for external trigger event on TI2. CC1P is written to ‘0’ in this example.

In our example, the DIR and CMS bits in the TIMx_CR1 register should be low. The user only wants one pulse (Single mode), so '1’ must be written in the OPM bit in the TIMx_CR1 register to stop the counter at the next update event (when the counter rolls over from the auto-reload value back to 0). When OPM bit in the TIMx_CR1 register is set to '0', so the Repetitive Mode is selected.

Particular case: OCx fast enable:

In One-pulse mode, the edge detection on TIx input set the CEN bit which enables the counter. Then the comparison between the counter and the compare value makes the output toggle. But several clock cycles are needed for these operations and it limits the minimum delay t DELAY min we can get.

If the user wants to output a waveform with the minimum delay, the OCxFE bit in the TIMx_CCMRx register must be set. Then OCxRef (and OCx) are forced in response to the stimulus, without taking in account the comparison. Its new level is the same as if a compare match had occurred. OCxFE acts only if the channel is configured in PWM1 or PWM2 mode.


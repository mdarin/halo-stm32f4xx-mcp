> [RM0090 ch.19 §19.3] TIM9 to TIM14 functional description


### 19.3.3 Clock selection

The counter clock can be provided by the following clock sources:

*   Internal clock (CK_INT)
*   External clock mode1 (for **TIM9** and **TIM12**): external input pin (Tlx)
*   Internal trigger inputs (ITRx) (for **TIM9** and **TIM12**): connecting the trigger output from another timer. Refer to *Using one timer as prescaler for another timer* for more details.

**Internal clock source (CK_INT)**

The internal clock source is the default clock source for TIM10/TIM11 and TIM13/TIM14.

For TIM9 and TIM12, the internal clock source is selected when the slave mode controller is disabled (SMS='000'). The CEN bit in the TIMx_CR1 register and the UG bit in the TIMx_EGR register are then used as control bits and can be changed only by software (except for UG which remains cleared). As soon as the CEN bit is programmed to 1, the prescaler is clocked by the internal clock CK_INT.

*Figure 190* shows the behavior of the control circuit and of the upcounter in normal mode, without prescaler.

**Figure 190. Control circuit in normal mode, internal clock divided by 1**

Internal clock
CEN=CNT_EN
UG
CNT_INIT
Counter clock = CK_CNT = CK_PSC
Counter register
31 32 33 34 35 36 00 01 02 03 04 05 06 07
MS31085V2

**External clock source mode 1(TIM9 and TIM12)**

This mode is selected when SMS='111' in the TIMx_SMCR register. The counter can count at each rising or falling edge on a selected input.

658/1749
RM0090 Rev 18
ST

--- Page 74 ---

RM0090
General-purpose timers (TIM9 to TIM14)

**Figure 191. TI2 external clock connection example**

For example, to configure the upcounter to count in response to a rising edge on the TI2 input, use the following procedure:

1. Configure channel 2 to detect rising edges on the TI2 input by writing CC2S = '01' in the TIMx_CCMR1 register.
2. Configure the input filter duration by writing the IC2F[3:0] bits in the TIMx_CCMR1 register (if no filter is needed, keep IC2F='0000').
3. Select the rising edge polarity by writing CC2P='0' and CC2NP='0' in the TIMx_CCER register.
4. Configure the timer in external clock mode 1 by writing SMS='111' in the TIMx_SMCR register.
5. Select TI2 as the trigger input source by writing TS='110' in the TIMx_SMCR register.
6. Enable the counter by writing CEN='1' in the TIMx_CR1 register.

**Note:**
The capture prescaler is not used for triggering, so no need to configure it.
When a rising edge occurs on TI2, the counter counts once and the TIF flag is set.
The delay between the rising edge on TI2 and the actual clock of the counter is due to the resynchronization circuit on TI2 input.

**Figure 192. Control circuit in external clock mode 1**

TI2
CNT_EN
Counter clock = CK_CNT = CK_PSC
Counter register 34 35 36
TIF
Write TIF = 0

ST
RM0090 Rev 18
659/1749

--- Page 75 ---

General-purpose timers (TIM9 to TIM14)
RM0090


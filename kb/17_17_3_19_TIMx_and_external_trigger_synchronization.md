**17.3.19 TIMx and external trigger synchronization**

The TIMx timer can be synchronized with an external trigger in several modes: Reset mode, Gated mode and Trigger mode.

**Slave mode: Reset mode**

The counter and its prescaler can be reinitialized in response to an event on a trigger input. Moreover, if the URS bit from the TIMx_CR1 register is low, an update event UEV is generated. Then all the preloaded registers (TIMx_ARR, TIMx_CCRx) are updated.

In the following example, the upcounter is cleared in response to a rising edge on TI1 input:

• Configure the channel 1 to detect rising edges on TI1. Configure the input filter duration

(in this example, we don’t need any filter, so we keep IC1F=0000). The capture prescaler is not used for triggering, so there’s no need to configure it. The CC1S bits select the input capture source only, CC1S = 01 in the TIMx_CCMR1 register. Write CC1P=0 and CC1NP=’0’ in TIMx_CCER register to validate the polarity (and detect rising edges only).

• Configure the timer in reset mode by writing SMS=100 in TIMx_SMCR register. Select

TI1 as the input source by writing TS=101 in TIMx_SMCR register.

• Start the counter by writing CEN=1 in the TIMx_CR1 register.

The counter starts counting on the internal clock, then behaves normally until TI1 rising edge. When TI1 rises, the counter is cleared and restarts from 0. In the meantime, the trigger flag is set (TIF bit in the TIMx_SR register) and an interrupt request, or a DMA request can be sent if enabled (depending on the TIE and TDE bits in TIMx_DIER register). The following figure shows this behavior when the auto-reload register TIMx_ARR=0x36. The delay between the rising edge on TI1 and the actual reset of the counter is due to the resynchronization circuit on TI1 input.

**Figure 130. Control circuit in reset mode**

TI1



UG

Counter clock = CK_CNT = CK_PSC



Counter register 00

TIF

MS31401V3



588

--- Page 168 ---

**Advanced-control timers (TIM1 and TIM8) RM0090**

**Slave mode: Gated mode**

The counter can be enabled depending on the level of a selected input.

In the following example, the upcounter counts only when TI1 input is low:

• Configure the channel 1 to detect low levels on TI1. Configure the input filter duration (in this example, we don’t need any filter, so we keep IC1F=0000). The capture prescaler is not used for triggering, so the user does not need to configure it. The CC1S bits select the input capture source only, CC1S=01 in TIMx_CCMR1 register. Write CC1P=1 and CC1NP=’0’ in TIMx_CCER register to validate the polarity (and detect low level only).

• Configure the timer in gated mode by writing SMS=101 in TIMx_SMCR register. Select TI1 as the input source by writing TS=101 in TIMx_SMCR register.

• Enable the counter by writing CEN=1 in the TIMx_CR1 register (in gated mode, the counter doesn’t start if CEN=0, whatever is the trigger input level).

The counter starts counting on the internal clock as long as TI1 is low and stops as soon as TI1 becomes high. The TIF flag in the TIMx_SR register is set both when the counter starts or stops.

The delay between the rising edge on TI1 and the actual stop of the counter is due to the resynchronization circuit on TI1 input.

**Figure 131. Control circuit in gated mode**

TI1

CNT_EN

Counter clock = ck_cnt = ck_psc



Counter register

TIF

Write TIF=0

MS31402V3



--- Page 169 ---

**RM0090 Advanced-control timers (TIM1 and TIM8)**

**Slave mode: Trigger mode**

The counter can start in response to an event on a selected input.

In the following example, the upcounter starts in response to a rising edge on TI2 input:

• Configure the channel 2 to detect rising edges on TI2. Configure the input filter duration (in this example, we don’t need any filter, so we keep IC2F=0000). The capture prescaler is not used for triggering, so there’s no need to configure it. The CC2S bits are configured to select the input capture source only, CC2S=01 in TIMx_CCMR1 register. Write CC2P=1 and CC2NP=0 in TIMx_CCER register to validate the polarity (and detect low level only).

• Configure the timer in trigger mode by writing SMS=110 in TIMx_SMCR register. Select TI2 as the input source by writing TS=110 in TIMx_SMCR register.

When a rising edge occurs on TI2, the counter starts counting on the internal clock and the TIF flag is set.

The delay between the rising edge on TI2 and the actual start of the counter is due to the resynchronization circuit on TI2 input.

**Figure 132. Control circuit in trigger mode**

TI2

CNT_EN

Counter clock = ck_cnt = ck_psc



Counter register 34

TIF

MS31403V2

**Slave mode: external clock mode 2 + trigger mode**

The external clock mode 2 can be used in addition to another slave mode (except external clock mode 1 and encoder mode). In this case, the ETR signal is used as external clock input, and another input can be selected as trigger input (in reset mode, gated mode or trigger mode). It is recommended not to select ETR as TRGI through the TS bits of TIMx_SMCR register.

In the following example, the upcounter is incremented at each rising edge of the ETR signal as soon as a rising edge of TI1 occurs:

1. Configure the external trigger input circuit by programming the TIMx_SMCR register as follows:

– ETF = 0000: no filter

– ETPS = 00: prescaler disabled

– ETP = 0: detection of rising edges on ETR and ECE=1 to enable the external clock

mode 2.

2. Configure the channel 1 as follows, to detect rising edges on TI:

– IC1F=0000: no filter.

– The capture prescaler is not used for triggering and does not need to be

configured.

– CC1S=01 in TIMx_CCMR1 register to select only the input capture source



588

--- Page 170 ---

**Advanced-control timers (TIM1 and TIM8) RM0090**

– CC1P=0 and CC1NP=’0’ in TIMx_CCER register to validate the polarity (and

detect rising edge only).

3. Configure the timer in trigger mode by writing SMS=110 in TIMx_SMCR register. Select

TI1 as the input source by writing TS=101 in TIMx_SMCR register.

A rising edge on TI1 enables the counter and sets the TIF flag. The counter then counts on ETR rising edges.

The delay between the rising edge of the ETR signal and the actual reset of the counter is due to the resynchronization circuit on ETRP input.

**Figure 133. Control circuit in external clock mode 2 + trigger mode**

TI1



CEN/CNT_EN

ETR

Counter clock = CK_CNT = CK_PSC

Counter register 34 36

TIF

MS33110V1

**17.3.20 Timer synchronization**

The TIM timers are linked together internally for timer synchronization or chaining. Refer to Section 18.3.15: Timer synchronization for details.

Note: The clock of the slave timer must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer.

**17.3.21 Debug mode**

When the microcontroller enters debug mode (Cortex ® -M4 with FPU core halted), the TIMx counter either continues to work normally or stops, depending on DBG_TIMx_STOP configuration bit in DBG module. For more details, refer to Section 38.16.2: Debug support for timers, watchdog, bxCAN and I 2 C .

For safety purposes, when the counter is stopped (DBG_TIMx_STOP = 1 in DBGMCU_APBx_FZ register), the outputs are disabled (as if the MOE bit was reset). The outputs can either be forced to an inactive state (OSSI bit = 1), or have their control taken over by the GPIO controller (OSSI bit = 0) to force them to Hi-Z.



--- Page 171 ---

**RM0090 Advanced-control timers (TIM1 and TIM8)**


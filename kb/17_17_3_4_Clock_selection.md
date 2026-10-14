**17.3.4 Clock selection**

The counter clock can be provided by the following clock sources:

• Internal clock (CK_INT)

• External clock mode1: external input pin

• External clock mode2: external trigger input ETR

• Internal trigger inputs (ITRx): using one timer as prescaler for another timer, for

example, the user can configure Timer 1 to act as a prescaler for Timer 2. Refer to Using one timer as prescaler for another timer for more details.

**Internal clock source (CK_INT)**

If the slave mode controller is disabled (SMS=000), then the CEN, DIR (in the TIMx_CR1 register) and UG bits (in the TIMx_EGR register) are actual control bits and can be changed only by software (except UG which remains cleared automatically). As soon as the CEN bit is written to 1, the prescaler is clocked by the internal clock CK_INT.

Figure 107 shows the behavior of the control circuit and the upcounter in normal mode, without prescaler.

**Figure 107. Control circuit in normal mode, internal clock divided by 1**

Internal clock

| ненорм останов | альных яв итесь. Ра | лен бот | ий, айт |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| вольно | пра |  |  |  |  |  |  |  |  |  |  |  |  |
| ть двиг мазки |  |  |  |  |  |  |  |  |  |  |  |  |  |
| мыват |  |  |  |  |  |  |  |  |  |  |  |  |  |
| корпус 31 | а. 3 2 | 33 | 34 | 35 | 36 | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 |


CEN=CNT_EN

UG

CNT_INIT

Counter clock = CK_CNT = CK_PSC

Counter register

MS31085V2

**External clock source mode 1**

This mode is selected when SMS=111 in the TIMx_SMCR register. The counter can count at each rising or falling edge on a selected input.



--- Page 143 ---

**RM0090 Advanced-control timers (TIM1 and TIM8)**

**Figure 108. TI2 external clock connection example**

TIMx_SMCR

TS[2:0]

or TI2F or

TI1F or Encoder

ITRx 0xx mode

TI1_ED

100 TRGI External clock TI2F_Rising TI1FP1 101 mode 1 CK_PSC TI2 Filter detector Edge TI2F_Falling 0 1 TI2FP2 ETRF 110 111 ETRF External clock mode 2

CK_INT Internal clock ICF[3:0] CC2P (internal clock) mode

TIMx_CCMR1 TIMx_CCER

ECE SMS[2:0] TIMx_SMCR

MS31196V1

For example, to configure the upcounter to count in response to a rising edge on the TI2 input, use the following procedure:

1. Configure channel 2 to detect rising edges on the TI2 input by writing CC2S = ‘01’ in the TIMx_CCMR1 register.

2. Configure the input filter duration by writing the IC2F[3:0] bits in the TIMx_CCMR1 register (if no filter is needed, keep IC2F=0000).

3. Select rising edge polarity by writing CC2P=0 and CC2NP=0 in the TIMx_CCER register.

4. Configure the timer in external clock mode 1 by writing SMS=111 in the TIMx_SMCR register.

5. Select TI2 as the trigger input source by writing TS=110 in the TIMx_SMCR register.

6. Enable the counter by writing CEN=1 in the TIMx_CR1 register.

Note: The capture prescaler is not used for triggering, so the user does not need to configure it. When a rising edge occurs on TI2, the counter counts once and the TIF flag is set.

The delay between the rising edge on TI2 and the actual clock of the counter is due to the resynchronization circuit on TI2 input.

RM0090 Rev 18 533/1749

588

--- Page 144 ---

**Advanced-control timers (TIM1 and TIM8)**

**RM0090**

**Figure 109. Control circuit in external clock mode 1**

TI2

CNT_EN

Counter clock = CK_CNT = CK_PSC

Counter register 34

35 36

TIF

Write TIF=0

MS31087V2

**External clock source mode 2**

This mode is selected by writing ECE=1 in the TIMx_SMCR register.

The counter can count at each rising or falling edge on the external trigger input ETR.

Figure 110 gives an overview of the external trigger input block.

**Figure 110. External trigger input block**

or TI2F or

TI1F or Encoder

mode

TRGI External clock

ETR

mode 1 CK_PSC Filter ETRF External clock downcounter mode 2 CK_INT Internal clock ETF[3:0] (internal clock) mode TIMx_SMCR

ETR pin 0 Divider ETRP

1 /1, /2, /4, /8 f DTS

ETP ETPS[1:0]

TIMx_SMCR TIMx_SMCR

ECE SMS[2:0] TIMx_SMCR

MS33116V1



--- Page 145 ---

**RM0090 Advanced-control timers (TIM1 and TIM8)**

For example, to configure the upcounter to count each 2 rising edges on ETR, use the following procedure:

1. As no filter is needed in this example, write ETF[3:0]=0000 in the TIMx_SMCR register.

2. Set the prescaler by writing ETPS[1:0]=01 in the TIMx_SMCR register

3. Select rising edge detection on the ETR pin by writing ETP=0 in the TIMx_SMCR

register

4. Enable external clock mode 2 by writing ECE=1 in the TIMx_SMCR register.

5. Enable the counter by writing CEN=1 in the TIMx_CR1 register.

The counter counts once each 2 ETR rising edges.

The delay between the rising edge on ETR and the actual clock of the counter is due to the resynchronization circuit on the ETRP signal.

**Figure 111. Control circuit in external clock mode 2**

f CK_INT

CNT_EN

ETR

ETRP

ETRF

Counter clock =

CK_INT =CK_PSC

Counter register 34 35 36

MS33111V2


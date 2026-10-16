### 18.3.3 Clock selection

The counter clock can be provided by the following clock sources:

*   Internal clock (CK_INT)
*   External clock mode1: external input pin (TIx)
*   External clock mode2: external trigger input (ETR) available on TIM2, TIM3 and TIM4 only.
*   Internal trigger inputs (ITRx): using one timer as prescaler for another timer, for example, Timer3 can be configured to act as a prescaler for Timer 2. Refer to *Using one timer as prescaler for another timer* for more details.

**Internal clock source (CK_INT)**

If the slave mode controller is disabled (SMS=000 in the TIMx_SMCR register), then the CEN, DIR (in the TIMx_CR1 register) and UG bits (in the TIMx_EGR register) are actual control bits and can be changed only by software (except UG which remains cleared automatically). As soon as the CEN bit is written to 1, the prescaler is clocked by the internal clock CK_INT.

*Figure 154* shows the behavior of the control circuit and the upcounter in normal mode, without prescaler.

**Figure 154. Control circuit in normal mode, internal clock divided by 1**

Internal clock
CEN=CNT_EN
UG
CNT_INIT
Counter clock = CK_CNT = CK_PSC
Counter register
31 32 33 34 35 36 00 01 02 03 04 05 06 07
MS31085V2

**External clock source mode 1**

This mode is selected when SMS=111 in the TIMx_SMCR register. The counter can count at each rising or falling edge on a selected input.

602/1749
RM0090 Rev 18
ST

--- Page 18 ---

RM0090
General-purpose timers (TIM2 to TIM5)

**Figure 155. TI2 external clock connection example**

TIMx_SMCR
TS[2:0]
or TI2F or TI1F
ITRx 0xx
T1_ED 100
T1FP1 101
TI2FP2 110
ETRF 111
TRGI
ETRF
CK_INT (internal clock)
Encoder mode
External clock mode 1
External clock mode 2
Internal clock mode
CK_PSC
ECE SMS[2:0]
TIMx_SMCR

TI2
Filter
Edge detector
TI2F_Rising 0
TI2F_Falling 1
ICF[3:0]
TIMx_CCMR1
CC2P
TIMx_CCER

For example, to configure the upcounter to count in response to a rising edge on the TI2 input, use the following procedure:

1. Configure channel 2 to detect rising edges on the TI2 input by writing CC2S= '01 in the TIMx_CCMR1 register.
2. Configure the input filter duration by writing the IC2F[3:0] bits in the TIMx_CCMR1 register (if no filter is needed, keep IC2F=0000).

*Note: The capture prescaler is not used for triggering, so there's no need to configure it.*

3. Select rising edge polarity by writing CC2P=0 and CC2NP=0 in the TIMx_CCER register.
4. Configure the timer in external clock mode 1 by writing SMS=111 in the TIMx_SMCR register.
5. Select TI2 as the input source by writing TS=110 in the TIMx_SMCR register.
6. Enable the counter by writing CEN=1 in the TIMx_CR1 register.

When a rising edge occurs on TI2, the counter counts once and the TIF flag is set.
The delay between the rising edge on TI2 and the actual clock of the counter is due to the resynchronization circuit on TI2 input.

MS31196V1

ST
RM0090 Rev 18
603/1749

--- Page 19 ---

General-purpose timers (TIM2 to TIM5)
RM0090

Figure 156. Control circuit in external clock mode 1

TI2
CNT_EN
Counter clock = CK_CNT = CK_PSC
Counter register 34 35 36
TIF
Write TIF=0
MS31087V2

External clock source mode 2
This mode is selected by writing ECE=1 in the TIMx_SMCR register.
The counter can count at each rising or falling edge on the external trigger input ETR.
Figure 157 gives an overview of the external trigger input block.

Figure 157. External trigger input block

ETR pin
ETR
0
1
ETP
TIMx_SMCR
Divider
/1, /2, /4, /8
ETPS[1:0]
TIMx_SMCR
ETRP
CK_INT
Filter
downcounter
ETF[3:0]
TIMx_SMCR
or TI2F or TI1F
Encoder mode
TRGI
External clock mode 1
ETRF
External clock mode 2
CK_INT
(internal clock)
Internal clock mode
ECE SMS[2:0]
TIMx_SMCR
CK_PSC
MS37365V1

For example, to configure the upcounter to count each 2 rising edges on ETR, use the following procedure:
1. As no filter is needed in this example, write ETF[3:0]=0000 in the TIMx_SMCR register.
2. Set the prescaler by writing ETPS[1:0]=01 in the TIMx_SMCR register
3. Select rising edge detection on the ETR pin by writing ETP=0 in the TIMx_SMCR register
4. Enable external clock mode 2 by writing ECE=1 in the TIMx_SMCR register.
5. Enable the counter by writing CEN=1 in the TIMx_CR1 register.
The counter counts once each 2 ETR rising edges.

604/1749
RM0090 Rev 18
ST

--- Page 20 ---

RM0090
General-purpose timers (TIM2 to TIM5)

The delay between the rising edge on ETR and the actual clock of the counter is due to the resynchronization circuit on the ETRP signal.

Figure 158. Control circuit in external clock mode 2

CK_INT
CNT_EN
ETR
ETRP
ETRF
Counter clock = CK_INT =CK_PSC
Counter register
34
35
36
MS37362V1

### 18.3.4 Capture/compare channels

Each Capture/Compare channel (see Figure 159) is built around a capture/compare register (including a shadow register), an input stage for capture (with digital filter, multiplexing and prescaler) and an output stage (with comparator and output control).

The input stage samples the corresponding Tlx input to generate a filtered signal TlxF. Then, an edge detector with polarity selection generates a signal (TlxFPx) which can be used as trigger input by the slave mode controller or as the capture command. It is prescaled before the capture register (ICxPS).

Figure 159. Capture/compare channel (example: channel 1 input stage)

TI1
f_DTS
filter downcounter
ICF[3:0]
TIMx_CCMR1
TI1F
Edge Detector
TI1F_Rising
TI1F_Falling
TI1F_ED
to the slave mode controller
TI1FP1
TI2FP1
01
10
IC1
divider
/1, /2, /4, /8
IC1PS
CC1P/CC1NP
TIMx_CCER
TRC
(from slave mode controller)
11
TI2F_rising
(from channel 2)
TI2F_falling
(from channel 2)
CC1S[1:0]
ICPS[1:0]
CC1E
TIMx_CCMR1
TIMx_CCER

The output stage generates an intermediate waveform which is then used for reference: OCxRef (active high). The polarity acts at the end of the chain.

ST
RM0090 Rev 18
605/1749

--- Page 21 ---

General-purpose timers (TIM2 to TIM5)
RM0090

Figure 160. Capture/compare channel 1 main circuit

APB Bus
MCU-peripheral interface
Read CCR1H
Read CCR1L
S
R
read_in_progress
8
high
(if 16-bit)
8
low
Capture/compare preload register
capture_transfer
compare_transfer
write_in_progress
S
write CCR1H
write CCR1L
R
Output mode
CC1S[1]
CC1S[0]
OC1PE
UEV
(from time base unit)
TIMx_CCMR1
CC1PE
Input mode
CC1S[1]
CC1S[0]
IC1PS
CC1E
Capture
Capture/compare shadow register
Comparator
Counter
CNT>CCR1
CNT=CCR1
CC1G
TIMx_EGR
MS33144V1

Figure 161. Output stage of capture/compare channel (channel 1)

ETRF
To the master mode controller
TIMx_CNT > TIMx_CCR1
TIMx_CNT = TIMx_CCR1
Output mode controller
oc1ref
0
1
CC1P
TIMx_CCER
Output Enable Circuit
OC1
OC1M[2:0]
TIMx_CCMR1
CC1E
TIMx_CCER
ai17187b

The capture/compare block is made of one preload register and one shadow register. Write and read always access the preload register.

In capture mode, captures are actually done in the shadow register, which is copied into the preload register.

In compare mode, the content of the preload register is copied into the shadow register which is compared to the counter.

606/1749
RM0090 Rev 18

--- Page 22 ---

RM0090
General-purpose timers (TIM2 to TIM5)


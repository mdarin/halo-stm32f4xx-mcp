> [RM0090 ch.19 §19.3] TIM9 to TIM14 functional description

### 19.3.4 Capture/compare channels**

**19.3.4 Capture/compare channels**

Each Capture/Compare channel is built around a capture/compare register (including a shadow register), a input stage for capture (with digital filter, multiplexing and prescaler) and an output stage (with comparator and output control).

*Figure 193 to Figure 195 give an overview of a capture/compare channel.*

The input stage samples the corresponding TIx input to generate a filtered signal TIxF. Then, an edge detector with polarity selection generates a signal (TIxFPx) which can be used as trigger input by the slave mode controller or as the capture command. It is prescaled before the capture register (ICxPS).

**Figure 193. Capture/compare channel (example: channel 1 input stage)**

TI1
f_DTS
Filter downcounter
TI1F
Edge detector
TI1F_Rising
TI1F_Falling
TI1F_ED
To the slave mode controller
0
1
TI1FP1
01
10
11
IC1
Divider
/1, /2, /4, /8
IC1PS
ICF[3:0]
TIMx_CCMR1
CC1P/CC1NP
TIMx_CCER
TI2F_Rising
(from channel 2)
TI2F_Falling
(from channel 2)
0
1
TI2FP1
TRC
(from slave mode controller)
CC1S[1:0]
ICPS[1:0]
CC1E
TIMx_CCMR1
TIMx_CCER
MS33115V1

The output stage generates an intermediate waveform which is then used for reference: OCxRef (active high). The polarity acts at the end of the chain.

660/1749
RM0090 Rev 18

--- Page 76 ---

RM0090
General-purpose timers (TIM9 to TIM14)

**Figure 194. Capture/compare channel 1 main circuit**

APB Bus
MCU-peripheral interface
Read CCR1H
Read CCR1L
S
R
read_in_progress
Capture/compare preload register
8
high
(if 16-bit)
8
low
write_in_progress
S
R
write CCR1H
write CCR1L
CC1S[1]
CC1S[0]
OC1PE
UEV
(from time base unit)
CC1PE
TIMx_CCMR1
CC1S[1]
CC1S[0]
Input mode
IC1PS
CC1E
CC1G
TIMx_EGR
Capture/compare shadow register
Capture
Counter
Comparator
CNT>CCR1
CNT=CCR1
MS31089V3

**Figure 195. Output stage of capture/compare channel (channel 1)**

To the master mode controller
CNT > CCR2
CNT = CCR2
Output mode controller
OC1_REF
0
1
Output enable circuit
OC1
OC2M[2:0]
TIMx_CCMR1
CC1P
TIMx_CCER
CC1E
TIMx_CCER
ai17720

The capture/compare block is made of one preload register and one shadow register. Write and read always access the preload register.

In capture mode, captures are actually done in the shadow register, which is copied into the preload register.

In compare mode, the content of the preload register is copied into the shadow register which is compared to the counter.


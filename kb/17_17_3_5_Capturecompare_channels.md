**17.3.5 Capture/compare channels**

Each Capture/Compare channel is built around a capture/compare register (including a shadow register), a input stage for capture (with digital filter, multiplexing and prescaler) and an output stage (with comparator and output control).

Figure 112 to Figure 115 give an overview of one Capture/Compare channel.

The input stage samples the corresponding TIx input to generate a filtered signal TIxF. Then, an edge detector with polarity selection generates a signal (TIxFPx) which can be used as trigger input by the slave mode controller or as the capture command. It is prescaled before the capture register (ICxPS).



588

--- Page 146 ---

**Advanced-control timers (TIM1 and TIM8)**

**RM0090**

**Figure 112. Capture/compare channel (example: channel 1 input stage)**

TI1F_ED

To the slave mode controller

TI1 TI1F_Rising

0 TI1FP1

Filter TI1F Edge



01

f DTS downcounter detector

1

TI2FP1 10 IC1 Divider IC1PS

/1, /2, /4, /8 TRC

ICF[3:0] CC1P/CC1NP

11

TIMx_CCMR1 TIMx_CCER

(from slave mode

controller)

TI2F_Rising

0

(from channel 2)



CC1E 1 TIMx_CCMR1 TIMx_CCER

TI2F_Falling

(from channel 2)

MS33115V1

The output stage generates an intermediate waveform that is then used for reference:

OCxRef (active high). The polarity acts at the end of the chain.

**Figure 113. Capture/compare channel 1 main circuit**

APB Bus

MCU-peripheral interface Read CCR1H S high 8 (if 16-bit)

8

low write_in_progress S write CCR1H

read_in_progress

R write CCR1L Output CC1S[1] compare_transfer mode CC1S[0]

Read CCR1L Capture/compare preload register R

capture_transfer

CC1S[1] Input

OC1PE

CC1S[0] mode Capture/compare shadow register

OC1PE UEV Comparator (from time TIMx_CCMR1

IC1PS Capture

base unit)

CC1E

CNT>CCR1

Counter

CNT=CCR1

CC1G

TIMx_EGR

MS31089V3



--- Page 147 ---

**RM0090**

**Advanced-control timers (TIM1 and TIM8)**

**Figure 114. Output stage of capture/compare channel (channel 1 to 3)**

To the master mode controller ETRF

0 Output OC1

enable

‘0’ x0 1 circuit

OC1REF OC1REFC

01

| Dead-time generator | OC1 DT _ |
|---|---|
| OC1N DT |  |


11 CC1P

CNT>CCR1 Output

mode Output

TIM1_CCER

CNT=CCR1 controller selector

11

‘0’ 10 0x 0 enable Output OC1N

(1)

OCxREF

OC5REF

1 circuit



TIM1_CCER







DTG[7:0] TIM1_CCMR1 TIM1_BDTR

CC1NP

TIM1_CCER TIM1_CCER TIM1_BDTR

MS35909V1

**Figure 115. Output stage of capture/compare channel (channel 4)**





The capture/compare block is made of one preload register and one shadow register. Write

and read always access the preload register.

In capture mode, captures are actually done in the shadow register, which is copied into the

preload register.

In compare mode, the content of the preload register is copied into the shadow register

which is compared to the counter.



588

--- Page 148 ---

**Advanced-control timers (TIM1 and TIM8) RM0090**


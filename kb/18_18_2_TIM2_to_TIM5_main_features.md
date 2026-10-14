# 18 General-purpose timers (TIM2 to TIM5)

This section applies to the whole STM32F4xx family, unless otherwise specified.

## 18.1 TIM2 to TIM5 introduction

The general-purpose timers consist of a 16-bit or 32-bit auto-reload counter driven by a programmable prescaler.

They may be used for a variety of purposes, including measuring the pulse lengths of input signals (*input capture*) or generating output waveforms (*output compare and PWM*).

Pulse lengths and waveform periods can be modulated from a few microseconds to several milliseconds using the timer prescaler and the RCC clock controller prescalers.

The timers are completely independent, and do not share any resources. They can be synchronized together as described in Section 18.3.15.

## 18.2 TIM2 to TIM5 main features

General-purpose TIMx timer features include:

*   16-bit (TIM3 and TIM4) or 32-bit (TIM2 and TIM5) up, down, up/down auto-reload counter.
*   16-bit programmable prescaler used to divide (also “on the fly”) the counter clock frequency by any factor between 1 and 65536.
*   Up to 4 independent channels for:
    *   Input capture
    *   Output compare
    *   PWM generation (Edge- and Center-aligned modes)
    *   One-pulse mode output
*   Synchronization circuit to control the timer with external signals and to interconnect several timers.
*   Interrupt/DMA generation on the following events:
    *   Update: counter overflow/underflow, counter initialization (by software or internal/external trigger)
    *   Trigger event (counter start, stop, initialization or count by internal/external trigger)
    *   Input capture
    *   Output compare
*   Supports incremental (quadrature) encoder and hall-sensor circuitry for positioning purposes
*   Trigger input for external clock or cycle-by-cycle current management

ST
RM0090 Rev 18
589/1749

--- Page 5 ---

General-purpose timers (TIM2 to TIM5)
RM0090

Figure 134. General-purpose timer block diagram

Internal clock (CK_INT)
TIMxCLK from RCC
TIMx_ETR
ETR
Polarity selection & edge detector & prescaler
ITR0
ITR1
ITR2
ITR3
ITR
ETR
ITR3
T1F_ED
ETRP
Input filter
ETRF
Trigger controller
TRGO
to other timers
to DAC/ADC
TGI
TRG1
Slave controller mode
Reset, enable, up, count
Encoder interface
T1IFP1
T12FP2
U
Auto-reload register
UI
Stop, clear or up/down
U
CK_PSC
PSC prescaler
CK_CNT
+/-
CNT counter
CC1
U
IC1
Prescaler
IC1PS
Capture/Compare 1 register
CC1
OC1REF
Output control
OC1
TIMx_CH1
TIMx_CH1
XOR
TI1
Input filter & edge detector
TI1FP1
TI1FP2
TRC
IC2
Prescaler
IC2PS
Capture/Compare 2 register
CC2
U
CC2
OC2REF
Output control
OC2
TIMx_CH2
TIMx_CH2
TI2
Input filter & edge detector
TI2FP1
TI2FP2
TRC
IC3
Prescaler
IC3PS
Capture/Compare 3 register
CC3
U
CC3
OC3REF
Output control
OC3
TIMx_CH3
TIMx_CH3
TI3
Input filter & edge detector
TI3FP3
TI3FP4
TRC
IC4
Prescaler
IC4PS
Capture/Compare 4 register
CC4
U
CC4
OC4REF
Output control
OC4
TIMx_CH4
TIMx_CH4
TI4
Input filter & edge detector
TI4FP3
TI4FP4
TRC
ETRF

Notes:
Reg
Preload registers transferred
to active registers on U event
according to control bit
Event
Interrupt & DMA output

MS19673V1

590/1749
RM0090 Rev 18

--- Page 6 ---

RM0090
General-purpose timers (TIM2 to TIM5)


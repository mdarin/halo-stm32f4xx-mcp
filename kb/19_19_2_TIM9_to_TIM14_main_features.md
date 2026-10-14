# 19 General-purpose timers (TIM9 to TIM14)

This section applies to the whole STM32F4xx family, unless otherwise specified.

## 19.1 TIM9 to TIM14 introduction

The TIM9 to TIM14 general-purpose timers consist of a 16-bit auto-reload counter driven by a programmable prescaler.

They may be used for a variety of purposes, including measuring the pulse lengths of input signals (input capture) or generating output waveforms (output compare, PWM).

Pulse lengths and waveform periods can be modulated from a few microseconds to several milliseconds using the timer prescaler and the RCC clock controller prescalers.

The TIM9 to TIM14 timers are completely independent, and do not share any resources. They can be synchronized together as described in Section 19.3.12.

## 19.2 TIM9 to TIM14 main features

### 19.2.1 TIM9/TIM12 main features

The features of the TIM9 to TIM14 general-purpose timers include:

*   16-bit auto-reload upcounter
*   16-bit programmable prescaler used to divide the counter clock frequency by any factor between 1 and 65536 (can be changed “on the fly”)
*   Up to 2 independent channels for:
    *   Input capture
    *   Output compare
    *   PWM generation (edge-aligned mode)
    *   One-pulse mode output
*   Synchronization circuit to control the timer with external signals and to interconnect several timers together
*   Interrupt generation on the following events:
    *   Update: counter overflow, counter initialization (by software or internal trigger)
    *   Trigger event (counter start, stop, initialization or count by internal trigger)
    *   Input capture
    *   Output compare

650/1749 RM0090 Rev 18 ST

--- Page 66 ---

RM0090
General-purpose timers (TIM9 to TIM14)

Figure 180. General-purpose timer block diagram (TIM9 and TIM12)

Internal clock (CK_INT)
Trigger controller
Slave controller mode
Reset, enable, up, count
ITR0
ITR1
ITR2
ITR3
ITR
T1F_ED
TRC
TGI
TRGI
T1IFP1
T2IFP2
U
Auto-reload register
Stop, Clear
U
U
CK_PSC
PSC prescaler
CK_CNT
+/-
CNT counter
CC11
U
CC1PS
Capture/Compare 1 register
CC11
OC1REF
Output control
OC1
TIMx_CH1
TIMx_CH1
TI1
Input filter & edge detector
TI1FP1
TI1FP2
IC1
Prescaler
TRC
TIMx_CH2
TI2
Input filter & edge detector
TI2FP1
TI2FP2
IC2
Prescaler
CC21
U
CC2PS
Capture/Compare 2 register
CC21
OC2REF
Output control
OC2
TIMx_CH2

Notes:
Reg Preload registers transferred to active registers on U event according to control bit
Event
Interrupt
ar17190b

19.2.2 TIM10/TIM11 and TIM13/TIM14 main features

The features of general-purpose timers TIM10/TIM11 and TIM13/TIM14 include:
* 16-bit auto-reload upcounter
* 16-bit programmable prescaler used to divide the counter clock frequency by any factor between 1 and 65536 (can be changed “on the fly”)
* independent channel for:
    - Input capture
    - Output compare
    - PWM generation (edge-aligned mode)
    - One-pulse mode output
* Interrupt generation on the following events:
    - Update: counter overflow, counter initialization (by software)
    - Input capture
    - Output compare

STI
RM0090 Rev 18
651/1749

--- Page 67 ---

General-purpose timers (TIM9 to TIM14)
RM0090

Figure 181. General-purpose timer block diagram (TIM10/11/13/14)

Internal clock (CK_INT)
Trigger Controller
Enable counter
U
Autoreload register
UI
Stop, Clear
U
CK_PSC
PSC prescaler
CK_CNT
+/-
CNT counter
CC1I
TI1
TI1FP1 IC1
Input filter & edge detector
Prescaler
IC1PS
U
Capture/Compare 1 register
CC1I
QC1REF
output control
OC1
TIMx_CH1
TIMx_CH1

Notes:
Reg
Preload registers transferred to active registers on U event according to control bit
event
interrupt & DMA output

ai17725c

652/1749
RM0090 Rev 18
ST

--- Page 68 ---

RM0090
General-purpose timers (TIM9 to TIM14)


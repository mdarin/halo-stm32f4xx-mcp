## 17 Advanced-control timers (TIM1 and TIM8)

This section applies to the whole STM32F4xx family, unless otherwise specified.

**17.1 TIM1 and TIM8 introduction**

The advanced-control timers (TIM1 and TIM8) consist of a 16-bit auto-reload counter driven by a programmable prescaler.

It may be used for a variety of purposes, including measuring the pulse lengths of input signals (input capture) or generating output waveforms (output compare, PWM, complementary PWM with dead-time insertion).

Pulse lengths and waveform periods can be modulated from a few microseconds to several milliseconds using the timer prescaler and the RCC clock controller prescalers.

The advanced-control (TIM1 and TIM8) and general-purpose (TIMx) timers are completely independent, and do not share any resources. They can be synchronized together as described in Section 17.3.20 .

RM0090 Rev 18 515/1749

588

--- Page 126 ---

**Advanced-control timers (TIM1 and TIM8) RM0090**

**17.2 TIM1 and TIM8 main features**

| TIM1 and TI • 16-bit u • 16-bit p frequen • Up to 4 – In – Ou | M8 timer features include: |  |
|---|---|---|
| p, down, up/down auto-relo rogrammable prescaler allo | ad counter. wing dividing (also “on the fly”) the counter clock |  |
| cy either by any factor betw | een 1 and 65536. |  |
| independent channels for: put Capture |  |  |
| tput Compare |  |  |
| – P – On • Comple • Synchr several • Repetit | WM generation (Edge and C e-pulse mode output | enter-aligned Mode) |
| mentary outputs with progr | ammable dead-time |  |
| onization circuit to control th | e timer with external signals and to interconnect |  |
| timers together. ion counter to update the ti | mer registers only after a given number of cycles of |  |
| the cou • Break i • Interrup – Up | nter. |  |
| nput to put the timer’s outp t/DMA generation on the fo | ut signals in reset state or in a known state. llowing events: |  |
| date: counter overflow/und | erflow, counter initialization (by software or |  |
| int – Tri – In – Ou – Br • Suppor | ernal/external trigger) |  |
| gger event (counter start, s | top, initialization or count by internal/external trigger) |  |
| put capture tput compare |  |  |
| eak input ts incremental (quadrature) | encoder and hall-sensor circuitry for positioning |  |
| purpos • Trigger | es |  |
| input for external clock or c | ycle-by-cycle current management |  |




--- Page 127 ---

**RM0090**

**Advanced-control timers (TIM1 and TIM8)**

**Figure 86. Advanced-control timer block diagram**

CK_TIM18 from RCC Internal clock (CK_INT)

**TIMx_ETR ETR Edge detector and Prescaler Polarity selection, ETRP ETRF**

Trigger

controller

TRGO To other timers

To DAC and ADC

Input filter

ITR0

TGI ITR1

ITR2 TRGI ITR3

Slave mode

controller

TIF_ED

Reset, Enable, UI

Up/Down,

Encoder Count REP Register

TI1FP1

interface U

TIMx_CH1 OC1 CK_CNT U AutoReload Register Repetition counter OC1N TIMx_CH1N CC4I CC4I DTG[7:0] registers TIMx_CH2 IC1PS OC1REF OC2 IC2PS CC3I U OC2REF CC3I TIMx_CH2N

TI2FP2

CK_PSC PSC



(prescaler) TIMx_CH1 TI1 Edge detector Input filter & TI1FP1 IC1 Prescaler

| Captu 1 | re R /C egi | om ste | pa r | re |  |
|---|---|---|---|---|---|
| Capture/C 2 Regi | om ste | pa r | re |  |  |
| re |  |  |  |  |  |


TI1FP2

TRC TIMx_CH2 TI2 Edge detector Input filter & TI2FP1 TI2FP2 IC2 Prescaler

U OC2N IC3PS CC2I 3 Register OC3REF CC2I OC3 TIMx_CH3 CC1I CC1I OC3N

**TRC TIMx_CH3 TI3 Edge detector Input filter & TI3FP3 TI3FP4 TRC IC3 Prescaler TIMx_CH4 TI4 Edge detector Input filter & TI4FP3 TI4FP4 TRC IC4 Prescaler TIMx_BKIN BRK Clock failure event from clock controller CSS (Clock Security System) Polarity selection BI**

U

IC4PS U Capture/Compare 4 Register OC4REF TIMx_CH3N

OC4

**TIMx_CH4**

Interrupt & DMA output Legend

Event DTG Output control

Reg

Preload registers transferred to active registers on U event according to control bit

MS39906V1



588

--- Page 128 ---

**Advanced-control timers (TIM1 and TIM8) RM0090**


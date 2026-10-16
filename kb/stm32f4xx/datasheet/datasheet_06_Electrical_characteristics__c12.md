> [DATASHEET ch.6 §6] Electrical characteristics

### 6.3.18 TIM timer characteristics

### 6.3.18 TIM timer characteristics

The parameters given in Table 52 and Table 53 are specified by design.

Refer to Section 6.3.16: I/O port characteristics for details on the input/output alternate function characteristics (output compare, input capture, external clock, PWM output).

ST
DS8626 Rev 12
121/206

--- Page 122 ---

Electrical characteristics
STM32F405xx, STM32F407xx

Table 52. Characteristics of TIMx connected to the APB1 domain(1)

| Symbol | Parameter | Conditions | Min | Max | Unit |
|---|---|---|---|---|---|
| tres(TIM) | Timer resolution time | AHB/APB1 prescaler distinct from 1, fTIMxCLK = 84 MHz | 1 | - | tTIMxCLK |
| 11.9 | - | ns |  |  |  |
| AHB/APB1 prescaler = 1, fTIMxCLK = 42 MHz | 1 | - | tTIMxCLK |  |  |
| 23.8 | - | ns |  |  |  |
| fEXT | Timer external clock frequency on CH1 to CH4 | fTIMxCLK = 84 MHzAP1= 42 MHz | 0 | fTIMxCLK/2 | MHz |
| 0 | 42 | MHz |  |  |  |
| ResTIM | Timer resolution | - | 16/32 | bit |  |
| tCOUNTER | 16-bit counter clock period when internal clock is selected | 1 | 65536 | tTIMxCLK |  |
| 0.0119 | 780 | µs |  |  |  |
| 32-bit counter clock period when internal clock is selected | 1 | - | tTIMxCLK |  |  |
| 0.0119 | 51130563 | µs |  |  |  |
| tMAX_COUNT | Maximum possible count | - | 65536 × 65536 | tTIMxCLK |  |
| - | 51.1 | s |  |  |  |


1. TIMx is used as a general term to refer to the TIM2, TIM3, TIM4, TIM5, TIM6, TIM7, and TIM12 timers.

122/206
DS8626 Rev 12

--- Page 123 ---

**STM32F405xx, STM32F407xx**
**Electrical characteristics**

**Table 53. Characteristics of TIMx connected to the APB2 domain(1)**

| Symbol | Parameter | Conditions | Min | Max | Unit |
|---|---|---|---|---|---|
| tres(TIM) | Timer resolution time | AHB/APB2 prescaler distinct from 1, fTIMxCLK = 168 MHz | 1 | - | tTIMxCLK |
| 5.95 | - | ns |  |  |  |
| AHB/APB2 prescaler = 1, fTIMxCLK = 84 MHz | 1 | - | tTIMxCLK |  |  |
| 11.9 | - | ns |  |  |  |
| fEXT | Timer external clock frequency on CH1 to CH4 | fTIMxCLK = 168 MHzAPB2 = 84 MHz | 0 | fTIMxCLK/2 | MHz |
| 0 | 84 | MHz |  |  |  |
| ResTIM | Timer resolution | - | 16 | bit |  |
| tCOUNTER | 16-bit counter clock period when internal clock is selected | 1 | 65536 | tTIMxCLK |  |
| tMAX_COUNT | Maximum possible count | - | 32768 | tTIMxCLK |  |


1. TIMx is used as a general term to refer to the TIM1, TIM8, TIM9, TIM10, and TIM11 timers.


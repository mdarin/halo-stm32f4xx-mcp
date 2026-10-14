> [RM0090 ch.18 §18.4] TIM2 to TIM5 registers


### 18.4.20 TIM5 option register (TIM5_OR)

Address offset: 0x50
Reset value: 0x0000

| 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved | TI4_RMP | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  | rw |  |  |  |  |


Bits 15:8 Reserved, must be kept at reset value.
Bits 7:6 **TI4_RMP**: Timer Input 4 remap
Set and cleared by software.
00: TIM5 Channel4 is connected to the GPIO: Refer to the Alternate function mapping table in the datasheets.
01: the LSI internal clock is connected to the TIM5_CH4 input for calibration purposes
10: the LSE internal clock is connected to the TIM5_CH4 input for calibration purposes
11: the RTC wakeup interrupt is connected to TIM5_CH4 input for calibration purposes.
Wakeup interrupt should be enabled.
Bits 5:0 Reserved, must be kept at reset value.

ST
RM0090 Rev 18
647/1749

--- Page 63 ---

General-purpose timers (TIM2 to TIM5)
RM0090

18.4.21 TIMx register map
TIMx registers are mapped as described in the table below:

Table 100. TIM2 to TIM5 register map and reset values

| Offset | Register | Bit values |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
| 0x00 | TIMx_CR1 | Reserved | CKD [1:0] | ARPE | CMS [1:0] | DIR | OPM | UDS | URS | CGF |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  | Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x04 | TIMx_CR2 | Reserved | MMS [2:0] | CCDS | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  | Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x08 | TIMx_SMCR | ETP | ECE | ETPS [1:0] | ETF [3:0] | MSM | Ts [2:0] | Reserved | sms [2:0] | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x0C | TIMx_DIER | TDE | COMDE | CC4DE | CC3DE | CC2DE | CC1DE | UDE | TIE | CC4IE | CC3IE | CC2IE | CC1IE | UIE | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x10 | TIMx_SR | CC4OF | CC3OF | CC2OF | CC1OF | TIF | Reserved | CC4IF | CC3IF | CC2IF | CC1IF | UIF | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x14 | TIMx_EGR | Reserved | TG | CC4G | CC3G | CC2G | CC1G | UG |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  | Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x18 | TIMx_CCMR1 | OC2CE | OC2M [2:0] | OC2PE | OC2FE | CC2S [1:0] | OC1CE | OC1M [2:0] | OC1PE | OC1FE | IC1F [3:0] | IC1 PSC [1:0] | IC1S [1:0] | CC1S [1:0] | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  | Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x1C | TIMx_CCMR2 | OC4CE | OC4M [2:0] | OC4PE | OC4FE | CC4S [1:0] | OC3CE | OC3M [2:0] | OC3PE | OC3FE | IC4F [3:0] | IC4 PSC [1:0] | IC4S [1:0] | CC4S [1:0] | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  | Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x20 | TIMx_CCER | CC4NP | CC4P | CC4E | CC3NP | CC3P | CC3E | CC2NP | CC2P | CC2E | CC1NP | CC1P | CC1E | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  | Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x24 | TIMx_CNT | CNT [15:0] (TIM2 and TIM5 only, reserved on the other timers) |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  | Reset value 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x28 | TIMx_PSC | PSC [15:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  | Reset value 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


648/1749
RM0090 Rev 18

--- Page 64 ---

RM0090
General-purpose timers (TIM2 to TIM5)

Table 100. TIM2 to TIM5 register map and reset values (continued)

Refer to Section 2.3: Memory map for the register boundary addresses.

| Offset | Register | Bit |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
| 0x2C | TIMx_ARR | ARR[31:16] (TIM2 and TIM5 only, reserved on the other timers) | ARR[15:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 1 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x30 | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x34 | TIMx_CCR1 | CCR1[31:16] (TIM2 and TIM5 only, reserved on the other timers) | CCR1[15:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x34 | Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x38 | TIMx_CCR2 | CCR2[31:16] (TIM2 and TIM5 only, reserved on the other timers) | CCR2[15:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x38 | Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x3C | TIMx_CCR3 | CCR3[31:16] (TIM2 and TIM5 only, reserved on the other timers) | CCR3[15:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x3C | Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x40 | TIMx_CCR4 | CCR4[31:16] (TIM2 and TIM5 only, reserved on the other timers) | CCR4[15:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x40 | Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x44 | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x48 | TIMx_DCR | Reserved | DBL[4:0] | DBA[4:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x48 | Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x4C | TIMx_DMAR | Reserved | DMAE[15:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x4C | Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x50 | TIM2_OR | Reserved | ITR1 RMP |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x50 | Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x50 | TIM5_OR | Reserved | IT4 RMP |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x50 | Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


649/1749
RM0090 Rev 18

--- Page 65 ---

General-purpose timers (TIM9 to TIM14) RM0090


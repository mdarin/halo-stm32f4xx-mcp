> [RM0090 ch.18 §18.4] TIM2 to TIM5 registers


### 18.4.8 TIMx capture/compare mode register 2 (TIMx_CCMR2)

Address offset: 0x1C
Reset value: 0x0000
Refer to the above CCMR1 register description.

| 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| OC4CE | OC4M[2:0] | OC4PE | OC4FE | CC4S[1:0] | OC3CE | OC3M[2:0] | OC3PE | OC3FE | CC3S[1:0] |  |  |  |  |  |  |
|  | IC4F[3:0] |  | IC4PSC[1:0] |  |  |  | IC3F[3:0] |  | IC3PSC[1:0] |  |  |  |  |  |  |
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


**Output compare mode**

Bit 15 **OC4CE**: Output compare 4 clear enable
Bits 14:12 **OC4M**: Output compare 4 mode
Bit 11 **OC4PE**: Output compare 4 preload enable
Bit 10 **OC4FE**: Output compare 4 fast enable
Bits 9:8 **CC4S**: Capture/Compare 4 selection
This bit-field defines the direction of the channel (input/output) as well as the used input.
00: CC4 channel is configured as output
01: CC4 channel is configured as input, IC4 is mapped on TI4
10: CC4 channel is configured as input, IC4 is mapped on TI3
11: CC4 channel is configured as input, IC4 is mapped on TRC. This mode is working only if an internal trigger input is selected through TS bit (TIMx_SMCR register)
*Note: CC4S bits are writable only when the channel is OFF (CC4E = 0 in TIMx_CCER).*

Bit 7 **OC3CE**: Output compare 3 clear enable
Bits 6:4 **OC3M**: Output compare 3 mode
Bit 3 **OC3PE**: Output compare 3 preload enable
Bit 2 **OC3FE**: Output compare 3 fast enable
Bits 1:0 **CC3S**: Capture/Compare 3 selection
This bit-field defines the direction of the channel (input/output) as well as the used input.
00: CC3 channel is configured as output
01: CC3 channel is configured as input, IC3 is mapped on TI3
10: CC3 channel is configured as input, IC3 is mapped on TI4
11: CC3 channel is configured as input, IC3 is mapped on TRC. This mode is working only if an internal trigger input is selected through TS bit (TIMx_SMCR register)
*Note: CC3S bits are writable only when the channel is OFF (CC3E = 0 in TIMx_CCER).*

STI
RM0090 Rev 18
639/1749

--- Page 55 ---

General-purpose timers (TIM2 to TIM5)
RM0090

**Input capture mode**

Bits 15:12 **IC4F**: Input capture 4 filter
Bits 11:10 **IC4PSC**: Input capture 4 prescaler
Bits 9:8 **CC4S**: Capture/Compare 4 selection
This bit-field defines the direction of the channel (input/output) as well as the used input.
00: CC4 channel is configured as output
01: CC4 channel is configured as input, IC4 is mapped on TI4
10: CC4 channel is configured as input, IC4 is mapped on TI3
11: CC4 channel is configured as input, IC4 is mapped on TRC. This mode is working only if an internal trigger input is selected through TS bit (TIMx_SMCR register)
*Note: CC4S bits are writable only when the channel is OFF (CC4E = 0 in TIMx_CCER).*

Bits 7:4 **IC3F**: Input capture 3 filter
Bits 3:2 **IC3PSC**: Input capture 3 prescaler
Bits 1:0 **CC3S**: Capture/Compare 3 selection
This bit-field defines the direction of the channel (input/output) as well as the used input.
00: CC3 channel is configured as output
01: CC3 channel is configured as input, IC3 is mapped on TI3
10: CC3 channel is configured as input, IC3 is mapped on TI4
11: CC3 channel is configured as input, IC3 is mapped on TRC. This mode is working only if an internal trigger input is selected through TS bit (TIMx_SMCR register)
*Note: CC3S bits are writable only when the channel is OFF (CC3E = 0 in TIMx_CCER).*


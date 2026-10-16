> [RM0090 ch.18 §18.4] TIM2 to TIM5 registers


### 18.4.7 TIMx capture/compare mode register 1 (TIMx_CCMR1)

Address offset: 0x18
Reset value: 0x0000

The channels can be used in input (capture mode) or in output (compare mode). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function in input and in output mode. For a given bit, OCxx describes its function when the channel is configured in output, ICxx describes its function when the channel is configured in input. Take care that the same bit can have a different meaning for the input stage and for the output stage.

| 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0 |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|
| OC2CE | OC2M[2:0] | OC2PE | OC2FE | CC2S[1:0] | OC1CE | OC1M[2:0] | OC1PE | OC1FE | CC1S[1:0] |
|  | IC2F[3:0] | IC2PSC[1:0] |  |  |  | IC1F[3:0] | IC1PSC[1:0] |  |  |
| rw |  |  |  |  |  |  |  |  |  |


**Output compare mode**

Bit 15 **OC2CE**: Output compare 2 clear enable
Bits 14:12 **OC2M[2:0]**: Output compare 2 mode
Bit 11 **OC2PE**: Output compare 2 preload enable
Bit 10 **OC2FE**: Output compare 2 fast enable
Bits 9:8 **CC2S[1:0]**: Capture/Compare 2 selection
This bit-field defines the direction of the channel (input/output) as well as the used input.
00: CC2 channel is configured as output
01: CC2 channel is configured as input, IC2 is mapped on TI2
10: CC2 channel is configured as input, IC2 is mapped on TI1
11: CC2 channel is configured as input, IC2 is mapped on TRC. This mode is working only if an internal trigger input is selected through the TS bit (TIMx_SMCR register)
*Note: CC2S bits are writable only when the channel is OFF (CC2E = 0 in TIMx_CCER).*

Bit 7 **OC1CE**: Output compare 1 clear enable
OC1CE: Output Compare 1 Clear Enable
0: OC1Ref is not affected by the ETRF input
1: OC1Ref is cleared as soon as a High level is detected on ETRF input

636/1749
RM0090 Rev 18
STI

--- Page 52 ---

RM0090
General-purpose timers (TIM2 to TIM5)

**Bits 6:4 OC1M:** Output compare 1 mode
These bits define the behavior of the output reference signal OC1REF from which OC1 and OC1N are derived. OC1REF is active high whereas OC1 and OC1N active level depends on CC1P and CC1NP bits.
000: Frozen - The comparison between the output compare register TIMx_CCR1 and the counter TIMx_CNT has no effect on the outputs.(this mode is used to generate a timing base).
001: Set channel 1 to active level on match. OC1REF signal is forced high when the counter TIMx_CNT matches the capture/compare register 1 (TIMx_CCR1).
010: Set channel 1 to inactive level on match. OC1REF signal is forced low when the counter TIMx_CNT matches the capture/compare register 1 (TIMx_CCR1).
011: Toggle - OC1REF toggles when TIMx_CNT=TIMx_CCR1.
100: Force inactive level - OC1REF is forced low.
101: Force active level - OC1REF is forced high.
110: PWM mode 1 - In upcounting, channel 1 is active as long as TIMx_CNT<TIMx_CCR1 else inactive. In downcounting, channel 1 is inactive (OC1REF=0) as long as TIMx_CNT>TIMx_CCR1 else active (OC1REF=1).
111: PWM mode 2 - In upcounting, channel 1 is inactive as long as TIMx_CNT<TIMx_CCR1 else active. In downcounting, channel 1 is active as long as TIMx_CNT>TIMx_CCR1 else inactive.

*Note: In PWM mode 1 or 2, the OCREF level changes only when the result of the comparison changes or when the output compare mode switches from "frozen" mode to "PWM" mode.*

**Bit 3 OC1PE:** Output compare 1 preload enable
0: Preload register on TIMx_CCR1 disabled. TIMx_CCR1 can be written at anytime, the new value is taken in account immediately.
1: Preload register on TIMx_CCR1 enabled. Read/Write operations access the preload register. TIMx_CCR1 preload value is loaded in the active register at each update event.

*Note: 1: These bits can not be modified as long as LOCK level 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S=00 (the channel is configured in output).*
*2: The PWM mode can be used without validating the preload register only in one-pulse mode (OPM bit set in TIMx_CR1 register). Else the behavior is not guaranteed.*

**Bit 2 OC1FE:** Output compare 1 fast enable
This bit is used to accelerate the effect of an event on the trigger in input on the CC output.
0: CC1 behaves normally depending on counter and CCR1 values even when the trigger is ON. The minimum delay to activate CC1 output when an edge occurs on the trigger input is 5 clock cycles.
1: An active edge on the trigger input acts like a compare match on CC1 output. Then, OC is set to the compare level independently from the result of the comparison. Delay to sample the trigger input and to activate CC1 output is reduced to 3 clock cycles. OCFE acts only if the channel is configured in PWM1 or PWM2 mode.

**Bits 1:0 CC1S:** Capture/Compare 1 selection
This bit-field defines the direction of the channel (input/output) as well as the used input.
00: CC1 channel is configured as output.
01: CC1 channel is configured as input, IC1 is mapped on TI1.
10: CC1 channel is configured as input, IC1 is mapped on TI2.
11: CC1 channel is configured as input, IC1 is mapped on TRC. This mode is working only if an internal trigger input is selected through TS bit (TIMx_SMCR register)

*Note: CC1S bits are writable only when the channel is OFF (CC1E = 0 in TIMx_CCER).*

ST
RM0090 Rev 18
637/1749

--- Page 53 ---

General-purpose timers (TIM2 to TIM5) RM0090

**Input capture mode**

Bits 15:12 **IC2F**: Input capture 2 filter
Bits 11:10 **IC2PSC[1:0]**: Input capture 2 prescaler
Bits 9:8 **CC2S**: Capture/compare 2 selection
This bit-field defines the direction of the channel (input/output) as well as the used input.
00: CC2 channel is configured as output.
01: CC2 channel is configured as input, IC2 is mapped on TI2.
10: CC2 channel is configured as input, IC2 is mapped on TI1.
11: CC2 channel is configured as input, IC2 is mapped on TRC. This mode is working only if an internal trigger input is selected through TS bit (TIMx_SMCR register)
*Note: CC2S bits are writable only when the channel is OFF (CC2E = 0 in TIMx_CCER).*

Bits 7:4 **IC1F**: Input capture 1 filter
This bit-field defines the frequency used to sample TI1 input and the length of the digital filter applied to TI1. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output:
0000: No filter, sampling is done at f_DTS
0001: f_SAMPLING=f_CK_INT, N=2
0010: f_SAMPLING=f_CK_INT, N=4
0011: f_SAMPLING=f_CK_INT, N=8
0100: f_SAMPLING=f_DTS/2, N=6
0101: f_SAMPLING=f_DTS/2, N=8
0110: f_SAMPLING=f_DTS/4, N=6
0111: f_SAMPLING=f_DTS/4, N=8
1000: f_SAMPLING=f_DTS/8, N=6
1001: f_SAMPLING=f_DTS/8, N=8
1010: f_SAMPLING=f_DTS/16, N=5
1011: f_SAMPLING=f_DTS/16, N=6
1100: f_SAMPLING=f_DTS/16, N=8
1101: f_SAMPLING=f_DTS/32, N=5
1110: f_SAMPLING=f_DTS/32, N=6
1111: f_SAMPLING=f_DTS/32, N=8

Bits 3:2 **IC1PSC**: Input capture 1 prescaler
This bit-field defines the ratio of the prescaler acting on CC1 input (IC1).
The prescaler is reset as soon as CC1E=0 (TIMx_CCER register).
00: no prescaler, capture is done each time an edge is detected on the capture input
01: capture is done once every 2 events
10: capture is done once every 4 events
11: capture is done once every 8 events

Bits 1:0 **CC1S**: Capture/Compare 1 selection
This bit-field defines the direction of the channel (input/output) as well as the used input.
00: CC1 channel is configured as output
01: CC1 channel is configured as input, IC1 is mapped on TI1
10: CC1 channel is configured as input, IC1 is mapped on TI2
11: CC1 channel is configured as input, IC1 is mapped on TRC. This mode is working only if an internal trigger input is selected through TS bit (TIMx_SMCR register)
*Note: CC1S bits are writable only when the channel is OFF (CC1E = 0 in TIMx_CCER).*

638/1749 RM0090 Rev 18

--- Page 54 ---

RM0090
General-purpose timers (TIM2 to TIM5)


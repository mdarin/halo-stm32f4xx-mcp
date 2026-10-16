**17.4.5 TIM1 and TIM8 status register (TIMx_SR)**

Address offset: 0x10

Reset value: 0x0000

15 14 13 12 11 10 9 8

7 6 5 4 3 2 1 0

| Reserved | CC4OF | CC3OF | CC2OF | CC1OF | Res. | BIF | TIF | COMIF | CC4IF | CC3IF | CC2IF | CC1IF | UIF |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| rc w0 _ | Res. | rc w0 _ |  |  |  |  |  |  |  |  |  |  |  |


Bits 15:13 Reserved, must be kept at reset value.

Bit 12 CC4OF : Capture/Compare 4 overcapture flag refer to CC1OF description

Bit 11 CC3OF : Capture/Compare 3 overcapture flag refer to CC1OF description

Bit 10 CC2OF : Capture/Compare 2 overcapture flag refer to CC1OF description

Bit 9 CC1OF : Capture/Compare 1 overcapture flag

This flag is set by hardware only when the corresponding channel is configured in input

capture mode. It is cleared by software by writing it to ‘0’. 0: No overcapture has been detected.

1: The counter value has been captured in TIMx_CCR1 register while CC1IF flag was

already set

Bit 8 Reserved, must be kept at reset value.

Bit 7 BIF : Break interrupt flag

This flag is set by hardware as soon as the break input goes active. It can be cleared by

software if the break input is not active.

0: No break event occurred.

1: An active level has been detected on the break input. Bit 6 TIF : Trigger interrupt flag

This flag is set by hardware on trigger event (active edge detected on TRGI input when the slave mode controller is enabled in all modes but gated mode. It is set when the counter starts or stops when gated mode is selected. It is cleared by software.

0: No trigger event occurred.

1: Trigger interrupt pending.

Bit 5 COMIF : COM interrupt flag

This flag is set by hardware on COM event (when Capture/compare Control bits - CCxE,

CCxNE, OCxM - have been updated). It is cleared by software. 0: No COM event occurred.

1: COM interrupt pending.

Bit 4 CC4IF : Capture/Compare 4 interrupt flag

refer to CC1IF description

Bit 3 CC3IF : Capture/Compare 3 interrupt flag

refer to CC1IF description



588

--- Page 180 ---

**Advanced-control timers (TIM1 and TIM8) RM0090**

Bit 2 CC2IF : Capture/Compare 2 interrupt flag

refer to CC1IF description

Bit 1 CC1IF : Capture/Compare 1 interrupt flag

**If channel CC1 is configured as output:**

This flag is set by hardware when the counter matches the compare value, with some exception in center-aligned mode (refer to the CMS bits in the TIMx_CR1 register description). It is cleared by software.

0: No match.

1: The content of the counter TIMx_CNT matches the content of the TIMx_CCR1 register. When the contents of TIMx_CCR1 are greater than the contents of TIMx_ARR, the CC1IF bit goes high on the counter overflow (in upcounting and up/down-counting modes) or underflow (in downcounting mode)

**If channel CC1 is configured as input:**

This bit is set by hardware on a capture. It is cleared by software or by reading the TIMx_CCR1 register.

0: No input capture occurred

1: The counter value has been captured in TIMx_CCR1 register (An edge has been detected on IC1 which matches the selected polarity)

Bit 0 UIF : Update interrupt flag

This bit is set by hardware on an update event. It is cleared by software.

0: No update occurred.

1: Update interrupt pending. This bit is set by hardware when the registers are updated:

– At overflow or underflow regarding the repetition counter value (update if repetition counter = 0) and if the UDIS=0 in the TIMx_CR1 register.

– When CNT is reinitialized by software using the UG bit in TIMx_EGR register, if URS=0 and UDIS=0 in the TIMx_CR1 register.

– When CNT is reinitialized by a trigger event (refer to Section 17.4.3: TIM1 and TIM8 slave mode control register (TIMx_SMCR) ), if URS=0 and UDIS=0 in the TIMx_CR1 register.


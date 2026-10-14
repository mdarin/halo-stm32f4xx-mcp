> [RM0090 ch.18 §18.4] TIM2 to TIM5 registers


### 18.4.5 TIMx status register (TIMx_SR)

Address offset: 0x10
Reset value: 0x0000

| 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved | CC4OFrc_w0 | CC3OFrc_w0 | CC2OFrc_w0 | CC1OFrc_w0 | Reserved | TIFrc_w0 | Res | CC4IFrc_w0 | CC3IFrc_w0 | CC2IFrc_w0 | CC1IFrc_w0 | UIFrc_w0 |  |  |  |


Bits 15:13 Reserved, must be kept at reset value.

Bit 12 **CC4OF**: Capture/Compare 4 overcapture flag
refer to CC1OF description

Bit 11 **CC3OF**: Capture/Compare 3 overcapture flag
refer to CC1OF description

Bit 10 **CC2OF**: Capture/compare 2 overcapture flag
refer to CC1OF description

Bit 9 **CC1OF**: Capture/Compare 1 overcapture flag
This flag is set by hardware only when the corresponding channel is configured in input capture mode. It is cleared by software by writing it to '0'.
0: No overcapture has been detected
1: The counter value has been captured in TIMx_CCR1 register while CC1IF flag was already set

Bits 8:7 Reserved, must be kept at reset value.

Bit 6 **TIF**: Trigger interrupt flag
This flag is set by hardware on trigger event (active edge detected on TRGI input when the slave mode controller is enabled in all modes but gated mode. It is set when the counter starts or stops when gated mode is selected. It is cleared by software.
0: No trigger event occurred
1: Trigger interrupt pending

Bit 5 Reserved, must be kept at reset value.

Bit 4 **CC4IF**: Capture/Compare 4 interrupt flag
refer to CC1IF description

STI
RM0090 Rev 18
633/1749

--- Page 49 ---

**General-purpose timers (TIM2 to TIM5)**
RM0090

**Bit 3 CC3IF**: Capture/Compare 3 interrupt flag
refer to CC1IF description

**Bit 2 CC2IF**: Capture/Compare 2 interrupt flag
refer to CC1IF description

**Bit 1 CC1IF**: Capture/compare 1 interrupt flag
**If channel CC1 is configured as output:**
This flag is set by hardware when the counter matches the compare value, with some exception in center-aligned mode (refer to the CMS bits in the TIMx_CR1 register description). It is cleared by software.
0: No match
1: The content of the counter TIMx_CNT matches the content of the TIMx_CCR1 register. When the contents of TIMx_CCR1 are greater than the contents of TIMx_ARR, the CC1IF bit goes high on the counter overflow (in upcounting and up/down-counting modes) or underflow (in downcounting mode)
**If channel CC1 is configured as input:**
This bit is set by hardware on a capture. It is cleared by software or by reading the TIMx_CCR1 register.
0: No input capture occurred
1: The counter value has been captured in TIMx_CCR1 register (An edge has been detected on IC1 which matches the selected polarity)

**Bit 0 UIF**: Update interrupt flag
" This bit is set by hardware on an update event. It is cleared by software.
0: No update occurred.
1: Update interrupt pending. This bit is set by hardware when the registers are updated:
" At overflow or underflow (for TIM2 to TIM5) and if UDIS=0 in the TIMx_CR1 register.
" When CNT is reinitialized by software using the UG bit in TIMx_EGR register, if URS=0 and UDIS=0 in the TIMx_CR1 register.
When CNT is reinitialized by a trigger event (refer to the synchro control register description), if URS=0 and UDIS=0 in the TIMx_CR1 register.

634/1749
RM0090 Rev 18

--- Page 50 ---

RM0090
General-purpose timers (TIM2 to TIM5)


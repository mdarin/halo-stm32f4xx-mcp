**17.4.6 TIM1 and TIM8 event generation register (TIMx_EGR)**

Address offset: 0x14

Reset value: 0x0000

15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0

| Reserved | BG | TG | COMG | CC4G | CC3G | CC2G | CC1G | UG |
|---|---|---|---|---|---|---|---|---|
| w |  |  |  |  |  |  |  |  |


Bits 15:8 Reserved, must be kept at reset value.

Bit 7 BG : Break generation

This bit is set by software in order to generate an event, it is automatically cleared by hardware.

0: No action

1: A break event is generated. MOE bit is cleared and BIF flag is set. Related interrupt or DMA transfer can occur if enabled.

Bit 6 TG : Trigger generation

This bit is set by software in order to generate an event, it is automatically cleared by hardware.

0: No action

1: The TIF flag is set in TIMx_SR register. Related interrupt or DMA transfer can occur if enabled.

570/1749 RM0090 Rev 18

--- Page 181 ---

**RM0090 Advanced-control timers (TIM1 and TIM8)**

Bit 5 COMG : Capture/Compare control update generation

This bit can be set by software, it is automatically cleared by hardware

0: No action

1: When CCPC bit is set, it allows to update CCxE, CCxNE and OCxM bits

Note: This bit acts only on channels having a complementary output.

Bit 4 CC4G : Capture/Compare 4 generation

refer to CC1G description

Bit 3 CC3G : Capture/Compare 3 generation

refer to CC1G description

Bit 2 CC2G : Capture/Compare 2 generation

refer to CC1G description

Bit 1 CC1G : Capture/Compare 1 generation

This bit is set by software in order to generate an event, it is automatically cleared by hardware.

0: No action

1: A capture/compare event is generated on channel 1:

**If channel CC1 is configured as output:**

CC1IF flag is set, Corresponding interrupt or DMA request is sent if enabled.

**If channel CC1 is configured as input:**

The current value of the counter is captured in TIMx_CCR1 register. The CC1IF flag is set, the corresponding interrupt or DMA request is sent if enabled. The CC1OF flag is set if the CC1IF flag was already high.

Bit 0 UG : Update generation

This bit can be set by software, it is automatically cleared by hardware.

0: No action

1: Reinitialize the counter and generates an update of the registers. Note that the prescaler counter is cleared too (anyway the prescaler ratio is not affected). The counter is cleared if the center-aligned mode is selected or if DIR=0 (upcounting), else it takes the auto-reload value (TIMx_ARR) if DIR=1 (downcounting).

RM0090 Rev 18 571/1749

588

--- Page 182 ---

**Advanced-control timers (TIM1 and TIM8) RM0090**


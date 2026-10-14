> [RM0090 ch.18 §18.4] TIM2 to TIM5 registers


### 18.4.6 TIMx event generation register (TIMx_EGR)

Address offset: 0x14
Reset value: 0x0000

| 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved | TGw | Res. | CC4Gw | CC3Gw | CC2Gw | CC1Gw | UGw |  |  |  |  |  |  |  |  |


Bits 15:7 Reserved, must be kept at reset value.

**Bit 6 TG: Trigger generation**
This bit is set by software in order to generate an event, it is automatically cleared by hardware.
0: No action
1: The TIF flag is set in TIMx_SR register. Related interrupt or DMA transfer can occur if enabled.

**Bit 5** Reserved, must be kept at reset value.

**Bit 4 CC4G: Capture/compare 4 generation**
refer to CC1G description

**Bit 3 CC3G: Capture/compare 3 generation**
refer to CC1G description

**Bit 2 CC2G: Capture/compare 2 generation**
refer to CC1G description

**Bit 1 CC1G: Capture/compare 1 generation**
This bit is set by software in order to generate an event, it is automatically cleared by hardware.
0: No action
1: A capture/compare event is generated on channel 1:
**If channel CC1 is configured as output:**
CC1IF flag is set, Corresponding interrupt or DMA request is sent if enabled.
**If channel CC1 is configured as input:**
The current value of the counter is captured in TIMx_CCR1 register. The CC1IF flag is set, the corresponding interrupt or DMA request is sent if enabled. The CC1OF flag is set if the CC1IF flag was already high.

**Bit 0 UG: Update generation**
This bit can be set by software, it is automatically cleared by hardware.
0: No action
1: Re-initialize the counter and generates an update of the registers. Note that the prescaler counter is cleared too (anyway the prescaler ratio is not affected). The counter is cleared if the center-aligned mode is selected or if DIR=0 (upcounting), else it takes the auto-reload value (TIMx_ARR) if DIR=1 (downcounting).

STI
RM0090 Rev 18
635/1749

--- Page 51 ---

**General-purpose timers (TIM2 to TIM5)**
RM0090


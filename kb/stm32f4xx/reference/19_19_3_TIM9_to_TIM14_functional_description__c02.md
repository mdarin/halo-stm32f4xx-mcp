> [RM0090 ch.19 §19.3] TIM9 to TIM14 functional description


### 19.3.2 Counter modes

**Upcounting mode**

In upcounting mode, the counter counts from 0 to the auto-reload value (content of the TIMx_ARR register), then restarts from 0 and generates a counter overflow event.

Setting the UG bit in the TIMx_EGR register (by software or by using the slave mode controller on TIM9 and TIM12) also generates an update event.

The UEV event can be disabled by software by setting the UDIS bit in the TIMx_CR1 register. This is to avoid updating the shadow registers while writing new values in the preload registers. Then no update event occurs until the UDIS bit has been written to 0. However, the counter restarts from 0, as well as the counter of the prescaler (but the prescale rate does not change). In addition, if the URS bit (update request selection) in TIMx_CR1 register is set, setting the UG bit generates an update event UEV but without setting the UIF flag (thus no interrupt is sent). This is to avoid generating both update and capture interrupts when clearing the counter on the capture event.

When an update event occurs, all the registers are updated and the update flag (UIF bit in TIMx_SR register) is set (depending on the URS bit):

*   The auto-reload shadow register is updated with the preload value (TIMx_ARR),
*   The buffer of the prescaler is reloaded with the preload value (content of the TIMx_PSC register).

The following figures show some examples of the counter behavior for different clock frequencies when TIMx_ARR=0x36.

**Figure 184. Counter timing diagram, internal clock divided by 1**

CK_PSC
CNT_EN
Timerclock = CK_CNT
Counter register 31 32 33 34 35 36 00 01 02 03 04 05 06 07
Counter overflow
Update event (UEV)
Update interrupt flag (UIF)

MS31078V3

ST
RM0090 Rev 18
655/1749

--- Page 71 ---

**General-purpose timers (TIM9 to TIM14)**
RM0090

**Figure 185. Counter timing diagram, internal clock divided by 2**

CK_PSC
CNT_EN
Timerclock = CK_CNT
Counter register 0034 0035 0036 0000 0001 0002 0003
Counter overflow
Update event (UEV)
Update interrupt flag (UIF)
MS31079V3

**Figure 186. Counter timing diagram, internal clock divided by 4**

CK_PSC
CNT_EN
Timerclock = CK_CNT
Counter register 0035 0036 0000 0001
Counter overflow
Update event (UEV)
Update interrupt flag (UIF)
MS31080V3

**Figure 187. Counter timing diagram, internal clock divided by N**

CK_PSC
Timerclock = CK_CNT
Counter register 1F 20 00
Counter overflow
Update event (UEV)
Update interrupt flag (UIF)
MS31081V3

656/1749
RM0090 Rev 18

--- Page 72 ---

RM0090
General-purpose timers (TIM9 to TIM14)

**Figure 188. Counter timing diagram, update event when ARPE=0 (TIMx_ARR not preloaded)**

CK_PSC
CEN
Timerclock = CK_CNT
Counter register 31 32 33 34 35 36 00 01 02 03 04 05 06 07
Counter overflow
Update event (UEV)
Update interrupt flag (UIF)
Auto-reload preload register FF 36
Write a new value in TIMx_ARR
MS31082V3

**Figure 189. Counter timing diagram, update event when ARPE=1 (TIMx_ARR preloaded)**

CK_PSC
CEN
Timerclock = CK_CNT
Counter register F0 F1 F2 F3 F4 F5 00 01 02 03 04 05 06 07
Counter overflow
Update event (UEV)
Update interrupt flag (UIF)
Auto-reload preload register F5 36
Auto-reload shadow register F5 36
Write a new value in TIMx_ARR
MS31083V2

ST
RM0090 Rev 18
657/1749

--- Page 73 ---

**General-purpose timers (TIM9 to TIM14)**
RM0090


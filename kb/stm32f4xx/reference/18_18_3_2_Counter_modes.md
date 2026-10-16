**18.3.2 Counter modes**

**Upcounting mode**

In upcounting mode, the counter counts from 0 to the auto-reload value (content of the TIMx_ARR register), then restarts from 0 and generates a counter overflow event.

An Update event can be generated at each counter overflow or by setting the UG bit in the TIMx_EGR register (by software or by using the slave mode controller).

The UEV event can be disabled by software by setting the UDIS bit in TIMx_CR1 register. This is to avoid updating the shadow registers while writing new values in the preload registers. Then no update event occurs until the UDIS bit has been written to 0. However, the counter restarts from 0, as well as the counter of the prescaler (but the prescale rate

592/1749
RM0090 Rev 18
ST

--- Page 8 ---

**RM0090**
**General-purpose timers (TIM2 to TIM5)**

does not change). In addition, if the URS bit (update request selection) in TIMx_CR1 register is set, setting the UG bit generates an update event UEV but without setting the UIF flag (thus no interrupt or DMA request is sent). This is to avoid generating both update and capture interrupts when clearing the counter on the capture event.

When an update event occurs, all the registers are updated and the update flag (UIF bit in TIMx_SR register) is set (depending on the URS bit):

*   The buffer of the prescaler is reloaded with the preload value (content of the TIMx_PSC register)
*   The auto-reload shadow register is updated with the preload value (TIMx_ARR)

The following figures show some examples of the counter behavior for different clock frequencies when TIMx_ARR=0x36.

**Figure 137. Counter timing diagram, internal clock divided by 1**

CK_INT
CNT_EN
Timerclock = CK_CNT
Counter register 31 32 33 34 35 36 00 01 02 03 04 05 06 07
Counter overflow
Update event (UEV)
Update interrupt flag (UIF)
MS35836V1

**Figure 138. Counter timing diagram, internal clock divided by 2**

CK_INT
CNT_EN
Timerclock = CK_CNT
Counter register 0034 0035 0036 0000 0001 0002 0003
Counter overflow
Update event (UEV)
Update interrupt flag (UIF)
MS35835V1

ST
RM0090 Rev 18
593/1749

--- Page 9 ---

General-purpose timers (TIM2 to TIM5)
RM0090

Figure 139. Counter timing diagram, internal clock divided by 4

CK_INT
CNT_EN
Timerclock = CK_CNT
Counter register
0035
0036
0000
0001
Counter overflow
Update event (UEV)
Update interrupt flag (UIF)
MSV37301V1

Figure 140. Counter timing diagram, internal clock divided by N

CK_INT
Timerclock = CK_CNT
Counter register
1F
20
00
Counter overflow
Update event (UEV)
Update interrupt flag (UIF)
MSV37302V1

594/1749
RM0090 Rev 18

--- Page 10 ---

RM0090
General-purpose timers (TIM2 to TIM5)

**Figure 141. Counter timing diagram, Update event when ARPE=0 (TIMx_ARR not preloaded)**

CK_INT
CNT_EN
Timerclock = CK_CNT
Counter register 31 32 33 34 35 36 00 01 02 03 04 05 06 07
Counter overflow
Update event (UEV)
Update interrupt flag (UIF)
Auto-reload register FF 36
Write a new value in TIMx_ARR
MSv37303V1

**Figure 142. Counter timing diagram, Update event when ARPE=1 (TIMx_ARR preloaded)**

CK_PSC
CNT_EN
Timerclock = CK_CNT
Counter register F0 F1 F2 F3 F4 F5 00 01 02 03 04 05 06 07
Counter overflow
Update event (UEV)
Update interrupt flag (UIF)
Auto-reload preload register F5 36
Auto-reload shadow register F5 36
Write a new value in TIMx_ARR
MSv37304V1

**Downcounting mode**

In downcounting mode, the counter counts from the auto-reload value (content of the TIMx_ARR register) down to 0, then restarts from the auto-reload value and generates a counter underflow event.

An Update event can be generate at each counter underflow or by setting the UG bit in the TIMx_EGR register (by software or by using the slave mode controller)

The UEV update event can be disabled by software by setting the UDIS bit in TIMx_CR1 register. This is to avoid updating the shadow registers while writing new values in the

ST
RM0090 Rev 18
595/1749

--- Page 11 ---

**General-purpose timers (TIM2 to TIM5)**
RM0090

preload registers. Then no update event occurs until UDIS bit has been written to 0.
However, the counter restarts from the current auto-reload value, whereas the counter of the prescaler restarts from 0 (but the prescale rate doesn't change).

In addition, if the URS bit (update request selection) in TIMx_CR1 register is set, setting the UG bit generates an update event UEV but without setting the UIF flag (thus no interrupt or DMA request is sent). This is to avoid generating both update and capture interrupts when clearing the counter on the capture event.

When an update event occurs, all the registers are updated and the update flag (UIF bit in TIMx_SR register) is set (depending on the URS bit):

*   The buffer of the prescaler is reloaded with the preload value (content of the TIMx_PSC register).
*   The auto-reload active register is updated with the preload value (content of the TIMx_ARR register). Note that the auto-reload is updated before the counter is reloaded, so that the next period is the expected one.

The following figures show some examples of the counter behavior for different clock frequencies when TIMx_ARR=0x36.

**Figure 143. Counter timing diagram, internal clock divided by 1**

CK_INT
CNT_EN
Timerclock = CK_CNT
Counter register 05 04 03 02 01 00 36 35 34 33 32 31 30 2F
Counter underflow (cnt_udf)
Update event (UEV)
Update interrupt flag (UIF)

MSv37305V1

596/1749
RM0090 Rev 18

--- Page 12 ---

RM0090
General-purpose timers (TIM2 to TIM5)

**Figure 144. Counter timing diagram, internal clock divided by 2**

CK_INT
CNT_EN
Timerclock = CK_CNT
Counter register 0002 0001 0000 0036 0035 0034 0033
Counter underflow
Update event (UEV)
Update interrupt flag (UIF)
MSv37306V1

**Figure 145. Counter timing diagram, internal clock divided by 4**

CK_INT
CNT_EN
Timerclock = CK_CNT
Counter register 0001 0000 0036 0035
Counter underflow
Update event (UEV)
Update interrupt flag (UIF)
MS40511V1

**Figure 146. Counter timing diagram, internal clock divided by N**

CK_INT
Timerclock = CK_CNT
Counter register 20 1F 00 36
Counter overflow
Update event (UEV)
Update interrupt flag (UIF)
MS37340V1

ST
RM0090 Rev 18
597/1749

--- Page 13 ---

General-purpose timers (TIM2 to TIM5)
RM0090

Figure 147. Counter timing diagram, Update event

CK_INT
CNT_EN
Timerclock = CK_CNT
Counter register 05 04 03 02 01 00 36 35 34 33 32 31 30 2F
Counter overflow
Update event (UEV)
Update interrupt flag (UIF)
Auto-reload preload register FF 36
Write a new value in TIMx_ARR
MS37341V1

**Center-aligned mode (up/down counting)**

In center-aligned mode, the counter counts from 0 to the auto-reload value (content of the TIMx_ARR register) – 1, generates a counter overflow, then counts from the auto-reload value down to 1 and generates a counter underflow event. Then it restarts counting from 0.

Center-aligned mode is active when the CMS bits in TIMx_CR1 register are not equal to '00'. The Output compare interrupt flag of channels configured in output is set when: the counter counts down (Center aligned mode 1, CMS = "01"), the counter counts up (Center aligned mode 2, CMS = "10") the counter counts up and down (Center aligned mode 3, CMS = "11").

In this mode, the direction bit (DIR from TIMx_CR1 register) cannot be written. It is updated by hardware and gives the current direction of the counter.

The update event can be generated at each counter overflow and at each counter underflow or by setting the UG bit in the TIMx_EGR register (by software or by using the slave mode controller) also generates an update event. In this case, the counter restarts counting from 0, as well as the counter of the prescaler.

The UEV update event can be disabled by software by setting the UDIS bit in TIMx_CR1 register. This is to avoid updating the shadow registers while writing new values in the preload registers. Then no update event occurs until the UDIS bit has been written to 0. However, the counter continues counting up and down, based on the current auto-reload value.

In addition, if the URS bit (update request selection) in TIMx_CR1 register is set, setting the UG bit generates an update event UEV but without setting the UIF flag (thus no interrupt or DMA request is sent). This is to avoid generating both update and capture interrupt when clearing the counter on the capture event.

598/1749
RM0090 Rev 18
ST

--- Page 14 ---

RM0090
General-purpose timers (TIM2 to TIM5)

When an update event occurs, all the registers are updated and the update flag (UIF bit in TIMx_SR register) is set (depending on the URS bit):

*   The buffer of the prescaler is reloaded with the preload value (content of the TIMx_PSC register).
*   The auto-reload active register is updated with the preload value (content of the TIMx_ARR register). Note that if the update source is a counter overflow, the auto-reload is updated before the counter is reloaded, so that the next period is the expected one (the counter is loaded with the new value).

The following figures show some examples of the counter behavior for different clock frequencies.

**Figure 148. Counter timing diagram, internal clock divided by 1, TIMx_ARR=0x6**

CK_INT
CNT_EN
Timerclock = CK_CNT
Counter register 04 03 02 01 00 01 02 03 04 05 06 05 04 03
Counter underflow
Counter overflow
Update event (UEV)
Update interrupt flag (UIF)

MS37342V1

1. Here, center-aligned mode 1 is used, for more details refer to Section 18.4.1: TIMx control register 1 (TIMx_CR1).

**Figure 149. Counter timing diagram, internal clock divided by 2**

CK_INT
CNT_EN
Timerclock = CK_CNT
Counter register 0003 0002 0001 0000 0001 0002 0003
Counter underflow
Update event (UEV)
Update interrupt flag (UIF)

MS37343V1

ST
RM0090 Rev 18
599/1749

--- Page 15 ---

**General-purpose timers (TIM2 to TIM5)**
RM0090

**Figure 150. Counter timing diagram, internal clock divided by 4, TIMx_ARR=0x36**

CK_INT
CNT_EN
Timerclock = CK_CNT
Counter register 0034 0035 0036 0035
Counter overflow (cnt_ovf)
Update event (UEV)
Update interrupt flag (UIF)

MS37344V1

1. Center-aligned mode 2 or 3 is used with an UIF on overflow.

**Figure 151. Counter timing diagram, internal clock divided by N**

CK_INT
Timerclock = CK_CNT
Counter register 20 1F 01 00
Counter underflow
Update event (UEV)
Update interrupt flag (UIF)

MS37345V1

600/1749
RM0090 Rev 18
ST

--- Page 16 ---

RM0090
General-purpose timers (TIM2 to TIM5)

**Figure 152. Counter timing diagram, Update event with ARPE=1 (counter underflow)**

CK_INT
CNT_EN
Timerclock = CK_CNT
Counter register 06 05 04 03 02 01 00 01 02 03 04 05 06 07
Counter underflow
Update event (UEV)
Update interrupt flag (UIF)
Auto-reload preload register FD 36
Write a new value in TIMx_ARR
Auto-reload active register FD 36

MS37360V1

**Figure 153. Counter timing diagram, Update event with ARPE=1 (counter overflow)**

CK_INT
CNT_EN
Timer clock = CK_CNT
Counter register F7 F8 F9 FA FB FC 36 35 34 33 32 31 30 2F
Counter overflow
Update event (UEV)
Update interrupt flag (UIF)
Auto-reload preload register FD 36
Write a new value in TIMx_ARR
Auto-reload active register FD 36

MS37361V1

ST
RM0090 Rev 18
601/1749

--- Page 17 ---

General-purpose timers (TIM2 to TIM5)
RM0090


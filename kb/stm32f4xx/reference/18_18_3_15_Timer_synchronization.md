### 18.3.15 Timer synchronization

The TIMx timers are linked together internally for timer synchronization or chaining. When one Timer is configured in Master Mode, it can reset, start, stop or clock the counter of another Timer configured in Slave Mode.

*Figure 174* presents an overview of the trigger selection and the master mode selection blocks.

**Note:** The clock of the slave timer must be enabled prior to receiving events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer.

**Using one timer as prescaler for another timer**

**Figure 174. Master/Slave timer example**

MS37387V1

STi
RM0090 Rev 18
621/1749

--- Page 37 ---

**General-purpose timers (TIM2 to TIM5)** RM0090

For example, the user can configure Timer 1 to act as a prescaler for Timer 2 (see Figure 174). To do this:

*   Configure Timer 1 in master mode so that it outputs a periodic trigger signal on each update event UEV. If you write MMS=010 in the TIM1_CR2 register, a rising edge is output on TRGO1 each time an update event is generated.
*   To connect the TRGO1 output of Timer 1 to Timer 2, Timer 2 must be configured in slave mode using ITR0 as internal trigger. You select this through the TS bits in the TIM2_SMCR register (writing TS=000).
*   Then you put the slave mode controller in external clock mode 1 (write SMS=111 in the TIM2_SMCR register). This causes Timer 2 to be clocked by the rising edge of the periodic Timer 1 trigger signal (which correspond to the timer 1 counter overflow).
*   Finally both timers must be enabled by setting their respective CEN bits (TIMx_CR1 register).

**Note:** If OCx is selected on Timer 1 as trigger output (MMS=1xx), its rising edge is used to clock the counter of timer 2.

### Using one timer to enable another timer

In this example, we control the enable of Timer 2 with the output compare 1 of Timer 1. Refer to Figure 174 for connections. Timer 2 counts on the divided internal clock only when OC1REF of Timer 1 is high. Both counter clock frequencies are divided by 3 by the prescaler compared to CK_INT ($f_{CK\_CNT} = f_{CK\_INT}/3$).

*   Configure Timer 1 master mode to send its Output Compare 1 Reference (OC1REF) signal as trigger output (MMS=100 in the TIM1_CR2 register).
*   Configure the Timer 1 OC1REF waveform (TIM1_CCMR1 register).
*   Configure Timer 2 to get the input trigger from Timer 1 (TS=000 in the TIM2_SMCR register).
*   Configure Timer 2 in gated mode (SMS=101 in TIM2_SMCR register).
*   Enable Timer 2 by writing '1 in the CEN bit (TIM2_CR1 register).
*   Start Timer 1 by writing '1 in the CEN bit (TIM1_CR1 register).

**Note:** The counter 2 clock is not synchronized with counter 1, this mode only affects the Timer 2 counter enable signal.

**Figure 175. Gating timer 2 with OC1REF of timer 1**

CK_INT
TIMER1-OC1REF
TIMER1-CNT FC FD FE FF 00 01
TIMER2-CNT 3045 3046 3047 3048
TIMER2-TIF
Write TIF = 0
MS37388V1

In the example in Figure 175, the Timer 2 counter and prescaler are not initialized before being started. So they start counting from their current value. It is possible to start from a given value by resetting both timers before starting Timer 1. You can then write any value

622/1749 RM0090 Rev 18 STI

--- Page 38 ---

RM0090
General-purpose timers (TIM2 to TIM5)

you want in the timer counters. The timers can easily be reset by software using the UG bit in the TIMx_EGR registers.

In the next example, we synchronize Timer 1 and Timer 2. Timer 1 is the master and starts from 0. Timer 2 is the slave and starts from 0xE7. The prescaler ratio is the same for both timers. Timer 2 stops when Timer 1 is disabled by writing '0 to the CEN bit in the TIM1_CR1 register:

*   Configure Timer 1 master mode to send its Output Compare 1 Reference (OC1REF) signal as trigger output (MMS=100 in the TIM1_CR2 register).
*   Configure the Timer 1 OC1REF waveform (TIM1_CCMR1 register).
*   Configure Timer 2 to get the input trigger from Timer 1 (TS=000 in the TIM2_SMCR register).
*   Configure Timer 2 in gated mode (SMS=101 in TIM2_SMCR register).
*   Reset Timer 1 by writing '1 in UG bit (TIM1_EGR register).
*   Reset Timer 2 by writing '1 in UG bit (TIM2_EGR register).
*   Initialize Timer 2 to 0xE7 by writing '0xE7' in the timer 2 counter (TIM2_CNTL).
*   Enable Timer 2 by writing '1 in the CEN bit (TIM2_CR1 register).
*   Start Timer 1 by writing '1 in the CEN bit (TIM1_CR1 register).
*   Stop Timer 1 by writing '0 in the CEN bit (TIM1_CR1 register).

Figure 176. Gating timer 2 with Enable of timer 1

CK_INT
TIMER1-CEN=CNT_EN
TIMER1-CNT_INIT
TIMER1-CNT 75 00 01 02
TIMER2-CNT AB 00 E7 E8 E9
TIMER2-CNT_INIT
TIMER2-write CNT
TIMER2-TIF
Write TIF = 0
MS37389V1

ST
RM0090 Rev 18
623/1749

--- Page 39 ---

**General-purpose timers (TIM2 to TIM5)**
RM0090

**Using one timer to start another timer**

In this example, we set the enable of Timer 2 with the update event of Timer 1. Refer to *Figure 174* for connections. Timer 2 starts counting from its current value (which can be nonzero) on the divided internal clock as soon as the update event is generated by Timer 1. When Timer 2 receives the trigger signal its CEN bit is automatically set and the counter counts until we write '0 to the CEN bit in the TIM2_CR1 register. Both counter clock frequencies are divided by 3 by the prescaler compared to CK_INT (fCK_CNT = fCK_INT/3).

*   Configure Timer 1 master mode to send its Update Event (UEV) as trigger output (MMS=010 in the TIM1_CR2 register).
*   Configure the Timer 1 period (TIM1_ARR registers).
*   Configure Timer 2 to get the input trigger from Timer 1 (TS=000 in the TIM2_SMCR register).
*   Configure Timer 2 in trigger mode (SMS=110 in TIM2_SMCR register).
*   Start Timer 1 by writing '1 in the CEN bit (TIM1_CR1 register).

**Figure 177. Triggering timer 2 with update of timer 1**

CK_INT
TIMER1-UEV
TIMER1-CNT FD FE FF 00 01 02
TIMER2-CNT 45 46 47 48
TIMER2-CEN=CNT_EN
TIMER2-TIF
Write TIF = 0
MS37390V1

As in the previous example, the user can initialize both counters before starting counting. *Figure 178* shows the behavior with the same configuration as in *Figure 177* but in trigger mode instead of gated mode (SMS=110 in the TIM2_SMCR register).

624/1749
RM0090 Rev 18
ST

--- Page 40 ---

RM0090
General-purpose timers (TIM2 to TIM5)

Figure 178. Triggering timer 2 with Enable of timer 1

CK_INT
TIMER1-CEN=CNT_EN
TIMER1-CNT_INIT
TIMER1-CNT 75 00 01 02
TIMER2-CNT CD 00 E7 E8 E9 EA
TIMER2-CNT_INIT
TIMER2-write CNT
TIMER2-TIF
Write TIF = 0
MS37391V1

**Starting 2 timers synchronously in response to an external trigger**

In this example, we set the enable of timer 1 when its TI1 input rises, and the enable of Timer 2 with the enable of Timer 1. Refer to Figure 174 for connections. To ensure the counters are aligned, Timer 1 must be configured in Master/Slave mode (slave with respect to TI1, master with respect to Timer 2):

*   Configure Timer 1 master mode to send its Enable as trigger output (MMS=001 in the TIM1_CR2 register).
*   Configure Timer 1 slave mode to get the input trigger from TI1 (TS=100 in the TIM1_SMCR register).
*   Configure Timer 1 in trigger mode (SMS=110 in the TIM1_SMCR register).
*   Configure the Timer 1 in Master/Slave mode by writing MSM=1 (TIM1_SMCR register).
*   Configure Timer 2 to get the input trigger from Timer 1 (TS=000 in the TIM2_SMCR register).
*   Configure Timer 2 in trigger mode (SMS=110 in the TIM2_SMCR register).

When a rising edge occurs on TI1 (Timer 1), both counters starts counting synchronously on the internal clock and both TIF flags are set.

**Note:**
In this example both timers are initialized before starting (by setting their respective UG bits). Both counters starts from 0, but you can easily insert an offset between them by writing any of the counter registers (TIMx_CNT). You can see that the master/slave mode insert a delay between CNT_EN and CK_PSC on timer 1.

ST
RM0090 Rev 18
625/1749

--- Page 41 ---

General-purpose timers (TIM2 to TIM5)
RM0090

Figure 179. Triggering timer 1 and 2 with timer 1 TI1 input

CK_INT
TIMER1-TI1
TIMER1-CEN=CNT_EN
TIMER1-CK_PSC
TIMER1-CNT 00 01 02 03 04 05 06 07 08 09
TIMER1-TIF
TIMER2-CEN=CNT_EN
TIMER2-CK_PSC
TIMER2-CNT 00 01 02 03 04 05 06 07 08 09
TIMER2-TIF

MS37392V1

18.3.16 Debug mode

When the microcontroller enters debug mode (Cortex®-M4 with FPU core - halted), the TIMx counter either continues to work normally or stops, depending on DBG_TIMx_STOP configuration bit in DBGMCU module. For more details, refer to Section 38.16.2: Debug support for timers, watchdog, bxCAN and I²C.

626/1749
RM0090 Rev 18
ST

--- Page 42 ---

RM0090
General-purpose timers (TIM2 to TIM5)


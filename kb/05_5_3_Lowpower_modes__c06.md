> [RM0090 ch.5 §5.3] Low-power modes**

### 5.3.7 Programming the RTC alternate functions to wake up the device from the Stop and Standby modes**

**5.3.7 Programming the RTC alternate functions to wake up the device from the Stop and Standby modes**

The MCU can be woken up from a low-power mode by an RTC alternate function. The RTC alternate functions are the RTC alarms (Alarm A and Alarm B), RTC wakeup, RTC tamper event detection and RTC time stamp event detection.

These RTC alternate functions can wake up the system from the Stop and Standby low- power modes.

The system can also wake up from low-power modes without depending on an external interrupt (Auto-wakeup mode), by using the RTC alarm or the RTC wakeup events. The RTC provides a programmable time base for waking up from the Stop or Standby mode at regular intervals.

For this purpose, two of the three alternate RTC clock sources can be selected by programming the RTCSEL[1:0] bits in the Section 7.3.20: RCC Backup domain control register (RCC_BDCR) :

• Low-power 32.768 kHz external crystal oscillator (LSE OSC)

This clock source provides a precise time base with a very low-power consumption (additional consumption of less than 1 µA under typical conditions)

• Low-power internal RC oscillator (LSI RC)

This clock source has the advantage of saving the cost of the 32.768 kHz crystal. This internal RC oscillator is designed to use minimum power.



--- Page 139 ---

**RM0090 Power controller (PWR)**

**RTC alternate functions to wake up the device from the Stop mode**

• To wake up the device from the Stop mode with an RTC alarm event, it is necessary to:

a) Configure the EXTI Line 17 to be sensitive to rising edges (Interrupt or Event

modes)

b) Enable the RTC Alarm Interrupt in the RTC_CR register

c) Configure the RTC to generate the RTC alarm

• To wake up the device from the Stop mode with an RTC tamper or time stamp event, it is necessary to:

a) Configure the EXTI Line 21 to be sensitive to rising edges (Interrupt or Event

modes)

b) Enable the RTC time stamp Interrupt in the RTC_CR register or the RTC tamper

interrupt in the RTC_TAFCR register

c) Configure the RTC to detect the tamper or time stamp event

• To wake up the device from the Stop mode with an RTC wakeup event, it is necessary to:

a) Configure the EXTI Line 22 to be sensitive to rising edges (Interrupt or Event

modes)

b) Enable the RTC wakeup interrupt in the RTC_CR register

c) Configure the RTC to generate the RTC Wakeup event

**RTC alternate functions to wake up the device from the Standby mode**

• To wake up the device from the Standby mode with an RTC alarm event, it is necessary to:

a) Enable the RTC alarm interrupt in the RTC_CR register

b) Configure the RTC to generate the RTC alarm

• To wake up the device from the Standby mode with an RTC tamper or time stamp event, it is necessary to:

a) Enable the RTC time stamp interrupt in the RTC_CR register or the RTC tamper

interrupt in the RTC_TAFCR register

b) Configure the RTC to detect the tamper or time stamp event

• To wake up the device from the Standby mode with an RTC wakeup event, it is necessary to:

a) Enable the RTC wakeup interrupt in the RTC_CR register

b) Configure the RTC to generate the RTC wakeup event

**Safe RTC alternate function wakeup flag clearing sequence**

If the selected RTC alternate function is set before the PWR wakeup flag (WUTF) is cleared, it will not be detected on the next event as detection is made once on the rising edge.

To avoid bouncing on the pins onto which the RTC alternate functions are mapped, and exit correctly from the Stop and Standby modes, it is recommended to follow the sequence below before entering the Standby mode:

• When using RTC alarm to wake up the device from the low-power modes:

a) Disable the RTC alarm interrupt (ALRAIE or ALRBIE bits in the RTC_CR register)

b) Clear the RTC alarm (ALRAF/ALRBF) flag



149

--- Page 140 ---

**Power controller (PWR)**

**RM0090**

c) Clear the PWR Wakeup (WUF) flag

d) Enable the RTC alarm interrupt

e) Re-enter the low-power mode •

When using RTC wakeup to wake up the device from the low-power modes: Disable the RTC Wakeup interrupt (WUTIE bit in the RTC_CR register)

a)

b) Clear the RTC Wakeup (WUTF) flag

c) Clear the PWR Wakeup (WUF) flag

d) Enable the RTC Wakeup interrupt

e) Re-enter the low-power mode •

When using RTC tamper to wake up the device from the low-power modes: Disable the RTC tamper interrupt (TAMPIE bit in the RTC_TAFCR register)

a)

b) Clear the Tamper (TAMP1F/TSF) flag

c) Clear the PWR Wakeup (WUF) flag

d) Enable the RTC tamper interrupt

e) Re-enter the low-power mode •

When using RTC time stamp to wake up the device from the low-power modes: Disable the RTC time stamp interrupt (TSIE bit in RTC_CR)

a)

b) Clear the RTC time stamp (TSF) flag

c) Clear the PWR Wakeup (WUF) flag

d) Enable the RTC TimeStamp interrupt

e) Re-enter the low-power mode



--- Page 141 ---

**RM0090 Power controller (PWR)**


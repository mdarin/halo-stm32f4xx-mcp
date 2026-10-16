# 26 Real-time clock (RTC)

This section applies to the whole STM32F4xx family, unless otherwise specified.

## 26.1 Introduction

The real-time clock (RTC) is an independent BCD timer/counter. The RTC provides a time-of-day clock/calendar, two programmable alarm interrupts, and a periodic programmable wakeup flag with interrupt capability. The RTC also includes an automatic wakeup unit to manage low-power modes.

Two 32-bit registers contain the seconds, minutes, hours (12- or 24-hour format), day (day of week), date (day of month), month, and year, expressed in binary coded decimal format (BCD). The sub-seconds value is also available in binary format.

Compensations for 28-, 29- (leap year), 30-, and 31-day months are performed automatically. Daylight saving time compensation can also be performed.

Additional 32-bit registers contain the programmable alarm subseconds, seconds, minutes, hours, day, and date.

A digital calibration feature is available to compensate for any deviation in crystal oscillator accuracy.

After backup domain reset, all RTC registers are protected against possible parasitic write accesses.

As long as the supply voltage remains in the operating range, the RTC never stops, regardless of the device status (Run mode, low-power mode or under reset).

798/1749
RM0090 Rev 18
ST

--- Page 19 ---

RM0090
Real-time clock (RTC)

## 26.2 RTC main features

The RTC unit main features are the following (see Figure 237: RTC block diagram):

*   Calendar with subseconds, seconds, minutes, hours (12 or 24 format), day (day of week), date (day of month), month, and year.
*   Daylight saving compensation programmable by software.
*   Two programmable alarms with interrupt function. The alarms can be triggered by any combination of the calendar fields.
*   Automatic wakeup unit generating a periodic flag that triggers an automatic wakeup interrupt.
*   Reference clock detection: a more precise second source clock (50 or 60 Hz) can be used to enhance the calendar precision.
*   Accurate synchronization with an external clock using the subsecond shift feature.
*   Maskable interrupts/events:
    *   Alarm A
    *   Alarm B
    *   Wakeup interrupt
    *   Timestamp
    *   Tamper detection
*   Digital calibration circuit (periodic counter correction)
    *   5 ppm accuracy
    *   0.95 ppm accuracy, obtained in a calibration window of several seconds
*   Timestamp function for event saving (1 event)
*   Tamper detection:
    *   2 tamper events with configurable filter and internal pull-up.
*   20 backup registers (80 bytes). The backup registers are reset when a tamper detection event occurs.
*   Alternate function output (RTC_OUT) which selects one of the following two outputs:
    *   RTC_CALIB: 512 Hz or 1 Hz clock output (with an LSE frequency of 32.768 kHz). This output is enabled by setting the COE bit in the RTC_CR register. It is routed to the device RTC_AF1 function.
    *   RTC_ALARM (Alarm A, Alarm B or wakeup). This output is selected by configuring the OSEL[1:0] bits in the RTC_CR register. It is routed to the device RTC_AF1 function.
*   RTC alternate function inputs:
    *   RTC_TS: timestamp event detection. It is routed to the device RTC_AF1 and RTC_AF2 functions.
    *   RTC_TAMP1: TAMPER1 event detection. It is routed to the device RTC_AF1 and RTC_AF2 functions.
    *   RTC_TAMP2: TAMPER2 event detection.
    *   RTC_REFIN: reference clock input (usually the mains, 50 or 60 Hz).

Refer to Section 8.3.15: Selection of RTC_AF1 and RTC_AF2 alternate functions.

ST
RM0090 Rev 18
799/1749

--- Page 20 ---

Real-time clock (RTC)
RM0090

Figure 237. RTC block diagram

RTC_TAMP3
RTC_TAMP2
RTC_TAMP1
Backup registers
and RTC tamper
control register
TAMPxF

RTC_TS
RTC_REFIN
Time stamp
registers
TSF

LSE (32.768 Hz)
HSE/32
LSI
RTCCLK

RTC_CALR
Smooth
calibration

RTC_PRER
Asynchronous
7-bit prescaler
(default = 128)
ck_apre
(default 256 Hz)

RTC_PRER
Synchronous
15-bit prescaler
(default = 256)
ck_spre
(default 1 Hz)

WUCKSEL[1:0]
Prescaler
1, 2, 4, 8, 16

Calendar
Shadow register
RTC_SSR

Shadow registers
RTC_TR,
RTC_DR

RTC_CALIB
RTC_ALARM
Output
control
RTC_OUT

RTC_WUTR
16-bit wakeup
auto reload timer
WUTF

Alarm A
RTC_ALRMAR
RTC_ALRMASSR
OSEL[1:0]
ALRAF

Alarm B
RTC_ALRMBR
RTC_ALRMBSSR
ALRBF

MS30216V6

1. On STM32F4xx devices, the RTC_AF1 and RTC_AF2 alternate functions are connected to PC13 and PI8, respectively.

800/1749
RM0090 Rev 18

--- Page 21 ---

RM0090
Real-time clock (RTC)


> [RM0090 ch.26 §26.6] RTC registers

### 26.6.4 RTC initialization and status register (RTC_ISR)**

**26.6.4 RTC initialization and status register (RTC_ISR)**
Address offset: 0x0C
Backup domain reset value: 0x0000 0007
System reset value: Not affected except INIT, INITF and RSF which are cleared to 0.

| 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved | RECAL PF |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | r |
| 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Res. | TAMP 2F | TAMP 1F | TSOVF | TSF | WUTF | ALRBF | ALRAF | INIT | INITF | RSF | INITS | SHPF | WUT WF | ALRB WF | ALRA WF |
|  | rc_w0 | nw | r | rc_w0 | r |  |  |  |  |  |  |  |  |  |  |


820/1749
RM0090 Rev 18

--- Page 41 ---

RM0090
Real-time clock (RTC)

Bits 31:17 Reserved

Bit 16 **RECALPF**: Recalibration pending Flag
The RECALPF status flag is automatically set to ‘1’ when software writes to the RTC_CALR register, indicating that the RTC_CALR register is blocked. When the new calibration settings are taken into account, this bit returns to ‘0’. Refer to Section : Re-calibration on-the-fly.

Bit 15 Reserved, must be kept at reset value.

Bit 14 **TAMP2F**: TAMPER2 detection flag
This flag is set by hardware when a tamper detection event is detected on tamper input 2.
It is cleared by software writing 0.

Bit 13 **TAMP1F**: Tamper detection flag
This flag is set by hardware when a tamper detection event is detected.
It is cleared by software writing 0.

Bit 12 **TSOVF**: Timestamp overflow flag
This flag is set by hardware when a timestamp event occurs while TSF is already set.
This flag is cleared by software by writing 0. It is recommended to check and then clear TSOVF only after clearing the TSF bit. Otherwise, an overflow might not be noticed if a timestamp event occurs immediately before the TSF bit is cleared.

Bit 11 **TSF**: Timestamp flag
This flag is set by hardware when a timestamp event occurs.
This flag is cleared by software by writing 0.

Bit 10 **WUTF**: Wakeup timer flag
This flag is set by hardware when the wakeup auto-reload counter reaches 0.
This flag is cleared by software by writing 0.
This flag must be cleared by software at least 1.5 RTCCLK periods before WUTF is set to 1 again.

Bit 9 **ALRBF**: Alarm B flag
This flag is set by hardware when the time/date registers (RTC_TR and RTC_DR) match the Alarm B register (RTC_ALRMAR).
This flag is cleared by software by writing 0.

Bit 8 **ALRAF**: Alarm A flag
This flag is set by hardware when the time/date registers (RTC_TR and RTC_DR) match the Alarm A register (RTC_ALRMAR).
This flag is cleared by software by writing 0.

Bit 7 **INIT**: Initialization mode
0: Free running mode
1: Initialization mode used to program time and date register (RTC_TR and RTC_DR), and prescaler register (RTC_PRER). Counters are stopped and start counting from the new value when INIT is reset.

Bit 6 **INITF**: Initialization flag
When this bit is set to 1, the RTC is in initialization state, and the time, date and prescaler registers can be updated.
0: Calendar registers update is not allowed
1: Calendar registers update is allowed.

ST
RM0090 Rev 18
821/1749

--- Page 42 ---

**Real-time clock (RTC)**
RM0090

**Bit 5 RSF:** Registers synchronization flag
This bit is set by hardware each time the calendar registers are copied into the shadow registers (RTC_SSRx, RTC_TRx and RTC_DRx). This bit is cleared by hardware in initialization mode, while a shift operation is pending (SHPF=1), or when in bypass shadow register mode (BYPSHAD=1). This bit can also be cleared by software.
0: Calendar shadow registers not yet synchronized
1: Calendar shadow registers synchronized

**Bit 4 INITS:** Initialization status flag
This bit is set by hardware when the calendar year field is different from 0 (backup domain reset value state).
0: Calendar has not been initialized
1: Calendar has been initialized

**Bit 3 SHPF:** Shift operation pending
0: No shift operation is pending
1: A shift operation is pending
This flag is set by hardware as soon as a shift operation is initiated by a write to the RTC_SHIFTR. It is cleared by hardware when the corresponding shift operation has been executed. Writing to SHPF has no effect.

**Bit 2 WUTF:** Wakeup timer write flag
This bit is set by hardware up to 2 RTCCLK cycles after the WUTE bit has been set to 0 in RTC_CR, and is cleared up to 2 RTCCLK cycles after the WUTE bit has been set to 1. The wakeup timer values can be changed when WUTE bit is cleared and WUTF is set.
0: Wakeup timer configuration update not allowed
1: Wakeup timer configuration update allowed

**Bit 1 ALRBWF:** Alarm B write flag
This bit is set by hardware when Alarm B values can be changed, after the ALRBIE bit has been set to 0 in RTC_CR.
It is cleared by hardware in initialization mode.
0: Alarm B update not allowed
1: Alarm B update allowed.

**Bit 0 ALRAWF:** Alarm A write flag
This bit is set by hardware when Alarm A values can be changed, after the ALRAE bit has been set to 0 in RTC_CR.
It is cleared by hardware in initialization mode.
0: Alarm A update not allowed
1: Alarm A update allowed

**Note:**
The ALRAF, ALRBF, WUTF and TSF bits are cleared 2 APB clock cycles after programming them to 0.
This register is write protected (except for RTC_ISR[13:8] bits). The write access procedure is described in RTC register write protection on page 803.

822/1749
RM0090 Rev 18
ST

--- Page 43 ---

RM0090 Real-time clock (RTC)

> [RM0090 ch.26 §26.6] RTC registers

### 26.6.5 RTC prescaler register (RTC_PRER)

### 26.6.5 RTC prescaler register (RTC_PRER)

Address offset: 0x10
Backup domain reset value: 0x007F 00FF
System reset: not affected

| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved | PREDIV_A[6:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  | rw |  |  |  |  |  |  |  |


| 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Res. | PREDIV_S[14:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  | rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bits 31:24 Reserved
Bit 23 Reserved, must be kept at reset value.
Bits 22:16 **PREDIV_A[6:0]**: Asynchronous prescaler factor
This is the asynchronous division factor:
ck_apre frequency = RTCCLK frequency/(PREDIV_A+1)
Bit 15 Reserved, must be kept at reset value.
Bits 14:0 **PREDIV_S[14:0]**: Synchronous prescaler factor
This is the synchronous division factor:
ck_spre frequency = ck_apre frequency/(PREDIV_S+1)

*Note:* This register must be written in initialization mode only. The initialization must be performed in two separate write accesses. Refer to *Calendar initialization and configuration on page 804*

This register is write protected. The write access procedure is described in *RTC register write protection on page 803*.

> [RM0090 ch.26 §26.6] RTC registers

### 26.6.6 RTC wakeup timer register (RTC_WUTR)

### 26.6.6 RTC wakeup timer register (RTC_WUTR)

Address offset: 0x14
Backup domain reset value: 0x0000 FFFF
System reset: not affected

| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


| 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| WUT[15:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


STI
RM0090 Rev 18
823/1749

--- Page 44 ---

**Real-time clock (RTC)**
RM0090

Bits 31:16 Reserved
Bits 15:0 **WUT[15:0]**: Wakeup auto-reload value bits
When the wakeup timer is enabled (WUTE set to 1), the WUTF flag is set every (WUT[15:0] + 1) ck_wut cycles. The ck_wut period is selected through WUCKSEL[2:0] bits of the RTC_CR register
When WUCKSEL[2] = 1, the wakeup timer becomes 17-bits and WUCKSEL[1] effectively becomes WUT[16] the most-significant bit to be reloaded into the timer.
*Note: The first assertion of WUTF occurs (WUT+1) ck_wut cycles after WUTE is set. Setting WUT[15:0] to 0x0000 with WUCKSEL[2:0] =011 (RTCCLK/2) is forbidden.*

**Note:**
This register can be written only when WUTF is set to 1 in RTC_ISR.
This register is write protected. The write access procedure is described in *RTC register write protection on page 803*.

> [RM0090 ch.26 §26.6] RTC registers

### 26.6.7 RTC calibration register (RTC_CALIBR)**

**26.6.7 RTC calibration register (RTC_CALIBR)**

Address offset: 0x18
Backup domain reset value: 0x0000 0000
System reset: not affected

| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


| 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved | DCS | Reserved | DC[4:0] |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  | rw |  |  | rw |  |  |  |  |


Bits 31:8 Reserved
Bit 7 **DCS**: Digital calibration sign
0: Positive calibration: calendar update frequency is increased
1: Negative calibration: calendar update frequency is decreased
Bits 6:5 Reserved, must be kept at reset value.
Bits 4:0 **DC[4:0]**: Digital calibration
DCS = 0 (positive calibration)
00000: + 0 ppm
00001: + 4 ppm (rounded value)
00010: + 8 ppm (rounded value)
..
11111: + 126 ppm (rounded value)
DCS = 1 (negative calibration)
00000: –0 ppm
00001: –2 ppm (rounded value)
00010: –4 ppm (rounded value)
..
11111: –63 ppm (rounded value)
Refer to *Case of RTCCLK=32.768 kHz and PREDIV_A+1=128 on page 808* for the exact step value.

824/1749
RM0090 Rev 18

--- Page 45 ---

**RM0090**
**Real-time clock (RTC)**

*Note:* This register can be written in initialization mode only (*RTC_ISR/INITF = '1'*).
This register is write protected. The write access procedure is described in *RTC register write protection on page 803*.

> [RM0090 ch.26 §26.6] RTC registers

### 26.6.8 RTC alarm A register (RTC_ALRMAR)

### 26.6.8 RTC alarm A register (RTC_ALRMAR)

Address offset: 0x1C
Backup domain reset value: 0x0000 0000
System reset: not affected

| 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|
| MSK4 | WDSEL | DT[1:0] | DU[3:0] | MSK3 | PM | HT[1:0] | HU[3:0] |
| rw |  |  |  |  |  |  |  |
| 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0 |  |  |  |  |  |  |  |
| MSK2 | MNT[2:0] | MNU[3:0] | MSK1 | ST[2:0] | SU[3:0] |  |  |
| rw |  |  |  |  |  |  |  |


**Bit 31 MSK4**: Alarm A date mask
0: Alarm A set if the date/day match
1: Date/day don't care in Alarm A comparison

**Bit 30 WDSEL**: Week day selection
0: DU[3:0] represents the date units
1: DU[3:0] represents the week day. DT[1:0] is don't care.

**Bits 29:28 DT[1:0]**: Date tens in BCD format.

**Bits 27:24 DU[3:0]**: Date units or day in BCD format.

**Bit 23 MSK3**: Alarm A hours mask
0: Alarm A set if the hours match
1: Hours don't care in Alarm A comparison

**Bit 22 PM**: AM/PM notation
0: AM or 24-hour format
1: PM

**Bits 21:20 HT[1:0]**: Hour tens in BCD format.

**Bits 19:16 HU[3:0]**: Hour units in BCD format.

**Bit 15 MSK2**: Alarm A minutes mask
0: Alarm A set if the minutes match
1: Minutes don't care in Alarm A comparison

**Bits 14:12 MNT[2:0]**: Minute tens in BCD format.

**Bits 11:8 MNU[3:0]**: Minute units in BCD format.

**Bit 7 MSK1**: Alarm A seconds mask
0: Alarm A set if the seconds match
1: Seconds don't care in Alarm A comparison

**Bits 6:4 ST[2:0]**: Second tens in BCD format.

**Bits 3:0 SU[3:0]**: Second units in BCD format.

ST
RM0090 Rev 18
825/1749

--- Page 46 ---

**Real-time clock (RTC)**
RM0090

*Note:* This register can be written only when ALRAWF is set to 1 in RTC_ISR, or in initialization mode.
This register is write protected. The write access procedure is described in *RTC register write protection on page 803*.


> [RM0090 ch.26 §26.6] RTC registers


### 26.6.9 RTC alarm B register (RTC_ALRMBR)

Address offset: 0x20
Backup domain reset value: 0x0000 0000
System reset: not affected

| 31 | 30 | 29 | 28 | 27 | 26 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| MSK4 | WDSEL | DT[1:0] | DU[3:0] | MSK3 | PM | HT[1:0] | HU[3:0] |  |  |  |  |  |  |
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |


| 15 | 14 | 13 | 12 | 11 | 10 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| MSK2 | MNT[2:0] | MNU[3:0] | MSK1 | ST[2:0] | SU[3:0] |  |  |  |  |  |  |  |  |
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |


**Bit 31 MSK4**: Alarm B date mask
0: Alarm B set if the date and day match
1: Date and day don't care in Alarm B comparison

**Bit 30 WDSEL**: Week day selection
0: DU[3:0] represents the date units
1: DU[3:0] represents the week day. DT[1:0] is don't care.

**Bits 29:28 DT[1:0]**: Date tens in BCD format
**Bits 27:24 DU[3:0]**: Date units or day in BCD format

**Bit 23 MSK3**: Alarm B hours mask
0: Alarm B set if the hours match
1: Hours don't care in Alarm B comparison

**Bit 22 PM**: AM/PM notation
0: AM or 24-hour format
1: PM

**Bits 21:20 HT[1:0]**: Hour tens in BCD format
**Bits 19:16 HU[3:0]**: Hour units in BCD format

**Bit 15 MSK2**: Alarm B minutes mask
0: Alarm B set if the minutes match
1: Minutes don't care in Alarm B comparison

**Bits 14:12 MNT[2:0]**: Minute tens in BCD format
**Bits 11:8 MNU[3:0]**: Minute units in BCD format

**Bit 7 MSK1**: Alarm B seconds mask
0: Alarm B set if the seconds match
1: Seconds don't care in Alarm B comparison

**Bits 6:4 ST[2:0]**: Second tens in BCD format
**Bits 3:0 SU[3:0]**: Second units in BCD format

826/1749
RM0090 Rev 18

--- Page 47 ---

**RM0090**
**Real-time clock (RTC)**

**Note:**
*This register can be written only when ALRBWF is set to 1 in RTC_ISR, or in initialization mode.*
*This register is write protected. The write access procedure is described in RTC register write protection on page 803.*

> [RM0090 ch.26 §26.6] RTC registers

### 26.6.10 RTC write protection register (RTC_WPR)

### 26.6.10 RTC write protection register (RTC_WPR)

Address offset: 0x24
Backup domain reset value: 0x0000 0000

| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
| Reserved | KEY |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  | w |  |  |  |  |  |  |  |


Bits 31:8 Reserved, must be kept at reset value.
Bits 7:0 **KEY**: Write protection key
This byte is written by software.
Reading this byte always returns 0x00.
Refer to *RTC register write protection* for a description of how to unlock RTC register write protection.

> [RM0090 ch.26 §26.6] RTC registers

### 26.6.11 RTC sub second register (RTC_SSR)

### 26.6.11 RTC sub second register (RTC_SSR)

Address offset: 0x28
Backup domain reset value: 0x0000 0000
System reset: 0x0000 0000 when BYPSHAD = 0. Not affected when BYPSHAD = 1.

| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| r |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
| SS[15:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| r |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bits 31:16 Reserved
Bits 15:0 **SS**: Sub second value
SS[15:0] is the value in the synchronous prescaler's counter. The fraction of a second is given by the formula below:
Second fraction = ( PREDIV_S - SS ) / ( PREDIV_S + 1 )
*Note: SS can be larger than PREDIV_S only after a shift operation. In that case, the correct time/date is one second less than as indicated by RTC_TR/RTC_DR.*

STI
RM0090 Rev 18
827/1749

--- Page 48 ---

Real-time clock (RTC)
RM0090


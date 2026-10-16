> [RM0090 ch.26 §26.6] RTC registers


## 26.6 RTC registers

Refer to Section 1.1: List of abbreviations for registers for registers for a list of abbreviations used in register descriptions.

The peripheral registers have to be accessed by words (32 bits).

> [RM0090 ch.26 §26.6] RTC registers

### 26.6.1 RTC time register (RTC_TR)

### 26.6.1 RTC time register (RTC_TR)

The RTC_TR is the calendar time shadow register. This register must be written in initialization mode only. Refer to Calendar initialization and configuration on page 804 and Reading the calendar on page 805.

Address offset: 0x00

Backup domain reset value: 0x0000 0000

System reset: 0x0000 0000 when BYPSHAD = 0. Not affected when BYPSHAD = 1.

| 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|
| Reserved | PM | HT[1:0] | HU[3:0] |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  | rw |
| 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0 |  |  |  |  |  |  |  |  |  |
| Res. | MNT[2:0] | MNU[3:0] | Res. | ST[2:0] | SU[3:0] |  |  |  |  |
| rw |  |  |  |  |  |  |  |  |  |


Bits 31-24 Reserved
Bit 23 Reserved, must be kept at reset value.
Bit 22 **PM**: AM/PM notation
0: AM or 24-hour format
1: PM
Bits 21:20 **HT[1:0]**: Hour tens in BCD format
Bits 19:16 **HU[3:0]**: Hour units in BCD format
Bit 15 Reserved, must be kept at reset value.
Bits 14:12 **MNT[2:0]**: Minute tens in BCD format
Bit 11:8 **MNU[3:0]**: Minute units in BCD format
Bit 7 Reserved, must be kept at reset value.
Bits 6:4 **ST[2:0]**: Second tens in BCD format
Bits 3:0 **SU[3:0]**: Second units in BCD format

*Note:* This register is write protected. The write access procedure is described in RTC register write protection on page 803.

816/1749
RM0090 Rev 18

--- Page 37 ---

RM0090
Real-time clock (RTC)

> [RM0090 ch.26 §26.6] RTC registers

### 26.6.2 RTC date register (RTC_DR)

## 26.6.2 RTC date register (RTC_DR)

The RTC_DR is the calendar date shadow register. This register must be written in initialization mode only. Refer to *Calendar initialization and configuration on page 804* and *Reading the calendar on page 805*.

Address offset: 0x04
Backup domain reset value: 0x0000 2101
System reset: 0x0000 2101 when BYPSHAD = 0. Not affected when BYPSHAD = 1.

| 31 30 29 28 27 26 25 24 | 23 22 21 20 | 19 18 17 16 |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|
| Reserved | YT[3:0] | YU[3:0] |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  | rw |
| 15 14 13 12 11 10 9 8 | 7 6 | 5 4 | 3 2 1 0 |  |  |  |  |  |
| WDU[2:0] | MT | MU[3:0] | Reserved | DT[1:0] | DU[3:0] |  |  |  |
| rw |  |  |  |  |  |  |  |  |


Bits 31-24 Reserved
Bits 23:20 **YT[3:0]**: Year tens in BCD format
Bits 19:16 **YU[3:0]**: Year units in BCD format
Bits 15:13 **WDU[2:0]**: Week day units
000: forbidden
001: Monday
...
111: Sunday
Bit 12 **MT**: Month tens in BCD format
Bits 11:8 **MU**: Month units in BCD format
Bits 7:6 Reserved, must be kept at reset value.
Bits 5:4 **DT[1:0]**: Date tens in BCD format
Bits 3:0 **DU[3:0]**: Date units in BCD format

*Note:* This register is write protected. The write access procedure is described in *RTC register write protection on page 803*.

ST
RM0090 Rev 18
817/1749

--- Page 38 ---

**Real-time clock (RTC)**
RM0090


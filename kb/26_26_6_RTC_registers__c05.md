> [RM0090 ch.26 §26.6] RTC registers

### 26.6.12 RTC shift control register (RTC_SHIFTR)**

**26.6.12 RTC shift control register (RTC_SHIFTR)**

Address offset: 0x2C
Backup domain reset value: 0x0000 0000
System reset: not affected

| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| ADD1S | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| w | r |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
| Res. | SUBFS[14:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| r | w |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


**Bit 31 ADD1S: Add one second**
0: No effect
1: Add one second to the clock/calendar
This bit is write only and is always read as zero. Writing to this bit has no effect when a shift operation is pending (when SHPF=1, in RTC_ISR).
This function is intended to be used with SUBFS (see description below) in order to effectively add a fraction of a second to the clock in an atomic operation.

**Bits 30:15 Reserved**

**Bits 14:0 SUBFS: Subtract a fraction of a second**
These bits are write only and is always read as zero. Writing to this bit has no effect when a shift operation is pending (when SHPF=1, in RTC_ISR).
The value which is written to SUBFS is added to the synchronous prescaler's counter. Since this counter counts down, this operation effectively subtracts from (delays) the clock by:
Delay (seconds) = SUBFS / ( PREDIV_S + 1 )
A fraction of a second can effectively be added to the clock (advancing the clock) when the ADD1S function is used in conjunction with SUBFS, effectively advancing the clock by:
Advance (seconds) = ( 1 - ( SUBFS / ( PREDIV_S + 1 ) ) ) .
*Note: Writing to SUBFS causes RSF to be cleared. Software can then wait until RSF=1 to be sure that the shadow registers have been updated with the shifted time.*
Refer to **Section 26.3.8: RTC synchronization**.

*Note: This register is write protected. The write access procedure is described in RTC register write protection on page 803*

> [RM0090 ch.26 §26.6] RTC registers

### 26.6.13 RTC time stamp time register (RTC_TSTR)**

**26.6.13 RTC time stamp time register (RTC_TSTR)**

Address offset: 0x30
Backup domain reset value: 0x0000 0000
System reset: not affected

| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved | PM | HT[1:0] | HU[3:0] |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  | r |  |  |  |  |  |


828/1749
RM0090 Rev 18

--- Page 49 ---

RM0090
Real-time clock (RTC)

| Res. | MNT[2:0] | MNU[3:0] | Res. | ST[2:0] | SU[3:0] |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| r |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |


Bits 31:23 Reserved, must be kept at reset value.
Bit 22 **PM**: AM/PM notation
0: AM or 24-hour format
1: PM
Bits 21:20 **HT[1:0]**: Hour tens in BCD format.
Bits 19:16 **HU[3:0]**: Hour units in BCD format.
Bit 15 Reserved, must be kept at reset value.
Bits 14:12 **MNT[2:0]**: Minute tens in BCD format.
Bits 11:8 **MNU[3:0]**: Minute units in BCD format.
Bit 7 Reserved, must be kept at reset value.
Bits 6:4 **ST[2:0]**: Second tens in BCD format.
Bits 3:0 **SU[3:0]**: Second units in BCD format.

*Note:* The content of this register is valid only when TSF is set to 1 in RTC_ISR. It is cleared when TSF bit is reset.

> [RM0090 ch.26 §26.6] RTC registers

### 26.6.14 RTC time stamp date register (RTC_TSDR)**

**26.6.14 RTC time stamp date register (RTC_TSDR)**

Address offset: 0x34
Backup domain reset value: 0x0000 0000
System reset: not affected

| Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| WDU[1:0] | MT | MU[3:0] | Reserved | DT[1:0] | DU[3:0] |  |  |  |  |  |  |  |  |  |  |
| r |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 |
| 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |


Bits 31:16 Reserved, must be kept at reset value.
Bits 15:13 **WDU[1:0]**: Week day units
Bit 12 **MT**: Month tens in BCD format
Bits 11:8 **MU[3:0]**: Month units in BCD format
Bits 7:6 Reserved, must be kept at reset value.
Bits 5:4 **DT[1:0]**: Date tens in BCD format
Bit 3:0 **DU[3:0]**: Date units in BCD format

*Note:* The content of this register is valid only when TSF is set to 1 in RTC_ISR. It is cleared when TSF bit is reset.

STI
RM0090 Rev 18
829/1749

--- Page 50 ---

Real-time clock (RTC)
RM0090

> [RM0090 ch.26 §26.6] RTC registers

### 26.6.15 RTC timestamp sub second register (RTC_TSSSR)

## 26.6.15 RTC timestamp sub second register (RTC_TSSSR)

Address offset: 0x38
Backup domain reset value: 0x0000 0000
System reset: not affected

| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| r |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
| SS[15:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| r |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bits 31:16 Reserved
Bits 15:0 **SS**: Sub second value
SS[15:0] is the value of the synchronous prescaler's counter when the timestamp event occurred.

*Note:* The content of this register is valid only when RTC_ISR/TSF is set. It is cleared when the RTC_ISR/TSF bit is reset.


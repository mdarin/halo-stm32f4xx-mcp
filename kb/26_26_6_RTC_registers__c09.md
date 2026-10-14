> [RM0090 ch.26 §26.6] RTC registers

### 26.6.19 RTC alarm B sub second register (RTC_ALRMBSSR)**

**26.6.19 RTC alarm B sub second register (RTC_ALRMBSSR)**

Address offset: 0x48
Backup domain reset value: 0x0000 0000
System reset: not affected

| Reserved | MASKSS[3:0] | Reserved |  |  |
|---|---|---|---|---|
| r | rw | r |  |  |
| Reserved | SS[14:0] |  |  |  |
| r | rw | w | rw |  |


**Bits 31:28** Reserved
**Bits 27:24 MASKSS[3:0]:** Mask the most-significant bits starting at this bit
0x0: No comparison on sub seconds for Alarm B. The alarm is set when the seconds unit is incremented (assuming that the rest of the fields match).
0x1: SS[14:1] are don't care in Alarm B comparison. Only SS[0] is compared.
0x2: SS[14:2] are don't care in Alarm B comparison. Only SS[1:0] are compared.
0x3: SS[14:3] are don't care in Alarm B comparison. Only SS[2:0] are compared.
...
0xC: SS[14:12] are don't care in Alarm B comparison. SS[11:0] are compared.
0xD: SS[14:13] are don't care in Alarm B comparison. SS[12:0] are compared.
0xE: SS[14] is don't care in Alarm B comparison. SS[13:0] are compared.
0xF: All 15 SS bits are compared and must match to activate alarm.
The overflow bits of the synchronous counter (bits 15) is never compared. This bit can be different from 0 only after a shift operation.

**Bits 23:15** Reserved
**Bits 14:0 SS[14:0]:** Sub seconds value
This value is compared with the contents of the synchronous prescaler's counter to determine if Alarm B is to be activated. Only bits 0 up to MASKSS-1 are compared.

**Note:**
*This register can be written only when ALRBIE is reset in RTC_CR register, or in initialization mode.*
*This register is write protected. The write access procedure is described in Section : RTC register write protection*

ST
RM0090 Rev 18
835/1749

--- Page 56 ---

Real-time clock (RTC)
RM0090

26.6.20 RTC backup registers (RTC_BKPxR)

Address offset: 0x50 to 0x9C
Backup domain reset value: 0x0000 0000
System reset: not affected

| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| BKP[31:16] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| NW |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


| 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| BKP[15:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| NW | W | NW |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bits 31:0 BKP[31:0]
The application can write or read data to and from these registers.
They are powered-on by VBAT when VDD is switched off, so that they are not reset by
System reset, and their contents remain valid when the device operates in low-power mode.
This register is reset on a tamper detection event, as long as TAMPEF=1.

26.6.21 RTC register map

Table 121. RTC register map and reset values

| Offset | Register | 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Offset | Register | Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |  |  |
| 0x1C | RTC_ALRMAR | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| MSK4WDSELDT[1:0]DU[3:0]MSK3PMHT[1:0]HU[3:0]MSK2MNT[2:0]MNU[3:0]ST[2:0]MSK1SU[3:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x20 | RTC_ALRMBR | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| MSK4WDSELDT[1:0]DU[3:0]MSK3PMHT[1:0]HU[3:0]MSK2MNT[2:0]MNU[3:0]ST[2:0]MSK1SU[3:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x24 | RTC_WPR | Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x28 | RTC_SSR | Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| SS[15:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x2C | RTC_SHIFTR | 0 | Reserved | ADD1S |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| SUBFS[14:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x30 | RTC_TSTR | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| PMHT[1:0]HU[3:0]ReservedMNT[2:0]MNU[3:0]ReservedST[2:0]SU[3:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x38 | RTC_TSSR | Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| SS[15:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x3C | RTC_CALR | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| ReservedCALPCALW8CALW16ReservedCALM[8:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x40 | RTC_TAFCR | Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| ReservedALARMOUTTYPETSINSELTAMP1INSELTAMP2INSELTAMP1PUDISTAMPPRCH[1:0]TAMPFLT[1:0]TAMPFREQ[2:0]ReservedTAMP1STAMP2TRGTAMP2ETAMP1ETRIGTAMP1E |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x44 | RTC_ALRMASSR | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| ReservedMASKSS[3:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| SS[14:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x48 | RTC_ALRMBSSR | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| ReservedMASKSS[3:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| SS[14:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x50 | RTC_BKPOR | Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| BKP[31:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| to 0x9C | RTC_BKP1R | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| BKP[31:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


837/1749
RM0090 Rev 18

--- Page 58 ---

**Real-time clock (RTC)** RM0090

Refer to Section 2.3: Memory map for the register boundary addresses.

**Caution:** In Table 121, the reset value is the value after a backup domain reset. The majority of the registers are not affected by a system reset. For more information, please refer to Section 26.3.7: Resetting the RTC.

838/1749 RM0090 Rev 18 ST

--- Page 59 ---

RM0090
Inter-integrated circuit (I2C) interface


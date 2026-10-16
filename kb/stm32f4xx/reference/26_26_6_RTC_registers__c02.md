> [RM0090 ch.26 §26.6] RTC registers

### 26.6.3 RTC control register (RTC_CR)**

**26.6.3 RTC control register (RTC_CR)**

Address offset: 0x08
Backup domain reset value: 0x0000 0000
System reset: not affected

| 31 30 29 28 27 26 25 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved | COErw | OSEL[1:0]rw | POLrw | COSELrw | BKPrw | SUB1Hw | ADD1Hw |  |  |  |  |  |  |  |  |
| 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
| TSIErw | WUTIErw | ALRBIErw | ALRAIErw | TSErw | WUTErw | ALRBErw | ALRAErw | DCErw | FMTrw | BYPSHADrw | REFCKONrw | TSEDGErw | WUCKSEL[2:0]rw |  |  |


Bits 31:24 Reserved, must be kept at reset value.

**Bit 23 COE: Calibration output enable**
This bit enables the RTC_CALIB output
0: Calibration output disabled
1: Calibration output enabled

**Bits 22:21 OSEL[1:0]: Output selection**
These bits are used to select the flag to be routed to RTC_ALARM output
00: Output disabled
01: Alarm A output enabled
10: Alarm B output enabled
11: Wakeup output enabled

**Bit 20 POL: Output polarity**
This bit is used to configure the polarity of RTC_ALARM output
0: The pin is high when ALRAF/ALRBF/WUTF is asserted (depending on OSEL[1:0])
1: The pin is low when ALRAF/ALRBF/WUTF is asserted (depending on OSEL[1:0]).

**Bit 19 COSEL: Calibration output selection**
When COE=1, this bit selects which signal is output on RTC_CALIB.
0: Calibration output is 512 Hz
1: Calibration output is 1 Hz
These frequencies are valid for RTCCLK at 32.768 kHz and prescalers at their default values (PREDIV_A=127 and PREDIV_S=255). Refer to Section 26.3.14: Calibration clock output

**Bit 18 BKP: Backup**
This bit can be written by the user to memorize whether the daylight saving time change has been performed or not.

**Bit 17 SUB1H: Subtract 1 hour (winter time change)**
When this bit is set outside initialization mode, 1 hour is subtracted to the calendar time if the current hour is not 0. This bit is always read as 0.
Setting this bit has no effect when current hour is 0.
0: No effect
1: Subtracts 1 hour to the current time. This can be used for winter time change.

818/1749
RM0090 Rev 18

--- Page 39 ---

RM0090
Real-time clock (RTC)

Bit 16 **ADD1H**: Add 1 hour (summer time change)
When this bit is set, 1 hour is added to the calendar time. This bit is always read as 0.
0: No effect
1: Adds 1 hour to the current time. This can be used for summer time change outside initialization mode.

Bit 15 **TSIE**: Timestamp interrupt enable
0: Timestamp Interrupt disable
1: Timestamp Interrupt enable

Bit 14 **WUTIE**: Wakeup timer interrupt enable
0: Wakeup timer interrupt disabled
1: Wakeup timer interrupt enabled

Bit 13 **ALRBIE**: *Alarm B interrupt enable*
0: Alarm B Interrupt disable
1: Alarm B Interrupt enable

Bit 12 **ALRAIE**: *Alarm A interrupt enable*
0: Alarm A interrupt disabled
1: Alarm A interrupt enabled

Bit 11 **TSE**: Time stamp enable
0: Time stamp disable
1: Time stamp enable

Bit 10 **WUTE**: Wakeup timer enable
0: Wakeup timer disabled
1: Wakeup timer enabled
*Note: When the wakeup timer is disabled, wait for WUTF=1 before enabling it again.*

Bit 9 **ALRBE**: *Alarm B enable*
0: Alarm B disabled
1: Alarm B enabled

Bit 8 **ALRAE**: *Alarm A enable*
0: Alarm A disabled
1: Alarm A enabled

Bit 7 **DCE**: Coarse digital calibration enable
0: Digital calibration disabled
1: Digital calibration enabled
PREDIV_A must be 6 or greater

Bit 6 **FMT**: Hour format
0: 24 hour/day format
1: AM/PM hour format

Bit 5 **BYPSHAD**: Bypass the shadow registers
0: Calendar values (when reading from RTC_SSR, RTC_TR, and RTC_DR) are taken from the shadow registers, which are updated once every two RTCCLK cycles.
1: Calendar values (when reading from RTC_SSR, RTC_TR, and RTC_DR) are taken directly from the calendar counters.
*Note: If the frequency of the APB1 clock is less than seven times the frequency of RTCCLK, BYPSHAD must be set to '1'.*

RM0090 Rev 18
819/1749

--- Page 40 ---

**Real-time clock (RTC)**
RM0090

Bit 4 **REFCKON**: Reference clock detection enable (50 or 60 Hz)
0: Reference clock detection disabled
1: Reference clock detection enabled
*Note: PREDIV_S must be 0x00FF.*

Bit 3 **TSEDGE**: Timestamp event active edge
0: TIMESTAMP rising edge generates a timestamp event
1: TIMESTAMP falling edge generates a timestamp event
TSE must be reset when TSEDGE is changed to avoid unwanted TSF setting

Bits 2:0 **WUCKSEL[2:0]**: Wakeup clock selection
000: RTC/16 clock is selected
001: RTC/8 clock is selected
010: RTC/4 clock is selected
011: RTC/2 clock is selected
10x: ck_spre (usually 1 Hz) clock is selected
11x: ck_spre (usually 1 Hz) clock is selected and 2^16 is added to the WUT counter value
(see note below)

*Note:*
WUT = Wakeup unit counter value. WUT = (0x0000 to 0xFFFF) + 0x10000 added when WUCKSEL[2:1 = 11].
Bits 7, 6 and 4 of this register can be written in initialization mode only (RTC_ISR/INITF = 1).
Bits 2 to 0 of this register can be written only when RTC_CR WUTE bit = 0 and RTC_ISR WUTWF bit = 1.
It is recommended not to change the hour during the calendar hour increment as it could mask the incrementation of the calendar hour.
ADD1H and SUB1H changes are effective in the next second.
To avoid spuriously setting of TSF, TSE must be reset when TSEDGE is changed.
This register is write protected. The write access procedure is described in RTC register write protection on page 803.


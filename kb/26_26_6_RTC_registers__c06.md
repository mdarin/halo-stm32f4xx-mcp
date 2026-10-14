> [RM0090 ch.26 §26.6] RTC registers


## 26.6.16 RTC calibration register (RTC_CALR)

Address offset: 0x3C
Backup domain reset value: 0x0000 0000
System reset: not affected

| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| r |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
| CALP | CALW8 | CALW16 | Reserved | CALM[8:0] |  |  |  |  |  |  |  |  |  |  |  |
| rw | r | rw |  |  |  |  |  |  |  |  |  |  |  |  |  |


830/1749
RM0090 Rev 18
STI

--- Page 51 ---

RM0090
Real-time clock (RTC)

Bits 31:16 Reserved

Bit 15 **CALP**: Increase frequency of RTC by 488.5 ppm
0: No RTCCLK pulses are added.
1: One RTCCLK pulse is effectively inserted every 2^11 pulses (frequency increased by 488.5 ppm).
This feature is intended to be used in conjunction with CALM, which lowers the frequency of the calendar with a fine resolution. If the input frequency is 32768 Hz, the number of RTCCLK pulses added during a 32-second window is calculated as follows: (512 * CALP) - CALM.
Refer to Section 26.3.11: *RTC smooth digital calibration*.

Bit 14 **CALW8**: Use an 8-second calibration cycle period
When CALW8 is set to '1', the 8-second calibration cycle period is selected.
CALM[1:0] are stuck at "00" when CALW8='1'.
Refer to Section 26.3.11: *RTC smooth digital calibration*.

Bit 13 **CALW16**: Use a 16-second calibration cycle period
When CALW16 is set to '1', the 16-second calibration cycle period is selected. This bit must not be set to '1' if CALW8=1.
*Note: CALM[0] is stuck at '0' when CALW16='1'.*
Refer to Section 26.3.11: *RTC smooth digital calibration*.

Bits 12:9 Reserved

Bits 8:0 **CALM[8:0]**: Calibration minus
The frequency of the calendar is reduced by masking CALM out of 2^20 RTCCLK pulses (32 seconds if the input frequency is 32768 Hz). This decreases the frequency of the calendar with a resolution of 0.9537 ppm.
To increase the frequency of the calendar, this feature should be used in conjunction with CALP.
See Section 26.3.11: *RTC smooth digital calibration* on page 809.

*Note:* This register is write protected. The write access procedure is described in *RTC register write protection* on page 803

ST
RM0090 Rev 18
831/1749

--- Page 52 ---

Real-time clock (RTC)
RM0090


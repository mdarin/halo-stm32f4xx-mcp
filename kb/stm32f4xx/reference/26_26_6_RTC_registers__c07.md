> [RM0090 ch.26 §26.6] RTC registers


## 26.6.17 RTC tamper and alternate function configuration register (RTC_TAFCR)

Address offset: 0x40
Backup domain reset value: 0x0000 0000
System reset: not affected

| 31 30 29 28 27 26 25 24 23 22 21 20 19 | 18 | 17 | 16 |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved | ALARMOUT TYPE | TSIN SEL | TAMP1 INSEL |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  |  | rw |  |  |
| 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
| TAMP-PUDIS | TAMP-PRCH[1:0] | TAMPFLT[1:0] | TAMPFREQ[2:0] | TAMPT S | Reserved | TAMP2 -TRG | TAMP2 E | TAMPIE | TAMP1 TRG | TAMP1 E |  |  |  |  |  |
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bits 31:19 Reserved. Always read as 0.

Bit 18 **ALARMOUTTYPE**: RTC_ALARM output type
0: RTC_ALARM is an open-drain output
1: RTC_ALARM is a push-pull output

Bit 17 **TSINSEL**: TIMESTAMP mapping
0: RTC_AF1 used as TIMESTAMP
1: RTC_AF2 used as TIMESTAMP

Bit 16 **TAMP1INSEL**: TAMPER1 mapping
0: RTC_AF1 used as TAMPER1
1: RTC_AF2 used as TAMPER1
*Note: TAMP1E must be reset when TAMP1INSEL is changed to avoid unwanted setting of TAMP1F.*

Bit 15 **TAMPPUDIS**: TAMPER pull-up disable
This bit determines if each of the tamper pins are pre-charged before each sample.
0: Precharge tamper pins before sampling (enable internal pull-up)
1: Disable precharge of tamper pins
*Note:*

Bits 14:13 **TAMPPRCH[1:0]**: Tamper precharge duration
These bit determines the duration of time during which the pull-up/is activated before each sample. TAMPPRCH is valid for each of the tamper inputs.
0x0: 1 RTCCLK cycle
0x1: 2 RTCCLK cycles
0x2: 4 RTCCLK cycles
0x3: 8 RTCCLK cycles

832/1749
RM0090 Rev 18

--- Page 53 ---

RM0090
Real-time clock (RTC)

Bits 12:11 **TAMPFLT[1:0]**: Tamper filter count
These bits determines the number of consecutive samples at the specified level
(TAMP*TRG) necessary to activate a Tamper event. TAMPFLT is valid for each of the tamper
inputs.
0x0: Tamper is activated on edge of tamper input transitions to the active level (no internal
pull-up on tamper input).
0x1: Tamper is activated after 2 consecutive samples at the active level.
0x2: Tamper is activated after 4 consecutive samples at the active level.
0x3: Tamper is activated after 8 consecutive samples at the active level.

Bits 10:8 **TAMPFREQ[2:0]**: Tamper sampling frequency
Determines the frequency at which each of the tamper inputs are sampled.
0x0: RTCCLK / 32768 (1 Hz when RTCCLK = 32768 Hz)
0x1: RTCCLK / 16384 (2 Hz when RTCCLK = 32768 Hz)
0x2: RTCCLK / 8192 (4 Hz when RTCCLK = 32768 Hz)
0x3: RTCCLK / 4096 (8 Hz when RTCCLK = 32768 Hz)
0x4: RTCCLK / 2048 (16 Hz when RTCCLK = 32768 Hz)
0x5: RTCCLK / 1024 (32 Hz when RTCCLK = 32768 Hz)
0x6: RTCCLK / 512 (64 Hz when RTCCLK = 32768 Hz)
0x7: RTCCLK / 256 (128 Hz when RTCCLK = 32768 Hz)

Bit 7 **TAMPTS**: Activate timestamp on tamper detection event
0: Tamper detection event does not cause a timestamp to be saved
1: Save timestamp on tamper detection event
TAMPTS is valid even if TSE=0 in the RTC_CR register.

Bits 6:5 Reserved. Always read as 0.

Bit 4 **TAMP2TRG**: Active level for tamper 2
if TAMPFLT != 00:
0: TAMPER2 staying low triggers a tamper detection event.
1: TAMPER2 staying high triggers a tamper detection event.
if TAMPFLT = 00:
0: TAMPER2 rising edge triggers a tamper detection event.
1: TAMPER2 falling edge triggers a tamper detection event.

Bit 3 **TAMP2E**: Tamper 2 detection enable
0: Tamper 2 detection disabled
1: Tamper 2 detection enabled

Bit 2 **TAMPIE**: Tamper interrupt enable
0: Tamper interrupt disabled
1: Tamper interrupt enabled

ST
RM0090 Rev 18
833/1749

--- Page 54 ---

Real-time clock (RTC)
RM0090

Bit 1 **TAMP1TRG**: Active level for tamper 1
if TAMPFLT != 00
0: TAMPER1 staying low triggers a tamper detection event.
1: TAMPER1 staying high triggers a tamper detection event.
if TAMPFLT = 00:
0: TAMPER1 rising edge triggers a tamper detection event.
1: TAMPER1 falling edge triggers a tamper detection event.

**Caution**: When TAMPFLT = 0, TAMPxE must be reset when TAMPxTRG is changed to avoid spurious setting TAMPxF.

Bit 0 **TAMP1E**: Tamper 1 detection enable
0: Tamper 1 detection disabled
1: Tamper 1 detection enabled


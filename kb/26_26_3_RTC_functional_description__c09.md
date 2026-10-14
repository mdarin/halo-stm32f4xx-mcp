> [RM0090 ch.26 §26.3] RTC functional description


### RTC backup registers

The backup registers (RTC_BKPxR) are twenty 32-bit registers for storing 80 bytes of user application data. They are implemented in the backup domain that remains powered-on by

ST
RM0090 Rev 18
811/1749

--- Page 32 ---

Real-time clock (RTC)
RM0090

V_BAT when the V_DD power is switched off. They are not reset by system reset or when the device wakes up from Standby mode. They are reset by a backup domain reset

The backup registers are reset when a tamper detection event occurs (see Section 26.6.20: RTC backup registers (RTC_BKPxR) and Tamper detection initialization on page 812.

**Tamper detection initialization**

Each tamper detection input is associated with the TAMP1F/TAMP2F flags in the RTC_ISR2 register. Each input can be enabled by setting the corresponding TAMP1E/TAMP2E bits to 1 in the RTC_TAFCR register.

A tamper detection event resets all backup registers (RTC_BKPxR).

By setting the TAMPIE bit in the RTC_TAFCR register, an interrupt is generated when a tamper detection event occurs.

**Timestamp on tamper event**

With TAMPTS set to '1', any tamper event causes a timestamp to occur. In this case, either the TSF bit or the TSOVF bit are set in RTC_ISR, in the same manner as if a normal timestamp event occurs. The affected tamper flag register (TAMP1F, TAMP2F) is set at the same time that TSF or TSOVF is set.

**Edge detection on tamper inputs**

If the TAMPFLT bits are "00", the TAMPER pins generate tamper detection events (RTC_TAMP[2:1]) when either a rising edge is observed or an falling edge is observed depending on the corresponding TAMPxTRG bit. The internal pull-up resistors on the TAMPER inputs are deactivated when edge detection is selected.

**Caution:** To avoid losing tamper detection events, the signal used for edge detection is logically ANDed with TAMPxE in order to detect a tamper detection event in case it occurs before the TAMPERx pin is enabled.

*   When TAMPxTRG = 0: if the TAMPERx alternate function is already high before tamper detection is enabled (TAMPxE bit set to 1), a tamper event is detected as soon as TAMPERx is enabled, even if there was no rising edge on TAMPERx after TAMPxE was set.
*   When TAMPxTRG = 1: if the TAMPERx alternate function is already low before tamper detection is enabled, a tamper event is detected as soon as TAMPERx is enabled (even if there was no falling edge on TAMPERx after TAMPxE was set.

After a tamper event has been detected and cleared, the TAMPERx alternate function should be disabled and then re-enabled (TAMPxE set to 1) before re-programming the backup registers (RTC_BKPxR). This prevents the application from writing to the backup registers while the TAMPERx value still indicates a tamper detection. This is equivalent to a level detection on the TAMPERx alternate function.

**Note:** Tamper detection is still active when V_DD power is switched off. To avoid unwanted resetting of the backup registers, the pin to which the TAMPER alternate function is mapped should be externally tied to the correct level.

**Level detection with filtering on tamper inputs**

Level detection with filtering is performed by setting TAMPFLT to a non-zero value. A tamper detection event is generated when either 2, 4, or 8 (depending on TAMPFLT) consecutive

812/1749
RM0090 Rev 18
ST

--- Page 33 ---

RM0090
Real-time clock (RTC)

samples are observed at the level designated by the TAMPxTRG bits
(TAMP1TRG/TAMP2TRG).

The TAMPER inputs are pre-charged through the I/O internal pull-up resistance before its
state is sampled, unless disabled by setting TAMPPUDIS to 1. The duration of the precharge
is determined by the TAMPPRCH bits, allowing for larger capacitances on the tamper
inputs.

The trade-off between tamper detection latency and power consumption through the pull-up
can be optimized by using TAMPFREQ to determine the frequency of the sampling for level
detection.

*Note:* Refer to the datasheets for the electrical characteristics of the pull-up resistors.

> [RM0090 ch.26 §26.3] RTC functional description

### TAMPER alternate function detection

### TAMPER alternate function detection

The TAMPER1 alternate function (RTC_TAMP1) can be mapped either to RTC_AF1(PC13)
or RTC_AF2 (PI8) depending on the value of TAMP1INSEL bit in RTC_TAFCR register (see
Section 26.6.17: RTC tamper and alternate function configuration register (RTC_TAFCR)).
TAMPE bit must be cleared when TAMP1INSEL is modified to avoid unwanted setting of
TAMPF.

The TAMPER 2 alternate function corresponds to RTC_TAMP2 pin.

> [RM0090 ch.26 §26.3] RTC functional description

### 26.3.14 Calibration clock output

### 26.3.14 Calibration clock output

When the COE bit is set to 1 in the RTC_CR register, a reference clock is provided on the
RTC_CALIB device output. If the COSEL bit in the RTC_CR register is reset and
PREDIV_A = 0x7F, the RTC_CALIB frequency is f_RTCLK/64. This corresponds to a
calibration output at 512 Hz for an RTCCLK frequency at 32.768 kHz.

The RTC_CALIB output is not impacted by the calibration value programmed in
RTC_CALIBR register. The RTC_CALIB duty cycle is irregular: there is a light jitter on falling
edges. It is therefore recommended to use rising edges.

If COSEL is set and (PREDIV_S+1) is a non-zero multiple of 256 (i.e: PREDIV_S[7:0] =
0xFF), the RTC_CALIB frequency is f_RTCLK/(256 * (PREDIV_A+1)). This corresponds to a
calibration output at 1 Hz for prescaler default values (PREDIV_A = 0x7F, PREDIV_S =
0xFF), with an RTCCLK frequency at 32.768 kHz. The 1 Hz output is affected when a shift
operation is on going and may toggle during the shift operation (SHPF=1).

> [RM0090 ch.26 §26.3] RTC functional description

### Calibration alternate function output

#### Calibration alternate function output

When the COE bit in the RTC_CR register is set to 1, the calibration alternate function
(RTC_CALIB) is enabled on RTC_AF1.

*Note:* When RTC_CALIB or RTC_ALARM is selected, RTC_AF1 is automatically configured in
output alternate function.

> [RM0090 ch.26 §26.3] RTC functional description

### 26.3.15 Alarm output

### 26.3.15 Alarm output

Three functions can be selected on Alarm output: ALRAF, ALRBF and WUTF. These
functions reflect the contents of the corresponding flags in the RTC_ISR register.

The OSEL[1:0] control bits in the RTC_CR register are used to activate the alarm alternate
function output (RTC_ALARM) in RTC_AF1, and to select the function which is output on
RTC_ALARM.

ST
RM0090 Rev 18
813/1749

--- Page 34 ---

Real-time clock (RTC)
RM0090

The polarity of the output is determined by the POL control bit in RTC_CR so that the opposite of the selected flag bit is output when POL is set to 1.

**Alarm alternate function output**

RTC_ALARM can be configured in output open drain or output push-pull using the control bit ALARMOUTTYPE in the RTC_TAFCR register.

Note:
Once RTC_ALARM is enabled, it has priority over RTC_CALIB (COE bit is don't care on RTC_AF1).
When RTC_CALIB or RTC_ALARM is selected, RTC_AF1 is automatically configured in output alternate function.

> [RM0090 ch.26 §26.3] RTC functional description

### 26.4 RTC and low-power modes

## 26.4 RTC and low-power modes

Table 119. Effect of low-power modes on RTC

| Mode | Description |
|---|---|
| Sleep | No effect RTC interrupts cause the device to exit the Sleep mode. |
| Stop | The RTC remains active when the RTC clock source is LSE or LSI. RTC alarm, RTC tamper event, RTC time stamp event, and RTC Wakeup cause the device to exit the Stop mode. |
| Standby | The RTC remains active when the RTC clock source is LSE or LSI. RTC alarm, RTC tamper event, RTC time stamp event, and RTC Wakeup cause the device to exit the Standby mode. |



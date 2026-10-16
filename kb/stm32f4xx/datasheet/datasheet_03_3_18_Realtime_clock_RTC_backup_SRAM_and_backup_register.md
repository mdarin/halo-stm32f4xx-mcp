### 3.18 Real-time clock (RTC), backup SRAM and backup registers

The backup domain of the STM32F405xx and STM32F407xx includes:
* The real-time clock (RTC)
* 4 Kbytes of backup SRAM
* 20 backup registers

The real-time clock (RTC) is an independent BCD timer/counter. Dedicated registers contain the second, minute, hour (in 12/24 hour), week day, date, month, year, in BCD (binary-coded decimal) format. Correction for 28, 29 (leap year), 30, and 31 day of the month are performed automatically. The RTC provides a programmable alarm and programmable periodic interrupts with wakeup from Stop and Standby modes. The sub-seconds value is also available in binary format.

It is clocked by a 32.768 kHz external crystal, resonator or oscillator, the internal low-power RC oscillator or the high-speed external clock divided by 2 to 31. The internal low-speed RC

DS8626 Rev 12
31/206

--- Page 32 ---

Functional overview
STM32F405xx, STM32F407xx

has a typical frequency of 32 kHz. The RTC can be calibrated using an external 512 Hz output to compensate for any natural quartz deviation.

Two alarm registers are used to generate an alarm at a specific time and calendar fields can be independently masked for alarm comparison. To generate a periodic interrupt, a 16-bit programmable binary auto-reload downcounter with programmable resolution is available and allows automatic wakeup and periodic alarms from every 120 µs to every 36 hours.

A 20-bit prescaler is used for the time base clock. It is by default configured to generate a time base of 1 second from a clock at 32.768 kHz.

The 4-Kbyte backup SRAM is an EEPROM-like memory area. It can be used to store data which need to be retained in VBAT and standby mode. This memory area is disabled by default to minimize power consumption (see Section 3.19: Low-power modes). It can be enabled by software.

The backup registers are 32-bit registers used to store 80 bytes of user application data when VDD power is not present. Backup registers are not reset by a system, a power reset, or when the device wakes up from the Standby mode (see Section 3.19: Low-power modes).

Additional 32-bit registers contain the programmable alarm subseconds, seconds, minutes, hours, day, and date.

Like backup SRAM, the RTC and backup registers are supplied through a switch that is powered either from the VDD supply when present or from the VBAT pin.


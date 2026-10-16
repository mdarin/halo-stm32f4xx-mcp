### 7.3.20 RCC Backup domain control register (RCC_BDCR)

Address offset: 0x70

Reset value: 0x0000 0000, reset by Backup domain reset.
Access: 0 ≤ wait state ≤ 3, word, half-word and byte access
Wait states are inserted in case of successive accesses to this register.

The LSEON, LSEBYP, RTCSEL and RTCEN bits in the *RCC Backup domain control register (RCC_BDCR)* are in the Backup domain. As a result, after Reset, these bits are write-protected and the DBP bit in the *PWR power control register (PWR_CR)* for STM32F405xx/07xx and STM32F415xx/17xx has to be set before these can be modified. Refer to Section 7.1.1: System reset on page 213 for further information. These bits are only reset after a Backup domain Reset (see Section 7.1.3: Backup domain reset). Any internal or external Reset will not have any effect on these bits.

| 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved | BDRST |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | rw |  |  |
| 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| RTCEN | Reserved | RTCSEL[1:0] | Reserved | LSEBYP | LSERDY | LSEON |  |  |  |  |  |  |  |  |  |  |  |
| rw |  |  |  |  |  |  |  |  | rw |  |  |  |  |  | rw | r | rw |


Bits 31:17 Reserved, must be kept at reset value.

**Bit 16 BDRST:** Backup domain software reset
Set and cleared by software.
0: Reset not activated
1: Resets the entire Backup domain
*Note: The BKPSRAM is not affected by this reset, the only way of resetting the BKPSRAM is through the Flash interface when a protection level change from level 1 to level 0 is requested.*

**Bit 15 RTCEN:** RTC clock enable
Set and cleared by software.
0: RTC clock disabled
1: RTC clock enabled

Bits 14:10 Reserved, must be kept at reset value.

**Bits 9:8 RTCSEL[1:0]:** RTC clock source selection
Set by software to select the clock source for the RTC. Once the RTC clock source has been selected, it cannot be changed anymore unless the Backup domain is reset. The BDRST bit can be used to reset them.
00: No clock
01: LSE oscillator clock used as the RTC clock
10: LSI oscillator clock used as the RTC clock
11: HSE oscillator clock divided by a programmable prescaler (selection through the RTCPRE[4:0] bits in the RCC clock configuration register (RCC_CFGR)) used as the RTC clock

Bits 7:3 Reserved, must be kept at reset value.

ST
RM0090 Rev 18
259/1749

--- Page 65 ---

Reset and clock control for STM32F405xx/07xx and STM32F415xx/17xx(RCC) RM0090

Bit 2 **LSEBYP**: External low-speed oscillator bypass
Set and cleared by software to bypass the oscillator. This bit can be written only when the LSE clock is disabled.
0: LSE oscillator not bypassed
1: LSE oscillator bypassed

Bit 1 **LSERDY**: External low-speed oscillator ready
Set and cleared by hardware to indicate when the external 32 kHz oscillator is stable. After the LSEON bit is cleared, LSERDY goes low after 6 external low-speed oscillator clock cycles.
0: LSE clock not ready
1: LSE clock ready

Bit 0 **LSEON**: External low-speed oscillator enable
Set and cleared by software.
0: LSE clock OFF
1: LSE clock ON


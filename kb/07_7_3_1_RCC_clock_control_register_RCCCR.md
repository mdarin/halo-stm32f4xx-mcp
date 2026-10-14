## 7.3 RCC registers

Refer to Section 1.1: List of abbreviations for registers for a list of abbreviations used in register descriptions.

### 7.3.1 RCC clock control register (RCC_CR)

Address offset: 0x00
Reset value: 0x0000 XX83 where X is undefined.
Access: no wait state, word, half-word and byte access

| 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|
| Reserved | PLL2SRDY | PLL2SON | PLLRDY | PLLON | Reserved | CSSON | HSE BYP | HSE RDY | HSE ON |
| r | rw | r | rw | r | rw |  |  |  |  |
| 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0 |  |  |  |  |  |  |  |  |  |
| HSICAL[7:0] | HSITRIM[4:0] | Res. | HSI RDY | HSION |  |  |  |  |  |
| r | rw |  | r | rw |  |  |  |  |  |


Bits 31:28 Reserved, must be kept at reset value.

**Bit 27 PLL2SRDY:** PLLI2S clock ready flag
Set by hardware to indicate that the PLLI2S is locked.
0: PLLI2S unlocked
1: PLLI2S locked

**Bit 26 PLL2SON:** PLLI2S enable
Set and cleared by software to enable PLLI2S.
Cleared by hardware when entering Stop or Standby mode.
0: PLLI2S OFF
1: PLLI2S ON

**Bit 25 PLLRDY:** Main PLL (PLL) clock ready flag
Set by hardware to indicate that PLL is locked.
0: PLL unlocked
1: PLL locked

**Bit 24 PLLON:** Main PLL (PLL) enable
Set and cleared by software to enable PLL.
Cleared by hardware when entering Stop or Standby mode. This bit cannot be reset if PLL clock is used as the system clock.
0: PLL OFF
1: PLL ON

Bits 23:20 Reserved, must be kept at reset value.

**Bit 19 CSSON:** Clock security system enable
Set and cleared by software to enable the clock security system. When CSSON is set, the clock detector is enabled by hardware when the HSE oscillator is ready, and disabled by hardware if an oscillator failure is detected.
0: Clock security system OFF (Clock detector OFF)
1: Clock security system ON (Clock detector ON if HSE oscillator is stable, OFF if not)

224/1749 RM0090 Rev 18

--- Page 30 ---

**RM0090** **Reset and clock control for STM32F405xx/07xx and STM32F415xx/17xx(RCC)**

**Bit 18 HSEBYP**: HSE clock bypass
Set and cleared by software to bypass the oscillator with an external clock. The external clock must be enabled with the HSEON bit, to be used by the device.
The HSEBYP bit can be written only if the HSE oscillator is disabled.
0: HSE oscillator not bypassed
1: HSE oscillator bypassed with an external clock

**Bit 17 HSERDY**: HSE clock ready flag
Set by hardware to indicate that the HSE oscillator is stable. After the HSEON bit is cleared, HSERDY goes low after 6 HSE oscillator clock cycles.
0: HSE oscillator not ready
1: HSE oscillator ready

**Bit 16 HSEON**: HSE clock enable
Set and cleared by software.
Cleared by hardware to stop the HSE oscillator when entering Stop or Standby mode. This bit cannot be reset if the HSE oscillator is used directly or indirectly as the system clock.
0: HSE oscillator OFF
1: HSE oscillator ON

**Bits 15:8 HSICAL[7:0]**: Internal high-speed clock calibration
These bits are initialized automatically at startup.

**Bits 7:3 HSITRIM[4:0]**: Internal high-speed clock trimming
These bits provide an additional user-programmable trimming value that is added to the HSICAL[7:0] bits. It can be programmed to adjust to variations in voltage and temperature that influence the frequency of the internal HSI RC.

**Bit 2** Reserved, must be kept at reset value.

**Bit 1 HSIRDY**: Internal high-speed clock ready flag
Set by hardware to indicate that the HSI oscillator is stable. After the HSION bit is cleared, HSIRDY goes low after 6 HSI clock cycles.
0: HSI oscillator not ready
1: HSI oscillator ready

**Bit 0 HSION**: Internal high-speed clock enable
Set and cleared by software.
Set by hardware to force the HSI oscillator ON when leaving the Stop or Standby mode or in case of a failure of the HSE oscillator used directly or indirectly as the system clock. This bit cannot be cleared if the HSI is used directly or indirectly as the system clock.
0: HSI oscillator OFF
1: HSI oscillator ON

ST
RM0090 Rev 18
225/1749

--- Page 31 ---

**Reset and clock control for STM32F405xx/07xx and STM32F415xx/17xx(RCC)** RM0090


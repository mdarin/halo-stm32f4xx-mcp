## 5 Power controller (PWR)

This section applies to the whole STM32F4xx family, unless otherwise specified.

**5.1 Power supplies**

The device requires a 1.8 to 3.6 V operating voltage supply (V voltage regulator is used to supply the internal 1.2 V digital power.

DD ). An embedded linear

The real-time clock (RTC), the RTC backup registers, and the backup SRAM (BKP SRAM) voltage when the main V DD supply is powered off. Depending on the operating power supply range, some peripheral may be used with limited functionality and performance. For more details refer to section “General operating

can be powered from the V BAT

Note:

conditions” in STM32F4xx datasheets.

**Figure 9. Power supply overview for STM32F405xx/07xx and STM32F415xx/17xx**

VBAT

Backup circuitry

VBAT =

Power (OSC32K,RTC,

1.65 to 3.6V

switch Wakeup logic

Backup registers,

backup RAM)



OUT

GPIOs

IN

VCAP_1

Kernel logic

2 × 2.2 μF VCAP_2

(CPU, digital

& RAM)

VDD VDD

1/2/...14/15 Voltage

15 × 100 nF VSS regulator

+ 1 × 4.7 μF 1/2/...14/15

Flash memory

BYPASS_REG

Reset

PDR_ON

controller

VDD

VDDA

VREF

VREF+

Analog:

100 nF 100 nF VREF- ADC

RCs,

+ 1 μF + 1 μF

PLL,..

VSSA

MS19911V2

1. V DDA and V SSA must be connected to V DD and V SS , respectively.

116/1749

RM0090 Rev 18

--- Page 117 ---

**RM0090**

**Power controller (PWR)**

**Figure 10. Power supply overview for STM32F42xxx and STM32F43xxx**

V BAT

Backup circuitry

Power switch (OSC32K,RTC,

VBAT =

Wakeup logic

1.65 to 3.6V

Backup registers,

backup RAM)



OUT

GP I/Os

IN Kernel logic

(CPU, digital & RAM)

2 × 2.2 μF V V CAP_1 CAP_2

V DD V DD Voltage

1/2/...14/20

regulator

19 × 100 nF

+ 1 × 4.7 μF 1/2/...14/20 VSS

Flash memory

BYPASS_REG

Reset

PDR_ON controller

VDD VDDA

| V REF+ |  |  |
|---|---|---|
| V SSA |  |  |


VREF

Analog:

100 nF + 1 μF 100 nF + 1 μF ADC

RCs, PLL,

...

MS30482V2

1. V DDA and V SSA must be connected to V DD and V SS , respectively.

**5.1.1 Independent A/D converter supply and reference voltage**

To improve conversion accuracy, the ADC has an independent power supply which can be

separately filtered and shielded from noise on the PCB.

DDA pin.

• The ADC voltage supply input is available on a separate V

• An isolated supply ground connection is provided on pin V

SSA .

To ensure a better accuracy of low voltage inputs, the user can connect a separate external . The voltage on V REF ranges from 1.8 V to V DDA .

reference voltage ADC input on V REF



149

--- Page 118 ---

**Power controller (PWR) RM0090**

**5.1.2 Battery backup domain**

**Backup domain description**

To retain the content of the RTC backup registers, backup SRAM, and supply the RTC when V DD is turned off, V BAT pin can be connected to an optional standby voltage supplied by a battery or by another source.

To allow the RTC to operate even when the main digital supply (V DD ) is turned off, the V BAT pin powers the following blocks:

• The RTC

• The LSE oscillator

• The backup SRAM when the low-power backup regulator is enabled

• PC13 to PC15 I/Os, plus PI8 I/O (when available)

The switch to the V BAT supply is controlled by the power-down reset embedded in the Reset block.

**Warning: During t RSTTEMPO (temporization at V DD startup) or after a PDR**

**is detected, the power switch between V BAT and V DD remains connected to V BAT .**

**During the startup phase, if V DD is established in less than t RSTTEMPO (Refer to the datasheet for the value of t RSTTEMPO ) and V DD > V BAT + 0.6 V, a current may be injected into V BAT through an internal diode connected between V DD and the power switch (V BAT ).**

**If the power supply/battery connected to the V BAT pin cannot support this current injection, it is strongly recommended to connect an external low-drop diode between this power supply and the V BAT pin.**

If no external battery is used in the application, it is recommended to connect the V BAT pin to V DD with a 100 nF external decoupling ceramic capacitor in parallel.

When the backup domain is supplied by V DD (analog switch connected to V DD ), the following functions are available:

• PC14 and PC15 can be used as either GPIO or LSE pins

• PC13 can be used as a GPIOas the RTC_AF1 pin (refer to Table 37: RTC_AF1 pin for

more details about this pin configuration)

Note: Due to the fact that the switch only sinks a limited amount of current (3 mA), the use of PI8 and PC13 to PC15 GPIOs in output mode is restricted: the speed has to be limited to 2 MHz with a maximum load of 30 pF and these I/Os must not be used as a current source (e.g. to drive an LED).



--- Page 119 ---

**RM0090 Power controller (PWR)**

When the backup domain is supplied by V BAT (analog switch connected to V BAT because V DD is not present), the following functions are available:

• PC14 and PC15 can be used as LSE pins only

• PC13 can be used as the RTC_AF1 pin (refer to Table 37: RTC_AF1 pin for more

details about this pin configuration)

• PI8 can be used as RTC_AF2

**Backup domain access**

After reset, the backup domain (RTC registers, RTC backup register and backup SRAM) is protected against possible unwanted write accesses. To enable access to the backup domain, proceed as follows:

• Access to the RTC and RTC backup registers

1. Enable the power interface clock by setting the PWREN bits in the RCC_APB1ENR

register (see Section 7.3.13 and Section 6.3.13 )

2. Set the DBP bit in the Section 5.4.1 and PWR power control register (PWR_CR) for

STM32F42xxx and STM32F43xxx to enable access to the backup domain

3. Select the RTC clock source: see Section 7.2.8: RTC/AWU clock

4. Enable the RTC clock by programming the RTCEN [15] bit in the Section 7.3.20: RCC

Backup domain control register (RCC_BDCR)

• Access to the backup SRAM

1. Enable the power interface clock by setting the PWREN bits in the RCC_APB1ENR

register (see Section 7.3.13 and Section 6.3.13 for STM32F405xx/07xx and STM32F415xx/17xx and STM32F42xxx and STM32F43xxx, respectively)

2. Set the DBP bit in the PWR power control register (PWR_CR) for STM32F405xx/07xx

and STM32F415xx/17xx and PWR power control register (PWR_CR) for STM32F42xxx and STM32F43xxx to enable access to the backup domain

3. Enable the backup SRAM clock by setting BKPSRAMEN bit in the RCC AHB1

peripheral clock enable register (RCC_AHB1ENR) .

**RTC and RTC backup registers**

The real-time clock (RTC) is an independent BCD timer/counter. The RTC provides a time- of-day clock/calendar, two programmable alarm interrupts, and a periodic programmable wakeup flag with interrupt capability. The RTC contains 20 backup data registers (80 bytes) which are reset when a tamper detection event occurs. For more details refer to Section 26: Real-time clock (RTC) .

**Backup SRAM**

The backup domain includes 4 Kbytes of backup SRAM addressed in 32-bit, 16-bit or 8-bit mode. Its content is retained even in Standby or V BAT mode when the low-power backup regulator is enabled. It can be considered as an internal EEPROM when V BAT is always present.

When the backup domain is supplied by V DD (analog switch connected to V DD ), the backup SRAM is powered from V DD which replaces the V BAT power supply to save battery life. When the backup domain is supplied by V BAT (analog switch connected to V BAT because V DD is not present), the backup SRAM is powered by a dedicated low-power regulator. This regulator can be ON or OFF depending whether the application needs the backup SRAM function in Standby and V BAT modes or not. The power-down of this regulator is controlled



149

--- Page 120 ---

**Power controller (PWR) RM0090**

by a dedicated bit, the BRE control bit of the PWR_CSR register (see Section 5.4.2: PWR power control/status register (PWR_CSR)  for STM32F405xx/07xx and STM32F415xx/17xx ).

The backup SRAM is not mass erased by an tamper event. When the Flash memory is read protected, the backup SRAM is also read protected to prevent confidential data, such as cryptographic private key, from being accessed. When the protection level change from level 1 to level 0 is requested, the backup SRAM content is erased.

**Figure 11. Backup domain**

Power switch LP  voltage regulator Voltage regulator 3.3 -> 1.2 V

3.3 -> 1.2 V

**1.2 V domain**

Backup SRAM Backup SRAM

interface 1.2 V

RTC LSE 32.768 Hz

**Backup domain**

MS30430V1

**5.1.3 Voltage regulator for STM32F405xx/07xx and STM32F415xx/17xx**

An embedded linear voltage regulator supplies all the digital circuitries except for the backup domain and the Standby circuitry. The regulator output voltage is around 1.2 V. This voltage regulator requires one or two external capacitors to be connected to one or two dedicated pins, V CAP_1 and  V CAP_2 available in all packages. Specific pins must be connected either to V SS or V DD to activate or deactivate the voltage regulator. These pins depend on the package.

When activated by software, the voltage regulator is always enabled after Reset. It works in three different modes depending on the application modes.

• In Run mode , the regulator supplies full power to the 1.2 V domain (core, memories

and digital peripherals). In this mode, the regulator output voltage (around 1.2 V) can be scaled by software to different voltage values:

Scale 1 or scale 2 can be configured on the fly through VOS (bit 15 of the PWR_CR register).

The voltage scaling allows optimizing the power consumption when the device is clocked below the maximum system frequency.

• In Stop mode, the main regulator or the low-power regulator supplies to the 1.2 V

domain, thus preserving the content of registers and internal SRAM. The voltage



--- Page 121 ---

**RM0090 Power controller (PWR)**

regulator can be put either in main regulator mode (MR) or in low-power mode (LPR). The programmed voltage scale remains the same during Stop mode:

The programmed voltage scale remains the same during Stop mode (see Section 5.4.1: PWR power control register (PWR_CR) for STM32F405xx/07xx and STM32F415xx/17xx ).

• In Standby mode , the regulator is powered down. The content of the registers and SRAM are lost except for the Standby circuitry and the backup domain.

Note: For more details, refer to the voltage regulator section in the STM32F405xx/07xx and STM32F415xx/17xx datasheets.

**5.1.4 Voltage regulator for STM32F42xxx and STM32F43xxx**

An embedded linear voltage regulator supplies all the digital circuitries except for the backup domain and the Standby circuitry. The regulator output voltage is around 1.2 V.

This voltage regulator requires two external capacitors to be connected to two dedicated pins, V CAP_1 and V CAP_2 available in all packages. Specific pins must be connected either to V SS or V DD to activate or deactivate the voltage regulator. These pins depend on the package.

When activated by software, the voltage regulator is always enabled after Reset. It works in three different modes depending on the application modes (Run, Stop, or Standby mode).

• In Run mode , the main regulator supplies full power to the 1.2 V domain (core, memories and digital peripherals). In this mode, the regulator output voltage (around

1.2 V) can be scaled by software to different voltage values (scale 1, scale 2, and scale 3 can be configured through VOS[1:0] bits of the PWR_CR register). The scale can be modified only when the PLL is OFF and the HSI or HSE clock source is selected as system clock source. The new value programmed is active only when the PLL is ON. When the PLL is OFF, the voltage scale 3 is automatically selected.

The voltage scaling allows optimizing the power consumption when the device is clocked below the maximum system frequency. After exit from Stop mode, the voltage



149

--- Page 122 ---

**Power controller (PWR) RM0090**

scale 3 is automatically selected.(see Section 5.4.1: PWR power control register (PWR_CR) for STM32F405xx/07xx and STM32F415xx/17xx .

2 operating modes are available:

– Normal mode : The CPU and core logic operate at maximum frequency at a given

voltage scaling (scale 1, scale 2 or scale 3)

– Over-drive mode : This mode allows the CPU and the core logic to operate at a

higher frequency than the normal mode for the voltage scaling scale 1 and scale 2.

• In Stop mode : the main regulator or low-power regulator supplies a low-power voltage to the 1.2V domain, thus preserving the content of registers and internal SRAM. The voltage regulator can be put either in main regulator mode (MR) or in low-power mode (LPR). Both modes can be configured by software as follows:

– Normal mode : the 1.2 V domain is preserved in nominal leakage mode. It is the

default mode when the main regulator (MR) or the low-power regulator (LPR) is enabled.

– Under-drive mode : the 1.2 V domain is preserved in reduced leakage mode. This

mode is only available with the main regulator or in low-power regulator mode (see Table 22 ).

• In Standby mode : the regulator is powered down. The content of the registers and SRAM are lost except for the Standby circuitry and the backup domain.

Note: Over-drive and under-drive mode are not available when the regulator is bypassed. For more details, refer to the voltage regulator section in the STM32F42xxx and STM32F43xxx datasheets.

**Table 22. Voltage regulator configuration mode versus device operating mode (1)**

| Voltage regulator configuration | Run mode | Sleep mode | Stop mode | Standby mode |
|---|---|---|---|---|
| Normal mode | MR | MR or LPR | - |  |
| Over-drive mode(2) | MR | - |  |  |
| Under-drive mode | - | MR or LPR | - |  |
| Power-down mode | - | Yes |  |  |


1. ‘-’ means that the corresponding configuration is not available.

2. The over-drive mode is not available when V DD = 1.8 to 2.1 V.



--- Page 123 ---

**RM0090 Power controller (PWR)**

**Entering Over-drive mode**

It is recommended to enter Over-drive mode when the application is not running critical tasks and when the system clock source is either HSI or HSE. To optimize the configuration time, enable the Over-drive mode during the PLL lock phase.

To enter Over-drive mode, follow the sequence below:

1. Select HSI or HSE as system clock.

2. Configure RCC_PLLCFGR register and set PLLON bit of RCC_CR register.

3. Set ODEN bit of PWR_CR register to enable the Over-drive mode and wait for the

ODRDY flag to be set in the PWR_CSR register.

4. Set the ODSW bit in the PWR_CR register to switch the voltage regulator from Normal

mode to Over-drive mode. The System will be stalled during the switch but the PLL clock system will be still running during locking phase.

5. Wait for the ODSWRDY flag in the PWR_CSR to be set.

6. Select the required Flash latency as well as AHB and APB prescalers.

7. Wait for PLL lock.

8. Switch the system clock to the PLL.

9. Enable the peripherals that are not generated by the System PLL (I2S clock, LCD-TFT

clock, SAI1 clock, USB_48MHz clock....).

Note: The PLLI2S and PLLSAI can be configured at the same time as the system PLL.

During the Over-drive switch activation, no peripheral clocks should be enabled. The peripheral clocks must be enabled once the Over-drive mode is activated.

Entering Stop mode disables the Over-drive mode, as well as the PLL. The application software has to configure again the Over-drive mode and the PLL after exiting from Stop mode.

**Exiting from Over-drive mode**

It is recommended to exit from Over-drive mode when the application is not running critical tasks and when the system clock source is either HSI or HSE.There are two sequences that allow exiting from over-drive mode:

• By resetting simultaneously the ODEN and ODSW bits bit in the PWR_CR register

(sequence 1)

• By resetting first the ODSW bit to switch the voltage regulator to Normal mode and then

resetting the ODEN bit to disable the Over-drive mode (sequence 2).

Example of sequence 1:

1. Select HSI or HSE as system clock source.

2. Disable the peripheral clocks that are not generated by the System PLL (I2S clock,

LCD-TFT clock, SAI1 clock, USB_48MHz clock,....)

3. Reset simultaneously the ODEN and the ODSW bits in the PWR_CR register to switch

back the voltage regulator to Normal mode and disable the Over-drive mode.

4. Wait for the ODWRDY flag of PWR_CSR to be reset.



149

--- Page 124 ---

**Power controller (PWR) RM0090**

Example of sequence 2:

1. Select HSI or HSE as system clock source.

2. Disable the peripheral clocks that are not generated by the System PLL (I2S clock,

LCD-TFT clock, SAI1 clock, USB_48MHz clock,....).

3. Reset the ODSW bit in the PWR_CR register to switch back the voltage regulator to

Normal mode. The system clock is stalled during voltage switching.

4. Wait for the ODWRDY flag of PWR_CSR to be reset.

5. Reset the ODEN bit in the PWR_CR register to disable the Over-drive mode. Note: During step 3, the ODEN bit remains set and the Over-drive mode is still enabled but not active (ODSW bit is reset). If the ODEN bit is reset instead, the Over-drive mode is disabled and the voltage regulator is switched back to the initial voltage.


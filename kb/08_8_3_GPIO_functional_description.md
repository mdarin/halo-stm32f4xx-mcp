# 8 General-purpose I/Os (GPIO)

This section applies to the whole STM32F4xx family, unless otherwise specified.

## 8.1 GPIO introduction

Each general-purpose I/O port has four 32-bit configuration registers (GPIOx_MODER, GPIOx_OTYPER, GPIOx_OSPEEDR and GPIOx_PUPDR), two 32-bit data registers (GPIOx_IDR and GPIOx_ODR), a 32-bit set/reset register (GPIOx_BSRR), a 32-bit locking register (GPIOx_LCKR) and two 32-bit alternate function selection register (GPIOx_AFRH and GPIOx_AFRL).

## 8.2 GPIO main features

*   Up to 16 I/Os under control
*   Output states: push-pull or open drain + pull-up/down
*   Output data from output data register (GPIOx_ODR) or peripheral (alternate function output)
*   Speed selection for each I/O
*   Input states: floating, pull-up/down, analog
*   Input data to input data register (GPIOx_IDR) or peripheral (alternate function input)
*   Bit set and reset register (GPIOx_BSRR) for bitwise write access to GPIOx_ODR
*   Locking mechanism (GPIOx_LCKR) provided to freeze the I/O configuration
*   Analog function
*   Alternate function input/output selection registers (at most 16 AFs per I/O)
*   Fast toggle capable of changing every two clock cycles
*   Highly flexible pin multiplexing allows the use of I/O pins as GPIOs or as one of several peripheral functions

## 8.3 GPIO functional description

Subject to the specific hardware characteristics of each I/O port listed in the datasheet, each port bit of the general-purpose I/O (GPIO) ports can be individually configured by software in several modes:

*   Input floating
*   Input pull-up
*   Input-pull-down
*   Analog
*   Output open-drain with pull-up or pull-down capability
*   Output push-pull with pull-up or pull-down capability
*   Alternate function push-pull with pull-up or pull-down capability
*   Alternate function open-drain with pull-up or pull-down capability

ST
RM0090 Rev 18
267/1749

--- Page 73 ---

**General-purpose I/Os (GPIO)**

RM0090

Each I/O port bit is freely programmable, however the I/O port registers have to be accessed as 32-bit words, half-words or bytes. The purpose of the GPIOx_BSRR register is to allow atomic read/modify accesses to any of the GPIO registers. In this way, there is no risk of an IRQ occurring between the read and the modify access.

*Figure 25* shows the basic structure of a 5 V tolerant I/O port bit. *Table 39* gives the possible port bit configurations.

**Figure 25. Basic structure of a five-volt tolerant I/O port bit**

[Diagram of a five-volt tolerant I/O port bit structure]

1. VDD_FT is a potential specific to five-volt tolerant I/Os and different from VDD.

**Table 35. Port bit configuration table(1)**

| MODER(i)[1:0] | OTYPER(i) | OSPEEDR(i)[B:A] | PUPDR(i)[1:0] | I/O configuration |  |
|---|---|---|---|---|---|
| 01 | 0 | SPEED[B:A] | 0 | GP output | PP |
| 0 | 1 | GP output | PP + PU |  |  |
| 0 | 1 | 0 | GP output | PP + PD |  |
| 0 | 1 | Reserved |  |  |  |
| 1 | 0 | GP output | OD |  |  |
| 1 | 0 | 1 | GP output | OD + PU |  |
| 1 | 0 | GP output | OD + PD |  |  |
| 1 | Reserved (GP output OD) |  |  |  |  |


268/1749 RM0090 Rev 18 ST

--- Page 74 ---

RM0090
General-purpose I/Os (GPIO)

Table 35. Port bit configuration table(1) (continued)

| MODER(i)[1:0] | OTYPER(i) | OSPEEDR(i)[B:A] | PUPDR(i)[1:0] | I/O configuration |  |
|---|---|---|---|---|---|
| 10 | 0 | SPEED[B:A] | 0 | AF | PP |
| 0 | 1 | AF | PP + PU |  |  |
| 0 | 1 | 0 | AF | PP + PD |  |
| 0 | 1 | Reserved |  |  |  |
| 1 | 0 | AF | OD |  |  |
| 1 | 0 | 1 | AF | OD + PU |  |
| 1 | 0 | AF | OD + PD |  |  |
| 1 | Reserved |  |  |  |  |
| 00 | x | 0 | Input | Floating |  |
| x | 0 | 1 | Input | PU |  |
| x | 1 | 0 | Input | PD |  |
| x | 1 | Reserved (input floating) |  |  |  |
| 11 | x | 0 | Input/output | Analog |  |
| x | 0 | 1 | Reserved |  |  |
| x | 1 | 0 |  |  |  |
| x | 1 |  |  |  |  |


1. GP = general-purpose, PP = push-pull, PU = pull-up, PD = pull-down, OD = open-drain, AF = alternate function.

### 8.3.1 General-purpose I/O (GPIO)

During and just after reset, the alternate functions are not active and the I/O ports are configured in input floating mode.

The debug pins are in AF pull-up/pull-down after reset:

* PA15: JTDI in pull-up
* PA14: JTCK/SWCLK in pull-down
* PA13: JTMS/SWDAT in pull-up
* PB4: NJTRST in pull-up
* PB3: JTDO in floating state

When the pin is configured as output, the value written to the output data register (GPIOx_ODR) is output on the I/O pin. It is possible to use the output driver in push-pull mode or open-drain mode (only the N-MOS is activated when 0 is output).

The input data register (GPIOx_IDR) captures the data present on the I/O pin at every AHB1 clock cycle.

All GPIO pins have weak internal pull-up and pull-down resistors, which can be activated or not depending on the value in the GPIOx_PUPDR register.

ST
RM0090 Rev 18
269/1749

--- Page 75 ---

General-purpose I/Os (GPIO) RM0090

### 8.3.2 I/O pin multiplexer and mapping

The microcontroller I/O pins are connected to onboard peripherals/modules through a multiplexer that allows only one peripheral's alternate function (AF) connected to an I/O pin at a time. In this way, there can be no conflict between peripherals sharing the same I/O pin.

Each I/O pin has a multiplexer with sixteen alternate function inputs (AF0 to AF15) that can be configured through the GPIOx_AFRL (for pin 0 to 7) and GPIOx_AFRH (for pin 8 to 15) registers:

*   After reset all I/Os are connected to the system's alternate function 0 (AF0)
*   The peripherals' alternate functions are mapped from AF1 to AF13
*   Cortex®-M4 with FPU EVENTOUT is mapped on AF15

This structure is shown in Figure 26 below.

In addition to this flexible I/O multiplexing architecture, each peripheral has alternate functions mapped onto different I/O pins to optimize the number of peripherals available in smaller packages.

To use an I/O in a given configuration, proceed as follows:

*   **System function**
    Connect the I/O to AF0 and configure it depending on the function used:
    - JTAG/SWD, after each device reset these pins are assigned as dedicated pins immediately usable by the debugger host (not controlled by the GPIO controller)
    - RTC_REFIN: this pin should be configured in Input floating mode
    - MCO1 and MCO2: these pins have to be configured in alternate function mode.

**Note:** The user can disable some or all of the JTAG/SWD pins and so release the associated pins for GPIO usage (released pins highlighted in gray in the table).

For more details please refer to Section 7.2.10: Clock-out capability and Section 6.2.10: Clock-out capability.

270/1749 RM0090 Rev 18 STI

--- Page 76 ---

RM0090
General-purpose I/Os (GPIO)

Table 36. Flexible SWJ-DP pin assignment

| Available debug ports | SWJ I/O pin assigned |  |  |  |
|---|---|---|---|---|
| PA13 / JTMS/ SWDIO | PA14 / JTCK/ SWCLK | PA15 / JTDI | PB3 / JTDO | PB4/ NJTRST |
| Full SWJ (JTAG-DP + SW-DP) - Reset state | X |  |  |  |
| Full SWJ (JTAG-DP + SW-DP) but without NJTRST | X |  |  |  |
| JTAG-DP Disabled and SW-DP Enabled | X |  |  |  |
| JTAG-DP Disabled and SW-DP Disabled | Released |  |  |  |


*   **GPIO**
    Configure the desired I/O as output or input in the GPIOx_MODER register.
*   **Peripheral alternate function**
    For the ADC and DAC, configure the desired I/O as analog in the GPIOx_MODER register.
    For other peripherals:
    - Configure the desired I/O as an alternate function in the GPIOx_MODER register
    - Select the type, pull-up/pull-down and output speed via the GPIOx_OTYPER, GPIOx_PUPDR and GPIOx_OSPEEDR registers, respectively
    - Connect the I/O to the desired AFx in the GPIOx_AFRL or GPIOx_AFRH register
*   **EVENTOUT**
    Configure the I/O pin used to output the Cortex®-M4 with FPU EVENTOUT signal by connecting it to AF15

Note: EVENTOUT is not mapped onto the following I/O pins: PC13, PC14, PC15, PH0, PH1 and PI8.

Please refer to the “Alternate function mapping” table in the datasheets for the detailed mapping of the system and peripherals’ alternate function I/O pins.

ST
RM0090 Rev 18
271/1749

--- Page 77 ---

General-purpose I/Os (GPIO)

RM0090

Figure 26. Selecting an alternate function on STM32F405xx/07xx and STM32F415xx/17xx

For pins 0 to 7, the GPIOx_AFRL[31:0] register selects the dedicated alternate function

AF0 (system)
AF1 (TIM1/TIM2)
AF2 (TIM3..5)
AF3 (TIM8..11)
AF4 (I2C1..3)
AF5 (SPI1/SPI2)
AF6 (SPI3)
AF7 (USART1..3)
AF8 (USART4..6)
AF9 (CAN1/CAN2, TIM12..14)
AF10 (OTG_FS, OTG_HS)
AF11 (ETH)
AF12 (FSMC, SDIO, OTG_HS(1))
AF13 (DCMI)
AF14
AF15 (EVENTOUT)

Pin x (x = 0..7)
1
AFRL[31:0]

For pins 8 to 15, the GPIOx_AFRH[31:0] register selects the dedicated alternate function

AF0 (system)
AF1 (TIM1/TIM2)
AF2 (TIM3..5)
AF3 (TIM8..11)
AF4 (I2C1..3)
AF5 (SPI1/SPI2)
AF6 (SPI3)
AF7 (USART1..3)
AF8 (USART4..6)
AF9 (CAN1/CAN2, TIM12..14)
AF10 (OTG_FS, OTG_HS)
AF11 (ETH)
AF12 (FSMC, SDIO, OTG_HS(1))
AF13 (DCMI)
AF14
AF15 (EVENTOUT)

Pin x (x = 8..15)
1
AFRH[31:0]

ai17538

1. Configured in FS.

272/1749
RM0090 Rev 18
ST

--- Page 78 ---

RM0090
General-purpose I/Os (GPIO)

**Figure 27. Selecting an alternate function on STM32F42xxx and STM32F43xxx**

For pins 0 to 7, the GPIOx_AFRL[31:0] register selects the dedicated alternate function

AF0 (system)
AF1 (TIM1/TIM2)
AF2 (TIM3..5)
AF3 (TIM8..11)
AF4 (I2C1..3)
AF5 (SPI1/2/3/4/5/6)
AF6 (SPI2/3/SAI1)
AF7 (USART1..3)
AF8 (USART4..8)
AF9 (CAN1/CAN2, LTDC, TIM12..14)
AF10 (OTG_FS, OTG_HS)
AF11 (ETH)
AF12 (FMC, SDIO, OTG_HS(1))
AF13 (DCMI)
AF14 (LTDC)
AF15 (EVENTOUT)

Pin x (x = 0..7)
1
AFRL[31:0]

For pins 8 to 15, the GPIOx_AFRH[31:0] register selects the dedicated alternate function

AF0 (system)
AF1 (TIM1/TIM2)
AF2 (TIM3..5)
AF3 (TIM8..11)
AF4 (I2C1..3)
AF5 (SPI1/2/4/5/6)
AF6 (SPI2/3/SAI1)
AF7 (USART1..3)
AF8 (USART4..8)
AF9 (CAN1/CAN2, TIM12..14)
AF10 (OTG_FS, OTG_HS)
AF11 (ETH)
AF12 (FMC, SDIO, OTG_HS(1))
AF13 (DCMI)
AF14 (LTDC)
AF15 (EVENTOUT)

Pin x (x = 8..15)
1
AFRH[31:0]

MS60208V1

1. Configured in FS.

ST
RM0090 Rev 18
273/1749

--- Page 79 ---

General-purpose I/Os (GPIO)
RM0090

### 8.3.3 I/O port control registers

Each of the GPIOs has four 32-bit memory-mapped control registers (GPIOx_MODER, GPIOx_OTYPER, GPIOx_OSPEEDR, GPIOx_PUPDR) to configure up to 16 I/Os.

The GPIOx_MODER register is used to select the I/O direction (input, output, AF, analog). The GPIOx_OTYPER and GPIOx_OSPEEDR registers are used to select the output type (push-pull or open-drain) and speed (the I/O speed pins are directly connected to the corresponding GPIOx_OSPEEDR register bits whatever the I/O direction). The GPIOx_PUPDR register is used to select the pull-up/pull-down whatever the I/O direction.

### 8.3.4 I/O port data registers

Each GPIO has two 16-bit memory-mapped data registers: input and output data registers (GPIOx_IDR and GPIOx_ODR). GPIOx_ODR stores the data to be output, it is read/write accessible. The data input through the I/O are stored into the input data register (GPIOx_IDR), a read-only register.

See Section 8.4.5: GPIO port input data register (GPIOx_IDR) (x = A..I/J/K) and Section 8.4.6: GPIO port output data register (GPIOx_ODR) (x = A..I/J/K) for the register descriptions.

### 8.3.5 I/O data bitwise handling

The bit set reset register (GPIOx_BSRR) is a 32-bit register which allows the application to set and reset each individual bit in the output data register (GPIOx_ODR). The bit set reset register has twice the size of GPIOx_ODR.

To each bit in GPIOx_ODR, correspond two control bits in GPIOx_BSRR: BSRR(i) and BSRR(i+SIZE). When written to 1, bit BSRR(i) sets the corresponding ODR(i) bit. When written to 1, bit BSRR(i+SIZE) resets the ODR(i) corresponding bit.

Writing any bit to 0 in GPIOx_BSRR does not have any effect on the corresponding bit in GPIOx_ODR. If there is an attempt to both set and reset a bit in GPIOx_BSRR, the set action takes priority.

Using the GPIOx_BSRR register to change the values of individual bits in GPIOx_ODR is a "one-shot" effect that does not lock the GPIOx_ODR bits. The GPIOx_ODR bits can always be accessed directly. The GPIOx_BSRR register provides a way of performing atomic bitwise handling.

There is no need for the software to disable interrupts when programming the GPIOx_ODR at bit level: it is possible to modify one or more bits in a single atomic AHB1 write access.

### 8.3.6 GPIO locking mechanism

It is possible to freeze the GPIO control registers by applying a specific write sequence to the GPIOx_LCKR register. The frozen registers are GPIOx_MODER, GPIOx_OTYPER, GPIOx_OSPEEDR, GPIOx_PUPDR, GPIOx_AFRH and GPIOx_AFRH.

To write the GPIOx_LCKR register, a specific write / read sequence has to be applied. When the right LOCK sequence is applied to bit 16 in this register, the value of LCKR[15:0] is used to lock the configuration of the I/Os (during the write sequence the LCKR[15:0] value must be the same). When the LOCK sequence has been applied to a port bit, the value of the port bit can no longer be modified until the next MCU or peripheral reset. Each GPIOx_LCKR bit

274/1749
RM0090 Rev 18
ST

--- Page 80 ---

RM0090
General-purpose I/Os (GPIO)

freezes the corresponding bit in the control registers (GPIOx_MODER, GPIOx_OTYPER, GPIOx_OSPEEDR, GPIOx_PUPDR, GPIOx_AFRL and GPIOx_AFRH).

The LOCK sequence (refer to Section 8.4.8: GPIO port configuration lock register (GPIOx_LCKR) (x = A..I/J/K)) can only be performed using a word (32-bit long) access to the GPIOx_LCKR register due to the fact that GPIOx_LCKR bit 16 has to be set at the same time as the [15:0] bits.

For more details please refer to LCKR register description in Section 8.4.8: GPIO port configuration lock register (GPIOx_LCKR) (x = A..I/J/K).

### 8.3.7 I/O alternate function input/output

Two registers are provided to select one out of the sixteen alternate function inputs/outputs available for each I/O. With these registers, you can connect an alternate function to some other pin as required by your application.

This means that a number of possible peripheral functions are multiplexed on each GPIO using the GPIOx_AFRL and GPIOx_AFRH alternate function registers. The application can thus select any one of the possible functions for each I/O. The AF selection signal being common to the alternate function input and alternate function output, a single channel is selected for the alternate function input/output of one I/O.

To know which functions are multiplexed on each GPIO pin, refer to the datasheets.

**Note:** The application is allowed to select one of the possible peripheral functions for each I/O at a time.

### 8.3.8 External interrupt/wakeup lines

All ports have external interrupt capability. To use external interrupt lines, the port must be configured in input mode, refer to Section 12.2: External interrupt/event controller (EXTI) and Section 12.2.3: Wakeup event management.

### 8.3.9 Input configuration

When the I/O port is programmed as Input:

*   the output buffer is disabled
*   the Schmitt trigger input is activated
*   the pull-up and pull-down resistors are activated depending on the value in the GPIOx_PUPDR register
*   The data present on the I/O pin are sampled into the input data register every AHB1 clock cycle
*   A read access to the input data register provides the I/O State

Figure 28 shows the input configuration of the I/O port bit.

ST
RM0090 Rev 18
275/1749

--- Page 81 ---

General-purpose I/Os (GPIO) RM0090

Figure 28. Input floating/pull up/pull down configurations

8.3.10 Output configuration

When the I/O port is programmed as output:

* The output buffer is enabled:
    * Open drain mode: A “0” in the Output register activates the N-MOS whereas a “1” in the Output register leaves the port in Hi-Z (the P-MOS is never activated)
    * Push-pull mode: A “0” in the Output register activates the N-MOS whereas a “1” in the Output register activates the P-MOS
* The Schmitt trigger input is activated
* The weak pull-up and pull-down resistors are activated or not depending on the value in the GPIOx_PUPDR register
* The data present on the I/O pin are sampled into the input data register every AHB1 clock cycle
* A read access to the input data register gets the I/O state
* A read access to the output data register gets the last written value

Figure 29 shows the output configuration of the I/O port bit.

276/1749 RM0090 Rev 18

--- Page 82 ---

RM0090
General-purpose I/Os (GPIO)

**Figure 29. Output configuration**

Read
Write
Read/write
Bit set/reset registers
Output data register
Input data register
Input driver
TTL Schmitt trigger
on
Output driver
Output control
VDD
P-MOS
VSS
N-MOS
Push-pull or Open-drain
VDD
on/off
pull up
VSS
pull down
VDD
protection diode
I/O pin
VSS
protection diode
ai15941b

**8.3.11 Alternate function configuration**

When the I/O port is programmed as alternate function:
* The output buffer can be configured as open-drain or push-pull
* The output buffer is driven by the signal coming from the peripheral (transmitter enable and data)
* The Schmitt trigger input is activated
* The weak pull-up and pull-down resistors are activated or not depending on the value in the GPIOx_PUPDR register
* The data present on the I/O pin are sampled into the input data register every AHB1 clock cycle
* A read access to the input data register gets the I/O state

*Figure 30* shows the Alternate function configuration of the I/O port bit.

**Figure 30. Alternate function configuration**

To on-chip peripheral
Alternate function input
Read
Write
Read/write
From on-chip peripheral
Bit set/reset registers
Output data register
Input data register
Input driver
TTL Schmitt trigger
on
Output driver
Output control
VDD
P-MOS
VSS
N-MOS
push-pull or open-drain
VDD
on/off
pull up
VSS
pull down
VDD
protection diode
I/O pin
VSS
protection diode
ai15942b

ST
RM0090 Rev 18
277/1749

--- Page 83 ---

General-purpose I/Os (GPIO)

RM0090

### 8.3.12 Analog configuration

When the I/O port is programmed as analog configuration:

*   The output buffer is disabled
*   The Schmitt trigger input is deactivated, providing zero consumption for every analog value of the I/O pin. The output of the Schmitt trigger is forced to a constant value (0).
*   The weak pull-up and pull-down resistors are disabled
*   Read access to the input data register gets the value "0"

**Note:** *In the analog configuration, the I/O pins cannot be 5 Volt tolerant.*

*Figure 31* shows the high-impedance, analog-input configuration of the I/O port bit.

**Figure 31. High impedance-analog configuration**

[Diagram: High impedance-analog configuration]
*   To on-chip peripheral
*   Analog
*   Read
*   Write
*   Bit set/reset registers
*   Input data register
*   Output data register
*   Read/write
*   From on-chip peripheral
*   Analog
*   Input driver
*   TTL Schmitt trigger
*   off
*   0
*   VDD
*   protection diode
*   I/O pin
*   protection diode
*   VSS
*   ai15943

### 8.3.13 Using the OSC32_IN/OSC32_OUT pins as GPIO PC14/PC15 port pins

The LSE oscillator pins OSC32_IN and OSC32_OUT can be used as general-purpose PC14 and PC15 I/Os, respectively, when the LSE oscillator is off. The PC14 and PC15 I/Os are only configured as LSE oscillator pins OSC32_IN and OSC32_OUT when the LSE oscillator is ON. This is done by setting the LSEON bit in the RCC_BDCR register. The LSE has priority over the GPIO function.

**Note:** *The PC14/PC15 GPIO functionality is lost when the 1.2 V domain is powered off (by the device entering the standby mode) or when the backup domain is supplied by VBAT (VDD no more supplied). In this case the I/Os are set in analog input mode.*

### 8.3.14 Using the OSC_IN/OSC_OUT pins as GPIO PH0/PH1 port pins

The HSE oscillator pins OSC_IN/OSC_OUT can be used as general-purpose PH0/PH1 I/Os, respectively, when the HSE oscillator is OFF. (after reset, the HSE oscillator is off). The PH0/PH1 I/Os are only configured as OSC_IN/OSC_OUT HSE oscillator pins when the HSE oscillator is ON. This is done by setting the HSEON bit in the RCC_CR register. The HSE has priority over the GPIO function.

278/1749
RM0090 Rev 18
ST

--- Page 84 ---

RM0090
General-purpose I/Os (GPIO)

### 8.3.15 Selection of RTC_AF1 and RTC_AF2 alternate functions

The STM32F4xx feature two GPIO pins RTC_AF1 and RTC_AF2 that can be used for the detection of a tamper or time stamp event, or RTC_ALARM, or RTC_CALIB RTC outputs.

*   The RTC_AF1 (PC13) can be used for the following purposes:
    *   RTC_ALARM output: this output can be RTC Alarm A, RTC Alarm B or RTC Wakeup depending on the OSEL[1:0] bits in the RTC_CR register
    *   RTC_CALIB output: this feature is enabled by setting the COE[23] in the RTC_CR register
    *   RTC_TAMP1: tamper event detection
    *   RTC_TS: time stamp event detection
*   The RTC_AF2 (PI8) can be used for the following purposes:
    *   RTC_TAMP1: tamper event detection
    *   RTC_TAMP2: tamper event detection
    *   RTC_TS: time stamp event detection

The selection of the corresponding pin is performed through the RTC_TAFCR register as follows:

*   TAMP1INSEL is used to select which pin is used as the RTC_TAMP1 tamper input
*   TSINSEL is used to select which pin is used as the RTC_TS time stamp input
*   ALARMOUTTYPE is used to select whether the RTC_ALARM is output in push-pull or open-drain mode

The output mechanism follows the priority order listed in Table 37 and Table 38.

**Table 37. RTC_AF1 pin(1)**

| Pin configuration and function | RTC_ALARM enabled | RTC_CALIB enabled | Tamper enabled | Time stamp enabled | TAMP1INSEL TAMPER1 pin selection | TSINSEL TIMESTAMP pin selection | ALARMOUTTYPE RTC_ALARM configuration |
|---|---|---|---|---|---|---|---|
| Alarm out output OD | 1 | Don't care | 0 |  |  |  |  |
| Alarm out output PP | 1 | Don't care | 1 |  |  |  |  |
| Calibration out output PP | 0 | 1 | Don't care |  |  |  |  |
| TAMPER1 input floating | 0 | 1 | 0 | Don't care |  |  |  |
| TIMESTAMP and TAMPER1 input floating | 0 | 1 | 0 | Don't care |  |  |  |
| TIMESTAMP input floating | 0 | 1 | Don't care | 0 | Don't care |  |  |
| Standard GPIO | 0 | Don't care |  |  |  |  |  |


1. OD: open drain; PP: push-pull.

ST
RM0090 Rev 18
279/1749

--- Page 85 ---

General-purpose I/Os (GPIO)
RM0090

Table 38. RTC_AF2 pin

| Pin configuration and function | Tamper enabled | Time stamp enabled | TAMP1INSEL TAMPER1 pin selection | TSINSEL TIMESTAMP pin selection | ALARMOUTTYPE RTC_ALARM configuration |
|---|---|---|---|---|---|
| TAMPER1 input floating | 1 | 0 | 1 | Don't care |  |
| TIMESTAMP and TAMPER1 input floating | 1 | Don't care |  |  |  |
| TIMESTAMP input floating | 0 | 1 | Don't care | 1 | Don't care |
| Standard GPIO | 0 | Don't care |  |  |  |


280/1749
RM0090 Rev 18
ST

--- Page 86 ---

RM0090
General-purpose I/Os (GPIO)


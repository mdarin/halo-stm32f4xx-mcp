## 2 Description

The STM32F405xx and STM32F407xx family is based on the high-performance Arm® Cortex®-M4 32-bit RISC core operating at a frequency of up to 168 MHz. The Cortex®-M4 core features a floating-point unit (FPU) single precision which supports all Arm single-precision data-processing instructions and data types. It also implements a full set of DSP instructions and a memory protection unit (MPU) which enhances application security.

The STM32F405xx and STM32F407xx family incorporates high-speed embedded memories (flash memory up to 1 Mbyte, up to 192 Kbytes of SRAM), up to 4 Kbytes of backup SRAM, and an extensive range of enhanced I/Os and peripherals connected to two APB buses, three AHB buses and a 32-bit multi-AHB bus matrix.

All devices offer three 12-bit ADCs, two DACs, a low-power RTC, 12 general-purpose 16-bit timers including two PWM timers for motor control, two general-purpose 32-bit timers, a true random number generator (RNG). They also feature standard and advanced communication interfaces.

*   Up to three I²Cs
*   Three SPIs, two I²Ss full duplex. To achieve audio class accuracy, the I2S peripherals can be clocked via a dedicated internal audio PLL or via an external clock to allow synchronization
*   Four USARTs plus two UARTs
*   A USB OTG full speed and a USB OTG high speed with full-speed capability (with the ULPI)
*   Two CANs
*   An SDIO/MMC interface
*   Ethernet and the camera interface available on STM32F407xx devices only

New advanced peripherals include an SDIO, an enhanced flexible static memory control (FSMC) interface (for devices offered in packages of 100 pins and more), a camera interface for CMOS sensors. Refer to Table 2: STM32F405xx and STM32F407xx: features and peripheral counts for the list of peripherals available on each part number.

The STM32F405xx and STM32F407xx family operates in the –40 to +105 °C temperature range from a 1.8 to 3.6 V power supply. The supply voltage can drop to 1.7 V when the device operates in the 0 to 70 °C temperature range using an external power supply supervisor: refer to Section 3.15.2: Internal reset OFF. A comprehensive set of power-saving mode allows the design of low-power applications.

The STM32F405xx and STM32F407xx family offers devices in various packages ranging from 64 pins to 176 pins. The set of included peripherals changes with the device chosen.

These features make the STM32F405xx and STM32F407xx microcontroller family suitable for a wide range of applications:

*   Motor drive and application control
*   Medical equipment
*   Industrial applications: PLC, inverters, circuit breakers
*   Printers, and scanners
*   Alarm systems, video intercom, and HVAC
*   Home audio appliances

14/206
DS8626 Rev 12
ST

--- Page 15 ---

Figure 5 shows the general block diagram of the device family.

**Table 2. STM32F405xx and STM32F407xx: features and peripheral counts**

| Peripherals | STM32F405RG | STM32F405OG | STM32F405VG | STM32F405ZG | STM32F405OE | STM32F407Vx | STM32F407Zx | STM32F407Ix |
|---|---|---|---|---|---|---|---|---|
| Flash memory in Kbytes | 1024 | 512 | 1024 | 512 | 1024 | 512 | 1024 |  |
| SRAM in Kbytes | System | 192(112+16+64) |  |  |  |  |  |  |
| Backup | 4 |  |  |  |  |  |  |  |
| FSMC memory controller | No | Yes(1) |  |  |  |  |  |  |
| Ethernet | No | Yes |  |  |  |  |  |  |
| Timers | General-purpose | 10 |  |  |  |  |  |  |
| Advanced-control | 2 |  |  |  |  |  |  |  |
| Basic | 2 |  |  |  |  |  |  |  |
| IWDG | Yes |  |  |  |  |  |  |  |
| WWDG | Yes |  |  |  |  |  |  |  |
| RTC | Yes |  |  |  |  |  |  |  |
| True random number generator | Yes |  |  |  |  |  |  |  |


--- Page 16 ---

16/206
DS8626 Rev 12

**Table 2. STM32F405xx and STM32F407xx: features and peripheral counts (continued)**

| Peripherals | STM32F405RG | STM32F405OG | STM32F405VG | STM32F405ZG | STM32F405OE | STM32F407Vx | STM32F407Zx | STM32F407Ix | Description |
|---|---|---|---|---|---|---|---|---|---|
| Communication interfaces | SPI / I2S 3/2 (full duplex)(2) |  |  |  |  |  |  |  |  |
| I2C 3 |  |  |  |  |  |  |  |  |  |
| USART/ UART 4/2 |  |  |  |  |  |  |  |  |  |
| USB OTG FS Yes |  |  |  |  |  |  |  |  |  |
| USB OTG HS Yes |  |  |  |  |  |  |  |  |  |
| CAN 2 |  |  |  |  |  |  |  |  |  |
| SDIO Yes |  |  |  |  |  |  |  |  |  |
| Camera interface | No | Yes |  |  |  |  |  |  |  |
| GPIOs | 51 | 72 | 82 | 114 | 72 | 82 | 114 | 140 |  |
| 12-bit ADC | 3 |  |  |  |  |  |  |  |  |
| Number of channels | 16 | 13 | 16 | 24 | 13 | 16 | 24 |  |  |
| 12-bit DAC | Yes |  |  |  |  |  |  |  |  |
| Number of channels | 2 |  |  |  |  |  |  |  |  |
| Maximum CPU frequency | 168 MHz |  |  |  |  |  |  |  |  |
| Operating voltage | 1.8 to 3.6 V(3) |  |  |  |  |  |  |  |  |
| Operating temperatures | Ambient temperatures: –40 to +85 °C / –40 to +105 °C |  |  |  |  |  |  |  |  |
| Junction temperature: –40 to + 125 °C |  |  |  |  |  |  |  |  |  |
| Package | LQFP64 | WLCSP90 | LQFP100 | LQFP144 | WLCSP90 | LQFP100 | LQFP144 | UFBGA176 LQFP176 |  |


1. For the LQFP100 and WLCSP90 packages, only FSMC Bank1 or Bank2 are available. Bank1 can only support a multiplexed NOR/PSRAM memory using the NE1 Chip Select. Bank2 can only support a 16- or 8-bit NAND flash memory using the NCE2 Chip Select. The interrupt line cannot be used since Port G is not available in this package.
2. The SPI2 and SPI3 interfaces give the flexibility to work in an exclusive way in either the SPI mode or the I<sup>2</sup>S audio mode.
3. V<sub>DD</sub>/V<sub>DDA</sub> minimum value of 1.7 V is obtained when the device operates in reduced temperature range, and with the use of an external power supply supervisor (refer to Section 3.15.2: Internal reset OFF).

STM32F-405xx, STM32F-407xx

--- Page 17 ---

STM32F405xx, STM32F407xx
Description

## 2.1 Full compatibility throughout the family

The STM32F405xx and STM32F407xx are part of the STM32F4 family. They are fully pin-to-pin, software and feature compatible with the STM32F2xx devices, allowing the user to try different memory densities, peripherals, and performances (FPU, higher frequency) for a greater degree of freedom during the development cycle.

The STM32F405xx and STM32F407xx devices maintain a close compatibility with the whole STM32F10xxx family. All functional pins are pin-to-pin compatible. The STM32F405xx and STM32F407xx, however, are not drop-in replacements for the STM32F10xxx devices: the two families do not have the same power scheme, and so their power pins are different. Nonetheless, transition from the STM32F10xxx to the STM32F40xxx family remains simple as only a few pins are impacted.

*Figure 4, Figure 3, Figure 2, and Figure 1* give compatible board designs between the STM32F40xxx, STM32F2, and STM32F10xxx families.

**Figure 1. Compatible board design between STM32F10xx/STM32F40xxx for LQFP64**

Vss
48
49 47
33
32
31
Vss
Vss
0 Ω resistor or soldering bridge present for the STM32F10xx configuration, not present in the STM32F4xx configuration
64
1
17
16
ai18489

ST
DS8626 Rev 12
17/206

--- Page 18 ---

**Description**
STM32F405xx, STM32F407xx

**Figure 2. Compatible board design STM32F10xx/STM32F2/STM32F40xxx for LQFP100 package**

75 VSS 51
76 73 50
49 VSS
VSS
0 Ω resistor or soldering bridge present for the STM32F10xxx configuration, not present in the STM32F4xx configuration
99 (VSS)
100 1 19 20 26
VSS VDD VSS
Two 0 Ω resistors connected to:
- VSS for the STM32F10xx
- VSS for the STM32F4xx
- VSS or NC for the STM32F2xx
VSS for STM32F10xx
VDD for STM32F4xx
ai18488d

**Figure 3. Compatible board design between STM32F10xx/STM32F2/STM32F40xxx for LQFP144 package**

108 VSS 73
109 106 72
71
0 Ω resistor or soldering bridge present for the STM32F10xxx configuration, not present in the STM32F4xx configuration
VSS
Not populated when 0 Ω resistor or soldering bridge present
Signal from external power supply supervisor
143 (PDR_ON)
144 1 30 31 37
VDD VSS VSS
Two 0 Ω resistors connected to:
- VSS for the STM32F10xx
- VSS, VDD or NC for the STM32F2xx
- VDD or signal from external power supply supervisor for the STM32F4xx
VDD VSS
Not populated for STM32F10xx
VSS for STM32F10xx
VDD for STM32F4xx
ai18487d

18/206
DS8626 Rev 12
ST

--- Page 19 ---

STM32F405xx, STM32F407xx
Description

Figure 4. Compatible board design between STM32F2 and STM32F40xxx
for LQFP176 and BGA176 packages

132
133
89
88
Signal from external
power supply
supervisor
171 (PDR_ON)
176
1
45
44
VDD VSS
Two 0 Ω resistors connected to:
- VSS, VDD or NC for the STM32F2xx
- VDD or signal from external power supply supervisor for the STM32F4xx
MS19919V3

ST
DS8626 Rev 12
19/206

--- Page 20 ---

# Functional overview

STM32F405xx, STM32F407xx


## 8.4 GPIO registers

This section gives a detailed description of the GPIO registers.
For a summary of register bits, register address offsets and reset values, refer to Table 39.
The GPIO registers can be accessed by byte (8 bits), half-words (16 bits) or words (32 bits).

### 8.4.1 GPIO port mode register (GPIOx_MODER) (x = A..I/J/K)

Address offset: 0x00
Reset values:
* 0xA800 0000 for port A
* 0x0000 0280 for port B
* 0x0000 0000 for other ports

| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| MODER15[1:0] | MODER14[1:0] | MODER13[1:0] | MODER12[1:0] | MODER11[1:0] | MODER10[1:0] | MODER9[1:0] | MODER8[1:0] |  |  |  |  |  |  |  |  |
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
| MODER7[1:0] | MODER6[1:0] | MODER5[1:0] | MODER4[1:0] | MODER3[1:0] | MODER2[1:0] | MODER1[1:0] | MODER0[1:0] |  |  |  |  |  |  |  |  |
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bits 2y:2y+1 **MODERy[1:0]**: Port x configuration bits (y = 0..15)
These bits are written by software to configure the I/O direction mode.
00: Input (reset state)
01: General purpose output mode
10: Alternate function mode
11: Analog mode

### 8.4.2 GPIO port output type register (GPIOx_OTYPER) (x = A..I/J/K)

Address offset: 0x04
Reset value: 0x0000 0000

| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
| OT15 | OT14 | OT13 | OT12 | OT11 | OT10 | OT9 | OT8 | OT7 | OT6 | OT5 | OT4 | OT3 | OT2 | OT1 | OT0 |
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bits 31:16 Reserved, must be kept at reset value.
Bits 15:0 **OTy**: Port x configuration bits (y = 0..15)
These bits are written by software to configure the output type of the I/O port.
0: Output push-pull (reset state)
1: Output open-drain

STI
RM0090 Rev 18
281/1749

--- Page 87 ---

General-purpose I/Os (GPIO)
RM0090

### 8.4.3 GPIO port output speed register (GPIOx_OSPEEDR) (x = A..I/J/K)

Address offset: 0x08

Reset values:
*   0x0C00 0000 for port A
*   0x0000 00C0 for port B
*   0x0000 0000 for other ports

| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| OSPEEDR15 [1:0] | OSPEEDR14 [1:0] | OSPEEDR13 [1:0] | OSPEEDR12 [1:0] | OSPEEDR11 [1:0] | OSPEEDR10 [1:0] | OSPEEDR9 [1:0] | OSPEEDR8 [1:0] |  |  |  |  |  |  |  |  |
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
| OSPEEDR7[1:0] | OSPEEDR6[1:0] | OSPEEDR5[1:0] | OSPEEDR4[1:0] | OSPEEDR3[1:0] | OSPEEDR2[1:0] | OSPEEDR1 [1:0] | OSPEEDR0 1:0] |  |  |  |  |  |  |  |  |
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bits 2y:2y+1 **OSPEEDRy[1:0]**: Port x configuration bits (y = 0..15)
These bits are written by software to configure the I/O output speed.
00: Low speed
01: Medium speed
10: High speed
11: Very high speed
*Note: Refer to the product datasheets for the values of OSPEEDRy bits versus VDD range and external load.*

### 8.4.4 GPIO port pull-up/pull-down register (GPIOx_PUPDR) (x = A..I/J/K)

Address offset: 0x0C

Reset values:
*   0x6400 0000 for port A
*   0x0000 0100 for port B
*   0x0000 0000 for other ports

| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| PUPDR15[1:0] | PUPDR14[1:0] | PUPDR13[1:0] | PUPDR12[1:0] | PUPDR11[1:0] | PUPDR10[1:0] | PUPDR9[1:0] | PUPDR8[1:0] |  |  |  |  |  |  |  |  |
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
| PUPDR7[1:0] | PUPDR6[1:0] | PUPDR5[1:0] | PUPDR4[1:0] | PUPDR3[1:0] | PUPDR2[1:0] | PUPDR1[1:0] | PUPDR0[1:0] |  |  |  |  |  |  |  |  |
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


282/1749
RM0090 Rev 18
ST

--- Page 88 ---

RM0090
General-purpose I/Os (GPIO)

Bits 2y:2y+1 **PUPDRy[1:0]**: Port x configuration bits (y = 0..15)
These bits are written by software to configure the I/O pull-up or pull-down
00: No pull-up, pull-down
01: Pull-up
10: Pull-down
11: Reserved

### 8.4.5 GPIO port input data register (GPIOx_IDR) (x = A..I/J/K)

Address offset: 0x10
Reset value: 0x0000 XXXX (where X means undefined)

| Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 |
| 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
| IDR15 | IDR14 | IDR13 | IDR12 | IDR11 | IDR10 | IDR9 | IDR8 | IDR7 | IDR6 | IDR5 | IDR4 | IDR3 | IDR2 | IDR1 | IDR0 |
| r |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bits 31:16 Reserved, must be kept at reset value.
Bits 15:0 **IDRy**: Port input data (y = 0..15)
These bits are read-only and can be accessed in word mode only. They contain the input value of the corresponding I/O port.

### 8.4.6 GPIO port output data register (GPIOx_ODR) (x = A..I/J/K)

Address offset: 0x14
Reset value: 0x0000 0000

| Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 |
| 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
| ODR15 | ODR14 | ODR13 | ODR12 | ODR11 | ODR10 | ODR9 | ODR8 | ODR7 | ODR6 | ODR5 | ODR4 | ODR3 | ODR2 | ODR1 | ODR0 |
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bits 31:16 Reserved, must be kept at reset value.
Bits 15:0 **ODRy**: Port output data (y = 0..15)
These bits can be read and written by software.
*Note: For atomic bit set/reset, the ODR bits can be individually set and reset by writing to the GPIOx_BSR register (x = A..I/J/K).*

STI
RM0090 Rev 18
283/1749

--- Page 89 ---

General-purpose I/Os (GPIO)
RM0090

### 8.4.7 GPIO port bit set/reset register (GPIOx_BSRR) (x = A..I/J/K)

Address offset: 0x18
Reset value: 0x0000 0000

| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| BR15 | BR14 | BR13 | BR12 | BR11 | BR10 | BR9 | BR8 | BR7 | BR6 | BR5 | BR4 | BR3 | BR2 | BR1 | BR0 |
| w |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
| BS15 | BS14 | BS13 | BS12 | BS11 | BS10 | BS9 | BS8 | BS7 | BS6 | BS5 | BS4 | BS3 | BS2 | BS1 | BS0 |
| w |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bits 31:16 **BRy**: Port x reset bit y (y = 0..15)
These bits are write-only and can be accessed in word, half-word or byte mode. A read to these bits returns the value 0x0000.
0: No action on the corresponding ODRx bit
1: Resets the corresponding ODRx bit
*Note: If both BSx and BRx are set, BSx has priority.*

Bits 15:0 **BSy**: Port x set bit y (y = 0..15)
These bits are write-only and can be accessed in word, half-word or byte mode. A read to these bits returns the value 0x0000.
0: No action on the corresponding ODRx bit
1: Sets the corresponding ODRx bit

### 8.4.8 GPIO port configuration lock register (GPIOx_LCKR) (x = A..I/J/K)

This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits [15:0] is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR[15:0] must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU or peripheral reset.

*Note:* A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this write sequence.

Each lock bit freezes a specific configuration register (control and alternate function registers).

Address offset: 0x1C
Reset value: 0x0000 0000
Access: 32-bit word only, read/write register

| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved | LCKK |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | rw |
| 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
| LCK15 | LCK14 | LCK13 | LCK12 | LCK11 | LCK10 | LCK9 | LCK8 | LCK7 | LCK6 | LCK5 | LCK4 | LCK3 | LCK2 | LCK1 | LCK0 |
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


284/1749
RM0090 Rev 18
STI

--- Page 90 ---

RM0090
General-purpose I/Os (GPIO)

Bits 31:17 Reserved, must be kept at reset value.

Bit 16 **LCKK[16]**: Lock key
This bit can be read any time. It can only be modified using the lock key write sequence.
0: Port configuration lock key not active
1: Port configuration lock key active. The GPIOx_LCKR register is locked until an MCU reset or a peripheral reset occurs.

LOCK key write sequence:
WR LCKR[16] = '1' + LCKR[15:0]
WR LCKR[16] = '0' + LCKR[15:0]
WR LCKR[16] = '1' + LCKR[15:0]
RD LCKR
RD LCKR[16] = '1' (this read operation is optional but it confirms that the lock is active)
*Note: During the LOCK key write sequence, the value of LCK[15:0] must not change.*
*Any error in the lock sequence aborts the lock.*
*After the first lock sequence on any bit of the port, any read access on the LCKK bit will return '1' until the next CPU reset.*

Bits 15:0 **LCKy**: Port x lock bit y (y = 0..15)
These bits are read/write but can only be written when the LCKK bit is '0'.
0: Port configuration not locked
1: Port configuration locked

### 8.4.9 GPIO alternate function low register (GPIOx_AFRL) (x = A..I/J/K)

Address offset: 0x20
Reset value: 0x0000 0000

| Bits 31:16 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 |
| AFRL7[3:0] | AFRL6[3:0] | AFRL5[3:0] | AFRL4[3:0] |  |  |  |  |  |  |  |  |  |  |  |  |
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Bits 15:0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
| AFRL3[3:0] | AFRL2[3:0] | AFRL1[3:0] | AFRL0[3:0] |  |  |  |  |  |  |  |  |  |  |  |  |
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bits 31:0 **AFRLy**: Alternate function selection for port x bit y (y = 0..7)
These bits are written by software to configure alternate function I/Os

AFRLy selection:
0000: AF0
0001: AF1
0010: AF2
0011: AF3
0100: AF4
0101: AF5
0110: AF6
0111: AF7
1000: AF8
1001: AF9
1010: AF10
1011: AF11
1100: AF12
1101: AF13
1110: AF14
1111: AF15

ST
RM0090 Rev 18
285/1749

--- Page 91 ---

General-purpose I/Os (GPIO) RM0090

### 8.4.10 GPIO alternate function high register (GPIOx_AFRH) (x = A..I/J)

Address offset: 0x24
Reset value: 0x0000 0000

| 31 | 27 | 23 | 19 |
|---|---|---|---|
| AFRH15[3:0] | AFRH14[3:0] | AFRH13[3:0] | AFRH12[3:0] |
| rw |  |  |  |
| 15 | 11 | 7 | 3 |
| AFRH11[3:0] | AFRH10[3:0] | AFRH9[3:0] | AFRH8[3:0] |
| rw |  |  |  |


Bits 31:0 **AFRHy**: Alternate function selection for port x bit y (y = 8..15)
These bits are written by software to configure alternate function I/Os

AFRHy selection:
0000: AF0
0001: AF1
0010: AF2
0011: AF3
0100: AF4
0101: AF5
0110: AF6
0111: AF7
1000: AF8
1001: AF9
1010: AF10
1011: AF11
1100: AF12
1101: AF13
1110: AF14
1111: AF15

286/1749 RM0090 Rev 18 ST

--- Page 92 ---

RM0090
General-purpose I/Os (GPIO)

### 8.4.11 GPIO register map

The following table gives the GPIO register map and the reset values.

**Table 39. GPIO register map and reset values**

| Offset | Register | 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 0x00 | GPIOA_MODER | MODER15[1:0] | MODER14[1:0] | MODER13[1:0] | MODER12[1:0] | MODER11[1:0] | MODER10[1:0] | MODER9[1:0] | MODER8[1:0] | MODER7[1:0] | MODER6[1:0] | MODER5[1:0] | MODER4[1:0] | MODER3[1:0] | MODER2[1:0] | MODER1[1:0] | MODER0[1:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value 1 0 1 0 1 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x00 | GPIOB_MODER | MODER15[1:0] | MODER14[1:0] | MODER13[1:0] | MODER12[1:0] | MODER11[1:0] | MODER10[1:0] | MODER9[1:0] | MODER8[1:0] | MODER7[1:0] | MODER6[1:0] | MODER5[1:0] | MODER4[1:0] | MODER3[1:0] | MODER2[1:0] | MODER1[1:0] | MODER0[1:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 1 0 1 0 0 0 0 0 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x00 | GPIOx_MODER (where x = C..I/J/K) | MODER15[1:0] | MODER14[1:0] | MODER13[1:0] | MODER12[1:0] | MODER11[1:0] | MODER10[1:0] | MODER9[1:0] | MODER8[1:0] | MODER7[1:0] | MODER6[1:0] | MODER5[1:0] | MODER4[1:0] | MODER3[1:0] | MODER2[1:0] | MODER1[1:0] | MODER0[1:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x04 | GPIOx_OTYPER (where x = A..I/J/K) | Reserved | OT15 | OT14 | OT13 | OT12 | OT11 | OT10 | OT9 | OT8 | OT7 | OT6 | OT5 | OT4 | OT3 | OT2 | OT1 | OT0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x08 | GPIOx_OSPEEDR (where x = A..I/J/K except B) | OSPEEDR15[1:0] | OSPEEDR14[1:0] | OSPEEDR13[1:0] | OSPEEDR12[1:0] | OSPEEDR11[1:0] | OSPEEDR10[1:0] | OSPEEDR9[1:0] | OSPEEDR8[1:0] | OSPEEDR7[1:0] | OSPEEDR6[1:0] | OSPEEDR5[1:0] | OSPEEDR4[1:0] | OSPEEDR3[1:0] | OSPEEDR2[1:0] | OSPEEDR1[1:0] | OSPEEDR0[1:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x08 | GPIOB_OSPEEDR | OSPEEDR15[1:0] | OSPEEDR14[1:0] | OSPEEDR13[1:0] | OSPEEDR12[1:0] | OSPEEDR11[1:0] | OSPEEDR10[1:0] | OSPEEDR9[1:0] | OSPEEDR8[1:0] | OSPEEDR7[1:0] | OSPEEDR6[1:0] | OSPEEDR5[1:0] | OSPEEDR4[1:0] | OSPEEDR3[1:0] | OSPEEDR2[1:0] | OSPEEDR1[1:0] | OSPEEDR0[1:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 1 1 0 0 0 0 0 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x0C | GPIOA_PUPDR | PUPDR15[1:0] | PUPDR14[1:0] | PUPDR13[1:0] | PUPDR12[1:0] | PUPDR11[1:0] | PUPDR10[1:0] | PUPDR9[1:0] | PUPDR8[1:0] | PUPDR7[1:0] | PUPDR6[1:0] | PUPDR5[1:0] | PUPDR4[1:0] | PUPDR3[1:0] | PUPDR2[1:0] | PUPDR1[1:0] | PUPDR0[1:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value 0 1 1 0 0 1 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x0C | GPIOB_PUPDR | PUPDR15[1:0] | PUPDR14[1:0] | PUPDR13[1:0] | PUPDR12[1:0] | PUPDR11[1:0] | PUPDR10[1:0] | PUPDR9[1:0] | PUPDR8[1:0] | PUPDR7[1:0] | PUPDR6[1:0] | PUPDR5[1:0] | PUPDR4[1:0] | PUPDR3[1:0] | PUPDR2[1:0] | PUPDR1[1:0] | PUPDR0[1:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 1 0 0 0 0 0 0 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


ST
RM0090 Rev 18
287/1749

--- Page 93 ---

General-purpose I/Os (GPIO)

RM0090

Table 39. GPIO register map and reset values (continued)

| Offset | Register | 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 0x0C | GPIOx_PUPDR(where x = C..I/J/K) | PUPDR5[1:0] | PUPDR4[1:0] | PUPDR3[1:0] | PUPDR2[1:0] | PUPDR1[1:0] | PUPDR0[1:0] | PUPDR9[1:0] | PUPDR8[1:0] | PUPDR7[1:0] | PUPDR6[1:0] | PUPDR5[1:0] | PUPDR4[1:0] | PUPDR3[1:0] | PUPDR2[1:0] | PUPDR1[1:0] | PUPDR0[1:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x10 | GPIOx_IDR(where x = A..I/J/K) | Reserved | IDR15 | IDR14 | IDR13 | IDR12 | IDR11 | IDR10 | IDR9 | IDR8 | IDR7 | IDR6 | IDR5 | IDR4 | IDR3 | IDR2 | IDR1 | IDR0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value x x x x x x x x x x x x x x x x | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x14 | GPIOx_ODR(where x = A..I/J/K) | Reserved | ODR15 | ODR14 | ODR13 | ODR12 | ODR11 | ODR10 | ODR9 | ODR8 | ODR7 | ODR6 | ODR5 | ODR4 | ODR3 | ODR2 | ODR1 | ODR0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x18 | GPIOx_BSRR(where x = A..I/J/K) | BR15 | BR14 | BR13 | BR12 | BR11 | BR10 | BR9 | BR8 | BR7 | BR6 | BR5 | BR4 | BR3 | BR2 | BR1 | BR0 | BS15 | BS14 | BS13 | BS12 | BS11 | BS10 | BS9 | BS8 | BS7 | BS6 | BS5 | BS4 | BS3 | BS2 | BS1 | BS0 |
| Reset value 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x1C | GPIOx_LCKR(where x = A..I/J/K) | Reserved | LCKK | LCK15 | LCK14 | LCK13 | LCK12 | LCK11 | LCK10 | LCK9 | LCK8 | LCK7 | LCK6 | LCK5 | LCK4 | LCK3 | LCK2 | LCK1 | LCK0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x20 | GPIOx_AFRL(where x = A..I/J/K) | AFRL7[3:0] | AFRL6[3:0] | AFRL5[3:0] | AFRL4[3:0] | AFRL3[3:0] | AFRL2[3:0] | AFRL1[3:0] | AFRL0[3:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x24 | GPIOx_AFRH(where x = A..I/J) | AFRH15[3:0] | AFRH14[3:0] | AFRH13[3:0] | AFRH12[3:0] | AFRH11[3:0] | AFRH10[3:0] | AFRH9[3:0] | AFRH8[3:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Refer to Section 2.3: Memory map for the register boundary addresses.

288/1749

RM0090 Rev 18

ST

--- Page 94 ---

RM0090
System configuration controller (SYSCFG)


## 12.3 EXTI registers

Refer to Section 1.1: List of abbreviations for registers for a list of abbreviations used in register descriptions.

### 12.3.1 Interrupt mask register (EXTI_IMR)

Address offset: 0x00
Reset value: 0x0000 0000

| 31 | 23 | 16 |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved | MR22 | MR21 | MR20 | MR19 | MR18 | MR17 | MR16 |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  | rw |  |  |  |  |  |  |
| 15 | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| MR15 | MR14 | MR13 | MR12 | MR11 | MR10 | MR9 | MR8 | MR7 | MR6 | MR5 | MR4 | MR3 | MR2 | MR1 | MR0 |
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bits 31:23 Reserved, must be kept at reset value.
Bits 22:0 **MRx**: Interrupt mask on line x
0: Interrupt request from line x is masked
1: Interrupt request from line x is not masked

### 12.3.2 Event mask register (EXTI_EMR)

Address offset: 0x04
Reset value: 0x0000 0000

| 31 | 23 | 16 |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved | MR22 | MR21 | MR20 | MR19 | MR18 | MR17 | MR16 |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  | rw |  |  |  |  |  |  |
| 15 | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| MR15 | MR14 | MR13 | MR12 | MR11 | MR10 | MR9 | MR8 | MR7 | MR6 | MR5 | MR4 | MR3 | MR2 | MR1 | MR0 |
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bits 31:23 Reserved, must be kept at reset value.
Bits 22:0 **MRx**: Event mask on line x
0: Event request from line x is masked
1: Event request from line x is not masked

384/1749
RM0090 Rev 18
ST

--- Page 190 ---

RM0090
Interrupts and events

### 12.3.3 Rising trigger selection register (EXTI_RTSR)

Address offset: 0x08
Reset value: 0x0000 0000

| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved | TR22 | TR21 | TR20 | TR19 | TR18 | TR17 | TR16 |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  | rw |  |  |  |  |  |  |
| 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
| TR15 | TR14 | TR13 | TR12 | TR11 | TR10 | TR9 | TR8 | TR7 | TR6 | TR5 | TR4 | TR3 | TR2 | TR1 | TR0 |
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bits 31:23 Reserved, must be kept at reset value.
Bits 22:0 **TRx**: Rising trigger event configuration bit of line x
0: Rising trigger disabled (for Event and Interrupt) for input line
1: Rising trigger enabled (for Event and Interrupt) for input line

**Note:** The external wakeup lines are edge triggered, no glitch must be generated on these lines. If a rising edge occurs on the external interrupt line while writing to the EXTI_RTSR register, the pending bit is be set.
Rising and falling edge triggers can be set for the same interrupt line. In this configuration, both generate a trigger condition.

### 12.3.4 Falling trigger selection register (EXTI_FTSR)

Address offset: 0x0C
Reset value: 0x0000 0000

| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved | TR22 | TR21 | TR20 | TR19 | TR18 | TR17 | TR16 |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  | rw |  |  |  |  |  |  |
| 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
| TR15 | TR14 | TR13 | TR12 | TR11 | TR10 | TR9 | TR8 | TR7 | TR6 | TR5 | TR4 | TR3 | TR2 | TR1 | TR0 |
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bits 31:23 Reserved, must be kept at reset value.
Bits 22:0 **TRx**: Falling trigger event configuration bit of line x
0: Falling trigger disabled (for Event and Interrupt) for input line
1: Falling trigger enabled (for Event and Interrupt) for input line.

**Note:** The external wakeup lines are edge triggered, no glitch must be generated on these lines. If a falling edge occurs on the external interrupt line while writing to the EXTI_FTSR register, the pending bit is not set.
Rising and falling edge triggers can be set for the same interrupt line. In this configuration, both generate a trigger condition.

ST
RM0090 Rev 18
385/1749

--- Page 191 ---

**Interrupts and events**
RM0090

### 12.3.5 Software interrupt event register (EXTI_SWIER)

Address offset: 0x10
Reset value: 0x0000 0000

| 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved | SWIER 22 | SWIER 21 | SWIER 20 | SWIER 19 | SWIER 18 | SWIER 17 | SWIER 16 |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | rw |
| 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| SWIER 15 | SWIER 14 | SWIER 13 | SWIER 12 | SWIER 11 | SWIER 10 | SWIER 9 | SWIER 8 | SWIER 7 | SWIER 6 | SWIER 5 | SWIER 4 | SWIER 3 | SWIER 2 | SWIER 1 | SWIER 0 |  |
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bits 31:23 Reserved, must be kept at reset value.
Bits 22:0 **SWIERx**: Software Interrupt on line x
If interrupt are enabled on line x in the EXTI_IMR register, writing '1' to SWIERx bit when it is set at '0' sets the corresponding pending bit in the EXTI_PR register, thus resulting in an interrupt request generation.
This bit is cleared by clearing the corresponding bit in EXTI_PR (by writing a 1 to the bit).

### 12.3.6 Pending register (EXTI_PR)

Address offset: 0x14
Reset value: undefined

| 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved | PR22 | PR21 | PR20 | PR19 | PR18 | PR17 | PR16 |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | rc_w1 |
| 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| PR15 | PR14 | PR13 | PR12 | PR11 | PR10 | PR9 | PR8 | PR7 | PR6 | PR5 | PR4 | PR3 | PR2 | PR1 | PR0 |  |
| rc_w1 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bits 31:23 Reserved, must be kept at reset value.
Bits 22:0 **PRx**: Pending bit
0: No trigger request occurred
1: selected trigger request occurred
This bit is set when the selected edge event arrives on the external interrupt line.
This bit is cleared by programming it to '1'.

386/1749
RM0090 Rev 18
ST

--- Page 192 ---

RM0090
Interrupts and events

## 12.3.7 EXTI register map

*Table 64* gives the EXTI register map and the reset values.

**Table 63. External interrupt/event controller register map and reset values**

| Offset | Register | 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 0x00 | EXTI_IMRReset value | Reserved | MR[22:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x04 | EXTI_EMRReset value | Reserved | MR[22:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x08 | EXTI_RTSRReset value | Reserved | TR[22:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x0C | EXTI_FTSRReset value | Reserved | TR[22:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x10 | EXTI_SWIERReset value | Reserved | SWIER[22:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x14 | EXTI_PRReset value | Reserved | PR[22:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Refer to *Section 2.3: Memory map* for the register boundary addresses.

ST
RM0090 Rev 18
387/1749

--- Page 193 ---

Analog-to-digital converter (ADC)
RM0090


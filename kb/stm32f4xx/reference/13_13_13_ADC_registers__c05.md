> [RM0090 ch.13 §13.13] ADC registers**

### 13.13.16 ADC common control register (ADC_CCR)**

**13.13.16 ADC common control register (ADC_CCR)**

Address offset: 0x04 (this offset address is relative to ADC1 base address + 0x300)

Reset value: 0x0000 0000

31 30 29 28 27 26 25 24

23 22 21 20 19 18 17 16



15 14 13 12 11 10 9 8

7 6 5 4 3 2 1 0



Bits 31:24 Reserved, must be kept at reset value.

Bit 23 TSVREFE: Temperature sensor and V

REFINT enable

This bit is set and cleared by software to enable/disable the temperature sensor and the

V REFINT channel.

0: Temperature sensor and V REFINT

channel disabled

1: Temperature sensor and V REFINT

channel enabled

Note:

On STM32F42x and STM32F43x devices, VBATE must be disabled when TSVREFE is set. If both bits are set, only the VBAT conversion is performed.

Bit 22 VBATE: V BAT enable

BAT channel.

This bit is set and cleared by software to enable/disable the V 0: V BAT channel disabled

1: V BAT channel enabled

Bits 21:18 Reserved, must be kept at reset value.

RM0090 Rev 18 427/1749

432

--- Page 38 ---

**Analog-to-digital converter (ADC) RM0090**

Bits 17:16 ADCPRE: ADC prescaler

Set and cleared by software to select the frequency of the clock to the ADC. The clock is common for all the ADCs.

Note: 00: PCLK2 divided by 2

01: PCLK2 divided by 4

10: PCLK2 divided by 6

11: PCLK2 divided by 8

Bits 15:14 DMA: Direct memory access mode for multi ADC mode

This bit-field is set and cleared by software. Refer to the DMA controller section for more details.

00: DMA mode disabled

01: DMA mode 1 enabled (2 / 3 half-words one by one - 1 then 2 then 3)

10: DMA mode 2 enabled (2 / 3 half-words by pairs - 2&1 then 1&3 then 3&2)

11: DMA mode 3 enabled (2 / 3 bytes by pairs - 2&1 then 1&3 then 3&2)

Bit 13 DDS: DMA disable selection (for multi-ADC mode)

This bit is set and cleared by software.

0: No new DMA request is issued after the last transfer (as configured in the DMA controller). DMA bits are not cleared by hardware, however they must have been cleared and set to the wanted mode by software before new DMA requests can be generated. 1: DMA requests are issued as long as data are converted and DMA = 01, 10 or 11. Bit 12 Reserved, must be kept at reset value.

428/1749 RM0090 Rev 18

--- Page 39 ---

**RM0090 Analog-to-digital converter (ADC)**

Bit 11:8 DELAY: Delay between 2 sampling phases

Set and cleared by software. These bits are used in dual or triple interleaved modes.

0000: 5 * T ADCCLK

0001: 6 * T ADCCLK

0010: 7 * T ADCCLK

...

1111: 20 * T ADCCLK

Bits 7:5 Reserved, must be kept at reset value.

Bits 4:0 MULTI[4:0]: Multi ADC mode selection

These bits are written by software to select the operating mode.

– All the ADCs independent:

00000: Independent mode

– 00001 to 01001: Dual mode, ADC1 and ADC2 working together, ADC3 is independent 00001: Combined regular simultaneous + injected simultaneous mode

00010: Combined regular simultaneous + alternate trigger mode

00011: Reserved

00101: Injected simultaneous mode only

00110: Regular simultaneous mode only

00111: interleaved mode only

01001: Alternate trigger mode only

– 10001 to 11001: Triple mode: ADC1, 2 and 3 working together

10001: Combined regular simultaneous + injected simultaneous mode

10010: Combined regular simultaneous + alternate trigger mode

10011: Reserved

10101: Injected simultaneous mode only

10110: Regular simultaneous mode only

10111: interleaved mode only

11001: Alternate trigger mode only

All other combinations are reserved and must not be programmed

Note: In multi mode, a change of channel configuration generates an abort that can cause a

loss of synchronization. It is recommended to disable the multi ADC mode before any configuration change.

RM0090 Rev 18 429/1749

432

--- Page 40 ---

**Analog-to-digital converter (ADC)**

**RM0090**

> [RM0090 ch.13 §13.13] ADC registers**

### 13.13.17 ADC common regular data register for dual and triple modes (ADC_CDR)**

**13.13.17 ADC common regular data register for dual and triple modes (ADC_CDR)**

Address offset: 0x08 (this offset address is relative to ADC1 base address + 0x300)

Reset value: 0x0000 0000

31 30 29 28 27 26 25 24

23 22 21 20 19 18 17 16

| DATA2[15:0] |
|---|
| r |


15 14 13 12 11 10 9 8

7 6 5 4 3 2 1 0

| DATA1[15:0] |
|---|
| r |


Bits 31:16 DATA2[15:0]:

2nd data item of a pair of regular conversions

– In dual mode, these bits contain the regular data of ADC2. Refer to –

Dual ADC mode . In triple mode, these bits contain alternatively the regular data of ADC2, ADC1 and ADC3.

Refer to Triple ADC mode .

Bits 15:0 DATA1[15:0]

: 1st data item of a pair of regular conversions

– In dual mode, these bits contain the regular data of ADC1. Refer to –

Dual ADC mode In triple mode, these bits contain alternatively the regular data of ADC1, ADC3 and ADC2.

Refer to Triple ADC mode .


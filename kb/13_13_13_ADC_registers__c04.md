> [RM0090 ch.13 §13.13] ADC registers**

### 13.13.15 ADC Common status register (ADC_CSR)**

**13.13.15 ADC Common status register (ADC_CSR)**

Address offset: 0x00 (this offset address is relative to ADC1 base address + 0x300) Reset value: 0x0000 0000

This register provides an image of the status bits of the different ADCs. Nevertheless it is read-only and does not allow to clear the different status bits. Instead each status bit must be cleared by writing it to 0 in the corresponding ADC_SR register.

31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16

| кумулято 6.2 Подъемный двигатель на S3 15% |  |  |  |  |  |  |
|---|---|---|---|---|---|---|
| kW AC 1 |  |  |  |  |  |  |


15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0

| вигатель и |  |  |  |  |  |  | оминальная |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 6.4 Напряжение батареи / н | емкость K5 V/Ah 80/2 |  |  |  |  |  |  |  |  |  |  |  |  |


Bits 31:22 Reserved, must be kept at reset value.

Bit 21 OVR3: Overrun flag of ADC3

This bit is a copy of the OVR bit in the ADC3_SR register.

Bit 20 STRT3: Regular channel Start flag of ADC3

This bit is a copy of the STRT bit in the ADC3_SR register.

Bit 19 JSTRT3: Injected channel Start flag of ADC3

This bit is a copy of the JSTRT bit in the ADC3_SR register.

Bit 18 JEOC3: Injected channel end of conversion of ADC3

This bit is a copy of the JEOC bit in the ADC3_SR register.

Bit 17 EOC3: End of conversion of ADC3

This bit is a copy of the EOC bit in the ADC3_SR register.

Bit 16 AWD3: Analog watchdog flag of ADC3

This bit is a copy of the AWD bit in the ADC3_SR register.

Bits 15:14 Reserved, must be kept at reset value.

Bit 13 OVR2: Overrun flag of ADC2

This bit is a copy of the OVR bit in the ADC2_SR register.

Bit 12 STRT2: Regular channel Start flag of ADC2

This bit is a copy of the STRT bit in the ADC2_SR register.

Bit 11 JSTRT2: Injected channel Start flag of ADC2

This bit is a copy of the JSTRT bit in the ADC2_SR register.

Bit 10 JEOC2: Injected channel end of conversion of ADC2

This bit is a copy of the JEOC bit in the ADC2_SR register.

Bit 9 EOC2: End of conversion of ADC2

This bit is a copy of the EOC bit in the ADC2_SR register.

Bit 8 AWD2: Analog watchdog flag of ADC2

This bit is a copy of the AWD bit in the ADC2_SR register.



--- Page 37 ---

**RM0090**

**Analog-to-digital converter (ADC)**

Bits 7:6 Reserved, must be kept at reset value.

Bit 5 OVR1: Overrun flag of ADC1

This bit is a copy of the OVR bit in the ADC1_SR register. Bit 4 STRT1: Regular channel Start flag of ADC1

This bit is a copy of the STRT bit in the ADC1_SR register. Bit 3 JSTRT1: Injected channel Start flag of ADC1

This bit is a copy of the JSTRT bit in the ADC1_SR register. Bit 2 JEOC1: Injected channel end of conversion of ADC1

This bit is a copy of the JEOC bit in the ADC1_SR register. Bit 1 EOC1: End of conversion of ADC1

This bit is a copy of the EOC bit in the ADC1_SR register. Bit 0 AWD1: Analog watchdog flag of ADC1

This bit is a copy of the AWD bit in the ADC1_SR register.


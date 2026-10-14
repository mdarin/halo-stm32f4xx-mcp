**6.3.4 RCC clock interrupt register (RCC_CIR)**

Address offset: 0x0C

Reset value: 0x0000 0000

Access: no wait state, word, half-word and byte access

31 30 29 28 27 26 25 24

23 22 21 20 19 18 17 16

| Reserved | CSSC | PLLSAI RDYC | PLLI2S RDYC | PLL RDYC | HSE RDYC | HSI RDYC | LSE RDYC | LSI RDYC |
|---|---|---|---|---|---|---|---|---|
| w |  |  |  |  |  |  |  |  |


15 14 13 12 11 10 9 8

7 6 5 4 3 2 1 0

| Reserv ed | PLLSAI RDYIE | PLLI2S RDYIE | PLL RDYIE | HSE RDYIE | HSI RDYIE | LSE RDYIE | LSI RDYIE | CSSF | PLLSAI RDYF | PLLI2S RDYF | PLL RDYF | HSE RDYF | HSI RDYF | LSE RDYF | LSI RDYF |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| rw | r |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


RM0090 Rev 18 167/1749

212

--- Page 168 ---

**Reset and clock control for   STM32F42xxx and STM32F43xxx (RCC) RM0090**

Bits 31:24 Reserved, must be kept at reset value.

Bit 23 CSSC: Clock security system interrupt clear

This bit is set by software to clear the CSSF flag.

0: No effect

1: Clear CSSF flag

Bit 22 PLLSAIRDYC : PLLSAI Ready Interrupt Clear

This bit is set by software to clear PLLSAIRDYF flag. It is reset by hardware when the PLLSAIRDYF is cleared.

0: PLLSAIRDYF not cleared

1: PLLSAIRDYF cleared

Bit 21 PLLI2SRDYC: PLLI2S ready interrupt clear

This bit is set by software to clear the PLLI2SRDYF flag.

0: No effect

1: PLLI2SRDYF cleared

Bit 20 PLLRDYC: Main PLL(PLL) ready interrupt clear

This bit is set by software to clear the PLLRDYF flag.

0: No effect

1: PLLRDYF cleared

Bit 19 HSERDYC: HSE ready interrupt clear

This bit is set by software to clear the HSERDYF flag.

0: No effect

1: HSERDYF cleared

Bit 18 HSIRDYC: HSI ready interrupt clear

This bit is set software to clear the HSIRDYF flag.

0: No effect

1: HSIRDYF cleared

Bit 17 LSERDYC: LSE ready interrupt clear

This bit is set by software to clear the LSERDYF flag.

0: No effect

1: LSERDYF cleared

Bit 16 LSIRDYC: LSI ready interrupt clear

This bit is set by software to clear the LSIRDYF flag.

0: No effect

1: LSIRDYF cleared

Bit 15 Reserved, must be kept at reset value.

Bit 14 PLLSAIRDYIE : PLLSAI Ready Interrupt Enable

This bit is set and reset by software to enable/disable interrupt caused by PLLSAI lock. 0: PLLSAI lock interrupt disabled

1: PLLSAI lock interrupt enabled

Bit 13 PLLI2SRDYIE: PLLI2S ready interrupt enable

This bit is set and cleared by software to enable/disable interrupt caused by PLLI2S lock. 0: PLLI2S lock interrupt disabled

1: PLLI2S lock interrupt enabled

168/1749 RM0090 Rev 18

--- Page 169 ---

**RM0090 Reset and clock control for   STM32F42xxx and STM32F43xxx (RCC)**

Bit 12 PLLRDYIE: Main PLL (PLL) ready interrupt enable

This bit is set and cleared by software to enable/disable interrupt caused by PLL lock.

0: PLL lock interrupt disabled

1: PLL lock interrupt enabled

Bit 11 HSERDYIE: HSE ready interrupt enable

This bit is set and cleared by software to enable/disable interrupt caused by the HSE oscillator stabilization.

0: HSE ready interrupt disabled

1: HSE ready interrupt enabled

Bit 10 HSIRDYIE: HSI ready interrupt enable

This bit is set and cleared by software to enable/disable interrupt caused by the HSI oscillator stabilization.

0: HSI ready interrupt disabled

1: HSI ready interrupt enabled

Bit 9 LSERDYIE: LSE ready interrupt enable

This bit is set and cleared by software to enable/disable interrupt caused by the LSE oscillator stabilization.

0: LSE ready interrupt disabled

1: LSE ready interrupt enabled

Bit 8 LSIRDYIE: LSI ready interrupt enable

This bit is set and cleared by software to enable/disable interrupt caused by LSI oscillator stabilization.

0: LSI ready interrupt disabled

1: LSI ready interrupt enabled

Bit 7 CSSF: Clock security system interrupt flag

This bit is set by hardware when a failure is detected in the HSE oscillator.

It is cleared by software by setting the CSSC bit.

0: No clock security interrupt caused by HSE clock failure

1: Clock security interrupt caused by HSE clock failure

Bit 6 PLLSAIRDYF : PLLSAI Ready Interrupt flag

This bit is set by hardware when the PLLSAI is locked and PLLSAIRDYDIE is set.

It is cleared by software by setting the PLLSAIRDYC bit.

0: No clock ready interrupt caused by PLLSAI lock

1: Clock ready interrupt caused by PLLSAI lock

Bit 5 PLLI2SRDYF: PLLI2S ready interrupt flag

This bit is set by hardware when the PLLI2S is locked and PLLI2SRDYDIE is set.

It is cleared by software by setting the PLLRI2SDYC bit.

0: No clock ready interrupt caused by PLLI2S lock

1: Clock ready interrupt caused by PLLI2S lock

Bit 4 PLLRDYF: Main PLL (PLL) ready interrupt flag

This bit is set by hardware when PLL is locked and PLLRDYDIE is set.

It is cleared by software setting the PLLRDYC bit.

0: No clock ready interrupt caused by PLL lock

1: Clock ready interrupt caused by PLL lock

RM0090 Rev 18 169/1749

212

--- Page 170 ---

**Reset and clock control for   STM32F42xxx and STM32F43xxx (RCC) RM0090**

Bit 3 HSERDYF: HSE ready interrupt flag

This bit is set by hardware when External High Speed clock becomes stable and HSERDYDIE is set.

It is cleared by software by setting the HSERDYC bit.

0: No clock ready interrupt caused by the HSE oscillator

1: Clock ready interrupt caused by the HSE oscillator

Bit 2 HSIRDYF: HSI ready interrupt flag

This bit is set by hardware when the Internal High Speed clock becomes stable and HSIRDYDIE is set.

It is cleared by software by setting the HSIRDYC bit.

0: No clock ready interrupt caused by the HSI oscillator

1: Clock ready interrupt caused by the HSI oscillator

Bit 1 LSERDYF: LSE ready interrupt flag

This bit is set by hardware when the External Low Speed clock becomes stable and LSERDYDIE is set.

It is cleared by software by setting the LSERDYC bit.

0: No clock ready interrupt caused by the LSE oscillator

1: Clock ready interrupt caused by the LSE oscillator

Bit 0 LSIRDYF: LSI ready interrupt flag

This bit is set by hardware when the internal low speed clock becomes stable and LSIRDYDIE is set.

It is cleared by software by setting the LSIRDYC bit.

0: No clock ready interrupt caused by the LSI oscillator

1: Clock ready interrupt caused by the LSI oscillator


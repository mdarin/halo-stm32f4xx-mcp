### 6.3.25 RCC Dedicated Clock Configuration Register (RCC_DCKCFGR)

Address offset: 0x8C
Reset value: 0x0000 0000
Access: no wait state, word, half-word and byte access.

The RCC_DCKCFGR register allows to configure the timer clock prescalers and the PLLSAI and PLLI2S output clock dividers for SAI1 and LTDC peripherals according to the following formula:

f(PLLSAIDIVQ clock output) = f(PLLSAI_Q) / PLLSAIDIVQ
f(PLLSAIDIVR clock output) = f(PLLSAI_R) / PLLSAIDIVR
f(PLLI2SDIVQ clock output) = f(PLLI2S_Q) / PLLI2SDIVQ

| 31 30 29 28 27 26 25 | 24 | 23 | 22 | 21 | 20 | 19 18 | 17 | 16 |
|---|---|---|---|---|---|---|---|---|
| Reserved | TIMPRErw | SAI1BSRCrw | SAI1ASRCrw | Reserved | PLLSAIDIVRrw | rw |  |  |
| 15 14 13 12 | 11 10 9 8 7 6 5 4 3 2 1 0 |  |  |  |  |  |  |  |
| Reserved | PLLSAIDIVQrw rw rw rw rw | Reserved | PLLS2DIVQrw rw rw rw rw |  |  |  |  |  |


ST
RM0090 Rev 18
207/1749

--- Page 13 ---

**Reset and clock control for STM32F42xxx and STM32F43xxx (RCC)** RM0090

Bits 31:25 Reserved, must be kept at reset value.

Bit 24 **TIMPRE**: Timers clocks prescalers selection
This bit is set and reset by software to control the clock frequency of all the timers connected to APB1 and APB2 domain.
0: If the APB prescaler (PPRE1, PPRE2 in the RCC_CFGR register) is configured to a division factor of 1, TIMxCLK = PCLKx. Otherwise, the timer clock frequencies are set to twice to the frequency of the APB domain to which the timers are connected:
TIMxCLK = 2xPCLKx.
1: If the APB prescaler (PPRE1, PPRE2 in the RCC_CFGR register) is configured to a division factor of 1, 2 or 4, TIMxCLK = HCLK. Otherwise, the timer clock frequencies are set to four times to the frequency of the APB domain to which the timers are connected:
TIMxCLK = 4xPCLKx.

Bits 23:22 **SA1BSRC**: SAI1-B clock source selection
These bits are set and cleared by software to control the SAI1-B clock frequency.
They should be written when the PLLSAI and PLLI2S are disabled.
00: SAI1-B clock frequency = f(PLLSAI_Q) / PLLSAIDIVQ
01: SAI1-B clock frequency = f(PLLI2S_Q) / PLLI2SDIVQ
10: SAI1-B clock frequency = Alternate function input frequency
11: wrong configuration

Bits 21:20 **SA1ASRC**: SAI1-A clock source selection
These bits are set and cleared by software to control the SAI1-A clock frequency.
They should be written when the PLLSAI and PLLI2S are disabled.
00: SAI1-A clock frequency = f(PLLSAI_Q) / PLLSAIDIVQ
01: SAI1-A clock frequency = f(PLLI2S_Q) / PLLI2SDIVQ
10: SAI1-A clock frequency = Alternate function input frequency
11: wrong configuration

Bits 19: 18 Reserved, must be kept at reset value.

Bits 17:16 **PLLSAIDIVR**: division factor for LCD_CLK
These bits are set and cleared by software to control the frequency of LCD_CLK.
They should be written only if PLLSAI is disabled.
LCD_CLK frequency = f(PLLSAI_R) / PLLSAIDIVR with 2 ≤PLLSAIDIVR ≤16
00: PLLSAIDIVR = /2
01: PLLSAIDIVR = /4
10: PLLSAIDIVR = /8
11: PLLSAIDIVR = /16

Bits 15: 13 Reserved, must be kept at reset value.

208/1749 RM0090 Rev 18

--- Page 14 ---

RM0090
Reset and clock control for STM32F42xxx and STM32F43xxx (RCC)

Bits 12:8 **PLLSAIDIVQ**: PLLSAI division factor for SAI1 clock
These bits are set and reset by software to control the SAI1 clock frequency.
They should be written only if PLLSAI is disabled.
SAI1 clock frequency = f(PLLSAI_Q) / PLLSAIDIVQ with 1 ≤PLLSAIDIVQ ≤31
00000: PLLSAIDIVQ = /1
00001: PLLSAIDIVQ = /2
00010: PLLSAIDIVQ = /3
00011: PLLSAIDIVQ = /4
00100: PLLSAIDIVQ = /5
...
11111: PLLSAIDIVQ = /32

Bits 7:5 Reserved, must be kept at reset value.

Bits 4:0 **PLLI2SDIVQ**: PLLI2S division factor for SAI1 clock
These bits are set and reset by software to control the SAI1 clock frequency.
They should be written only if PLLI2S is disabled.
SAI1 clock frequency = f(PLLI2S_Q) / PLLI2SDIVQ with 1 <= PLLI2SDIVQ <= 31
00000: PLLI2SDIVQ = /1
00001: PLLI2SDIVQ = /2
00010: PLLI2SDIVQ = /3
00011: PLLI2SDIVQ = /4
00100: PLLI2SDIVQ = /5
...
11111: PLLI2SDIVQ = /32

ST
RM0090 Rev 18
209/1749

--- Page 15 ---

6.3.26 RCC register map
Reset and clock control for STM32F42xxx and STM32F43xxx (RCC)
Table 33 gives the register map and reset values.
Table 33. RCC register map and reset values for STM32F42xxx and STM32F43xxx

| Addr. offset | Register name | RCC_CFGR | RCC_CIR | RCC_AHBRSTR | RCC_AHB2RSTR | RCC_AHB3RSTR | RCC_APB1RSTR | RCC_APB2RSTR | Reserved | RCC_AHB1ENR | RCC_AHB2ENR | RCC_AHB3ENR | Reserved |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 0x00 | 0x04 | 0x08 | 0x0C | 0x10 | 0x14 | 0x18 | 0x1C | 0x20 | 0x24 | 0x28 | 0x2C | 0x30 | 0x34 | 0x38 | 0x3C |
| 0 | Reserved | PLLON | Reserved | PLLRDY | Reserved | OTGHSRST | Reserved | OTGSOPIEN | OTCHSEN | ETHMACPTPEN | ETHMACRST |  |  |  |  |
| 1 | Reserved | PLLON | Reserved | PLLRDY | Reserved | OTGHSRST | Reserved | OTGSOPIEN | OTCHSEN | ETHMACPTPEN | ETHMACRST |  |  |  |  |
| 2 | Reserved | PLLON | Reserved | PLLRDY | Reserved | OTGHSRST | Reserved | OTGSOPIEN | OTCHSEN | ETHMACPTPEN | ETHMACRST |  |  |  |  |
| 3 | Reserved | PLLON | Reserved | PLLRDY | Reserved | OTGHSRST | Reserved | OTGSOPIEN | OTCHSEN | ETHMACPTPEN | ETHMACRST |  |  |  |  |
| 4 | Reserved | PLLON | Reserved | PLLRDY | Reserved | OTGHSRST | Reserved | OTGSOPIEN | OTCHSEN | ETHMACPTPEN | ETHMACRST |  |  |  |  |
| 5 | Reserved | PLLON | Reserved | PLLRDY | Reserved | OTGHSRST | Reserved | OTGSOPIEN | OTCHSEN | ETHMACPTPEN | ETHMACRST |  |  |  |  |
| 6 | Reserved | PLLON | Reserved | PLLRDY | Reserved | OTGHSRST | Reserved | OTGSOPIEN | OTCHSEN | ETHMACPTPEN | ETHMACRST |  |  |  |  |
| 7 | Reserved | PLLON | Reserved | PLLRDY | Reserved | OTGHSRST | Reserved | OTGSOPIEN | OTCHSEN | ETHMACPTPEN | ETHMACRST |  |  |  |  |
| 8 | Reserved | PLLON | Reserved | PLLRDY | Reserved | OTGHSRST | Reserved | OTGSOPIEN | OTCHSEN | ETHMACPTPEN | ETHMACRST |  |  |  |  |
| 9 | Reserved | PLLON | Reserved | PLLRDY | Reserved | OTGHSRST | Reserved | OTGSOPIEN | OTCHSEN | ETHMACPTPEN | ETHMACRST |  |  |  |  |
| 10 | Reserved | PLLON | Reserved | PLLRDY | Reserved | OTGHSRST | Reserved | OTGSOPIEN | OTCHSEN | ETHMACPTPEN | ETHMACRST |  |  |  |  |
| 11 | Reserved | PLLON | Reserved | PLLRDY | Reserved | OTGHSRST | Reserved | OTGSOPIEN | OTCHSEN | ETHMACPTPEN | ETHMACRST |  |  |  |  |
| 12 | Reserved | PLLON | Reserved | PLLRDY | Reserved | OTGHSRST | Reserved | OTGSOPIEN | OTCHSEN | ETHMACPTPEN | ETHMACRST |  |  |  |  |
| 13 | Reserved | PLLON | Reserved | PLLRDY | Reserved | OTGHSRST | Reserved | OTGSOPIEN | OTCHSEN | ETHMACPTPEN | ETHMACRST |  |  |  |  |
| 14 | Reserved | PLLON | Reserved | PLLRDY | Reserved | OTGHSRST | Reserved | OTGSOPIEN | OTCHSEN | ETHMACPTPEN | ETHMACRST |  |  |  |  |
| 15 | Reserved | PLLON | Reserved | PLLRDY | Reserved | OTGHSRST | Reserved | OTGSOPIEN | OTCHSEN | ETHMACPTPEN | ETHMACRST |  |  |  |  |
| 16 | Reserved | PLLON | Reserved | PLLRDY | Reserved | OTGHSRST | Reserved | OTGSOPIEN | OTCHSEN | ETHMACPTPEN | ETHMACRST |  |  |  |  |
| 17 | Reserved | PLLON | Reserved | PLLRDY | Reserved | OTGHSRST | Reserved | OTGSOPIEN | OTCHSEN | ETHMACPTPEN | ETHMACRST |  |  |  |  |
| 18 | Reserved | PLLON | Reserved | PLLRDY | Reserved | OTGHSRST | Reserved | OTGSOPIEN | OTCHSEN | ETHMACPTPEN | ETHMACRST |  |  |  |  |
| 19 | Reserved | PLLON | Reserved | PLLRDY | Reserved | OTGHSRST | Reserved | OTGSOPIEN | OTCHSEN | ETHMACPTPEN | ETHMACRST |  |  |  |  |
| 20 | Reserved | PLLON | Reserved | PLLRDY | Reserved | OTGHSRST | Reserved | OTGSOPIEN | OTCHSEN | ETHMACPTPEN | ETHMACRST |  |  |  |  |
| 21 | Reserved | PLLON | Reserved | PLLRDY | Reserved | OTGHSRST | Reserved | OTGSOPIEN | OTCHSEN | ETHMACPTPEN | ETHMACRST |  |  |  |  |
| 22 | Reserved | PLLON | Reserved | PLLRDY | Reserved | OTGHSRST | Reserved | OTGSOPIEN | OTCHSEN | ETHMACPTPEN | ETHMACRST |  |  |  |  |
| 23 | Reserved | PLLON | Reserved | PLLRDY | Reserved | OTGHSRST | Reserved | OTGSOPIEN | OTCHSEN | ETHMACPTPEN | ETHMACRST |  |  |  |  |
| 24 | Reserved | PLLON | Reserved | PLLRDY | Reserved | OTGHSRST | Reserved | OTGSOPIEN | OTCHSEN | ETHMACPTPEN | ETHMACRST |  |  |  |  |
| 25 | Reserved | PLLON | Reserved | PLLRDY | Reserved | OTGHSRST | Reserved | OTGSOPIEN | OTCHSEN | ETHMACPTPEN | ETHMACRST |  |  |  |  |
| 26 | Reserved | PLLON | Reserved | PLLRDY | Reserved | OTGHSRST | Reserved | OTGSOPIEN | OTCHSEN | ETHMACPTPEN | ETHMACRST |  |  |  |  |
| 27 | Reserved | PLLON | Reserved | PLLRDY | Reserved | OTGHSRST | Reserved | OTGSOPIEN | OTCHSEN | ETHMACPTPEN | ETHMACRST |  |  |  |  |
| 28 | Reserved | PLLON | Reserved | PLLRDY | Reserved | OTGHSRST | Reserved | OTGSOPIEN | OTCHSEN | ETHMACPTPEN | ETHMACRST |  |  |  |  |
| 29 | Reserved | PLLON | Reserved | PLLRDY | Reserved | OTGHSRST | Reserved | OTGSOPIEN | OTCHSEN | ETHMACPTPEN | ETHMACRST |  |  |  |  |
| 30 | Reserved | PLLON | Reserved | PLLRDY | Reserved | OTGHSRST | Reserved | OTGSOPIEN | OTCHSEN | ETHMACPTPEN | ETHMACRST |  |  |  |  |
| 31 | Reserved | PLLON | Reserved | PLLRDY | Reserved | OTGHSRST | Reserved | OTGSOPIEN | OTCHSEN | ETHMACPTPEN | ETHMACRST |  |  |  |  |


--- Page 16 ---

RM0090
Reset and clock control for STM32F42xxx and STM32F43xxx (RCC)

Table 33. RCC register map and reset values for STM32F42xxx and STM32F43xxx (continued)

| Addr. | Register | name | 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 0x40 | RCC_APB1ENR | UART8EN | UART7EN | DACEN | PWREN | Reserved | CAN2EN | CAN1EN | Reserved | SP15EN | SP16EN | SP17EN | SP18EN | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x44 | RCC_APB2ENR | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x48 | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x4C | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x50 | RCC_AHB1LPEN | OTGHSLP | OTGHSULP | ETHMACPTP | ETHMAC | ETHMACRX | ETHMAC | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x54 | RCC_AHB2LPEN | OTGFS | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x58 | RCC_AHB3LPEN | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x5C | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x60 | RCC_APB1LPEN | UART8LP | UART7LP | DACLP | PWRLP | Reserved | CAN2LP | CAN1LP | Reserved | SP15LP | SP16LP | SP17LP | SP18LP | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x64 | RCC_APB2LPEN | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x68 | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x6C | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x70 | RCC_BDCR | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x74 | RCC_CSR | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x78 | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x7C | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x80 | RCC_SSCGR | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x84 | RCC_PLLI2SCFGR | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


RM0090 Rev 18
211/1749

--- Page 17 ---

Reset and clock control for STM32F42xxx and STM32F43xxx (RCC) RM0090

Table 33. RCC register map and reset values for STM32F42xxx and STM32F43xxx (continued)

| Addr. offset | Register name | 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 0x88 | RCC_PLLSAICFGR |  | PLLSAIR |  | PLLSAIQ |  |  |  |  |  |  |  |  | Reserved |  |  |  |  |  |  |  | PLLSAIN |  |  |  |  |  |  |  |  |  | Reserved |  |
| 0x8C | RCC_DCKCFGR | Reserved | TIMPRE | SA1BSCR | SA1ASCR | Reserved | PLLSAIDIVR | Reserved | PLLSAIDIVQ | Reserved | PLL2SDIVQ |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Refer to Section 2.3: Memory map for the register boundary addresses.

212/1749 RM0090 Rev 18 ST

--- Page 18 ---

RM0090 Reset and clock control for STM32F405xx/07xx and STM32F415xx/17xx(RCC)


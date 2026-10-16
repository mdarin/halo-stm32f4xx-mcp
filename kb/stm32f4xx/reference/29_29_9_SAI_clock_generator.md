## 29.9 SAI clock generator

Each audio block has its own clock generator to make these two blocks completely independent. There is no difference in terms of functionality between these two clock generators. They are exactly the same.

When the audio block is defined as Master, the clock generator generates the communication clock (the bit clock) and the master clock for external decoders.

When the audio block is defined as slave, the clock generator is OFF.

*Figure 289* illustrates the architecture of the audio block clock generator.

**Figure 289. Audio block clock generator overview**

Note:
*If NoDiv is set to 1, the MCLK_x signal will be set at 0 level if this pin is configured as the SAI pin in GPIO peripherals.*

The clock source for the clock generator comes from the product clock controller. The SAI_CK_x clock is equivalent to the master clock which may be divided for the external decoders using bit MCKDIV[3:0]:
MCLK_x = SAI_CK_x / (MCKDIV[3:0] * 2), if MCKDIV[3:0] is not equal to 0000.
MCLK_x = SAI_CK_x, if MCKDIV[3:0] is equal to 0000.
MCLK_x signal is used only in TDM.

The division must be even in order to keep 50% on the Duty cycle on the MCLK output and on the SCK_x clock. If bit MCKDIV[3:0] = 0000, division by one is applied to have MCLK_x = SAI_CK_x.

In the SAI, the single ratio MCLK/FS = 256 is considered. Mostly, three frequency ranges will be encountered as illustrated in the *Table 130*.

ST
RM0090 Rev 18
935/1749

--- Page 156 ---

**Serial audio interface (SAI)**
RM0090

**Table 130. Example of possible audio frequency sampling range**

| Input SAI_CK_x clock frequency | Most usual audio frequency sampling achievable | MCKDIV[3:0] |
|---|---|---|
| 192 kHz x 256 | 192 kHz | MCKDIV[3:0] = 0000 |
| 96 kHz | MCKDIV[3:0] = 0001 |  |
| 48 kHz | MCKDIV[3:0] = 0010 |  |
| 16 kHz | MCKDIV[3:0] = 0100 |  |
| 8 kHz | MCKDIV[3:0] = 1000 |  |
| 44.1 kHz x 256 | 44.1 kHz | MCKDIV[3:0] = 0000 |
| 22.05 kHz | MCKDIV[3:0] = 0001 |  |
| 11.025 kHz | MCKDIV[3:0] = 0010 |  |
| SAI_CK_x = MCLK(1) | MCLK | MCKDIV[3:0] = 0000 |


1. This may happen when the product clock controller selects an external clock source, instead of PLL clock.

The master clock may be generated externally on an I/O pad for external decoders if the corresponding audio block is declared as master with bit NODIV = 0 in the SAI_xCR1 register. In slave, the value set in this last bit is ignored since the clock generator is OFF, and the MCLK_x I/O pin is released for use as a general purpose I/O.

The bit clock is derived from the master clock. The bit clock divider sets the divider factor between the bit clock SCK_x and the master clock MCLK_x following the formula:
SCK_x = MCLK x (FRL[7:0] +1) / 256

where:
256 is the fixed ratio between MCLK and the audio frequency sampling.
FRL[7:0] is the number of bit clock - 1 in the audio frame, configured in the SAI_xFRCR register.

It is mandatory in master mode that (FRL[7:0] +1) should be equal to a number with a power of 2 (refer to Section 29.7) in order to have an even integer number of MCLK_x pulses by bit clock. The 50% duty cycle is guaranteed on the bit clock SCK_x.

The SAI_CK_x clock can be also equal to the bit clock frequency. In this case, bit NODIV in the SAI_xCR1 register should be set and the value inside the MCKDIV divider and the bit clock divider will be ignored. In this case, the number of bits per frame is fully configurable without the need to be equal to a power of two.

The bit clock strobing edge on SCK can be configured by bit CKSTR in the SAI_xCR1 register.


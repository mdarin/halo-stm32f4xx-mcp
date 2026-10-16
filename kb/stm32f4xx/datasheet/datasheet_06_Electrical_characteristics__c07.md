> [DATASHEET ch.6 §6] Electrical characteristics

### 6.3.11 PLL spread spectrum clock generation (SSCG) characteristics

### 6.3.11 PLL spread spectrum clock generation (SSCG) characteristics

The spread spectrum clock generation (SSCG) feature allows to reduce electromagnetic interferences (see Table 44: EMI characteristics for fHSE = 25 MH and fCPU = 168 MHz). It is available only on the main PLL.

Table 38. SSCG parameters constraint

| Symbol | Parameter | Min | Typ | Max(1) | Unit |
|---|---|---|---|---|---|
| fMod | Modulation frequency | - | 10 | KHz |  |
| md | Peak modulation depth | 0.25 | - | 2 | % |
| MODEPER * INCSTEP | - | 215-1 | - |  |  |


1. Specified by design.

Equation 1
The frequency modulation period (MODEPER) is given by the equation below:
MODEPER = round[f<sub>PLL_IN</sub>/ (4 × f<sub>Mod</sub>)]

f<sub>PLL_IN</sub> and f<sub>Mod</sub> must be expressed in Hz.

As an example:
If f<sub>PLL_IN</sub> = 1 MHz, and f<sub>MOD</sub> = 1 kHz, the modulation depth (MODEPER) is given by equation 1:
MODEPER = round[10<sup>6</sup>/ (4 × 10<sup>3</sup>)] = 250

Equation 2
Equation 2 allows to calculate the increment step (INCSTEP):
INCSTEP = round[((2<sup>15</sup> - 1) × md × PLLN)/ (100 × 5 × MODEPER)]

f<sub>VCO_OUT</sub> must be expressed in MHz.

108/206
DS8626 Rev 12
ST

--- Page 109 ---

**STM32F405xx, STM32F407xx**
**Electrical characteristics**

With a modulation depth (md) = ±2 % (4 % peak to peak), and PLLN = 240 (in MHz):

INCSTEP = round[((2^15 - 1) × 2 × 240) / (100 × 5 × 250)] = 126md(quantitazed)%

An amplitude quantization error may be generated because the linear modulation profile is obtained by taking the quantized values (rounded to the nearest integer) of MODPER and INCSTEP. As a result, the achieved modulation depth is quantized. The percentage quantized modulation depth is given by the following formula:

md_quantized% = (MODEPER × INCSTEP × 100 × 5) / ((2^15 - 1) × PLLN)

As a result:

md_quantized% = (250 × 126 × 100 × 5) / ((2^15 - 1) × 240) = 2.002%(peak)

**Figure 35** and **Figure 36** show the main PLL output clock waveforms in center spread and down spread modes, where:
F0 is f_PLL_OUT nominal.
T_mode is the modulation period.
md is the modulation depth.

**Figure 35. PLL output clock waveforms in center spread mode**

Frequency (PLL_OUT)
F0
md
md
tmode
2xtmode
Time
ai17291

ST
DS8626 Rev 12
109/206

--- Page 110 ---

Electrical characteristics
STM32F405xx, STM32F407xx

Figure 36. PLL output clock waveforms in down spread mode

Frequency (PLL_OUT)
F0
2xmd
tmode
2xtmode
Time
ai17292b

6.3.12 Memory characteristics

Flash memory

The characteristics are given at T_A = -40 to 105 °C unless otherwise specified.
The devices are shipped to customers with the flash memory erased.

Table 39. Flash memory characteristics
| Symbol | Parameter | Conditions | Min | Typ | Max | Unit |
|---|---|---|---|---|---|---|
| I_DD | Supply current | Write / Erase 8-bit mode, V_DD = 1.8 V | - | 5 | - | mA |
| Write / Erase 16-bit mode, V_DD = 2.1 V | - | 8 | - |  |  |  |
| Write / Erase 32-bit mode, V_DD = 3.3 V | - | 12 | - |  |  |  |


Table 40. Flash memory programming
| Symbol | Parameter | Conditions | Min(1) | Typ | Max(1) | Unit |
|---|---|---|---|---|---|---|
| t_prog | Word programming time | Program/erase parallelism (PSIZE) = x 8/16/32 | - | 16 | 100(2) | µs |
| t_ERASE16KB | Sector (16 KB) erase time | Program/erase parallelism (PSIZE) = x 8 | - | 400 | 800 | ms |
| Program/erase parallelism (PSIZE) = x 16 | - | 300 | 600 |  |  |  |
| Program/erase parallelism (PSIZE) = x 32 | - | 250 | 500 |  |  |  |
| t_ERASE64KB | Sector (64 KB) erase time | Program/erase parallelism (PSIZE) = x 8 | - | 1200 | 2400 | ms |
| Program/erase parallelism (PSIZE) = x 16 | - | 700 | 1400 |  |  |  |
| Program/erase parallelism (PSIZE) = x 32 | - | 550 | 1100 |  |  |  |


110/206
DS8626 Rev 12

--- Page 111 ---

STM32F405xx, STM32F407xx
Electrical characteristics

Table 40. Flash memory programming (continued)

| Symbol | Parameter | Conditions | Min(1) | Typ | Max(1) | Unit |
|---|---|---|---|---|---|---|
| tERASE128KB | Sector (128 KB) erase time | Program/erase parallelism (PSIZE) = x 8 | - | 2 | 4 | s |
| Program/erase parallelism (PSIZE) = x 16 | - | 1.3 | 2.6 |  |  |  |
| Program/erase parallelism (PSIZE) = x 32 | - | 1 | 2 |  |  |  |
| tME | Mass erase time | Program/erase parallelism (PSIZE) = x 8 | - | 16 | 32 | s |
| Program/erase parallelism (PSIZE) = x 16 | - | 11 | 22 |  |  |  |
| Program/erase parallelism (PSIZE) = x 32 | - | 8 | 16 |  |  |  |
| Vprog | Programming voltage | 32-bit program operation | 2.7 | - | 3.6 | V |
| 16-bit program operation | 2.1 | - | 3.6 | V |  |  |
| 8-bit program operation | 1.8 | - | 3.6 | V |  |  |


1. Evaluated by characterization - not tested in production.
2. The maximum programming time is measured after 100K erase operations.

ST
DS8626 Rev 12
111/206

--- Page 112 ---

Electrical characteristics
STM32F405xx, STM32F407xx

Table 41. Flash memory programming with Vpp

| Symbol | Parameter | Conditions | Min(1) | Typ | Max(1) | Unit |
|---|---|---|---|---|---|---|
| tprog | Double word programming | TA = 0 to +40 °CVDD = 3.3 VVPP = 8.5 V | - | 16 | 100(2) | µs |
| tERASE16KB | Sector (16 KB) erase time | - | 230 | - | ms |  |
| tERASE64KB | Sector (64 KB) erase time | - | 490 | - |  |  |
| tERASE128KB | Sector (128 KB) erase time | - | 875 | - |  |  |
| tME | Mass erase time | - | 6.9 | - | s |  |
| Vprog | Programming voltage | - | 2.7 | - | 3.6 | V |
| VPP | VPP voltage range | - | 7 | - | 9 | V |
| IPP | Minimum current sunk on the VPP pin | - | 10 | - | mA |  |
| tVPP(3) | Cumulative time during which VPP is applied | - | 1 | hour |  |  |


1. Specified by design.
2. The maximum programming time is measured after 100K erase operations.
3. V<sub>PP</sub> should only be connected during programming/erasing.

Table 42. Flash memory endurance and data retention

| Symbol | Parameter | Conditions | Value | Unit |
|---|---|---|---|---|
| Min(1) |  |  |  |  |
| NEND | Endurance | TA = -40 to +85 °C (6 suffix versions)TA = -40 to +105 °C (7 suffix versions) | 10 | kcyccles |
| tRET | Data retention | 1 kcycle(2) at TA = 85 °C | 30 | Years |
| 1 kcycle(2) at TA = 105 °C | 10 |  |  |  |
| 10 kcyccles(2) at TA = 55 °C | 20 |  |  |  |


1. Evaluated by characterization - not tested in production.
2. Cycling performed over the whole temperature range.


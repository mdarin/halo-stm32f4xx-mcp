> [DATASHEET ch.6 §6] Electrical characteristics

### 6.3.10 PLL characteristics

### 6.3.10 PLL characteristics

The parameters given in Table 36 and Table 37 are derived from tests performed under temperature and V$_{DD}$ supply voltage conditions summarized in Table 14.

Table 36. Main PLL characteristics

| Symbol | Parameter | Conditions | Min | Typ | Max | Unit |
|---|---|---|---|---|---|---|
| fPLL_IN | PLL input clock(1) | - | 0.95(2) | 1 | 2.10 | MHz |
| fPLL_OUT | PLL multiplier output clock | - | 24 | - | 168 | MHz |
| fPLL48_OUT | 48 MHz PLL multiplier output clock | - | 48 | 75 | MHz |  |
| fVCO_OUT | PLL VCO output | - | 100 | - | 432 | MHz |
| tLOCK | PLL lock time | VCO freq = 100 MHz | 75 | - | 200 | µs |
| VCO freq = 432 MHz | 100 | - | 300 |  |  |  |


106/206
DS8626 Rev 12

--- Page 107 ---

STM32F405xx, STM32F407xx
Electrical characteristics

Table 36. Main PLL characteristics (continued)

| Symbol | Parameter | Conditions | Min | Typ | Max | Unit |  |
|---|---|---|---|---|---|---|---|
| Jitter(3) | Cycle-to-cycle jitter | System clock120 MHz | RMS | - | 25 | - | ps |
| peak to peak | - | ±150 | - |  |  |  |  |
| Period Jitter | RMS | - | 15 | - |  |  |  |
| peak to peak | - | ±200 | - |  |  |  |  |
| Main clock output (MCO) for RMII Ethernet | Cycle to cycle at 50 MHz on 1000 samples | - | 32 | - |  |  |  |
| Main clock output (MCO) for MII Ethernet | Cycle to cycle at 25 MHz on 1000 samples | - | 40 | - |  |  |  |
|  | Bit Time CAN jitter | Cycle to cycle at 1 MHz on 1000 samples | - | 330 | - |  |  |
| IDD(PLL)(4) | PLL power consumption on VDD | VCO freq = 100 MHzVCO freq = 432 MHz | 0.150.45 | - | 0.400.75 | mA |  |
| IDDA(PLL)(4) | PLL power consumption on VDDA | VCO freq = 100 MHzVCO freq = 432 MHz | 0.300.55 | - | 0.400.85 | mA |  |


1. Take care of using the appropriate division factor M to obtain the specified PLL input clock values. The M factor is shared between PLL and PLLI2S.
2. Specified by design.
3. The use of 2 PLLs in parallel could degraded the Jitter up to +30%.
4. Evaluated by characterization - not tested in production.

Table 37. PLLI2S (audio PLL) characteristics

| Symbol | Parameter | Conditions | Min | Typ | Max | Unit |  |
|---|---|---|---|---|---|---|---|
| fPLLI2S_IN | PLLI2S input clock(1) | - | 0.95(2) | 1 | 2.10 | MHz |  |
| fPLLI2S_OUT | PLLI2S multiplier output clock | - | 216 | MHz |  |  |  |
| fVCO_OUT | PLLI2S VCO output | - | 100 | - | 432 | MHz |  |
| tLOCK | PLLI2S lock time | VCO freq = 100 MHz | 75 | - | 200 | µs |  |
| VCO freq = 432 MHz | 100 | - | 300 |  |  |  |  |
| Jitter(3) | Master I²S clock jitter | Cycle to cycle at 12.288 MHz on 48KHz period, N=432, R=5 | RMS | - | 90 | - | ps |
| peak to peak | - | ±280 | - |  |  |  |  |
|  | Average frequency of 12.288 MHz N = 432, R = 5 on 1000 samples | - | 90 | - |  |  |  |
|  | WS I²S clock jitter | Cycle to cycle at 48 KHz on 1000 samples | - | 400 | - | ps |  |


DS8626 Rev 12
107/206

--- Page 108 ---

Electrical characteristics
STM32F405xx, STM32F407xx

Table 37. PLLI2S (audio PLL) characteristics (continued)

| Symbol | Parameter | Conditions | Min | Typ | Max | Unit |
|---|---|---|---|---|---|---|
| IDD(PLLI2S)(4) | PLLI2S power consumption on VDD | VCO freq = 100 MHz | 0.15 | - | 0.40 | mA |
| VCO freq = 432 MHz | 0.45 | 0.75 |  |  |  |  |
| IDDA(PLLI2S)(4) | PLLI2S power consumption on VDDA | VCO freq = 100 MHz | 0.30 | - | 0.40 | mA |
| VCO freq = 432 MHz | 0.55 | 0.85 |  |  |  |  |


1. Take care of using the appropriate division factor M to have the specified PLL input clock values.
2. Specified by design.
3. Value given with main PLL running.
4. Evaluated by characterization - not tested in production.


**5.6 PWR register map**

The following table summarizes the PWR registers.

**Table 31. PWR - register map and reset values for STM32F405xx/07xx and STM32F415xx/17xx**

| Offset | Register | 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 0x000 | PWR CR _ Reset value | Reserved | VOS | Reserved | FPDS | DBP | PLS[2:0] | PVDE | CSBF | CWUF | PDDS | LPDS |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 1 | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x004 | PWR CSR _ Reset value | Reserved | VOSRDY | Reserved | BRE | EWUP | Reserved | BRR | PVDO | SBF | WUF |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


**Table 32. PWR - register map and reset values for STM32F42xxx and STM32F43xxx**

| Offset | Register | 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 0x000 | PWR CR _ Reset value | Reserved | UDEN[1:0] | ODSWEN | ODEN | VOS[1:0] | ADCDC1 | Reserved | MRUDS | LPUDS | FPDS | DBP | PLS[2:0] | PVDE | CSBF | CWUF | PDDS | LPDS |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 1 | 0 | 1 | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x004 | PWR CSR _ Reset value | Reserved | UDRDY[1:0] | ODSWRDY | ODRDY | Reserved | VOSRDY | Reserved | BRE 0 | EWUP 0 | Reserved | BRR 0 | PVDO 0 | SBF 0 | WUF 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Refer to Section 2.3: Memory map

for the register boundary addresses.



149

--- Page 150 ---

**Reset and clock control for   STM32F42xxx and STM32F43xxx (RCC) RM0090**


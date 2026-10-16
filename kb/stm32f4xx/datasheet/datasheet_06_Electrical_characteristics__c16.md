> [DATASHEET ch.6 §6] Electrical characteristics

### 6.3.26 FSMC characteristics**

**6.3.26 FSMC characteristics**

Unless otherwise specified, the parameters given in Table 75 to Table 86 for the FSMC interface are derived from tests performed under the ambient temperature, fHCLK frequency and VDD supply voltage conditions summarized in Table 14, with the following configuration:

*   Output speed is set to OSPEEDRy[1:0] = 10
*   Capacitive load C = 30 pF
*   Measurement points are done at CMOS levels: 0.5VDD

Refer to Section Section 6.3.16: I/O port characteristics for more details on the input/output characteristics.

**Asynchronous waveforms and timings**

Figure 54 through Figure 57 represent asynchronous waveforms and Table 75 through Table 78 provide the corresponding timings. The results shown in these tables are obtained with the following FSMC configuration:

*   AddressSetupTime = 1
*   AddressHoldTime = 0x1
*   DataSetupTime = 0x1
*   BusTurnAroundDuration = 0x0

In all timing tables, the THCLK is the HCLK clock period.

144/206
DS8626 Rev 12
STI

--- Page 145 ---

STM32F405xx, STM32F407xx
Electrical characteristics

Figure 54. Asynchronous non-multiplexed SRAM/PSRAM/NOR read waveforms

1. Mode 2/B, C and D only. In Mode 1, FSMC_NADV is not used.

Table 75. Asynchronous non-multiplexed SRAM/PSRAM/NOR read timings(1)(2)

| Symbol | Parameter | Min | Max | Unit |
|---|---|---|---|---|
| tw(NE) | FSMC_NE low time | 2THCLK−0.5 | 2 THCLK+1 | ns |
| tv(NOE_NE) | FSMC_NEx low to FSMC_NOE low | 0.5 | 3 | ns |
| tw(NOE) | FSMC_NOE low time | 2THCLK−2 | 2THCLK+ 2 | ns |
| th(NE_NOE) | FSMC_NOE high to FSMC_NE high hold time | 0 | - | ns |
| tv(A_NE) | FSMC_NEx low to FSMC_A valid | - | 4.5 | ns |
| th(A_NOE) | Address hold time after FSMC_NOE high | 4 | - | ns |
| tv(BL_NE) | FSMC_NEx low to FSMC_BL valid | - | 1.5 | ns |
| th(BL_NOE) | FSMC_BL hold time after FSMC_NOE high | 0 | - | ns |
| tsu(Data_NE) | Data to FSMC_NEx high setup time | THCLK+4 | - | ns |
| tsu(Data_NOE) | Data to FSMC_NOEx high setup time | THCLK+4 | - | ns |
| th(Data_NOE) | Data hold time after FSMC_NOE high | 0 | - | ns |
| th(Data_NE) | Data hold time after FSMC_NEx high | 0 | - | ns |
| tv(NADV_NE) | FSMC_NEx low to FSMC_NADV low | - | 2 | ns |
| tw(NADV) | FSMC_NADV low time | - | THCLK | ns |


1. C<sub>L</sub> = 30 pF.
2. Evaluated by characterization - not tested in production.

ST
DS8626 Rev 12
145/206

--- Page 146 ---

Electrical characteristics
STM32F405xx, STM32F407xx

Figure 55. Asynchronous non-multiplexed SRAM/PSRAM/NOR write waveforms

ai14990

1. Mode 2/B, C and D only. In Mode 1, FSMC_NADV is not used.

Table 76. Asynchronous non-multiplexed SRAM/PSRAM/NOR write timings(1)(2)

| Symbol | Parameter | Min | Max | Unit |
|---|---|---|---|---|
| tw(NE) | FSMC_NE low time | 3THCLK | 3THCLK+ 4 | ns |
| tv(NWE_NE) | FSMC_NEx low to FSMC_NWE low | THCLK-0.5 | THCLK+0.5 | ns |
| tw(NWE) | FSMC_NWE low time | THCLK-1 | THCLK+2 | ns |
| th(NE_NWE) | FSMC_NWE high to FSMC_NE high hold time | THCLK-1 | - | ns |
| tv(A_NE) | FSMC_NEx low to FSMC_A valid | - | 0 | ns |
| th(A_NWE) | Address hold time after FSMC_NWE high | THCLK- 2 | - | ns |
| tv(BL_NE) | FSMC_NEx low to FSMC_BL valid | - | 1.5 | ns |
| th(BL_NWE) | FSMC_BL hold time after FSMC_NWE high | THCLK- 1 | - | ns |
| tv(Data_NE) | Data to FSMC_NEx low to Data valid | - | THCLK+3 | ns |
| th(Data_NWE) | Data hold time after FSMC_NWE high | THCLK-1 | - | ns |
| tv(NADV_NE) | FSMC_NEx low to FSMC_NADV low | - | 2 | ns |
| tw(NADV) | FSMC_NADV low time | - | THCLK+0.5 | ns |


1. CL = 30 pF.
2. Evaluated by characterization - not tested in production.

146/206
DS8626 Rev 12

--- Page 147 ---

STM32F405xx, STM32F407xx
Electrical characteristics

Figure 56. Asynchronous multiplexed PSRAM/NOR read waveforms

ai14892b

Table 77. Asynchronous multiplexed PSRAM/NOR read timings(1)(2)

| Symbol | Parameter | Min | Max | Unit |
|---|---|---|---|---|
| $t_{w(NE)}$ | FSMC_NE low time | $3T_{HCLK}-1$ | $3T_{HCLK}+1$ | ns |
| $t_{v(NOE\_NE)}$ | FSMC_NEx low to FSMC_NOE low | $2T_{HCLK}-0.5$ | $2T_{HCLK}+0.5$ | ns |
| $t_{w(NOE)}$ | FSMC_NOE low time | $T_{HCLK}-1$ | $T_{HCLK}+1$ | ns |
| $t_{h(NE\_NOE)}$ | FSMC_NOE high to FSMC_NE high hold time | 0 | - | ns |
| $t_{v(A\_NE)}$ | FSMC_NEx low to FSMC_A valid | - | 3 | ns |
| $t_{v(NADV\_NE)}$ | FSMC_NEx low to FSMC_NADV low | 1 | 2 | ns |
| $t_{w(NADV)}$ | FSMC_NADV low time | $T_{HCLK}-2$ | $T_{HCLK}+1$ | ns |
| $t_{h(AD\_NADV)}$ | FSMC_AD(adress) valid hold time after FSMC_NADV high | $T_{HCLK}$ | - | ns |
| $t_{h(A\_NOE)}$ | Address hold time after FSMC_NOE high | $T_{HCLK}-1$ | - | ns |
| $t_{h(BL\_NOE)}$ | FSMC_BL time after FSMC_NOE high | 0 | - | ns |
| $t_{v(BL\_NE)}$ | FSMC_NEx low to FSMC_BL valid | - | 2 | ns |
| $t_{su(Data\_NE)}$ | Data to FSMC_NEx high setup time | $T_{HCLK}+4$ | - | ns |
| $t_{su(Data\_NOE)}$ | Data to FSMC_NOE high setup time | $T_{HCLK}+4$ | - | ns |
| $t_{h(Data\_NE)}$ | Data hold time after FSMC_NEx high | 0 | - | ns |
| $t_{h(Data\_NOE)}$ | Data hold time after FSMC_NOE high | 0 | - | ns |


1. $C_L$ = 30 pF.
2. Evaluated by characterization - not tested in production.

ST
DS8626 Rev 12
147/206

--- Page 148 ---

Electrical characteristics
STM32F405xx, STM32F407xx

Figure 57. Asynchronous multiplexed PSRAM/NOR write waveforms

ai14891B

Table 78. Asynchronous multiplexed PSRAM/NOR write timings(1)(2)

| Symbol | Parameter | Min | Max | Unit |
|---|---|---|---|---|
| tw(NE) | FSMC_NE low time | 4THCLK-0.5 | 4THCLK+3 | ns |
| tv(NWE_NE) | FSMC_NEx low to FSMC_NWE low | THCLK-0.5 | ns |  |
| tw(NWE) | FSMC_NWE low tim e | 2THCLK-0.5 | 2THCLK+3 | ns |
| th(NE_NWE) | FSMC_NWE high to FSMC_NE high hold time | THCLK | - | ns |
| tv(A_NE) | FSMC_NEx low to FSMC_A valid | - | 0 | ns |
| tv(NADV_NE) | FSMC_NEx low to FSMC_NADV low | 1 | 2 | ns |
| tw(NADV) | FSMC_NADV low time | THCLK- 2 | THCLK+ 1 | ns |
| th(AD_NADV) | FSMC_AD(address) valid hold time after FSMC_NADV high) | THCLK-2 | - | ns |
| th(A_NWE) | Address hold time after FSMC_NWE high | THCLK | - | ns |
| th(BL_NWE) | FSMC_BL hold time after FSMC_NWE high | THCLK-2 | - | ns |
| tv(BL_NE) | FSMC_NEx low to FSMC_BL valid | - | 1.5 | ns |
| tv(Data_NADV) | FSMC_NADV high to Data valid | - | THCLK-0.5 | ns |
| th(Data_NWE) | Data hold time after FSMC_NWE high | THCLK | - | ns |


1. C<sub>L</sub> = 30 pF.

148/206
DS8626 Rev 12

--- Page 149 ---

**STM32F405xx, STM32F407xx**
**Electrical characteristics**

2. Evaluated by characterization - not tested in production.

**Synchronous waveforms and timings**

*Figure 58* through *Figure 61* represent synchronous waveforms and *Table 80* through *Table 82* provide the corresponding timings. The results shown in these tables are obtained with the following FSMC configuration:

*   BurstAccessMode = FSMC_BurstAccessMode_Enable;
*   MemoryType = FSMC_MemoryType_CRAM;
*   WriteBurst = FSMC_WriteBurst_Enable;
*   CLKDivision = 1; (0 is not supported, see the STM32F40xxx/41xxx reference manual)
*   DataLatency = 1 for NOR flash; DataLatency = 0 for PSRAM

In all timing tables, the T<sub>HCLK</sub> is the HCLK clock period (with maximum FSMC_CLK = 60 MHz).

**Figure 58. Synchronous multiplexed NOR/PSRAM read timings**

ai14893g

ST
DS8626 Rev 12
149/206

--- Page 150 ---

**Electrical characteristics**
**STM32F405xx, STM32F407xx**

**Table 79. Synchronous multiplexed NOR/PSRAM read timings(1)(2)**

| Symbol | Parameter | Min | Max | Unit |
|---|---|---|---|---|
| tw(CLK) | FSMC_CLK period | 2THCLK | - | ns |
| td(CLKL-NExL) | FSMC_CLK low to FSMC_NEx low (x=0..2) | - | 0 | ns |
| td(CLKL-NExH) | FSMC_CLK low to FSMC_NEx high (x= 0...2) | 2 | - | ns |
| td(CLKL-NADVL) | FSMC_CLK low to FSMC_NADV low | - | 2 | ns |
| td(CLKL-NADVH) | FSMC_CLK low to FSMC_NADV high | 2 | - | ns |
| td(CLKL-AV) | FSMC_CLK low to FSMC_Ax valid (x=16...25) | - | 0 | ns |
| td(CLKL-AIV) | FSMC_CLK low to FSMC_Ax invalid (x=16...25) | 0 | - | ns |
| td(CLKL-NOEL) | FSMC_CLK low to FSMC_NOE low | - | 0 | ns |
| td(CLKL-NOEH) | FSMC_CLK low to FSMC_NOE high | 2 | - | ns |
| td(CLKL-ADV) | FSMC_CLK low to FSMC_AD[15:0] valid | - | 4.5 | ns |
| td(CLKL-ADIV) | FSMC_CLK low to FSMC_AD[15:0] invalid | 0 | - | ns |
| tsu(ADV-CLKH) | FSMC_A/D[15:0] valid data before FSMC_CLK high | 6 | - | ns |
| th(CLKH-ADV) | FSMC_A/D[15:0] valid data after FSMC_CLK high | 0 | - | ns |
| tsu(NWAIT-CLKH) | FSMC_NWAIT valid before FSMC_CLK high | 4 | - | ns |
| th(CLKH-NWAIT) | FSMC_NWAIT valid after FSMC_CLK high | 0 | - | ns |


1. C<sub>L</sub> = 30 pF.
2. Evaluated by characterization - not tested in production.

150/206
DS8626 Rev 12
ST

--- Page 151 ---

**STM32F405xx, STM32F407xx**
**Electrical characteristics**

Figure 59. Synchronous multiplexed PSRAM write timings

ai14992g

Table 80. Synchronous multiplexed PSRAM write timings(1)(2)

| Symbol | Parameter | Min | Max | Unit |
|---|---|---|---|---|
| $t_{w(CLK)}$ | FSMC_CLK period | $2T_{HCLK}$ | - | ns |
| $t_{d(CLKL-NExL)}$ | FSMC_CLK low to FSMC_NEx low (x=0..2) | - | 1 | ns |
| $t_{d(CLKL-NExH)}$ | FSMC_CLK low to FSMC_NEx high (x= 0...2) | 1 | - | ns |
| $t_{d(CLKL-NADVL)}$ | FSMC_CLK low to FSMC_NADV low | - | 0 | ns |
| $t_{d(CLKL-NADVH)}$ | FSMC_CLK low to FSMC_NADV high | 0 | - | ns |
| $t_{d(CLKL-AV)}$ | FSMC_CLK low to FSMC_Ax valid (x=16...25) | - | 0 | ns |
| $t_{d(CLKL-AIV)}$ | FSMC_CLK low to FSMC_Ax invalid (x=16...25) | 8 | - | ns |
| $t_{d(CLKL-NWEL)}$ | FSMC_CLK low to FSMC_NWE low | - | 0.5 | ns |
| $t_{d(CLKL-NWEH)}$ | FSMC_CLK low to FSMC_NWE high | 0 | - | ns |
| $t_{d(CLKL-ADIV)}$ | FSMC_CLK low to FSMC_AD[15:0] invalid | 0 | - | ns |
| $t_{d(CLKL-DATA)}$ | FSMC_A/D[15:0] valid data after FSMC_CLK low | - | 3 | ns |


ST
DS8626 Rev 12
151/206

--- Page 152 ---

Electrical characteristics
STM32F405xx, STM32F407xx

Table 80. Synchronous multiplexed PSRAM write timings(1)(2) (continued)

| Symbol | Parameter | Min | Max | Unit |
|---|---|---|---|---|
| td(CLKL-NBLH) | FSMC_CLK low to FSMC_NBL high | 0 | - | ns |
| tsu(NWAIT-CLKH) | FSMC_NWAIT valid before FSMC_CLK high | 4 | - | ns |
| th(CLKH-NWAIT) | FSMC_NWAIT valid after FSMC_CLK high | 0 | - | ns |


1. CL = 30 pF.
2. Evaluated by characterization - not tested in production.

Figure 60. Synchronous non-multiplexed NOR/PSRAM read timings

ai14894f

152/206
DS8626 Rev 12
ST

--- Page 153 ---

**STM32F405xx, STM32F407xx**

**Electrical characteristics**

**Table 81. Synchronous non-multiplexed NOR/PSRAM read timings(1)(2)**

| Symbol | Parameter | Min | Max | Unit |
|---|---|---|---|---|
| tw(CLK) | FSMC_CLK period | 2THCLK -0.5 | - | ns |
| td(CLKL-NExL) | FSMC_CLK low to FSMC_NEx low (x=0...2) | - | 0.5 | ns |
| td(CLKL-NExH) | FSMC_CLK low to FSMC_NEx high (x= 0...2) | 0 | - | ns |
| td(CLKL-NADVL) | FSMC_CLK low to FSMC_NADV low | - | 2 | ns |
| td(CLKL-NADVH) | FSMC_CLK low to FSMC_NADV high | 3 | - | ns |
| td(CLKL-AV) | FSMC_CLK low to FSMC_Ax valid (x=16...25) | - | 0 | ns |
| td(CLKL-AIV) | FSMC_CLK low to FSMC_Ax invalid (x=16...25) | 2 | - | ns |
| td(CLKL-NOEL) | FSMC_CLK low to FSMC_NOE low | - | 0.5 | ns |
| td(CLKL-NOEH) | FSMC_CLK low to FSMC_NOE high | 1.5 | - | ns |
| tsu(DV-CLKH) | FSMC_D[15:0] valid data before FSMC_CLK high | 6 | - | ns |
| th(CLKH-DV) | FSMC_D[15:0] valid data after FSMC_CLK high | 3 | - | ns |
| tsu(NWAIT-CLKH) | FSMC_NWAIT valid before FSMC_CLK high | 4 | - | ns |
| th(CLKH-NWAIT) | FSMC_NWAIT valid after FSMC_CLK high | 0 | - | ns |


1. C<sub>L</sub> = 30 pF.
2. Evaluated by characterization - not tested in production.

ST
DS8626 Rev 12
153/206

--- Page 154 ---

Electrical characteristics
STM32F405xx, STM32F407xx

Figure 61. Synchronous non-multiplexed PSRAM write timings

ai14993g

Table 82. Synchronous non-multiplexed PSRAM write timings(1)(2)

| Symbol | Parameter | Min | Max | Unit |
|---|---|---|---|---|
| $t_w(CLK)$ | FSMC_CLK period | $2T_{HCLK}$ | - | ns |
| $t_d(CLKL-NExL)$ | FSMC_CLK low to FSMC_NEx low (x=0..2) | - | 1 | ns |
| $t_d(CLKL-NExH)$ | FSMC_CLK low to FSMC_NEx high (x= 0...2) | 1 | - | ns |
| $t_d(CLKL-NADVL)$ | FSMC_CLK low to FSMC_NADV low | - | 7 | ns |
| $t_d(CLKL-NADVH)$ | FSMC_CLK low to FSMC_NADV high | 6 | - | ns |
| $t_d(CLKL-AV)$ | FSMC_CLK low to FSMC_Ax valid (x=16...25) | - | 0 | ns |
| $t_d(CLKL-AIV)$ | FSMC_CLK low to FSMC_Ax invalid (x=16...25) | 6 | - | ns |
| $t_d(CLKL-NWEL)$ | FSMC_CLK low to FSMC_NWE low | - | 1 | ns |
| $t_d(CLKL-NWEH)$ | FSMC_CLK low to FSMC_NWE high | 2 | - | ns |
| $t_d(CLKL-Data)$ | FSMC_D[15:0] valid data after FSMC_CLK low | - | 3 | ns |
| $t_d(CLKL-NBLH)$ | FSMC_CLK low to FSMC_NBL high | 3 | - | ns |
| $t_{su}(NWAITV-CLKH)$ | FSMC_NWAIT valid before FSMC_CLK high | 4 | - | ns |
| $t_h(CLKH-NWAIT)$ | FSMC_NWAIT valid after FSMC_CLK high | 0 | - | ns |


1. $C_L$ = 30 pF.
2. Evaluated by characterization - not tested in production.

154/206
DS8626 Rev 12
ST

--- Page 155 ---

STM32F405xx, STM32F407xx
Electrical characteristics

**PC Card/CompactFlash controller waveforms and timings**

*Figure 62* through *Figure 67* represent synchronous waveforms, and *Table 83* and *Table 84* provide the corresponding timings. The results shown in this table are obtained with the following FSMC configuration:

*   COM.FSMC_SetupTime = 0x04;
*   COM.FSMC_WaitSetupTime = 0x07;
*   COM.FSMC_HoldSetupTime = 0x04;
*   COM.FSMC_HiZSetupTime = 0x00;
*   ATT.FSMC_SetupTime = 0x04;
*   ATT.FSMC_WaitSetupTime = 0x07;
*   ATT.FSMC_HoldSetupTime = 0x04;
*   ATT.FSMC_HiZSetupTime = 0x00;
*   IO.FSMC_SetupTime = 0x04;
*   IO.FSMC_WaitSetupTime = 0x07;
*   IO.FSMC_HoldSetupTime = 0x04;
*   IO.FSMC_HiZSetupTime = 0x00;
*   TCLRSetupTime = 0;
*   TARSetupTime = 0.

In all timing tables, the T_HCLK is the HCLK clock period.

**Figure 62. PC Card/CompactFlash controller waveforms for common memory read access**

FSMC_NCE4_2(1)
FSMC_NCE4_1
FSMC_A[10:0]
FSMC_NREG
FSMC_NIOWR
FSMC_NIORD
FSMC_NWE
FSMC_NOE
FSMC_D[15:0]

t_v(NCEx-A)
t_h(NCEx-AI)
t_d(NREG-NCEx)
t_d(NIORD-NCEx)
t_h(NCEx-NREG)
t_h(NCEx-NIORD)
t_h(NCEx-NIOWR)
t_d(NCE4_1-NOE)
t_w(NOE)
t_su(D-NOE)
t_h(NOE-D)

ai14895b

1. FSMC_NCE4_2 remains high (inactive during 8-bit access.

STi
DS8626 Rev 12
155/206

--- Page 156 ---

Electrical characteristics
STM32F405xx, STM32F407xx

Figure 63. PC Card/CompactFlash controller waveforms for common memory write access

FSMC_NCE4_1
FSMC_NCE4_2 High
FSMC_A[10:0]
FSMC_NREG
FSMC_NIOWR
FSMC_NIORD
FSMC_NWE
FSMC_NOE
SMC_D[15:0]

t_v(NCE4_1-A)
t_h(NCE4_1-AI)
t_d(NREG-NCE4_1)
t_d(NIORD-NCE4_1)
t_h(NCE4_1-NREG)
t_h(NCE4_1-NIORD)
t_h(NCE4_1-NIOWR)
t_d(NCE4_1-NWE)
t_w(NWE)
t_d(NWE-NCE4_1)
MEMxHIZ = 1
t_d(D-NWE)
t_v(NWE-D)
t_h(NWE-D)

ai14896

156/206
DS8626 Rev 12

--- Page 157 ---

STM32F405xx, STM32F407xx
Electrical characteristics

**Figure 64. PC Card/CompactFlash controller waveforms for attribute memory read access**

FSMC_NCE4_1
FSMC_NCE4_2 High
FSMC_A[10:0]
FSMC_NIOWR
FSMC_NIORD
FSMC_NREG
FSMC_NWE
FSMC_NOE
FSMC_D[15:0](1)

t_v(NCE4_1-A)
t_h(NCE4_1-AI)
t_d(NREG-NCE4_1)
t_h(NCE4_1-NREG)
t_d(NCE4_1-NOE)
t_w(NOE)
t_d(NOE-NCE4_1)
t_su(D-NOE)
t_h(NOE-D)

ai14897b

1. Only data bits 0...7 are read (bits 8...15 are disregarded).

ST
DS8626 Rev 12
157/206

--- Page 158 ---

**Electrical characteristics**
STM32F405xx, STM32F407xx

**Figure 65. PC Card/CompactFlash controller waveforms for attribute memory write access**

FSMC_NCE4_1
FSMC_NCE4_2 High
FSMC_A[10:0]
FSMC_NIOWR
FSMC_NIORD
FSMC_NREG
FSMC_NWE
FSMC_NOE
FSMC_D[7:0](1)

$t_v(NCE4\_1-A)$
$t_h(NCE4\_1-A)$
$t_d(NREG-NCE4\_1)$
$t_h(NCE4\_1-NREG)$
$t_d(NCE4\_1-NWE)$
$t_w(NWE)$
$t_d(NWE-NCE4\_1)$
$t_v(NWE-D)$

1. Only data bits 0...7 are driven (bits 8...15 remains Hi-Z).

**Figure 66. PC Card/CompactFlash controller waveforms for I/O space read access**

FSMC_NCE4_1
FSMC_NCE4_2
FSMC_A[10:0]
FSMC_NREG
FSMC_NWE
FSMC_NOE
FSMC_NIOWR
FSMC_NIORD
FSMC_D[15:0]

$t_v(NCEX-A)$
$t_h(NCE4\_1-A)$
$t_d(NIORD-NCE4\_1)$
$t_w(NIORD)$
$t_{su}(D-NIORD)$
$t_d(NIORD-D)$

158/206
DS8626 Rev 12

--- Page 159 ---

**STM32F405xx, STM32F407xx**
Electrical characteristics

**Figure 67. PC Card/CompactFlash controller waveforms for I/O space write access**

ai14900c

**Table 83. Switching characteristics for PC Card/CF read and write cycles in attribute/common space(1)(2)**

| Symbol | Parameter | Min | Max | Unit |
|---|---|---|---|---|
| tv(NCEx-A) | FSMC_Ncex low to FSMC_Ay valid | - | 0 | ns |
| th(NCEx-AI) | FSMC_NCEx high to FSMC_Ax invalid | 4 | - | ns |
| td(NREG-NCEx) | FSMC_NCEx low to FSMC_NREG valid | - | 3.5 | ns |
| th(NCEx-NREG) | FSMC_NCEx high to FSMC_NREG invalid | THCLK+4 | - | ns |
| td(NCEx-NWE) | FSMC_NCEx low to FSMC_NWE low | - | 5THCLK+0.5 | ns |
| td(NCEx-NOE) | FSMC_NCEx low to FSMC_NOE low | - | 5THCLK+0.5 | ns |
| tw(NOE) | FSMC_NOE low width | 8THCLK-1 | 8THCLK+1 | ns |
| td(NOE_NCEx) | FSMC_NOE high to FSMC_NCEx high | 5THCLK+2.5 | - | ns |
| tsu (D-NOE) | FSMC_D[15:0] valid data before FSMC_NOE high | 4.5 | - | ns |
| th(NOE-D) | FSMC_NOE high to FSMC_D[15:0] invalid | 3 | - | ns |
| tw(NWE) | FSMC_NWE low width | 8THCLK-0.5 | 8THCLK+ 3 | ns |
| td(NWE_NCEx) | FSMC_NWE high to FSMC_NCEx high | 5THCLK-1 | - | ns |
| td(NCEx-NWE) | FSMC_NCEx low to FSMC_NWE low | - | 5THCLK+ 1 | ns |
| tv(NWE-D) | FSMC_NWE low to FSMC_D[15:0] valid | - | 0 | ns |
| th (NWE-D) | FSMC_NWE high to FSMC_D[15:0] invalid | 8THCLK -1 | - | ns |
| td (D-NWE) | FSMC_D[15:0] valid before FSMC_NWE high | 13THCLK -1 | - | ns |


1. C<sub>L</sub> = 30 pF.
2. Evaluated by characterization - not tested in production.

ST
DS8626 Rev 12
159/206

--- Page 160 ---

Electrical characteristics
STM32F405xx, STM32F407xx

Table 84. Switching characteristics for PC Card/CF read and write cycles
in I/O space(1)(2)

| Symbol | Parameter | Min | Max | Unit |
|---|---|---|---|---|
| tw(NIOWR) | FSMC_NIOWR low width | 8THCLK−1 | - | ns |
| tv(NIOWR-D) | FSMC_NIOWR low to FSMC_D[15:0] valid | - | 5THCLK−1 | ns |
| th(NIOWR-D) | FSMC_NIOWR high to FSMC_D[15:0] invalid | 8THCLK−2 | - | ns |
| td(NCE4_1-NIOWR) | FSMC_NCE4_1 low to FSMC_NIOWR valid | - | 5THCLK+ 2.5 | ns |
| th(NCEX-NIOWR) | FSMC_NCEX high to FSMC_NIOWR invalid | 5THCLK−1.5 | - | ns |
| td(NIORD-NCEX) | FSMC_NCEX low to FSMC_NIORD valid | - | 5THCLK+ 2 | ns |
| th(NCEX-NIORD) | FSMC_NCEX high to FSMC_NIORD) valid | 5THCLK−1.5 | - | ns |
| tw(NIORD) | FSMC_NIORD low width | 8THCLK−0.5 | - | ns |
| tsu(D-NIORD) | FSMC_D[15:0] valid before FSMC_NIORD high | 9 | - | ns |
| td(NIORD-D) | FSMC_D[15:0] valid after FSMC_NIORD high | 0 | - | ns |


1. C<sub>L</sub> = 30 pF.
2. Evaluated by characterization - not tested in production.

**NAND controller waveforms and timings**

*Figure 68* and *Figure 69* represent synchronous waveforms, and *Table 85* and *Table 86* provide the corresponding timings. The results shown in this table are obtained with the following FSMC configuration:

*   COM.FSMC_SetupTime = 0x01;
*   COM.FSMC_WaitSetupTime = 0x03;
*   COM.FSMC_HoldSetupTime = 0x02;
*   COM.FSMC_HiZSetupTime = 0x01;
*   ATT.FSMC_SetupTime = 0x01;
*   ATT.FSMC_WaitSetupTime = 0x03;
*   ATT.FSMC_HoldSetupTime = 0x02;
*   ATT.FSMC_HiZSetupTime = 0x01;
*   Bank = FSMC_Bank_NAND;
*   MemoryDataWidth = FSMC_MemoryDataWidth_16b;
*   ECC = FSMC_ECC_Enable;
*   ECCPageSize = FSMC_ECCPageSize_512Bytes;
*   TCLRSetupTime = 0;
*   TARSetupTime = 0.

In all timing tables, the T<sub>HCLK</sub> is the HCLK clock period.

160/206
DS8626 Rev 12

--- Page 161 ---

STM32F405xx, STM32F407xx
Electrical characteristics

Figure 68. NAND controller waveforms for read access

FMC_NCEx
ALE (FMC_A17)
CLE (FMC_A16)
FMC_NWE
FMC_NOE (NRE)
FMC_D[y:0]
t_d(ALE-NOE)
t_h(NOe-ALE)
t_w(NOe)
t_su(D-NOE)
t_h(NOe-D)
MSv73150V1

1. y = 7 or 15 depending on the NAND flash memory interface.

Figure 69. NAND controller waveforms for write access

FMC_NCEx
ALE (FMC_A17)
CLE (FMC_A16)
FMC_NWE
FMC_NOE (NRE)
FMC_D[y:0]
t_d(ALE-NWE)
t_w(NWE)
t_h(NWE-ALE)
t_d(D-NWE)
t_w(NWE-D)
t_h(NWE-D)
MSv73151V1

1. y = 7 or 15 depending on the NAND flash memory interface.

Table 85. Switching characteristics for NAND flash read cycles(1)

| Symbol | Parameter | Min | Max | Unit |
|---|---|---|---|---|
| t_w(NOe) | FSMC_NOE low width | 4T_HCLK - 0.5 | 4T_HCLK + 3 | ns |
| t_su(D-NOe) | FSMC_D[15-0] valid data before FSMC_NOE high | 10 | - | ns |
| t_h(NOe-D) | FSMC_D[15-0] valid data after FSMC_NOE high | 0 | - | ns |
| t_d(ALE-NOe) | FSMC_ALE valid before FSMC_NOE low | - | 3T_HCLK | ns |
| t_h(NOe-ALE) | FSMC_NWE high to FSMC_ALE invalid | 3T_HCLK - 2 | - | ns |


1. C_L = 30 pF.

ST
DS8626 Rev 12
161/206

--- Page 162 ---

Electrical characteristics
STM32F405xx, STM32F407xx

Table 86. Switching characteristics for NAND flash write cycles(1)

| Symbol | Parameter | Min | Max | Unit |
|---|---|---|---|---|
| tW(NWE) | FSMC_NWE low width | 4THCLK−1 | 4THCLK+ 3 | ns |
| tv(NWE-D) | FSMC_NWE low to FSMC_D[15-0] valid | - | 0 | ns |
| th(NWE-D) | FSMC_NWE high to FSMC_D[15-0] invalid | 3THCLK−2 | - | ns |
| td(D-NWE) | FSMC_D[15-0] valid before FSMC_NWE high | 5THCLK−3 | - | ns |
| td(ALE-NWE) | FSMC_ALE valid before FSMC_NWE low | - | 3THCLK | ns |
| th(NWE-ALE) | FSMC_NWE high to FSMC_ALE invalid | 3THCLK−2 | - | ns |


1. C<sub>L</sub> = 30 pF.

6.3.27 **Camera interface (DCMI) timing specifications**

Unless otherwise specified, the parameters given in **Table 87** for DCMI are derived from tests performed under the ambient temperature, f<sub>HCLK</sub> frequency and V<sub>DD</sub> supply voltage summarized in **Table 13**, with the following configuration:
* PCK polarity: falling
* VSYNC and HSYNC polarity: high
* Data format: 14 bits

Figure 70. DCMI timing diagram

MS32414V2

Table 87. DCMI characteristics(1)

| Symbol | Parameter | Min | Max | Unit |
|---|---|---|---|---|
| - | Frequency ratio DCMI_PIXCLK/fHCLK | - | 0.4 | - |
| DCMI_PIXCLK | Pixel clock input | - | 54 | MHz |
| Dpixel | Pixel clock input duty cycle | 30 | 70 | % |


162/206
DS8626 Rev 12
ST

--- Page 163 ---

STM32F405xx, STM32F407xx
Electrical characteristics

Table 87. DCMI characteristics(1) (continued)

| Symbol | Parameter | Min | Max | Unit |
|---|---|---|---|---|
| tsu(DATA) | Data input setup time | 2.5 | - | ns |
| th(DATA) | Data hold time | 1 | - |  |
| tsu(HSYNC),tsu(VSYNC) | HSYNC/VSYNC input setup time | 2 | - |  |
| th(HSYNC),th(VSYNC) | HSYNC/VSYNC input hold time | 0.5 | - |  |


1. Evaluated by characterization - not tested in production.


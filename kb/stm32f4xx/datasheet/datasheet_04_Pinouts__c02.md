**Figure 16. STM32F40xxx UFBGA176 ballout**

|  | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| A | PE3 | PE2 | PE1 | PE0 | PB8 | PB5 | PG14 | PG13 | PB4 | PB3 | PD7 | PC12 | PA15 | PA14 | PA13 |
| B | PE4 | PE5 | PE6 | PB9 | PB7 | PB6 | PG15 | PG12 | PG11 | PG10 | PD6 | PD0 | PC11 | PC10 | PA12 |
| C | VBAT | PI7 | PI6 | PI5 | VDD | PDR_ON | VDD | PG9 | PD5 | PD1 | PI3 | PI2 | PA11 |  |  |
| D | PC13 | PI8 | PI9 | PI4 | VSS | BOOT0 | VSS | PD4 | PD3 | PD2 | PH15 | PI1 | PA10 |  |  |
| E | PC14 | PF0 | PI10 | PI11 |  |  |  |  |  | PH13 | PH14 | PI0 | PA9 |  |  |
| F | PC15 | VSS | VDD | PH2 | VSS VSS VSS VSS VSS | VSS | VCAP_2 | PC9 | PA8 |  |  |  |  |  |  |
| G | PH0 | VSS | VDD | PH3 | VSS VSS VSS VSS VSS | VSS | VDD | PC8 | PC7 |  |  |  |  |  |  |
| H | PH1 | PF2 | PF1 | PH4 | VSS VSS VSS VSS VSS | VSS | VDD | PG8 | PC6 |  |  |  |  |  |  |
| J | NRST | PF3 | PF4 | PH5 | VSS VSS VSS VSS VSS | VDD | PG7 | PG6 |  |  |  |  |  |  |  |
| K | PF7 | PF6 | PF5 | VDD | VSS VSS VSS VSS VSS | PH12 | PG5 | PG4 | PG3 |  |  |  |  |  |  |
| L | PF10 | PF9 | PF8 | BYPASS_REG |  |  |  |  |  | PH11 | PH10 | PD15 | PG2 |  |  |
| M | VSSA | PC0 | PC1 | PC2 | PC3 | PB2 | PG1 | VSS | VCAP_1 | PH6 | PH8 | PH9 | PD14 | PD13 |  |
| N | VREF- | PA1 | PA0 | PA4 | PC4 | PF13 | PG0 | VDD | PE13 | PH7 | PD12 | PD11 | PD10 |  |  |
| P | VREF+ | PA2 | PA6 | PA5 | PC5 | PF12 | PF15 | PE8 | PE9 | PE11 | PE14 | PB12 | PB13 | PD9 | PD8 |
| R | VDDA | PA3 | PA7 | PB1 | PB0 | PF11 | PF14 | PE7 | PE10 | PE12 | PE15 | PB10 | PB11 | PB14 | PB15 |


ai18497b

1. This figure shows the package top view.

ST
DS8626 Rev 12
47/206

--- Page 48 ---

Pinouts and pin description
STM32F405xx, STM32F407xx

Figure 17. STM32F40xxx WLCSP90 ballouts

| A | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 |
|---|---|---|---|---|---|---|---|---|---|---|
| B | VBAT | PC13 | PDR_ON | BOOT0 | PB4 | PD7 | PD4 | PC12 | PA14 | VDD |
| C | PC14 | PC15 | VDD | PB7 | PB3 | PD6 | PD2 | PA15 | PI1 | VCAP_2 |
| D | PA0 | VSS | PB9 | PB6 | PD5 | PD1 | PC11 | PI0 | PA12 | PA11 |
| E | PC2 | BYPASS_REG | PB8 | PB5 | PB0 | PD0 | PC10 | PA13 | PA10 | PA9 |
| F | PC0 | PC3 | VSS | VDD | VSS | VDD | PC9 | PC8 | PC7 |  |
| G | PH0 | PH1 | PA1 | VDD | PE10 | PE14 | VCAP_1 | PC6 | PD14 | PD15 |
| H | NRST | VDDA | PA5 | PB0 | PE7 | PE13 | PE15 | PD10 | PD12 | PD11 |
| J | VSSA | PA3 | PA6 | PB1 | PE8 | PE12 | PB10 | PD9 | PD8 | PB15 |
|  | PA2 | PA4 | PA7 | PB2 | PE9 | PE11 | PB11 | PB12 | PB14 | PB13 |


MS30402V1

1. This figure shows the package bump view.

Table 6. Legend/abbreviations used in the pinout table

| Name | Abbreviation | Definition |
|---|---|---|
| Pin name | Unless otherwise specified in brackets below the pin name, the pin function during and after reset is the same as the actual pin name |  |
| Pin type | S | Supply pin |
| I | Input only pin |  |
| I/O | Input / output pin |  |
| I/O structure | FT | 5 V tolerant I/O |
| TTa | 3.3 V tolerant I/O directly connected to ADC |  |
| B | Dedicated BOOT0 pin |  |
| RST | Bidirectional reset pin with embedded weak pull-up resistor |  |
| Notes | Unless otherwise specified by a note, all I/Os are set as floating inputs during and after reset |  |
| Alternate functions | Functions selected through GPIOx_AFR registers |  |
| Additional functions | Functions directly selected/enabled through peripheral registers |  |


48/206
DS8626 Rev 12

--- Page 49 ---

STM32F405xx, STM32F407xx
Pinouts and pin description

Table 7. STM32F40xxx pin and ball definitions(1)

| Pin number | Pin name(function after reset)(2) | Pin type | I / O structure | Notes | Alternate functions | Additional functions |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|
| LQFP64 | WLCSP90 | LQFP100 | LQFP144 | UFBGA176 | LQFP176 |  |  |  |  |  |
| - | 1 | A2 | 1 | PE2 | I/O | FT | - | TRACECLK/ FSMC_A23 / ETH_MII_TXD3 / EVENTOUT | - |  |
| - | 2 | A1 | 2 | PE3 | I/O | FT | - | TRACED0/FSMC_A19 / EVENTOUT | - |  |
| - | 3 | B1 | 3 | PE4 | I/O | FT | - | TRACED1/FSMC_A20 / DCMI_D4/ EVENTOUT | - |  |
| - | 4 | B2 | 4 | PE5 | I/O | FT | - | TRACED2 / FSMC_A21 / TIM9_CH1 / DCMI_D6 / EVENTOUT | - |  |
| - | 5 | B3 | 5 | PE6 | I/O | FT | - | TRACED3 / FSMC_A22 / TIM9_CH2 / DCMI_D7 / EVENTOUT | - |  |
| 1 | A10 | 6 | C1 | 6 | VBAT | S | - |  |  |  |
| - | D2 | 7 | PI8 | I/O | FT | (3)(4) | - | RTC_TAMP1, RTC_TAMP2, RTC_TS |  |  |
| 2 | A9 | 7 | D1 | 8 | PC13 | I/O | FT | (3)(4) | - | RTC_OUT, RTC_TAMP1, RTC_TS |
| 3 | B10 | 8 | E1 | 9 | PC14/OSC32_IN (PC14) | I/O | FT | (3)(4) | - | OSC32_IN(5) |
| 4 | B9 | 9 | F1 | 10 | PC15/ OSC32_OUT (PC15) | I/O | FT | (3)(4) | - | OSC32_OUT(5) |
| - | D3 | 11 | PI9 | I/O | FT | - | CAN1_RX / EVENTOUT | - |  |  |
| - | E3 | 12 | PI10 | I/O | FT | - | ETH_MII_RX_ER / EVENTOUT | - |  |  |
| - | E4 | 13 | PI11 | I/O | FT | - | OTG_HS_ULPI_DIR / EVENTOUT | - |  |  |
| - | F2 | 14 | VSS | S | - |  |  |  |  |  |
| - | F3 | 15 | VDD | S | - |  |  |  |  |  |
| - | 10 | E2 | 16 | PF0 | I/O | FT | - | FSMC_A0 / I2C2_SDA / EVENTOUT | - |  |


ST
DS8626 Rev 12
49/206

--- Page 50 ---

**Pinouts and pin description**
STM32F405xx, STM32F407xx

Table 7. STM32F40xxx pin and ball definitions(1) (continued)

| Pin number | LQFP64 | WL CSP90 | LQFP100 | LQFP144 | UFBGA176 | LQFP176 | Pin name (function after reset)(2) | Pin type | I / O structure | Notes | Alternate functions | Additional functions |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| - | 11 | H3 | 17 | PF1 | I/O | FT | - | FSMC_A1 / I2C2_SCL / EVENTOUT | - |  |  |  |
| - | 12 | H2 | 18 | PF2 | I/O | FT | - | FSMC_A2 / I2C2_SMBAL / EVENTOUT | - |  |  |  |
| - | 13 | J2 | 19 | PF3 | I/O | FT | (5) | FSMC_A3/EVENTOUT | ADC3_IN9 |  |  |  |
| - | 14 | J3 | 20 | PF4 | I/O | FT | (5) | FSMC_A4/EVENTOUT | ADC3_IN14 |  |  |  |
| - | 15 | K3 | 21 | PF5 | I/O | FT | (5) | FSMC_A5/EVENTOUT | ADC3_IN15 |  |  |  |
| C9 | 10 | 16 | G2 | 22 | VSS | - | S | - |  |  |  |  |
| B8 | 11 | 17 | G3 | 23 | VDD | - | S | - |  |  |  |  |
| - | 18 | K2 | 24 | PF6 | I/O | FT | (5) | FSMC_NIORD/ TIM10_CH1 / EVENTOUT | ADC3_IN4 |  |  |  |
| - | 19 | K1 | 25 | PF7 | I/O | FT | (5) | FSMC_NREG/ TIM11_CH1/FSMC_NREG/ EVENTOUT | ADC3_IN5 |  |  |  |
| - | 20 | L3 | 26 | PF8 | I/O | FT | (5) | FSMC_NIOWR/ TIM13_CH1 / EVENTOUT | ADC3_IN6 |  |  |  |
| - | 21 | L2 | 27 | PF9 | I/O | FT | (5) | TIM14_CH1 / FSMC_CD/ EVENTOUT | ADC3_IN7 |  |  |  |
| - | 22 | L1 | 28 | PF10 | I/O | FT | (5) | FSMC_INTR/ EVENTOUT | ADC3_IN8 |  |  |  |
| F10 | 12 | 23 | G1 | 29 | PH0/OSC_IN (PH0) | I/O | FT | - | EVENTOUT | OSC_IN(5) |  |  |
| F9 | 13 | 24 | H1 | 30 | PH1/OSC_OUT (PH1) | I/O | FT | - | EVENTOUT | OSC_OUT(5) |  |  |
| G10 | 14 | 25 | J1 | 31 | NRST | I/O | RST | - |  |  |  |  |
| E10 | 15 | 26 | M2 | 32 | PC0 | I/O | FT | (5) | OTG_HS_ULPI_STP/ EVENTOUT | ADC123_IN10 |  |  |
| - | 16 | 27 | M3 | 33 | PC1 | I/O | FT | (5) | ETH_MDC/ EVENTOUT | ADC123_IN11 |  |  |
| D10 | 17 | 28 | M4 | 34 | PC2 | I/O | FT | (5) | SPI2_MISO / OTG_HS_ULPI_DIR / ETH_MII_TXD2 / I2S2ext_SD/ EVENTOUT | ADC123_IN12 |  |  |


--- Page 51 ---

STM32F405xx, STM32F407xx
Pinouts and pin description

Table 7. STM32F40xxx pin and ball definitions(1) (continued)

| Pin number | Pin name(function after reset)(2) | Pin type | I/O structure | Notes | Alternate functions | Additional functions |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|
| LQFP64 | WLCSP90 | LQFP100 | LQFP144 | UFBGA176 | LQFP176 |  |  |  |  |  |  |
| 11 | E9 | 18 | 29 | M5 | 35 | PC3 | I/O | FT | (5) | SPI2_MOSI / I2S2_SD / OTG_HS_ULPI_NXT / ETH_MII_TX_CLK/ EVENTOUT | ADC123_IN13 |
| - | 19 | 30 | - | 36 | VDD | S | - |  |  |  |  |
| 12 | H10 | 20 | 31 | M1 | 37 | VSSA | S | - |  |  |  |
| - | N1 | - | VREF- | S | - |  |  |  |  |  |  |
| - | 21 | 32 | P1 | 38 | VREF+ | S | - |  |  |  |  |
| 13 | G9 | 22 | 33 | R1 | 39 | VDDA | S | - |  |  |  |
| 14 | C10 | 23 | 34 | N3 | 40 | PA0/WKUP (PA0) | I/O | FT | (6) | USART2_CTS/ UART4_TX/ ETH_MII_CRS / TIM2_CH1_ETR/ TIM5_CH1 / TIM8_ETR/ EVENTOUT | ADC123_IN0/WK UP(5) |
| 15 | F8 | 24 | 35 | N2 | 41 | PA1 | I/O | FT | (5) | USART2_RTS / UART4_RX/ ETH_RMII_REF_CLK / ETH_MII_RX_CLK / TIM5_CH2/TIM2_CH2/ EVENTOUT | ADC123_IN1 |
| 16 | J10 | 25 | 36 | P2 | 42 | PA2 | I/O | FT | (5) | USART2_TX/TIM5_CH3 / TIM9_CH1 / TIM2_CH3 / ETH_MDIO/ EVENTOUT | ADC123_IN2 |
| - | F4 | 43 | PH2 | I/O | FT | - | ETH_MII_CRS/EVENTOUT | - |  |  |  |
| - | G4 | 44 | PH3 | I/O | FT | - | ETH_MII_COL/EVENTOUT | - |  |  |  |
| - | H4 | 45 | PH4 | I/O | FT | - | I2C2_SCL / OTG_HS_ULPI_NXT/ EVENTOUT | - |  |  |  |
| - | J4 | 46 | PH5 | I/O | FT | - | I2C2_SDA/ EVENTOUT | - |  |  |  |
| 17 | H9 | 26 | 37 | R2 | 47 | PA3 | I/O | FT | (5) | USART2_RX/TIM5_CH4 / TIM9_CH2 / TIM2_CH4 / OTG_HS_ULPI_D0 / ETH_MII_COL/ EVENTOUT | ADC123_IN3 |
| 18 | E5 | 27 | 38 | - | VSS | S | - |  |  |  |  |


DS8626 Rev 12
51/206

--- Page 52 ---

Pinouts and pin description
STM32F405xx, STM32F407xx

Table 7. STM32F40xxx pin and ball definitions(1) (continued)

| Pin number | Pin name(function after reset)(2) | Pin type | I / O structure | Notes | Alternate functions | Additional functions |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|
| LQFP64 | WLCSP90 | LQFP100 | LQFP144 | UFBA176 | LQFP176 |  |  |  |  |  |  |
| - | D9 | - | L4 | 48 | BYPASS_REG | I | FT | - |  |  |  |
| 19 | E4 | 28 | 39 | K4 | 49 | VDD | S | - |  |  |  |
| 20 | J9 | 29 | 40 | N4 | 50 | PA4 | I/O | TTa | (5) | SPI1_NSS / SPI3_NSS / USART2_CK / DCMI_HSYNC / OTG_HS_SOF / I2S3_WS / EVENTOUT | ADC12_IN4 / DAC_OUT1 |
| 21 | G8 | 30 | 41 | P4 | 51 | PA5 | I/O | TTa | (5) | SPI1_SCK / OTG_HS_ULPI_CK / TIM2_CH1_ETR / TIM8_CH1N / EVENTOUT | ADC12_IN5 / DAC_OUT2 |
| 22 | H8 | 31 | 42 | P3 | 52 | PA6 | I/O | FT | (5) | SPI1_MISO / TIM8_BKIN / TIM13_CH1 / DCMI_PIXCLK / TIM3_CH1 / TIM1_BKIN / EVENTOUT | ADC12_IN6 |
| 23 | J8 | 32 | 43 | R3 | 53 | PA7 | I/O | FT | (5) | SPI1_MOSI / TIM8_CH1N / TIM14_CH1 / TIM3_CH2 / ETH_MII_RX_DV / TIM1_CH1N / ETH_RMII_CRS_DV / EVENTOUT | ADC12_IN7 |
| 24 | - | 33 | 44 | N5 | 54 | PC4 | I/O | FT | (5) | ETH_RMII_RX_D0 / ETH_MII_RX_D0 / EVENTOUT | ADC12_IN14 |
| 25 | - | 34 | 45 | P5 | 55 | PC5 | I/O | FT | (5) | ETH_RMII_RX_D1 / ETH_MII_RX_D1 / EVENTOUT | ADC12_IN15 |
| 26 | G7 | 35 | 46 | R5 | 56 | PB0 | I/O | FT | (5) | TIM3_CH3 / TIM8_CH2N / OTG_HS_ULPI_D1 / ETH_MII_RXD2 / TIM1_CH2N / EVENTOUT | ADC12_IN8 |
| 27 | H7 | 36 | 47 | R4 | 57 | PB1 | I/O | FT | (5) | TIM3_CH4 / TIM8_CH3N / OTG_HS_ULPI_D2 / ETH_MII_RXD3 / TIM1_CH3N / EVENTOUT | ADC12_IN9 |
| 28 | J7 | 37 | 48 | M6 | 58 | PB2 / BOOT1 (PB2) | I/O | FT | - | EVENTOUT | - |


52/206
DS8626 Rev 12

--- Page 53 ---

STM32F405xx, STM32F407xx
Pinouts and pin description

Table 7. STM32F40xxx pin and ball definitions(1) (continued)

| Pin number | Pin name(function after reset)(2) | Pin type | I/O structure | Notes | Alternate functions | Additional functions |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|
| LQFP64 | WLCSP90 | LQFP100 | LQFP144 | UFBA176 | LQFP176 |  |  |  |  |  |  |
| - | 49 | R6 | 59 | PF11 | I/O | FT | - | DCMI_D12/ EVENTOUT | - |  |  |
| - | 50 | P6 | 60 | PF12 | I/O | FT | - | FSMC_A6/ EVENTOUT | - |  |  |
| - | 51 | M8 | 61 | VSS | S | - |  |  |  |  |  |
| - | 52 | N8 | 62 | VDD | S | - |  |  |  |  |  |
| - | 53 | N6 | 63 | PF13 | I/O | FT | - | FSMC_A7/ EVENTOUT | - |  |  |
| - | 54 | R7 | 64 | PF14 | I/O | FT | - | FSMC_A8/ EVENTOUT | - |  |  |
| - | 55 | P7 | 65 | PF15 | I/O | FT | - | FSMC_A9/ EVENTOUT | - |  |  |
| - | 56 | N7 | 66 | PG0 | I/O | FT | - | FSMC_A10/ EVENTOUT | - |  |  |
| - | 57 | M7 | 67 | PG1 | I/O | FT | - | FSMC_A11/ EVENTOUT | - |  |  |
| - | G6 | 38 | 58 | R8 | 68 | PE7 | I/O | FT | - | FSMC_D4/TIM1_ETR/ EVENTOUT | - |
| - | H6 | 39 | 59 | P8 | 69 | PE8 | I/O | FT | - | FSMC_D5/TIM1_CH1N/ EVENTOUT | - |
| - | J6 | 40 | 60 | P9 | 70 | PE9 | I/O | FT | - | FSMC_D6/TIM1_CH1/ EVENTOUT | - |
| - | 61 | M9 | 71 | VSS | S | - |  |  |  |  |  |
| - | 62 | N9 | 72 | VDD | S | - |  |  |  |  |  |
| - | F6 | 41 | 63 | R9 | 73 | PE10 | I/O | FT | - | FSMC_D7/TIM1_CH2N/ EVENTOUT | - |
| - | J5 | 42 | 64 | P10 | 74 | PE11 | I/O | FT | - | FSMC_D8/TIM1_CH2/ EVENTOUT | - |
| - | H5 | 43 | 65 | R10 | 75 | PE12 | I/O | FT | - | FSMC_D9/TIM1_CH3N/ EVENTOUT | - |
| - | G5 | 44 | 66 | N11 | 76 | PE13 | I/O | FT | - | FSMC_D10/TIM1_CH3/ EVENTOUT | - |
| - | F5 | 45 | 67 | P11 | 77 | PE14 | I/O | FT | - | FSMC_D11/TIM1_CH4/ EVENTOUT | - |
| - | G4 | 46 | 68 | R11 | 78 | PE15 | I/O | FT | - | FSMC_D12/TIM1_BKIN/ EVENTOUT | - |


ST
DS8626 Rev 12
53/206

--- Page 54 ---

Pinouts and pin description
STM32F405xx, STM32F407xx

Table 7. STM32F40xxx pin and ball definitions(1) (continued)

| Pin number | Pin name(function after reset)(2) | Pin type | I/O structure | Notes | Alternate functions | Additional functions |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|
| LQFP64 | WLCSP90 | LQFP100 | LQFP144 | UFBGA176 | LQFP176 |  |  |  |  |  |  |
| 29 | H4 | 47 | 69 | R12 | 79 | PB10 | I/O | FT | - | SPI2_SCK / I2S2_CK / I2C2_SCL / USART3_TX / OTG_HS_ULPI_D3 / ETH_MII_RX_ER / TIM2_CH3/ EVENTOUT | - |
| 30 | J4 | 48 | 70 | R13 | 80 | PB11 | I/O | FT | - | I2C2_SDA/USART3_RX/ OTG_HS_ULPI_D4 / ETH_RMII_TX_EN/ ETH_MII_TX_EN / TIM2_CH4/ EVENTOUT | - |
| 31 | F4 | 49 | 71 | M10 | 81 | VCAP_1 | S | - |  |  |  |
| 32 | - | 50 | 72 | N10 | 82 | VDD | S | - |  |  |  |
| - | M11 | 83 | PH6 | I/O | FT | - | I2C2_SMBA / TIM12_CH1 / ETH_MII_RXD2/ EVENTOUT | - |  |  |  |
| - | N12 | 84 | PH7 | I/O | FT | - | I2C3_SCL / ETH_MII_RXD3/ EVENTOUT | - |  |  |  |
| - | M12 | 85 | PH8 | I/O | FT | - | I2C3_SDA / DCMI_HSYNC/ EVENTOUT | - |  |  |  |
| - | M13 | 86 | PH9 | I/O | FT | - | I2C3_SMBA / TIM12_CH2/ DCMI_D0/ EVENTOUT | - |  |  |  |
| - | L13 | 87 | PH10 | I/O | FT | - | TIM5_CH1 / DCMI_D1/ EVENTOUT | - |  |  |  |
| - | L12 | 88 | PH11 | I/O | FT | - | TIM5_CH2 / DCMI_D2/ EVENTOUT | - |  |  |  |
| - | K12 | 89 | PH12 | I/O | FT | - | TIM5_CH3 / DCMI_D3/ EVENTOUT | - |  |  |  |
| - | H12 | 90 | VSS | S | - |  |  |  |  |  |  |
| - | J12 | 91 | VDD | S | - |  |  |  |  |  |  |


54/206
DS8626 Rev 12

--- Page 55 ---

STM32F2405xx, STM32F2407xx
Pinouts and pin description

Table 7. STM32F240xxx pin and ball definitions(1) (continued)

| Pin number | Pin name(function afterreset)(2) | Pin type | I / O structure | Notes | Alternate functions | Additionalfunctions |
|---|---|---|---|---|---|---|
| LQFP64 | WLCSP90 | LQFP100 | LQFP144 | UFBGA176 | LQFP176 |  |
| 33 | PB12 | I/O | FT | - | SPI2_NSS / I2S2_WS /I2C2_SMBAL / TIM1_BKIN /USART3_CK / TIM1_BKIN /CAN2_RX /OTG_HS_ULPI_D6/ETH_RMII_TXD0/ETH_MII_TXD0/OTG_HS_ID / EVENTOUT | - |
| 34 | PB13 | I/O | FT | - | SPI2_SCK / I2S2_CK /USART3_CTS/TIM1_CH1N /CAN2_TX /OTG_HS_ULPI_D6 /ETH_RMII_TXD1 /ETH_MII_TXD1 /OTG_HS_VBUS | - |
| 35 | PB14 | I/O | FT | - | SPI2_MISO / TIM1_CH2N /TIM12_CH1 /OTG_HS_DM/USART3_RTS /TIM8_CH2N/I2S2ext_SD/EVENTOUT | - |
| 36 | PB15 | I/O | FT | - | SPI2_MOSI / I2S2_SD/TIM1_CH3N / TIM8_CH3N/TIM12_CH2 /OTG_HS_DP / EVENTOUT | RTC_REFIN |
| - | PD8 | I/O | FT | - | FSMC_13 / USART3_TX/EVENTOUT | - |
| - | PD9 | I/O | FT | - | FSMC_14 / USART3_RX/EVENTOUT | - |
| - | PD10 | I/O | FT | - | FSMC_D15 / USART3_CK/EVENTOUT | - |
| - | PD11 | I/O | FT | - | FSMC_CLE /FSMC_A16/USART3_CTS/EVENTOUT | - |
| - | PD12 | I/O | FT | - | FSMC_ALE/FSMC_A17/TIM4_CH1 /USART3_RTS /EVENTOUT | - |


--- Page 56 ---

**Pinouts and pin description**
**STM32F405xx, STM32F407xx**

Table 7. STM32F40xxx pin and ball definitions(1) (continued)

| Pin number | Pin name(function after reset)(2) | Pin type | I / O structure | Notes | Alternate functions | Additional functions |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|
| LQFP64 | WLCSP90 | LQFP100 | LQFP144 | UFBGA176 | LQFP176 |  |  |  |  |  |  |
| - | 60 | 82 | M15 | 101 | PD13 | I/O | FT | - | FSMC_A18/TIM4_CH2/EVENTOUT | - |  |
| - | 83 | - | 102 | VSS | S | - |  |  |  |  |  |
| - | 84 | J13 | 103 | VDD | S | - |  |  |  |  |  |
| - | F2 | 61 | 85 | M14 | 104 | PD14 | I/O | FT | - | FSMC_D0/TIM4_CH3/EVENTOUT/EVENTOUT | - |
| - | F1 | 62 | 86 | L14 | 105 | PD15 | I/O | FT | - | FSMC_D1/TIM4_CH4/EVENTOUT | - |
| - | 87 | L15 | 106 | PG2 | I/O | FT | - | FSMC_A12/EVENTOUT | - |  |  |
| - | 88 | K15 | 107 | PG3 | I/O | FT | - | FSMC_A13/EVENTOUT | - |  |  |
| - | 89 | K14 | 108 | PG4 | I/O | FT | - | FSMC_A14/EVENTOUT | - |  |  |
| - | 90 | K13 | 109 | PG5 | I/O | FT | - | FSMC_A15/EVENTOUT | - |  |  |
| - | 91 | J15 | 110 | PG6 | I/O | FT | - | FSMC_INT2/EVENTOUT | - |  |  |
| - | 92 | J14 | 111 | PG7 | I/O | FT | - | FSMC_INT3/USART6_CK/EVENTOUT | - |  |  |
| - | 93 | H14 | 112 | PG8 | I/O | FT | - | USART6_RTS / ETH_PPS_OUT/EVENTOUT | - |  |  |
| - | 94 | G12 | 113 | VSS | S | - |  |  |  |  |  |
| - | 95 | H13 | 114 | VDD | S | - |  |  |  |  |  |
| 37 | F3 | 63 | 96 | H15 | 115 | PC6 | I/O | FT | - | I2S2_MCK / TIM8_CH1/SDIO_D6 / USART6_TX / DCMI_D0/TIM3_CH1/EVENTOUT | - |
| 38 | E1 | 64 | 97 | G15 | 116 | PC7 | I/O | FT | - | I2S3_MCK / TIM8_CH2/SDIO_D7 / USART6_RX / DCMI_D1/TIM3_CH2/EVENTOUT | - |
| 39 | E2 | 65 | 98 | G14 | 117 | PC8 | I/O | FT | - | TIM8_CH3/SDIO_D0 /TIM3_CH3/USART6_CK / DCMI_D2/EVENTOUT | - |


56/206
DS8626 Rev 12

--- Page 57 ---

STM32F405xx, STM32F407xx
Pinouts and pin description

Table 7. STM32F40xxx pin and ball definitions(1) (continued)

| Pin number | Pin name(function after reset)(2) | Pin type | I / O structure | Notes | Alternate functions | Additional functions |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|
| LQFP64 | WLCSP90 | LQFP100 | LQFP144 | UFBGA176 | LQFP176 |  |  |  |  |  |  |
| 40 | E3 | 66 | 99 | F14 | 118 | PC9 | I/O | FT | - | I2S_CKIN/ MCO2 / TIM8_CH4/SDIO_D1 / /I2C3_SDA/ DCMI_D3 / TIM3_CH4/ EVENTOUT | - |
| 41 | D1 | 67 | 100 | F15 | 119 | PA8 | I/O | FT | - | MCO1 / USART1_CK/ TIM1_CH1/ I2C3_SCL/ OTG_FS_SOF/ EVENTOUT | - |
| 42 | D2 | 68 | 101 | E15 | 120 | PA9 | I/O | FT | - | USART1_TX/ TIM1_CH2 / I2C3_SMBA / DCMI_D0/ EVENTOUT | OTG_FS_VBUS |
| 43 | D3 | 69 | 102 | D15 | 121 | PA10 | I/O | FT | - | USART1_RX/ TIM1_CH3/ OTG_FS_ID/DCMI_D1/ EVENTOUT | - |
| 44 | C1 | 70 | 103 | C15 | 122 | PA11 | I/O | FT | - | USART1_CTS / CAN1_RX / TIM1_CH4 / OTG_FS_DM/ EVENTOUT | - |
| 45 | C2 | 71 | 104 | B15 | 123 | PA12 | I/O | FT | - | USART1_RTS / CAN1_TX/ TIM1_ETR/ OTG_FS_DP/ EVENTOUT | - |
| 46 | D4 | 72 | 105 | A15 | 124 | PA13 (JTMS-SWDIO) | I/O | FT | - | JTMS-SWDIO/ EVENTOUT | - |
| 47 | B1 | 73 | 106 | F13 | 125 | VCAP_2 | S | - |  |  |  |
| - | E7 | 74 | 107 | F12 | 126 | VSS | S | - |  |  |  |
| 48 | E6 | 75 | 108 | G13 | 127 | VDD | S | - |  |  |  |
| - | E12 | 128 | PH13 | I/O | FT | - | TIM8_CH1N / CAN1_TX/ EVENTOUT | - |  |  |  |
| - | E13 | 129 | PH14 | I/O | FT | - | TIM8_CH2N / DCMI_D4/ EVENTOUT | - |  |  |  |
| - | D13 | 130 | PH15 | I/O | FT | - | TIM8_CH3N / DCMI_D11/ EVENTOUT | - |  |  |  |
| - | C3 | - | E14 | 131 | PI0 | I/O | FT | - | TIM5_CH4 / SPI2_NSS / I2S2_WS / DCMI_D13/ EVENTOUT | - |  |
| - | B2 | - | D14 | 132 | PI1 | I/O | FT | - | SPI2_SCK / I2S2_CK / DCMI_D8/ EVENTOUT | - |  |


DS8626 Rev 12
57/206

--- Page 58 ---

Pinouts and pin description
STM32F405xx, STM32F407xx

Table 7. STM32F40xxx pin and ball definitions(1) (continued)

| Pin number | Pin name (function after reset)(2) | Pin type | I/O structure | Notes | Alternate functions | Additional functions |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|
| LQFP64 | WLCSP90 | LQFP100 | LQFP144 | UFBGA176 | LQFP176 |  |  |  |  |  |  |
| - | C14 | 133 | PI2 | I/O | FT | - | TIM8_CH4 /SPI2_MISO / DCMI_D9 / I2S2ext_SD/ EVENTOUT | - |  |  |  |
| - | C13 | 134 | PI3 | I/O | FT | - | TIM8_ETR / SPI2_MOSI / I2S2_SD / DCMI_D10/ EVENTOUT | - |  |  |  |
| - | D9 | 135 | Vss | S | - |  |  |  |  |  |  |
| - | C9 | 136 | VDD | S | - |  |  |  |  |  |  |
| 49 | A2 | 76 | 109 | A14 | 137 | PA14 (JTCK/SWCLK) | I/O | FT | - | JTCK-SWCLK/ EVENTOUT | - |
| 50 | B3 | 77 | 110 | A13 | 138 | PA15 (JTDI) | I/O | FT | - | JTDI/ SPI3_NSS/ I2S3_WS/TIM2_CH1_ETR / SPI1_NSS / EVENTOUT | - |
| 51 | D5 | 78 | 111 | B14 | 139 | PC10 | I/O | FT | - | SPI3_SCK / I2S3_CK/ UART4_TX/SDIO_D2 / DCMI_D8 / USART3_TX/ EVENTOUT | - |
| 52 | C4 | 79 | 112 | B13 | 140 | PC11 | I/O | FT | - | UART4_RX/ SPI3_MISO / SDIO_D3 / DCMI_D4/USART3_RX / I2S3ext_SD/ EVENTOUT | - |
| 53 | A3 | 80 | 113 | A12 | 141 | PC12 | I/O | FT | - | UART5_TX/SDIO_CK / DCMI_D9 / SPI3_MOSI /I2S3_SD / USART3_CK/ EVENTOUT | - |
| - | D6 | 81 | 114 | B12 | 142 | PD0 | I/O | FT | - | FSMC_D2/CAN1_RX/ EVENTOUT | - |
| - | C5 | 82 | 115 | C12 | 143 | PD1 | I/O | FT | - | FSMC_D3 / CAN1_TX/ EVENTOUT | - |
| 54 | B4 | 83 | 116 | D12 | 144 | PD2 | I/O | FT | - | TIM3_ETR/UART5_RX/ SDIO_CMD / DCMI_D11/ EVENTOUT | - |
| - | 84 | 117 | D11 | 145 | PD3 | I/O | FT | - | FSMC_CLK/ USART2_CTS/ EVENTOUT | - |  |


58/206
DS8626 Rev 12
ST

--- Page 59 ---

STM32F405xx, STM32F407xx
Pinouts and pin description

Table 7. STM32F40xxx pin and ball definitions(1) (continued)

| Pin number | Pin name(function after reset)(2) | Pin type | I/O structure | Notes | Alternate functions | Additional functions |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|
| LQFP64 | WLCSP90 | LQFP100 | LQFP144 | UFQFN76 | LQFP176 |  |  |  |  |  |  |
| - | A4 | 85 | 118 | D10 | 146 | PD4 | I/O | FT | - | FSMC_NOE/USART2_RTS/EVENTOUT | - |
| - | C6 | 86 | 119 | C11 | 147 | PD5 | I/O | FT | - | FSMC_NWE/USART2_TX/EVENTOUT | - |
| - | 120 | D8 | 148 | VSS | S | - |  |  |  |  |  |
| - | 121 | C8 | 149 | VDD | S | - |  |  |  |  |  |
| - | B5 | 87 | 122 | B11 | 150 | PD6 | I/O | FT | - | FSMC_NWAIT/USART2_RX/ EVENTOUT | - |
| - | A5 | 88 | 123 | A11 | 151 | PD7 | I/O | FT | - | USART2_CK/FSMC_NE1/FSMC_NCE2/ EVENTOUT | - |
| - | 124 | C10 | 152 | PG9 | I/O | FT | - | USART6_RX /FSMC_NE2/FSMC_NCE3/EVENTOUT | - |  |  |
| - | 125 | B10 | 153 | PG10 | I/O | FT | - | FSMC_NCE4_1/FSMC_NE3/ EVENTOUT | - |  |  |
| - | 126 | B9 | 154 | PG11 | I/O | FT | - | FSMC_NCE4_2 /ETH_MII_TX_EN/ETH_RMII_TX_EN/EVENTOUT | - |  |  |
| - | 127 | B8 | 155 | PG12 | I/O | FT | - | FSMC_NE4 /USART6_RTS/EVENTOUT | - |  |  |
| - | 128 | A8 | 156 | PG13 | I/O | FT | - | FSMC_A24 /USART6_CTS/ETH_MII_TXD0/ETH_RMII_TXD0/EVENTOUT | - |  |  |
| - | 129 | A7 | 157 | PG14 | I/O | FT | - | FSMC_A25 / USART6_TX/ETH_MII_TXD1/ETH_RMII_TXD1/EVENTOUT | - |  |  |
| - | E8 | - | 130 | D7 | 158 | VSS | S | - |  |  |  |
| - | F7 | - | 131 | C7 | 159 | VDD | S | - |  |  |  |
| - | 132 | B7 | 160 | PG15 | I/O | FT | - | USART6_CTS /DCMI_D13/ EVENTOUT | - |  |  |


ST
DS8626 Rev 12
59/206

--- Page 60 ---

Pinouts and pin description
STM32F405xx, STM32F407xx

Table 7. STM32F40xxx pin and ball definitions(1) (continued)

| Pin number | Pin name(function after reset)(2) | Pin type | I / O structure | Notes | Alternate functions | Additional functions |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|
| LQFP64 | WLCSP90 | LQFP100 | LQFP144 | UFQFN176 | LQFP176 |  |  |  |  |  |  |
| 55 | B6 | 89 | 133 | A10 | 161 | PB3(JTDO/TRACESWO) | I/O | FT | - | JTDO/ TRACESWO/SPI3_SCK / I2S3_CK /TIM2_CH2 / SPI1_SCK/EVENTOUT | - |
| 56 | A6 | 90 | 134 | A9 | 162 | PB4(NJTRST) | I/O | FT | - | NJTRST/ SPI3_MISO /TIM3_CH1 / SPI1_MISO /I2S3ext_SD/ EVENTOUT | - |
| 57 | D7 | 91 | 135 | A6 | 163 | PB5 | I/O | FT | - | I2C1_SMBA/ CAN2_RX /OTG_HS_ULPI_D7 /ETH_PPS_OUT/TIM3_CH2/ SPI1_MOSI/ SPI3_MOSI /DCMI_D10 / I2S3_SD/EVENTOUT | - |
| 58 | C7 | 92 | 136 | B6 | 164 | PB6 | I/O | FT | - | I2C1_SCL/ TIM4_CH1 /CAN2_TX /DCMI_D5/USART1_TX/EVENTOUT | - |
| 59 | B7 | 93 | 137 | B5 | 165 | PB7 | I/O | FT | - | I2C1_SDA / FSMC_NL /DCMI_VSYNC /USART1_RX/ TIM4_CH2/EVENTOUT | - |
| 60 | A7 | 94 | 138 | D6 | 166 | BOOT0 | I | B | - | VPP |  |
| 61 | D8 | 95 | 139 | A5 | 167 | PB8 | I/O | FT | - | TIM4_CH3/SDIO_D4/TIM10_CH1 / DCMI_D6 /ETH_MII_TXD3 /I2C1_SCL/ CAN1_RX/EVENTOUT | - |
| 62 | C8 | 96 | 140 | B4 | 168 | PB9 | I/O | FT | - | SPI2_NSS/ I2S2_WS /TIM4_CH4/ TIM11_CH1/SDIO_D5 / DCMI_D7 /I2C1_SDA / CAN1_TX/EVENTOUT | - |
| - | 97 | 141 | A4 | 169 | PE0 | I/O | FT | - | TIM4_ETR / FSMC_NBL0 /DCMI_D2/ EVENTOUT | - |  |
| - | 98 | 142 | A3 | 170 | PE1 | I/O | FT | - | FSMC_NBL1 / DCMI_D3/EVENTOUT | - |  |
| 63 | - | 99 | - | D5 | - | VSS | S | - |  |  |  |


60/206
DS8626 Rev 12

--- Page 61 ---

**STM32F405xx, STM32F407xx**
**Pinouts and pin description**

**Table 7. STM32F40xxx pin and ball definitions(1) (continued)**

| Pin number | Pin name(function after reset)(2) | Pin type | I/O structure | Notes | Alternate functions | Additional functions |  |  |  |
|---|---|---|---|---|---|---|---|---|---|
| LQFP64 | WLCSP90 | LQFP100 | LQFP144 | UFBGA176 | LQFP176 |  |  |  |  |
| - | A8 | - | 143 | C6 | 171 | PDR_ON | I | FT | - |
| 64 | A1 | 100 | 144 | C5 | 172 | VDD | S | - |  |
| - | D4 | 173 | PI4 | I/O | FT | - | TIM8_BKIN / DCMI_D5/EVENTOUT | - |  |
| - | C4 | 174 | PI5 | I/O | FT | - | TIM8_CH1 /DCMI_VSYNC/EVENTOUT | - |  |
| - | C3 | 175 | PI6 | I/O | FT | - | TIM8_CH2 / DCMI_D6/EVENTOUT | - |  |
| - | C2 | 176 | PI7 | I/O | FT | - | TIM8_CH3 / DCMI_D7/EVENTOUT | - |  |


1. UFBGA176 F6, F7, F8, F9, F10, G6, G7, G8, G9, G10, H6, H7, H8, H9, H10, J6, J7, J8, J9, J10, K6, K7, K8, K9 and K10 balls are connected to V<sub>SS</sub> for heat dissipation and package mechanical stability.
2. Function availability depends on the chosen device.
3. PC13, PC14, PC15 and PI8 are supplied through the power switch. Since the switch only sinks a limited amount of current (3 mA), the use of GPIOs PC13 to PC15 and PI8 in output mode is limited:
   - The speed should not exceed 2 MHz with a maximum load of 30 pF.
   - These I/Os must not be used as a current source (e.g. to drive an LED).
4. Main function after the first backup domain power-up. Later on, it depends on the contents of the RTC registers even after reset (because these registers are not reset by the main reset). For details on how to manage these I/Os, refer to the RTC register description sections in the STM32F4xx reference manual, available from the STMicroelectronics website: www.st.com.
5. FT = 5 V tolerant except when in analog mode or oscillator mode (for PC14, PC15, PH0 and PH1).
6. If the device is delivered in an UFBGA176 or WLCSP90 and the BYPASS_REG pin is set to VDD (Regulator off/internal reset ON mode), then PA0 is used as an internal Reset (active low).


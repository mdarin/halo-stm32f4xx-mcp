**Table 9. Alternate function mapping**

| Port | AF0 | AF1 | AF2 | AF3 | AF4 | AF5 | AF6 | AF7 | AF8 | AF9 | AF10 | AF11 | AF12 | AF13 | AF14 | AF15 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| SYS | TIM1/2 | TIM3/4/5 | TIM8/9/10 /11 | I2C1/2/3 | SP11/SP12/ I2S2/I2S2ext | SP13/I2Sext /I2S3 | USART1/2/3/ I2S3ext | UART4/5/ USART6 | CAN1/2 TIM12/13/ 14 | OTG_FS/ OTG_HS | ETH | FSMC/SDIO /OTG_FS | DCMI |  |  |  |
| Port A | PA0 | - | TIM2_CH1_ ETR | TIM 5_CH1 | TIM8_ETR | - | USART2_CTS | UART4_TX | - | ETH_MII_CRS | - | EVENTOUT |  |  |  |  |
| PA1 | - | TIM2_CH2 | TIM5_CH2 | - | USART2_RTS | UART4_RX | - | ETH_MII _RX_CLK ETH_RMII_REF _CLK | - | EVENTOUT |  |  |  |  |  |  |
| PA2 | - | TIM2_CH3 | TIM5_CH3 | TIM9_CH1 | - | USART2_TX | - | ETH_MDIO | - | EVENTOUT |  |  |  |  |  |  |
| PA3 | - | TIM2_CH4 | TIM5_CH4 | TIM9_CH2 | - | USART2_RX | - | OTG_HS_ULPI_ D0 | ETH_MII_COL | - | EVENTOUT |  |  |  |  |  |
| PA4 | - | SP11_NSS | SP13_NSS I2S3_WS | USART2_CK | - | OTG_HS_SOF | DCMI_ HSYNC | - | EVENTOUT |  |  |  |  |  |  |  |
| PA5 | - | TIM2_CH1_ ETR | - | TIM8_CH1N | - | SP11_SCK | - | OTG_HS_ULPI_ CK | - | EVENTOUT |  |  |  |  |  |  |
| PA6 | - | TIM1_BKIN | TIM3_CH1 | TIM8_BKIN | - | SP11_MISO | - | TIM13_CH1 | - | DCMI_ PIXCLK | - | EVENTOUT |  |  |  |  |
| PA7 | - | TIM1_CH1N | TIM3_CH2 | TIM8_CH1N | - | SP11_MOSI | - | TIM14_CH1 | - | ETH_MII_RX_DV ETH_RMII _CRS_DV | - | EVENTOUT |  |  |  |  |
| PA8 | MCO1 | TIM1_CH1 | - | I2C3_SCL | - | USART1_CK | - | OTG_FS_SOF | - | EVENTOUT |  |  |  |  |  |  |
| PA9 | - | TIM1_CH2 | - | I2C3_ SMBDA | - | USART1_TX | - | DCMI_D0 | - | EVENTOUT |  |  |  |  |  |  |
| PA10 | - | TIM1_CH3 | - | USART1_RX | - | OTG_FS_ID | - | DCMI_D1 | - | EVENTOUT |  |  |  |  |  |  |
| PA11 | - | TIM1_CH4 | - | USART1_CTS | - | CAN1_RX | OTG_FS_DM | - | EVENTOUT |  |  |  |  |  |  |  |
| PA12 | - | TIM1_ETR | - | USART1_RTS | - | CAN1_TX | OTG_FS_DP | - | EVENTOUT |  |  |  |  |  |  |  |
| PA13 | JTMS- SWDIO | - | EVENTOUT |  |  |  |  |  |  |  |  |  |  |  |  |  |
| PA14 | JTCK- SWCLK | - | EVENTOUT |  |  |  |  |  |  |  |  |  |  |  |  |  |
| PA15 | JTDI | TIM 2_CH1 TIM 2_ETR | - | SP11_NSS | SP13_NSS/ I2S3_WS | - | EVENTOUT |  |  |  |  |  |  |  |  |  |


--- Page 65 ---

STMicroelectronics
DS8626 Rev 12
65/206
Table 9. Alternate function mapping (continued)
STM32F405xx, STM32F407xx
Pinouts and pin description

| Port | AF0 | AF1 | AF2 | AF3 | AF4 | AF5 | AF6 | AF7 | AF8 | AF9 | AF10 | AF11 | AF12 | AF13 | AF14 | AF15 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| SYS | TIM1/2 | TIM3/4/5 | TIM8/9/10 /11 | I2C1/2/3 | SP11/SP12/ I2S2/I2S2ext | SP13/I2Sext /I2S3 | USART1/2/3/ I2S3ext | UART4/5/ USART6 | CAN1/2 TIM12/13/ 14 | OTG_FS/ OTG_HS | ETH | FMC/SDIO /OTG_FS | DCMI |  |  |  |
| Port B | PB0 | - | TIM1_CH2N | TIM3_CH3 | TIM8_CH2N | - | OTG_HS_ULPI_D1 | ETH_MII_RXD2 | - | EVENTOUT |  |  |  |  |  |  |
| PB1 | - | TIM1_CH3N | TIM3_CH4 | TIM8_CH3N | - | OTG_HS_ULPI_D2 | ETH_MII_RXD3 | - | EVENTOUT |  |  |  |  |  |  |  |
| PB2 | - | EVENTOUT |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| PB3 | JTD0/ TRACES WO | TIM2_CH2 | - | SP11_SCK | SP13_SCK I2S3_CK | - | EVENTOUT |  |  |  |  |  |  |  |  |  |
| PB4 | NJTRST | - | TIM3_CH1 | - | SP11_MISO | SP13_MISO | I2S3ext_SD | - | EVENTOUT |  |  |  |  |  |  |  |
| PB5 | - | TIM3_CH2 | - | I2C1_SMB A | SP11_MOSI | SP13_MOSI I2S3_SD | - | CAN2_RX | OTG_HS_ULPI_D7 | ETH_PPS_OUT | - | DCMI_D10 | - | EVENTOUT |  |  |
| PB6 | - | TIM4_CH1 | - | I2C1_SCL | - | USART1_TX | - | CAN2_TX | - | DCMI_D5 | - | EVENTOUT |  |  |  |  |
| PB7 | - | TIM4_CH2 | - | I2C1_SDA | - | USART1_RX | - | FSMC_NL | DCMI_VSYNC C | - | EVENTOUT |  |  |  |  |  |
| PB8 | - | TIM4_CH3 | TIM10_CH1 | I2C1_SCL | - | CAN1_RX | - | ETH_MII_TXD3 | SDIO_D4 | DCMI_D6 | - | EVENTOUT |  |  |  |  |
| PB9 | - | TIM4_CH4 | TIM11_CH1 | I2C1_SDA | SP12_NSS I2S2_WS | - | CAN1_TX | - | SDIO_D5 | DCMI_D7 | - | EVENTOUT |  |  |  |  |
| PB10 | - | TIM2_CH3 | - | I2C2_SCL | SP12_SCK I2S2_CK | - | USART3_TX | - | OTG_HS_ULPI_D3 | ETH_MII_RX_ER | - | EVENTOUT |  |  |  |  |
| PB11 | - | TIM2_CH4 | - | I2C2_SDA | - | USART3_RX | - | OTG_HS_ULPI_D4 | ETH_MII_TX_EN ETH_RMII_TX_EN | - | EVENTOUT |  |  |  |  |  |
| PB12 | - | TIM1_BKIN | - | I2C2_SMBA | SP12_NSS I2S2_WS | - | USART3_CK | - | CAN2_RX | OTG_HS_ULPI_D5 | ETH_MII_TXD0 ETH_RMII_TXD0 | OTG_HS_ID | - | EVENTOUT |  |  |
| PB13 | - | TIM1_CH1N | - | SP12_SCK I2S2_CK | - | USART3_CTS | - | CAN2_TX | OTG_HS_ULPI_D6 | ETH_MII_TXD1 ETH_RMII_TXD1 | - | EVENTOUT |  |  |  |  |
| PB14 | - | TIM1_CH2N | - | TIM8_CH2N | - | SP12_MISO | I2S2ext_SD | USART3_RTS | - | TIM12_CH1 | - | OTG_HS_DM | - | EVENTOUT |  |  |
| PB15 | RTC_REFIN | TIM1_CH3N | - | TIM8_CH3N | - | SP12_MOSI I2S2_SD | - | TIM12_CH2 | - | OTG_HS_DP | - | EVENTOUT |  |  |  |  |


--- Page 66 ---

66/206
DS8626 Rev 12


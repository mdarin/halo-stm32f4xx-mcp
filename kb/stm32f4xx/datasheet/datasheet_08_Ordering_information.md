# 8 Ordering information

Example: STM32 F 405 R E T 6 xxx

**Device family**
STM32 = Arm-based 32-bit microcontroller

**Product type**
F = general-purpose

**Device subfamily**
405 = STM32F40xxx, connectivity
407= STM32F40xxx, connectivity, camera interface, Ethernet

**Pin count**
R = 64 pins
O = 90 pins
V = 100 pins
Z = 144 pins
I = 176 pins

**Flash memory size**
E = 512 Kbytes of flash memory
G = 1024 Kbytes of flash memory

**Package**
T = LQFP
H = UFBGA
Y = WLCSP

**Temperature range**
6 = Industrial temperature range, -40 to 85 °C.
7 = Industrial temperature range, -40 to 105 °C.

**Options**
xxx = programmed parts
TR = tape and reel

For a list of available options (speed, package, etc.) or for further information on any aspect of this device, please contact your nearest ST sales office.

186/206
DS8626 Rev 12

--- Page 187 ---

STM32F405xx, STM32F407xx
Application block diagrams

# Appendix A Application block diagrams

## A.1 USB OTG full speed (FS) interface solutions

Figure 85. USB controller configured as peripheral-only and used in Full speed mode

STM32F4xx
VDD
5V to VDD
Volatge regulator (1)
OSC_IN
PA11//PB14
PA12/PB15
OSC_OUT
VBUS
DM
DP
VSS
USB Std-B connector
MS19000V5

1. External voltage regulator only needed when building a VBUS powered device.
2. The same application can be developed using the OTG HS in FS mode to achieve enhanced performance thanks to the large Rx/Tx FIFO and to a dedicated DMA controller.

Figure 86. USB controller configured as host-only and used in full speed mode

STM32F4xx
GPIO
GPIO+IRQ
OSC_IN
PA11//PB14
PA12/PB15
OSC_OUT
VDD
EN
Overcurrent
Current limiter
power switch(1)
5 V Pwr
VBUS
DM
DP
VSS
USB Std-A connector
MS19001V4

1. The current limiter is required only if the application has to support a VBUS powered device. A basic power switch can be used if 5 V are available on the application board.
2. The same application can be developed using the OTG HS in FS mode to achieve enhanced performance thanks to the large Rx/Tx FIFO and to a dedicated DMA controller.

ST
DS8626 Rev 12
187/206

--- Page 188 ---

Application block diagrams
STM32F405xx, STM32F407xx

Figure 87. USB controller configured in dual mode and used in full speed mode

1. External voltage regulator only needed when building a VBUS powered device.
2. The current limiter is required only if the application has to support a VBUS powered device. A basic power switch can be used if 5 V are available on the application board.
3. The ID pin is required in dual role only.
4. The same application can be developed using the OTG HS in FS mode to achieve enhanced performance thanks to the large Rx/Tx FIFO and to a dedicated DMA controller.

188/206
DS8626 Rev 12

--- Page 189 ---

STM32F405xx, STM32F407xx
Application block diagrams

### A.2 USB OTG high speed (HS) interface solutions

Figure 88. USB controller configured as peripheral, host, or dual-mode and used in high speed mode

STM32F4xx
USB HS OTG Ctrl
FS PHY
DP
DM
not connected
ULPI
ULPI_CLK
ULPI_D[7:0]
ULPI_DIR
ULPI_STP
ULPI_NXT
PLL
XT1
24 or 26 MHz XT(1)
MCO1 or MCO2
XI
High speed OTG PHY
DP
DM
ID(2)
V_BUS
V_SS
USB connector

MS19005V2

1. It is possible to use MCO1 or MCO2 to save a crystal. It is however not mandatory to clock the STM32F40xxx with a 24 or 26 MHz crystal when using USB HS. The above figure only shows an example of a possible connection.
2. The ID pin is required in dual role only.

ST
DS8626 Rev 12
189/206

--- Page 190 ---

Application block diagrams
STM32F405xx, STM32F407xx

A.3 Ethernet interface solutions

Figure 89. MII mode using a 25 MHz crystal

STM32
MCU
Ethernet MAC 10/100
HCLK(1)
IEEE1588 PTP
Timer input trigger
TIM2
Timestamp comparator
XTAL 25 MHz
OSC
PLL
HCLK
MCO1/MCO2
MII_TX_CLK
MII_TX_EN
MII_TXD[3:0]
MII_CRS
MII_COL
MII_RX_CLK
MII_RXD[3:0]
MII_RX_DV
MII_RX_ER
MDIO
MDC
PPS_OUT(2)
PHY_CLK 25 MHz
MII = 15 pins
Ethernet PHY 10/100
MII + MDC = 17 pins
XT1
MS19968V1

1. f_HCLK must be greater than 25 MHz.
2. Pulse per second when using IEEE1588 PTP optional signal.

Figure 90. RMII with a 50 MHz oscillator

STM32
MCU
Ethernet MAC 10/100
HCLK(1)
IEEE1588 PTP
Timer input trigger
TIM2
Timestamp comparator
2.5 or 25 MHz
/2 or /20 synchronous
50 MHz
OSC 50 MHz
PLL
HCLK
RMII_TX_EN
RMII_TXD[1:0]
RMII_RXD[1:0]
RMII_CRX_DV
RMII_REF_CLK
MDIO
MDC
PHY_CLK
50 MHz
RMII = 7 pins
Ethernet PHY 10/100
RMII + MDC = 9 pins
XT1
50 MHz
MS19969V1

1. f_HCLK must be greater than 25 MHz.

190/206
DS8626 Rev 12
ST

--- Page 191 ---

STM32F405xx, STM32F407xx
Application block diagrams

Figure 91. RMII with a 25 MHz crystal and PHY with PLL

STM32F
MCU
Ethernet
MAC 10/100
HCLK(1)
IEEE1588 PTP
Timer
input
trigger
TIM2
Timestamp
comparator
/2 or /20
synchronous
2.5 or 25 MHz
50 MHz
XTAL
25 MHz
OSC
PLL
HCLK
MCO1/MCO2
RMII_TX_EN
RMII_TXD[1:0]
RMII_RXD[1:0]
RMII_CRX_DV
RMII_REF_CLK
MDIO
MDC
Ethernet
PHY 10/100
RMII
= 7 pins
REF_CLK
RMII + MDC
= 9 pins
PLL
PHY_CLK 25 MHz
XT1
MS19970V1

1. fHCLK must be greater than 25 MHz.
2. The 25 MHz (PHY_CLK) must be derived directly from the HSE oscillator, before the PLL block.

ST
DS8626 Rev 12
191/206

--- Page 192 ---

Important security notice
STM32F405xx, STM32F407xx


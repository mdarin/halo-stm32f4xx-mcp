> [RM0090 ch.16 §16.7] LTDC registers**

### 16.7.14 LTDC Layerx Control Register (LTDC_LxCR) (where x=1..2)**

**16.7.14 LTDC Layerx Control Register (LTDC_LxCR) (where x=1..2)**

Address offset: 0x84 + 0x80 x ( Layerx

-1), Layerx = 1 or 2

Reset value: 0x0000 0000

31 30 29 28 27 26 25 24

23 22 21 20 19 18 17 16

Reserved

15 14 13 12 11 10 9 8

7 6 5 4 3 2 1 0

| Reserved | CLUTEN | Reserved | COLKEN | LEN |
|---|---|---|---|---|
| rw |  |  |  |  |


Bits 31:5 Reserved, must be kept at reset value

Bit 4 CLUTEN : Color Look-Up Table Enable

This bit is set and cleared by software.

0: Color Look-Up Table disable

1: Color Look-Up Table enable

The CLUT is only meaningful for L8, AL44 and AL88 pixel format. Refer to Color Look-Up

Table (CLUT) on page 488

Bit 3 Reserved, must be kept at reset value

Bit 2 Reserved, must be kept at reset value

Bit 1 COLKEN : Color Keying Enable

This bit is set and cleared by software.

0: Color Keying disable

1: Color Keying enable

Bit 0 LEN : Layer Enable

This bit is set and cleared by software.

0: Layer disable

1: Layer enable



--- Page 113 ---

**RM0090**

**LCD-TFT controller (LTDC)**

**16.7.15**

**LTDC Layerx Window Horizontal Position Configuration Register**

**(LTDC_LxWHPCR) (where x=1..2)**

This register defines the Horizontal Position (first and last pixel) of the layer 1 or 2 window.

The first visible pixel of a line is the programmed value of LTDC_BPCR register.

**AHBP[10:0] bits + 1 in the**

The last visible pixel of a line is the programmed value of LTDC_AWCR register.

**AAW[10:0] bits in the**

Address offset: 0x88 + 0x80 x ( Layerx Reset value: 0x0000 0000

-1), Layerx = 1 or 2

31 30 29 28 27 26 25 24

23 22 21 20 19 18 17 16

| Reserved | WHSPPOS[11:0] |
|---|---|
| rw |  |


15 14 13 12 11 10 9 8

7 6 5 4 3 2 1 0

| Reserved | WHSTPOS[11:0] |
|---|---|
| rw |  |


Bits 31:28 Reserved, must be kept at reset value Bits 27:16 WHSPPOS[11:0]

: Window Horizontal Stop Position

These bits configure the last visible pixel of a line of the layer window.

The following condition must be respected: WHSPPOS[11:0] ≥

AHBP[10:0] bits + 1 (programmed in LTDC_BPCR register)

Bits 15:12 Reserved, must be kept at reset value Bits 11:0 WHSTPOS[11:0]

: Window Horizontal Start Position

These bits configure the first visible pixel of a line of the layer window.

The following condition must be respected: WHSTPOS[11:0] must be ≤ register).

AAW[10:0] bits (programmed in the LTDC_AWCR

Example:

The LTDC_BPCR register is configured to 0x000E0005(AHBP[11:0] is 0xE) and the LTDC_AWCR register is configured to 0x028E01E5(AAW[11:0] is 0x28E). To configure the horizontal position of a window size of 630x460, with horizontal start offset of 5 pixels in the

Active data area.

1.

Layer window first pixel: WHSTPOS[11:0] should be programmed to 0x14 (0xE+1+0x5) Layer window last pixel: WHSPPOS[11:0] should be programmed to 0x28A

2.



514

--- Page 114 ---

**LCD-TFT controller (LTDC)**

**RM0090**

**16.7.16**

**LTDC Layerx Window Vertical Position Configuration Register**

**(LTDC_LxWVPCR) (where x=1..2)**

This register defines the vertical position (first and last line) of the layer1 or 2 window.

The first visible line of a frame is the programmed value of register LTDC_BPCR register.

**AVBP[10:0] bits + 1 in the**

The last visible line of a frame is the programmed value of LTDC_AWCR register.

**AAH[10:0] bits in the**

Address offset: 0x8C + 0x80 x ( Layerx Reset value: 0x0000 0000

-1), Layerx = 1 or 2

31 30 29 28 27 26 25 24

23 22 21 20 19 18 17 16

| С Н | мешанная с маслом грязь Очистите масло от загрязнений |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|
| естаб | ильна | я рабо | та при |  | Взл | ом на | некото | рое вр | емя |  |


15 14 13 12 11 10 9 8

7 6 5 4 3 2 1 0

| С Скольжение | мешанная с маслом грязь Очистите масло от загрязнений |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|
|  |  |  |  |  | Уме | ньшит | е нагр | узку ил | и |  |


Bits 31:27 Reserved, must be kept at reset value Bits 26:16 WVSPPOS[10:0]

: Window Vertical Stop Position

These bits configures the last visible line of the layer window.

The following condition must be respected: WHSPPOS[11:0] must be ≥ register)

AVBP[10:0] bits + 1 (programmed in LTDC_BPCR

Bits 15:11 Reserved, must be kept at reset value Bits 10:0 WVSTPOS[10:0]

: Window Vertical Start Position

These bits configure the first visible line of the layer window.

The following condition must be respected: WHSTPOS[11:0] must be ≤

AAH[10:0] bits (programmed in the LTDC_AWCR register)

Example:

The LTDC_BPCR register is configured to 0x000E0005 (AVBP[10:0] is 0x5) and the LTDC_AWCR register is configured to 0x028E01E5 (AAH[10:0] is 0x1E5). To configure the vertical position of a window size of 630x460, with vertical start offset of 8 lines in the Active

data area:

1.

Layer window first line: WVSTPOS[10:0] should be programmed to 0xE (0x5 + 1 + 0x8) Layer window last line: WVSPPOS[10:0] should be programmed to 0x1DA

2.



--- Page 115 ---

**RM0090**

**LCD-TFT controller (LTDC)**

**16.7.17**

**LTDC Layerx Color Keying Configuration Register (LTDC_LxCKCR)**

**(where x=1..2)**

This register defines the color key value (RGB), which is used by the Color Keying.

Address offset: 0x90 + 0x80 x ( Layerx Reset value: 0x0000 0000

-1), Layerx = 1 or 2

31 30 29 28 27 26 25 24

23 22 21 20 19 18 17 16

| 7 10 7 | крышки Крепежный болт |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|
| кр | ышки |  |  |  |  |  |  |


15 14 13 12 11 10 9 8

7 6 5 4 3 2 1 0

| 9 | Крепежный болт |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|
| 1 2 1 | оп | орно | й плит | ы |  |  |  |  |


Bits 31:24 Reserved, must be kept at reset value Bits 23:16 CKRED[7:0] : Color Key Red value Bits 15:8 CKGREEN[7:0] : Color Key Green value Bits 7:0 CKBLUE[7:0] : Color Key Blue value

**16.7.18**

**LTDC Layerx Pixel Format Configuration Register (LTDC_LxPFCR)**

**(where x=1..2)**

This register defines the pixel format which is used for the stored data in the frame buffer of a layer. The pixel data is read from the frame buffer and then transformed to the internal

format 8888 (ARGB).

Address offset: 0x94 + 0x80 x ( Layerx Reset value: 0x0000 0000

-1), Layerx = 1 or 2

31 30 29 28 27 26 25 24

23 22 21 20 19 18 17 16

Reserved

15 14 13 12 11 10 9 8

7 6 5 4 3 2 1 0

| 1 3 Приводной д |  |  |
|---|---|---|
| вигате | ль |  |




514

--- Page 116 ---

**LCD-TFT controller (LTDC)**

**RM0090**

Bits 31:3 Reserved, must be kept at reset value Bits 2:0 PF[2:0] : Pixel Format

These bits configures the Pixel format 000: ARGB8888

001: RGB888

010: RGB565

011: ARGB1555

100: ARGB4444

101: L8 (8-Bit Luminance)

110: AL44 (4-Bit Alpha, 4-Bit Luminance) 111: AL88 (8-Bit Alpha, 8-Bit Luminance)

**16.7.19**

**LTDC Layerx Constant Alpha Configuration Register (LTDC_LxCACR)**

**(where x=1..2)**

This register defines the constant alpha value (divided by 255 by Hardware), which is used

in the alpha blending. Refer to LTDC_LxBFCR register. Address offset: 0x98 + 0x80 x ( Layerx Reset value: (Layerx -1) 0x0000 00FF

-1), Layerx = 1 or 2

31 30 29 28 27 26 25 24

23 22 21 20 19 18 17 16

Reserved

15 14 13 12 11 10 9 8

7 6 5 4 3 2 1 0

| Reserved | CONSTA[7:0] |
|---|---|
| rw |  |


Bits 31:8 Reserved, must be kept at reset value Bits 7:0 CONSTA[7:0] : Constant Alpha

These bits configure the Constant Alpha used for blending. The Constant Alpha is divided

by 255 by hardware.

Example: if the programmed Constant Alpha is 0xFF, the Constant Alpha value is

255/255=1

**16.7.20**

**LTDC Layerx Default Color Configuration Register (LTDC_LxDCCR)**

**(where x=1..2)**

This register defines the default color of a layer in the format ARGB. The default color is used outside the defined layer window or when a layer is disabled. The reset value of

0x00000000 defines a transparent black color. Address offset: 0x9C + 0x80 x ( Layerx Reset value: 0x0000 0000

-1), Layerx = 1 or 2

506/1749

RM0090 Rev 18

--- Page 117 ---

**RM0090**

**LCD-TFT controller (LTDC)**

31 30 29 28 27 26 25 24

23 22 21 20 19 18 17 16

| DCALPHA[7:0] | DCRED[7:0] |
|---|---|
| rw |  |


15 14 13 12 11 10 9 8

7 6 5 4 3 2 1 0

| DCGREEN[7:0] | DCBLUE[7:0] |
|---|---|
| rw |  |


Bits 31:24 DCALPHA[7:0] : Default Color Alpha

These bits configure the default alpha value Bits 23:16 DCRED[7:0] : Default Color Red

These bits configure the default red value Bits 15:8 DCGREEN[7:0] : Default Color Green

These bits configure the default green value Bits 7:0 DCBLUE[7:0] : Default Color Blue

These bits configure the default blue value



514

--- Page 118 ---

**LCD-TFT controller (LTDC)**

**RM0090**

**16.7.21**

**LTDC Layerx Blending Factors Configuration Register (LTDC_LxBFCR)**

**(where x=1..2)**

This register defines the blending factors F1 and F2. The general blending formula is: BC = BF1 x C + BF2 x Cs

• BC = Blended color

• BF1 = Blend Factor 1

• C = Current layer color

• BF2 = Blend Factor 2

• Cs = subjacent layers blended color Address offset: 0xA0 + 0x80 x ( Layerx

-1), Layerx = 1 or 2

Reset value: 0x0000 0607

31 30 29 28 27 26 25 24

23 22 21 20 19 18 17 16

Reserved

15 14 13 12 11 10 9 8

7 6 5 4 3 2 1 0

| Reserved | BF1[2:0] | Reserved | BF2[2:0] |
|---|---|---|---|
| rw |  |  |  |


Bits 31:11 Reserved, must be kept at reset value Bits 10:8 BF1[2:0] : Blending Factor 1

These bits select the blending factor F1 000: Reserved

001: Reserved

010: Reserved

011: Reserved

100: Constant Alpha

101: Reserved

110: Pixel Alpha x Constant Alpha 111:Reserved

Bits 7:3 Reserved, must be kept at reset value Bits 2:0 BF2[2:0] : Blending Factor 2

These bits select the blending factor F2 000: Reserved

001: Reserved

010: Reserved

011: Reserved

100: Reserved

101: 1 - Constant Alpha

110: Reserved

111: 1 - (Pixel Alpha x Constant Alpha)



--- Page 119 ---

**RM0090**

**LCD-TFT controller (LTDC)**

| Note: The Constant Alpha value, is th by hardware. Example: Only layer1 is enable BF2 configured to 1 - Constant Constant Alpha: The Constant Thus, the Constant Alpha value C: Current Layer Color is 128 Cs: Background color is 48 Layer1 is blended with the back BC = Constant Alpha x C + (1 - 16.7.22 LTDC Layerx Color Fram (where x=1..2) This register defines the color fr |  |  |
|---|---|---|
| e program | med value in the LxCA |  |
| d, BF1 co | nfigured to Constant Alp |  |
| Alpha |  |  |
| Alpha pro is 240/25 | grammed in the LxCACR 5 = 0.94 |  |
| ground c Constant | olor. Alpha) x Cs = 0.94 x 12 |  |
| e Buffe | r Address Register |  |
| ame buff | er start address which h |  |


= 0.94 x 128 + (1- 0.94) x 48 = 123.

where the pixel data of the top left pixel of a layer is stored in the frame buffer. Address offset: 0xAC + 0x80 x ( Layerx

-1), Layerx = 1 or 2

Reset value: 0x0000 0000

31 30 29 28 27 26 25 24

23 22 21 20 19 18 17 16

| CFBADD[31:0] |  |  |
|---|---|---|
| rw | rw rw rw | rw |


15 14 13 12 11 10 9 8

7 6 5 4 3 2 1 0



Bits 31:0 CFBADD[31:0]

: Color Frame Buffer Start Address

These bits defines the color frame buffer start address.

**16.7.23**

**LTDC Layerx Color Frame Buffer Length Register (LTDC_LxCFBLR)**

**(where x=1..2)**

This register defines the color frame buffer line length and pitch.

Address offset: 0xB0 + 0x80 x ( Layerx

-1), Layerx = 1 or 2

Reset value: 0x0000 0000

31 30 29 28 27 26 25 24

23 22 21 20 19 18 17 16



15 14 13 12 11 10 9 8

7 6 5 4 3 2 1 0





514

--- Page 120 ---

**LCD-TFT controller (LTDC) RM0090**

| No. Об 9 Ви | означени | е |
|---|---|---|
| нт |  |  |


These bits define the pitch which is the increment from the start of one line of pixels to the start of the next line in bytes.

Bits 15:13 Reserved, must be kept at reset value

Bits 12:0 CFBLL[12:0] : Color Frame Buffer Line Length

These bits define the length of one line of pixels in bytes + 3.

The line length is computed as follows: Active high width x number of bytes per pixel + 3.

Example:

• A frame buffer having the format RGB565 (2 bytes per pixel) and a width of 256 pixels

(total number of bytes per line is 256x2=512 bytes), where pitch = line length requires a value of 0x02000203 to be written into this register.

• A frame buffer having the format RGB888 (3 bytes per pixel) and a width of 320 pixels

(total number of bytes per line is 320x3=960), where pitch = line length requires a value of 0x03C003C3 to be written into this register.

> [RM0090 ch.16 §16.7] LTDC registers**

### 16.7.24 LTDC Layerx ColorFrame Buffer Line Number Register (LTDC_LxCFBLNR) (where x=1..2)**

**16.7.24 LTDC Layerx ColorFrame Buffer Line Number Register (LTDC_LxCFBLNR) (where x=1..2)**

This register defines the number of lines in the color frame buffer.

Address offset: 0xB4 + 0x80 x ( Layerx -1), Layerx = 1 or 2

Reset value: 0x0000 0000

31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16

Reserved

15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0



Bits 31:11 Reserved, must be kept at reset value

Bits 10:0 CFBLNBR[10:0] : Frame Buffer Line Number

These bits define the number of lines in the frame buffer which corresponds to the Active high width.

Note: The number of lines and line length settings define how much data is fetched per frame for every layer. If it is configured to less bytes than required, a FIFO underrun interrupt will be generated if enabled.

The start address and pitch settings on the other hand define the correct start of every line in memory.

510/1749 RM0090 Rev 18

--- Page 121 ---

**RM0090**

**LCD-TFT controller (LTDC)**

| Вилочный погрузчик этой серии оснащ который может осуществлять поворот колеса. Рулевое устройство в основно | ен элект на 180 ° | рическим прив или 360 ° на з |
|---|---|---|
| м состои | т из рулевого к |  |


| яю ние | щей ре рулев |
|---|---|
| осво |  |


Bits 31:24 CLUTADD[7:0] : CLUT Address

These bits configure the CLUT address (color position within the CLUT) of each RGB

value





514

--- Page 122 ---

**LCD-TFT controller (LTDC) RM0090**


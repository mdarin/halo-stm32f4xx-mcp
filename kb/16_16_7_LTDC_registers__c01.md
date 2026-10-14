> [RM0090 ch.16 §16.7] LTDC registers**

### 16.7 LTDC registers**

**16.7 LTDC registers**

**16.7.1**

**LTDC Synchronization Size Configuration Register (LTDC_SSCR)**

This register defines the number of Horizontal Synchronization pixels minus 1 and the

number of Vertical Synchronization lines minus 1. Refer to LTDC programmable parameters

Figure 82 and Section 16.4: for an example of configuration.

Address offset: 0x08

Reset value: 0x0000 0000

31 30 29 28 27 26 25 24

23 22 21 20 19 18 17 16

| Reserved | HSW[11:0] |
|---|---|
| rw |  |


15 14 13 12 11 10 9 8

7 6 5 4 3 2 1 0

| Reserved | VSH[10:0] |
|---|---|
| rw |  |


Bits 31:28 Reserved, must be kept at reset value

Bits 27:16 HSW[11:0]

: Horizontal Synchronization Width (in units of pixel clock period)

These bits define the number of Horizontal Synchronization pixel minus 1.

Bits 15:11 Reserved, must be kept at reset value

Bits 10:0 VSH[10:0]

: Vertical Synchronization Height (in units of horizontal scan line)

These bits define the vertical Synchronization height minus 1. It represents the

number

of horizontal synchronization lines.

> [RM0090 ch.16 §16.7] LTDC registers**

### 16.7.2 LTDC Back Porch Configuration Register (LTDC_BPCR)**

**16.7.2 LTDC Back Porch Configuration Register (LTDC_BPCR)**

This register defines the accumulated number of Horizontal Synchronization and back porch and the accumulated number of Vertical

pixels minus 1 ( HSYNC Width + HBP- 1)

Synchronization and back porch lines minus 1 (

**VSYNC Height + VBP - 1) . Refer to**

Figure 82 and Section 16.4: LTDC programmable parameters configuration.

for an example of

Address offset: 0x0C

Reset value: 0x0000 0000

31 30 29 28 27 26 25 24

23 22 21 20 19 18 17 16

| Reserved | AHBP[11:0] |
|---|---|
| rw |  |


15 14 13 12 11 10 9 8

7 6 5 4 3 2 1 0

| Reserved | AVBP[10:0] |
|---|---|
| rw |  |




514

--- Page 104 ---

**LCD-TFT controller (LTDC) RM0090**

Bits 31:28 Reserved, must be kept at reset value

Bits 27:16 AHBP[11:0] : Accumulated Horizontal back porch (in units of pixel clock period)

These bits define the Accumulated Horizontal back porch width which includes the Horizontal Synchronization and Horizontal back porch pixels minus 1.

The Horizontal back porch is the period between Horizontal Synchronization going inactive and the start of the active display part of the next scan line.

Bits 15:11 Reserved, must be kept at reset value

Bits 10:0 AVBP[10:0]: Accumulated Vertical back porch (in units of horizontal scan line)

These bits define the accumulated Vertical back porch width which includes the Vertical Synchronization and Vertical back porch lines minus 1.

The Vertical back porch is the number of horizontal scan lines at a start of frame to the start of the first active scan line of the next frame.

> [RM0090 ch.16 §16.7] LTDC registers**

### 16.7.3 LTDC Active Width Configuration Register (LTDC_AWCR)**

**16.7.3 LTDC Active Width Configuration Register (LTDC_AWCR)**

This register defines the accumulated number of Horizontal Synchronization, back porch and Active pixels minus 1 ( HSYNC width + HBP + Active Width - 1) and the accumulated number of Vertical Synchronization, back porch lines and Active lines minus 1 ( VSYNC Height+ BVBP + Active Height - 1) . Refer to Figure 82 and Section 16.4: LTDC programmable parameters for an example of configuration.

Address offset: 0x10

Reset value: 0x0000 0000

31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16

| Reserved | AAW[11:0] |
|---|---|
| rw |  |


15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0

| Reserved | AAH[10:0] |
|---|---|
| rw |  |


Bits 31:28 Reserved, must be kept at reset value

Bits 27:16 AAW[11:0] : Accumulated Active Width (in units of pixel clock period)

These bits define the Accumulated Active Width which includes the Horizontal Synchronization, Horizontal back porch and Active pixels minus 1.

The Active Width is the number of pixels in active display area of the panel scan line. The maximum Active Width supported is 0x400.

Bits 15:11 Reserved, must be kept at reset value

Bits 10:0 AAH[10:0] : Accumulated Active Height (in units of horizontal scan line)

These bits define the Accumulated Height which includes the Vertical Synchronization, Vertical back porch and the Active Height lines minus 1. The Active Height is the number of active lines in the panel. The maximum Active Height supported is 0x300.

494/1749 RM0090 Rev 18

--- Page 105 ---

**RM0090**

**LCD-TFT controller (LTDC)**

> [RM0090 ch.16 §16.7] LTDC registers**

### 16.7.4 LTDC Total Width Configuration Register (LTDC_TWCR)**

**16.7.4 LTDC Total Width Configuration Register (LTDC_TWCR)**

This register defines the accumulated number of Horizontal Synchronization, back porch, HSYNC Width + HBP + Active Width + HFP - 1) and the accumulated number of Vertical Synchronization, back porch lines, Active and Front VSYNC Height+ BVBP + Active Height + VFP - 1) . Refer to Figure 82 and

Active and front porch pixels minus 1 (

lines minus 1 (

Section 16.4: LTDC programmable parameters

for an example of configuration.

Address offset: 0x14

Reset value: 0x0000 0000

31 30 29 28 27 26 25 24

23 22 21 20 19 18 17 16

|  Не прилагайте двигателя, не п отверстии и пов | слишком больших усилий при надавливании фитинга на вал овредите поверхность трения, удалите заусенцы на монтажном |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|
| ерхн | ости, | устано | вите в | тулку | вала | на вал | и заф | икс | ируйт | е ее осев | ой |


16 14 13 12 11 10 9 8

7 6 5 4 3 2 1 0

|  Измерьте напряжени |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|
| е пос | тоянно | го ток | а, под | ключ | енное к | торм | озу, | и сра | вните его | с |  |


Bits 31:28 Reserved, must be kept at reset value

Bits 27:16 TOTALW[11:0]:

Total Width (in units of pixel clock period)

These bits defines the accumulated Total Width which includes the Horizontal Synchronization, Horizontal back porch, Active Width and Horizontal front porch pixels

minus 1.

Bits 15:11 Reserved, must be kept at reset value

Bits 10:0 TOTALH[10:0] : Total Height

(in units of horizontal scan line)

These bits defines the accumulated Height which includes the Vertical Synchronization, Vertical back porch, the Active Height and Vertical front porch Height lines minus 1.


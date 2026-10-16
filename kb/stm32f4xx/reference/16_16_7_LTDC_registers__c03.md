> [RM0090 ch.16 §16.7] LTDC registers**

### 16.7.6 LTDC Shadow Reload Configuration Register (LTDC_SRCR)**

**16.7.6 LTDC Shadow Reload Configuration Register (LTDC_SRCR)**

This register allows to reload either immediately or during the vertical blanking period, the shadow registers values to the active registers. The shadow registers are all Layer1 and

Layer2 registers except the LTDC_L1CLUTWR and the LTDC_L2CLUTWR. Address offset: 0x24

Reset value: 0x0000 0000

31 30 29 28 27 26 25 24

23 22 21 20 19 18 17 16

Reserved

15 14 13 12 11 10 9 8

7 6 5 4 3 2 1 0

| Reserved | VBR | IMR |
|---|---|---|
| rw |  |  |


Bits 31:2 Reserved, must be kept at reset value

Bit 1 VBR : Vertical Blanking Reload

This bit is set by software and cleared only by hardware after reload. (it cannot be cleared

through register write once it is set)

0: No effect

1: The shadow registers are reloaded during the vertical blanking period (at the

beginning of the first line after the Active Display Area)

Bit 0 IMR : Immediate Reload

This bit is set by software and cleared only by hardware after reload.

0: No effect

1: The shadow registers are reloaded immediately

Note:

The shadow registers read back the active values. Until the reload has been done, the 'old'

value will be read.

**16.7.7**

**LTDC Background Color Configuration Register (LTDC_BCCR)**

This register defines the background color (RGB888).

Address offset: 0x2C

Reset value: 0x0000 0000

31 30 29 28 27 26 25 24

23 22 21 20 19 18 17 16

| Reserved | BCRED[7:0] |
|---|---|
| rw |  |


15 14 13 12 11 10 9 8

7 6 5 4 3 2 1 0

| BCGREEN[7:0] | BCBLUE[7:0] |
|---|---|
| rw |  |




514

--- Page 108 ---

**LCD-TFT controller (LTDC)**

**RM0090**

Bits 31:24 Reserved, must be kept at reset value Bits 23:16 BCRED[7:0] : Background Color Red value

These bits configure the background red value Bits 15:8 BCGREEN[7:0]

: Background Color Green value

These bits configure the background green value Bits 7:0 BCBLUE[7:0] : Background Color Blue value

These bits configure the background blue value

> [RM0090 ch.16 §16.7] LTDC registers**

### 16.7.8 LTDC Interrupt Enable Register (LTDC_IER)**

**16.7.8 LTDC Interrupt Enable Register (LTDC_IER)**

This register determines which status flags generate an interrupt request by setting the

corresponding bit to 1.

Address offset: 0x34

Reset value: 0x0000 0000

31 30 29 28 27 26 25 24

23 22 21 20 19 18 17 16

Reserved

15 14 13 12 11 10 9 8

7 6 5 4 3 2 1 0

| Reserved | RRIE | TERRIE | FUIE | LIE |
|---|---|---|---|---|
| rw |  |  |  |  |


Bits 31:4 Reserved, must be kept at reset value Bit 3 RRIE : Register Reload interrupt enable

This bit is set and cleared by software 0: Register Reload interrupt disable 1: Register Reload interrupt enable Bit 2 TERRIE : Transfer Error Interrupt Enable

This bit is set and cleared by software 0: Transfer Error interrupt disable 1: Transfer Error interrupt enable Bit 1 FUIE : FIFO Underrun Interrupt Enable

This bit is set and cleared by software 0: FIFO Underrun interrupt disable 1: FIFO Underrun Interrupt enable Bit 0 LIE : Line Interrupt Enable

This bit is set and cleared by software 0: Line interrupt disable

1: Line Interrupt enable

498/1749

RM0090 Rev 18

--- Page 109 ---

**RM0090**

**LCD-TFT controller (LTDC)**

> [RM0090 ch.16 §16.7] LTDC registers**

### 16.7.9 LTDC Interrupt Status Register (LTDC_ISR)**

**16.7.9 LTDC Interrupt Status Register (LTDC_ISR)**

This register returns the interrupt status flag Address offset: 0x38

Reset value: 0x0000 0000

31 30 29 28 27 26 25 24

23 22 21 20 19 18 17 16

Reserved

15 14 13 12 11 10 9 8

7 6 5 4 3 2 1 0

| – Открутите три крепежных винта(1). – Немного поверните гаечным ключом три полых винта(7) по часово |  |  |  |  |
|---|---|---|---|---|
| й стр | елке. |  |  |  |


Bits 31:24 Reserved, must be kept at reset value

Bit 3 RRIF : Register Reload Interrupt Flag

0: No Register Reload interrupt generated

1: Register Reload interrupt generated when a vertical blanking reload occurs (and the

first line after the active area is reached) Bit 2 TERRIF : Transfer Error interrupt flag

0: No Transfer Error interrupt generated

1: Transfer Error interrupt generated when a Bus error occurs

Bit 1 FUIF : FIFO Underrun Interrupt flag

0: NO FIFO Underrun interrupt generated.

1: A FIFO underrun interrupt is generated, if one of the layer FIFOs is empty and pixel

data is read from the FIFO Bit 0 LIF : Line Interrupt flag

0: No Line interrupt generated

1: A Line interrupt is generated, when a programmed line is reached

> [RM0090 ch.16 §16.7] LTDC registers**

### 16.7.10 LTDC Interrupt Clear Register (LTDC_ICR)**

**16.7.10 LTDC Interrupt Clear Register (LTDC_ICR)**

Address offset: 0x3C

Reset value: 0x0000 0000

31 30 29 28 27 26 25 24

23 22 21 20 19 18 17 16

Reserved

15 14 13 12 11 10 9 8

7 6 5 4 3 2 1 0

| он не сможет достичь максимального крутящего момента. – Температура окружающей среды общего пользования составл |  |  |  |  |
|---|---|---|---|---|
| яет -1 | 0 ℃ ~ + | 40 ℃ | . |  |




514

--- Page 110 ---

**LCD-TFT controller (LTDC)**

**RM0090**

Bits 31:24 Reserved, must be kept at reset value

Bit 3 CRRIF : Clears Register Reload Interrupt Flag

0: No effect

1: Clears the RRIF flag in the LTDC_ISR register Bit 2 CTERRIF : Clears the Transfer Error Interrupt Flag

0: No effect

1: Clears the TERRIF flag in the LTDC_ISR register. Bit 1 CFUIF : Clears the FIFO Underrun Interrupt flag

0: No effect

1: Clears the FUDERRIF flag in the LTDC_ISR register. Bit 0 CLIF : Clears the Line Interrupt Flag

0: No effect

1: Clears the LIF flag in the LTDC_ISR register.

**16.7.11**

**LTDC Line Interrupt Position Configuration Register (LTDC_LIPCR)**

This register defines the position of the line interrupt. The line value to be programmed

depends on the timings parameters. Refer to

Figure 82 .

Address offset: 0x40

Reset value: 0x0000 0000

31 30 29 28 27 26 25 24

23 22 21 20 19 18 17 16

Reserved

15 14 13 12 11 10 9 8

7 6 5 4 3 2 1 0

| Reserved | LIPOS[10:0] |
|---|---|
| rw |  |


Bits 31:11 Reserved, must be kept at reset value

Bits 10:0 LIPOS[10:0] : Line Interrupt Position

These bits configure the line interrupt position

> [RM0090 ch.16 §16.7] LTDC registers**

### 16.7.12 LTDC Current Position Status Register (LTDC_CPSR)**

**16.7.12 LTDC Current Position Status Register (LTDC_CPSR)**

Address offset: 0x44

Reset value: 0x0000 0000

31 30 29 28 27 26 25 24

23 22 21 20 19 18 17 16

| CXPOS[15:0] |
|---|
| r |


15 14 13 12 11 10 9 8

7 6 5 4 3 2 1 0

| CYPOS{15;0] |
|---|
| r |


500/1749

RM0090 Rev 18

--- Page 111 ---

**RM0090**

**LCD-TFT controller (LTDC)**

Bits 31:16: CXPOS[15:0] : Current X Position

These bits return the current X position

Bits 15:0 CYPOS[15:0] : Current Y Position

These bits return the current Y position

> [RM0090 ch.16 §16.7] LTDC registers**

### 16.7.13 LTDC Current Display Status Register (LTDC_CDSR)**

**16.7.13 LTDC Current Display Status Register (LTDC_CDSR)**

This register returns the status of the current display phase which is controlled by the

HSYNC, VSYNC, and Horizontal/Vertical DE signals.

Example: if the current display phase is the vertical synchronization, the VSYNCS bit is set (active high). If the current display phase is the horizontal synchronization, the HSYNCS bit

is active high.

Address offset: 0x48

Reset value: 0x0000 000F

31 30 29 28 27 26 25 24

23 22 21 20 19 18 17 16

Reserved

15 14 13 12 11 10 9 8

7 6 5 4 3 2 1 0

| Reserved | HSYNC S | VSYNC S | HDES | VDES |
|---|---|---|---|---|
| r |  |  |  |  |


Bits 31:24 Reserved, must be kept at reset value

Bit 3 HSYNCS : Horizontal Synchronization display Status

0: Active low

1: Active high

Bit 2 VSYNCS : Vertical Synchronization display Status

0: Active low

1: Active high

Bit 1 HDES : Horizontal Data Enable display Status

0: Active low

1: Active high

Bit 0 VDES : Vertical Data Enable display Status

0: Active low

1: Active high

Note: The returned status does not depend on the configured polarity in the instead it returns the current active display phase.

**LTDC_GCR register,**

RM0090 Rev 18 501/1749

514

--- Page 112 ---

**LCD-TFT controller (LTDC)**

**RM0090**


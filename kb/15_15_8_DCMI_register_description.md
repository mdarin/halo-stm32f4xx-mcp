**15.8 DCMI register description**

All DCMI registers have to be accessed as 32-bit words, otherwise a bus error occurs.

**15.8.1 DCMI control register 1 (DCMI_CR)**

Address offset: 0x00

Reset value: 0x0000 0x0000

31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16

15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0

| Reserved | ENABLE | Reserved | EDM | FCRC | VSPOL | HSPOL | PCKPOL | ESS | JPEG | CROP | CM | CAPTURE |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| rw |  |  |  |  |  |  |  |  |  |  |  |  |


Bit 31:15 Reserved, must be kept at reset value. Bit 14 ENABLE: DCMI enable

0: DCMI disabled 1: DCMI enabled Note:

The DCMI configuration registers should be programmed correctly before

enabling this Bit Bit 13: 12 Reserved, must be kept at reset value.



479

--- Page 78 ---

**Digital camera interface (DCMI) RM0090**

Bits 11:10 EDM[1:0]: Extended data mode

00: Interface captures 8-bit data on every pixel clock

01: Interface captures 10-bit data on every pixel clock

10: Interface captures 12-bit data on every pixel clock

11: Interface captures 14-bit data on every pixel clock

Bits 9:8 FCRC[1:0]: Frame capture rate control

These bits define the frequency of frame capture. They are meaningful only in Continuous grab mode. They are ignored in snapshot mode.

00: All frames are captured

01: Every alternate frame captured (50% bandwidth reduction)

10: One frame in 4 frames captured (75% bandwidth reduction)

11: reserved

Bit 7 VSPOL: Vertical synchronization polarity

This bit indicates the level on the VSYNC pin when the data are not valid on the parallel interface.

0: VSYNC active low

1: VSYNC active high

Bit 6 HSPOL: Horizontal synchronization polarity

This bit indicates the level on the HSYNC pin when the data are not valid on the parallel interface.

0: HSYNC active low

1: HSYNC active high

Bit 5 PCKPOL: Pixel clock polarity

This bit configures the capture edge of the pixel clock

0: Falling edge active.

1: Rising edge active.

Bit 4 ESS: Embedded synchronization select

0: Hardware synchronization data capture (frame/line start/stop) is synchronized with the HSYNC/VSYNC signals.

1: Embedded synchronization data capture is synchronized with synchronization codes embedded in the data flow.

Note: Valid only for 8-bit parallel data. HSPOL/VSPOL are ignored when the ESS

bit is set.

This bit is disabled in JPEG mode.

Bit 3 JPEG: JPEG format

0: Uncompressed video format

1: This bit is used for JPEG data transfers. The HSYNC signal is used as data enable. The crop and embedded synchronization features (ESS bit) cannot be used in this mode.

Bits 2 CROP: Crop feature

0: The full image is captured. In this case the total number of bytes in an image frame should be a multiple of 4

1: Only the data inside the window specified by the crop register will be captured. If the size of the crop window exceeds the picture size, then only the picture size is captured.

468/1749 RM0090 Rev 18

--- Page 79 ---

**RM0090 Digital camera interface (DCMI)**

Bit 1 CM: Capture mode

0: Continuous grab mode - The received data are transferred into the destination memory through the DMA. The buffer location and mode (linear or circular buffer) is controlled through the system DMA.

1: Snapshot mode (single frame) - Once activated, the interface waits for the start of frame and then transfers a single frame through the DMA. At the end of the frame, the CAPTURE bit is automatically reset.

Bit 0 CAPTURE: Capture enable

0: Capture disabled.

1: Capture enabled.

The camera interface waits for the first start of frame, then a DMA request is generated to transfer the received data into the destination memory.

In snapshot mode, the CAPTURE bit is automatically cleared at the end of the 1st frame received.

In continuous grab mode, if the software clears this bit while a capture is ongoing, the bit will be effectively cleared after the frame end.

Note: The DMA controller and all DCMI configuration registers should be

programmed correctly before enabling this bit.

RM0090 Rev 18 469/1749

479

--- Page 80 ---

**Digital camera interface (DCMI)**

**RM0090**

**15.8.2 DCMI status register (DCMI_SR)**

Address offset: 0x04

Reset value: 0x0000 0x0000

31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16

15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0

| Reserved | FNE | VSYNC | HSYNC |
|---|---|---|---|
| r |  |  |  |


Bits 31:3 Reserved, must be kept at reset value. Bit 2 FNE: FIFO not empty This bit gives the status of the FIFO 1: FIFO contains valid data 0: FIFO empty Bit 1 VSYNC

This bit gives the state of the VSYNC pin with the correct programmed polarity. When embedded synchronization codes are used, the meaning of this bit is the

following:

0: active frame 1: synchronization between frames

In case of embedded synchronization, this bit is meaningful only if the

CAPTURE bit in DCMI_CR is set. Bit 0 HSYNC

This bit gives the state of the HSYNC pin with the correct programmed polarity. When embedded synchronization codes are used, the meaning of this bit is the

following:

0: active line

1: synchronization between lines

In case of embedded synchronization, this bit is meaningful only if the

CAPTURE bit in DCMI_CR is set.



--- Page 81 ---

**RM0090 Digital camera interface (DCMI)**

**15.8.3 DCMI raw interrupt status register (DCMI_RIS)**

Address offset: 0x08

Reset value: 0x0000 0x0000

31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0

| Reserved | LINE_RIS | VSYNC_RIS | ERR_RIS | OVR_RIS | FRAME_RIS |
|---|---|---|---|---|---|
| r |  |  |  |  |  |


DCMI_RIS gives the raw interrupt status and is accessible in read only. When read, this register returns the status of the corresponding interrupt before masking with the DCMI_IER register value.

Bits 31:5 Reserved, must be kept at reset value.

Bit 4 LINE_RIS: Line raw interrupt status

This bit gets set when the HSYNC signal changes from the inactive state to the active state. It goes high even if the line is not valid.

In the case of embedded synchronization, this bit is set only if the CAPTURE bit in DCMI_CR is set.

It is cleared by writing a ‘1’ to the LINE_ISC bit in DCMI_ICR.

Bit 3 VSYNC_RIS: VSYNC raw interrupt status

This bit is set when the VSYNC signal changes from the inactive state to the active state.

In the case of embedded synchronization, this bit is set only if the CAPTURE bit is set in DCMI_CR.

It is cleared by writing a ‘1’ to the VSYNC_ISC bit in DCMI_ICR.

Bit 2 ERR_RIS: Synchronization error raw interrupt status

0: No synchronization error detected

1: Embedded synchronization characters are not received in the correct order. This bit is valid only in the embedded synchronization mode. It is cleared by writing a ‘1’ to the ERR_ISC bit in DCMI_ICR.

Note: This bit is available only in embedded synchronization mode.

Bit 1 OVR_RIS: Overrun raw interrupt status

0: No data buffer overrun occurred

1: A data buffer overrun occurred and the data FIFO is corrupted.

This bit is cleared by writing a ‘1’ to the OVR_ISC bit in DCMI_ICR.

Bit 0 FRAME_RIS: Capture complete raw interrupt status

0: No new capture

1: A frame has been captured.

This bit is set when a frame or window has been captured.

In case of a cropped window, this bit is set at the end of line of the last line in the crop. It is set even if the captured frame is empty (e.g. window cropped outside the frame).

This bit is cleared by writing a ‘1’ to the FRAME_ISC bit in DCMI_ICR.



479

--- Page 82 ---

**Digital camera interface (DCMI) RM0090**

**15.8.4 DCMI interrupt enable register (DCMI_IER)**

Address offset: 0x0C

Reset value: 0x0000 0x0000

31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0

| Reserved | LINE_IE | VSYNC_IE | ERR_IE | OVR_IE | FRAME_IE |
|---|---|---|---|---|---|
| rw |  |  |  |  |  |


The DCMI_IER register is used to enable interrupts. When one of the DCMI_IER bits is set, the corresponding interrupt is enabled. This register is accessible in both read and write.

Bits 31:5 Reserved, must be kept at reset value.

Bit 4 LINE_IE: Line interrupt enable

0: No interrupt generation when the line is received

1: An Interrupt is generated when a line has been completely received Bit 3 VSYNC_IE: VSYNC interrupt enable

0: No interrupt generation

1: An interrupt is generated on each VSYNC transition from the inactive to the active state

The active state of the VSYNC signal is defined by the VSPOL bit.

Bit 2 ERR_IE: Synchronization error interrupt enable

0: No interrupt generation

1: An interrupt is generated if the embedded synchronization codes are not received in the correct order.

Note: This bit is available only in embedded synchronization mode.

Bit 1 OVR_IE: Overrun interrupt enable

0: No interrupt generation

1: An interrupt is generated if the DMA was not able to transfer the last data before new data (32-bit) are received.

Bit 0 FRAME_IE: Capture complete interrupt enable

0: No interrupt generation

1: An interrupt is generated at the end of each received frame/crop window (in crop mode).



--- Page 83 ---

**RM0090 Digital camera interface (DCMI)**

**15.8.5 DCMI masked interrupt status register (DCMI_MIS)**

This DCMI_MIS register is a read-only register. When read, it returns the current masked status value (depending on the value in DCMI_IER) of the corresponding interrupt. A bit in this register is set if the corresponding enable bit in DCMI_IER is set and the corresponding bit in DCMI_RIS is set.

Address offset: 0x10

Reset value: 0x0000

31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0

| Reserved | LINE_MIS | VSYNC_MIS | ERR_MIS | OVR_MIS | FRAME_MIS |
|---|---|---|---|---|---|
| r |  |  |  |  |  |


Bits 31:5 Reserved, must be kept at reset value.

Bit 4 LINE_MIS: Line masked interrupt status

This bit gives the status of the masked line interrupt

0: No interrupt generation when the line is received

1: An Interrupt is generated when a line has been completely received and the LINE_IE bit is set in DCMI_IER.

Bit 3 VSYNC_MIS: VSYNC masked interrupt status

This bit gives the status of the masked VSYNC interrupt

0: No interrupt is generated on VSYNC transitions

1: An interrupt is generated on each VSYNC transition from the inactive to the active state and the VSYNC_IE bit is set in DCMI_IER.

The active state of the VSYNC signal is defined by the VSPOL bit.

Bit 2 ERR_MIS: Synchronization error masked interrupt status

This bit gives the status of the masked synchronization error interrupt 0: No interrupt is generated on a synchronization error

1: An interrupt is generated if the embedded synchronization codes are not received in the correct order and the ERR_IE bit in DCMI_IER is set.

Note: This bit is available only in embedded synchronization mode.

Bit 1 OVR_MIS: Overrun masked interrupt status

This bit gives the status of the masked overflow interrupt

0: No interrupt is generated on overrun

1: An interrupt is generated if the DMA was not able to transfer the last data before new data (32-bit) are received and the OVR_IE bit is set in DCMI_IER. Bit 0 FRAME_MIS: Capture complete masked interrupt status

This bit gives the status of the masked capture complete interrupt 0: No interrupt is generated after a complete capture

1: An interrupt is generated at the end of each received frame/crop window (in crop mode) and the FRAME_IE bit is set in DCMI_IER.



479

--- Page 84 ---

**Digital camera interface (DCMI)**

**RM0090**

**15.8.6 DCMI interrupt clear register (DCMI_ICR)**

Address offset: 0x14

Reset value: 0x0000 0x0000

31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16

15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0

| Reserved | LINE_ISC | VSYNC_ISC | ERR_ISC | OVR_ISC | FRAME_ISC |
|---|---|---|---|---|---|
| w |  |  |  |  |  |


The DCMI_ICR register is write-only. Writing a ‘1’ into a bit of this register clears the corresponding bit in the DCMI_RIS and DCMI_MIS registers. Writing a ‘0’ has no effect.

Bits 15:5 Reserved, must be kept at reset value. Bit 4 LINE_ISC: line interrupt status clear

Writing a ‘1’ into this bit clears LINE_RIS in the DCMI_RIS register

Bit 3 VSYNC_ISC:

Vertical synch interrupt status clear

Writing a ‘1’ into this bit clears the VSYNC_RIS bit in DCMI_RIS Synchronization error interrupt status clear

Bit 2 ERR_ISC:

Writing a ‘1’ into this bit clears the ERR_RIS bit in DCMI_RIS This bit is available only in embedded synchronization mode.

Note:

Bit 1 OVR_ISC:

Overrun interrupt status clear

Writing a ‘1’ into this bit clears the OVR_RIS bit in DCMI_RIS

Bits 0 FRAME_ISC:

Capture complete interrupt status clear

Writing a ‘1’ into this bit clears the FRAME_RIS bit in DCMI_RIS



--- Page 85 ---

**RM0090 Digital camera interface (DCMI)**

**15.8.7 DCMI embedded synchronization code register (DCMI_ESCR)**

Address offset: 0x18

Reset value: 0x0000 0x0000

31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0

| 1 | 5. Фрикцио | нный диск |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
|  |  |  | 7 |  |  |  |  |  |  |  |  | 6. | Пы | ле | за | щит | ны | й ч | ех | ол |  |  |  |


Bits 31:24 FEC: Frame end delimiter code

This byte specifies the code of the frame end delimiter. The code consists of 4 bytes in the form of 0xFF, 0x00, 0x00, FEC.

If FEC is programmed to 0xFF, all the unused codes (0xFF0000XY) are interpreted as frame end delimiters.

Bits 23:16 LEC: Line end delimiter code

This byte specifies the code of the line end delimiter. The code consists of 4 bytes in the form of 0xFF, 0x00, 0x00, LEC.

Bits 15:8 LSC: Line start delimiter code

This byte specifies the code of the line start delimiter. The code consists of 4 bytes in the form of 0xFF, 0x00, 0x00, LSC.

Bits 7:0 FSC: Frame start delimiter code

This byte specifies the code of the frame start delimiter. The code consists of 4 bytes in the form of 0xFF, 0x00, 0x00, FSC.

If FSC is programmed to 0xFF, no frame start delimiter is detected. But, the 1 st occurrence of LSC after an FEC code will be interpreted as a start of frame delimiter.



479

--- Page 86 ---

**Digital camera interface (DCMI) RM0090**

**15.8.8 DCMI embedded synchronization unmask register (DCMI_ESUR)**

Address offset: 0x1C

Reset value: 0x0000 0x0000

31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0



Bits 31:24 FEU: Frame end delimiter unmask

This byte specifies the mask to be applied to the code of the frame end delimiter. 0: The corresponding bit in the FEC byte in DCMI_ESCR is masked while comparing the frame end delimiter with the received data.

1: The corresponding bit in the FEC byte in DCMI_ESCR is compared while comparing the frame end delimiter with the received data

Bits 23:16 LEU: Line end delimiter unmask

This byte specifies the mask to be applied to the code of the line end delimiter. 0: The corresponding bit in the LEC byte in DCMI_ESCR is masked while comparing the line end delimiter with the received data

1: The corresponding bit in the LEC byte in DCMI_ESCR is compared while comparing the line end delimiter with the received data

Bits 15:8 LSU: Line start delimiter unmask

This byte specifies the mask to be applied to the code of the line start delimiter. 0: The corresponding bit in the LSC byte in DCMI_ESCR is masked while comparing the line start delimiter with the received data

1: The corresponding bit in the LSC byte in DCMI_ESCR is compared while comparing the line start delimiter with the received data

Bits 7:0 FSU: Frame start delimiter unmask

This byte specifies the mask to be applied to the code of the frame start delimiter.

0: The corresponding bit in the FSC byte in DCMI_ESCR is masked while comparing the frame start delimiter with the received data 1: The corresponding bit in the FSC byte in DCMI_ESCR is compared while comparing the frame start delimiter with the received data



--- Page 87 ---

**RM0090**

**Digital camera interface (DCMI)**

**15.8.9 DCMI crop window start (DCMI_CWSTRT)**

Address offset: 0x20

Reset value: 0x0000 0x0000

31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16

15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0

| Reserved | VST[12:0 | Reserv ed | HOFFCNT[13:0] |
|---|---|---|---|
| rw |  |  |  |


Bits 31:29 Reserved, must be kept at reset value. Bits 28:16 VST[12:0]: Vertical start line count

The image capture starts with this line number. Previous line data are ignored.

0x0000 => line 1 0x0001 => line 2 0x0002 => line 3 ....

Bits 15:14 Reserved, must be kept at reset value. Bit 13:0 HOFFCNT[13:0]:

Horizontal offset count

This value gives the number of pixel clocks to count before starting a capture.

**15.8.10 DCMI crop window size (DCMI_CWSIZE)**

Address offset: 0x24

Reset value: 0x0000 0x0000

31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16

15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0

| Reserved | VLINE13:0] | Reserved | CAPCNT[13:0] |
|---|---|---|---|
| rw |  |  |  |


Bits 31:30 Reserved, must be kept at reset value. Bitst 29:16 VLINE[13:0]: Vertical line count

This value gives the number of lines to be captured from the starting point.

0x0000 => 1 line 0x0001 => 2 lines 0x0002 => 3 lines ....

Bits 15:14 Reserved, must be kept at reset value. Bits 13:0 CAPCNT[13:0]: Capture count

This value gives the number of pixel clocks to be captured from the starting point on the same line. It value should corresponds to word-aligned data for

different widths of parallel interfaces. 0x0000 => 1 pixel 0x0001 => 2 pixels 0x0002 => 3 pixels ....



479

--- Page 88 ---

**Digital camera interface (DCMI)**

**RM0090**

**15.8.11 DCMI data register (DCMI_DR)**

Address offset: 0x28

Reset value: 0x0000 0x0000

31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16

15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0

| Byte3 | Byte2 | Byte1 | Byte0 |
|---|---|---|---|
| r |  |  |  |


Bits 31:24 Data byte 3

Bits 23:16 Data byte 2

Bits 15:8 Data byte 1

Bits 7:0 Data byte 0

The digital camera Interface packages all the received data in 32-bit format before requesting a DMA transfer. A 4-word deep FIFO is available to leave enough time for DMA

transfers and avoid DMA overrun conditions.

**15.8.12 DCMI register map**

Table 87 summarizes the DCMI registers.

**Table 87. DCMI register map and reset values**

| Offset | Register | 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 0x00 | DCMI CR _ | Reserved | ENABLE | Reserved | EDM | FCR C | VSPOL | HSPOL | PCKPOL | ESS | JPEG | CROP | CM | CAPTURE |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x04 | DCMI SR _ | Reserved | FNE | VSYNC | HSYNC |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x08 | DCMI RIS _ | Reserved | LINE_RIS | VSYNC_RIS | ERR_RIS | OVR_RIS | FRAME_RIS |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x0C | DCMI IER _ | Reserved | LINE_IE | VSYNC_IE | ERR_IE | OVR_IE | FRAME_IE |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x10 | DCMI MIS _ | Reserved | LINE_MIS | VSYNC_MIS | ERR_MIS | OVR_MIS | FRAME_MIS |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |




--- Page 89 ---

**RM0090**

**Digital camera interface (DCMI)**

**Table 87. DCMI register map and reset values (continued)**

| Offset | Register | 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 0x14 | DCMI ICR _ | Reserved | LINE_ISC | VSYNC_ISC | ERR_ISC | OVR_ISC | FRAME_ISC |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x18 | DCMI ESCR _ | FEC | LEC | LSC | FSC |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x1C | DCMI ESUR _ | FEU | LEU | LSU | FSU |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x20 | DCMI CWSTR _ T | Reserve d | VST[12:0 | Reserved | HOFFCNT[13:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x24 | DCMI CWSIZ _ E | Reserved | VLINE13:0] | Reserved | CAPCNT[13:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x28 | DCMI DR _ | Byte3 | Byte2 | Byte1 | Byte0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Refer to Section 2.3: Memory map

for the register boundary addresses.



479

--- Page 90 ---

**LCD-TFT controller (LTDC) RM0090**


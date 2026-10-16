**15.5 DCMI functional overview**

The digital camera interface is a synchronous parallel interface that can receive high-speed (up to 54 Mbytes/s) data flows. It consists of up to 14 data lines (D13-D0) and a pixel clock line (PIXCLK). The pixel clock has a programmable polarity, so that data can be captured on either the rising or the falling edge of the pixel clock.

The data are packed into a 32-bit data register (DCMI_DR) and then transferred through a general-purpose DMA channel. The image buffer is managed by the DMA, not by the camera interface.

The data received from the camera can be organized in lines/frames (raw YUB/RGB/Bayer modes) or can be a sequence of JPEG images. To enable JPEG image reception, the JPEG bit (bit 3 of DCMI_CR register) must be set.

The data flow is synchronized either by hardware using the optional HSYNC (horizontal synchronization) and VSYNC (vertical synchronization) signals or by synchronization codes embedded in the data flow.

Figure 72 shows the DCMI block diagram.

**Figure 72. DCMI block diagram**

DMA Control/Status

interface register

AHB

interface

FIFO/ Data Data extraction Synchronizer DCMI_PIXCLK formatter

DCMI_D[0:13], DCMI_HSYNC, DCMI_VSYNC

ai15604b



--- Page 67 ---

**RM0090**

**Digital camera interface (DCMI)**

| Figure 73. Top-level block HCLK DCMI Interrupt DCMI IT _ controller DMA REQ _ 15.5.1 DMA interface The DMA interface is active when the CAPTURE bit in t request is generated each time the camera interface re in its register. 15.5.2 DCMI physical interface The interface is composed of 11/13/15/17 inputs. Only t The camera interface can capture 8-bit, 10-bit, 12-bit or EDM[1:0] bits in the DCMI CR register. If less than 14 b _ must not be assigned to DCMI interface through GPIO | diagram |  |  |
|---|---|---|---|
| HCLK DCMI Interrupt DCMI IT _ controller DMA REQ _ | DCMI D[0:1 _ | 3] |  |
| DCMI PIXC _ | LK | External interface |  |
| DCMI HSY _ | NC NC |  |  |
| DCMI VSY |  |  |  |
| _ |  |  |  |
|  | ai15603b |  |  |
| he DCMI | CR register is set. A DMA |  |  |
| _ ceives a c | omplete 32-bit data block |  |  |
| he Slave | mode is supported. |  |  |
| 14-bit da its are us alternate f | ta depending on the ed, the unused data pins unctions. |  |  |


**Table 78. DCMI signals**

| Signal description |  |  |
|---|---|---|
| D[0..7] D[0..9] D[0..11] D[0..13] | Data |  |
|  | Pixel clock |  |
| C |  | Horizontal synchronization / Data valid |
|  | Vertical synchronization |  |


8 bits

10 bits

12 bits

14 bits

PIXCLK

HSYNC

VSYNC

The data are synchronous with PIXCLK and change on the rising/falling edge of the pixel

clock depending on the polarity.

The HSYNC signal indicates the start/end of a line. The VSYNC signal indicates the start/end of a frame



479

--- Page 68 ---

**Digital camera interface (DCMI) RM0090**

**Figure 74. DCMI signal waveforms**

DCMI_PIXCLK

DCMI_DR[0:13]

DCMI_HSYNC

DCMI_VSYNC

ai15606b

1. The capture edge of DCMI_PIXCLK is the falling edge, the active state of DCMI_HSYNC and DCMI_VSYNC is 1.

1. DCMI_HSYNC and DCMI_VSYNC can change states at the same time.

**8-bit data**

When EDM[1:0] in DCMI_CR are programmed to “00” the interface captures 8 LSB’s at its input (D[0:7]) and stores them as 8-bit data. The D[13:8] inputs are ignored. In this case, to capture a 32-bit word, the camera interface takes four pixel clock cycles.

The first captured data byte is placed in the LSB position in the 32-bit word and the 4 th captured data byte is placed in the MSB position in the 32-bit word. Table 79 gives an example of the positioning of captured data bytes in two 32-bit words.

**Table 79. Positioning of captured data bytes in 32-bit words (8-bit width)**

| Byte address | 31:24 | 23:16 | 15:8 | 7:0 |
|---|---|---|---|---|
| 0 | D [7:0] n+3 | D [7:0] n+2 | D [7:0] n+1 | D [7:0] n |
| 4 | D [7:0] n+7 | D [7:0] n+6 | D [7:0] n+5 | D [7:0] n+4 |


**10-bit data**

When EDM[1:0] in DCMI_CR are programmed to “01”, the camera interface captures 10-bit data at its input D[0..9] and stores them as the 10 least significant bits of a 16-bit word. The remaining most significant bits in the DCMI_DR register (bits 11 to 15) are cleared to zero. So, in this case, a 32-bit data word is made up every two pixel clock cycles.

The first captured data are placed in the LSB position in the 32-bit word and the 2 nd captured data are placed in the MSB position in the 32-bit word as shown in Table 80 .

**Table 80. Positioning of captured data bytes in 32-bit words (10-bit width)**

| Byte address | 31:26 | 25:16 | 15:10 | 9:0 |
|---|---|---|---|---|
| 0 | D [9:0] n+1 | 0 | D [9:0] n |  |
| 4 | 0 | D [9:0] n+3 | 0 | D [9:0] n+2 |




--- Page 69 ---

**RM0090 Digital camera interface (DCMI)**

**12-bit data**

When EDM[1:0] in DCMI_CR are programmed to “10”, the camera interface captures the 12-bit data at its input D[0..11] and stores them as the 12 least significant bits of a 16-bit word. The remaining most significant bits are cleared to zero. So, in this case a 32-bit data word is made up every two pixel clock cycles.

The first captured data are placed in the LSB position in the 32-bit word and the 2 nd captured data are placed in the MSB position in the 32-bit word as shown in Table 81 .

**Table 81. Positioning of captured data bytes in 32-bit words (12-bit width)**

| Byte address | 31:28 | 27:16 | 15:12 | 11:0 |
|---|---|---|---|---|
| 0 | D [11:0] n+1 | 0 | D [11:0] n |  |
| 4 | 0 | D [11:0] n+3 | 0 | D [11:0] n+2 |


**14-bit data**

When EDM[1:0] in DCMI_CR are programmed to “11”, the camera interface captures the 14-bit data at its input D[0..13] and stores them as the 14 least significant bits of a 16-bit word. The remaining most significant bits are cleared to zero. So, in this case a 32-bit data word is made up every two pixel clock cycles.

The first captured data are placed in the LSB position in the 32-bit word and the 2 nd captured data are placed in the MSB position in the 32-bit word as shown in Table 82 .

**Table 82. Positioning of captured data bytes in 32-bit words (14-bit width)**

| Byte address | 31:30 | 29:16 | 15:14 | 13:0 |
|---|---|---|---|---|
| 0 | D [13:0] n+1 | 0 | D [13:0] n |  |
| 4 | 0 | D [13:0] n+3 | 0 | D [13:0] n+2 |


**15.5.3 Synchronization**

The digital camera interface supports embedded or hardware (HSYNC & VSYNC)

synchronization. When embedded synchronization is used, it is up to the digital camera module to make sure that the 0x00 and 0xFF values are used ONLY for synchronization (not in data). Embedded synchronization codes are supported only for the 8-bit parallel data interface width (that is, in the DCMI_CR register, the EDM[1:0] bits should be cleared to “00”).

For compressed data, the DCMI supports only the hardware synchronization mode. In this case, VSYNC is used as a start/end of the image, and HSYNC is used as a Data Valid signal. Figure 75 shows the corresponding timing diagram.



479

--- Page 70 ---

**Digital camera interface (DCMI)**

**RM0090**

**Figure 75. Timing diagram**

Padding data at the end of the JPEG stream Programmable

Beginning of JPEG stream

JPEG packet size

JPEG data

End of JPEG stream

DCMI_HSYNC

DCMI_VSYNC

Packet dispatching depends on the image content.

This results in a variable blanking duration.

JPEG packet data

ai15944b



--- Page 71 ---

**RM0090 Digital camera interface (DCMI)**

**Hardware synchronization mode**

In hardware synchronisation mode, the two synchronization signals (HSYNC/VSYNC) are used.

Depending on the camera module/mode, data may be transmitted during horizontal/vertical synchronisation periods. The HSYNC/VSYNC signals act like blanking signals since all the data received during HSYNC/VSYNC active periods are ignored.

In order to correctly transfer images into the DMA/RAM buffer, data transfer is synchronized with the VSYNC signal. When the hardware synchronisation mode is selected, and capture is enabled (CAPTURE bit set in DCMI_CR), data transfer is synchronized with the deactivation of the VSYNC signal (next start of frame).

Transfer can then be continuous, with successive frames transferred by DMA to successive buffers or the same/circular buffer. To allow the DMA management of successive frames, a VSIF (Vertical synchronization interrupt flag) is activated at the end of each frame.

**Embedded data synchronization mode**

In this synchronisation mode, the data flow is synchronised using 32-bit codes embedded in the data flow. These codes use the 0x00/0xFF values that are not used in data anymore. There are 4 types of codes, all with a 0xFF0000XY format. The embedded synchronization codes are supported only in 8-bit parallel data width capture (in the DCMI_CR register, the EDM[1:0] bits should be programmed to “00”). For other data widths, this mode generates unpredictable results and must not be used.

Note: Camera modules can have 8 such codes (in interleaved mode). For this reason, the interleaved mode is not supported by the camera interface (otherwise, every other half- frame would be discarded).

• Mode 2

Four embedded codes signal the following events

– Frame start (FS)

– Frame end (FE)

– Line start (LS)

– Line end (LE)

The XY values in the 0xFF0000XY format of the four codes are programmable (see Section 15.8.7: DCMI embedded synchronization code register (DCMI_ESCR) ).

A 0xFF value programmed as a “frame end” means that all the unused codes are interpreted as valid frame end codes.

In this mode, once the camera interface has been enabled, the frame capture starts after the first occurrence of the frame end (FE) code followed by a frame start (FS) code.

• Mode 1

An alternative coding is the camera mode 1. This mode is ITU656 compatible.

The codes signal another set of events:

– SAV (active line) - line start

– EAV (active line) - line end

– SAV (blanking) - end of line during interframe blanking period

– EAV (blanking) - end of line during interframe blanking period



479

--- Page 72 ---

**Digital camera interface (DCMI) RM0090**

This mode can be supported by programming the following codes:

• FS ≤ 0xFF

• FE ≤ 0xFF

• LS ≤ SAV (active)

• LE ≤ EAV (active)

An embedded unmask code is also implemented for frame/line start and frame/line end codes. Using it, it is possible to compare only the selected unmasked bits with the programmed code. You can therefore select a bit to compare in the embedded code and detect a frame/line start or frame/line end. This means that there can be different codes for the frame/line start and frame/line end with the unmasked bit position remaining the same.

**Example**

FS = 0xA5

Unmask code for FS = 0x10

In this case the frame start code is embedded in the bit 4 of the frame start code.

**15.5.4 Capture modes**

This interface supports two types of capture: snapshot (single frame) and continuous grab.

**Snapshot mode (single frame)**

In this mode, a single frame is captured (CM = ‘1’ in the DCMI_CR register). After the CAPTURE bit is set in DCMI_CR, the interface waits for the detection of a start of frame before sampling the data. The camera interface is automatically disabled (CAPTURE bit cleared in DCMI_CR) after receiving the first complete frame. An interrupt is generated (IT_FRAME) if it is enabled.

In case of an overrun, the frame is lost and the CAPTURE bit is cleared.

**Figure 76. Frame capture waveforms in Snapshot mode**

DCMI_HSYNC

DCMI_VSYNC

Frame 2 Frame 1 captured not captured

ai15832b

1. Here, the active state of DCMI_HSYNC and DCMI_VSYNC is 1.

2. DCMI_HSYNC and DCMI_VSYNC can change states at the same time.



--- Page 73 ---

**RM0090 Digital camera interface (DCMI)**

**Continuous grab mode**

In this mode (CM bit = ‘0’ in DCMI_CR), once the CAPTURE bit has been set in DCMI_CR, the grabbing process starts on the next VSYNC or embedded frame start depending on the mode. The process continues until the CAPTURE bit is cleared in DCMI_CR. Once the CAPTURE bit has been cleared, the grabbing process continues until the end of the current frame.

**Figure 77. Frame capture waveforms in continuous grab mode**

DCMI_HSYNC

DCMI_VSYNC

Frame 1 captured Frame 2 captured

ai15833b

1. Here, the active state of DCMI_HSYNC and DCMI_VSYNC is 1.

2. DCMI_HSYNC and DCMI_VSYNC can change states at the same time.

In continuous grab mode, you can configure the FCRC bits in DCMI_CR to grab all pictures, every second picture or one out of four pictures to decrease the frame capture rate.

Note: In the hardware synchronization mode (ESS = ‘0’ in DCMI_CR), the IT_VSYNC interrupt is generated (if enabled) even when CAPTURE = ‘0’ in DCMI_CR so, to reduce the frame capture rate even further, the IT_VSYNC interrupt can be used to count the number of frames between 2 captures in conjunction with the Snapshot mode. This is not allowed by embedded data synchronization mode.

**15.5.5 Crop feature**

With the crop feature, the camera interface can select a rectangular window from the received image. The start (upper left corner) coordinates and size (horizontal dimension in number of pixel clocks and vertical dimension in number of lines) are specified using two 32- bit registers (DCMI_CWSTRT and DCMI_CWSIZE). The size of the window is specified in number of pixel clocks (horizontal dimension) and in number of lines (vertical dimension).



479

--- Page 74 ---

**Digital camera interface (DCMI) RM0090**

**Figure 78. Coordinates and size of the window after cropping**



VST bit in DCMI_CSTRT





VLINE bit in DCMI_CSIZE

HOFFCNT bit in DCMI_CSTRT

CAPCNT bit in DCMI_CSIZE

ai15834

These registers specify the coordinates of the starting point of the capture window as a line number (in the frame, starting from 0) and a number of pixel clocks (on the line, starting from 0), and the size of the window as a line number and a number of pixel clocks. The CAPCNT value can only be a multiple of 4 (two least significant bits are forced to 0) to allow the correct transfer of data through the DMA.

If the VSYNC signal goes active before the number of lines is specified in the DCMI_CWSIZE register, then the capture stops and an IT_FRAME interrupt is generated when enabled.

**Figure 79. Data capture waveforms**

DCMI_HSYNC

DCMI_VSYNC

HOFFCNT

CAPCNT

Data not captured in this phase

Data captured in this phase

ai15835

1. Here, the active state of DCMI_HSYNC and DCMI_VSYNC is 1.

2. DCMI_HSYNC and DCMI_VSYNC can change states at the same time.



--- Page 75 ---

**RM0090 Digital camera interface (DCMI)**

**15.5.6 JPEG format**

To allow JPEG image reception, it is necessary to set the JPEG bit in the DCMI_CR register. JPEG images are not stored as lines and frames, so the VSYNC signal is used to start the capture while HSYNC serves as a data enable signal. The number of bytes in a line may not be a multiple of 4, you should therefore be careful when handling this case since a DMA request is generated each time a complete 32-bit word has been constructed from the captured data. When an end of frame is detected and the 32-bit word to be transferred has not been completely received, the remaining data are padded with ‘0s’ and a DMA request is generated.

The crop feature and embedded synchronization codes cannot be used in the JPEG format.

**15.5.7 FIFO**

A four-word FIFO is implemented to manage data rate transfers on the AHB. The DCMI features a simple FIFO controller with a read pointer incremented each time the camera interface reads from the AHB, and a write pointer incremented each time the camera interface writes to the FIFO. There is no overrun protection to prevent the data from being overwritten if the AHB interface does not sustain the data transfer rate.

In case of overrun or errors in the synchronization signals, the FIFO is reset and the DCMI interface waits for a new start of frame.


**15.6 Data format description**

**15.6.1 Data formats**

Three types of data are supported:

• 8-bit progressive video: either monochrome or raw Bayer format

• YCbCr 4:2:2 progressive video

• RGB565 progressive video. A pixel coded in 16 bits (5 bits for blue, 5 bits for red, 6 bits

for green) takes two clock cycles to be transferred.

Compressed data: JPEG

For B&W, YCbCr or RGB data, the maximum input size is 2048 × 2048 pixels. No limit in JPEG compressed mode.

For monochrome, RGB & YCbCr, the frame buffer is stored in raster mode. 32-bit words are used. Only the little endian format is supported.

**Figure 80. Pixel raster scan order**

| Word 0 | Word 1 | Word 2 |  |  |
|---|---|---|---|---|
| помощью плоской шпонки |  |  |  |  |
| помощью шлица. Когда с |  |  |  |  |
| лие воздействует на якор |  |  |  |  |
| м(9), захватывается межд |  |  |  |  |


Pixel row 0

Pixel raster

scan order

(increasing

addresses)

Pixel row n – 1

ai15848



479

--- Page 76 ---

**Digital camera interface (DCMI) RM0090**

**15.6.2 Monochrome format**

Characteristics:

• Raster format

• 8 bits per pixel

Table 83 shows how the data are stored.

**Table 83. Data storage in monochrome progressive video format**

| Byte address | 31:24 | 23:16 | 15:8 | 7:0 |
|---|---|---|---|---|
| 0 | n + 3 | n + 2 | n + 1 | n |
| 4 | n + 7 | n + 6 | n + 5 | n + 4 |


**15.6.3 RGB format**

Characteristics:

• Raster format

• RGB

• Interleaved: one buffer: R, G & B interleaved: BRGBRGBRG, etc.

• Optimized for display output

The RGB planar format is compatible with standard OS frame buffer display formats. Only 16 BPP (bits per pixel): RGB565 (2 pixels per 32-bit word) is supported.

The 24 BPP (palletized format) and grayscale formats are not supported. Pixels are stored in a raster scan order, that is from top to bottom for pixel rows, and from left to right within a pixel row. Pixel components are R (red), G (green) and B (blue). All components have the same spatial resolution (4:4:4 format). A frame is stored in a single part, with the components interleaved on a pixel basis.

Table 84 shows how the data are stored.

**Table 84. Data storage in RGB progressive video format**

| Byte address | 31:27 | 26:21 | 20:16 | 15:11 | 10:5 | 4:0 |
|---|---|---|---|---|---|---|
| 0 | Red n + 1 | Green n + 1 | Blue n + 1 | Red n | Green n | Blue n |
| 4 | Red n + 4 | Green n + 3 | Blue n + 3 | Red n + 2 | Green n + 2 | Blue n + 2 |


**15.6.4 YCbCr format**

Characteristics:

• Raster format

• YCbCr 4:2:2

• Interleaved: one Buffer: Y, Cb & Cr interleaved: CbYCrYCbYCr, etc.

Pixel components are Y (luminance or “luma”), Cb and Cr (chrominance or “chroma” blue and red). Each component is encoded in 8 bits. Luma and chroma are stored together (interleaved) as shown in Table 85 .



--- Page 77 ---

**RM0090**

**Digital camera interface (DCMI)**

**Table 85. Data storage in YCbCr progressive video format**

| Byte address | 31:24 | 23:16 | 15:8 | 7:0 |
|---|---|---|---|---|
| 0 | Y n + 1 | Cr n | Y n | Cb n |
| 4 | Y n + 3 | Cr n + 2 | Y n + 2 | Cb n + 2 |


**15.7 DCMI interrupts**

Five interrupts are generated. All interrupts are maskable by software. The global interrupt

(IT_DCMI) is the OR of all the individual interrupts.

Table 86 gives the list of all interrupts.

**Table 86. DCMI interrupts**

| Interrupt name | Interrupt event |
|---|---|
| IT LINE _ | Indicates the end of line |
| IT FRAME _ | Indicates the end of frame capture |
| IT OVR _ | indicates the overrun of data reception |
| IT VSYNC _ | Indicates the synchronization frame |
| IT ERR _ | Indicates the detection of an error in the embedded synchronization frame detection |
| IT DCMI _ | Logic OR of the previous interrupts |



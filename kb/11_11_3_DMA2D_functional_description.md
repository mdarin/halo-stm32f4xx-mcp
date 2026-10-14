## 11.3 DMA2D functional description

### 11.3.1 General description

The DMA2D controller performs direct memory transfer. As an AHB master, it can take the control of the AHB bus matrix to initiate AHB transactions.

The DMA2D can operate in the following modes:

*   Register-to-memory
*   Memory-to-memory
*   Memory-to-memory with Pixel Format Conversion
*   Memory-to-memory with Pixel Format Conversion and Blending

The AHB slave port is used to program the DMA2D controller.

The block diagram of the DMA2D is shown in Figure 40: DMA2D block diagram.

340/1749
RM0090 Rev 18
ST

--- Page 146 ---

RM0090 Chrom-Art Accelerator™ controller (DMA2D)

Figure 40. DMA2D block diagram

AHB MASTER

FG PFC
Color mode
α mode
8-bit α
FG FIFO
Expander
32
α 8
X
α
32
32
RGB
CLUT itf
256x32-bit RAM

BG PFC
Color mode
α mode
8-bit α
BG FIFO
Expander
32
α 8
X
α
32
32
RGB
CLUT itf
256x32-bit RAM

BLENDER
32
α
Red
Green
Blue
32

OUT PFC
Color mode
Converter
32
32/24/16
Color
OUT FIFO

AHB SLAVE

MS30439V1

11.3.2 DMA2D control

The DMA2D controller is configured through the DMA2D Control Register (DMA2D_CR) which allows selecting:

The user application can perform the following operations:
* Select the operating mode
* Enable/disable the DMA2D interrupt
* Start/suspend/abort ongoing data transfers

11.3.3 DMA2D foreground and background FIFOs

The DMA2D foreground (FG) FG FIFO and background (BG) FIFO fetch the input data to be copied and/or processed.

The FIFOs fetch the pixels according to the color format defined in their respective pixel format converter (PFC).

STI
RM0090 Rev 18
341/1749

--- Page 147 ---

Chrom-Art Accelerator™ controller (DMA2D)
RM0090

They are programmed through a set of control registers:
* DMA2D foreground memory address register (DMA2D_FGMAR)
* DMA2D foreground offset register (DMA2D_FGOR)
* DMA2D background memory address register (DMA2D_BGMAR)
* DMA2D background offset register (DMA2D_BGBOR)
* DMA2D number of lines register (number of lines and pixel per lines) (DMA2D_NLR)

When the DMA2D operates in register-to-memory mode, none of the FIFOs is activated.

When the DMA2D operates in memory-to-memory mode (no pixel format conversion nor blending operation), only the FG FIFO is activated and acts as a buffer.

When the DMA2D operates in memory-to-memory operation with pixel format conversion (no blending operation), the BG FIFO is not activated.

### 11.3.4 DMA2D foreground and background pixel format converter (PFC)

DMA2D foreground pixel format converter (PFC) and background pixel format converter perform the pixel format conversion to generate a 32-bit per pixel value. The PFC can also modify the alpha channel.

The first stage of the converter converts the color format. The original color format of the foreground pixel and background pixels are configured through the CM[3:0] bits of the DMA2D_FGPFCR and DMA2D_BGPFCR, respectively.

The supported input formats are given in Table 52: *Supported color mode in input*.

**Table 52. Supported color mode in input**

| CM[3:0] | Color mode |
|---|---|
| 0000 | ARGB8888 |
| 0001 | RGB888 |
| 0010 | RGB565 |
| 0011 | ARGB1555 |
| 0100 | ARGB4444 |
| 0101 | L8 |
| 0110 | AL44 |
| 0111 | AL88 |
| 1000 | L4 |
| 1001 | A8 |
| 1010 | A4 |


342/1749
RM0090 Rev 18
ST

--- Page 148 ---

RM0090
Chrom-Art Accelerator™ controller (DMA2D)

The color format are coded as follows:

*   Alpha value field: transparency
    0xFF value corresponds to an opaque pixel and 0x00 to a transparent one.
*   R field for Red
*   G field for Green
*   B field for Blue
*   L field: luminance
    This field is the index to a CLUT to retrieve the three/four RGB/ARGB components.

If the original format was direct color mode, then the extension to 8-bit per channel is performed by copying the MSBs into the LSBs. This ensures a perfect linearity of the conversion.

If the original format does not include an alpha channel, the alpha value is automatically set to 0xFF (opaque).

If the original format is indirect color mode, a CLUT is required and each pixel format converter is associated with a 256 entry 32-bit CLUT.

For the specific alpha mode A4 and A8, no color information is stored nor indexed. The color to be used for the image generation is fixed and is defined in the DMA2D_FGCOLR for foreground pixels and in the DMA2D_BGCOLR register for background pixels.

The order of the fields in the system memory is defined in Table 53: Data order in memory.

Table 53. Data order in memory

| Color Mode | @ + 3 | @ + 2 | @ + 1 | @ + 0 |
|---|---|---|---|---|
| ARGB8888 | A0[7:0] | R0[7:0] | G0[7:0] | B0[7:0] |
| RGB888 | B1[7:0] | R0[7:0] | G0[7:0] | B0[7:0] |
| G2[7:0] | B2[7:0] | R1[7:0] | G1[7:0] |  |
| R3[7:0] | G3[7:0] | B3[7:0] | R2[7:0] |  |
| RGB565 | R1[4:0]G1[5:3] | G1[2:0]B1[4:0] | R0[4:0]G0[5:3] | G0[2:0]B0[4:0] |
| ARGB1555 | A1[0]R1[4:0]G1[4:3] | G1[2:0]B1[4:0] | A0[0]R0[4:0]G0[4:3] | G0[2:0]B0[4:0] |
| ARGB4444 | A1[3:0]R1[3:0] | G1[3:0]B1[3:0] | A0[3:0]R0[3:0] | G0[3:0]B0[3:0] |
| L8 | L3[7:0] | L2[7:0] | L1[7:0] | L0[7:0] |
| AL44 | A3[3:0]L3[3:0] | A2[3:0]L2[3:0] | A1[3:0]L1[3:0] | A0[3:0]L0[3:0] |
| AL88 | A1[7:0] | L1[7:0] | A0[7:0] | L0[7:0] |
| L4 | L7[3:0]L6[3:0] | L5[3:0]L4[3:0] | L3[3:0]L2[3:0] | L1[3:0]L0[3:0] |
| A8 | A3[7:0] | A2[7:0] | A1[7:0] | A0[7:0] |
| A4 | A7[3:0]A6[3:0] | A5[3:0]A4[3:0] | A3[3:0]A2[3:0] | A1[3:0]A0[3:0] |


The 24-bit RGB888 aligned on 32-bit is supported through the ARGB8888 mode.

Once the 32-bit value is generated, the alpha channel can be modified according to the AM[1:0] field of the DMA2D_FGPFCCR/DMA2D_BGPFCCR registers as shown in Table 54: Alpha mode configuration.

ST
RM0090 Rev 18
343/1749

--- Page 149 ---

Chrom-Art Accelerator™ controller (DMA2D)
RM0090

The alpha channel can be:
* kept as it is (no modification),
* replaced by the ALPHA[7:0] value of DMA2D_FGPFCCR/DMA2D_BGPFCCR,
* or replaced by the original alpha value multiplied by the ALPHA[7:0] value of DMA2D_FGPFCCR/DMA2D_BGPFCCR divided by 255.

Table 54. Alpha mode configuration

| AM[1:0] | Alpha mode |
|---|---|
| 00 | No modification |
| 01 | Replaced by value in DMA2D_xxPFCCR |
| 10 | Replaced by original value multiplied by the value in DMA2D_xxPFCCR / 255 |
| 11 | Reserved |


### 11.3.5 DMA2D foreground and background CLUT interface

The CLUT interface manages the CLUT memory access and the automatic loading of the CLUT.

Three kinds of accesses are possible:
* CLUT read by the PFC during pixel format conversion operation
* CLUT accessed through the AHB slave port when the CPU is reading or writing data into the CLUT
* CLUT written through the AHB master port when an automatic loading of the CLUT is performed

The CLUT memory loading can be done in two different ways:
* Automatic loading
  The following sequence should be followed to load the CLUT:
  a) Program the CLUT address into the DMA2D_FGCMAR register (foreground CLUT) or DMA2D_BGCMAR register (background CLUT)
  b) Program the CLUT size in the CS[7:0] field of the DMA2D_FGPFCCR register (foreground CLUT) or DMA2D_BGPFCCR register (background CLUT).
  c) Set the START bit of the DMA2D_FGPFCCR register (foreground CLUT) or DMA2D_BGPFCCR register (background CLUT) to start the transfer. During this automatic loading process, the CLUT is not accessible by the CPU. If a conflict occurs, a CLUT access error interrupt is raised assuming CAEIE is set to '1' in DMA2D_CR.
* Manual loading
  The application has to program the CLUT manually through the DMA2D AHB slave port to which the local CLUT memory is mapped. The foreground CLUT is located at address offset 0x0400 and the background CLUT at address offset 0x0800.

The CLUT format can be 24 or 32 bits. It is configured through the CCM bit of the DMA2D_FGPFCCR register (foreground CLUT) or DMA2D_BGPFCCR register (background CLUT) as shown in Table 55: Supported CLUT color mode.

344/1749
RM0090 Rev 18
ST

--- Page 150 ---

RM0090
Chrom-Art Accelerator™ controller (DMA2D)

**Table 55. Supported CLUT color mode**

| CCM | CLUT color mode |
|---|---|
| 0 | 32-bit ARGB8888 |
| 1 | 24-bit RGB888 |


The way the CLUT data are organized in the system memory is specified in Table 56: CLUT data order in memory.

**Table 56. CLUT data order in memory**

| CLUT Color Mode | @ + 3 | @ + 2 | @ + 1 | @ + 0 |
|---|---|---|---|---|
| ARGB8888 | A0[7:0] | R0[7:0] | G0[7:0] | B0[7:0] |
| RGB888 | B1[7:0] | R0[7:0] | G0[7:0] | B0[7:0] |
| G2[7:0] | B2[7:0] | R1[7:0] | G1[7:0] |  |
| R3[7:0] | G3[7:0] | B3[7:0] | R2[7:0] |  |


### 11.3.6 DMA2D blender

The DMA2D blender blends the source pixels by pair to compute the resulting pixel.
The blending is performed according to the following equation:

with $\alpha_{Mult} = \frac{\alpha_{FG} \cdot \alpha_{BG}}{255}$

$\alpha_{OUT} = \alpha_{FG} + \alpha_{BG} - \alpha_{Mult}$

$C_{OUT} = \frac{C_{FG} \cdot \alpha_{FG} + C_{BG} \cdot \alpha_{BG} - C_{BG} \cdot \alpha_{Mult}}{\alpha_{OUT}}$ with C = R or G or B

Division is rounded to the nearest lower integer

No configuration register is required by the blender. The blender usage depends on the DMA2D operating mode defined in MODE[1:0] field of the DMA2D_CR register.

### 11.3.7 DMA2D output PFC

The output PFC performs the pixel format conversion from 32 bits to the output format defined in the CM[2:0] field of the DMA2D output pixel format converter configuration register (DMA2D_OPFCCR).
The supported output formats are given in Table 57: Supported color mode in output

ST
RM0090 Rev 18
345/1749

--- Page 151 ---

Chrom-Art Accelerator™ controller (DMA2D)
RM0090

Table 57. Supported color mode in output

| CM[2:0] | Color mode |
|---|---|
| 000 | ARGB8888 |
| 001 | RGB888 |
| 010 | RGB565 |
| 011 | ARGB1555 |
| 100 | ARGB4444 |


### 11.3.8 DMA2D output FIFO

The output FIFO programs the pixels according to the color format defined in the output PFC.

The destination area is defined through a set of control registers:
* DMA2D output memory address register (DMA2D_OMAR)
* DMA2D output offset register (DMA2D_OOR)
* DMA2D number of lines register (number of lines and pixel per lines) (DMA2D_NLR)

If the DMA2D operates in register-to-memory mode, the configured output rectangle is filled by the color specified in the DMA2D output color register (DMA2D_OCOLR) which contains a fixed 32-bit, 24-bit or 16-bit value. The format is selected by the CM[2:0] field of the DMA2D_OPFCCR register.

The data are stored into the memory in the order defined in Table 58: Data order in memory

Table 58. Data order in memory

| Color Mode | @ + 3 | @ + 2 | @ + 1 | @ + 0 |
|---|---|---|---|---|
| ARGB8888 | A0[7:0] | R0[7:0] | G0[7:0] | B0[7:0] |
| RGB888 | B1[7:0] | R0[7:0] | G0[7:0] | B0[7:0] |
| G2[7:0] | B2[7:0] | R1[7:0] | G1[7:0] |  |
| R3[7:0] | G3[7:0] | B3[7:0] | R2[7:0] |  |
| RGB565 | R1[4:0]G1[5:3] | G1[2:0]B1[4:0] | R0[4:0]G0[5:3] | G0[2:0]B0[4:0] |
| ARGB1555 | A1[0]R1[4:0]G1[4:3] | G1[2:0]B1[4:0] | A0[0]R0[4:0]G0[4:3] | G0[2:0]B0[4:0] |
| ARGB4444 | A1[3:0]R1[3:0] | G1[3:0]B1[3:0] | A0[3:0]R0[3:0] | G0[3:0]B0[3:0] |


The RGB888 aligned on 32-bit is supported through the ARGB8888 mode.

### 11.3.9 DMA2D AHB master port timer

An 8-bit timer is embedded into the AHB master port to provide an optional limitation of the bandwidth on the crossbar.

This timer is clocked by the AHB clock and counts a dead time between two consecutive accesses. This limits the bandwidth usage.

346/1749
RM0090 Rev 18

--- Page 152 ---

RM0090
Chrom-Art Accelerator™ controller (DMA2D)

The timer enabling and the dead time value are configured through the AHB master port timer configuration register (DMA2D_AMPTCR).

### 11.3.10 DMA2D transactions

DMA2D transactions consist of a sequence of a given number of data transfers. The number of data and the width can be programmed by software.

Each DMA2D data transfer is composed of up to 4 steps:

1. Data loading from the memory location pointed by the DMA2D_FGMAR register and pixel format conversion as defined in DMA2D_FGCR.
2. Data loading from a memory location pointed by the DMA2D_BGMAR register and pixel format conversion as defined in DMA2D_BGCR.
3. Blending of all retrieved pixels according to the alpha channels resulting of the PFC operation on alpha values.
4. Pixel format conversion of the resulting pixels according to the DMA2D_OCR register and programming of the data to the memory location addressed through the DMA2D_OMAR register.

### 11.3.11 DMA2D configuration

Both source and destination data transfers can target peripherals and memories in the whole 4 Gbyte memory area, at addresses ranging between 0x0000 0000 and 0xFFFF FFFF.

The DMA2D can operate in any of the four following modes selected through MODE[1:0] bits of the DMA2D_CR register:

* Register-to-memory
* Memory-to-memory
* Memory-to-memory with PFC
* Memory-to-memory with PFC and blending

**Register-to-memory**

The register-to-memory mode is used to fill a user defined area with a predefined color.

The color format is set in the DMA2D_OPFCCR.

The DMA2D does not perform any data fetching from any source. It just writes the color defined in the DMA2D_OCOLR register to the area located at the address pointed by the DMA2D_OMAR and defined in the DMA2D_NLR and DMA2D_OOR.

**Memory-to-memory**

In memory-to-memory mode, the DMA2D does not perform any graphical data transformation. The foreground input FIFO acts as a buffer and the data are transferred from the source memory location defined in DMA2D_FGMAR to the destination memory location pointed by DMA2D_OMAR.

The color mode programmed in the CM[3:0] bits of the DMA2D_FGPFCCR register defines the number of bits per pixel for both input and output.

ST
RM0090 Rev 18
347/1749

--- Page 153 ---

Chrom-Art Accelerator™ controller (DMA2D)
RM0090

The size of the area to be transferred is defined by the DMA2D_NLR and DMA2D_FGOR registers for the source, and by DMA2D_NLR and DMA2D_OOR registers for the destination.

**Memory-to-memory with PFC**

In this mode, the DMA2D performs a pixel format conversion of the source data and stores them in the destination memory location.

The size of the areas to be transferred are defined by the DMA2D_NLR and DMA2D_FGOR registers for the source, and by DMA2D_NLR and DMA2D_OOR registers for the destination.

Data are fetched from the location defined in the DMA2D_FGMAR register and processed by the foreground PFC. The original pixel format is configured through the DMA2D_FGPFCCR register.

If the original pixel format is direct color mode, then the color channels are all expanded to 8 bits.

If the pixel format is indirect color mode, the associated CLUT has to be loaded into the CLUT memory.

The CLUT loading can be done automatically by following the sequence below:

1. Set the CLUT address into the DMA2D_FGCMAR.
2. Set the CLUT size in the CS[7:0] bits of the DMA2D_FGPFCCR register.
3. Set the CLUT format (24 or 32 bits) in the CCM bit of the DMA2D_FGPFCCR register.
4. Start the CLUT loading by setting the START bit of the DMA2D_FGPFCCR register.

Once the CLUT loading is complete, the CTCIF flag of the DMA2D_IFR register is raised, and an interrupt is generated if the CTCIE bit is set in DMA2D_CR. The automatic CLUT loading process can not work in parallel with classical DMA2D transfers.

The CLUT can also be filled by the CPU or by any other master through the APB port. The access to the CLUT is not possible when a DMA2D transfer is ongoing and uses the CLUT (indirect color format).

In parallel to the color conversion process, the alpha value can be added or changed depending on the value programmed in the DMA2D_FGPFCCR register. If the original image does not have an alpha channel, a default alpha value of 0xFF is automatically added to obtain a fully opaque pixel. The alpha value can be modified according to the AM[1:0] bits of the DMA2D_FGPFCCR register:

* It can be unchanged.
* It can be replaced by the value defined in the ALPHA[7:0] value of the DMA2D_FGPFCCR register.
* It can be replaced by the original value multiplied by the ALPHA[7:0] value of the DMA2D_FGPFCCR register divided by 255.

The resulting 32-bit data are encoded by the OUT PFC into the format specified by the CM[2:0] field of the DMA2D_OPFCCR register. The output pixel format cannot be the indirect mode since no CLUT generation process is supported.

The processed data are written into the destination memory location pointed by DMA2D_OMAR.

348/1749
RM0090 Rev 18

--- Page 154 ---

RM0090
Chrom-Art Accelerator™ controller (DMA2D)

**Memory-to-memory with PFC and blending**

In this mode, 2 sources are fetched in the foreground FIFO and background FIFO from the memory locations defined by DMA2D_FGMAR and DMA2D_BGMAR.

The two pixel format converters have to be configured as described in the memory-to-memory mode. Their configurations can be different as each pixel format converter are independent and have their own CLUT memory.

Once each pixel has been converted into 32 bits by their respective PFCs, they are blended according to the equation below:

with $\alpha_{Mult} = \frac{\alpha_{FG} \cdot \alpha_{BG}}{255}$

$\alpha_{OUT} = \alpha_{FG} + \alpha_{BG} - \alpha_{Mult}$

$C_{OUT} = \frac{C_{FG} \cdot \alpha_{FG} + C_{BG} \cdot \alpha_{BG} - C_{BG} \cdot \alpha_{Mult}}{\alpha_{OUT}}$ with C = R or G or B

*Division are rounded to the nearest lower integer*

The resulting 32-bit pixel value is encoded by the output PFC according to the specified output format, and the data are written into the destination memory location pointed by DMA2D_OMAR.

**Configuration error detection**

The DMA2D checks that the configuration is correct before any transfer. The configuration error interrupt flag is set by hardware when a wrong configuration is detected when a new transfer/automatic loading starts. An interrupt is then generated if the CEIE bit of the DMA2D_CR is set.

ST
RM0090 Rev 18
349/1749

--- Page 155 ---

**Chrom-Art Accelerator™ controller (DMA2D)**
RM0090

The wrong configurations that can be detected are listed below:

*   Foreground CLUT automatic loading: MA bits of DMA2D_FGCMAR not aligned with CCM of DMA2D_FGPFCCR.
*   Background CLUT automatic loading: MA of DMA2D_BGC MAR not aligned with CCM of DMA2D_BGP FCCR
*   Memory transfer (except in register-to-memory mode): MA of DMA2D_FGMAR not aligned with CM of DMA2D_FGPFCCR
*   Memory transfer (except in register-to-memory mode): CM in DMA2D_FGPFCCR are invalid
*   Memory transfer (except in register-to-memory mode): PL bits of DMA2D_NLR odd while CM of DMA2D_FGPFCCR is A4 or L4
*   Memory transfer (except in register-to-memory mode): LO bits in DMA2D_FGOR odd while CM of DMA2D_FGPFCCR is A4 or L4
*   Memory transfer (only in blending mode): MA bits in DMA2D_BGMAR are not aligned with the CM of DMA2D_BGP FCCR
*   Memory transfer: CM of DMA2D_BGP FCCR invalid (only in blending mode)
*   Memory transfer (only in blending mode): PL bits of DMA2D_NLR odd while CM of DMA2D_BGP FCCR is A4 or L4
*   Memory transfer (only in blending mode): LO bits of DMA2D_BGOR odd while CM of DMA2D_BGP FCCR is A4 or L4
*   Memory transfer (except in memory to memory mode): MA bits in DMA2D_OMAR are not aligned with CM bits in DMA2D_OPFCCR.
*   Memory transfer (except in memory to memory mode): CM bits in DMA2D_OPFCCR invalid
*   Memory transfer: NL bits in DMA2D_NLR = 0
*   Memory transfer: PL bits in DMA2D_NLR = 0

**11.3.12 DMA2D transfer control (start, suspend, abort and completion)**

Once the DMA2D is configured, the transfer can be launched by setting the START bit of the DMA2D_CR register. Once the transfer is completed, the START bit is automatically reset and the TCIF flag of the DMA2D_ISR register is raised. An interrupt can be generated if the TCIE bit of the DMA2D_CR is set.

The user application can suspend the DMA2D at any time by setting the SUSP bit of the DMA2D_CR register. The transaction can then be aborted by setting the ABORT bit of the DMA2D_CR register or can be restarted by resetting the SUSP bit of the DMA2D_CR register.

The user application can abort at any time an ongoing transaction by setting the ABORT bit of the DMA2D_CR register. In this case, the TCIF flag is not raised.

Automatic CLUT transfers can also be aborted or suspended by using the ABORT or the SUSP bit of the DMA2D_CR register.

350/1749
RM0090 Rev 18

--- Page 156 ---

RM0090 Chrom-Art Accelerator™ controller (DMA2D)

### 11.3.13 Watermark

A watermark can be programmed to generate an interrupt when the last pixel of a given line has been written to the destination memory area.

The line number is defined in the LW[15:0] field of the DMA2D_LWR register.

When the last pixel of this line has been transferred, the TWIF flag of the DMA2D_ISR register is raised and an interrupt is generated if the TWIE bit of the DMA2D_CR is set.

### 11.3.14 Error management

Two kind of errors can be triggered:

*   AHB master port errors signalled by the TEIF flag of the DMA2D_ISR register.
*   Conflicts caused by CLUT access (CPU trying to access the CLUT while a CLUT loading or a DMA2D transfer is ongoing) signalled by the CAEIF flag of the DMA2D_ISR register.

Both flags are associated to their own interrupt enable flag in the DMA2D_CR register to generate an interrupt if need be (TEIE and CAEIE).

### 11.3.15 AHB dead time

To limit the AHB bandwidth usage, a dead time between two consecutive AHB accesses can be programmed.

This feature can be enabled by setting the EN bit in the DMA2D_AMTCR register.

The dead time value is stored in the DT[7:0] field of the DMA2D_AMTCR register. This value represents the guaranteed minimum number of cycles between two consecutive transactions on the AHB bus.

The update of the dead time value while the DMA2D is running will be taken into account for the next AHB transfer.

### 11.4 DMA2D interrupts

An interrupt can be generated on the following events:

*   Configuration error
*   CLUT transfer complete
*   CLUT access error
*   Transfer watermark reached
*   Transfer complete
*   Transfer error

Separate interrupt enable bits are available for flexibility.

Table 59. DMA2D interrupt requests

| Interrupt event | Event flag | Enable control bit |
|---|---|---|
| Configuration error | CEIF | CEIE |
| CLUT transfer complete | CTCIF | CTCIE |


ST
RM0090 Rev 18
351/1749

--- Page 157 ---

Chrom-Art Accelerator™ controller (DMA2D)
RM0090

Table 59. DMA2D interrupt requests (continued)

| Interrupt event | Event flag | Enable control bit |
|---|---|---|
| CLUT access error | CAEIF | CAEIE |
| Transfer watermark | TWF | TWIE |
| Transfer complete | TCIF | TCIE |
| Transfer error | TEIF | TEIE |



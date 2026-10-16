# 11 Chrom-Art Accelerator™ controller (DMA2D)

## 11.1 DMA2D introduction

The Chrom-Art Accelerator™ (DMA2D) is a specialized DMA dedicated to image manipulation. It can perform the following operations:

*   Filling a part or the whole of a destination image with a specific color
*   Copying a part or the whole of a source image into a part or the whole of a destination image
*   Copying a part or the whole of a source image into a part or the whole of a destination image with a pixel format conversion
*   Blending a part and/or two complete source images with different pixel format and copy the result into a part or the whole of a destination image with a different color format.

All the classical color coding schemes are supported from 4-bit up to 32-bit per pixel with indexed or direct color mode. The DMA2D has its own dedicated memories for CLUTs (color look-up tables).

ST
RM0090 Rev 18
339/1749

--- Page 145 ---

Chrom-Art Accelerator™ controller (DMA2D)
RM0090

## 11.2 DMA2D main features

The main DMA2D features are:

*   Single AHB master bus architecture.
*   AHB slave programming interface supporting 8/16/32-bit accesses (except for CLUT accesses which are 32-bit).
*   User programmable working area size
*   User programmable offset for sources and destination areas
*   User programmable sources and destination addresses on the whole memory space
*   Up to 2 sources with blending operation
*   Alpha value can be modified (source value, fixed value or modulated value)
*   User programmable source and destination color format
*   Up to 11 color formats supported from 4-bit up to 32-bit per pixel with indirect or direct color coding
*   2 internal memories for CLUT storage in indirect color mode
*   Automatic CLUT loading or CLUT programming via the CPU
*   User programmable CLUT size
*   Internal timer to control AHB bandwidth
*   4 operating modes: register-to-memory, memory-to-memory, memory-to-memory with pixel format conversion, and memory-to-memory with pixel format conversion and blending
*   Area filling with a fixed color
*   Copy from an area to another
*   Copy with pixel format conversion between source and destination images
*   Copy from two sources with independent color format and blending
*   Abort and suspend of DMA2D operations
*   Watermark interrupt on a user programmable destination line
*   Interrupt generation on bus error or access conflict
*   Interrupt generation on process completion


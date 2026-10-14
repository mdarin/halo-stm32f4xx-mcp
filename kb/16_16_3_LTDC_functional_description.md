## 16 LCD-TFT controller (LTDC)

This section applies only to STM32F429xx/439xx devices.

**16.1 Introduction**

The LCD-TFT (Liquid Crystal Display - Thin Film Transistor) display controller provides a parallel digital RGB (Red, Green, Blue) and signals for horizontal, vertical synchronisation, Pixel Clock and Data Enable as output to interface directly to a variety of LCD and TFT panels.

**16.2 LTDC main features**

• 24-bit RGB Parallel Pixel Output; 8 bits-per-pixel (RGB888)

• 2 display layers with dedicated FIFO (64x32-bit)

• Color Look-Up Table (CLUT) up to 256 color (256x24-bit) per layer

• Supports up to XGA (1024x768) resolution

• Programmable timings for different display panels

• Programmable Background color

• Programmable polarity for HSync, VSync and Data Enable

• Up to 8 Input color formats selectable per layer

– ARGB8888

– RGB888

– RGB565

– ARGB1555

– ARGB4444

– L8 (8-bit Luminance or CLUT)

– AL44 (4-bit alpha + 4-bit luminance)

– AL88 (8-bit alpha + 8-bit luminance)

• Pseudo-random dithering output for low bits per channel

– Dither width 2-bits for Red, Green, Blue

• Flexible blending between two layers using alpha value (per pixel or constant)

• Color Keying (transparency color)

• Programmable Window position and size

• Supports thin film transistor (TFT) color displays

• AHB master interface with burst of 16 words

• Up to 4 programmable interrupt events

480/1749 RM0090 Rev 18

--- Page 91 ---

**RM0090 LCD-TFT controller (LTDC)**

**16.3 LTDC functional description**

**16.3.1 LTDC block diagram**

The block diagram of the LTDC is shown in Figure 81: LTDC block diagram .

**Figure 81. LTDC block diagram**

Pixel clock domain

| AHB interface | Layer1 PFC FIFO Blending Dithering unit unit Layer1 PFC FIFO Timing generator |
|---|---|
| Configuration and status registers Configuration and status registers |  |


AHB clock domain LCD_VSYNC LCD_DE LCD_CLK LCD_HSYNC LCD-TFT panel

LCD_R[7:0] LCD_G[7:0] APB2 clock domain LCD_B[7:0]

Interrupts

MSv19675V1

Layer FIFO: One FIFO 64x32 bit per layer.

PFC: Pixel Format Convertor performing the pixel format conversion from the selected input pixel format of a layer to words.

AHB interface: For data transfer from memories to the FIFO.

Blending, Dithering unit and Timings Generator: Refer to Section 16.4.1 and Section 16.4.2 .

**16.3.2 LTDC reset and clocks**

The LCD-TFT controller peripheral uses 3 clock domains:

• The AHB clock domain (HCLK) is used for data transfer from the memories to the

Layer FIFO and frame buffer configuration register

• The APB2 clock domain (PCLK2) is used for global configuration register and interrupt

registers

• The Pixel Clock domain (LCD_CLK) is used to generate LCD-TFT interface signals,

pixel data generation and layer configuration. The LCD_CLK output should be configured following the panel requirements. The LCD_CLK is configured through the PLLSAI (refer to RCC section).

Table 88 summarizes the clock domain for each register.



514

--- Page 92 ---

**LCD-TFT controller (LTDC) RM0090**

**Table 88. LTDC registers versus clock domain**

| LTDC registers | Clock domain |
|---|---|
| LTDC LxCR _ | HCLK |
| LTDC LxCFBAR _ |  |
| LTDC LxCFBLR _ |  |
| LTDC LxCFBLNR _ |  |
| LTDC SRCR _ | PCLK2 |
| LTDC IER _ |  |
| LTDC ISR _ |  |
| LTDC ICR _ |  |
| LTDC SSCR _ | Pixel Clock (LCD CLK) _ |
| LTDC BPCR _ |  |
| LTDC AWCR _ |  |
| LTDC TWCR _ |  |
| LTDC GCR _ |  |
| LTDC BCCR _ |  |
| LTDC LIPCR _ |  |
| LTDC CPSR _ |  |
| LTDC CDSR _ |  |
| LTDC LxWHPCR _ |  |
| LTDC LxWVPCR _ |  |
| LTDC LxCKCR _ |  |
| LTDC LxPFCR _ |  |
| LTDC LxCACR _ |  |
| LTDC LxDCCR _ |  |
| LTDC LxBFCR _ |  |
| LTDC LxCLUTWR _ |  |


Care must be taken when accessing the LTDC registers since the APB2 bus is stalling when the following operations are ongoing:

• Register write access and update for 6 xPCKL2 period + 5x LCD_CLK period (5x

HCLK period for register on AHB clock domain)

• Register read access for 7xPCKL2 period + 5x LCD_CLK period (5x HCLK period for

register on AHB clock domain).

For registers on PCLK2 clock domain, APB2 bus is stalling during the register write access for 6 xPCKL2 period and 7xPCKL2 period for read access.

The LCD controller can be reset by setting the corresponding bit in the RCC_APB2RSTR register. It resets the three clock domains.



--- Page 93 ---

**RM0090 LCD-TFT controller (LTDC)**

**16.3.3 LCD-TFT pins and signal interface**

The Table below summarizes the LTDC signal interface:

**Table 89. LCD-TFT pins and signal interface**

| LCD-TFT signals | I/O | Description |
|---|---|---|
| LCD CLK _ | O | Clock Output |
| LCD HSYNC _ | O | Horizontal Synchronization |
| LCD VSYNC _ | O | Vertical Synchronization |
| LCD DE _ | O | Not Data Enable |
| LCD R[7:0] _ | O | Data: 8-bit Red data |
| LCD G[7:0] _ | O | Data: 8-bit Green data |
| LCD B[7:0] _ | O | Data: 8-bit Blue data |


The LCD-TFT controller pins must be configured by the user application. The unused pins can be used for other purposes.

For LTDC outputs up to 24-bit (RGB888), if less than 8bpp are used to output for example RGB565 or RGB666 to interface on 16b-bit or 18-bit displays, the RGB display data lines must be connected to the MSB of the LCD-TFT controller RGB data lines. As an example, in the case of an LCD-TFT controller interfacing with a RGB565 16-bit display, the LCD display R[4:0], G[5:0] and B[4:0] data lines pins must be connected to LCD-TFT controller LCD_R[7:3], LCD_G[7:2] and LCD_B[7:3].


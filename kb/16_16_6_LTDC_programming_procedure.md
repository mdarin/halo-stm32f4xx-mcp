**16.6 LTDC programming procedure**

• Enable the LTDC clock in the RCC register

• Configure the required Pixel clock following the panel datasheet

• Configure the Synchronous timings: VSYNC, HSYNC, Vertical and Horizontal back

porch, active data area and the front porch timings following the panel datasheet as described in the Section 16.4.1: LTDC Global configuration parameters

• Configure the synchronous signals and clock polarity in the LTDC_GCR register

• If needed, configure the background color in the LTDC_BCCR register

• Configure the needed interrupts in the LTDC_IER and LTDC_LIPCR register

• Configure the Layer1/2 parameters by programming:

– The Layer window horizontal and vertical position in the LTDC_LxWHPCR and

**LTDC_WVPCR registers. The layer window must be in the active data area.**

– The pixel input format in the LTDC_LxPFCR register

– The color frame buffer start address in the LTDC_LxCFBAR register

– The line length and pitch of the color frame buffer in the LTDC_LxCFBLR register

– The number of lines of the color frame buffer in the LTDC_LxCFBLNR register

– if needed, load the CLUT with the RGB values and its address in the

**LTDC_LxCLUTWR register**

– If needed, configure the default color and the blending factors respectively in the

**LTDC_LxDCCR and LTDC_LxBFCR registers**

• Enable Layer1/2 and if needed the CLUT in the LTDC_LxCR register

• If needed, dithering and color keying can be enabled respectively in the LTDC_GCR

**and LTDC_LxCKCR registers. It can be also enabled on the fly.**

• Reload the shadow registers to active register through the LTDC_SRCR register.

• Enable the LCD-TFT controller in the LTDC_GCR register.

• All layer parameters can be modified on the fly except the CLUT. The new configuration

has to be either reloaded immediately or during vertical blanking period by configuring the LTDC_SRCR register.

Note: All layer’s registers are shadowed. Once a register is written, it should not be modified again before the reload has been done. Thus, a new write to the same register will override the previous configuration if not yet reloaded.



--- Page 103 ---

**RM0090**

**LCD-TFT controller (LTDC)**


> [RM0090 ch.16 §16.7] LTDC registers**

### 16.7.5 LTDC Global Control Register (LTDC_GCR)**

**16.7.5 LTDC Global Control Register (LTDC_GCR)**

This register defines the global configuration of the LCD-TFT controller. Address offset: 0x18

Reset value: 0x0000 2220

31 30 29 28 27 26 25 24

23 22 21 20 19 18 17 16

| Отрег | улиру | йте в | озду | шный зазор тормоза ый зазор ”Z” будет больше из-за износа. Убедитесь, что тормо |  |
|---|---|---|---|---|---|
| Номи | нальн | ый воз | душн | з |  |


15 14 13 12 11 10 9 8

7 6 5 4 3 2 1 0

| получ возду повто | ает достаточный шный зазор дости | тормо гнет а фр | зной момент, от наибольшего зна | регул чени зной | ируйте воздушны я. Воздушный заз | й зазор до того, к ор можно регулир нет допустимой | ак овать |  |  |
|---|---|---|---|---|---|---|---|---|---|
| рно, к | огда т | олщин | икцио | нной | тормо | плас | тины д | остиг |  |




514

--- Page 106 ---

**LCD-TFT controller (LTDC) RM0090**

Bit 31 HSPOL : Horizontal Synchronization Polarity

This bit is set and cleared by software.

0: Horizontal Synchronization polarity is active low

1: Horizontal Synchronization polarity is active high

Bit 30 VSPOL : Vertical Synchronization Polarity

This bit is set and cleared by software.

0: Vertical Synchronization is active low

1: Vertical Synchronization is active high

Bit 29 DEPOL : Data Enable Polarity

This bit is set and cleared by software.

0: Data Enable polarity is active low

1: Data Enable polarity is active high

Bit 28 PCPOL : Pixel Clock Polarity

This bit is set and cleared by software.

0: input pixel clock

1: inverted input pixel clock

Bits 27:17 Reserved, must be kept at reset value

Bit 16 DEN : Dither Enable

This bit is set and cleared by software.

0: Dither disable

1: Dither enable

Bit 15 Reserved, must be kept at reset value

Bits 14:12 DRW[2:0] : Dither Red Width

These bits return the Dither Red Bits

Bit 11 Reserved, must be kept at reset value

Bits 10:8 DGW[2:0] : Dither Green Width

These bits return the Dither Green Bits

Bit 7 Reserved, must be kept at reset value

Bits 6:4 DBW[2:0] : Dither Blue Width

These bits return the Dither Blue Bits

Bits 3:1 Reserved, must be kept at reset value

Bit 0 LTDCEN : LCD-TFT controller enable bit

This bit is set and cleared by software.

0: LTDC disable

1: LTDC enable

496/1749 RM0090 Rev 18

--- Page 107 ---

**RM0090**

**LCD-TFT controller (LTDC)**


**2.11 SDIO peripheral limitations**

**2.11.1 SDIO HW flow control**

**Description**

When enabling the HW flow control by setting bit 14 of the SDIO_CLKCR register to ‘1’, glitches can occur on the SDIOCLK output clock resulting in wrong data to be written into the SD/MMC card or into the SDIO device. As a consequence, a CRC error will be reported to the SD/SDIO MMC host interface (DCRCFAIL bit set to ‘1’ in SDIO_STA register).

**Workaround**

None.

Note: Do not use the HW flow control. Overrun errors (Rx mode) and FIFO underrun (Tx mode) should be managed by the application software.

**2.11.2 Wrong CCRCFAIL status after a response without CRC is received**

**Description**

The CRC is calculated even if the response to a command does not contain any CRC field. As a consequence, after the SDIO command IO_SEND_OP_COND (CMD5) is sent, the CCRCFAIL bit of the SDIO_STA register is set.

**Workaround**

The CCRCFAIL bit in the SDIO_STA register shall be ignored by the software. CCRCFAIL must be cleared by setting CCRCFAILC bit of the SDIO_ICR register after reception of the response to the CMD5 command.

**2.11.3 SDIO clock divider BYPASS mode may not work properly**

**Description**

In high speed communication mode, when SDIO_CK is equal to 48 MHz (PLL48_output = 48 MHz), the BYPASS bit is equal to ‘1’ and the NEGEDGE bit is equal to ‘0’ (respectively bit 10 and bit 13 in the SDIO_CLKCR register), the hold timing at the I/O pin is not aligned with the SD/MMC 2.0 specifications.

**Workaround**

When not using USB nor RNG, PLL48_output (SDIOCLK) frequency can be raised up to 75 MHz, allowing to reach 37.5 MHz on SDIO_CK in high speed mode. The BYPASS bit, the CLKDIV bit and the NEGEDGE bit are equal to ‘0’.



--- Page 31 ---

**STM32F40x and STM32F41x STM32F40x and STM32F41x silicon limitations**

**2.11.4 Data corruption in SDIO clock dephasing (NEGEDGE) mode**

**Description**

When NEGEDGE bit is set to ‘1’, it may lead to invalid data and command response read.

**Workaround**

None. A configuration with the NEGEDGE bit equal to ‘1’ should not be used.

**2.11.5 CE-ATA multiple write command and card busy signal management**

**Description**

The CE-ATA card may inform the host that it is busy by driving the SDIO_D0 line low, two cycles after the transfer of a write command (RW_MULTIPLE_REGISTER or RW_MULTIPLE_BLOCK). When the card is in a busy state, the host must not send any data until the BUSY signal is de-asserted (SDIO_D0 released by the card).

This condition is not respected if the data state machine leaves the IDLE state (Write operation programmed and started, DTEN = 1, DTDIR = 0 in SDIO_DCTRL register and TXFIFOE = 0 in SDIO_STA register).

As a consequence, the write transfer fails and the data lines are corrupted.

**Workaround**

After sending the write command (RW_MULTIPLE_REGISTER or

RW_MULTIPLE_BLOCK), the application must check that the card is not busy by polling the BSY bit of the ATA status register using the FAST_IO (CMD39) command before enabling the data state machine.

**2.11.6 No underrun detection with wrong data transmission**

**Description**

In case there is an ongoing data transfer from the SDIO host to the SD card and the hardware flow control is disabled (bit 14 of the SDIO_CLKCR is not set), if an underrun condition occurs, the controller may transmit a corrupted data block (with wrong data word) without detecting the underrun condition when the clock frequencies have the following relationship:

[3 x period(PCLK2) + 3 x period(SDIOCLK)] >= (32 / (BusWidth)) x period(SDIO_CK)

**Workaround**

Avoid the above-mentioned clock frequency relationship, by:

• Incrementing the APB frequency

• or decreasing the transfer bandwidth

• or reducing SDIO_CK frequency



37

--- Page 32 ---

**STM32F40x and STM32F41x silicon limitations STM32F40x and STM32F41x**

**2.12 ADC peripheral limitations**

**2.12.1 ADC sequencer modification during conversion**

**Description**

When a software start of conversion is used as an ADC trigger, and if the ADC_SQRx or ADC_JSQRx registers are modified during the conversion, the current conversion is reset and the ADC does not automatically restart the new conversion sequence. The hardware start of conversion trigger is not impacted and the ADC automatically restarts the new sequence when the next hardware trigger occurs.

**Workaround**

When a software start of conversion is used, the user application must first set the SWSART bit in the ADC_CR2 register, and then restart the new conversion sequence.



--- Page 33 ---

**STM32F40x and STM32F41x STM32F40x and STM32F41x silicon limitations**

**2.13 DAC peripheral limitations**

**2.13.1 DMA underrun flag management**

**Description**

If the DMA is not fast enough to input the next digital data to the DAC, as a consequence, the same digital data is converted twice. In these conditions, the DMAUDR flag is set, which usually leads to disable the DMA data transfers. This is not the case: the DMA is not disabled by DMAUDR=1, and it keeps servicing the DAC.

**Workaround**

To disable the DAC DMA stream, reset the EN bit (corresponding to the DAC DMA stream) in the DMA_SxCR register.

**2.13.2 DMA request not automatically cleared by DMAEN=0**

**Description**

if the application wants to stop the current DMA-to-DAC transfer, the DMA request is not automatically cleared by DMAEN=0, or by DACEN=0.

If the application stops the DAC operation while the DMA request is high, the DMA request will be pending while the DAC is reinitialized and restarted; with the risk that a spurious unwanted DMA request is serviced as soon as the DAC is re-enabled.

**Workaround**

To stop the current DMA-to-DAC transfer and restart, the following sequence should be applied:

1. Check if DMAUDR is set.

2. Clear the DAC/DMAEN bit.

3. Clear the EN bit of the DAC DMA/Stream

4. Reconfigure by software the DAC, DMA, triggers etc.

5. Restart the application.


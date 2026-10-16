## 2 STM32F40x and STM32F41x silicon limitations

Table 4 gives quick references to all documented limitations.

Legend for Table 4 : A = workaround available; N = no workaround available; P = partial workaround available, ‘-’ and grayed = fixed.

**Table 4. Summary of silicon limitations**

| Links to silicon limitations | Revision A | Revision ‘Z’, ‘1’, ‘2’ and ‘Y’ |  |
|---|---|---|---|
| Section 2.1: System limitations | Section 2.1.1: ART Accelerator prefetch queue instruction is not supported | N | - |
| Section 2.1.2: MCU device ID is incorrect | A | - |  |
| Section 2.1.3: Debugging Sleep/Stop mode with WFE/WFI entry | A |  |  |
| Section 2.1.4: Debugging Stop mode and system tick timer | A |  |  |
| Section 2.1.5: Wakeup sequence from Standby mode when using more than one wakeup source | A |  |  |
| Section 2.1.6: Full JTAG configuration without NJTRST pin cannot be used | A |  |  |
| Section 2.1.7: PDR ON pin not available on LQFP100 _ package for revision Z devices | - | N |  |
| Section 2.1.8: Incorrect BOR option byte when consecutively programming BOR option byte | A |  |  |
| Section 2.1.9: Configuration of PH10 and PI10 as external interrupts is erroneous | N |  |  |
| Section 2.1.10: DMA2 data corruption when managing AHB and APB peripherals in a concurrent way | A |  |  |
| Section 2.1.11: Slowing down APB clock during a DMA transfer | A |  |  |
| Section 2.1.12: MPU attribute to RTC and IWDG registers could be managed incorrectly | A |  |  |
| Section 2.1.13: Delay after an RCC peripheral clock enabling | A |  |  |
| Section 2.1.14: Battery charge monitoring lower than 2.4 Volts | P |  |  |
| Section 2.1.15: Internal noise impacting the ADC accuracy | A |  |  |
| Section 2.2: IWDG peripheral limitations | Section 2.2.1: RVU and PVU flags are not reset in STOP mode | A |  |
| Section 2.3: RTC Tamper _ limitations | Section 2.3.1: Spurious tamper detection when disabling the tamper channel | N |  |
| Section 2.3.2: Detection of a tamper event occuring before enabling the tamper detection is not supported in edge detection mode | A |  |  |


8/38 DocID022183 Rev 7

--- Page 9 ---

**STM32F40x and STM32F41x STM32F40x and STM32F41x silicon limitations**

**Table 4. Summary of silicon limitations (continued)**

| Links to silicon limitations | Revision A | Revision ‘Z’, ‘1’, ‘2’ and ‘Y’ |
|---|---|---|
| Section 2.4: I2C peripheral limitations | Section 2.4.1: SMBus standard not fully supported | A |
| Section 2.4.2: Start cannot be generated after a misplaced Stop | A |  |
| Section 2.4.3: Mismatch on the “Setup time for a repeated Start condition” timing parameter | A |  |
| Section 2.4.4: Data valid time (tVD;DAT) violated without the OVR flag being set | A |  |
| Section 2.4.5: Both SDA and SCL maximum rise time (tr) violated when VDD I2C bus higher than ((VDD+0.3) / 0.7) V _ | A |  |
| Section 2.4.6: Spurious Bus Error detection in master mode | A |  |
| Section 2.4.7: Wrong behavior related with MCU Stop mode when wakeup from Stop mode by I2C peripheral is disabled | A |  |
| Section 2.5: I2S peripheral limitations | Section 2.5.1: In I2S slave mode, WS level must be set by the external master when enabling the I2S | A |
| Section 2.5.2: I2S2 in full-duplex mode may not work properly when SCK and WS signals are mapped on PI1 and PI0 respectively | A |  |
| Section 2.6: USART peripheral limitations | Section 2.6.1: Idle frame is not detected if receiver clock speed is deviated | N |
| Section 2.6.2: In full duplex mode, the Parity Error (PE) flag can be cleared by writing to the data register | A |  |
| Section 2.6.3: Parity Error (PE) flag is not set when receiving in Mute mode using address mark detection | N |  |
| Section 2.6.4: Break frame is transmitted regardless of nCTS input line status | N |  |
| Section 2.6.5: nRTS signal abnormally driven low after a protocol violation | A |  |
| Section 2.6.6: Start bit detected too soon when sampling for NACK signal from the smartcard | N |  |
| Section 2.6.7: Break request can prevent the Transmission Complete flag (TC) from being set | A |  |
| Section 2.6.8: Guard time is not respected when data are sent on TXE events | A |  |
| Section 2.6.9: nRTS is active while RE or UE = 0 | A |  |
| Section 2.7: bxCAN limitations | Section 2.7.1: bxCAN time triggered communication mode not supported | N |
| Section 2.8: OTG FS _ peripheral limitations | Section 2.8.1: Data in RxFIFO is overwritten when all channels are disabled simultaneously | A |
| Section 2.8.2: OTG host blocks the receive channel when receiving IN packets and no TxFIFO is configured | A |  |
| Section 2.8.3: Host channel-halted interrupt not generated when the channel is disabled | A |  |
| Section 2.8.4: Error in software-read OTG FS DCFG _ _ register values | A |  |




37

--- Page 10 ---

**STM32F40x and STM32F41x silicon limitations STM32F40x and STM32F41x**

**Table 4. Summary of silicon limitations (continued)**

| Links to silicon limitations | Revision A | Revision ‘Z’, ‘1’, ‘2’ and ‘Y’ |
|---|---|---|
| Section 2.9: Ethernet peripheral limitations | Section 2.9.1: Incorrect layer 3 (L3) checksum is inserted in transmitted IPv6 packets without TCP, UDP or ICMP payloads | A |
| Section 2.9.2: The Ethernet MAC processes invalid extension headers in the received IPv6 frames | N |  |
| Section 2.9.3: MAC stuck in the Idle state on receiving the TxFIFO flush command exactly 1 clock cycle after a transmission completes | A |  |
| Section 2.9.4: Transmit frame data corruption | A |  |
| Section 2.9.5: Successive write operations to the same register might not be fully taken into account | A |  |
| Section 2.10: FSMC peripheral limitations | Section 2.10.1: Dummy read cycles inserted when reading synchronous memories | N |
| Section 2.10.2: FSMC synchronous mode and NWAIT signal disabled | A |  |
| Section 2.10.3: FSMC NOR Flash/PSRAM controller asynchronous access on bank 2 to 4 when bank 1 is in synchronous mode (CBURSTRW bit is set) | A |  |
| Section 2.11: SDIO peripheral limitations | Section 2.11.1: SDIO HW flow control | N |
| Section 2.11.2: Wrong CCRCFAIL status after a response without CRC is received | A |  |
| Section 2.11.3: SDIO clock divider BYPASS mode may not work properly | A |  |
| Section 2.11.4: Data corruption in SDIO clock dephasing (NEGEDGE) mode | N |  |
| Section 2.11.5: CE-ATA multiple write command and card busy signal management | A |  |
| Section 2.11.6: No underrun detection with wrong data transmission | A |  |
| Section 2.12: ADC peripheral limitations | Section 2.12.1: ADC sequencer modification during conversion | A |
| Section 2.13: DAC peripheral limitations | Section 2.13.1: DMA underrun flag management | A |
| Section 2.13.2: DMA request not automatically cleared by DMAEN=0 | A |  |
| Section 2.14: SPI peripheral limitations | Section 2.14.1: Wrong CRC calculation when the polynomial is even. | A |
| Section 2.14.2: BSY bit may stay high at the end of a data transfer in slave mode | A |  |
| Section 2.14.3: LSB bit of data or CRC calculation can be corrupted for the received data in master mode depending on the timing of the feedback communication clock respect to the APB clock (SPI or I2S) | A |  |




--- Page 11 ---

**STM32F40x and STM32F41x STM32F40x and STM32F41x silicon limitations**

**2.1 System limitations**

**2.1.1 ART Accelerator prefetch queue instruction is not supported**

**Description**

The ART Accelerator prefetch queue instruction is not supported on revision A devices. This limitation does not prevent the ART Accelerator from using the cache enable/disable capability and the selection of the number of wait states according to the system frequency.

**Workaround**

• Revision A devices: none

• Revision Z and 1 devices: fixed.

**2.1.2 MCU device ID is incorrect**

**Description**

On revision A devices, the STM32F40x and STM32F41x have the same MCU device ID as the STM32F20x and STM32F21x devices. On revision A devices, when reading the Revision identifier, this will return 0x2000 instead of 0x1000. The device ID and revision ID can be read from address 0xE0042000.

**Workaround**

• Revision A devices

To differentiate the STM32F4xxx from the STM32F2xxx series, read the MCU device ID and the Core Device.

– For STM32F2xxx

MCU device ID = STM32F2xxx device ID

Core Device = Cortex ® -M3

– For STM32F4xxx

MCU device ID = STM32F4xxx device ID

Core Device = Cortex ® -M4

• Revision Z and 1 devices: fixed.

**2.1.3 Debugging Sleep/Stop mode with WFE/WFI entry**

**Description**

When the Sleep debug or Stop debug mode is enabled (DBG_SLEEP bit or DBG_STOP bit are set in the DBGMCU_CR register), this allows software debugging during Sleep or Stop mode. After wakeup some unreachable instructions could be executed if the following condition are met:

• If the application software disables the Prefetch queue

• The number of wait state configured on Flash interface is higher than 0

• And Linker place WFE or WFI instructions on 4-bytes aligned addresses

(0x080xx_xxx4)



37

--- Page 12 ---

**STM32F40x and STM32F41x silicon limitations STM32F40x and STM32F41x**

**Workaround**

• Add three NOPs after WFI/WFE instruction

• Keep one AHB master active during sleep (example keep DMA1 or DMA2 RCC clock

enable bit set)

• Execute WFI/WFE instruction from routines inside the SRAM

**2.1.4 Debugging Stop mode and system tick timer**

**Description**

If the system tick timer interrupt is enabled during the Stop mode debug (DBG_STOP bit set in the DBGMCU_CR register), it will wake up the system from Stop mode.

**Workaround**

To debug the Stop mode, disable the system tick timer interrupt.

**2.1.5 Wakeup sequence from Standby mode when using more than one wakeup source**

**Description**

The various wakeup sources are logically OR-ed in front of the rising-edge detector which generates the wakeup flag (WUF). The WUF needs to be cleared prior to Standby mode entry, otherwise the MCU wakes up immediately.

If one of the configured wakeup sources is kept high during the clearing of the WUF (by setting the CWUF bit), it may mask further wakeup events on the input of the edge detector. As a consequence, the MCU might not be able to wake up from Standby mode.

**Workaround**

To avoid this problem, the following sequence should be applied before entering Standby mode:

• Disable all used wakeup sources,

• Clear all related wakeup flags,

• Re-enable all used wakeup sources,

• Enter Standby mode

Note: Be aware that, when applying this workaround, if one of the wakeup sources is still kept high, the MCU enters Standby mode but then it wakes up immediately generating a power reset.

**2.1.6 Full JTAG configuration without NJTRST pin cannot be used**

**Description**

When using the JTAG debug port in debug mode, the connection with the debugger is lost if the NJTRST pin (PB4) is used as a GPIO. Only the 4-wire JTAG port configuration is impacted.



--- Page 13 ---

**STM32F40x and STM32F41x STM32F40x and STM32F41x silicon limitations**

**Workaround**

Use the SWD debug port instead of the full 4-wire JTAG port.

**2.1.7 PDR_ON pin not available on LQFP100 package**

**for revision Z devices**

**Description**

On revision Z devices, the PDR_ON pin (pin 99) available on LQFP100 package is replaced by V SS . As a consequence, the POR/PDR feature is always enabled.

**Workaround**

• Applications using on revision A devices with PDR_ON pin connected to V DD

(POR/PDR feature enabled)

Connect the former PDR_ON pin to V SS on revision Z devices.

• Applications using revision A devices with PDR_ON pin connected to V SS (POR/PDR

feature disabled)

No modification is required when migrating to revision Z devices. However, it is no longer possible to supply the product from a 1.7 V V DD on LQFP100 package since V DD minimum value is 1.8 V when the POR/PDR feature is enabled.

**2.1.8 Incorrect BOR option byte when consecutively programming BOR option byte**

**Description**

When the AHB prescaler is greater than 2, and consecutive BOR option byte program operations are performed without resetting the device, then an incorrect value might be programmed in the BOR option byte.

**Workaround**

To program consecutive BOR option byte values, either configure the AHB prescaler to 1 or 2, or perform a system reset between each BOR option byte program operation.

**2.1.9 Configuration of PH10 and PI10 as external interrupts is erroneous**

**Description**

PH10 or PI10 is selected as the source for the EXTI10 external interrupt by setting bits EXTI10[3:0] of SYSCFG_EXTICR3 register to 0x0111 or 0x1000, respectively. However, this erroneous operation enables PH2 and PI2 as external interrupt inputs.

As a result, it is not possible to use PH10/PI10 as interrupt sources if PH2/PI2 are not selected as the interrupt source, as well. This means that bits EXTI10[3:0] of SYSCFG_EXTICR3 register and bits EXTI2[3:0] of SYSCFG_EXTICR1 should be programmed to the same value:

• 0x0111 to select PH10/PH2

• 0x1000 to select PI10/PI2



37

--- Page 14 ---

**STM32F40x and STM32F41x silicon limitations STM32F40x and STM32F41x**

**Workaround**

None.

**2.1.10 DMA2 data corruption when managing AHB and APB peripherals in a concurrent way**

**Description**

When the DMA2 is managing AHB Peripherals (read- or write-sensitive devices such as peripherals embedding FIFOs or GPIOs) and also APB transfers in a concurrent way, this generates a data corruption (multiple DMA access). When this condition occurs:

• The data transferred by the DMA to the AHB peripherals could be corrupted in case of

a FIFO target.

• For memories, it will result in multiple access (not visible by the Software) and the data

is not corrupted.

• For the DCMI, a multiple unacknowledged request could be generated, which implies

an unknown behavior of the DMA.

AHB peripherals embedding FIFO are DCMI, CRYPTO, and HASH. Also we can consider external FIFO controlled by the FSMC and GPIO Output Data register as AHB write sensitive peripherals.

On sales types without CRYPTO, mainly impacted peripheral is the DCMI peripheral which embeds a FIFO. External FIFO controlled by the FSMC and GPIOs when used as parallel output are also impacted.

**Workaround**

Avoid concurrent AHB (DCMI, CRYPTO, HASH, FSMC with external FIFO, or GPIOs output data register) and APB transfer management using the DMA2.

**2.1.11 Slowing down APB clock during a DMA transfer**

**Description**

When the CPU modifies the APB clock (slows down the clock: changes AHB/APB prescaler from 1 to 2, 1 to 4, 1 to 8 or 1 to 16) while the DMA is performing a write access to the same APB peripherals, the current DMA transfer will be blocked. Only system reset will recover.

**Workaround**

Before slowing down the APB clock, wait until the end of the DMA transfer on this APB.

**2.1.12 MPU attribute to RTC and IWDG registers could be managed incorrectly**

**Description**

If the MPU is used and the non bufferable attribute is set to the RTC or IWDG memory map region, the CPU access to the RTC or IWDG registers could be treated as bufferable, provided that there is no APB prescaler configured (AHB/APB prescaler is equal to 1).



--- Page 15 ---

**STM32F40x and STM32F41x STM32F40x and STM32F41x silicon limitations**

**Workaround**

If the non bufferable attribute is required for these registers, the software could perform a read after the write to guaranty the completion of the write access.

**2.1.13 Delay after an RCC peripheral clock enabling**

**Description**

A delay between an RCC peripheral clock enable and the effective peripheral enabling should be taken into account in order to manage the peripheral read/write to registers. This delay depends on the peripheral’s mapping:

• If the peripheral is mapped on AHB: the delay should be equal to 2 AHB cycles.

• If the peripheral is mapped on APB: the delay should be equal to 1 + (AHB/APB

prescaler) cycles.

**Workarounds**

1. Use the DSB instruction to stall the Cortex ® -M4 CPU pipeline until the instruction is

completed.

2. Insert “n” NOPs between the RCC enable bit write and the peripheral register writes

(n = 2 for AHB peripherals, n = 1 + AHB/APB prescaler in case of APB peripherals).

**2.1.14 Battery charge monitoring lower than 2.4 Volts**

**Description**

If (V DD = V DDA ) is lower than or equal to 2.4 V, the V BAT conversion correctness is not guaranteed in full temperature and voltage ranges. When V BAT is set, the voltage divider bridge is enabled and V BAT /2 is connected to the ADC input. In order to monitor the battery charge correctly, the input of the ADC must not be higher than (V DDA - 0.6 V).

Thus, V BAT /2 < V DD – 0.6 V implies that V DD > 2.4 V.

**Workaround**

None. (V DD = V DDA ) should be greater than 2.4 V.

**2.1.15 Internal noise impacting the ADC accuracy**

**Description**

An internal noise generated on V DD supplies and propagated internally may impact the ADC accuracy.

This noise is always active whatever the power mode of the MCU (RUN or Sleep).

**Workarounds**

Two steps could be followed to adapt the accuracy level to the application requirements:

1. Configure the Flash ART as Prefetch OFF and (Data + Instruction) cache ON.

2. Use averaging and filtering algorithms on ADC output codes.

For more workaround details of this limitation, refer to AN4073.



37

--- Page 16 ---

**STM32F40x and STM32F41x silicon limitations STM32F40x and STM32F41x**

**2.2 IWDG peripheral limitations**

**2.2.1 RVU and PVU flags are not reset in STOP mode**

**Description**

The RVU and PVU flags of the IWDG_SR register are set by hardware after a write access to the IWDG_RLR and the IWDG_PR registers, respectively. If the Stop mode is entered immediately after the write access, the RVU and PVU flags are not reset by hardware. Before performing a second write operation to the IWDG_RLR or the IWDG_PR register, the application software must wait for the RVU or PVU flag to be reset. However, since the RVU/PVU bit is not reset after exiting the Stop mode, the software goes into an infinite loop and the independent watchdog (IWDG) generates a reset after the programmed timeout period.

**Workaround**

Wait until the RVU or PVU flag of the IWDG_SR register is reset before entering the Stop mode.

**2.3 RTC_Tamper limitations**

**2.3.1 Spurious tamper detection when disabling the tamper channel**

**Description**

If the tamper detection is configured for detection on falling edge event (TAMPFLT=00 and TAMPxTRG=1) and if the tamper event detection is disabled when the tamper pin is at high level, a false tamper event is detected.

**Workaround**

None

**2.3.2 Detection of a tamper event occuring before enabling the tamper detection is not supported in edge detection mode**

**Description**

When the tamper detection is enabled in edge detection mode (TAMPFLT=00):

• When TAMPxTRG=0 (rising edge detection): if the tamper input is already high before

enabling the tamper detection, the tamper event may or may not be detected when enabling the tamper detection. The probability to detect it increases with the APB frequency.

• When TAMPxTRG=1 (falling edge detection): if the tamper input is already low before

enabling the tamper detection, the tamper event is not detected when enabling the tamper detection.



--- Page 17 ---

**STM32F40x and STM32F41x STM32F40x and STM32F41x silicon limitations**

**Workaround**

The I/O state should be checked by software in the GPIO registers, just after enabling the tamper detection and before writing sensitive values in the backup registers, in order to ensure that no active edge occurred before enabling the tamper event detection.


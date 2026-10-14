# RM0090 semantic index — справочник RAG для STM32F407

> Файлы-разделы лежат рядом; `manifest.json` — полные метаданные, `index.json` — машинный индекс.

## Быстрый поиск задачи → глава

- **Настроить/использовать GPIO (режимы, AF, скорость)** → главы 08 (General-purpose I/Os (GPIO))
- **Тактирование, RCC, PLL, prescalers, включение периферии** → главы 07 (Reset and clock control (RCC) — F405/07/15/17)
- **Сбросы, причины reset, BOR** → главы 07 (Reset and clock control (RCC) — F405/07/15/17)
- **Питание, режимы low-power (Sleep/Stop/Standby), PVD** → главы 05 (Power controller (PWR))
- **Прошивка/стирание Flash, protection, option bytes** → главы 03 (Embedded Flash memory interface)
- **DMA: потоки, каналы, FIFO, приоритеты** → главы 10 (DMA controller)
- **Прерывания, EXTI, NVIC, вектора** → главы 12 (EXTI / interrupts and events / NVIC vectors)
- **ADC: режимы, каналы, regular/injected, sampling** → главы 13 (Analog-to-digital converter (ADC))
- **DAC** → главы 14 (Digital-to-analog converter (DAC))
- **TIM1/TIM8: advanced PWM, dead-time, complementary,brake** → главы 17 (Advanced timers TIM1/TIM8)
- **Обычные таймеры PWM/encoder/IC (TIM2–TIM5)** → главы 18 (General-purpose timers TIM2–TIM5)
- **TIM9–TIM14** → главы 19 (General-purpose timers TIM9–TIM14)
- **Time-base, периодические прерывания (TIM6/TIM7, DAC trigger)** → главы 20 (Basic timers TIM6/TIM7)
- **Watchdog IWDG** → главы 21 (Independent watchdog (IWDG))
- **Watchdog WWDG** → главы 22 (Window watchdog (WWDG))
- **RTC, календарь, alarm, wakeup, tamper** → главы 26 (Real-time clock (RTC))
- **I2C** → главы 27 (Inter-integrated circuit (I2C))
- **SPI** → главы 28 (Serial peripheral interface (SPI))
- **USART/UART** → главы 30 (Universal synchronous/asynchronous receiver (USART))
- **CRC** → главы 04 (CRC calculation unit)
- **RNG (random)** → главы 24 (Random number generator (RNG))
- **Crypto AES/DES (CRYP)** → главы 23 (CRYP crypto processor)
- **HASH** → главы 25 (HASH processor)
- **bxCAN (CAN2.0B)** → главы 32 (bxCAN)
- **USB device (FS)** → главы 34 (USB OTG full-speed (OTG_FS))
- **USB host/HS, ULPI, PHY** → главы 35 (USB OTG high-speed (OTG_HS))
- **SD card (SDIO)** → главы 31 (SDIO)
- **Ethernet MAC, PHY, PTP** → главы 33 (Ethernet MAC (with DMA))
- **Внешняя память FSMC (SRAM/NAND/NOR/LCD)** → главы 36 (Flexible static memory controller (FSMC))
- **UID, размер flash, electronic signature** → главы 39 (Device electronic signature (UID, flash size))
- **Отладка, DBGMCU** → главы 38 (Debug support (DBG))
- **Карта памяти, bus matrix, bit-banding** → главы 02 (Memory and bus architecture)

## Внимание: главы НЕ для F407

Главы **6 (RCC F42x), 11 (DMA2D), 15 (DCMI), 16 (LTDC), 29 (SAI), 37 (FMC)** описывают периферию STM32F42x/43x.

## Карта глав → файлы

### 01 Documentation conventions
- `01_Documentation_conventions.md` §1 — Documentation conventions
### 02 Memory and bus architecture
- `02_Memory_and_bus_architecture.md` §2 — Memory and bus architecture
### 03 Embedded Flash memory interface
- `03_3_2_Main_features.md` §3.2 — Main features**
- `03_3_3_Embedded_Flash_memory_in_STM32F405xx07xx_and.md` §3.3 — Embedded Flash memory in STM32F405xx/07xx and STM32F415xx/17xx**
- `03_3_4_Embedded_Flash_memory_in_STM32F42xxx_and_STM3.md` §3.4 — Embedded Flash memory in STM32F42xxx and STM32F43xxx**
- `03_3_5_Read_interface.md` §3.5 — Read interface**
- `03_3_6_Erase_and_program_operations.md` §3.6 — Erase and program operations**
- `03_3_7_Option_bytes.md` §3.7 — Option bytes**
- `03_3_9_Flash_interface_registers.md` §3.9 — Flash interface registers**
### 04 CRC calculation unit
- `04_CRC_calculation_unit.md` §4 — CRC calculation unit
### 05 Power controller (PWR)
- `05_5_1_Power_supplies.md` §5.1 — Power supplies**
- `05_5_2_Power_supply_supervisor.md` §5.2 — Power supply supervisor**
- `05_5_3_Lowpower_modes__c01.md` §5.3 — Low-power modes** — 5.3 Low-power modes**
- `05_5_3_Lowpower_modes__c02.md` §5.3 — Low-power modes** — 5.3.3 Sleep mode**
- `05_5_3_Lowpower_modes__c03.md` §5.3 — Low-power modes** — 5.3.4 Stop mode (STM32F405xx/07xx and STM32F415xx/17xx)**
- `05_5_3_Lowpower_modes__c04.md` §5.3 — Low-power modes** — 5.3.5 Stop mode (STM32F42xxx and STM32F43xxx)**
- `05_5_3_Lowpower_modes__c05.md` §5.3 — Low-power modes** — 5.3.6 Standby mode**
- `05_5_3_Lowpower_modes__c06.md` §5.3 — Low-power modes** — 5.3.7 Programming the RTC alternate functions to wake up the device from the Stop and Standby modes**
- `05_5_4_Power_control_registers_STM32F405xx07xx_and.md` §5.4 — Power control registers (STM32F405xx/07xx and**
- `05_5_5_Power_control_registers_STM32F42xxx_and_STM32.md` §5.5 — Power control registers (STM32F42xxx and STM32F43xxx)**
- `05_5_6_PWR_register_map.md` §5.6 — PWR register map**
### 06 RCC — STM32F42x/43x variant (NOT F407) *(NOT on F407)*
- `06_6_1_Reset.md` §6.1 — Reset**
- `06_6_2_Clocks.md` §6.2 — Clocks**
- `06_6_3_1_RCC_clock_control_register_RCCCR.md` §6.3.1 — RCC clock control register (RCC_CR)**
- `06_6_3_2_RCC_PLL_configuration_register_RCCPLLCFGR.md` §6.3.2 — RCC PLL configuration register (RCC_PLLCFGR)**
- `06_6_3_3_RCC_clock_configuration_register_RCCCFGR.md` §6.3.3 — RCC clock configuration register (RCC_CFGR)**
- `06_6_3_4_RCC_clock_interrupt_register_RCCCIR.md` §6.3.4 — RCC clock interrupt register (RCC_CIR)**
- `06_6_3_5_RCC_AHB1_peripheral_reset_register_RCCAHB1RST.md` §6.3.5 — RCC AHB1 peripheral reset register (RCC_AHB1RSTR)**
- `06_6_3_8_RCC_APB1_peripheral_reset_register_RCCAPB1RST.md` §6.3.8 — RCC APB1 peripheral reset register (RCC_APB1RSTR)**
- `06_6_3_9_RCC_APB2_peripheral_reset_register_RCCAPB2RST.md` §6.3.9 — RCC APB2 peripheral reset register (RCC_APB2RSTR)**
- `06_6_3_10_RCC_AHB1_peripheral_clock_register_RCCAHB1ENR.md` §6.3.10 — RCC AHB1 peripheral clock register (RCC_AHB1ENR)**
- `06_6_3_13_RCC_APB1_peripheral_clock_enable_register_RCC.md` §6.3.13 — RCC APB1 peripheral clock enable register (RCC_APB1ENR)**
- `06_6_3_14_RCC_APB2_peripheral_clock_enable_register_RCC.md` §6.3.14 — RCC APB2 peripheral clock enable register (RCC_APB2ENR)**
- `06_6_3_15_RCC_AHB1_peripheral_clock_enable_in_low_power.md` §6.3.15 — RCC AHB1 peripheral clock enable in low power mode register (RCC_AHB1LPENR)**
- `06_6_3_16_RCC_AHB2_peripheral_clock_enable_in_low_power.md` §6.3.16 — RCC AHB2 peripheral clock enable in low power mode register (RCC_AHB2LPENR)**
- `06_6_3_19_RCC_APB2_peripheral_clock_enabled_in_low_powe.md` §6.3.19 — RCC APB2 peripheral clock enabled in low power mode register (RCC_APB2LPENR)
- `06_6_3_20_RCC_Backup_domain_control_register_RCCBDCR.md` §6.3.20 — RCC Backup domain control register (RCC_BDCR)
- `06_6_3_21_RCC_clock_control_status_register_RCCCSR.md` §6.3.21 — RCC clock control & status register (RCC_CSR)
- `06_6_3_23_RCC_PLLI2S_configuration_register_RCCPLLI2SCF.md` §6.3.23 — RCC PLLI2S configuration register (RCC_PLLI2SCFGR)
- `06_6_3_24_RCC_PLL_configuration_register_RCCPLLSAICFGR.md` §6.3.24 — RCC PLL configuration register (RCC_PLLSAICFGR)**
- `06_6_3_25_RCC_Dedicated_Clock_Configuration_Register_RC.md` §6.3.25 — RCC Dedicated Clock Configuration Register (RCC_DCKCFGR)
### 07 Reset and clock control (RCC) — F405/07/15/17
- `07_7_1_Reset.md` §7.1 — Reset
- `07_7_2_Clocks.md` §7.2 — Clocks
- `07_7_3_1_RCC_clock_control_register_RCCCR.md` §7.3.1 — RCC clock control register (RCC_CR)
- `07_7_3_2_RCC_PLL_configuration_register_RCCPLLCFGR.md` §7.3.2 — RCC PLL configuration register (RCC_PLLCFGR)
- `07_7_3_3_RCC_clock_configuration_register_RCCCFGR.md` §7.3.3 — RCC clock configuration register (RCC_CFGR)
- `07_7_3_4_RCC_clock_interrupt_register_RCCCIR.md` §7.3.4 — RCC clock interrupt register (RCC_CIR)
- `07_7_3_5_RCC_AHB1_peripheral_reset_register_RCCAHB1RST.md` §7.3.5 — RCC AHB1 peripheral reset register (RCC_AHB1RSTR)
- `07_7_3_8_RCC_APB1_peripheral_reset_register_RCCAPB1RST.md` §7.3.8 — RCC APB1 peripheral reset register (RCC_APB1RSTR)
- `07_7_3_10_RCC_AHB1_peripheral_clock_enable_register_RCC.md` §7.3.10 — RCC AHB1 peripheral clock enable register (RCC_AHB1ENR)
- `07_7_3_13_RCC_APB1_peripheral_clock_enable_register_RCC.md` §7.3.13 — RCC APB1 peripheral clock enable register (RCC_APB1ENR)
- `07_7_3_14_RCC_APB2_peripheral_clock_enable_register_RCC.md` §7.3.14 — RCC APB2 peripheral clock enable register (RCC_APB2ENR)
- `07_7_3_15_RCC_AHB1_peripheral_clock_enable_in_low_power.md` §7.3.15 — RCC AHB1 peripheral clock enable in low power mode register (RCC_AHB1LPENR)
- `07_7_3_18_RCC_APB1_peripheral_clock_enable_in_low_power.md` §7.3.18 — RCC APB1 peripheral clock enable in low power mode register (RCC_APB1LPENR)
- `07_7_3_19_RCC_APB2_peripheral_clock_enabled_in_low_powe.md` §7.3.19 — RCC APB2 peripheral clock enabled in low power mode register (RCC_APB2LPENR)
- `07_7_3_20_RCC_Backup_domain_control_register_RCCBDCR.md` §7.3.20 — RCC Backup domain control register (RCC_BDCR)
- `07_7_3_21_RCC_clock_control_status_register_RCCCSR.md` §7.3.21 — RCC clock control & status register (RCC_CSR)
- `07_7_3_23_RCC_PLLI2S_configuration_register_RCCPLLI2SCF.md` §7.3.23 — RCC PLLI2S configuration register (RCC_PLLI2SCFGR)
### 08 General-purpose I/Os (GPIO)
- `08_8_3_GPIO_functional_description.md` §8.3 — GPIO functional description
- `08_8_4_GPIO_registers.md` §8.4 — GPIO registers
### 09 System configuration controller (SYSCFG)
- `09_System_configuration_controller_SYSCFG.md` §9 — System configuration controller (SYSCFG)
### 10 DMA controller
- `10_10_2_DMA_main_features.md` §10.2 — DMA main features
- `10_10_3_DMA_functional_description__c01.md` §10.3 — DMA functional description — 10.3 DMA functional description
- `10_10_3_DMA_functional_description__c02.md` §10.3 — DMA functional description — 10.3.1 General description
- `10_10_3_DMA_functional_description__c03.md` §10.3 — DMA functional description — 10.3.3 Channel selection
- `10_10_3_DMA_functional_description__c04.md` §10.3 — DMA functional description — Peripheral-to-memory mode
- `10_10_3_DMA_functional_description__c05.md` §10.3 — DMA functional description — 10.3.9 Double buffer mode
- `10_10_3_DMA_functional_description__c06.md` §10.3 — DMA functional description — 10.3.10 Programmable data width, packing/unpacking, endianness
- `10_10_3_DMA_functional_description__c07.md` §10.3 — DMA functional description — 10.3.11 Single and burst transfers
- `10_10_3_DMA_functional_description__c08.md` §10.3 — DMA functional description — FIFO structure
- `10_10_3_DMA_functional_description__c09.md` §10.3 — DMA functional description — 10.3.14 DMA transfer suspension
- `10_10_3_DMA_functional_description__c10.md` §10.3 — DMA functional description — 10.3.15 Flow controller
- `10_10_3_DMA_functional_description__c11.md` §10.3 — DMA functional description — 10.3.17 Stream configuration procedure
- `10_10_3_DMA_functional_description__c12.md` §10.3 — DMA functional description — 10.3.18 Error management
- `10_10_5_DMA_registers__c01.md` §10.5 — DMA registers — 10.5 DMA registers
- `10_10_5_DMA_registers__c02.md` §10.5 — DMA registers — 10.5.1 DMA low interrupt status register (DMA_LISR)
- `10_10_5_DMA_registers__c03.md` §10.5 — DMA registers — 10.5.2 DMA high interrupt status register (DMA_HISR)
- `10_10_5_DMA_registers__c04.md` §10.5 — DMA registers — 10.5.5 DMA stream x configuration register (DMA_SxCR) (x = 0..7)
- `10_10_5_DMA_registers__c05.md` §10.5 — DMA registers — 10.5.10 DMA stream x FIFO control register (DMA_SxFCR) (x = 0..7)
### 11 Chrom-Art DMA2D — NOT on F407 *(NOT on F407)*
- `11_11_1_DMA2D_introduction.md` §11.1 — DMA2D introduction
- `11_11_3_DMA2D_functional_description.md` §11.3 — DMA2D functional description
- `11_11_5_DMA2D_registers.md` §11.5 — DMA2D registers**
### 12 EXTI / interrupts and events / NVIC vectors
- `12_12_2_External_interruptevent_controller_EXTI.md` §12.2 — External interrupt/event controller (EXTI)
- `12_12_3_EXTI_registers.md` §12.3 — EXTI registers
### 13 Analog-to-digital converter (ADC)
- `13_13_1_ADC_introduction.md` §13.1 — ADC introduction
- `13_13_3_ADC_functional_description.md` §13.3 — ADC functional description
- `13_13_6_Conversion_on_external_trigger_and_trigger_po.md` §13.6 — Conversion on external trigger and trigger polarity**
- `13_13_8_Data_management.md` §13.8 — Data management**
- `13_13_9_Multi_ADC_mode.md` §13.9 — Multi ADC mode**
- `13_13_10_Temperature_sensor.md` §13.10 — Temperature sensor**
- `13_13_13_ADC_registers__c01.md` §13.13 — ADC registers** — 13.13 ADC registers**
- `13_13_13_ADC_registers__c02.md` §13.13 — ADC registers** — 13.13.2 ADC control register 1 (ADC_CR1)**
- `13_13_13_ADC_registers__c03.md` §13.13 — ADC registers** — 13.13.3 ADC control register 2 (ADC_CR2)**
- `13_13_13_ADC_registers__c04.md` §13.13 — ADC registers** — 13.13.15 ADC Common status register (ADC_CSR)**
- `13_13_13_ADC_registers__c05.md` §13.13 — ADC registers** — 13.13.16 ADC common control register (ADC_CCR)**
- `13_13_13_ADC_registers__c06.md` §13.13 — ADC registers** — 13.13.18 ADC register map**
### 14 Digital-to-analog converter (DAC)
- `14_14_2_DAC_main_features.md` §14.2 — DAC main features**
- `14_14_3_DAC_functional_description.md` §14.3 — DAC functional description**
- `14_14_4_Dual_DAC_channel_conversion.md` §14.4 — Dual DAC channel conversion**
- `14_14_5_DAC_registers.md` §14.5 — DAC registers**
### 15 DCMI — NOT on F407 *(NOT on F407)*
- `15_15_1_DCMI_introduction.md` §15.1 — DCMI introduction**
- `15_15_5_DCMI_functional_overview.md` §15.5 — DCMI functional overview**
- `15_15_6_Data_format_description.md` §15.6 — Data format description**
- `15_15_8_DCMI_register_description.md` §15.8 — DCMI register description**
### 16 LTDC — NOT on F407 *(NOT on F407)*
- `16_16_3_LTDC_functional_description.md` §16.3 — LTDC functional description**
- `16_16_4_LTDC_programmable_parameters.md` §16.4 — LTDC programmable parameters**
- `16_16_6_LTDC_programming_procedure.md` §16.6 — LTDC programming procedure**
- `16_16_7_LTDC_registers__c01.md` §16.7 — LTDC registers** — 16.7 LTDC registers**
- `16_16_7_LTDC_registers__c02.md` §16.7 — LTDC registers** — 16.7.5 LTDC Global Control Register (LTDC_GCR)**
- `16_16_7_LTDC_registers__c03.md` §16.7 — LTDC registers** — 16.7.6 LTDC Shadow Reload Configuration Register (LTDC_SRCR)**
- `16_16_7_LTDC_registers__c04.md` §16.7 — LTDC registers** — 16.7.14 LTDC Layerx Control Register (LTDC_LxCR) (where x=1..2)**
- `16_16_7_LTDC_registers__c05.md` §16.7 — LTDC registers** — 16.7.26 LTDC register map**
### 17 Advanced timers TIM1/TIM8
- `17_17_2_TIM1_and_TIM8_main_features.md` §17.2 — TIM1 and TIM8 main features**
- `17_17_3_1_Timebase_unit.md` §17.3.1 — Time-base unit**
- `17_17_3_2_Counter_modes.md` §17.3.2 — Counter modes**
- `17_17_3_3_Repetition_counter.md` §17.3.3 — Repetition counter**
- `17_17_3_4_Clock_selection.md` §17.3.4 — Clock selection**
- `17_17_3_5_Capturecompare_channels.md` §17.3.5 — Capture/compare channels**
- `17_17_3_6_Input_capture_mode.md` §17.3.6 — Input capture mode**
- `17_17_3_10_PWM_mode.md` §17.3.10 — PWM mode**
- `17_17_3_12_Using_the_break_function.md` §17.3.12 — Using the break function**
- `17_17_3_15_Onepulse_mode.md` §17.3.15 — One-pulse mode**
- `17_17_3_16_Encoder_interface_mode.md` §17.3.16 — Encoder interface mode**
- `17_17_3_18_Interfacing_with_Hall_sensors.md` §17.3.18 — Interfacing with Hall sensors**
- `17_17_3_19_TIMx_and_external_trigger_synchronization.md` §17.3.19 — TIMx and external trigger synchronization**
- `17_17_4_1_TIM1_and_TIM8_control_register_1_TIMxCR1.md` §17.4.1 — TIM1 and TIM8 control register 1 (TIMx_CR1)**
- `17_17_4_2_TIM1_and_TIM8_control_register_2_TIMxCR2.md` §17.4.2 — TIM1 and TIM8 control register 2 (TIMx_CR2)**
- `17_17_4_3_TIM1_and_TIM8_slave_mode_control_register_TIM.md` §17.4.3 — TIM1 and TIM8 slave mode control register (TIMx_SMCR)**
- `17_17_4_5_TIM1_and_TIM8_status_register_TIMxSR.md` §17.4.5 — TIM1 and TIM8 status register (TIMx_SR)**
- `17_17_4_6_TIM1_and_TIM8_event_generation_register_TIMxE.md` §17.4.6 — TIM1 and TIM8 event generation register (TIMx_EGR)**
- `17_17_4_7_TIM1_and_TIM8_capturecompare_mode_register_1.md` §17.4.7 — TIM1 and TIM8 capture/compare mode register 1 (TIMx_CCMR1)**
- `17_17_4_8_TIM1_and_TIM8_capturecompare_mode_register_2.md` §17.4.8 — TIM1 and TIM8 capture/compare mode register 2 (TIMx_CCMR2)**
- `17_17_4_9_TIM1_and_TIM8_capturecompare_enable_register.md` §17.4.9 — TIM1 and TIM8 capture/compare enable register (TIMx_CCER)**
- `17_17_4_18_TIM1_and_TIM8_break_and_deadtime_register_TIM.md` §17.4.18 — TIM1 and TIM8 break and dead-time register (TIMx_BDTR)**
- `17_17_4_20_TIM1_and_TIM8_DMA_address_for_full_transfer_T.md` §17.4.20 — TIM1 and TIM8 DMA address for full transfer (TIMx_DMAR)
### 18 General-purpose timers TIM2–TIM5
- `18_18_2_TIM2_to_TIM5_main_features.md` §18.2 — TIM2 to TIM5 main features
- `18_18_3_1_Timebase_unit.md` §18.3.1 — Time-base unit
- `18_18_3_2_Counter_modes.md` §18.3.2 — Counter modes**
- `18_18_3_3_Clock_selection.md` §18.3.3 — Clock selection
- `18_18_3_5_Input_capture_mode.md` §18.3.5 — Input capture mode
- `18_18_3_9_PWM_mode.md` §18.3.9 — PWM mode**
- `18_18_3_10_Onepulse_mode.md` §18.3.10 — One-pulse mode
- `18_18_3_11_Clearing_the_OCxREF_signal_on_an_external_eve.md` §18.3.11 — Clearing the OCxREF signal on an external event**
- `18_18_3_14_Timers_and_external_trigger_synchronization.md` §18.3.14 — Timers and external trigger synchronization**
- `18_18_3_15_Timer_synchronization.md` §18.3.15 — Timer synchronization
- `18_18_4_TIM2_to_TIM5_registers__c01.md` §18.4 — TIM2 to TIM5 registers — 18.4 TIM2 to TIM5 registers
- `18_18_4_TIM2_to_TIM5_registers__c02.md` §18.4 — TIM2 to TIM5 registers — 18.4.1 TIMx control register 1 (TIMx_CR1)
- `18_18_4_TIM2_to_TIM5_registers__c03.md` §18.4 — TIM2 to TIM5 registers — 18.4.2 TIMx control register 2 (TIMx_CR2)
- `18_18_4_TIM2_to_TIM5_registers__c04.md` §18.4 — TIM2 to TIM5 registers — 18.4.3 TIMx slave mode control register (TIMx_SMCR)
- `18_18_4_TIM2_to_TIM5_registers__c05.md` §18.4 — TIM2 to TIM5 registers — 18.4.4 TIMx DMA/Interrupt enable register (TIMx_DIER)
- `18_18_4_TIM2_to_TIM5_registers__c06.md` §18.4 — TIM2 to TIM5 registers — 18.4.5 TIMx status register (TIMx_SR)
- `18_18_4_TIM2_to_TIM5_registers__c07.md` §18.4 — TIM2 to TIM5 registers — 18.4.6 TIMx event generation register (TIMx_EGR)
- `18_18_4_TIM2_to_TIM5_registers__c08.md` §18.4 — TIM2 to TIM5 registers — 18.4.7 TIMx capture/compare mode register 1 (TIMx_CCMR1)
- `18_18_4_TIM2_to_TIM5_registers__c09.md` §18.4 — TIM2 to TIM5 registers — 18.4.8 TIMx capture/compare mode register 2 (TIMx_CCMR2)
- `18_18_4_TIM2_to_TIM5_registers__c10.md` §18.4 — TIM2 to TIM5 registers — 18.4.9 TIMx capture/compare enable register (TIMx_CCER)**
- `18_18_4_TIM2_to_TIM5_registers__c11.md` §18.4 — TIM2 to TIM5 registers — 18.4.18 TIMx DMA address for full transfer (TIMx_DMAR)
- `18_18_4_TIM2_to_TIM5_registers__c12.md` §18.4 — TIM2 to TIM5 registers — 18.4.20 TIM5 option register (TIM5_OR)
### 19 General-purpose timers TIM9–TIM14
- `19_19_2_TIM9_to_TIM14_main_features.md` §19.2 — TIM9 to TIM14 main features
- `19_19_3_TIM9_to_TIM14_functional_description__c01.md` §19.3 — TIM9 to TIM14 functional description — 19.3.1 Time-base unit
- `19_19_3_TIM9_to_TIM14_functional_description__c02.md` §19.3 — TIM9 to TIM14 functional description — 19.3.2 Counter modes
- `19_19_3_TIM9_to_TIM14_functional_description__c03.md` §19.3 — TIM9 to TIM14 functional description — 19.3.3 Clock selection
- `19_19_3_TIM9_to_TIM14_functional_description__c04.md` §19.3 — TIM9 to TIM14 functional description — 19.3.4 Capture/compare channels**
- `19_19_3_TIM9_to_TIM14_functional_description__c05.md` §19.3 — TIM9 to TIM14 functional description — 19.3.5 Input capture mode**
- `19_19_3_TIM9_to_TIM14_functional_description__c06.md` §19.3 — TIM9 to TIM14 functional description — 19.3.6 PWM input mode (only for TIM9/12)
- `19_19_3_TIM9_to_TIM14_functional_description__c07.md` §19.3 — TIM9 to TIM14 functional description — 19.3.8 Output compare mode
- `19_19_3_TIM9_to_TIM14_functional_description__c08.md` §19.3 — TIM9 to TIM14 functional description — 19.3.10 One-pulse mode**
- `19_19_3_TIM9_to_TIM14_functional_description__c09.md` §19.3 — TIM9 to TIM14 functional description — 19.3.11 TIM9/12 external trigger synchronization**
- `19_19_4_TIM9_and_TIM12_registers.md` §19.4 — TIM9 and TIM12 registers
- `19_19_5_TIM10111314_registers.md` §19.5 — TIM10/11/13/14 registers
### 20 Basic timers TIM6/TIM7
- `20_Basic_timers_TIM6_and_TIM7.md` §20 — Basic timers (TIM6 and TIM7)
### 21 Independent watchdog (IWDG)
- `21_Independent_watchdog_IWDG.md` §21 — Independent watchdog (IWDG)
### 22 Window watchdog (WWDG)
- `22_Window_watchdog_WWDG.md` §22 — Window watchdog (WWDG)
### 23 CRYP crypto processor
- `23_23_2_CRYP_main_features.md` §23.2 — CRYP main features
- `23_23_3_CRYP_functional_description__c01.md` §23.3 — CRYP functional description — 23.3 CRYP functional description
- `23_23_3_CRYP_functional_description__c02.md` §23.3 — CRYP functional description — 23.3.3 Data type**
- `23_23_3_CRYP_functional_description__c03.md` §23.3 — CRYP functional description — 23.3.4 Initialization vectors - CRYP_IV0...1(L/R)
- `23_23_3_CRYP_functional_description__c04.md` §23.3 — CRYP functional description — 23.3.6 Procedure to perform an encryption or a decryption
- `23_23_3_CRYP_functional_description__c05.md` §23.3 — CRYP functional description — 23.3.7 Context swapping**
- `23_23_4_CRYP_interrupts__c01.md` §23.4 — CRYP interrupts — 23.4 CRYP interrupts
- `23_23_4_CRYP_interrupts__c02.md` §23.4 — CRYP interrupts — Input FIFO service interrupt - INMIS
- `23_23_4_CRYP_interrupts__c03.md` §23.4 — CRYP interrupts — 23.6.2 CRYP control register (CRYP_CR) for STM32F415/417xx**
- `23_23_4_CRYP_interrupts__c04.md` §23.4 — CRYP interrupts — 23.6.4 CRYP data input register (CRYP_DIN)
- `23_23_4_CRYP_interrupts__c05.md` §23.4 — CRYP interrupts — 23.6.5 CRYP data output register (CRYP_DOUT)
- `23_23_4_CRYP_interrupts__c06.md` §23.4 — CRYP interrupts — 23.6.10 CRYP key registers (CRYP_K0...3(L/R)R)
- `23_23_4_CRYP_interrupts__c07.md` §23.4 — CRYP interrupts — 23.6.11 CRYP initialization vector registers (CRYP_IV0...1(L/R)R)**
- `23_23_4_CRYP_interrupts__c08.md` §23.4 — CRYP interrupts — 23.6.12 CRYP context swap registers (CRYP_CSGCMCCM0..7R and CRYP_CSGCM0..7R) for STM32F42xxx and STM32F43xxx**
### 24 Random number generator (RNG)
- `24_Random_number_generator_RNG.md` §24 — Random number generator (RNG)
### 25 HASH processor
- `25_25_2_HASH_main_features.md` §25.2 — HASH main features
- `25_25_3_HASH_functional_description.md` §25.3 — HASH functional description
- `25_25_4_HASH_registers__c01.md` §25.4 — HASH registers — 25.4 HASH registers
- `25_25_4_HASH_registers__c02.md` §25.4 — HASH registers — 25.4.1 HASH control register (HASH_CR) for STM32F415/417xx
- `25_25_4_HASH_registers__c03.md` §25.4 — HASH registers — 25.4.2 HASH control register (HASH_CR) for STM32F43xxx
- `25_25_4_HASH_registers__c04.md` §25.4 — HASH registers — 25.4.4 HASH start register (HASH_STR)
- `25_25_4_HASH_registers__c05.md` §25.4 — HASH registers — HASH_HR2
- `25_25_4_HASH_registers__c06.md` §25.4 — HASH registers — HASH_CSRx
### 26 Real-time clock (RTC)
- `26_26_2_RTC_main_features.md` §26.2 — RTC main features
- `26_26_3_RTC_functional_description__c01.md` §26.3 — RTC functional description — 26.3 RTC functional description
- `26_26_3_RTC_functional_description__c02.md` §26.3 — RTC functional description — 26.3.1 Clock and prescalers
- `26_26_3_RTC_functional_description__c03.md` §26.3 — RTC functional description — 26.3.5 RTC initialization and configuration**
- `26_26_3_RTC_functional_description__c04.md` §26.3 — RTC functional description — 26.3.6 Reading the calendar**
- `26_26_3_RTC_functional_description__c05.md` §26.3 — RTC functional description — 26.3.8 RTC synchronization**
- `26_26_3_RTC_functional_description__c06.md` §26.3 — RTC functional description — 26.3.9 RTC reference clock detection
- `26_26_3_RTC_functional_description__c07.md` §26.3 — RTC functional description — 26.3.10 RTC coarse digital calibration
- `26_26_3_RTC_functional_description__c08.md` §26.3 — RTC functional description — 26.3.11 RTC smooth digital calibration**
- `26_26_3_RTC_functional_description__c09.md` §26.3 — RTC functional description — RTC backup registers
- `26_26_5_RTC_interrupts.md` §26.5 — RTC interrupts
- `26_26_6_RTC_registers__c01.md` §26.6 — RTC registers — 26.6 RTC registers
- `26_26_6_RTC_registers__c02.md` §26.6 — RTC registers — 26.6.3 RTC control register (RTC_CR)**
- `26_26_6_RTC_registers__c03.md` §26.6 — RTC registers — 26.6.4 RTC initialization and status register (RTC_ISR)**
- `26_26_6_RTC_registers__c04.md` §26.6 — RTC registers — 26.6.9 RTC alarm B register (RTC_ALRMBR)
- `26_26_6_RTC_registers__c05.md` §26.6 — RTC registers — 26.6.12 RTC shift control register (RTC_SHIFTR)**
- `26_26_6_RTC_registers__c06.md` §26.6 — RTC registers — 26.6.16 RTC calibration register (RTC_CALR)
- `26_26_6_RTC_registers__c07.md` §26.6 — RTC registers — 26.6.17 RTC tamper and alternate function configuration register (RTC_TAFCR)
- `26_26_6_RTC_registers__c08.md` §26.6 — RTC registers — 26.6.18 RTC alarm A sub second register (RTC_ALRMASSR)
- `26_26_6_RTC_registers__c09.md` §26.6 — RTC registers — 26.6.19 RTC alarm B sub second register (RTC_ALRMBSSR)**
### 27 Inter-integrated circuit (I2C)
- `27_27_2_IC_main_features.md` §27.2 — I²C main features
- `27_27_3_IC_functional_description__c01.md` §27.3 — I²C functional description — 27.3 I²C functional description
- `27_27_3_IC_functional_description__c02.md` §27.3 — I²C functional description — 27.3.1 Mode selection
- `27_27_3_IC_functional_description__c03.md` §27.3 — I²C functional description — Slave address transmission
- `27_27_3_IC_functional_description__c04.md` §27.3 — I²C functional description — Unique device identifier (UDID)
- `27_27_3_IC_functional_description__c05.md` §27.3 — I²C functional description — 27.3.8 DMA requests**
- `27_27_6_IC_registers.md` §27.6 — I²C registers
### 28 Serial peripheral interface (SPI)
- `28_28_1_SPI_introduction.md` §28.1 — SPI introduction
- `28_28_2_SPI_and_IS_main_features.md` §28.2 — SPI and I²S main features
- `28_28_3_SPI_functional_description__c01.md` §28.3 — SPI functional description — 28.3 SPI functional description
- `28_28_3_SPI_functional_description__c02.md` §28.3 — SPI functional description — 28.3.1 General description
- `28_28_3_SPI_functional_description__c03.md` §28.3 — SPI functional description — 28.3.2 Configuring the SPI in slave mode
- `28_28_3_SPI_functional_description__c04.md` §28.3 — SPI functional description — SPI TI protocol in slave mode
- `28_28_3_SPI_functional_description__c05.md` §28.3 — SPI functional description — 28.3.5 Data transmission and reception procedures
- `28_28_3_SPI_functional_description__c06.md` §28.3 — SPI functional description — 28.3.7 Status flags
- `28_28_3_SPI_functional_description__c07.md` §28.3 — SPI functional description — 28.3.8 Disabling the SPI
- `28_28_3_SPI_functional_description__c08.md` §28.3 — SPI functional description — 28.3.9 SPI communication using DMA (direct memory addressing)
- `28_28_3_SPI_functional_description__c09.md` §28.3 — SPI functional description — Overrun condition
- `28_28_4_IS_functional_description__c01.md` §28.4 — I²S functional description — 28.4 I²S functional description
- `28_28_4_IS_functional_description__c02.md` §28.4 — I²S functional description — 28.4.1 I²S general description
- `28_28_4_IS_functional_description__c03.md` §28.4 — I²S functional description — I²S Philips standard
- `28_28_4_IS_functional_description__c04.md` §28.4 — I²S functional description — 28.4.4 Clock generator**
- `28_28_4_IS_functional_description__c05.md` §28.4 — I²S functional description — 28.4.5 I²S master mode
- `28_28_4_IS_functional_description__c06.md` §28.4 — I²S functional description — 28.4.6 I²S slave mode**
- `28_28_4_IS_functional_description__c07.md` §28.4 — I²S functional description — 28.4.8 Error flags**
- `28_28_5_SPI_and_IS_registers.md` §28.5 — SPI and I²S registers
### 29 SAI serial audio — NOT on F407 *(NOT on F407)*
- `29_29_2_Main_features.md` §29.2 — Main features
- `29_29_3_Functional_block_diagram.md` §29.3 — Functional block diagram
- `29_29_4_Main_SAI_modes.md` §29.4 — Main SAI modes
- `29_29_7_Frame_synchronization.md` §29.7 — Frame synchronization
- `29_29_8_Slot_configuration.md` §29.8 — Slot configuration
- `29_29_9_SAI_clock_generator.md` §29.9 — SAI clock generator
- `29_29_10_Internal_FIFOs.md` §29.10 — Internal FIFOs**
- `29_29_11_AC97_link_controller.md` §29.11 — AC'97 link controller
- `29_29_12_Specific_features.md` §29.12 — Specific features
- `29_29_17_SAI_registers__c01.md` §29.17 — SAI registers — 29.17 SAI registers
- `29_29_17_SAI_registers__c02.md` §29.17 — SAI registers — 29.17.1 SAI xConfiguration register 1 (SAI_xCR1) where x is A or B
- `29_29_17_SAI_registers__c03.md` §29.17 — SAI registers — 29.17.2 SAI xConfiguration register 2 (SAI_xCR2) where x is A or B
- `29_29_17_SAI_registers__c04.md` §29.17 — SAI registers — 29.17.3 SAI xFrame configuration register (SAI_XFRCR) where x is A or B
- `29_29_17_SAI_registers__c05.md` §29.17 — SAI registers — 29.17.4 SAI xSlot register (SAI_xSLOTR) where x is A or B
- `29_29_17_SAI_registers__c06.md` §29.17 — SAI registers — 29.17.5 SAI xInterrupt mask register (SAI_xIM) where x is A or B**
- `29_29_17_SAI_registers__c07.md` §29.17 — SAI registers — 29.17.6 SAI xStatus register (SAI_xSR) where x is A or B
- `29_29_17_SAI_registers__c08.md` §29.17 — SAI registers — 29.17.7 SAI xClear flag register (SAI_xCLRFR) where X is A or B**
- `29_29_17_SAI_registers__c09.md` §29.17 — SAI registers — 29.17.9 SAI register map
### 30 Universal synchronous/asynchronous receiver (USART)
- `30_30_2_USART_main_features.md` §30.2 — USART main features
- `30_30_3_USART_functional_description_intro.md` §30.3 — USART functional description (intro)
- `30_30_3_2_Transmitter.md` §30.3.2 — Transmitter
- `30_30_3_3_Receiver.md` §30.3.3 — Receiver**
- `30_30_3_4_Fractional_baud_rate_generation.md` §30.3.4 — Fractional baud rate generation
- `30_30_3_6_Multiprocessor_communication.md` §30.3.6 — Multiprocessor communication
- `30_30_3_8_LIN_local_interconnection_network_mode.md` §30.3.8 — LIN (local interconnection network) mode
- `30_30_3_9_USART_synchronous_mode.md` §30.3.9 — USART synchronous mode
- `30_30_3_11_Smartcard.md` §30.3.11 — Smartcard
- `30_30_3_12_IrDA_SIR_ENDEC_block.md` §30.3.12 — IrDA SIR ENDEC block
- `30_30_3_13_Continuous_communication_using_DMA.md` §30.3.13 — Continuous communication using DMA
- `30_30_3_14_Hardware_flow_control.md` §30.3.14 — Hardware flow control**
- `30_30_6_USART_registers.md` §30.6 — USART registers
### 31 SDIO
- `31_31_1_SDIO_main_features.md` §31.1 — SDIO main features
- `31_31_2_SDIO_bus_topology.md` §31.2 — SDIO bus topology
- `31_31_3_SDIO_functional_description.md` §31.3 — SDIO functional description
- `31_31_4_Card_functional_description__c01.md` §31.4 — Card functional description** — 31.4 Card functional description**
- `31_31_4_Card_functional_description__c02.md` §31.4 — Card functional description** — 31.4.4 Card identification process
- `31_31_4_Card_functional_description__c03.md` §31.4 — Card functional description** — 31.4.5 Block write
- `31_31_4_Card_functional_description__c04.md` §31.4 — Card functional description** — 31.4.7 Stream access, stream write and stream read (MultiMediaCard only)
- `31_31_4_Card_functional_description__c05.md` §31.4 — Card functional description** — 31.4.10 Protection management
- `31_31_4_Card_functional_description__c06.md` §31.4 — Card functional description** — 31.4.11 Card status register**
- `31_31_4_Card_functional_description__c07.md` §31.4 — Card functional description** — 31.4.12 SD status register
- `31_31_4_Card_functional_description__c08.md` §31.4 — Card functional description** — SD I/O interrupts
- `31_31_4_Card_functional_description__c09.md` §31.4 — Card functional description** — 31.4.14 Commands and responses**
- `31_31_4_Card_functional_description__c10.md` §31.4 — Card functional description** — Commands for the MultiMediaCard/SD module
- `31_31_5_Response_formats.md` §31.5 — Response formats
- `31_31_6_SDIO_IO_cardspecific_operations.md` §31.6 — SDIO I/O card-specific operations
- `31_31_7_CEATA_specific_operations.md` §31.7 — CE-ATA specific operations
- `31_31_9_SDIO_registers__c01.md` §31.9 — SDIO registers** — 31.9 SDIO registers**
- `31_31_9_SDIO_registers__c02.md` §31.9 — SDIO registers** — 31.9.2 SDI clock control register (SDIO_CLKCR)
- `31_31_9_SDIO_registers__c03.md` §31.9 — SDIO registers** — 31.9.4 SDIO command register (SDIO\_CMD)
- `31_31_9_SDIO_registers__c04.md` §31.9 — SDIO registers** — 31.9.9 SDIO data control register (SDIO_DCTRL)
- `31_31_9_SDIO_registers__c05.md` §31.9 — SDIO registers** — 31.9.11 SDIO status register (SDIO_STA)
- `31_31_9_SDIO_registers__c06.md` §31.9 — SDIO registers** — 31.9.12 SDIO interrupt clear register (SDIO\_ICR)
- `31_31_9_SDIO_registers__c07.md` §31.9 — SDIO registers** — 31.9.13 SDIO mask register (SDIO_MASK)
- `31_31_9_SDIO_registers__c08.md` §31.9 — SDIO registers** — 31.9.16 SDIO register map**
### 32 bxCAN
- `32_32_1_bxCAN_introduction.md` §32.1 — bxCAN introduction
- `32_32_3_bxCAN_general_description.md` §32.3 — bxCAN general description
- `32_32_4_bxCAN_operating_modes.md` §32.4 — bxCAN operating modes**
- `32_32_5_Test_mode.md` §32.5 — Test mode**
- `32_32_7_bxCAN_functional_description.md` §32.7 — bxCAN functional description
- `32_32_8_bxCAN_interrupts.md` §32.8 — bxCAN interrupts**
- `32_32_9_CAN_registers__c01.md` §32.9 — CAN registers — 32.9 CAN registers
- `32_32_9_CAN_registers__c02.md` §32.9 — CAN registers — 32.9.2 CAN control and status registers
- `32_32_9_CAN_registers__c03.md` §32.9 — CAN registers — CAN error status register (CAN_ESR)
- `32_32_9_CAN_registers__c04.md` §32.9 — CAN registers — 32.9.3 CAN mailbox registers
- `32_32_9_CAN_registers__c05.md` §32.9 — CAN registers — CAN receive FIFO mailbox identifier register (CAN_RIxR) (x=0..1)
- `32_32_9_CAN_registers__c06.md` §32.9 — CAN registers — CAN filter scale register (CAN_FS1R)
- `32_32_9_CAN_registers__c07.md` §32.9 — CAN registers — 32.9.5 bxCAN register map
### 33 Ethernet MAC (with DMA)
- `33_33_2_Ethernet_main_features.md` §33.2 — Ethernet main features
- `33_33_4_Ethernet_functional_description_SMI_MII_and_R.md` §33.4 — Ethernet functional description: SMI, MII and RMII
- `33_33_5_1_MAC_8023_frame_format.md` §33.5.1 — MAC 802.3 frame format
- `33_33_5_2_MAC_frame_transmission.md` §33.5.2 — MAC frame transmission
- `33_33_5_3_MAC_frame_reception.md` §33.5.3 — MAC frame reception
- `33_33_5_5_MAC_filtering.md` §33.5.5 — MAC filtering**
- `33_33_5_8_Power_management_PMT.md` §33.5.8 — Power management: PMT
- `33_33_5_9_Precision_time_protocol_IEEE1588_PTP.md` §33.5.9 — Precision time protocol (IEEE1588 PTP)
- `33_33_6_Ethernet_functional_description_DMA_controlle.md` §33.6 — Ethernet functional description: DMA controller operation** (intro)
- `33_33_6_7_Tx_DMA_configuration.md` §33.6.7 — Tx DMA configuration
- `33_33_6_8_Rx_DMA_configuration__c01.md` §33.6.8 — Rx DMA configuration — 33.6.8 Rx DMA configuration
- `33_33_7_Ethernet_interrupts.md` §33.7 — Ethernet interrupts
- `33_33_8_Ethernet_register_descriptions_intro__c01.md` §33.8 — Ethernet register descriptions (intro) — 33.8 Ethernet register descriptions
- `33_33_8_Ethernet_register_descriptions_intro__c02.md` §33.8 — Ethernet register descriptions (intro) — 33.8.1 MAC register description
- `33_33_8_Ethernet_register_descriptions_intro__c03.md` §33.8 — Ethernet register descriptions (intro) — Ethernet MAC frame filter register (ETH_MACFFR)
- `33_33_8_Ethernet_register_descriptions_intro__c04.md` §33.8 — Ethernet register descriptions (intro) — Ethernet MAC flow control register (ETH_MACFCR)
- `33_33_8_2_MMC_register_description.md` §33.8.2 — MMC register description
- `33_33_8_3_IEEE_1588_time_stamp_registers.md` §33.8.3 — IEEE 1588 time stamp registers
- `33_33_8_4_DMA_register_description.md` §33.8.4 — DMA register description
- `33_33_8_5_Ethernet_register_maps.md` §33.8.5 — Ethernet register maps
### 34 USB OTG full-speed (OTG_FS)
- `34_34_2_OTGFS_main_features.md` §34.2 — OTG_FS main features
- `34_34_3_OTGFS_functional_description.md` §34.3 — OTG_FS functional description
- `34_34_4_OTG_dual_role_device_DRD.md` §34.4 — OTG dual role device (DRD)
- `34_34_5_USB_peripheral.md` §34.5 — USB peripheral
- `34_34_6_USB_host.md` §34.6 — USB host**
- `34_34_7_SOF_trigger.md` §34.7 — SOF trigger
- `34_34_8_OTG_lowpower_modes.md` §34.8 — OTG low-power modes
- `34_34_11_Peripheral_FIFO_architecture.md` §34.11 — Peripheral FIFO architecture
- `34_34_12_Host_FIFO_architecture.md` §34.12 — Host FIFO architecture
- `34_34_13_FIFO_RAM_allocation.md` §34.13 — FIFO RAM allocation
- `34_34_14_USB_system_performance.md` §34.14 — USB system performance
- `34_34_15_OTGFS_interrupts.md` §34.15 — OTG_FS interrupts
- `34_34_16_1_CSR_memory_map.md` §34.16.1 — CSR memory map**
- `34_34_16_2_OTGFS_global_registers__c01.md` §34.16.2 — OTG_FS global registers — 34.16.2 OTG_FS global registers
- `34_34_16_2_OTGFS_global_registers__c02.md` §34.16.2 — OTG_FS global registers — OTG_FS AHB configuration register (OTG_FS_GAHBCFG)
- `34_34_16_3_Hostmode_registers.md` §34.16.3 — Host-mode registers
- `34_34_16_4_Devicemode_registers__c01.md` §34.16.4 — Device-mode registers — 34.16.4 Device-mode registers
- `34_34_16_6_OTGFS_register_map.md` §34.16.6 — OTG_FS register map
- `34_34_17_OTGFS_programming_model_intro.md` §34.17 — OTG_FS programming model (intro)
- `34_34_17_4_Host_programming_model.md` §34.17.4 — Host programming model
- `34_34_17_5_Device_programming_model.md` §34.17.5 — Device programming model
- `34_34_17_6_Operational_model.md` §34.17.6 — Operational model**
- `34_34_17_7_Worst_case_response_time.md` §34.17.7 — Worst case response time**
### 35 USB OTG high-speed (OTG_HS)
- `35_35_1_OTGHS_introduction.md` §35.1 — OTG_HS introduction
- `35_35_2_OTGHS_main_features.md` §35.2 — OTG_HS main features
- `35_35_3_OTGHS_functional_description.md` §35.3 — OTG_HS functional description
- `35_35_4_OTG_dualrole_device.md` §35.4 — OTG dual-role device
- `35_35_5_USB_functional_description_in_peripheral_mode.md` §35.5 — USB functional description in peripheral mode
- `35_35_6_USB_functional_description_on_host_mode.md` §35.6 — USB functional description on host mode
- `35_35_7_SOF_trigger.md` §35.7 — SOF trigger
- `35_35_8_OTGHS_lowpower_modes.md` §35.8 — OTG_HS low-power modes
- `35_35_10_FIFO_RAM_allocation.md` §35.10 — FIFO RAM allocation
- `35_35_11_OTGHS_interrupts.md` §35.11 — OTG_HS interrupts
- `35_35_12_1_CSR_memory_map.md` §35.12.1 — CSR memory map
- `35_35_12_2_OTGHS_global_registers__c01.md` §35.12.2 — OTG_HS global registers — 35.12.2 OTG_HS global registers
- `35_35_12_2_OTGHS_global_registers__c02.md` §35.12.2 — OTG_HS global registers — OTG_HS AHB configuration register (OTG_HS_GAHBCFG)
- `35_35_12_2_OTGHS_global_registers__c03.md` §35.12.2 — OTG_HS global registers — OTG_HS reset register (OTG_HS_GRSTCTL)
- `35_35_12_2_OTGHS_global_registers__c04.md` §35.12.2 — OTG_HS global registers — OTG_HS core interrupt register (OTG_HS_GINTSTS)
- `35_35_12_3_Hostmode_registers.md` §35.12.3 — Host-mode registers**
- `35_35_12_4_Devicemode_registers__c01.md` §35.12.4 — Device-mode registers — 35.12.4 Device-mode registers
- `35_35_12_4_Devicemode_registers__c02.md` §35.12.4 — Device-mode registers — OTG_HS device control register (OTG_HS_DCTL)
- `35_35_12_4_Devicemode_registers__c03.md` §35.12.4 — Device-mode registers — OTG_HS device status register (OTG_HS_DSTS)
- `35_35_12_4_Devicemode_registers__c04.md` §35.12.4 — Device-mode registers — OTG_HS device control OUT endpoint 0 control register (OTG_HS_DOEPCTL0)
- `35_35_12_4_Devicemode_registers__c05.md` §35.12.4 — Device-mode registers — OTG_HS device OUT endpoint 0 transfer size register (OTG_HS_DOEPTSIZ0)
- `35_35_12_6_OTGHS_register_map.md` §35.12.6 — OTG_HS register map
- `35_35_13_OTGHS_programming_model_intro.md` §35.13 — OTG_HS programming model (intro)
- `35_35_13_5_Host_programming_model.md` §35.13.5 — Host programming model
- `35_35_13_6_Device_programming_model.md` §35.13.6 — Device programming model
- `35_35_13_7_Operational_model.md` §35.13.7 — Operational model**
- `35_35_13_8_Worst_case_response_time.md` §35.13.8 — Worst case response time**
- `35_35_13_9_OTG_programming_model.md` §35.13.9 — OTG programming model
### 36 Flexible static memory controller (FSMC)
- `36_36_1_FSMC_main_features.md` §36.1 — FSMC main features
- `36_36_3_AHB_interface.md` §36.3 — AHB interface
- `36_36_4_External_device_address_mapping.md` §36.4 — External device address mapping**
- `36_36_5_NOR_FlashPSRAM_controller__c01.md` §36.5 — NOR Flash/PSRAM controller — 36.5 NOR Flash/PSRAM controller
- `36_36_5_NOR_FlashPSRAM_controller__c02.md` §36.5 — NOR Flash/PSRAM controller — 36.5.1 External memory interface signals**
- `36_36_5_NOR_FlashPSRAM_controller__c03.md` §36.5 — NOR Flash/PSRAM controller — 36.5.2 Supported memories and transactions
- `36_36_5_NOR_FlashPSRAM_controller__c04.md` §36.5 — NOR Flash/PSRAM controller — 36.5.4 NOR Flash/PSRAM controller asynchronous transactions
- `36_36_5_NOR_FlashPSRAM_controller__c05.md` §36.5 — NOR Flash/PSRAM controller — Wait management
- `36_36_5_NOR_FlashPSRAM_controller__c06.md` §36.5 — NOR Flash/PSRAM controller — 36.5.6 NOR/PSRAM control registers
- `36_36_6_NAND_FlashPC_Card_controller__c01.md` §36.6 — NAND Flash/PC Card controller — 36.6 NAND Flash/PC Card controller
- `36_36_6_NAND_FlashPC_Card_controller__c02.md` §36.6 — NAND Flash/PC Card controller — 36.6.4 NAND Flash operations
- `36_36_6_NAND_FlashPC_Card_controller__c03.md` §36.6 — NAND Flash/PC Card controller — 36.6.6 Computation of the error correction code (ECC) in NAND Flash memory
- `36_36_6_NAND_FlashPC_Card_controller__c04.md` §36.6 — NAND Flash/PC Card controller — 36.6.7 PC Card/CompactFlash operations
- `36_36_6_NAND_FlashPC_Card_controller__c05.md` §36.6 — NAND Flash/PC Card controller — 36.6.8 NAND Flash/PC Card control registers
- `36_36_6_NAND_FlashPC_Card_controller__c06.md` §36.6 — NAND Flash/PC Card controller — Common memory space timing register 2..4 (FSMC_PMEM2..4)
### 37 FMC — NOT on F407 (F407 has FSMC ch.36) *(NOT on F407)*
- `37_37_1_FMC_main_features.md` §37.1 — FMC main features
- `37_37_2_Block_diagram.md` §37.2 — Block diagram
- `37_37_4_External_device_address_mapping.md` §37.4 — External device address mapping
- `37_37_5_NOR_FlashPSRAM_controller__c01.md` §37.5 — NOR Flash/PSRAM controller — 37.5 NOR Flash/PSRAM controller
- `37_37_5_NOR_FlashPSRAM_controller__c02.md` §37.5 — NOR Flash/PSRAM controller — 37.5.1 External memory interface signals
- `37_37_5_NOR_FlashPSRAM_controller__c03.md` §37.5 — NOR Flash/PSRAM controller — 37.5.2 Supported memories and transactions**
- `37_37_5_NOR_FlashPSRAM_controller__c04.md` §37.5 — NOR Flash/PSRAM controller — 37.5.4 NOR Flash/PSRAM controller asynchronous transactions
- `37_37_5_NOR_FlashPSRAM_controller__c05.md` §37.5 — NOR Flash/PSRAM controller — Data latency versus NOR memory latency
- `37_37_5_NOR_FlashPSRAM_controller__c06.md` §37.5 — NOR Flash/PSRAM controller — 37.5.6 NOR/PSRAM controller registers
- `37_37_6_NAND_FlashPC_Card_controller__c01.md` §37.6 — NAND Flash/PC Card controller — 37.6 NAND Flash/PC Card controller
- `37_37_6_NAND_FlashPC_Card_controller__c02.md` §37.6 — NAND Flash/PC Card controller — 37.6.3 Timing diagrams for NAND Flash memory and PC Card
- `37_37_6_NAND_FlashPC_Card_controller__c03.md` §37.6 — NAND Flash/PC Card controller — 37.6.6 Computation of the error correction code (ECC) in NAND Flash memory
- `37_37_6_NAND_FlashPC_Card_controller__c04.md` §37.6 — NAND Flash/PC Card controller — Address spaces and memory accesses
- `37_37_6_NAND_FlashPC_Card_controller__c05.md` §37.6 — NAND Flash/PC Card controller — 37.6.8 NAND Flash/PC Card controller registers**
- `37_37_7_SDRAM_controller__c01.md` §37.7 — SDRAM controller — 37.7 SDRAM controller
- `37_37_7_SDRAM_controller__c02.md` §37.7 — SDRAM controller — SDRAM initialization
- `37_37_7_SDRAM_controller__c03.md` §37.7 — SDRAM controller — SDRAM controller read cycle
- `37_37_7_SDRAM_controller__c04.md` §37.7 — SDRAM controller — 37.7.4 Low power modes
- `37_37_7_SDRAM_controller__c05.md` §37.7 — SDRAM controller — 37.7.5 SDRAM controller registers
- `37_37_8_FMC_register_map.md` §37.8 — FMC register map
### 38 Debug support (DBG)
- `38_38_1_Overview.md` §38.1 — Overview
- `38_38_4_Pinout_and_debug_port_pins.md` §38.4 — Pinout and debug port pins
- `38_38_6_ID_codes_and_locking_mechanism.md` §38.6 — ID codes and locking mechanism
- `38_38_7_JTAG_debug_port.md` §38.7 — JTAG debug port
- `38_38_8_SW_debug_port.md` §38.8 — SW debug port
- `38_38_9_AHBAP_AHB_access_port_valid_for_both_JTAGDP_a.md` §38.9 — AHB-AP (AHB access port) - valid for both JTAG-DP and SW-DP
- `38_38_10_Core_debug.md` §38.10 — Core debug
- `38_38_14_ITM_instrumentation_trace_macrocell.md` §38.14 — ITM (instrumentation trace macrocell)
- `38_38_15_ETM_Embedded_trace_macrocell.md` §38.15 — ETM (Embedded trace macrocell)**
- `38_38_16_MCU_debug_component_DBGMCU.md` §38.16 — MCU debug component (DBGMCU)
- `38_38_17_TPIU_trace_port_interface_unit.md` §38.17 — TPIU (trace port interface unit)
- `38_38_18_DBG_register_map.md` §38.18 — DBG register map
### 39 Device electronic signature (UID, flash size)
- `39_Device_electronic_signature.md` §39 — Device electronic signature

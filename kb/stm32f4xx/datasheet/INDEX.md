# Datasheet index — STM32F405/407 (DM00037051, production data) для RAG

> Параметры чипа: пины, AF-маппинг, электрика, варианты. Детали регистров — в ../rm0090_sections/, баги кремния — в ../errata_sections/.

## Быстрый поиск: задача → файл

- **Распиновка корпуса (LQFP48/64/100/144/176, UFBGA176, WLCSP90)** → `datasheet_04_Pinouts__c01.md`, `datasheet_04_Pinouts__c02.md`
- **Назначение пинов, типы, I/O structure (pin definition)** → `datasheet_04_Pinouts__c02.md`
- **Alternate function mapping: какой сигнал на каком пине через AF** → `datasheet_04_Pinouts__c04.md`, `datasheet_04_Pinouts__c05.md`, `datasheet_04_Pinouts__c06.md`, `datasheet_04_Pinouts__c07.md`
- **FSMC-пины** → `datasheet_04_Pinouts__c03.md`
- **Обзор периферии, частоты, объёмы памяти вариантов F405/407** → `datasheet_02_Description.md`, `datasheet_03_3_1_Arm_CortexM4_core_with_FPU_and_embedded_flash_and.md`
- **Boot modes, тактирование, PLLI2S** → `datasheet_03_3_1_Arm_CortexM4_core_with_FPU_and_embedded_flash_and.md`
- **Питание, VBAT, регулятор, low-power overview** → `datasheet_03_3_15_Power_supply_supervisor.md`, `datasheet_03_3_16_Voltage_regulator.md`, `datasheet_03_3_19_Lowpower_modes.md`
- **Таймеры/собаки: какие есть и их возможности** → `datasheet_03_3_21_Timers_and_watchdogs.md`
- **USART/SPI/I2C/SDIO/Ethernet/USB/CAN/ADC/DAC — обзорные возможности** → `datasheet_03_3_23_Universal_synchronousasynchronous_receiver_transmi.md`, `datasheet_03_3_28_Ethernet_MAC_interface_with_dedicated_DMA_and_IEEE.md`
- **Карта памяти, base addresses** → `datasheet_05_Memory_mapping.md`
- **Предельные параметры, токи потребления по режимам** → `datasheet_06_Electrical_characteristics__c04.md`, `datasheet_07_Electrical_characteristics_cont.md`
- **Питание: схема, ёмкости, wakeup-времена** → `datasheet_06_Electrical_characteristics__c02.md`, `datasheet_06_Electrical_characteristics__c03.md`, `datasheet_06_Electrical_characteristics__c05.md`
- **PLL: характеристики, spread spectrum** → `datasheet_06_Electrical_characteristics__c06.md`, `datasheet_06_Electrical_characteristics__c07.md`
- **GPIO: скорости, токи, injection** → `datasheet_06_Electrical_characteristics__c09.md`, `datasheet_06_Electrical_characteristics__c10.md`
- **ADC/DAC точность, времена преобразования** → `datasheet_06_Electrical_characteristics__c14.md`, `datasheet_06_Electrical_characteristics__c15.md`
- **UART/SPI/I2C/CAN/SDIO тайминги** → `datasheet_06_Electrical_characteristics__c13.md`, `datasheet_06_Electrical_characteristics__c17.md`
- **FSMC тайминги, SDRAM** → `datasheet_06_Electrical_characteristics__c16.md`
- **Варианты заказа: flash/RAM/корпус/температура** → `datasheet_08_Ordering_information.md`

## Все файлы

- `datasheet_01_Introduction.md` §1 — Introduction
- `datasheet_02_Description.md` §2 — Description
- `datasheet_03_3_1_Arm_CortexM4_core_with_FPU_and_embedded_flash_and.md` §3.1 — Arm® Cortex®-M4 core with FPU and embedded flash and SRAM
- `datasheet_03_3_15_Power_supply_supervisor.md` §3.15 — Power supply supervisor
- `datasheet_03_3_16_Voltage_regulator.md` §3.16 — Voltage regulator
- `datasheet_03_3_18_Realtime_clock_RTC_backup_SRAM_and_backup_register.md` §3.18 — Real-time clock (RTC), backup SRAM and backup registers
- `datasheet_03_3_19_Lowpower_modes.md` §3.19 — Low-power modes
- `datasheet_03_3_21_Timers_and_watchdogs.md` §3.21 — Timers and watchdogs
- `datasheet_03_3_23_Universal_synchronousasynchronous_receiver_transmi.md` §3.23 — Universal synchronous/asynchronous receiver transmitters (USART)
- `datasheet_03_3_28_Ethernet_MAC_interface_with_dedicated_DMA_and_IEEE.md` §3.28 — Ethernet MAC interface with dedicated DMA and IEEE 1588 support
- `datasheet_04_Pinouts__c01.md` §4 — Pinouts — (intro)
- `datasheet_04_Pinouts__c02.md` §4 — Pinouts — Figure 16 STM32F40xxx UFBGA176 ballout
- `datasheet_04_Pinouts__c03.md` §4 — Pinouts — Table 8 FSMC pin definition
- `datasheet_04_Pinouts__c04.md` §4 — Pinouts — Table 9 Alternate function mapping
- `datasheet_04_Pinouts__c05.md` §4 — Pinouts — Table 9 Alternate function mapping (continued)
- `datasheet_04_Pinouts__c06.md` §4 — Pinouts — Table 9 Alternate function mapping (continued)
- `datasheet_04_Pinouts__c07.md` §4 — Pinouts — Table 9 Alternate function mapping (continued)
- `datasheet_05_Memory_mapping.md` §5 — Memory mapping
- `datasheet_06_Electrical_characteristics__c01.md` §6 — Electrical characteristics — 6.1 Parameter conditions
- `datasheet_06_Electrical_characteristics__c02.md` §6 — Electrical characteristics — 6.1.6 Power supply scheme
- `datasheet_06_Electrical_characteristics__c03.md` §6 — Electrical characteristics — 6.3.5 Embedded reset and power control block characteristics
- `datasheet_06_Electrical_characteristics__c04.md` §6 — Electrical characteristics — 6.3.6 Supply current characteristics
- `datasheet_06_Electrical_characteristics__c05.md` §6 — Electrical characteristics — 6.3.7 Wakeup time from low-power mode
- `datasheet_06_Electrical_characteristics__c06.md` §6 — Electrical characteristics — 6.3.10 PLL characteristics
- `datasheet_06_Electrical_characteristics__c07.md` §6 — Electrical characteristics — 6.3.11 PLL spread spectrum clock generation (SSCG) characteristics
- `datasheet_06_Electrical_characteristics__c08.md` §6 — Electrical characteristics — 6.3.13 EMC characteristics
- `datasheet_06_Electrical_characteristics__c09.md` §6 — Electrical characteristics — 6.3.15 I/O current injection characteristics
- `datasheet_06_Electrical_characteristics__c10.md` §6 — Electrical characteristics — 6.3.16 I/O port characteristics
- `datasheet_06_Electrical_characteristics__c11.md` §6 — Electrical characteristics — 6.3.17 NRST pin characteristics
- `datasheet_06_Electrical_characteristics__c12.md` §6 — Electrical characteristics — 6.3.18 TIM timer characteristics
- `datasheet_06_Electrical_characteristics__c13.md` §6 — Electrical characteristics — 6.3.19 Communications interfaces
- `datasheet_06_Electrical_characteristics__c14.md` §6 — Electrical characteristics — 6.3.21 12-bit ADC characteristics
- `datasheet_06_Electrical_characteristics__c15.md` §6 — Electrical characteristics — 6.3.25 DAC electrical characteristics
- `datasheet_06_Electrical_characteristics__c16.md` §6 — Electrical characteristics — 6.3.26 FSMC characteristics
- `datasheet_06_Electrical_characteristics__c17.md` §6 — Electrical characteristics — 6.3.28 SD/SDIO MMC card host interface (SDIO) characteristics
- `datasheet_07_Electrical_characteristics_cont.md` §7 — Electrical characteristics (cont.)
- `datasheet_08_Ordering_information.md` §8 — Ordering information
- `datasheet_09_Important_security_notice.md` §9 — Important security notice

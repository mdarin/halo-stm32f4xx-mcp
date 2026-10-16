# Errata index — STM32F405/407/415/417 silicon limitations (для RAG)

> Каждая секция = известный баг кремния + workaround. Читай перед доверием к RM0090-поведению.

## Быстрый поиск: симптом/периферия → файл

- **Неизвестные сбросы/зависания, ART prefetch, flash** → `errata_02_2_1_System_limitations.md`
- **Отладка Sleep/Stop, WFE/WFI, systick в Stop** → `errata_02_2_1_System_limitations.md`
- **Wakeup из Standby с несколькими источниками** → `errata_02_2_1_System_limitations.md`
- **I2C: flag/stop, DMA, SMBus** → `errata_02_2_4_I2C_peripheral_limitations.md`
- **USART: LIN break, флаги, DMA** → `errata_02_2_6_USART_peripheral_limitations.md`
- **USB OTG_FS** → `errata_02_2_8_OTGFS_peripheral_limitations.md`
- **Ethernet MAC/DMA** → `errata_02_2_9_Ethernet_peripheral_limitations.md`
- **FSMC (SRAM/NAND)** → `errata_02_2_10_FSMC_peripheral_limitations.md`
- **SDIO** → `errata_02_2_11_SDIO_peripheral_limitations.md`
- **SPI: CRC, I2S, режимы** → `errata_02_2_14_SPI_peripheral_limitations.md`
- **Cortex-M4 core: SP load, VDIV/VSQRT в коротких ISR** → `errata_01_CortexM4_core_limitations.md`

## Все файлы

- `errata_01_CortexM4_core_limitations.md` §1 — Cortex-M4 core limitations
- `errata_02_2_1_System_limitations.md` §2.1 — System limitations
- `errata_02_2_4_I2C_peripheral_limitations.md` §2.4 — I2C peripheral limitations
- `errata_02_2_6_USART_peripheral_limitations.md` §2.6 — USART peripheral limitations
- `errata_02_2_8_OTGFS_peripheral_limitations.md` §2.8 — OTG_FS peripheral limitations
- `errata_02_2_9_Ethernet_peripheral_limitations.md` §2.9 — Ethernet peripheral limitations
- `errata_02_2_10_FSMC_peripheral_limitations.md` §2.10 — FSMC peripheral limitations
- `errata_02_2_11_SDIO_peripheral_limitations.md` §2.11 — SDIO peripheral limitations
- `errata_02_2_14_SPI_peripheral_limitations.md` §2.14 — SPI peripheral limitations

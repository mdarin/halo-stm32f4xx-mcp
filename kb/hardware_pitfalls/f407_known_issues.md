---
mcu: STM32F407
type: known_issues
tags: [errata, pitfalls, workarounds, ccm-dma, nvic, usb, timer, boot, flash]
schema_version: 1
---

# STM32F407 Known Issues & Hardware Pitfalls

Критические проблемы STM32F407, которые необходимо учитывать при разработке. Каждая проблема проверена на практике и имеет workaround.

---

## 1. CCM RAM + DMA — НЕСОВМЕСТИМО!

**Проблема:**
DMA контроллер **не имеет доступа** к CCM RAM (Core Coupled Memory, 64KB).
Если разместить буфер в `.ccmram`, DMA зависнет или будет читать мусор.

**Детали:**
- CCM RAM расположен по адресу `0x10000000` (см. RM0090 Chapter 2, Figure 2)
- DMA1/DMA2 находятся на AHB1 bus, CCM RAM — на DTCM bus
- Шина DTCM не видна для DMA контроллера
- Проблема проявляется при использовании: UART RX DMA, SPI DMA, ADC DMA, I2C DMA

**Решение:**
```c
// ПЛОХО (не работает!)
uint8_t dma_buffer[1024] __attribute__((section(".ccmram")));

// ХОРОШО — явно указать SRAM1
uint8_t dma_buffer[1024] __attribute__((section(".sram1")));

// ИЛИ просто без атрибута (по умолчанию в SRAM1)
uint8_t dma_buffer[1024];
```

**Как отладить:**
- Симптом: DMA transfer complete flag (`TCIF`) не устанавливается
- Симптом: В буфере мусор после DMA transfer
- Симптом: HardFault при обращении к буферу из DMA ISR
- Проверка: Откройте MAP файл — смотрите, где размещён буфер
- Проверка: `objdump -h firmware.elf | grep dma_buffer`

**Ссылки:**
- RM0090 Chapter 2: Memory and bus architecture (Figure 2)
- RM0090 Chapter 9: DMA controller (Section 9.3.1)
- Errata Sheet ES0113: DMA limitations

---

## 2. NVIC Priority Grouping

**Проблема:**
CubeMX по умолчанию устанавливает `NVIC_PRIORITYGROUP_4` (4 бита preemption, 0 бит subpriority).
Если вы вручную меняете grouping в коде — могут конфликтовать прерывания.

**Детали:**
- `NVIC_PRIORITYGROUP_4`: 4 бита preemption (16 уровней), 0 бит subpriority
- `NVIC_PRIORITYGROUP_3`: 3 бита preemption (8 уровней), 1 бит subpriority (2 уровня)
- Если разные части кода используют разные grouping — приоритеты работают непредсказуемо
- CubeMX генерирует `HAL_NVIC_SetPriorityGrouping(NVIC_PRIORITYGROUP_4)` в `main.c`

**Решение:**
```c
// Оставляйте как в CubeMX (обычно в main.c, USER CODE BEGIN 1)
HAL_NVIC_SetPriorityGrouping(NVIC_PRIORITYGROUP_4);

// Приоритеты: 0 (highest) - 15 (lowest)
HAL_NVIC_SetPriority(USART1_IRQn, 5, 0);  // Preemption = 5
HAL_NVIC_SetPriority(TIM2_IRQn, 10, 0);   // Preemption = 10 (ниже)

// НЕ МЕНЯЙТЕ grouping после инициализации!
```

**Как отладить:**
- Симптом: Прерывания не срабатывают в ожидаемом порядке
- Симптом: Одно прерывание блокирует другое
- Проверка: `NVIC->IPR[x]` — регистры приоритетов
- Проверка: GDB — `print/x NVIC->IPR`

**Ссылки:**
- RM0090 Chapter 12: EXTI / interrupts and events / NVIC vectors
- Cortex-M4 Technical Reference Manual: Nested Vectored Interrupt Controller

---

## 3. USB/SDIO требуют 48 MHz

**Проблема:**
USB OTG и SDIO требуют тактовую частоту ровно 48 MHz.
Если PLL настроен неправильно — USB не работает, SDIO работает нестабильно.

**Детали:**
- USB OTG FS/HS требуют `PLL48CLK = 48 MHz` (±0.1%)
- SDIO использует `48MHz` clock для SD card interface
- `PLL48CLK` получается из `VCO / PLLQ`
- Формула: `PLL48CLK = (HSE * PLLN / PLLM) / PLLQ`

**Решение:**
```c
// Проверьте RCC configuration в CubeMX
// HSE = 8 MHz -> PLL -> SYSCLK = 168 MHz
// PLL48CLK = 48 MHz (для USB)

// В CubeMX Clock Configuration:
// - HSE: 8 MHz
// - SYSCLK: 168 MHz
// - AHB: 168 MHz
// - APB1: 42 MHz (x2 таймеры = 84 MHz)
// - APB2: 84 MHz (x2 таймеры = 168 MHz)
// - 48 MHz clock for USB: OK

// Проверка в коде:
uint32_t pll48clk = HAL_RCCEx_GetPeriphCLKFreq(RCC_PERIPHCLK_CK48);
assert_param(pll48clk == 48000000);  // Должно быть 48 MHz
```

**Как отладить:**
- Симптом: USB device не определяется хостом
- Симптом: SD card не инициализируется
- Симптом: USB enumeration fails
- Проверка: `RCC->PLLCFGR` — регистр PLL configuration
- Проверка: `HAL_RCCEx_GetPeriphCLKFreq(RCC_PERIPHCLK_CK48)`

**Ссылки:**
- RM0090 Chapter 7: RCC (Section 7.3.2 PLL configuration)
- RM0090 Chapter 34: USB OTG FS (Section 34.3.1 Clock requirements)
- RM0090 Chapter 31: SDIO (Section 31.4 Clock requirements)

---

## 4. Таймеры APB1/APB2 умножают частоту

**Проблема:**
Если APB prescaler != 1, частота таймера умножается на 2 (APB1) или 4 (APB2).
Это влияет на расчёт PWM frequency, encoder resolution, input capture.

**Детали:**
- APB1 prescaler = 4 → TIM clock = APB1 × 2 = 84 MHz
- APB2 prescaler = 2 → TIM clock = APB2 × 2 = 168 MHz
- Если prescaler = 1 → TIM clock = APB clock (без умножения)
- Это влияет на: ARR, PSC, CCR расчёты

**Пример:**
```
SYSCLK = 168 MHz
AHB = 168 MHz (prescaler = 1)
APB1 = 42 MHz (prescaler = 4) → TIM clock = 84 MHz (x2)
APB2 = 84 MHz (prescaler = 2) → TIM clock = 168 MHz (x2)
```

**Решение:**
```c
// При расчёте prescaler для TIM:
// TIM_CLK = 84 MHz (для APB1 таймеров: TIM2-7, TIM12-14)
// TIM_CLK = 168 MHz (для APB2 таймеров: TIM1, TIM8-11)

// Пример: PWM 1 kHz на TIM2 (APB1, TIM_CLK = 84 MHz)
// Desired frequency = 1 kHz
// ARR = 999 (0..999 = 1000 отсчётов)
// PSC = TIM_CLK / (Desired * (ARR+1)) - 1
// PSC = 84000000 / (1000 * 1000) - 1 = 83

__HAL_TIM_SET_PRESCALER(&htim2, 83);  // Для 1 kHz
__HAL_TIM_SET_AUTORELOAD(&htim2, 999);

// Пример: PWM 20 kHz на TIM1 (APB2, TIM_CLK = 168 MHz)
// PSC = 168000000 / (20000 * 420) - 1 = 1
__HAL_TIM_SET_PRESCALER(&htim1, 1);
__HAL_TIM_SET_AUTORELOAD(&htim1, 419);
```

**Как отладить:**
- Симптом: PWM частота в 2 раза выше ожидаемой
- Симптом: Encoder считает в 2 раза быстрее
- Проверка: `RCC->CFGR` — регистр clock configuration
- Проверка: `TIMx->PSC` и `TIMx->ARR` — регистры таймера

**Ссылки:**
- RM0090 Chapter 7: RCC (Section 7.2 Clocks)
- RM0090 Chapter 17: Advanced timers TIM1/TIM8
- RM0090 Chapter 18: General-purpose timers TIM2-TIM5

---

## 5. Boot configuration

**Проблема:**
STM32F407 может загружаться из Flash, System Memory (bootloader), или SRAM.
Если BOOT0/BOOT1 неправильно настроены — прошивка не стартует.

**Детали:**
- BOOT0 (pin 60 on LQFP144) — sampled at reset
- BOOT1 (PB2) — sampled at reset when BOOT0 = 1
- Таблица загрузки:
  - `BOOT0=0, BOOT1=X` → Boot from Flash (нормальный режим)
  - `BOOT0=1, BOOT1=0` → Boot from System Memory (UART bootloader)
  - `BOOT0=1, BOOT1=1` → Boot from SRAM (для отладки)

**Решение:**
```c
// На плате:
// BOOT0 обычно подтянут к GND через резистор 10K (permanently 0)
// Если нужно перепрошить через UART — перемычка BOOT0 на 3.3V

// Проверка в коде (опционально):
// BOOT0 sampled at reset, cannot be read in software
// Use hardware pull-down resistor on BOOT0 pin
```

**Как отладить:**
- Симптом: Прошивка не стартует после power-on
- Симптом: J-Link может прошить, но код не выполняется
- Проверка: Осциллограф на BOOT0 pin при reset
- Проверка: Мультиметр — BOOT0 должен быть 0V (Flash boot)

**Ссылки:**
- RM0090 Chapter 3: Embedded Flash memory interface (Section 3.4 Boot configuration)
- Datasheet DM00037053: Pin descriptions (BOOT0, BOOT1)
- Application Note AN2606: STM32 microcontroller system memory boot mode

---

## 6. Flash write protection (ROP)

**Проблема:**
Flash может быть защищён от записи (ROP — Read-Out Protection).
Если включён — J-Link не может ни читать, ни писать.

**Детали:**
- ROP Level 0: No protection (default)
- ROP Level 1: Read protection enabled (can write, cannot read)
- ROP Level 2: Full protection (cannot read or write, permanent)
- ROP активируется через Option Bytes (`FLASH_OPTCR`)
- Level 2 — необратим (чип заблокирован навсегда)

**Решение:**
```bash
# Через J-Link Commander:
JLink.exe
> connect
> Device = STM32F407ZET6
> Interface = SWD
> Speed = 4000

# Проверка ROP level:
> mem 0x1FFFC000 4  # Option bytes
# Если значение != 0x0FFFAA55 — ROP enabled

# Снятие ROP (стирает весь Flash!):
> unlock
# Подтвердите удаление (Yes)

# После unlock — чип полностью стёрт
# Нужно заново прошить firmware
```

**Как отладить:**
- Симптом: J-Link: "Cannot connect to target"
- Симптом: J-Link: "Flash download failed"
- Симптом: OpenOCD: "target not halted"
- Проверка: `FLASH->OPTCR` — регистр Option bytes
- Проверка: J-Link Commander: `mem 0x1FFFC000`

**Внимание:** 
- `unlock` стирает весь Flash! Делайте backup.
- ROP Level 2 — необратим. НЕ УСТАНАВЛИВАЙТЕ Level 2.
- ROP Level 1 — обратим, но стирает Flash.

**Ссылки:**
- RM0090 Chapter 3: Embedded Flash memory interface (Section 3.7 Option bytes)
- Application Note AN3969: STM32 Flash programming

---

## Дополнительные ресурсы

**Официальные документы ST:**
- RM0090: STM32F4xx Reference Manual (уже в `kb/`)
- DM00037053: STM32F407 Datasheet
- ES0113: STM32F407 Errata Sheet
- AN2606: System memory boot mode
- AN3969: Flash programming

**Сообщество:**
- STM32CubeIDE forums: https://community.st.com/s/topic/0TO0X000000Bs9OWAS/stm32cubeide
- EEVblog STM32 section: https://www.eevblog.com/forum/microcontrollers/
- Stack Overflow [stm32] tag: https://stackoverflow.com/questions/tagged/stm32

---

**Последнее обновление:** 2026-10-14  
**Автор:** H.A.L.O. project  
**Статус:** ✅ Проверено на практике

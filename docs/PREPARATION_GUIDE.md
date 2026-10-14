# Руководство по подготовке MD файлов для индексации

## 📋 Общие рекомендации

После конвертации PDF через Marker, файлы требуют дополнительной обработки для качественной индексации в Qdrant.

---

## 1. **Разбивка большого файла на главы**

Marker создаёт один большой файл (~2000 страниц). Для RAG нужно разбить на логические части.

### Скрипт `scripts/split_and_clean.py`

```python
import re
import os
from pathlib import Path
import frontmatter

def split_by_chapters(md_file, output_dir):
    """Разбивает большой MD файл на главы"""
    content = md_file.read_text(encoding='utf-8')
    
    # Ищем заголовки глав (## 1. Introduction, ## 2. Memory, etc.)
    chapters = re.split(r'\n(?=## \d+\.)', content)
    
    for i, chapter in enumerate(chapters):
        if len(chapter.strip()) < 100:
            continue
        
        # Извлекаем номер и название главы
        match = re.match(r'## (\d+)\.\s+(.+)', chapter)
        if match:
            chapter_num = match.group(1)
            chapter_title = match.group(2).strip()
            
            # Создаём отдельный файл
            filename = f"{chapter_num.zfill(2)}_{chapter_title.lower().replace(' ', '_')}.md"
            output_path = output_dir / filename
            
            # Добавляем YAML frontmatter
            post = frontmatter.Post(chapter)
            post.metadata = {
                'mcu': 'STM32F407',
                'chapter': int(chapter_num),
                'title': chapter_title,
                'source': md_file.name,
                'schema_version': 1
            }
            
            output_path.write_text(frontmatter.dumps(post), encoding='utf-8')
            print(f"Created: {filename}")

def clean_markdown(md_file):
    """Очистка артефактов распознавания"""
    content = md_file.read_text(encoding='utf-8')
    
    # Удаление пустых таблиц
    content = re.sub(r'\|[\s\|]*\|\n\|[-\|\s]*\|\n\|[\s\|]*\|', '', content)
    
    # Исправление разорванных ссылок
    content = re.sub(r'\[([^\]]+)\]\n\s*\(([^\)]+)\]', r'[\1](\2)', content)
    
    # Удаление дублирующихся заголовков
    lines = content.split('\n')
    seen_headers = set()
    cleaned_lines = []
    for line in lines:
        if line.startswith('#'):
            if line in seen_headers:
                continue
            seen_headers.add(line)
        cleaned_lines.append(line)
    
    return '\n'.join(cleaned_lines)

def add_peripheral_metadata(md_file):
    """Добавляет метаданные о периферии на основе содержимого"""
    content = md_file.read_text(encoding='utf-8')
    
    # Определяем периферию по ключевым словам
    peripherals = {
        'GPIO': r'\bGPIO[A-Z]\b|GPIO_',
        'TIM': r'\bTIM\d+\b|timer',
        'UART': r'\bUART\d+\b|USART\d+\b',
        'SPI': r'\bSPI\d+\b',
        'I2C': r'\bI2C\d+\b',
        'ADC': r'\bADC\d*\b',
        'DMA': r'\bDMA\d*\b|DMA_',
        'CAN': r'\bCAN\d+\b',
        'USB': r'\bUSB_OTG\b|USB_',
        'RCC': r'\bRCC\b|clock',
        'NVIC': r'\bNVIC\b|interrupt',
        'FLASH': r'\bFLASH\b|flash memory',
        'RTC': r'\bRTC\b',
        'WWDG': r'\bWWDG\b',
        'IWDG': r'\bIWDG\b',
    }
    
    detected = []
    for periph, pattern in peripherals.items():
        if re.search(pattern, content, re.IGNORECASE):
            detected.append(periph)
    
    return detected

if __name__ == "__main__":
    input_file = Path("docs/reference_manual/STM32F4xx Reference Manual.md")
    output_dir = Path("docs/reference_manual/chapters")
    output_dir.mkdir(parents=True, exist_ok=True)
    
    print(f"Splitting {input_file}...")
    split_by_chapters(input_file, output_dir)
    
    print("\nCleaning and adding metadata...")
    for md_file in output_dir.glob("*.md"):
        # Очистка
        cleaned = clean_markdown(md_file)
        md_file.write_text(cleaned, encoding='utf-8')
        
        # Добавление метаданных о периферии
        post = frontmatter.load(md_file)
        peripherals = add_peripheral_metadata(md_file)
        if peripherals:
            post.metadata['peripherals'] = peripherals
            md_file.write_text(frontmatter.dumps(post), encoding='utf-8')
        
        print(f"Processed: {md_file.name} (peripherals: {', '.join(peripherals)})")
```

---

## 2. **Шаблоны YAML frontmatter для ключевых периферий**

### GPIO
```markdown
---
mcu: STM32F407
peripheral: GPIO
chapter: 8
registers:
  - MODER
  - OTYPER
  - OSPEEDR
  - PUPDR
  - IDR
  - ODR
  - BSRR
  - LCKR
  - AFRL
  - AFRH
tags: [gpio, digital-io, pin-config]
---
```

### TIM (Timers)
```markdown
---
mcu: STM32F407
peripheral: TIM
chapter: 17
registers:
  - CR1
  - CR2
  - SMCR
  - DIER
  - SR
  - EGR
  - CCMR1
  - CCMR2
  - CCER
  - CNT
  - PSC
  - ARR
  - CCR1
  - CCR2
  - CCR3
  - CCR4
  - BDTR  # Для advanced timers (TIM1, TIM8)
tags: [timer, pwm, capture, compare, dead-time]
---
```

### DMA
```markdown
---
mcu: STM32F407
peripheral: DMA
chapter: 9
registers:
  - LISR
  - HISR
  - LIFCR
  - HIFCR
  - SxCR
  - SxNDTR
  - SxPAR
  - SxM0AR
  - SxM1AR
  - SxFCR
tags: [dma, stream, channel, transfer, circular]
critical_notes:
  - "DMA buffers CANNOT be in CCM RAM!"
  - "Stream/Channel mapping is fixed (see Table 23)"
---
```

### RCC (Clock system)
```markdown
---
mcu: STM32F407
peripheral: RCC
chapter: 6
registers:
  - CR
  - PLLCFGR
  - CFGR
  - CIR
  - AHB1RSTR
  - AHB2RSTR
  - AHB3RSTR
  - APB1RSTR
  - APB2RSTR
  - AHB1ENR
  - AHB2ENR
  - AHB3ENR
  - APB1ENR
  - APB2ENR
tags: [clock, pll, hse, hsi, peripheral-clock]
---
```

### NVIC (Interrupts)
```markdown
---
mcu: STM32F407
peripheral: NVIC
chapter: 10
registers:
  - ISER
  - ICER
  - ISPR
  - ICPR
  - IABR
  - IPR
tags: [interrupt, nvic, priority, preemption]
critical_notes:
  - "Priority Grouping usually = 4 (4 bits preemption, 0 sub)"
---
```

---

## 3. **Известные проблемы F407 (обязательно вручную!)**

Файл `docs/hardware_pitfalls/f407_known_issues.md` нужно написать **вручную** — это критически важная информация.

### Структура файла:

```markdown
---
mcu: STM32F407
type: known_issues
tags: [errata, pitfalls, workarounds]
schema_version: 1
---

# STM32F407 Known Issues & Hardware Pitfalls

## 1. CCM RAM + DMA — НЕСОВМЕСТИМО!

**Проблема:**
DMA контроллер **не имеет доступа** к CCM RAM (Core Coupled Memory).
Если разместить буфер в `.ccmram`, DMA зависнет или будет читать мусор.

**Решение:**
Размещайте DMA-буферы только в SRAM1 (64KB) или SRAM2 (16KB).

```c
// ПЛОХО (не работает!)
uint8_t dma_buffer[1024] __attribute__((section(".ccmram")));

// ХОРОШО
uint8_t dma_buffer[1024] __attribute__((section(".sram1")));
// Или просто без атрибута (по умолчанию в SRAM)
```

**Как отладить:**
- Симптом: DMA transfer complete flag не устанавливается
- Симптом: В буфере мусор после DMA transfer
- Проверка: `MAP` файл — смотрите, где размещён буфер

---

## 2. NVIC Priority Grouping

**Проблема:**
CubeMX по умолчанию устанавливает `NVIC_PRIORITYGROUP_4` (4 бита preemption, 0 бит subpriority).
Если вы вручную меняете grouping, могут конфликтовать прерывания.

**Решение:**
```c
// Оставляйте как в CubeMX (обычно в main.c, USER CODE BEGIN 1)
HAL_NVIC_SetPriorityGrouping(NVIC_PRIORITYGROUP_4);

// Приоритеты: 0 (highest) - 15 (lowest)
HAL_NVIC_SetPriority(USART1_IRQn, 5, 0);  // Preemption = 5
```

---

## 3. USB/SDIO требуют 48 MHz

**Проблема:**
USB OTG и SDIO требуют тактовую частоту ровно 48 MHz.
Если PLL настроен неправильно — USB не работает.

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
```

---

## 4. Таймеры APB1/APB2 умножают частоту

**Проблема:**
Если APB prescaler != 1, частота таймера умножается на 2 (APB1) или 4 (APB2).

**Пример:**
```
SYSCLK = 168 MHz
AHB = 168 MHz (prescaler = 1)
APB1 = 42 MHz (prescaler = 4) -> TIM clock = 84 MHz (x2)
APB2 = 84 MHz (prescaler = 2) -> TIM clock = 168 MHz (x2)
```

**Решение:**
```c
// При расчёте prescaler для TIM:
// TIM_CLK = 84 MHz (для APB1 таймеров)
// Desired frequency = 1 kHz
// ARR = 999 (0..999 = 1000 отсчётов)
// PSC = TIM_CLK / (Desired * (ARR+1)) - 1
// PSC = 84000000 / (1000 * 1000) - 1 = 83

__HAL_TIM_SET_PRESCALER(&htim1, 83);  // Для 1 kHz
```

---

## 5. Boot configuration

**Проблема:**
STM32F407 может загружаться из Flash, System Memory (bootloader), или SRAM.
Если BOOT0/BOOT1 неправильно настроены — прошивка не стартует.

**Решение:**
```
BOOT0 = 0, BOOT1 = X  -> Boot from Flash (нормальный режим)
BOOT0 = 1, BOOT1 = 0  -> Boot from System Memory (UART bootloader)
BOOT0 = 1, BOOT1 = 1  -> Boot from SRAM (для отладки)
```

**На плате:**
- BOOT0 обычно подтянут к GND (permanently 0)
- Если нужно перепрошить через UART — перемычка на 3.3V

---

## 6. Flash write protection

**Проблема:**
Flash может быть защищён от записи (ROP — Read-Out Protection).
Если включён — J-Link не может ни читать, ни писать.

**Решение:**
```bash
# Через J-Link Commander:
JLink.exe
> connect
> Device = STM32F407ZET6
> Interface = SWD
> Speed = 4000
> unlock  # Снимает ROP (стирает весь Flash!)
```

**Внимание:** `unlock` стирает весь Flash! Делайте backup.
```

---

## 4. **Оптимальная стратегия чанкинга**

### Параметры:
- **Размер чанка:** ~800 символов
- **Минимальный размер:** 50 символов
- **Максимальный размер:** 1500 символов
- **Разбиение:** По заголовкам `##`

### Пример чанка:
```markdown
## TIM1_BDTR — Break and dead-time register

### Bits 15:8 — DTG[7:0]: Dead-time generator setup
- `0x00-0x7F`: DT = DTG[7:0] × t_DTS
- `0x80-0xBF`: DT = (64 + DTG[5:0]) × t_DTS
- `0xC0-0xDF`: DT = (32 + DTG[4:0]) × 2 × t_DTS
- `0xE0-0xFF`: DT = (32 + DTG[4:0]) × 8 × t_DTS

### Bit 15 — MOE: Main output enable
- `0`: Outputs disabled
- `1`: Outputs enabled

### Примечания
- t_DTS зависит от CKD в TIMx_CR1
- MOE сбрасывается аппаратно при Break event
```

---

## 5. **Порядок действий**

1. ✅ **Запустить Marker** (конвертация PDF → MD)
2. ✅ **Запустить `split_and_clean.py`** — разбивка на главы + очистка
3. ✅ **Вручную обработать ключевые разделы** (GPIO, TIM, DMA, RCC)
4. ✅ **Написать `f407_known_issues.md`** (по шаблону выше)
5. ✅ **Создать 3-5 тестовых MD файлов** с полным frontmatter
6. ✅ **Проверить качество** (открыть 10 случайных файлов)
7. ✅ **Запустить `index_docs.py`** — индексация в Qdrant

---

## 6. **Критерии качества**

### Хороший MD файл:
- ✅ Есть YAML frontmatter с метаданными
- ✅ Заголовки структурированы (`##`, `###`)
- ✅ Таблицы регистров не разорваны
- ✅ Код в блоках ```c ... ```
- ✅ Нет артефактов распознавания (мусорные символы)

### Плохой MD файл:
- ❌ Нет frontmatter
- ❌ Таблицы разорваны на несколько строк
- ❌ Заголовки дублируются
- ❌ Ссылки разорваны `[text]\n(url)`
- ❌ Мусорные символы (â, ï, etc.)

---

## 7. **Автоматизация проверки качества**

Скрипт `scripts/validate_md.py`:

```python
from pathlib import Path
import frontmatter
import re

def validate_md_file(md_file):
    """Проверка качества MD файла"""
    issues = []
    
    # Загрузка файла
    try:
        post = frontmatter.load(md_file)
    except Exception as e:
        return [f"Failed to parse frontmatter: {e}"]
    
    # Проверка frontmatter
    if 'mcu' not in post.metadata:
        issues.append("Missing 'mcu' in frontmatter")
    if 'peripheral' not in post.metadata:
        issues.append("Missing 'peripheral' in frontmatter")
    
    # Проверка заголовков
    content = post.content
    if not re.search(r'^##\s+', content, re.MULTILINE):
        issues.append("No ## headers found")
    
    # Проверка таблиц
    if '|' in content:
        # Есть таблицы — проверяем, что они не разорваны
        lines = content.split('\n')
        for i, line in enumerate(lines):
            if line.startswith('|') and i > 0:
                prev_line = lines[i-1]
                if not prev_line.strip():
                    issues.append(f"Table broken at line {i}")
    
    # Проверка мусорных символов
    if re.search(r'[âïöü]', content):
        issues.append("Garbage characters detected (encoding issue)")
    
    return issues

if __name__ == "__main__":
    md_dir = Path("docs/reference_manual/chapters")
    
    total_files = 0
    files_with_issues = 0
    
    for md_file in md_dir.glob("*.md"):
        total_files += 1
        issues = validate_md_file(md_file)
        
        if issues:
            files_with_issues += 1
            print(f"\n{md_file.name}:")
            for issue in issues:
                print(f"  - {issue}")
    
    print(f"\n\nSummary:")
    print(f"Total files: {total_files}")
    print(f"Files with issues: {files_with_issues}")
    print(f"Quality: {100 * (total_files - files_with_issues) / total_files:.1f}%")
```

---

## 8. **Ссылки**

- **TASK.md** — полный план архитектуры (Фаза 0, Шаг 0.1-0.3)
- **BOARD.md** — рабочая доска для текущих задач
- **index_docs.py** — скрипт индексации в Qdrant (из TASK.md)

---

## 9. **Дополнительные рекомендации**

### Если Marker дал плохое качество:
1. Попробуйте **Nougat** (от Meta) — лучше для академических документов
2. Используйте **GPT-4/Claude** для пост-обработки таблиц
3. Обработайте ключевые разделы вручную (GPIO, TIM, DMA)

### Если таблицы всё ещё разорваны:
1. Используйте скрипт для восстановления таблиц
2. Или замените Markdown-таблицы на HTML-таблицы
3. Или используйте списки вместо таблиц

### Если нужно улучшить поиск:
1. Добавьте больше тегов в frontmatter
2. Добавьте `keywords` в frontmatter
3. Используйте hybrid search (vector + BM25) в Qdrant

---

**Готово к использованию!** Следуйте порядку действий для качественной подготовки базы знаний.

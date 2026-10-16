#!/usr/bin/env python3
"""
Обогащает manifest.json дополнительными полями:
- peripheral: тип периферии (GPIO, TIM, DMA, UART, SPI, I2C, ADC, CAN, USB, etc.)
- registers: список регистров, описываемых в файле
- tags: ключевые слова для улучшения поиска

Анализ на основе:
1. Имени файла (например, "08_8_3_GPIO_functional_description.md" → peripheral: GPIO)
2. Содержимого файла (поиск названий регистров, ключевых слов)
3. Номера главы (маппинг глав → периферия)
"""

import json
import re
from pathlib import Path

KB_DIR = Path("kb/stm32f4xx/reference")
MANIFEST_PATH = KB_DIR / "manifest.json"

# Маппинг глав → периферия (на основе RM0090)
CHAPTER_TO_PERIPHERAL = {
    1: None,  # Documentation conventions
    2: None,  # Memory and bus architecture
    3: "FLASH",  # Embedded Flash memory interface
    4: "CRC",  # CRC calculation unit
    5: "PWR",  # Power controller
    6: "RCC",  # Reset and clock control (F42x/43x variant)
    7: "RCC",  # Reset and clock control (F407)
    8: "GPIO",  # General-purpose I/Os
    9: "SYSCFG",  # System configuration controller
    10: "DMA",  # DMA controller
    11: "DMA2D",  # Chrom-Art DMA2D (not on F407)
    12: "EXTI",  # EXTI / interrupts
    13: "ADC",  # Analog-to-digital converter
    14: "DAC",  # Digital-to-analog converter
    15: "DCMI",  # Digital camera (not on F407)
    16: "LTDC",  # LCD controller (not on F407)
    17: "TIM",  # Advanced timers TIM1/TIM8
    18: "TIM",  # General-purpose timers TIM2-TIM5
    19: "TIM",  # General-purpose timers TIM9-TIM14
    20: "TIM",  # Basic timers TIM6/TIM7
    21: "IWDG",  # Independent watchdog
    22: "WWDG",  # Window watchdog
    23: "CRYP",  # Crypto processor
    24: "RNG",  # Random number generator
    25: "HASH",  # HASH processor
    26: "RTC",  # Real-time clock
    27: "I2C",  # Inter-integrated circuit
    28: "SPI",  # Serial peripheral interface
    29: "SAI",  # Serial audio interface (not on F407)
    30: "USART",  # Universal synchronous/asynchronous receiver
    31: "SDIO",  # SD card interface
    32: "CAN",  # bxCAN
    33: "ETH",  # Ethernet MAC
    34: "USB",  # USB OTG full-speed
    35: "USB",  # USB OTG high-speed
    36: "FSMC",  # Flexible static memory controller
    37: "FMC",  # Flexible memory controller (not on F407)
    38: "DBG",  # Debug support
    39: "UID",  # Device electronic signature
}

# Паттерны для поиска регистров в содержимом
REGISTER_PATTERNS = {
    "GPIO": [
        r"GPIO[x]?_MODER", r"GPIO[x]?_OTYPER", r"GPIO[x]?_OSPEEDR", r"GPIO[x]?_PUPDR",
        r"GPIO[x]?_IDR", r"GPIO[x]?_ODR", r"GPIO[x]?_BSRR", r"GPIO[x]?_LCKR",
        r"GPIO[x]?_AFR[LH]?"
    ],
    "TIM": [
        r"TIM[x]?_CR[12]", r"TIM[x]?_SMCR", r"TIM[x]?_DIER", r"TIM[x]?_SR",
        r"TIM[x]?_EGR", r"TIM[x]?_CCM[R12]", r"TIM[x]?_CCER", r"TIM[x]?_CNT",
        r"TIM[x]?_PSC", r"TIM[x]?_ARR", r"TIM[x]?_CCR[1-4]", r"TIM[x]?_BDTR",
        r"TIM[x]?_DMAR"
    ],
    "DMA": [
        r"DMA[x]?_LISR", r"DMA[x]?_HISR", r"DMA[x]?_LIFCR", r"DMA[x]?_HIFCR",
        r"DMA[x]?_S[0-7]CR", r"DMA[x]?_S[0-7]NDTR", r"DMA[x]?_S[0-7]PAR",
        r"DMA[x]?_S[0-7]M[01]AR", r"DMA[x]?_S[0-7]FCR"
    ],
    "RCC": [
        r"RCC_CR", r"RCC_PLLCFGR", r"RCC_CFGR", r"RCC_CIR", r"RCC_AHB[123]RSTR",
        r"RCC_APB[12]RSTR", r"RCC_AHB[123]ENR", r"RCC_APB[12]ENR",
        r"RCC_AHB[123]LPENR", r"RCC_APB[12]LPENR", r"RCC_BDCR", r"RCC_CSR"
    ],
    "USART": [
        r"USART[x]?_SR", r"USART[x]?_DR", r"USART[x]?_BRR", r"USART[x]?_CR[123]",
        r"USART[x]?_GTPR"
    ],
    "SPI": [
        r"SPI[x]?_CR[12]", r"SPI[x]?_SR", r"SPI[x]?_DR", r"SPI[x]?_CRCPR",
        r"SPI[x]?_RXCRCR", r"SPI[x]?_TXCRCR"
    ],
    "I2C": [
        r"I2C[x]?_CR[12]", r"I2C[x]?_OAR[12]", r"I2C[x]?_DR", r"I2C[x]?_SR[12]",
        r"I2C[x]?_CCR", r"I2C[x]?_TRISE"
    ],
    "ADC": [
        r"ADC_CR[12]", r"ADC_SR", r"ADC_DR", r"ADC_JDR[1-4]", r"ADC_SMPR[12]",
        r"ADC_SQR[1-3]", r"ADC_JSQR"
    ],
    "CAN": [
        r"CAN_MCR", r"CAN_MSR", r"CAN_TSR", r"CAN_RF[01]R", r"CAN_IER", r"CAN_ESR",
        r"CAN_BTR", r"CAN_TI[0-2]R", r"CAN_TDT[0-2]R", r"CAN_TDLR", r"CAN_TDHR"
    ],
    "USB": [
        r"OTG_FS_G[A-Z]+", r"OTG_HS_G[A-Z]+", r"USB_OTG_"
    ],
    "RTC": [
        r"RTC_TR", r"RTC_DR", r"RTC_CR", r"RTC_ISR", r"RTC_PRER", r"RTC_WUTR",
        r"RTC_ALRM[AB]R"
    ],
    "FLASH": [
        r"FLASH_ACR", r"FLASH_KEYR", r"FLASH_OPTKEYR", r"FLASH_SR", r"FLASH_CR",
        r"FLASH_OPTCR"
    ],
    "PWR": [
        r"PWR_CR", r"PWR_CSR"
    ],
    "DAC": [
        r"DAC_CR", r"DAC_SWTRIGR", r"DAC_DHR[12]2R", r"DAC_DHR12RD", r"DAC_DOR[12]R"
    ],
    "WWDG": [
        r"WWDG_CR", r"WWDG_CFR", r"WWDG_SR"
    ],
    "IWDG": [
        r"IWDG_KR", r"IWDG_PR", r"IWDG_RLR"
    ],
    "EXTI": [
        r"EXTI_IMR", r"EXTI_EMR", r"EXTI_RTSR", r"EXTI_FTSR", r"EXTI_SWIER",
        r"EXTI_PR"
    ],
    "SYSCFG": [
        r"SYSCFG_MEMRMP", r"SYSCFG_PMC", r"SYSCFG_EXTICR[1-4]"
    ],
}

# Теги для периферии
PERIPHERAL_TAGS = {
    "GPIO": ["gpio", "digital-io", "pin-config", "alternate-function"],
    "TIM": ["timer", "pwm", "capture", "compare", "encoder", "dead-time", "break"],
    "DMA": ["dma", "stream", "channel", "transfer", "circular", "fifo"],
    "RCC": ["clock", "pll", "hse", "hsi", "prescaler", "peripheral-clock"],
    "USART": ["uart", "serial", "baud-rate", "interrupt", "dma"],
    "SPI": ["spi", "serial", "master", "slave", "full-duplex"],
    "I2C": ["i2c", "i2c-bus", "master", "slave", "address"],
    "ADC": ["adc", "analog", "conversion", "sampling", "channel"],
    "DAC": ["dac", "analog", "output", "voltage"],
    "CAN": ["can", "can-bus", "message", "filter", "interrupt"],
    "USB": ["usb", "otg", "device", "host", "endpoint"],
    "RTC": ["rtc", "calendar", "alarm", "wakeup", "backup"],
    "FLASH": ["flash", "memory", "erase", "program", "option-bytes"],
    "PWR": ["power", "low-power", "sleep", "stop", "standby"],
    "WWDG": ["watchdog", "wwdg", "timeout"],
    "IWDG": ["watchdog", "iwdg", "timeout"],
    "EXTI": ["interrupt", "exti", "event", "edge"],
    "SYSCFG": ["syscfg", "multiplexer", "boot"],
    "CRC": ["crc", "checksum"],
    "RNG": ["random", "rng"],
    "CRYP": ["crypto", "aes", "des"],
    "HASH": ["hash", "sha", "md5"],
    "SDIO": ["sd-card", "mmc", "sdio"],
    "ETH": ["ethernet", "mac", "phy"],
    "FSMC": ["fsmc", "external-memory", "sram", "nand"],
    "FMC": ["fmc", "external-memory", "sdram"],
    "DBG": ["debug", "dbgmcu", "trace"],
    "UID": ["uid", "signature", "flash-size"],
    "DMA2D": ["dma2d", "chrom-art", "graphics"],
    "DCMI": ["camera", "dcmi"],
    "LTDC": ["lcd", "display", "tft"],
    "SAI": ["audio", "sai", "i2s"],
}


def extract_peripheral(file_info: dict, content: str) -> str | None:
    """Определяет периферию на основе главы и содержимого"""
    chapter = file_info.get("chapter", 0)
    
    # Сначала пробуем по главе
    peripheral = CHAPTER_TO_PERIPHERAL.get(chapter)
    if peripheral:
        return peripheral
    
    # Если не найдено, ищем в содержимом
    for periph, patterns in REGISTER_PATTERNS.items():
        for pattern in patterns:
            if re.search(pattern, content, re.IGNORECASE):
                return periph
    
    return None


def extract_registers(file_info: dict, content: str, peripheral: str | None) -> list[str]:
    """Извлекает список регистров из содержимого"""
    if not peripheral:
        return []
    
    registers = set()
    patterns = REGISTER_PATTERNS.get(peripheral, [])
    
    for pattern in patterns:
        matches = re.findall(pattern, content, re.IGNORECASE)
        for match in matches:
            # Нормализация имени регистра
            reg_name = match.upper().replace("[X]", "").replace("[x]", "")
            registers.add(reg_name)
    
    return sorted(list(registers))


def extract_tags(peripheral: str | None) -> list[str]:
    """Извлекает теги на основе периферии"""
    if not peripheral:
        return []
    
    return PERIPHERAL_TAGS.get(peripheral, [])


def enrich_manifest():
    """Основная функция обогащения manifest.json"""
    print(f"Loading manifest from {MANIFEST_PATH}...")
    manifest = json.loads(MANIFEST_PATH.read_text(encoding="utf-8"))
    
    files = manifest.get("files", [])
    print(f"Found {len(files)} files")
    
    enriched_count = 0
    
    for i, file_info in enumerate(files):
        file_name = file_info.get("file", "")
        file_path = KB_DIR / file_name
        
        print(f"\r[{i+1}/{len(files)}] Processing {file_name}...", end="")
        
        # Чтение содержимого
        if file_path.exists():
            content = file_path.read_text(encoding="utf-8")
        else:
            content = ""
        
        # Извлечение периферии
        peripheral = extract_peripheral(file_info, content)
        if peripheral:
            file_info["peripheral"] = peripheral
            enriched_count += 1
        
        # Извлечение регистров
        registers = extract_registers(file_info, content, peripheral)
        if registers:
            file_info["registers"] = registers
        
        # Извлечение тегов
        tags = extract_tags(peripheral)
        if tags:
            file_info["tags"] = tags
    
    print(f"\n\nEnriched {enriched_count} files with peripheral information")
    
    # Сохранение
    backup_path = KB_DIR / "manifest.backup.json"
    print(f"Creating backup at {backup_path}...")
    backup_path.write_text(json.dumps(manifest, indent=1, ensure_ascii=False), encoding="utf-8")
    
    print(f"Saving enriched manifest to {MANIFEST_PATH}...")
    MANIFEST_PATH.write_text(json.dumps(manifest, indent=1, ensure_ascii=False), encoding="utf-8")
    
    print("\n✅ Done!")
    
    # Статистика
    print("\n📊 Statistics:")
    peripheral_counts = {}
    for file_info in files:
        periph = file_info.get("peripheral")
        if periph:
            peripheral_counts[periph] = peripheral_counts.get(periph, 0) + 1
    
    for periph, count in sorted(peripheral_counts.items(), key=lambda x: -x[1]):
        print(f"  {periph}: {count} files")


if __name__ == "__main__":
    enrich_manifest()

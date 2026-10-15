package cubemx

import (
	"os"
	"path/filepath"
	"regexp"
	"strings"
)

// WeakOverride — информация о переопределённой weak функции
type WeakOverride struct {
	Function string `json:"function"`
	File     string `json:"file"`
	Line     int    `json:"line"`
}

// Finder — поиск переопределённых weak функций
type Finder struct{}

// NewFinder создаёт новый finder
func NewFinder() *Finder {
	return &Finder{}
}

// FindOverrides ищет переопределения weak функций HAL в проекте
func (f *Finder) FindOverrides(projectRoot string) ([]WeakOverride, error) {
	var overrides []WeakOverride

	// Список известных HAL weak функций, которые обычно переопределяют
	halWeakFuncs := map[string]bool{
		// GPIO
		"HAL_GPIO_MspInit":   true,
		"HAL_GPIO_MspDeInit": true,
		// TIM
		"HAL_TIM_Base_MspInit":      true,
		"HAL_TIM_Base_MspDeInit":    true,
		"HAL_TIM_IC_MspInit":        true,
		"HAL_TIM_IC_MspDeInit":      true,
		"HAL_TIM_PWM_MspInit":       true,
		"HAL_TIM_PWM_MspDeInit":     true,
		"HAL_TIM_Encoder_MspInit":   true,
		"HAL_TIM_Encoder_MspDeInit": true,
		// Callbacks
		"HAL_TIM_PeriodElapsedCallback":     true,
		"HAL_TIM_IC_CaptureCallback":        true,
		"HAL_TIM_PWM_PulseFinishedCallback": true,
		"HAL_TIM_TriggerCallback":           true,
		"HAL_GPIO_EXTI_Callback":            true,
		"HAL_UART_RxCpltCallback":           true,
		"HAL_UART_TxCpltCallback":           true,
		"HAL_UART_ErrorCallback":            true,
		"HAL_SPI_RxCpltCallback":            true,
		"HAL_SPI_TxCpltCallback":            true,
		"HAL_I2C_MasterRxCpltCallback":      true,
		"HAL_I2C_MasterTxCpltCallback":      true,
		"HAL_ADC_ConvCpltCallback":          true,
		"HAL_CAN_RxFifo0MsgPendingCallback": true,
		// MSP
		"HAL_MspInit":   true,
		"HAL_MspDeInit": true,
		// UART
		"HAL_UART_MspInit":   true,
		"HAL_UART_MspDeInit": true,
		// SPI
		"HAL_SPI_MspInit":   true,
		"HAL_SPI_MspDeInit": true,
		// I2C
		"HAL_I2C_MspInit":   true,
		"HAL_I2C_MspDeInit": true,
		// ADC
		"HAL_ADC_MspInit":   true,
		"HAL_ADC_MspDeInit": true,
		// CAN
		"HAL_CAN_MspInit":   true,
		"HAL_CAN_MspDeInit": true,
	}

	// Паттерн для поиска определений функций
	// Ищем: void FunctionName(...) {
	funcPattern := regexp.MustCompile(`(?m)^void\s+(\w+)\s*\([^)]*\)\s*\{`)

	// Системные директории, которые нужно пропускать
	skipDirs := map[string]bool{
		"Drivers":     true,
		"Middlewares": true,
		".git":        true,
		"build":       true,
		"debug":       true,
		"release":     true,
	}

	err := filepath.Walk(projectRoot, func(path string, info os.FileInfo, err error) error {
		if err != nil {
			return nil
		}

		// Пропускаем системные директории
		if info.IsDir() {
			if skipDirs[info.Name()] {
				return filepath.SkipDir
			}
			return nil
		}

		// Парсим только .c файлы
		if !strings.HasSuffix(info.Name(), ".c") {
			return nil
		}

		// Чтение файла
		content, err := os.ReadFile(path)
		if err != nil {
			return nil
		}

		// Поиск определений функций
		matches := funcPattern.FindAllStringSubmatchIndex(string(content), -1)

		for _, match := range matches {
			if len(match) < 2 {
				continue
			}

			funcName := string(content[match[2]:match[3]])

			// Проверяем, является ли это HAL weak функцией
			if halWeakFuncs[funcName] {
				// Вычисляем номер строки
				line := strings.Count(string(content[:match[0]]), "\n") + 1

				overrides = append(overrides, WeakOverride{
					Function: funcName,
					File:     path,
					Line:     line,
				})
			}
		}

		return nil
	})

	if err != nil {
		return nil, err
	}

	return overrides, nil
}

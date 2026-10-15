package main

import (
	"context"
	"fmt"
	"log"
	"os"
	"regexp"
	"strings"

	"github.com/mdari/halo-f407-mcp/mcp-server/internal/config"
	"github.com/mdari/halo-f407-mcp/mcp-server/internal/rag/embedder"
	"github.com/mdari/halo-f407-mcp/mcp-server/internal/rag/qdrant"
	"github.com/mdari/halo-f407-mcp/mcp-server/internal/rag/searcher"
	"github.com/modelcontextprotocol/go-sdk/mcp"
)

// QueryDocsInput — входные параметры для query_hardware_docs
type QueryDocsInput struct {
	Query      string `json:"query" jsonschema:"the search query for hardware documentation"`
	Peripheral string `json:"peripheral,omitempty" jsonschema:"optional filter by peripheral type (GPIO, TIM, DMA, etc.)"`
	Limit      int    `json:"limit,omitempty" jsonschema:"optional maximum number of results (default: 5)"`
}

// QueryDocsOutput — результат поиска
type QueryDocsOutput struct {
	Query      string                  `json:"query"`
	Peripheral string                  `json:"peripheral,omitempty"`
	Count      int                     `json:"count"`
	Results    []searcher.SearchResult `json:"results"`
}

// CheckF407RulesInput — входные параметры для check_f407_rules
type CheckF407RulesInput struct {
	Code     string `json:"code" jsonschema:"the C code to check for STM32F407 hardware pitfalls"`
	FilePath string `json:"file_path,omitempty" jsonschema:"optional file path for context"`
	CheckAll bool   `json:"check_all,omitempty" jsonschema:"optional check all rules (default: true)"`
}

// Violation — нарушение правила
type Violation struct {
	Rule      string `json:"rule"`
	Severity  string `json:"severity"` // "critical", "warning", "info"
	Line      int    `json:"line"`
	Message   string `json:"message"`
	Fix       string `json:"fix"`
	Reference string `json:"reference"`
}

// CheckF407RulesOutput — результат проверки
type CheckF407RulesOutput struct {
	FilePath        string      `json:"file_path"`
	ViolationsCount int         `json:"violations_count"`
	Violations      []Violation `json:"violations"`
}

// queryDocsHandler — обработчик инструмента query_hardware_docs
func queryDocsHandler(ctx context.Context, req *mcp.CallToolRequest, input QueryDocsInput) (
	*mcp.CallToolResult,
	QueryDocsOutput,
	error,
) {
	// Получение searcher из контекста (или глобальная переменная)
	// Временно используем глобальную переменную
	if globalSearcher == nil {
		return nil, QueryDocsOutput{}, fmt.Errorf("searcher not initialized")
	}

	// Установка лимита по умолчанию
	if input.Limit <= 0 {
		input.Limit = 5
	}

	// Поиск
	results, err := globalSearcher.Search(ctx, input.Query, input.Peripheral, input.Limit)
	if err != nil {
		return nil, QueryDocsOutput{}, fmt.Errorf("search failed: %w", err)
	}

	output := QueryDocsOutput{
		Query:      input.Query,
		Peripheral: input.Peripheral,
		Count:      len(results),
		Results:    results,
	}

	return nil, output, nil
}

// checkF407RulesHandler — обработчик инструмента check_f407_rules
func checkF407RulesHandler(ctx context.Context, req *mcp.CallToolRequest, input CheckF407RulesInput) (
	*mcp.CallToolResult,
	CheckF407RulesOutput,
	error,
) {
	var violations []Violation

	// Проверка каждого правила
	if v := checkCCMDMA(input.Code); v != nil {
		violations = append(violations, *v)
	}
	if v := checkNVICPriority(input.Code); v != nil {
		violations = append(violations, *v)
	}
	if v := checkUSB48MHz(input.Code); v != nil {
		violations = append(violations, *v)
	}
	if v := checkFlashROP(input.Code); v != nil {
		violations = append(violations, *v)
	}

	output := CheckF407RulesOutput{
		FilePath:        input.FilePath,
		ViolationsCount: len(violations),
		Violations:      violations,
	}

	return nil, output, nil
}

// checkCCMDMA — проверка: DMA буфер в CCM RAM
func checkCCMDMA(code string) *Violation {
	// Ищем атрибуты секций CCM RAM
	ccmPattern := regexp.MustCompile(`__attribute__\s*\(\(\s*section\s*\(\s*["'].*ccm.*["']\s*\)\s*\)\)`)
	if ccmPattern.MatchString(code) {
		return &Violation{
			Rule:      "ccm_dma_incompatible",
			Severity:  "critical",
			Line:      1,
			Message:   "DMA buffer is placed in CCM RAM. DMA controller has NO access to CCM RAM! This will cause DMA transfer failures or data corruption.",
			Fix:       "Move buffer to SRAM1: use __attribute__((section(\".sram1\"))) or remove the section attribute entirely (default is SRAM1).",
			Reference: "kb/hardware_pitfalls/f407_known_issues.md#1-ccm-ram--dma--несовместимо",
		}
	}
	return nil
}

// checkNVICPriority — проверка: изменение NVIC priority grouping
func checkNVICPriority(code string) *Violation {
	// Ищем вызовы HAL_NVIC_SetPriorityGrouping с неправильным аргументом
	nvicPattern := regexp.MustCompile(`HAL_NVIC_SetPriorityGrouping\s*\(\s*NVIC_PRIORITYGROUP_(\d+)\s*\)`)
	matches := nvicPattern.FindAllStringSubmatch(code, -1)

	for _, match := range matches {
		if len(match) > 1 && match[1] != "4" {
			return &Violation{
				Rule:      "nvic_priority_grouping",
				Severity:  "warning",
				Line:      1,
				Message:   fmt.Sprintf("NVIC priority grouping changed from default GROUP_4 to GROUP_%s. This may cause interrupt conflicts if other code expects GROUP_4.", match[1]),
				Fix:       "Use HAL_NVIC_SetPriorityGrouping(NVIC_PRIORITYGROUP_4) or ensure all code uses the same grouping consistently.",
				Reference: "kb/hardware_pitfalls/f407_known_issues.md#2-nvic-priority-grouping",
			}
		}
	}
	return nil
}

// checkUSB48MHz — проверка: USB/SDIO требуют 48 MHz
func checkUSB48MHz(code string) *Violation {
	// Ищем инициализацию USB без явной настройки PLL48CLK
	usbInitPattern := regexp.MustCompile(`HAL_PCD_Init\s*\(|HAL_HCD_Init\s*\(`)

	if usbInitPattern.MatchString(code) {
		// Проверяем, есть ли настройка PLL48CLK
		if !strings.Contains(code, "PLL48CLK") && !strings.Contains(code, "PLLQ") {
			return &Violation{
				Rule:      "usb_48mhz_required",
				Severity:  "critical",
				Line:      1,
				Message:   "USB OTG initialized but PLL48CLK configuration not found. USB OTG requires exactly 48 MHz clock. Verify RCC PLL configuration.",
				Fix:       "Ensure PLL48CLK = (HSE * PLLN / PLLM) / PLLQ = 48 MHz. Check CubeMX Clock Configuration tab.",
				Reference: "kb/hardware_pitfalls/f407_known_issues.md#3-usbsdio-требуют-48-mhz",
			}
		}
	}
	return nil
}

// checkFlashROP — проверка: Flash write protection (ROP Level 2)
func checkFlashROP(code string) *Violation {
	// Ищем попытки установки ROP Level 2
	ropPattern := regexp.MustCompile(`OB_RDP_LEVEL_2|FLASH_OB_RDP_LEVEL_2`)

	if ropPattern.MatchString(code) {
		return &Violation{
			Rule:      "flash_rop_level2",
			Severity:  "critical",
			Line:      1,
			Message:   "ROP Level 2 is being set. This is IRREVERSIBLE and will permanently lock the chip! Debug access will be disabled forever.",
			Fix:       "Use ROP Level 1 (reversible, erases Flash) or remove ROP configuration entirely.",
			Reference: "kb/hardware_pitfalls/f407_known_issues.md#6-flash-write-protection-rop",
		}
	}
	return nil
}

// Глобальная переменная для searcher (будет инициализирована в main)
var globalSearcher *searcher.Searcher

func main() {
	// Загрузка конфигурации
	cfg := config.Load()

	// Логирование в stderr (не мешает STDIO транспорту)
	log.SetOutput(os.Stderr)
	log.Printf("Starting STM32F407 MCP Server with official MCP SDK v1.8.0...")
	log.Printf("Qdrant: %s", cfg.QdrantURL)
	log.Printf("Ollama: %s", cfg.OllamaURL)
	log.Printf("Collection: %s", cfg.Collection)

	// Инициализация компонентов
	qdrantClient := qdrant.NewClient(cfg.QdrantURL)
	embedderClient := embedder.NewClient(cfg.OllamaURL, cfg.Model)
	globalSearcher = searcher.NewSearcher(qdrantClient, embedderClient, cfg.Collection)

	// Создание MCP сервера
	server := mcp.NewServer(
		&mcp.Implementation{
			Name:    "stm32f407-mcp",
			Version: "0.2.0", // Обновлена версия (добавлен check_f407_rules)
		},
		nil, // options
	)

	// Регистрация инструмента query_hardware_docs
	mcp.AddTool(
		server,
		&mcp.Tool{
			Name:        "query_hardware_docs",
			Description: "Search STM32F407 hardware documentation (RM0090) for registers, peripherals, clock configuration, etc. Returns relevant documentation chunks with metadata.",
		},
		queryDocsHandler,
	)

	// Регистрация инструмента check_f407_rules
	mcp.AddTool(
		server,
		&mcp.Tool{
			Name:        "check_f407_rules",
			Description: "Check C code for STM32F407 hardware pitfalls and critical errors. Validates: CCM RAM + DMA incompatibility, NVIC priority grouping, USB 48MHz requirement, Flash ROP Level 2. Returns list of violations with severity, fix suggestions, and references to documentation.",
		},
		checkF407RulesHandler,
	)

	log.Printf("Registered tools: query_hardware_docs, check_f407_rules")
	log.Printf("MCP Server ready, listening on stdin...")

	// Запуск STDIO транспорта
	if err := server.Run(context.Background(), &mcp.StdioTransport{}); err != nil {
		log.Fatalf("Server error: %v", err)
	}
}

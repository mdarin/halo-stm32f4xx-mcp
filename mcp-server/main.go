package main

import (
	"context"
	"fmt"
	"log"
	"os"
	"regexp"
	"strings"

	"github.com/mdari/halo-f407-mcp/mcp-server/internal/config"
	"github.com/mdari/halo-f407-mcp/mcp-server/internal/cubemx"
	"github.com/mdari/halo-f407-mcp/mcp-server/internal/toolkit"
	"github.com/modelcontextprotocol/go-sdk/mcp"
)

// ServerVersion — semver MCP-сервера (single source of truth).
// MINOR — добавление/изменение инструментов, PATCH — исправления.
const ServerVersion = "0.6.0"

// QueryDocsInput — входные параметры для query_hardware_docs
type QueryDocsInput struct {
	Query      string `json:"query" jsonschema:"the search query for hardware documentation"`
	Peripheral string `json:"peripheral,omitempty" jsonschema:"optional filter by peripheral type (GPIO, TIM, DMA, etc.)"`
	Limit      int    `json:"limit,omitempty" jsonschema:"optional maximum number of results (default: 5)"`
}

// QueryDocsOutput — результат поиска (через toolkit rag-query, Ф2-1)
type QueryDocsOutput struct {
	Query      string           `json:"query"`
	Peripheral string           `json:"peripheral,omitempty"`
	Count      int              `json:"count"`
	Results    []toolkit.DocHit `json:"results"`
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

// ReadUserBlocksInput — входные параметры для cubemx_read_user_blocks
type ReadUserBlocksInput struct {
	ProjectRoot string `json:"project_root,omitempty" jsonschema:"the root directory of the CubeMX project"`
	FilePath    string `json:"file_path,omitempty" jsonschema:"optional specific file to parse (if empty, parses entire project)"`
}

// ReadUserBlocksOutput — результат чтения USER CODE блоков
type ReadUserBlocksOutput struct {
	Files       []cubemx.ParseResult `json:"files"`
	TotalBlocks int                  `json:"total_blocks"`
	FilesCount  int                  `json:"files_count"`
}

// InsertCodeInput — входные параметры для cubemx_insert_code
type InsertCodeInput struct {
	FilePath  string `json:"file_path" jsonschema:"path to the CubeMX-generated .c file"`
	BlockName string `json:"block_name" jsonschema:"name of the USER CODE block (e.g., 'Includes', 'PV', '2', '4')"`
	Code      string `json:"code" jsonschema:"the C code to insert into the block"`
	Mode      string `json:"mode,omitempty" jsonschema:"insert mode: 'append' (add to end) or 'replace' (replace content). Default: append"`
}

// InsertCodeOutput — результат вставки кода
type InsertCodeOutput struct {
	FilePath     string `json:"file_path"`
	BlockName    string `json:"block_name"`
	Mode         string `json:"mode"`
	BytesWritten int    `json:"bytes_written"`
	Success      bool   `json:"success"`
	Message      string `json:"message"`
}

// FindWeakOverridesInput — входные параметры для cubemx_find_weak_overrides
type FindWeakOverridesInput struct {
	ProjectRoot string `json:"project_root" jsonschema:"the root directory of the CubeMX project"`
}

// FindWeakOverridesOutput — результат поиска переопределённых weak функций
type FindWeakOverridesOutput struct {
	Overrides []cubemx.WeakOverride `json:"overrides"`
	Count     int                   `json:"count"`
}

// ValidateProjectInput — входные параметры для cubemx_validate_project
type ValidateProjectInput struct {
	ProjectRoot string `json:"project_root" jsonschema:"the root directory of the CubeMX project"`
}

// ValidateProjectOutput — результат валидации проекта
type ValidateProjectOutput struct {
	Valid         bool     `json:"valid"`
	Errors        []string `json:"errors"`
	Warnings      []string `json:"warnings"`
	CubeMXVersion string   `json:"cubemx_version"`
	FilesCount    int      `json:"files_count"`
	BlocksCount   int      `json:"blocks_count"`
}

// queryDocsHandler — обработчик инструмента query_hardware_docs
func queryDocsHandler(ctx context.Context, req *mcp.CallToolRequest, input QueryDocsInput) (
	*mcp.CallToolResult,
	QueryDocsOutput,
	error,
) {
	// Поиск через Rust toolkit (Ф2-1: toolkit — единственный владелец RAG-доступа)
	if globalToolkit == nil {
		return nil, QueryDocsOutput{}, fmt.Errorf("toolkit is not available: set TOOLKIT_PATH to the toolkit binary (required for query_hardware_docs)")
	}

	// Установка лимита по умолчанию
	if input.Limit <= 0 {
		input.Limit = 5
	}

	// Поиск
	resp, err := globalToolkit.RagQuery(ctx, input.Query, toolkit.RagQueryOptions{
		Peripheral: input.Peripheral,
		Limit:      input.Limit,
	})
	if err != nil {
		return nil, QueryDocsOutput{}, fmt.Errorf("search failed: %w", err)
	}
	results := resp.Results

	output := QueryDocsOutput{
		Query:      input.Query,
		Peripheral: input.Peripheral,
		Count:      len(results),
		Results:    results,
	}

	return nil, output, nil
}

// readUserBlocksHandler — обработчик инструмента cubemx_read_user_blocks
func readUserBlocksHandler(ctx context.Context, req *mcp.CallToolRequest, input ReadUserBlocksInput) (
	*mcp.CallToolResult,
	ReadUserBlocksOutput,
	error,
) {
	if globalParser == nil {
		return nil, ReadUserBlocksOutput{}, fmt.Errorf("parser not initialized")
	}

	var results []*cubemx.ParseResult
	var err error

	if input.FilePath != "" {
		// Парсинг одного файла
		result, err := globalParser.ParseFile(input.FilePath)
		if err != nil {
			return nil, ReadUserBlocksOutput{}, fmt.Errorf("parse file failed: %w", err)
		}
		results = append(results, result)
	} else {
		// Парсинг всего проекта
		results, err = globalParser.ParseProject(input.ProjectRoot)
		if err != nil {
			return nil, ReadUserBlocksOutput{}, fmt.Errorf("parse project failed: %w", err)
		}
	}

	// Подсчёт общего количества блоков
	totalBlocks := 0
	for _, r := range results {
		totalBlocks += r.BlocksCount
	}

	output := ReadUserBlocksOutput{
		Files:       make([]cubemx.ParseResult, 0, len(results)),
		TotalBlocks: totalBlocks,
		FilesCount:  len(results),
	}

	for _, r := range results {
		output.Files = append(output.Files, *r)
	}

	return nil, output, nil
}

// insertCodeHandler — обработчик инструмента cubemx_insert_code
func insertCodeHandler(ctx context.Context, req *mcp.CallToolRequest, input InsertCodeInput) (
	*mcp.CallToolResult,
	InsertCodeOutput,
	error,
) {
	if globalPatcher == nil {
		return nil, InsertCodeOutput{}, fmt.Errorf("patcher not initialized")
	}

	// Установка режима по умолчанию
	mode := cubemx.InsertModeAppend
	if input.Mode == "replace" {
		mode = cubemx.InsertModeReplace
	}

	insertReq := &cubemx.InsertRequest{
		FilePath:  input.FilePath,
		BlockName: input.BlockName,
		Code:      input.Code,
		Mode:      mode,
	}

	result, err := globalPatcher.InsertCode(insertReq)
	if err != nil {
		return nil, InsertCodeOutput{}, fmt.Errorf("insert code failed: %w", err)
	}

	output := InsertCodeOutput{
		FilePath:     result.FilePath,
		BlockName:    result.BlockName,
		Mode:         result.Mode,
		BytesWritten: result.BytesWritten,
		Success:      result.Success,
		Message:      result.Message,
	}

	return nil, output, nil
}

// findWeakOverridesHandler — обработчик инструмента cubemx_find_weak_overrides
func findWeakOverridesHandler(ctx context.Context, req *mcp.CallToolRequest, input FindWeakOverridesInput) (
	*mcp.CallToolResult,
	FindWeakOverridesOutput,
	error,
) {
	if globalFinder == nil {
		return nil, FindWeakOverridesOutput{}, fmt.Errorf("finder not initialized")
	}

	overrides, err := globalFinder.FindOverrides(input.ProjectRoot)
	if err != nil {
		return nil, FindWeakOverridesOutput{}, fmt.Errorf("find overrides failed: %w", err)
	}

	output := FindWeakOverridesOutput{
		Overrides: overrides,
		Count:     len(overrides),
	}

	return nil, output, nil
}

// validateProjectHandler — обработчик инструмента cubemx_validate_project
func validateProjectHandler(ctx context.Context, req *mcp.CallToolRequest, input ValidateProjectInput) (
	*mcp.CallToolResult,
	ValidateProjectOutput,
	error,
) {
	if globalValidator == nil {
		return nil, ValidateProjectOutput{}, fmt.Errorf("validator not initialized")
	}

	result, err := globalValidator.ValidateProject(input.ProjectRoot)
	if err != nil {
		return nil, ValidateProjectOutput{}, fmt.Errorf("validate project failed: %w", err)
	}

	output := ValidateProjectOutput{
		Valid:         result.Valid,
		Errors:        result.Errors,
		Warnings:      result.Warnings,
		CubeMXVersion: result.CubeMXVersion,
		FilesCount:    result.FilesCount,
		BlocksCount:   result.BlocksCount,
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

// Глобальные переменные для parser, patcher, finder и validator (будут инициализированы в main)
var globalParser *cubemx.Parser
var globalPatcher *cubemx.Patcher
var globalFinder *cubemx.Finder
var globalValidator *cubemx.Validator

func main() {
	// Загрузка конфигурации
	cfg := config.Load()

	// Логирование в stderr (не мешает STDIO транспорту)
	log.SetOutput(os.Stderr)
	log.Printf("Starting STM32F407 MCP Server with official MCP SDK v1.8.0...")
	log.Printf("Collection: %s (RAG access via toolkit: QDRANT_URL/OLLAMA_URL env)", cfg.Collection)

	// Инициализация компонентов
	globalParser = cubemx.NewParser()
	globalPatcher = cubemx.NewPatcher()
	globalFinder = cubemx.NewFinder()
	globalValidator = cubemx.NewValidator()

	// Инициализация клиента Rust toolkit: TOOLKIT_PATH обязателен для всех
	// toolkit-инструментов (Фаза 1: ast_analyze_code, generate_doc_comments,
	// advanced_lint; Ф2-1: query_hardware_docs через rag-query).
	if tp := os.Getenv("TOOLKIT_PATH"); tp != "" {
		tc, err := toolkit.NewClient()
		if err != nil {
			log.Printf("toolkit client init failed: %v", err)
		} else {
			globalToolkit = tc
			vinfo, err := tc.Version(context.Background())
			if err != nil {
				log.Printf("toolkit version check failed: %v (advanced tools may misbehave)", err)
			} else {
				log.Printf("toolkit: %s", vinfo.Raw)
			}
		}
	} else {
		log.Printf("TOOLKIT_PATH is not set: query_hardware_docs, ast_analyze_code, generate_doc_comments and advanced_lint are disabled")
	}

	// Создание MCP сервера
	server := mcp.NewServer(
		&mcp.Implementation{
			Name:    "stm32f407-mcp",
			Version: ServerVersion,
		},
		nil, // options
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

	// Регистрация инструмента cubemx_read_user_blocks
	mcp.AddTool(
		server,
		&mcp.Tool{
			Name:        "cubemx_read_user_blocks",
			Description: "Read all USER CODE blocks from CubeMX-generated project files. Returns list of blocks with name, content, line number, and file path. Can parse single file or entire project. Use this BEFORE modifying any CubeMX-generated code to understand existing user code.",
		},
		readUserBlocksHandler,
	)

	// Регистрация инструмента cubemx_insert_code
	mcp.AddTool(
		server,
		&mcp.Tool{
			Name:        "cubemx_insert_code",
			Description: "Safely insert C code into a CubeMX USER CODE block. NEVER writes outside BEGIN/END markers. Supports 'append' (add to end of block) and 'replace' (replace block content) modes. Use cubemx_read_user_blocks first to see available blocks.",
		},
		insertCodeHandler,
	)

	// Регистрация инструмента cubemx_find_weak_overrides
	mcp.AddTool(
		server,
		&mcp.Tool{
			Name:        "cubemx_find_weak_overrides",
			Description: "Find all HAL weak function overrides in a CubeMX project. Scans all .c files for functions like HAL_GPIO_MspInit, HAL_TIM_PeriodElapsedCallback, etc. Use BEFORE creating a new override to avoid duplicates.",
		},
		findWeakOverridesHandler,
	)

	// Регистрация инструмента cubemx_validate_project
	mcp.AddTool(
		server,
		&mcp.Tool{
			Name:        "cubemx_validate_project",
			Description: "Validate CubeMX project integrity BEFORE making changes. Checks: main.c exists, USER CODE blocks are balanced, no duplicate weak overrides, detects CubeMX version. Call this FIRST before any project modifications.",
		},
		validateProjectHandler,
	)

	// Регистрация toolkit-инструментов (Фаза 1: AST, Ф2-1: RAG-поиск)
	if globalToolkit != nil {
		mcp.AddTool(
			server,
			&mcp.Tool{
				Name:        "query_hardware_docs",
				Description: "Search STM32F407 hardware documentation (RM0090) for registers, peripherals, clock configuration, etc. Returns relevant documentation chunks with metadata.",
			},
			queryDocsHandler,
		)
		mcp.AddTool(
			server,
			&mcp.Tool{
				Name:        "ast_analyze_code",
				Description: "Parse C code with a real C AST (tree-sitter) and extract: function definitions, variable declarations (including locals), HAL function calls, and section attributes (__attribute__((section(...)))). Returns exact positions (byte range + line/col) for every item. Use it to build precise edits instead of regex matching.",
			},
			astAnalyzeHandler,
		)
		mcp.AddTool(
			server,
			&mcp.Tool{
				Name:        "generate_doc_comments",
				Description: "Generate Doxygen comment stubs (/** @brief ... @param ... @retval ... */) for C functions that have no doc comment. Returns insertion points (byte_start == byte_end) so the agent can apply them without breaking formatting. Functions that already have a doc comment are skipped and counted in functions_skipped.",
			},
			generateDocCommentsHandler,
		)
		mcp.AddTool(
			server,
			&mcp.Tool{
				Name:        "advanced_lint",
				Description: "AST-based lint for STM32F407 pitfalls: DMA buffers in CCM RAM (only when the buffer is actually used by a DMA call), NVIC priority grouping, USB 48 MHz clock requirement, Flash ROP level 2. Returns violations with exact positions and fix hints. Set flow=true to additionally track variable usage in DMA call paths.",
			},
			advancedLintHandler,
		)
	}

	log.Printf("Registered tools: query_hardware_docs, check_f407_rules, cubemx_read_user_blocks, cubemx_insert_code, cubemx_find_weak_overrides, cubemx_validate_project%s", advancedToolsLogSuffix())
	log.Printf("MCP Server ready (v%s, %d tools), listening on stdin...", ServerVersion, 6+advancedToolsCount())

	// Запуск STDIO транспорта
	if err := server.Run(context.Background(), &mcp.StdioTransport{}); err != nil {
		log.Fatalf("Server error: %v", err)
	}
}

// globalToolkit — общий клиент Rust toolkit для advanced-инструментов.
var globalToolkit *toolkit.Client

// advancedToolsCount — сколько advanced-инструментов реально зарегистрировано.
func advancedToolsCount() int {
	if globalToolkit != nil {
		return 4
	}
	return 0
}

func advancedToolsLogSuffix() string {
	if globalToolkit != nil {
		return ", query_hardware_docs, ast_analyze_code, generate_doc_comments, advanced_lint"
	}
	return ""
}

package cubemx

import (
	"fmt"
	"os"
	"path/filepath"
	"regexp"
	"strings"
)

// ValidationResult — результат валидации проекта
type ValidationResult struct {
	Valid         bool     `json:"valid"`
	Errors        []string `json:"errors"`
	Warnings      []string `json:"warnings"`
	CubeMXVersion string   `json:"cubemx_version"`
	FilesCount    int      `json:"files_count"`
	BlocksCount   int      `json:"blocks_count"`
}

// Validator — валидатор CubeMX проекта
type Validator struct {
	parser *Parser
	finder *Finder
}

// NewValidator создаёт новый валидатор
func NewValidator() *Validator {
	return &Validator{
		parser: NewParser(),
		finder: NewFinder(),
	}
}

// ValidateProject валидирует целостность CubeMX проекта
func (v *Validator) ValidateProject(projectRoot string) (*ValidationResult, error) {
	result := &ValidationResult{
		Valid:    true,
		Errors:   make([]string, 0),
		Warnings: make([]string, 0),
	}

	// 1. Проверка наличия main.c
	mainC := filepath.Join(projectRoot, "src", "main.c")
	if _, err := os.Stat(mainC); os.IsNotExist(err) {
		result.Valid = false
		result.Errors = append(result.Errors, "main.c not found in src/")
		return result, nil
	}

	// 2. Подсчёт .c файлов
	fileCount := 0
	err := filepath.Walk(projectRoot, func(path string, info os.FileInfo, err error) error {
		if err != nil {
			return nil
		}
		if !info.IsDir() && strings.HasSuffix(info.Name(), ".c") {
			fileCount++
		}
		return nil
	})
	if err != nil {
		return nil, err
	}
	result.FilesCount = fileCount

	// 3. Извлечение версии CubeMX из main.c
	content, err := os.ReadFile(mainC)
	if err != nil {
		return nil, err
	}

	// Поиск версии CubeMX в комментарии
	// Формат: "This file is part of STM32CubeF4.1.0" или "MXCubeF4.1.0"
	versionPattern := regexp.MustCompile(`(?i)STM32Cube[F4][\w.]*|MXCube[F4][\w.]*`)
	if matches := versionPattern.FindString(string(content)); matches != "" {
		result.CubeMXVersion = matches
	}

	// 4. Проверка парности USER CODE BEGIN/END
	parseResults, err := v.parser.ParseProject(projectRoot)
	if err != nil {
		return nil, err
	}

	totalBlocks := 0
	for _, r := range parseResults {
		totalBlocks += r.BlocksCount
	}
	result.BlocksCount = totalBlocks

	// 5. Проверка на дубликаты weak функций
	overrides, err := v.finder.FindOverrides(projectRoot)
	if err != nil {
		return nil, err
	}

	// Группировка по имени функции
	seen := make(map[string][]WeakOverride)
	for _, o := range overrides {
		seen[o.Function] = append(seen[o.Function], o)
	}

	// Проверка дубликатов
	for funcName, occurrences := range seen {
		if len(occurrences) > 1 {
			result.Valid = false
			files := make([]string, 0, len(occurrences))
			for _, o := range occurrences {
				files = append(files, fmt.Sprintf("%s:%d", o.File, o.Line))
			}
			result.Errors = append(result.Errors,
				fmt.Sprintf("Duplicate override of %s in %d files: %s",
					funcName, len(occurrences), strings.Join(files, ", ")))
		}
	}

	// 6. Предупреждения
	if totalBlocks == 0 {
		result.Warnings = append(result.Warnings,
			"No USER CODE blocks found. Project may not be CubeMX-generated.")
	}

	if result.CubeMXVersion == "" {
		result.Warnings = append(result.Warnings,
			"Could not detect CubeMX version from main.c")
	}

	return result, nil
}

package cubemx

import (
	"fmt"
	"os"
	"path/filepath"
	"regexp"
	"strings"
)

// UserBlock — USER CODE блок из CubeMX файла
type UserBlock struct {
	Name    string `json:"name"`
	Content string `json:"content"`
	Line    int    `json:"line"`
	File    string `json:"file"`
}

// ParseResult — результат парсинга файла
type ParseResult struct {
	FilePath    string      `json:"file_path"`
	Blocks      []UserBlock `json:"blocks"`
	BlocksCount int         `json:"blocks_count"`
}

// Parser — парсер CubeMX файлов
type Parser struct{}

// NewParser создаёт новый парсер
func NewParser() *Parser {
	return &Parser{}
}

// ParseFile парсит один файл и извлекает все USER CODE блоки
func (p *Parser) ParseFile(filePath string) (*ParseResult, error) {
	content, err := os.ReadFile(filePath)
	if err != nil {
		return nil, err
	}

	return p.ParseContent(string(content), filePath)
}

// ParseContent парсит содержимое файла и извлекает USER CODE блоки
func (p *Parser) ParseContent(content string, filePath string) (*ParseResult, error) {
	result := &ParseResult{
		FilePath: filePath,
		Blocks:   make([]UserBlock, 0),
	}

	// Go regex не поддерживает backreference \1, поэтому используем двухэтапный подход:
	// 1. Находим все USER CODE BEGIN с их позициями
	// 2. Для каждого BEGIN ищем соответствующий END с тем же именем

	beginPattern := regexp.MustCompile(`/\*\s*USER CODE BEGIN ([\w\s]+?)\s*\*/`)
	endPatternTemplate := `/\*\s*USER CODE END %s\s*\*/`

	beginMatches := beginPattern.FindAllStringSubmatchIndex(content, -1)

	for _, beginMatch := range beginMatches {
		if len(beginMatch) < 4 {
			continue
		}

		// Извлечение имени блока
		nameStart := beginMatch[2]
		nameEnd := beginMatch[3]
		name := strings.TrimSpace(content[nameStart:nameEnd])

		// Создание паттерна для END с тем же именем
		endPattern := regexp.MustCompile(fmt.Sprintf(endPatternTemplate, regexp.QuoteMeta(name)))

		// Поиск END после текущего BEGIN
		searchStart := beginMatch[1] // Конец BEGIN комментария
		remainingContent := content[searchStart:]

		endMatch := endPattern.FindStringIndex(remainingContent)
		if endMatch == nil {
			continue // Нет соответствующего END, пропускаем
		}

		// Извлечение содержимого между BEGIN и END
		contentEnd := searchStart + endMatch[0] // Начало END комментария

		// Корректировка: нужно найти конец BEGIN комментария
		beginCommentEnd := beginMatch[1]
		blockContent := content[beginCommentEnd:contentEnd]

		// Очистка содержимого
		blockContent = strings.TrimSpace(blockContent)

		// Вычисление номера строки
		line := strings.Count(content[:beginMatch[0]], "\n") + 1

		block := UserBlock{
			Name:    name,
			Content: blockContent,
			Line:    line,
			File:    filePath,
		}

		result.Blocks = append(result.Blocks, block)
	}

	result.BlocksCount = len(result.Blocks)
	return result, nil
}

// ParseProject парсит все .c файлы в проекте
func (p *Parser) ParseProject(projectRoot string) ([]*ParseResult, error) {
	var results []*ParseResult

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
			return nil // Пропускаем файлы с ошибками
		}

		// Пропускаем системные директории
		if info.IsDir() {
			if skipDirs[info.Name()] {
				return filepath.SkipDir
			}
			return nil
		}

		// Парсим только .c файлы
		if strings.HasSuffix(info.Name(), ".c") {
			result, err := p.ParseFile(path)
			if err != nil {
				return nil // Пропускаем файлы с ошибками парсинга
			}
			if result.BlocksCount > 0 {
				results = append(results, result)
			}
		}

		return nil
	})

	if err != nil {
		return nil, err
	}

	return results, nil
}

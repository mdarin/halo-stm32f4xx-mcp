package cubemx

import (
	"fmt"
	"os"
	"path/filepath"
	"regexp"
	"strings"
)

// Patcher — безопасная вставка кода в USER CODE блоки
type Patcher struct{}

// NewPatcher создаёт новый патчер
func NewPatcher() *Patcher {
	return &Patcher{}
}

// InsertMode — режим вставки
type InsertMode string

const (
	InsertModeAppend  InsertMode = "append"
	InsertModeReplace InsertMode = "replace"
)

// InsertRequest — запрос на вставку кода
type InsertRequest struct {
	FilePath  string     `json:"file_path"`
	BlockName string     `json:"block_name"`
	Code      string     `json:"code"`
	Mode      InsertMode `json:"mode"` // "append" или "replace"
}

// InsertResult — результат вставки
type InsertResult struct {
	FilePath     string `json:"file_path"`
	BlockName    string `json:"block_name"`
	Mode         string `json:"mode"`
	BytesWritten int    `json:"bytes_written"`
	Success      bool   `json:"success"`
	Message      string `json:"message"`
}

// InsertCode вставляет код в USER CODE блок
func (p *Patcher) InsertCode(req *InsertRequest) (*InsertResult, error) {
	// Нормализация пути (кросс-платформенность)
	normalizedPath := filepath.Clean(req.FilePath)

	// Проверка существования файла
	if _, err := os.Stat(normalizedPath); os.IsNotExist(err) {
		return nil, fmt.Errorf("file not found: %s", normalizedPath)
	}

	// Чтение файла
	content, err := os.ReadFile(normalizedPath)
	if err != nil {
		return nil, fmt.Errorf("read file failed: %w", err)
	}

	// Поиск блока
	beginPattern := regexp.MustCompile(`/\*\s*USER CODE BEGIN ` + regexp.QuoteMeta(req.BlockName) + `\s*\*/`)
	endPattern := regexp.MustCompile(`/\*\s*USER CODE END ` + regexp.QuoteMeta(req.BlockName) + `\s*\*/`)

	beginMatch := beginPattern.FindStringIndex(string(content))
	if beginMatch == nil {
		return nil, fmt.Errorf("USER CODE BEGIN %s not found in %s", req.BlockName, normalizedPath)
	}

	endMatch := endPattern.FindStringIndex(string(content))
	if endMatch == nil {
		return nil, fmt.Errorf("USER CODE END %s not found in %s", req.BlockName, normalizedPath)
	}

	// Проверка, что END после BEGIN
	if endMatch[0] < beginMatch[1] {
		return nil, fmt.Errorf("USER CODE END %s appears before BEGIN in %s", req.BlockName, normalizedPath)
	}

	// Извлечение текущего содержимого блока
	blockContentStart := beginMatch[1]
	blockContentEnd := endMatch[0]
	currentContent := string(content[blockContentStart:blockContentEnd])

	// Вставка кода в зависимости от режима
	var newContent string
	switch req.Mode {
	case InsertModeAppend:
		// Добавление в конец блока
		newBlockContent := strings.TrimSpace(currentContent)
		if newBlockContent != "" {
			newBlockContent += "\n" + req.Code
		} else {
			newBlockContent = req.Code
		}
		newContent = string(content[:blockContentStart]) + "\n" + newBlockContent + "\n" + string(content[blockContentEnd:])

	case InsertModeReplace:
		// Замена содержимого блока
		newContent = string(content[:blockContentStart]) + "\n" + req.Code + "\n" + string(content[blockContentEnd:])

	default:
		return nil, fmt.Errorf("invalid insert mode: %s (use 'append' or 'replace')", req.Mode)
	}

	// Атомарная запись файла
	tempFile := normalizedPath + ".tmp"
	if err := os.WriteFile(tempFile, []byte(newContent), 0644); err != nil {
		return nil, fmt.Errorf("write temp file failed: %w", err)
	}

	if err := os.Rename(tempFile, normalizedPath); err != nil {
		os.Remove(tempFile)
		return nil, fmt.Errorf("rename temp file failed: %w", err)
	}

	result := &InsertResult{
		FilePath:     normalizedPath,
		BlockName:    req.BlockName,
		Mode:         string(req.Mode),
		BytesWritten: len(req.Code),
		Success:      true,
		Message:      fmt.Sprintf("Successfully %s code in USER CODE block '%s'", req.Mode, req.BlockName),
	}

	return result, nil
}

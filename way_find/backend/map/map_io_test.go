package mappkg

import (
	"os"
	"path/filepath"
	"testing"
)

func TestMapToFile(t *testing.T) {
	m := NewMap(10, 10)
	m.SetCell(0, 0, CELL_START)
	m.SetCell(9, 9, CELL_END)

	for i := 0; i < 5; i++ {
		m.SetCell(i, 5, CELL_WALL)
		m.SetCell(5, i+5, CELL_WALL)
	}

	text, err := m.ToText()
	if err != nil {
		t.Errorf("导出文本失败: %v", err)
	}

	dataDir := filepath.Join("..", "data")
	if err := os.MkdirAll(dataDir, 0755); err != nil {
		t.Errorf("创建 data 目录失败: %v", err)
	}

	filePath := filepath.Join(dataDir, "test_map.txt")
	if err := os.WriteFile(filePath, []byte(text), 0644); err != nil {
		t.Errorf("写入文件失败: %v", err)
	}

	content, err := os.ReadFile(filePath)
	if err != nil {
		t.Errorf("读取文件失败: %v", err)
	}

	if string(content) != text {
		t.Errorf("文件内容不匹配")
	}

	t.Logf("地图已保存到: %s", filePath)
	t.Logf("地图内容:\n%s", text)
}

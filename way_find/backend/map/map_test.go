package mappkg

import (
	"testing"
)

func TestMapToText(t *testing.T) {
	m := NewMap(5, 5)
	m.SetCell(0, 0, CELL_START)
	m.SetCell(4, 4, CELL_END)

	text, err := m.ToText()
	if err != nil {
		t.Errorf("导出文本失败: %v", err)
	}

	expected := `5 5
S . . . .
. . . . .
. . . . .
. . . . .
. . . . E`

	if text != expected {
		t.Errorf("导出文本不符合预期:\n期望:\n%s\n实际:\n%s", expected, text)
	}
}

func TestMapFromText(t *testing.T) {
	original := NewMap(5, 5)
	original.SetCell(0, 0, CELL_START)
	original.SetCell(4, 4, CELL_END)

	text, err := original.ToText()
	if err != nil {
		t.Errorf("导出文本失败: %v", err)
	}

	restored, err := MapFromText(text)
	if err != nil {
		t.Errorf("导入文本失败: %v", err)
	}

	if restored.Width != original.Width {
		t.Errorf("宽度不匹配: 期望 %d, 实际 %d", original.Width, restored.Width)
	}
	if restored.Height != original.Height {
		t.Errorf("高度不匹配: 期望 %d, 实际 %d", original.Height, restored.Height)
	}
	if restored.Start != original.Start {
		t.Errorf("起点不匹配: 期望 %v, 实际 %v", original.Start, restored.Start)
	}
	if restored.End != original.End {
		t.Errorf("终点不匹配: 期望 %v, 实际 %v", original.End, restored.End)
	}

	for y := 0; y < original.Height; y++ {
		for x := 0; x < original.Width; x++ {
			origCell, _ := original.GetCell(x, y)
			restCell, _ := restored.GetCell(x, y)
			if origCell != restCell {
				t.Errorf("格子 (%d, %d) 不匹配: 期望 %d, 实际 %d", x, y, origCell, restCell)
			}
		}
	}
}

func TestMapFromTextWithWalls(t *testing.T) {
	original := NewMap(10, 10)
	original.SetCell(0, 0, CELL_START)
	original.SetCell(9, 9, CELL_END)

	for i := 0; i < 5; i++ {
		original.SetCell(i, 5, CELL_WALL)
		original.SetCell(5, i+5, CELL_WALL)
	}

	text, err := original.ToText()
	if err != nil {
		t.Errorf("导出文本失败: %v", err)
	}

	restored, err := MapFromText(text)
	if err != nil {
		t.Errorf("导入文本失败: %v", err)
	}

	if !restored.IsWall(0, 5) {
		t.Errorf("墙壁应该保留在 (0, 5)")
	}
	if !restored.IsWall(5, 9) {
		t.Errorf("墙壁应该保留在 (5, 9)")
	}
}

func TestMapFromTextInvalid(t *testing.T) {
	_, err := MapFromText("invalid text")
	if err == nil {
		t.Errorf("无效的文本应该返回错误")
	}

	_, err = MapFromText("5")
	if err == nil {
		t.Errorf("无效的格式应该返回错误")
	}

	_, err = MapFromText("5 5\nS")
	if err == nil {
		t.Errorf("缺少终点应该返回错误")
	}

	_, err = MapFromText("5 5\nS . . . .\n. . . . .\n. . . . .\n. . . . .")
	if err == nil {
		t.Errorf("缺少终点应该返回错误")
	}
}

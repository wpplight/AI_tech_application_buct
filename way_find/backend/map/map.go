package mappkg

import (
	"errors"
	"fmt"
	"strconv"
	"strings"
)

type CellType uint8

const (
	CELL_ROAD  CellType = 0
	CELL_WALL  CellType = 1
	CELL_START CellType = 2
	CELL_END   CellType = 3
)

type Point struct {
	X int `json:"x"`
	Y int `json:"y"`
}

func (p Point) String() string {
	return fmt.Sprintf("(%d, %d)", p.X, p.Y)
}

func (p Point) Equals(other Point) bool {
	return p.X == other.X && p.Y == other.Y
}

type Map struct {
	Width  int
	Height int
	Grid   [][]CellType
	Start  Point
	End    Point
}

func NewMap(width, height int) *Map {
	if width < 0 || height < 0 {
		return nil
	}

	grid := make([][]CellType, height)
	for y := 0; y < height; y++ {
		grid[y] = make([]CellType, width)
		for x := 0; x < width; x++ {
			grid[y][x] = CELL_ROAD
		}
	}

	return &Map{
		Width:  width,
		Height: height,
		Grid:   grid,
		Start:  Point{-1, -1},
		End:    Point{-1, -1},
	}
}

func (m *Map) IsValid(x, y int) bool {
	return x >= 0 && x < m.Width && y >= 0 && y < m.Height
}

func (m *Map) IsWall(x, y int) bool {
	if !m.IsValid(x, y) {
		return true
	}
	return m.Grid[y][x] == CELL_WALL
}

func (m *Map) IsRoad(x, y int) bool {
	return !m.IsWall(x, y)
}

func (m *Map) SetCell(x, y int, cellType CellType) error {
	if !m.IsValid(x, y) {
		return errors.New("坐标超出地图范围")
	}

	if cellType == CELL_START {
		if m.Grid[y][x] == CELL_WALL {
			return errors.New("起点不能设置在墙壁上")
		}
		m.Start = Point{X: x, Y: y}
	} else if cellType == CELL_END {
		if m.Grid[y][x] == CELL_WALL {
			return errors.New("终点不能设置在墙壁上")
		}
		m.End = Point{X: x, Y: y}
	}

	m.Grid[y][x] = cellType
	return nil
}

func (m *Map) GetCell(x, y int) (CellType, error) {
	if !m.IsValid(x, y) {
		return CELL_ROAD, errors.New("坐标超出地图范围")
	}
	return m.Grid[y][x], nil
}

func (m *Map) HasStart() bool {
	return m.Start.X != -1 && m.Start.Y != -1
}

func (m *Map) HasEnd() bool {
	return m.End.X != -1 && m.End.Y != -1
}

func (m *Map) CanSearch() bool {
	return m.HasStart() && m.HasEnd()
}

func (m *Map) GetNeighbors(p Point) []Point {
	neighbors := make([]Point, 0, 4)
	directions := []Point{
		{X: 0, Y: -1},
		{X: 0, Y: 1},
		{X: -1, Y: 0},
		{X: 1, Y: 0},
	}

	for _, dir := range directions {
		nx, ny := p.X+dir.X, p.Y+dir.Y
		if m.IsValid(nx, ny) && m.IsRoad(nx, ny) {
			neighbors = append(neighbors, Point{X: nx, Y: ny})
		}
	}

	return neighbors
}

func (m *Map) Validate() error {
	if m.Width < 5 || m.Height < 5 {
		return errors.New("地图尺寸太小，最小为 5x5")
	}
	if m.Width > 50 || m.Height > 50 {
		return errors.New("地图尺寸太大，最大为 50x50")
	}
	if !m.HasStart() {
		return errors.New("未设置起点")
	}
	if !m.HasEnd() {
		return errors.New("未设置终点")
	}
	if m.Start.Equals(m.End) {
		return errors.New("起点和终点不能相同")
	}
	return nil
}

func (m *Map) String() string {
	result := fmt.Sprintf("地图: %dx%d\n", m.Width, m.Height)
	result += fmt.Sprintf("起点: %s\n", m.Start)
	result += fmt.Sprintf("终点: %s\n", m.End)
	result += "网格状态:\n"
	for y := 0; y < m.Height; y++ {
		for x := 0; x < m.Width; x++ {
			if x == m.Start.X && y == m.Start.Y {
				result += " S"
			} else if x == m.End.X && y == m.End.Y {
				result += " E"
			} else if m.Grid[y][x] == CELL_WALL {
				result += " #"
			} else {
				result += " ."
			}
		}
		result += "\n"
	}
	return result
}

func (m *Map) ToText() (string, error) {
	var lines []string

	lines = append(lines, fmt.Sprintf("%d %d", m.Width, m.Height))

	for y := 0; y < m.Height; y++ {
		row := make([]string, m.Width)
		for x := 0; x < m.Width; x++ {
			switch m.Grid[y][x] {
			case CELL_START:
				row[x] = "S"
			case CELL_END:
				row[x] = "E"
			case CELL_WALL:
				row[x] = "#"
			default:
				row[x] = "."
			}
		}
		lines = append(lines, strings.Join(row, " "))
	}

	return strings.Join(lines, "\n"), nil
}

func MapFromText(text string) (*Map, error) {
	lines := strings.Split(strings.TrimSpace(text), "\n")
	if len(lines) < 2 {
		return nil, errors.New("地图格式错误，至少需要 2 行")
	}

	sizeParts := strings.Split(strings.TrimSpace(lines[0]), " ")
	if len(sizeParts) != 2 {
		return nil, errors.New("第一行格式错误，应为: 宽度 高度")
	}

	width, err := strconv.Atoi(sizeParts[0])
	if err != nil {
		return nil, errors.New("宽度必须是数字")
	}
	height, err := strconv.Atoi(sizeParts[1])
	if err != nil {
		return nil, errors.New("高度必须是数字")
	}

	if width < 5 || height < 5 {
		return nil, errors.New("地图尺寸太小，最小为 5x5")
	}
	if width > 50 || height > 50 {
		return nil, errors.New("地图尺寸太大，最大为 50x50")
	}

	if len(lines)-1 != height {
		return nil, fmt.Errorf("地图行数不匹配，期望 %d 行，实际 %d 行", height, len(lines)-1)
	}

	grid := make([][]CellType, height)
	var start, end Point
	startSet, endSet := false, false

	for y := 0; y < height; y++ {
		cells := strings.Split(strings.TrimSpace(lines[y+1]), " ")
		if len(cells) != width {
			return nil, fmt.Errorf("第 %d 行格子数不匹配，期望 %d 个，实际 %d 个", y+2, width, len(cells))
		}

		grid[y] = make([]CellType, width)
		for x := 0; x < width; x++ {
			switch cells[x] {
			case "S":
				grid[y][x] = CELL_START
				start = Point{X: x, Y: y}
				startSet = true
			case "E":
				grid[y][x] = CELL_END
				end = Point{X: x, Y: y}
				endSet = true
			case "#":
				grid[y][x] = CELL_WALL
			default:
				grid[y][x] = CELL_ROAD
			}
		}
	}

	m := &Map{
		Width:  width,
		Height: height,
		Grid:   grid,
		Start:  start,
		End:    end,
	}

	if !startSet {
		return nil, errors.New("地图中未设置起点 (S)")
	}
	if !endSet {
		return nil, errors.New("地图中未设置终点 (E)")
	}

	if err := m.Validate(); err != nil {
		return nil, fmt.Errorf("地图数据无效: %v", err)
	}

	return m, nil
}

package main

import (
	"context"
	"fmt"
	"os"
	"path/filepath"
	"time"

	"wayfind/backend/algorithms"
	mappkg "wayfind/backend/map"
	"wayfind/backend/storage"
)

type WayFindService struct {
	ctx           context.Context
	currentMap    *mappkg.Map
	currentSearch algorithms.Searcher
	mapStorage    *storage.FileMapStorage
}

func NewWayFindService() *WayFindService {
	homeDir, err := os.UserHomeDir()
	if err != nil {
		homeDir = "."
	}
	defaultPath := filepath.Join(homeDir, "Documents", "WayFind", "maps")
	s, _ := storage.NewFileMapStorage(defaultPath)

	return &WayFindService{
		ctx:           nil,
		currentMap:    nil,
		currentSearch: nil,
		mapStorage:    s,
	}
}

func (s *WayFindService) ServiceStartup(ctx context.Context) error {
	s.ctx = ctx
	return nil
}

type MapData struct {
	Width      int       `json:"width"`
	Height     int       `json:"height"`
	Grid       [][]int   `json:"grid"`
	StartPoint PointData `json:"startPoint"`
	EndPoint   PointData `json:"endPoint"`
}

type PointData struct {
	X int `json:"x"`
	Y int `json:"y"`
}

type SearchResultData struct {
	Found     bool        `json:"found"`
	Distance  int         `json:"distance"`
	Path      []PointData `json:"path"`
	Algorithm string      `json:"algorithm"`
}

type StepData struct {
	State      int         `json:"state"`
	Current    PointData   `json:"current"`
	Neighbors  []PointData `json:"neighbors"`
	Added      []PointData `json:"added"`
	Pruned     []PointData `json:"pruned"`
	Path       []PointData `json:"path"`
	Visited    []PointData `json:"visited"`
	Distance   int         `json:"distance"`
	Expanded   int         `json:"expanded"`
	StepsTaken int         `json:"stepsTaken"`
}

func (s *WayFindService) CreateMap(width, height int) (*MapData, error) {
	m := mappkg.NewMap(width, height)
	if m == nil {
		return nil, fmt.Errorf("invalid map dimensions")
	}

	m.Start = mappkg.Point{X: 1, Y: 1}
	m.End = mappkg.Point{X: width - 2, Y: height - 2}
	m.Grid[1][1] = mappkg.CELL_START
	m.Grid[height-2][width-2] = mappkg.CELL_END

	s.currentMap = m
	return convertMapToData(m), nil
}

func (s *WayFindService) LoadMap(mapData *MapData) error {
	if mapData.Width <= 0 || mapData.Height <= 0 {
		return fmt.Errorf("invalid map dimensions")
	}

	m := mappkg.NewMap(mapData.Width, mapData.Height)
	if m == nil {
		return fmt.Errorf("failed to create map")
	}

	for y := 0; y < mapData.Height && y < len(mapData.Grid); y++ {
		for x := 0; x < mapData.Width && x < len(mapData.Grid[y]); x++ {
			switch mapData.Grid[y][x] {
			case 1:
				m.Grid[y][x] = mappkg.CELL_WALL
			case 2:
				m.Grid[y][x] = mappkg.CELL_START
				m.Start = mappkg.Point{X: x, Y: y}
			case 3:
				m.Grid[y][x] = mappkg.CELL_END
				m.End = mappkg.Point{X: x, Y: y}
			default:
				m.Grid[y][x] = mappkg.CELL_ROAD
			}
		}
	}

	s.currentMap = m
	return nil
}

func (s *WayFindService) SetCell(x, y, cellType int) error {
	if s.currentMap == nil {
		return fmt.Errorf("no map loaded")
	}

	var ct mappkg.CellType
	switch cellType {
	case 1:
		ct = mappkg.CELL_WALL
	case 2:
		ct = mappkg.CELL_START
	case 3:
		ct = mappkg.CELL_END
	default:
		ct = mappkg.CELL_ROAD
	}

	return s.currentMap.SetCell(x, y, ct)
}

func (s *WayFindService) InitializeSearch(algorithm string) error {
	if s.currentMap == nil {
		return fmt.Errorf("no map loaded")
	}

	if err := s.currentMap.Validate(); err != nil {
		return err
	}

	switch algorithm {
	case "bfs":
		s.currentSearch = algorithms.NewBFSAlgorithm()
	case "dfs":
		s.currentSearch = algorithms.NewDFSAlgorithm()
	case "astar":
		s.currentSearch = algorithms.NewAStarAlgorithm()
	default:
		return fmt.Errorf("unknown algorithm: %s", algorithm)
	}

	s.currentSearch.Initialize(s.currentMap)
	return nil
}

func (s *WayFindService) SearchStep() (*StepData, error) {
	if s.currentSearch == nil {
		return nil, fmt.Errorf("search not initialized")
	}

	if s.currentSearch.IsDone() {
		return nil, fmt.Errorf("search already completed")
	}

	result := s.currentSearch.Step()
	return convertStepResultToData(result), nil
}

func (s *WayFindService) IsSearchDone() bool {
	if s.currentSearch == nil {
		return true
	}
	return s.currentSearch.IsDone()
}

func (s *WayFindService) GetSearchResult() (*SearchResultData, error) {
	if s.currentSearch == nil {
		return nil, fmt.Errorf("search not initialized")
	}

	result := s.currentSearch.GetResult()
	return &SearchResultData{
		Found:     result.Found,
		Distance:  result.Distance,
		Path:      convertPointsToData(result.Path),
		Algorithm: result.Algorithm,
	}, nil
}

func (s *WayFindService) GetCurrentPath() ([]PointData, error) {
	if s.currentSearch == nil {
		return nil, fmt.Errorf("search not initialized")
	}

	path := s.currentSearch.GetCurrentPath()
	return convertPointsToData(path), nil
}

func (s *WayFindService) GetDraw() ([][]int, error) {
	if s.currentMap == nil {
		return nil, fmt.Errorf("no map loaded")
	}

	draw := make([][]int, s.currentMap.Height)
	for y := 0; y < s.currentMap.Height; y++ {
		draw[y] = make([]int, s.currentMap.Width)
		for x := 0; x < s.currentMap.Width; x++ {
			switch s.currentMap.Grid[y][x] {
			case mappkg.CELL_WALL:
				draw[y][x] = 1
			case mappkg.CELL_START:
				draw[y][x] = 2
			case mappkg.CELL_END:
				draw[y][x] = 3
			default:
				draw[y][x] = 0
			}
		}
	}

	if s.currentSearch != nil {
		visited := s.currentSearch.GetVisited()
		for _, p := range visited {
			if p.Y >= 0 && p.Y < s.currentMap.Height && p.X >= 0 && p.X < s.currentMap.Width {
				if draw[p.Y][p.X] == 0 {
					draw[p.Y][p.X] = 10
				}
			}
		}

		path := s.currentSearch.GetCurrentPath()
		for _, p := range path {
			if p.Y >= 0 && p.Y < s.currentMap.Height && p.X >= 0 && p.X < s.currentMap.Width {
				if draw[p.Y][p.X] != 2 && draw[p.Y][p.X] != 3 {
					draw[p.Y][p.X] = 20
				}
			}
		}
	}

	return draw, nil
}

func (s *WayFindService) GetFinalDraw() ([][]int, error) {
	if s.currentMap == nil {
		return nil, fmt.Errorf("no map loaded")
	}

	draw := make([][]int, s.currentMap.Height)
	for y := 0; y < s.currentMap.Height; y++ {
		draw[y] = make([]int, s.currentMap.Width)
		for x := 0; x < s.currentMap.Width; x++ {
			switch s.currentMap.Grid[y][x] {
			case mappkg.CELL_WALL:
				draw[y][x] = 1
			case mappkg.CELL_START:
				draw[y][x] = 2
			case mappkg.CELL_END:
				draw[y][x] = 3
			default:
				draw[y][x] = 0
			}
		}
	}

	if s.currentSearch != nil {
		visited := s.currentSearch.GetVisited()
		for _, p := range visited {
			if p.Y >= 0 && p.Y < s.currentMap.Height && p.X >= 0 && p.X < s.currentMap.Width {
				if draw[p.Y][p.X] == 0 {
					draw[p.Y][p.X] = 10
				}
			}
		}

		shortestPath := s.currentSearch.GetShortestPath()
		for _, p := range shortestPath {
			if p.Y >= 0 && p.Y < s.currentMap.Height && p.X >= 0 && p.X < s.currentMap.Width {
				if draw[p.Y][p.X] != 2 && draw[p.Y][p.X] != 3 {
					draw[p.Y][p.X] = 20
				}
			}
		}
	}

	return draw, nil
}

func (s *WayFindService) GetMap() (*MapData, error) {
	if s.currentMap == nil {
		return nil, fmt.Errorf("no map loaded")
	}

	return convertMapToData(s.currentMap), nil
}

func convertMapToData(m *mappkg.Map) *MapData {
	grid := make([][]int, m.Height)
	for y := 0; y < m.Height; y++ {
		grid[y] = make([]int, m.Width)
		for x := 0; x < m.Width; x++ {
			switch m.Grid[y][x] {
			case mappkg.CELL_WALL:
				grid[y][x] = 1
			case mappkg.CELL_START:
				grid[y][x] = 2
			case mappkg.CELL_END:
				grid[y][x] = 3
			default:
				grid[y][x] = 0
			}
		}
	}

	return &MapData{
		Width:      m.Width,
		Height:     m.Height,
		Grid:       grid,
		StartPoint: PointData{X: m.Start.X, Y: m.Start.Y},
		EndPoint:   PointData{X: m.End.X, Y: m.End.Y},
	}
}

func convertStepResultToData(result *algorithms.StepResult) *StepData {
	return &StepData{
		State:      int(result.State),
		Current:    PointData{X: result.Current.X, Y: result.Current.Y},
		Neighbors:  convertPointsToData(result.Neighbors),
		Added:      convertPointsToData(result.Added),
		Pruned:     convertPointsToData(result.Pruned),
		Path:       convertPointsToData(result.Path),
		Visited:    convertPointsToData(result.Visited),
		Distance:   result.Distance,
		Expanded:   result.Expanded,
		StepsTaken: result.StepsTaken,
	}
}

func convertPointsToData(points []mappkg.Point) []PointData {
	result := make([]PointData, len(points))
	for i, p := range points {
		result[i] = PointData{X: p.X, Y: p.Y}
	}
	return result
}

type MapInfoData struct {
	Name       string    `json:"name"`
	Width      int       `json:"width"`
	Height     int       `json:"height"`
	CreatedAt  time.Time `json:"createdAt"`
	ModifiedAt time.Time `json:"modifiedAt"`
}

func (s *WayFindService) GetStoragePath() string {
	if s.mapStorage == nil {
		return ""
	}
	return s.mapStorage.GetStoragePath()
}

func (s *WayFindService) SetStoragePath(path string) error {
	if s.mapStorage == nil {
		s.mapStorage, _ = storage.NewFileMapStorage(path)
	} else {
		return s.mapStorage.SetStoragePath(path)
	}
	return nil
}

func (s *WayFindService) SaveMap(name string) error {
	if s.currentMap == nil {
		return fmt.Errorf("没有可保存的地图")
	}
	if s.mapStorage == nil {
		return fmt.Errorf("存储未初始化")
	}
	return s.mapStorage.Save(name, s.currentMap)
}

func (s *WayFindService) LoadMapByName(name string) (*MapData, error) {
	if s.mapStorage == nil {
		return nil, fmt.Errorf("存储未初始化")
	}

	m, err := s.mapStorage.Load(name)
	if err != nil {
		return nil, err
	}

	s.currentMap = m
	return convertMapToData(m), nil
}

func (s *WayFindService) DeleteMap(name string) error {
	if s.mapStorage == nil {
		return fmt.Errorf("存储未初始化")
	}
	return s.mapStorage.Delete(name)
}

func (s *WayFindService) ListMaps() ([]MapInfoData, error) {
	if s.mapStorage == nil {
		return nil, fmt.Errorf("存储未初始化")
	}

	infos, err := s.mapStorage.List()
	if err != nil {
		return nil, err
	}

	result := make([]MapInfoData, len(infos))
	for i, info := range infos {
		result[i] = MapInfoData{
			Name:       info.Name,
			Width:      info.Width,
			Height:     info.Height,
			CreatedAt:  info.CreatedAt,
			ModifiedAt: info.ModifiedAt,
		}
	}
	return result, nil
}

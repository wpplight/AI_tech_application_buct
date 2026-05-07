package algorithms

import (
	"container/heap"
	"math"
	mappkg "wayfind/backend/map"
	pathpkg "wayfind/backend/path"
)

type AStarItem struct {
	Point mappkg.Point
	G     int
	H     int
	F     int
	Index int
}

type AStarHeap []AStarItem

func (h AStarHeap) Len() int           { return len(h) }
func (h AStarHeap) Less(i, j int) bool { return h[i].F < h[j].F }
func (h AStarHeap) Swap(i, j int)      { h[i], h[j] = h[j], h[i]; h[i].Index = i; h[j].Index = j }

func (h *AStarHeap) Push(x interface{}) {
	item := x.(AStarItem)
	item.Index = len(*h)
	*h = append(*h, item)
}

func (h *AStarHeap) Pop() interface{} {
	old := *h
	n := len(old)
	item := old[n-1]
	*h = old[0 : n-1]
	return item
}

func ManhattanDistance(p1, p2 mappkg.Point) int {
	return int(math.Abs(float64(p1.X-p2.X)) + math.Abs(float64(p1.Y-p2.Y)))
}

type AStarAlgorithm struct {
	m        *mappkg.Map
	heap     AStarHeap
	distGrid *pathpkg.DistGrid
	visited  []mappkg.Point
	state    SearchState
	current  mappkg.Point
	bestDist int
	bestPath []mappkg.Point
}

func NewAStarAlgorithm() *AStarAlgorithm {
	return &AStarAlgorithm{
		m:        nil,
		heap:     nil,
		distGrid: nil,
		visited:  nil,
		state:    StateReady,
		bestDist: math.MaxInt,
		bestPath: nil,
	}
}

func (a *AStarAlgorithm) Initialize(m *mappkg.Map) {
	a.m = m
	a.heap = make(AStarHeap, 0)
	a.distGrid = pathpkg.NewDistGrid(m.Width, m.Height)
	a.visited = make([]mappkg.Point, 0)
	a.state = StateReady
	a.bestDist = math.MaxInt
	a.bestPath = nil

	if err := m.Validate(); err != nil {
		a.state = StateNotFound
		return
	}

	g := 0
	h := ManhattanDistance(m.Start, m.End)
	item := AStarItem{Point: m.Start, G: g, H: h, F: g + h}
	heap.Push(&a.heap, item)
	a.distGrid.Set(m.Start, g)
	a.state = StateRunning
}

func (a *AStarAlgorithm) IsDone() bool {
	if a.state == StateFound || a.state == StateNotFound {
		return true
	}
	if len(a.heap) == 0 {
		if a.bestDist < math.MaxInt {
			a.state = StateFound
		} else {
			a.state = StateNotFound
		}
		return true
	}
	return false
}

func (a *AStarAlgorithm) GetResult() *SearchResult {
	return &SearchResult{
		Found:     a.state == StateFound,
		Distance:  a.bestDist,
		Path:      a.bestPath,
		Algorithm: "A*",
	}
}

func (a *AStarAlgorithm) Step() *StepResult {
	if a.IsDone() {
		return &StepResult{
			State:   a.state,
			Current: a.m.Start,
			Visited: a.visited,
		}
	}

	item := heap.Pop(&a.heap).(AStarItem)
	current := item.Point
	currentG := item.G
	a.current = current

	if a.distGrid.Get(current) < currentG {
		return &StepResult{
			State:      a.state,
			Current:    current,
			Path:       a.bestPath,
			Visited:    a.visited,
			Distance:   a.bestDist,
			StepsTaken: len(a.visited),
		}
	}

	if current.Equals(a.m.End) {
		a.bestDist = currentG
		a.recordBestPath(current)
		a.state = StateFound
		return &StepResult{
			State:      a.state,
			Current:    current,
			Path:       a.bestPath,
			Visited:    a.visited,
			Distance:   a.bestDist,
			StepsTaken: len(a.visited),
		}
	}

	neighbors := a.m.GetNeighbors(current)
	added := make([]mappkg.Point, 0)

	for _, neighbor := range neighbors {
		newG := currentG + 1
		if a.distGrid.IsBetter(neighbor, newG) {
			a.distGrid.Set(neighbor, newG)
			h := ManhattanDistance(neighbor, a.m.End)
			newItem := AStarItem{Point: neighbor, G: newG, H: h, F: newG + h}
			heap.Push(&a.heap, newItem)
			added = append(added, neighbor)
		}
	}

	a.visited = append(a.visited, current)

	return &StepResult{
		State:      a.state,
		Current:    current,
		Neighbors:  neighbors,
		Added:      added,
		Path:       a.bestPath,
		Visited:    a.visited,
		Distance:   a.bestDist,
		StepsTaken: len(a.visited),
	}
}

func (a *AStarAlgorithm) recordBestPath(end mappkg.Point) {
	path := make([]mappkg.Point, 0)
	current := end
	for {
		path = append(path, current)
		if current.Equals(a.m.Start) {
			break
		}
		step := a.distGrid.Get(current)
		if step == 0 {
			break
		}
		neighbors := a.m.GetNeighbors(current)
		found := false
		for _, n := range neighbors {
			if a.distGrid.Get(n) == step-1 {
				current = n
				found = true
				break
			}
		}
		if !found {
			break
		}
	}
	for i, j := 0, len(path)-1; i < j; i, j = i+1, j-1 {
		path[i], path[j] = path[j], path[i]
	}
	a.bestPath = path
}

func (a *AStarAlgorithm) GetHeapSize() int {
	return len(a.heap)
}

func (a *AStarAlgorithm) GetCurrentPath() []mappkg.Point {
	if a.current.X == 0 && a.current.Y == 0 && !a.current.Equals(a.m.Start) {
		return nil
	}
	return a.getPathFromPoint(a.current)
}

func (a *AStarAlgorithm) getPathFromPoint(p mappkg.Point) []mappkg.Point {
	path := make([]mappkg.Point, 0)
	current := p
	for {
		path = append(path, current)
		if current.Equals(a.m.Start) {
			break
		}
		step := a.distGrid.Get(current)
		if step == 0 {
			break
		}
		neighbors := a.m.GetNeighbors(current)
		found := false
		for _, n := range neighbors {
			if a.distGrid.Get(n) == step-1 {
				current = n
				found = true
				break
			}
		}
		if !found {
			break
		}
	}
	for i, j := 0, len(path)-1; i < j; i, j = i+1, j-1 {
		path[i], path[j] = path[j], path[i]
	}
	return path
}

func (a *AStarAlgorithm) GetVisited() []mappkg.Point {
	result := make([]mappkg.Point, len(a.visited))
	copy(result, a.visited)
	return result
}

func (a *AStarAlgorithm) GetShortestPath() []mappkg.Point {
	endDist := a.distGrid.Get(a.m.End)
	if endDist == math.MaxInt {
		return nil
	}
	return a.getPathFromPoint(a.m.End)
}

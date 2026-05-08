package algorithms

import (
	"math"
	mappkg "wayfind/backend/map"
	pathpkg "wayfind/backend/path"
	"wayfind/backend/queue"
)

type BFSAlgorithm struct {
	m        *mappkg.Map
	queue    []queue.BFSItem
	distGrid *pathpkg.DistGrid
	visited  []mappkg.Point
	state    SearchState
	current  mappkg.Point
	bestDist int
	bestPath []mappkg.Point
	Expanded int
}

func NewBFSAlgorithm() *BFSAlgorithm {
	return &BFSAlgorithm{
		m:        nil,
		queue:    nil,
		distGrid: nil,
		visited:  nil,
		state:    StateReady,
		bestDist: math.MaxInt,
		bestPath: nil,
	}
}

func (b *BFSAlgorithm) Initialize(m *mappkg.Map) {
	b.m = m
	b.queue = make([]queue.BFSItem, 0)
	b.distGrid = pathpkg.NewDistGrid(m.Width, m.Height)
	b.visited = make([]mappkg.Point, 0)
	b.state = StateReady
	b.bestDist = math.MaxInt
	b.bestPath = nil

	if err := m.Validate(); err != nil {
		b.state = StateNotFound
		return
	}

	b.distGrid.Set(m.Start, 0)
	b.queue = append(b.queue, queue.BFSItem{Point: m.Start, Step: 0})
	b.state = StateRunning
}

func (b *BFSAlgorithm) IsDone() bool {
	if b.state == StateFound || b.state == StateNotFound {
		return true
	}
	if len(b.queue) == 0 {
		if b.bestDist < math.MaxInt {
			b.state = StateFound
		} else {
			b.state = StateNotFound
		}
		return true
	}
	return false
}

func (b *BFSAlgorithm) GetResult() *SearchResult {
	return &SearchResult{
		Found:     b.state == StateFound,
		Distance:  b.bestDist,
		Path:      b.bestPath,
		Algorithm: "BFS",
		Expanded:  b.Expanded,
	}
}

func (b *BFSAlgorithm) Step() *StepResult {
	if b.IsDone() {
		return &StepResult{
			State:      b.state,
			Current:    b.m.Start,
			Visited:    b.visited,
			Distance:   b.bestDist,
			StepsTaken: len(b.visited),
			Expanded:   b.Expanded,
		}
	}

	item := b.queue[0]
	b.queue = b.queue[1:]
	b.Expanded++
	current := item.Point
	currentStep := item.Step
	b.current = current

	if b.distGrid.Get(current) < currentStep {
		return &StepResult{
			State:      b.state,
			Current:    current,
			Path:       b.bestPath,
			Visited:    b.visited,
			Distance:   b.bestDist,
			StepsTaken: len(b.visited),
			Expanded:   b.Expanded,
		}
	}

	b.distGrid.Set(current, currentStep)

	if current.Equals(b.m.End) {
		b.bestDist = currentStep
		b.recordBestPath(current)
		b.state = StateFound
		return &StepResult{
			State:      b.state,
			Current:    current,
			Path:       b.bestPath,
			Visited:    b.visited,
			Distance:   b.bestDist,
			StepsTaken: len(b.visited),
			Expanded:   b.Expanded,
		}
	}

	neighbors := b.m.GetNeighbors(current)
	added := make([]mappkg.Point, 0)

	for _, neighbor := range neighbors {
		newStep := currentStep + 1
		if b.distGrid.IsBetter(neighbor, newStep) {
			b.distGrid.Set(neighbor, newStep)
			b.queue = append(b.queue, queue.BFSItem{Point: neighbor, Step: newStep})
			added = append(added, neighbor)
		}
	}

	b.visited = append(b.visited, current)

	return &StepResult{
		State:      b.state,
		Current:    current,
		Neighbors:  neighbors,
		Added:      added,
		Path:       b.bestPath,
		Visited:    b.visited,
		Distance:   b.bestDist,
		StepsTaken: len(b.visited),
		Expanded:   b.Expanded,
	}
}

func (b *BFSAlgorithm) recordBestPath(end mappkg.Point) {
	path := make([]mappkg.Point, 0)
	current := end
	for {
		path = append(path, current)
		if current.Equals(b.m.Start) {
			break
		}
		step := b.distGrid.Get(current)
		if step == 0 {
			break
		}
		neighbors := b.m.GetNeighbors(current)
		found := false
		for _, n := range neighbors {
			if b.distGrid.Get(n) == step-1 {
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
	b.bestPath = path
}

func (b *BFSAlgorithm) GetQueueSize() int {
	return len(b.queue)
}

func (b *BFSAlgorithm) GetCurrentPath() []mappkg.Point {
	if b.current.X == 0 && b.current.Y == 0 && !b.current.Equals(b.m.Start) {
		return nil
	}
	return b.getPathFromPoint(b.current)
}

func (b *BFSAlgorithm) getPathFromPoint(p mappkg.Point) []mappkg.Point {
	path := make([]mappkg.Point, 0)
	current := p
	for {
		path = append(path, current)
		if current.Equals(b.m.Start) {
			break
		}
		step := b.distGrid.Get(current)
		if step == 0 {
			break
		}
		neighbors := b.m.GetNeighbors(current)
		found := false
		for _, n := range neighbors {
			if b.distGrid.Get(n) == step-1 {
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

func (b *BFSAlgorithm) GetVisited() []mappkg.Point {
	result := make([]mappkg.Point, len(b.visited))
	copy(result, b.visited)
	return result
}

func (b *BFSAlgorithm) GetShortestPath() []mappkg.Point {
	endDist := b.distGrid.Get(b.m.End)
	if endDist == math.MaxInt {
		return nil
	}
	return b.getPathFromPoint(b.m.End)
}

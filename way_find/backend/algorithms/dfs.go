package algorithms

import (
	"math"
	mappkg "wayfind/backend/map"
	pathpkg "wayfind/backend/path"
)

type DFSItem struct {
	Point mappkg.Point
	Step  int
}

type DFSAlgorithm struct {
	m          *mappkg.Map
	stack      []DFSItem
	distGrid   *pathpkg.DistGrid
	visited    []mappkg.Point
	state      SearchState
	current    mappkg.Point
	bestDist   int
	bestPath   []mappkg.Point
	parentSnap [][]mappkg.Point
	Expanded   int
}

func NewDFSAlgorithm() *DFSAlgorithm {
	return &DFSAlgorithm{
		m:        nil,
		stack:    nil,
		distGrid: nil,
		visited:  nil,
		state:    StateReady,
		bestDist: math.MaxInt,
		bestPath: nil,
	}
}

func (d *DFSAlgorithm) Initialize(m *mappkg.Map) {
	d.m = m
	d.stack = make([]DFSItem, 0)
	d.distGrid = pathpkg.NewDistGrid(m.Width, m.Height)
	d.visited = make([]mappkg.Point, 0)
	d.state = StateReady
	d.bestDist = math.MaxInt
	d.bestPath = nil
	d.parentSnap = nil

	if err := m.Validate(); err != nil {
		d.state = StateNotFound
		return
	}

	d.distGrid.Set(m.Start, 0)
	d.stack = append(d.stack, DFSItem{Point: m.Start, Step: 0})
	d.state = StateRunning
}

func (d *DFSAlgorithm) IsDone() bool {
	if d.state == StateFound || d.state == StateNotFound {
		return true
	}
	if len(d.stack) == 0 {
		if d.bestDist < math.MaxInt {
			d.state = StateFound
		} else {
			d.state = StateNotFound
		}
		return true
	}
	return false
}

func (d *DFSAlgorithm) GetResult() *SearchResult {
	return &SearchResult{
		Found:     d.state == StateFound,
		Distance:  d.bestDist,
		Path:      d.bestPath,
		Algorithm: "DFS",
		Expanded:  d.Expanded,
	}
}

func (d *DFSAlgorithm) Step() *StepResult {
	if d.IsDone() {
		return &StepResult{
			State:      d.state,
			Current:    d.m.Start,
			Visited:    d.visited,
			Distance:   d.bestDist,
			StepsTaken: len(d.visited),
			Expanded:   d.Expanded,
		}
	}

	item := d.stack[len(d.stack)-1]
	d.stack = d.stack[:len(d.stack)-1]
	d.Expanded++
	current := item.Point
	currentStep := item.Step
	d.current = current

	if d.distGrid.Get(current) < currentStep {
		return &StepResult{
			State:      d.state,
			Current:    current,
			Neighbors:  nil,
			Added:      nil,
			Path:       d.bestPath,
			Visited:    d.visited,
			Distance:   d.bestDist,
			StepsTaken: len(d.visited),
			Expanded:   d.Expanded,
		}
	}

	d.distGrid.Set(current, currentStep)

	if current.Equals(d.m.End) && currentStep < d.bestDist {
		d.bestDist = currentStep
		d.recordBestPath(current)
	}

	if current.Equals(d.m.End) {
		return &StepResult{
			State:      d.state,
			Current:    current,
			Path:       d.bestPath,
			Visited:    d.visited,
			Distance:   d.bestDist,
			StepsTaken: len(d.visited),
			Expanded:   d.Expanded,
		}
	}

	neighbors := d.m.GetNeighbors(current)
	added := make([]mappkg.Point, 0)

	for _, neighbor := range neighbors {
		newStep := currentStep + 1
		if d.distGrid.IsBetter(neighbor, newStep) && newStep < d.bestDist {
			d.distGrid.Set(neighbor, newStep)
			d.stack = append(d.stack, DFSItem{Point: neighbor, Step: newStep})
			added = append(added, neighbor)
		}
	}

	d.visited = append(d.visited, current)

	return &StepResult{
		State:      d.state,
		Current:    current,
		Neighbors:  neighbors,
		Added:      added,
		Path:       d.bestPath,
		Visited:    d.visited,
		Distance:   d.bestDist,
		StepsTaken: len(d.visited),
		Expanded:   d.Expanded,
	}
}

func (d *DFSAlgorithm) recordBestPath(end mappkg.Point) {
	path := make([]mappkg.Point, 0)
	current := end
	for {
		path = append(path, current)
		if current.Equals(d.m.Start) {
			break
		}
		step := d.distGrid.Get(current)
		if step == 0 {
			break
		}
		neighbors := d.m.GetNeighbors(current)
		found := false
		for _, n := range neighbors {
			if d.distGrid.Get(n) == step-1 {
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
	d.bestPath = path
}

func (d *DFSAlgorithm) GetStackSize() int {
	return len(d.stack)
}

func (d *DFSAlgorithm) GetCurrentPath() []mappkg.Point {
	if d.current.X == 0 && d.current.Y == 0 && !d.current.Equals(d.m.Start) {
		return nil
	}
	return d.getPathFromPoint(d.current)
}

func (d *DFSAlgorithm) getPathFromPoint(p mappkg.Point) []mappkg.Point {
	path := make([]mappkg.Point, 0)
	current := p
	for {
		path = append(path, current)
		if current.Equals(d.m.Start) {
			break
		}
		step := d.distGrid.Get(current)
		if step == 0 {
			break
		}
		neighbors := d.m.GetNeighbors(current)
		found := false
		for _, n := range neighbors {
			if d.distGrid.Get(n) == step-1 {
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

func (d *DFSAlgorithm) GetVisited() []mappkg.Point {
	result := make([]mappkg.Point, len(d.visited))
	copy(result, d.visited)
	return result
}

func (d *DFSAlgorithm) GetShortestPath() []mappkg.Point {
	if d.bestDist == math.MaxInt {
		return nil
	}
	return d.getPathFromPoint(d.m.End)
}

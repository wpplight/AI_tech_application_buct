package pathpkg

import (
	"math"
	mappkg "wayfind/backend/map"
)

type DistGrid struct {
	grid   [][]int
	width  int
	height int
}

func NewDistGrid(width, height int) *DistGrid {
	grid := make([][]int, height)
	for y := 0; y < height; y++ {
		grid[y] = make([]int, width)
		for x := 0; x < width; x++ {
			grid[y][x] = math.MaxInt
		}
	}
	return &DistGrid{
		grid:   grid,
		width:  width,
		height: height,
	}
}

func (dg *DistGrid) Get(p mappkg.Point) int {
	if p.X < 0 || p.X >= dg.width || p.Y < 0 || p.Y >= dg.height {
		return math.MaxInt
	}
	return dg.grid[p.Y][p.X]
}

func (dg *DistGrid) Set(p mappkg.Point, dist int) {
	if p.X < 0 || p.X >= dg.width || p.Y < 0 || p.Y >= dg.height {
		return
	}
	dg.grid[p.Y][p.X] = dist
}

func (dg *DistGrid) IsBetter(p mappkg.Point, newDist int) bool {
	return newDist < dg.Get(p)
}

func (dg *DistGrid) MarkVisited(p mappkg.Point) {
	dg.Set(p, 0)
}

func (dg *DistGrid) Width() int {
	return dg.width
}

func (dg *DistGrid) Height() int {
	return dg.height
}

func (dg *DistGrid) IsUnvisited(p mappkg.Point) bool {
	return dg.Get(p) == math.MaxInt
}

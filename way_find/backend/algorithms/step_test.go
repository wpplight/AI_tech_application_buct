package algorithms

import (
	"testing"
	mappkg "wayfind/backend/map"
)

func TestBFSStep(t *testing.T) {
	m := mappkg.NewMap(5, 5)
	m.SetCell(0, 0, mappkg.CELL_START)
	m.SetCell(4, 4, mappkg.CELL_END)

	bfs := NewBFSAlgorithm()
	bfs.Initialize(m)

	step := 0
	for !bfs.IsDone() {
		step++
		bfs.Step()
	}

	final := bfs.GetResult()

	if !final.Found {
		t.Errorf("BFS 应该找到路径")
	}
	if final.Distance != 8 {
		t.Errorf("BFS 距离应为 8, 实际为 %d", final.Distance)
	}
}

func TestDFSStep(t *testing.T) {
	m := mappkg.NewMap(5, 5)
	m.SetCell(0, 0, mappkg.CELL_START)
	m.SetCell(4, 4, mappkg.CELL_END)

	dfs := NewDFSAlgorithm()
	dfs.Initialize(m)

	step := 0
	for !dfs.IsDone() {
		step++
		dfs.Step()
	}

	final := dfs.GetResult()

	if !final.Found {
		t.Errorf("DFS 应该找到路径")
	}
}

func TestAStarStep(t *testing.T) {
	m := mappkg.NewMap(5, 5)
	m.SetCell(0, 0, mappkg.CELL_START)
	m.SetCell(4, 4, mappkg.CELL_END)

	astar := NewAStarAlgorithm()
	astar.Initialize(m)

	step := 0
	for !astar.IsDone() {
		step++
		astar.Step()
	}

	final := astar.GetResult()

	if !final.Found {
		t.Errorf("A* 应该找到路径")
	}
	if final.Distance != 8 {
		t.Errorf("A* 距离应为 8, 实际为 %d", final.Distance)
	}
}

func TestPathOverwrite(t *testing.T) {
	m := mappkg.NewMap(10, 10)
	m.SetCell(0, 0, mappkg.CELL_START)
	m.SetCell(9, 9, mappkg.CELL_END)

	bfs := NewBFSAlgorithm()
	bfs.Initialize(m)
	for !bfs.IsDone() {
		bfs.Step()
	}
	result := bfs.GetResult()

	dfs := NewDFSAlgorithm()
	dfs.Initialize(m)
	for !dfs.IsDone() {
		dfs.Step()
	}
	resultDFS := dfs.GetResult()

	astar := NewAStarAlgorithm()
	astar.Initialize(m)
	for !astar.IsDone() {
		astar.Step()
	}
	resultAStar := astar.GetResult()

	if !result.Found || !resultDFS.Found || !resultAStar.Found {
		t.Errorf("所有算法都应该找到路径")
	}

	if result.Distance != resultAStar.Distance {
		t.Errorf("BFS 和 A* 应该找到相同的最短距离")
	}
}

func TestCompareAlgorithms(t *testing.T) {
	m := mappkg.NewMap(10, 10)
	m.SetCell(0, 0, mappkg.CELL_START)
	m.SetCell(9, 9, mappkg.CELL_END)

	for i := 0; i < 5; i++ {
		m.SetCell(i, 5, mappkg.CELL_WALL)
		m.SetCell(5, i+5, mappkg.CELL_WALL)
	}

	bfs := NewBFSAlgorithm()
	bfs.Initialize(m)
	for !bfs.IsDone() {
		bfs.Step()
	}
	resultBFS := bfs.GetResult()

	dfs := NewDFSAlgorithm()
	dfs.Initialize(m)
	for !dfs.IsDone() {
		dfs.Step()
	}
	resultDFS := dfs.GetResult()

	astar := NewAStarAlgorithm()
	astar.Initialize(m)
	for !astar.IsDone() {
		astar.Step()
	}
	resultAStar := astar.GetResult()

	if !resultBFS.Found || !resultDFS.Found || !resultAStar.Found {
		t.Errorf("所有算法应该找到路径")
	}

	if resultBFS.Distance != resultAStar.Distance {
		t.Errorf("BFS 和 A* 应该找到相同的最短距离")
	}
}

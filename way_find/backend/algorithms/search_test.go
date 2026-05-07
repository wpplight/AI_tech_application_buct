package algorithms

import (
	"fmt"
	"testing"
	mappkg "wayfind/backend/map"
)

func TestBFS(t *testing.T) {
	m := mappkg.NewMap(5, 5)
	m.SetCell(0, 0, mappkg.CELL_START)
	m.SetCell(4, 4, mappkg.CELL_END)

	bfs := NewBFSAlgorithm()
	bfs.Initialize(m)

	for !bfs.IsDone() {
		bfs.Step()
	}

	result := bfs.GetResult()

	fmt.Printf("BFS 结果:\n")
	fmt.Printf("  找到: %v\n", result.Found)
	fmt.Printf("  路径长度: %d\n", result.Distance)
	fmt.Printf("  扩展节点数: %d\n", result.Expanded)
	if result.Found {
		fmt.Printf("  路径: %v\n", result.Path)
	}

	if !result.Found {
		t.Errorf("BFS 应该找到路径")
	}
}

func TestDFS(t *testing.T) {
	m := mappkg.NewMap(5, 5)
	m.SetCell(0, 0, mappkg.CELL_START)
	m.SetCell(4, 4, mappkg.CELL_END)

	dfs := NewDFSAlgorithm()
	dfs.Initialize(m)

	for !dfs.IsDone() {
		dfs.Step()
	}

	result := dfs.GetResult()

	fmt.Printf("DFS 结果:\n")
	fmt.Printf("  找到: %v\n", result.Found)
	fmt.Printf("  路径长度: %d\n", result.Distance)
	fmt.Printf("  扩展节点数: %d\n", result.Expanded)
	if result.Found {
		fmt.Printf("  路径: %v\n", result.Path)
	}

	if !result.Found {
		t.Errorf("DFS 应该找到路径")
	}
}

func TestAStar(t *testing.T) {
	m := mappkg.NewMap(5, 5)
	m.SetCell(0, 0, mappkg.CELL_START)
	m.SetCell(4, 4, mappkg.CELL_END)

	astar := NewAStarAlgorithm()
	astar.Initialize(m)

	for !astar.IsDone() {
		astar.Step()
	}

	result := astar.GetResult()

	fmt.Printf("A* 结果:\n")
	fmt.Printf("  找到: %v\n", result.Found)
	fmt.Printf("  路径长度: %d\n", result.Distance)
	fmt.Printf("  扩展节点数: %d\n", result.Expanded)
	if result.Found {
		fmt.Printf("  路径: %v\n", result.Path)
	}

	if !result.Found {
		t.Errorf("A* 应该找到路径")
	}
}

func TestBFSWithObstacles(t *testing.T) {
	m := mappkg.NewMap(5, 5)
	m.SetCell(0, 0, mappkg.CELL_START)
	m.SetCell(4, 4, mappkg.CELL_END)

	m.SetCell(1, 0, mappkg.CELL_WALL)
	m.SetCell(1, 1, mappkg.CELL_WALL)
	m.SetCell(1, 2, mappkg.CELL_WALL)

	bfs := NewBFSAlgorithm()
	bfs.Initialize(m)

	for !bfs.IsDone() {
		bfs.Step()
	}

	result := bfs.GetResult()

	fmt.Printf("BFS（有障碍物）结果:\n")
	fmt.Printf("  找到: %v\n", result.Found)
	fmt.Printf("  路径长度: %d\n", result.Distance)
	if result.Found {
		fmt.Printf("  路径: %v\n", result.Path)
	}
}

func TestNoPath(t *testing.T) {
	m := mappkg.NewMap(5, 5)
	m.SetCell(0, 0, mappkg.CELL_START)
	m.SetCell(4, 4, mappkg.CELL_END)

	for x := 0; x < 5; x++ {
		m.SetCell(x, 2, mappkg.CELL_WALL)
	}

	bfs := NewBFSAlgorithm()
	bfs.Initialize(m)

	for !bfs.IsDone() {
		bfs.Step()
	}

	result := bfs.GetResult()

	fmt.Printf("BFS（无路径）结果:\n")
	fmt.Printf("  找到: %v\n", result.Found)

	if result.Found {
		t.Errorf("BFS 不应该找到路径")
	}
}

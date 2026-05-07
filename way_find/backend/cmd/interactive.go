package main

import (
	"bufio"
	"fmt"
	"os"
	"path/filepath"
	"strings"

	algorithms "wayfind/backend/algorithms"
	mappkg "wayfind/backend/map"
)

func main() {
	fmt.Println("======================================")
	fmt.Println("      迷宫路径查找 - 交互式演示")
	fmt.Println("======================================")
	fmt.Println()

	mapFile := selectMapFile()
	fmt.Printf("加载地图: %s\n\n", mapFile)

	text, err := os.ReadFile(mapFile)
	if err != nil {
		fmt.Printf("读取文件失败: %v\n", err)
		return
	}

	m, err := mappkg.MapFromText(string(text))
	if err != nil {
		fmt.Printf("解析地图失败: %v\n", err)
		return
	}

	algo := selectAlgorithm()

	searcher := createSearcher(algo, m)
	searcher.Initialize(m)

	fmt.Println("\n======================================")
	fmt.Println("          开始逐步演示")
	fmt.Println("======================================")
	fmt.Printf("算法: %s\n", getAlgorithmName(algo))
	fmt.Printf("起点: (%d, %d)\n", m.Start.X, m.Start.Y)
	fmt.Printf("终点: (%d, %d)\n", m.End.X, m.End.Y)
	fmt.Println("======================================")

	printMap(m, nil, nil, nil)

	reader := bufio.NewReader(os.Stdin)
	step := 0

	for !searcher.IsDone() {
		fmt.Print("\n按回车执行一步 (q 退出): ")
		input, _ := reader.ReadString('\n')
		input = strings.TrimSpace(input)

		if input == "q" || input == "Q" {
			fmt.Println("退出程序")
			return
		}

		step++
		result := searcher.Step()

		fmt.Printf("\n===== 第 %d 步 =====\n", step)
		fmt.Printf("状态: %s\n", getStateName(result.State))
		if result.Current.X != 0 || result.Current.Y != 0 {
			fmt.Printf("当前位置: (%d, %d)\n", result.Current.X, result.Current.Y)
		}
		fmt.Printf("已扩展节点: %d\n", result.Expanded)
		fmt.Printf("路径长度: %d\n", len(result.Path))

		if len(result.Added) > 0 {
			fmt.Printf("新增节点: %v\n", result.Added)
		}
		if len(result.Pruned) > 0 {
			fmt.Printf("剪枝节点: %v\n", result.Pruned)
		}

		currentPath := searcher.GetCurrentPath()
		printMap(m, currentPath, result.Visited, result.Path)

		if result.State == algorithms.StateFound {
			fmt.Println("\n🎉 找到终点!")
			fmt.Printf("路径长度: %d\n", result.Distance)
			fmt.Printf("总步数: %d\n", step)
			break
		}

		if result.State == algorithms.StateNotFound {
			fmt.Println("\n❌ 未找到路径!")
			break
		}
	}

	fmt.Println("\n演示结束")
}

func selectMapFile() string {
	files, err := filepath.Glob("./data/*.txt")
	if err != nil || len(files) == 0 {
		fmt.Println("未找到地图文件，使用默认地图")
		return "../data/test_map.txt"
	}

	fmt.Println("选择地图文件:")
	for i, f := range files {
		name := filepath.Base(f)
		fmt.Printf("  %d. %s\n", i+1, name)
	}
	fmt.Printf("选择 (1-%d) [默认 1]: ", len(files))

	reader := bufio.NewReader(os.Stdin)
	input, _ := reader.ReadString('\n')
	input = strings.TrimSpace(input)

	if input == "" {
		return files[0]
	}

	var idx int
	fmt.Sscanf(input, "%d", &idx)
	if idx < 1 || idx > len(files) {
		return files[0]
	}

	return files[idx-1]
}

func selectAlgorithm() int {
	fmt.Println("选择算法:")
	fmt.Println("  1. BFS (广度优先搜索)")
	fmt.Println("  2. DFS (深度优先搜索)")
	fmt.Println("  3. A* (A星算法)")
	fmt.Print("选择 (1-3) [默认 1]: ")

	reader := bufio.NewReader(os.Stdin)
	input, _ := reader.ReadString('\n')
	input = strings.TrimSpace(input)

	if input == "" {
		return 1
	}

	var choice int
	fmt.Sscanf(input, "%d", &choice)
	if choice < 1 || choice > 3 {
		return 1
	}

	return choice
}

func createSearcher(choice int, m *mappkg.Map) algorithms.Searcher {
	switch choice {
	case 2:
		return algorithms.NewDFSAlgorithm()
	case 3:
		return algorithms.NewAStarAlgorithm()
	default:
		return algorithms.NewBFSAlgorithm()
	}
}

func getAlgorithmName(choice int) string {
	switch choice {
	case 2:
		return "DFS"
	case 3:
		return "A*"
	default:
		return "BFS"
	}
}

func getStateName(state algorithms.SearchState) string {
	switch state {
	case algorithms.StateReady:
		return "就绪"
	case algorithms.StateRunning:
		return "运行中"
	case algorithms.StateFound:
		return "已找到"
	case algorithms.StateNotFound:
		return "未找到"
	default:
		return "未知"
	}
}

func printMap(m *mappkg.Map, currentPath []mappkg.Point, visited []mappkg.Point, bestPath []mappkg.Point) {
	currentPathSet := make(map[string]bool)
	for _, p := range currentPath {
		currentPathSet[fmt.Sprintf("%d,%d", p.X, p.Y)] = true
	}

	bestPathSet := make(map[string]bool)
	for _, p := range bestPath {
		bestPathSet[fmt.Sprintf("%d,%d", p.X, p.Y)] = true
	}

	visitedSet := make(map[string]bool)
	for _, p := range visited {
		visitedSet[fmt.Sprintf("%d,%d", p.X, p.Y)] = true
	}

	fmt.Println()
	for y := 0; y < m.Height; y++ {
		line := ""
		for x := 0; x < m.Width; x++ {
			pos := fmt.Sprintf("%d,%d", x, y)
			isOnCurrentPath := currentPathSet[pos]
			isOnBestPath := bestPathSet[pos]
			isVisited := visitedSet[pos]

			if x == m.Start.X && y == m.Start.Y {
				line += "S "
			} else if x == m.End.X && y == m.End.Y {
				if isOnBestPath {
					line += "@ "
				} else {
					line += "E "
				}
			} else if m.Grid[y][x] == mappkg.CELL_WALL {
				line += "# "
			} else if isOnCurrentPath {
				line += "@ "
			} else if isOnBestPath {
				line += "+ "
			} else if isVisited {
				line += "* "
			} else {
				line += ". "
			}
		}
		fmt.Println(line)
	}
	fmt.Println("图例: S=起点, E=终点, #=墙, .=未探索, *=已访问, +=最短路径, @=当前路径")
}

package main

import (
	"fmt"
	"sync"
	"time"

	"wayfind/backend/algorithms"
	mappkg "wayfind/backend/map"
	"wayfind/backend/storage"
)

type TaskState int

const (
	TaskStateIdle       TaskState = 0
	TaskStateSearching  TaskState = 1
	TaskStateDone       TaskState = 2
	TaskStateFailed     TaskState = 3
)

type Task struct {
	ID      string
	Name    string
	MapName string // 来源地图名称
	State   TaskState
	Algorithm string
	Map     *mappkg.Map
	Search  algorithms.Searcher
	CreatedAt time.Time
	UpdatedAt time.Time
}

type TaskManager struct {
	mu         sync.RWMutex
	tasks      map[string]*Task
	mapStorage *storage.FileMapStorage
}

func NewTaskManager(storagePath string) *TaskManager {
	var s *storage.FileMapStorage
	if storagePath != "" {
		s, _ = storage.NewFileMapStorage(storagePath)
	}
	return &TaskManager{
		tasks:      make(map[string]*Task),
		mapStorage: s,
	}
}

func (tm *TaskManager) Create(width, height int, name string, mapName string) (*Task, error) {
	if width < 3 || width > 500 || height < 3 || height > 500 {
		return nil, fmt.Errorf("地图尺寸必须在 3-500 之间")
	}

	taskID := fmt.Sprintf("task_%d", time.Now().UnixNano())
	m := mappkg.NewMap(width, height)

	if mapName != "" && tm.mapStorage != nil {
		if loaded, err := tm.mapStorage.Load(mapName); err == nil {
			for y := 0; y < loaded.Height && y < m.Height; y++ {
				for x := 0; x < loaded.Width && x < m.Width; x++ {
					if cell, err := loaded.GetCell(x, y); err == nil {
						m.SetCell(x, y, cell)
					}
				}
			}
		}
	}

	task := &Task{
		ID:        taskID,
		Name:      name,
		MapName:   mapName,
		State:     TaskStateIdle,
		Algorithm: "",
		Map:       m,
		CreatedAt: time.Now(),
		UpdatedAt: time.Now(),
	}

	tm.mu.Lock()
	tm.tasks[taskID] = task
	tm.mu.Unlock()

	return task, nil
}

func (tm *TaskManager) Get(taskID string) (*Task, error) {
	tm.mu.RLock()
	defer tm.mu.RUnlock()

	task, ok := tm.tasks[taskID]
	if !ok {
		return nil, fmt.Errorf("任务不存在: %s", taskID)
	}
	return task, nil
}

func (tm *TaskManager) Delete(taskID string) error {
	tm.mu.Lock()
	defer tm.mu.Unlock()

	if _, ok := tm.tasks[taskID]; !ok {
		return fmt.Errorf("任务不存在: %s", taskID)
	}
	delete(tm.tasks, taskID)
	return nil
}

func (tm *TaskManager) InitSearch(taskID, algorithm string) error {
	task, err := tm.Get(taskID)
	if err != nil {
		return err
	}

	if task.Map == nil {
		return fmt.Errorf("地图未创建")
	}
	if !task.Map.CanSearch() {
		return fmt.Errorf("地图缺少起点或终点")
	}

	var search algorithms.Searcher
	switch algorithm {
	case "bfs":
		search = algorithms.NewBFSAlgorithm()
	case "dfs":
		search = algorithms.NewDFSAlgorithm()
	case "astar":
		search = algorithms.NewAStarAlgorithm()
	default:
		return fmt.Errorf("不支持的算法: %s", algorithm)
	}

	search.Initialize(task.Map)

	task.Search = search
	task.Algorithm = algorithm
	task.State = TaskStateSearching
	task.UpdatedAt = time.Now()

	return nil
}

func (tm *TaskManager) Step(taskID string) (*algorithms.StepResult, error) {
	task, err := tm.Get(taskID)
	if err != nil {
		return nil, err
	}

	if task.Search == nil {
		return nil, fmt.Errorf("请先调用 /search/init 初始化搜索")
	}

	if task.State == TaskStateDone || task.State == TaskStateFailed {
		return nil, fmt.Errorf("搜索已完成，无法继续")
	}

	result := task.Search.Step()

	if result.State == algorithms.StateFound || result.State == algorithms.StateNotFound {
		task.State = TaskStateDone
	} else {
		task.State = TaskStateSearching
	}
	task.UpdatedAt = time.Now()

	return result, nil
}

func (tm *TaskManager) GetResult(taskID string) (*algorithms.SearchResult, error) {
	task, err := tm.Get(taskID)
	if err != nil {
		return nil, err
	}
	if task.Search == nil {
		return nil, fmt.Errorf("请先调用 /search/init 初始化搜索")
	}
	return task.Search.GetResult(), nil
}

func (tm *TaskManager) ResetSearch(taskID string) error {
	task, err := tm.Get(taskID)
	if err != nil {
		return err
	}
	task.Search = nil
	task.Algorithm = ""
	task.State = TaskStateIdle
	task.UpdatedAt = time.Now()
	return nil
}

func (tm *TaskManager) List() []*Task {
	tm.mu.RLock()
	defer tm.mu.RUnlock()

	tasks := make([]*Task, 0, len(tm.tasks))
	for _, t := range tm.tasks {
		tasks = append(tasks, t)
	}
	return tasks
}

func (tm *TaskManager) Count() int {
	tm.mu.RLock()
	defer tm.mu.RUnlock()
	return len(tm.tasks)
}

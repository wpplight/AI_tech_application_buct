package main

import (
	"net/http"
	mappkg "wayfind/backend/map"

	"github.com/gin-gonic/gin"
)

type Handler struct {
	taskMgr *TaskManager
}

func NewHandler(taskMgr *TaskManager) *Handler {
	return &Handler{taskMgr: taskMgr}
}

type CreateMapRequest struct {
	Width   int    `json:"width" binding:"required,min=3,max=500"`
	Height  int    `json:"height" binding:"required,min=3,max=500"`
	Name    string `json:"name"`
	MapName string `json:"mapName"`
}

type SetCellRequest struct {
	TaskID   string `json:"taskId" binding:"required"`
	X        int    `json:"x" binding:"min=0"`
	Y        int    `json:"y" binding:"min=0"`
	CellType int    `json:"cellType" binding:"min=0,max=3"`
}

type InitSearchRequest struct {
	TaskID    string `json:"taskId" binding:"required"`
	Algorithm string `json:"algorithm" binding:"required,oneof=bfs dfs astar"`
}

func (h *Handler) HealthCheck(c *gin.Context) {
	respondOK(c, gin.H{
		"status":    "ok",
		"taskCount": h.taskMgr.Count(),
	})
}

func (h *Handler) GetAlgorithms(c *gin.Context) {
	respondOK(c, gin.H{
		"algorithms": []string{"bfs", "dfs", "astar"},
		"default":    "astar",
	})
}

func (h *Handler) CreateTask(c *gin.Context) {
	var req CreateMapRequest
	if err := c.ShouldBindJSON(&req); err != nil {
		respondError(c, 40001, "参数错误: "+err.Error())
		return
	}

	name := req.Name
	if name == "" {
		name = "未命名地图"
	}

	task, err := h.taskMgr.Create(req.Width, req.Height, name, req.MapName)
	if err != nil {
		respondError(c, 50001, err.Error())
		return
	}

	respondOK(c, TaskInfo{
		TaskID:    task.ID,
		Name:      task.Name,
		MapName:   task.MapName,
		State:     "idle",
		Algorithm: "",
		Width:     task.Map.Width,
		Height:    task.Map.Height,
		CreatedAt: task.CreatedAt,
		UpdatedAt: task.UpdatedAt,
	})
}

func (h *Handler) GetTask(c *gin.Context) {
	taskID := c.Param("taskId")
	if taskID == "" {
		respondError(c, 40001, "taskId 不能为空")
		return
	}

	task, err := h.taskMgr.Get(taskID)
	if err != nil {
		respondError(c, 40401, err.Error())
		return
	}

	stateStr := taskStateToString(task.State)

	var mapData *MapData
	if task.Map != nil {
		mapData = mapToMapData(task.Map)
	}

	respondOK(c, gin.H{
		"task": TaskInfo{
			TaskID:    task.ID,
			Name:      task.Name,
			MapName:   task.MapName,
			State:     stateStr,
			Algorithm: task.Algorithm,
			Width:     task.Map.Width,
			Height:    task.Map.Height,
			CreatedAt: task.CreatedAt,
			UpdatedAt: task.UpdatedAt,
		},
		"map": mapData,
	})
}

func (h *Handler) DeleteTask(c *gin.Context) {
	taskID := c.Param("taskId")
	if taskID == "" {
		respondError(c, 40001, "taskId 不能为空")
		return
	}

	if err := h.taskMgr.Delete(taskID); err != nil {
		respondError(c, 40401, err.Error())
		return
	}
	respondOK(c, gin.H{"taskId": taskID})
}

func (h *Handler) ListTasks(c *gin.Context) {
	tasks := h.taskMgr.List()
	taskList := make([]TaskInfo, 0, len(tasks))
	for _, t := range tasks {
		taskList = append(taskList, TaskInfo{
			TaskID:    t.ID,
			Name:      t.Name,
			MapName:   t.MapName,
			State:     taskStateToString(t.State),
			Algorithm: t.Algorithm,
			Width:     t.Map.Width,
			Height:    t.Map.Height,
			CreatedAt: t.CreatedAt,
			UpdatedAt: t.UpdatedAt,
		})
	}
	respondOK(c, gin.H{
		"tasks": taskList,
		"total": len(taskList),
	})
}

func (h *Handler) SetCell(c *gin.Context) {
	var req SetCellRequest
	if err := c.ShouldBindJSON(&req); err != nil {
		respondError(c, 40001, "参数错误: "+err.Error())
		return
	}

	task, err := h.taskMgr.Get(req.TaskID)
	if err != nil {
		respondError(c, 40401, err.Error())
		return
	}

	if err := task.Map.SetCell(req.X, req.Y, mappkg.CellType(req.CellType)); err != nil {
		respondError(c, 40101, err.Error())
		return
	}
	task.UpdatedAt = task.UpdatedAt

	respondOK(c, gin.H{"message": "ok"})
}

func (h *Handler) InitializeSearch(c *gin.Context) {
	var req InitSearchRequest
	if err := c.ShouldBindJSON(&req); err != nil {
		respondError(c, 40001, "参数错误: "+err.Error())
		return
	}

	if err := h.taskMgr.InitSearch(req.TaskID, req.Algorithm); err != nil {
		respondError(c, 40101, err.Error())
		return
	}

	task, _ := h.taskMgr.Get(req.TaskID)
	respondOK(c, gin.H{
		"taskId":    req.TaskID,
		"algorithm": req.Algorithm,
		"state":     taskStateToString(task.State),
		"hasStart":  task.Map.HasStart(),
		"hasEnd":    task.Map.HasEnd(),
	})
}

func (h *Handler) SearchStep(c *gin.Context) {
	taskID := c.Query("taskId")
	if taskID == "" {
		var req struct {
			TaskID string `json:"taskId" binding:"required"`
		}
		if err := c.ShouldBindJSON(&req); err == nil {
			taskID = req.TaskID
		}
	}
	if taskID == "" {
		respondError(c, 40001, "taskId 不能为空")
		return
	}

	step, err := h.taskMgr.Step(taskID)
	if err != nil {
		respondError(c, 40201, err.Error())
		return
	}

	task, _ := h.taskMgr.Get(taskID)
	draw, _ := buildDraw(task)

	respondOK(c, gin.H{
		"step":      step,
		"draw":      draw,
		"taskState": taskStateToString(task.State),
	})
}

func (h *Handler) GetSearchDone(c *gin.Context) {
	taskID := c.Query("taskId")
	if taskID == "" {
		respondError(c, 40001, "taskId 不能为空")
		return
	}

	task, err := h.taskMgr.Get(taskID)
	if err != nil {
		respondError(c, 40401, err.Error())
		return
	}
	respondOK(c, gin.H{
		"done":   task.State == TaskStateDone || task.State == TaskStateFailed,
		"state":  taskStateToString(task.State),
		"taskId": taskID,
	})
}

func (h *Handler) GetSearchResult(c *gin.Context) {
	taskID := c.Query("taskId")
	if taskID == "" {
		respondError(c, 40001, "taskId 不能为空")
		return
	}

	result, err := h.taskMgr.GetResult(taskID)
	if err != nil {
		respondError(c, 40201, err.Error())
		return
	}

	task, _ := h.taskMgr.Get(taskID)
	finalDraw, _ := buildFinalDraw(task)

	respondOK(c, gin.H{
		"result":    result,
		"finalDraw": finalDraw,
	})
}

func (h *Handler) ResetSearch(c *gin.Context) {
	taskID := c.Query("taskId")
	if taskID == "" {
		respondError(c, 40001, "taskId 不能为空")
		return
	}

	if err := h.taskMgr.ResetSearch(taskID); err != nil {
		respondError(c, 40201, err.Error())
		return
	}

	task, _ := h.taskMgr.Get(taskID)
	draw, _ := buildDraw(task)

	respondOK(c, gin.H{
		"taskId": taskID,
		"state":  taskStateToString(task.State),
		"draw":   draw,
	})
}

func (h *Handler) GetCurrentPath(c *gin.Context) {
	taskID := c.Query("taskId")
	if taskID == "" {
		respondError(c, 40001, "taskId 不能为空")
		return
	}

	task, err := h.taskMgr.Get(taskID)
	if err != nil {
		respondError(c, 40401, err.Error())
		return
	}
	if task.Search == nil {
		respondError(c, 40201, "搜索未初始化")
		return
	}

	path := task.Search.GetCurrentPath()
	pathData := make([]PointData, 0, len(path))
	for _, p := range path {
		pathData = append(pathData, PointData{X: p.X, Y: p.Y})
	}

	respondOK(c, pathData)
}

func (h *Handler) GetMap(c *gin.Context) {
	taskID := c.Query("taskId")
	if taskID == "" {
		respondError(c, 40001, "taskId 不能为空")
		return
	}

	task, err := h.taskMgr.Get(taskID)
	if err != nil {
		respondError(c, 40401, err.Error())
		return
	}

	respondOK(c, mapToMapData(task.Map))
}

func (h *Handler) GetDraw(c *gin.Context) {
	taskID := c.Query("taskId")
	if taskID == "" {
		respondError(c, 40001, "taskId 不能为空")
		return
	}

	task, err := h.taskMgr.Get(taskID)
	if err != nil {
		respondError(c, 40401, err.Error())
		return
	}

	draw, _ := buildDraw(task)
	respondOK(c, draw)
}

func (h *Handler) GetFinalDraw(c *gin.Context) {
	taskID := c.Query("taskId")
	if taskID == "" {
		respondError(c, 40001, "taskId 不能为空")
		return
	}

	task, err := h.taskMgr.Get(taskID)
	if err != nil {
		respondError(c, 40401, err.Error())
		return
	}

	draw, _ := buildFinalDraw(task)
	respondOK(c, draw)
}

func (h *Handler) ListMaps(c *gin.Context) {
	s := NewWayFindService()
	maps, err := s.mapStorage.List()
	if err != nil {
		respondError(c, 50001, err.Error())
		return
	}
	respondOK(c, maps)
}

func (h *Handler) SaveMap(c *gin.Context) {
	name := c.Param("name")
	if name == "" {
		respondError(c, 40001, "地图名称不能为空")
		return
	}

	var req struct {
		TaskID string  `json:"taskId"`
		Grid   [][]int `json:"grid"`
		Width  int     `json:"width"`
		Height int     `json:"height"`
	}

	if err := c.ShouldBindJSON(&req); err != nil {
		respondError(c, 40001, "参数错误: "+err.Error())
		return
	}

	s := NewWayFindService()

	if req.TaskID != "" && len(req.Grid) > 0 {
		task, err := h.taskMgr.Get(req.TaskID)
		if err != nil {
			respondError(c, 40401, err.Error())
			return
		}
		for y := 0; y < task.Map.Height && y < len(req.Grid); y++ {
			for x := 0; x < task.Map.Width && x < len(req.Grid[y]); x++ {
				task.Map.SetCell(x, y, mappkg.CellType(req.Grid[y][x]))
			}
		}
		if err := s.mapStorage.Save(name, task.Map); err != nil {
			respondError(c, 50001, err.Error())
			return
		}
	} else if len(req.Grid) > 0 && req.Width > 0 && req.Height > 0 {
		m := mappkg.NewMap(req.Width, req.Height)
		for y := 0; y < req.Height && y < len(req.Grid); y++ {
			for x := 0; x < req.Width && x < len(req.Grid[y]); x++ {
				m.SetCell(x, y, mappkg.CellType(req.Grid[y][x]))
			}
		}
		if err := s.mapStorage.Save(name, m); err != nil {
			respondError(c, 50001, err.Error())
			return
		}
	} else {
		respondError(c, 40001, "缺少地图数据")
		return
	}

	respondOK(c, gin.H{"name": name})
}

func (h *Handler) LoadMap(c *gin.Context) {
	name := c.Param("name")

	s := NewWayFindService()
	loaded, err := s.mapStorage.Load(name)
	if err != nil {
		respondError(c, 40101, "加载失败: "+err.Error())
		return
	}

	respondOK(c, gin.H{
		"map": mapToMapData(loaded),
	})
}

func (h *Handler) DeleteMap(c *gin.Context) {
	name := c.Param("name")
	if name == "" {
		respondError(c, 40001, "地图名称不能为空")
		return
	}

	s := NewWayFindService()
	if err := s.mapStorage.Delete(name); err != nil {
		respondError(c, 50001, err.Error())
		return
	}
	respondOK(c, gin.H{"name": name})
}

func respondOK(c *gin.Context, data interface{}) {
	c.JSON(http.StatusOK, gin.H{"code": 0, "data": data})
}

func respondError(c *gin.Context, code int, message string) {
	c.JSON(http.StatusOK, gin.H{"code": code, "message": message})
}

type TaskInfo struct {
	TaskID    string `json:"taskId"`
	Name      string `json:"name"`
	MapName   string `json:"mapName,omitempty"`
	State     string `json:"state"`
	Algorithm string `json:"algorithm"`
	Width     int    `json:"width"`
	Height    int    `json:"height"`
	CreatedAt any    `json:"createdAt"`
	UpdatedAt any    `json:"updatedAt"`
}

func taskStateToString(s TaskState) string {
	switch s {
	case TaskStateIdle:
		return "idle"
	case TaskStateSearching:
		return "searching"
	case TaskStateDone:
		return "done"
	case TaskStateFailed:
		return "failed"
	default:
		return "unknown"
	}
}

func mapToMapData(m *mappkg.Map) *MapData {
	data := &MapData{
		Width:  m.Width,
		Height: m.Height,
		Grid:   make([][]int, m.Height),
	}
	if m.HasStart() {
		data.StartPoint = PointData{X: m.Start.X, Y: m.Start.Y}
	}
	if m.HasEnd() {
		data.EndPoint = PointData{X: m.End.X, Y: m.End.Y}
	}
	for y := 0; y < m.Height; y++ {
		data.Grid[y] = make([]int, m.Width)
		for x := 0; x < m.Width; x++ {
			cell, _ := m.GetCell(x, y)
			data.Grid[y][x] = int(cell)
		}
	}
	return data
}

func buildDraw(task *Task) (map[string]interface{}, error) {
	if task.Map == nil {
		return nil, nil
	}

	cells := make([][]int, task.Map.Height)
	for y := 0; y < task.Map.Height; y++ {
		cells[y] = make([]int, task.Map.Width)
		for x := 0; x < task.Map.Width; x++ {
			cell, _ := task.Map.GetCell(x, y)
			cells[y][x] = int(cell)
		}
	}

	if task.Search != nil {
		visited := task.Search.GetVisited()
		for _, p := range visited {
			if p.X >= 0 && p.X < task.Map.Width && p.Y >= 0 && p.Y < task.Map.Height {
				cells[p.Y][p.X] = 4
			}
		}

		path := task.Search.GetCurrentPath()
		for _, p := range path {
			if p.X >= 0 && p.X < task.Map.Width && p.Y >= 0 && p.Y < task.Map.Height {
				cells[p.Y][p.X] = 5
			}
		}
	}

	return gin.H{
		"width":  task.Map.Width,
		"height": task.Map.Height,
		"cells":  cells,
	}, nil
}

func buildFinalDraw(task *Task) (map[string]interface{}, error) {
	if task.Map == nil {
		return nil, nil
	}

	cells := make([][]int, task.Map.Height)
	for y := 0; y < task.Map.Height; y++ {
		cells[y] = make([]int, task.Map.Width)
		for x := 0; x < task.Map.Width; x++ {
			cell, _ := task.Map.GetCell(x, y)
			cells[y][x] = int(cell)
		}
	}

	if task.Search != nil {
		visited := task.Search.GetVisited()
		for _, p := range visited {
			if p.X >= 0 && p.X < task.Map.Width && p.Y >= 0 && p.Y < task.Map.Height {
				cells[p.Y][p.X] = 4
			}
		}

		path := task.Search.GetShortestPath()
		for _, p := range path {
			if p.X >= 0 && p.X < task.Map.Width && p.Y >= 0 && p.Y < task.Map.Height {
				cells[p.Y][p.X] = 6
			}
		}
	}

	return gin.H{
		"width":  task.Map.Width,
		"height": task.Map.Height,
		"cells":  cells,
	}, nil
}

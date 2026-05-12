package main

import (
	"github.com/gin-gonic/gin"
	"wayfind/backend/middleware"
)

func NewRouter(taskMgr *TaskManager) *gin.Engine {
	gin.SetMode(gin.ReleaseMode)
	r := gin.New()
	r.Use(gin.Logger())
	r.Use(gin.Recovery())
	r.Use(middleware.CORS())

	h := NewHandler(taskMgr)

	v1 := r.Group("/api/v1")
	{
		v1.GET("/health", h.HealthCheck)
		v1.GET("/algorithms", h.GetAlgorithms)

		tasks := v1.Group("/tasks")
		{
			tasks.POST("", h.CreateTask)
			tasks.GET("", h.ListTasks)
			tasks.GET("/:taskId", h.GetTask)
			tasks.DELETE("/:taskId", h.DeleteTask)
		}

		mapCtrl := v1.Group("/map")
		{
			mapCtrl.GET("", h.GetMap)
			mapCtrl.PUT("/cell", h.SetCell)
			mapCtrl.GET("/draw", h.GetDraw)
			mapCtrl.GET("/final-draw", h.GetFinalDraw)
		}

		search := v1.Group("/search")
		{
			search.POST("/init", h.InitializeSearch)
			search.POST("/step", h.SearchStep)
			search.POST("/reset", h.ResetSearch)
			search.GET("/done", h.GetSearchDone)
			search.GET("/result", h.GetSearchResult)
			search.GET("/path", h.GetCurrentPath)
		}

		maps := v1.Group("/maps")
		{
			maps.GET("", h.ListMaps)
			maps.GET("/:name", h.LoadMap)
			maps.POST("/:name", h.SaveMap)
			maps.DELETE("/:name", h.DeleteMap)
		}
	}

	return r
}

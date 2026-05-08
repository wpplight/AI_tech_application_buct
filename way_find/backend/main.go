package main

import (
	"flag"
	"log"
	"os"
	"path/filepath"
)

func main() {
	port := flag.String("port", "8081", "server port")
	flag.Parse()

	homeDir, _ := os.UserHomeDir()
	if homeDir == "" {
		homeDir = "."
	}
	storagePath := filepath.Join(homeDir, "Documents", "WayFind", "maps")

	taskMgr := NewTaskManager(storagePath)
	router := NewRouter(taskMgr)

	log.Printf("WayFind API server starting on :%s", *port)
	log.Printf("API docs: http://localhost:%s/api/v1/health", *port)

	if err := router.Run(":" + *port); err != nil {
		log.Fatal("Failed to start server:", err)
	}
}

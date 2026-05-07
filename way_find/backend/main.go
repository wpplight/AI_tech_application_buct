package main

import (
	"embed"
	"log"

	"github.com/wailsapp/wails/v3/pkg/application"
)

//go:embed frontend/dist
var assets embed.FS

func main() {
	app := application.New(application.Options{
		Name: "WayFind",
		Services: []application.Service{
			application.NewService(NewWayFindService()),
		},
		Assets: application.AssetOptions{
			Handler: application.AssetFileServerFS(assets),
		},
	})

	app.Window.NewWithOptions(application.WebviewWindowOptions{
		Title:            "WayFind - Pathfinding Visualizer",
		Width:            1280,
		Height:           800,
		BackgroundColour: application.NewRGB(15, 20, 25),
	})

	err := app.Run()
	if err != nil {
		log.Fatal("Error running application:", err)
	}
}

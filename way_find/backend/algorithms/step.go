package algorithms

import (
	mappkg "wayfind/backend/map"
)

type SearchState int

const (
	StateReady SearchState = iota
	StateRunning
	StateFound
	StateNotFound
)

type StepResult struct {
	State      SearchState `json:"state"`
	Current    mappkg.Point `json:"current"`
	Neighbors  []mappkg.Point `json:"neighbors"`
	Added      []mappkg.Point `json:"added"`
	Pruned     []mappkg.Point `json:"pruned"`
	Path       []mappkg.Point `json:"path"`
	Visited    []mappkg.Point `json:"visited"`
	Distance   int `json:"distance"`
	Expanded   int `json:"expanded"`
	StepsTaken int `json:"stepsTaken"`
}

type Searcher interface {
	Initialize(m *mappkg.Map)
	Step() *StepResult
	IsDone() bool
	GetResult() *SearchResult
	GetCurrentPath() []mappkg.Point
	GetVisited() []mappkg.Point
	GetShortestPath() []mappkg.Point
}
